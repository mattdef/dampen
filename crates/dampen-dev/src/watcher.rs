//! File watching functionality for hot-reload
//!
//! This module wraps the `notify` crate to provide file system watching
//! with debouncing and filtering for .dampen files.

use crossbeam_channel::{Receiver, Sender};
use notify::{RecursiveMode, Watcher};
use notify_debouncer_full::{new_debouncer, DebounceEventResult, Debouncer, FileIdMap};
use std::path::{Path, PathBuf};
use std::time::Duration;

/// Configuration for file watcher behavior
#[derive(Debug, Clone)]
pub struct FileWatcherConfig {
    /// Paths to watch (directories or specific files)
    pub watch_paths: Vec<PathBuf>,

    /// Debounce interval in milliseconds
    pub debounce_ms: u64,

    /// File extension filter (default: ".dampen")
    pub extension_filter: String,

    /// Whether to watch recursively
    pub recursive: bool,
}

impl Default for FileWatcherConfig {
    fn default() -> Self {
        Self {
            watch_paths: vec![PathBuf::from("src/ui")],
            debounce_ms: 100,
            extension_filter: ".dampen".to_string(),
            recursive: true,
        }
    }
}

/// Runtime state of file watcher
#[derive(Debug)]
pub enum FileWatcherState {
    /// Watcher is initialized but not started
    Idle,

    /// Actively watching for changes
    Watching {
        /// Paths being watched
        paths: Vec<PathBuf>,
    },

    /// Error state (watcher failed to initialize)
    Failed {
        /// Error description
        error: String,
    },
}

/// File watcher wrapper around notify crate
///
/// Wraps `notify::RecommendedWatcher` with debouncing and filtering
/// for `.dampen` files. Provides a channel-based API for receiving
/// file change events.
pub struct FileWatcher {
    config: FileWatcherConfig,
    debouncer: Debouncer<notify::RecommendedWatcher, FileIdMap>,
    receiver: Receiver<PathBuf>,
}

impl FileWatcher {
    /// Create a new file watcher with the given configuration
    ///
    /// Sets up a debounced file watcher with crossbeam channels for
    /// event communication. The watcher is created but not yet watching
    /// any paths - use `watch()` to add paths.
    ///
    /// # Arguments
    /// * `config` - File watcher configuration
    ///
    /// # Returns
    /// A new FileWatcher instance or an error if watcher creation fails
    ///
    /// # Errors
    /// Returns an error if:
    /// - The file watcher cannot be initialized (OS limitations, permissions)
    /// - The debouncer setup fails
    ///
    /// # Example
    /// ```no_run
    /// use dampen_dev::watcher::{FileWatcher, FileWatcherConfig};
    ///
    /// let config = FileWatcherConfig::default();
    /// let watcher = FileWatcher::new(config).expect("Failed to create watcher");
    /// ```
    pub fn new(config: FileWatcherConfig) -> Result<Self, FileWatcherError> {
        let (tx, rx) = crossbeam_channel::unbounded();
        let extension_filter = config.extension_filter.clone();

        // Create debouncer with configured interval
        let debouncer = new_debouncer(
            Duration::from_millis(config.debounce_ms),
            None, // Use default tick rate
            move |result: DebounceEventResult| {
                handle_debounced_events(result, &tx, &extension_filter);
            },
        )
        .map_err(|e| FileWatcherError::InitializationFailed(e.to_string()))?;

        Ok(Self {
            config,
            debouncer,
            receiver: rx,
        })
    }

    /// Add a path to watch for changes
    ///
    /// Watches the specified path for file system changes. If the path is a directory
    /// and `recursive` is enabled in the config, watches all subdirectories as well.
    ///
    /// # Arguments
    /// * `path` - Path to watch (file or directory)
    ///
    /// # Errors
    /// Returns an error if:
    /// - The path does not exist
    /// - Permission denied to watch the path
    /// - The path is already being watched
    /// - OS-specific watcher limitations reached
    ///
    /// # Example
    /// ```no_run
    /// use dampen_dev::watcher::{FileWatcher, FileWatcherConfig};
    /// use std::path::PathBuf;
    ///
    /// let mut watcher = FileWatcher::new(FileWatcherConfig::default()).unwrap();
    /// watcher.watch(PathBuf::from("src/ui")).expect("Failed to watch path");
    /// ```
    pub fn watch(&mut self, path: PathBuf) -> Result<(), FileWatcherError> {
        // Check if path exists
        if !path.exists() {
            return Err(FileWatcherError::PathNotFound(path));
        }

        // Determine recursive mode from config
        let recursive_mode = if self.config.recursive {
            RecursiveMode::Recursive
        } else {
            RecursiveMode::NonRecursive
        };

        // Add path to watcher with enhanced error handling
        self.debouncer
            .watcher()
            .watch(&path, recursive_mode)
            .map_err(|e| {
                // Check if this is a permission error by examining the error chain
                // notify::Error wraps std::io::Error, so we check the source
                let error_string = e.to_string().to_lowercase();
                if error_string.contains("permission denied")
                    || error_string.contains("access is denied")
                {
                    return FileWatcherError::PermissionDenied(path.clone());
                }

                // Generic watch error for other cases
                FileWatcherError::WatchError {
                    path: path.clone(),
                    error: e.to_string(),
                }
            })?;

        Ok(())
    }

    /// Remove a path from the watch list
    ///
    /// Stops watching the specified path for changes.
    ///
    /// # Arguments
    /// * `path` - Path to unwatch
    ///
    /// # Errors
    /// Returns an error if the path is not currently being watched
    ///
    /// # Example
    /// ```no_run
    /// use dampen_dev::watcher::{FileWatcher, FileWatcherConfig};
    /// use std::path::PathBuf;
    ///
    /// let mut watcher = FileWatcher::new(FileWatcherConfig::default()).unwrap();
    /// let path = PathBuf::from("src/ui");
    /// watcher.watch(path.clone()).unwrap();
    /// watcher.unwatch(path).expect("Failed to unwatch path");
    /// ```
    pub fn unwatch(&mut self, path: PathBuf) -> Result<(), FileWatcherError> {
        self.debouncer
            .watcher()
            .unwatch(&path)
            .map_err(|e| FileWatcherError::WatchError {
                path: path.clone(),
                error: e.to_string(),
            })?;

        Ok(())
    }

    /// Get the receiver for file change events
    ///
    /// Returns a reference to the channel receiver that will receive
    /// paths of changed `.dampen` files. Events are debounced according
    /// to the configuration.
    ///
    /// # Returns
    /// A reference to the crossbeam channel receiver
    ///
    /// # Example
    /// ```no_run
    /// use dampen_dev::watcher::{FileWatcher, FileWatcherConfig};
    ///
    /// let watcher = FileWatcher::new(FileWatcherConfig::default()).unwrap();
    /// let receiver = watcher.receiver();
    ///
    /// // In an event loop:
    /// // for changed_file in receiver.try_iter() {
    /// //     println!("File changed: {:?}", changed_file);
    /// // }
    /// ```
    pub fn receiver(&self) -> &Receiver<PathBuf> {
        &self.receiver
    }

    /// Get the configuration used by this watcher
    ///
    /// # Returns
    /// A reference to the FileWatcherConfig
    pub fn config(&self) -> &FileWatcherConfig {
        &self.config
    }
}

/// Handle debounced file system events and filter for .dampen files
///
/// This function is called by the notify-debouncer when file events occur.
/// It filters events to only include files matching the extension filter
/// and sends the paths through the channel.
///
/// **File Deletion Handling**: If a file is deleted during watching, the event
/// is silently ignored. This is graceful behavior - deleted files don't trigger
/// hot-reload attempts.
///
/// **Simultaneous Multi-File Changes** (T124): The debouncing mechanism (100ms window)
/// naturally batches rapid file changes together. When multiple files are modified
/// simultaneously (e.g., save-all in IDE), all events within the debounce window
/// are processed together in a single batch. Each file change triggers its own
/// hot-reload attempt sequentially, with the most recent change winning.
fn handle_debounced_events(
    result: DebounceEventResult,
    sender: &Sender<PathBuf>,
    extension_filter: &str,
) {
    match result {
        Ok(events) => {
            for event in events {
                // Extract paths from the event
                for path in &event.paths {
                    // Filter by extension
                    if !path_matches_extension(path, extension_filter) {
                        continue;
                    }

                    // Check if file still exists (handles deletion gracefully)
                    if !path.exists() {
                        // File was deleted - this is normal, don't send event
                        // In development mode, file deletions are intentional (e.g., cleanup)
                        // and don't require hot-reload attempts
                        #[cfg(debug_assertions)]
                        eprintln!("File watcher: ignoring deleted file {:?}", path);
                        continue;
                    }

                    // Send the path through the channel
                    // If the receiver is dropped, we silently ignore the error
                    let _ = sender.send(path.clone());
                }
            }
        }
        Err(errors) => {
            // Log errors but don't stop watching
            // These could be permission errors, I/O errors, etc.
            for error in errors {
                eprintln!("File watcher error: {:?}", error);
            }
        }
    }
}

/// Check if a path matches the extension filter
///
/// # Arguments
/// * `path` - Path to check
/// * `extension` - Extension to match (e.g., ".dampen")
///
/// # Returns
/// True if the path's extension matches the filter
fn path_matches_extension(path: &Path, extension: &str) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| format!(".{}", ext) == extension)
        .unwrap_or(false)
}

/// Errors that can occur during file watching
#[derive(Debug, thiserror::Error)]
pub enum FileWatcherError {
    /// Failed to initialize the file watcher
    #[error("Failed to initialize file watcher: {0}")]
    InitializationFailed(String),

    /// Path does not exist
    #[error("Path not found: {0}")]
    PathNotFound(PathBuf),

    /// Error while watching a path
    #[error("Failed to watch path {path}: {error}")]
    WatchError {
        /// Path that failed to be watched
        path: PathBuf,
        /// Error description
        error: String,
    },

    /// Permission denied
    #[error("Permission denied for path: {0}")]
    PermissionDenied(PathBuf),

    /// File was deleted during watch
    #[error("File was deleted: {0}")]
    FileDeleted(PathBuf),
}
