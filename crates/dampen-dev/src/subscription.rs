//! Iced subscription integration for file watching
//!
//! This module provides an Iced subscription that bridges file system events
//! from the notify crate into Iced's async message system.

use crate::watcher::{FileWatcher, FileWatcherConfig};
use dampen_core::ir::DampenDocument;
use dampen_core::parser;
use dampen_core::parser::error::ParseError;

use iced::advanced::subscription::{EventStream, Hasher, Recipe};
use iced::Subscription;
use std::hash::Hash;
use std::path::PathBuf;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;

/// Domain event for file watcher subscription output
#[derive(Debug, Clone)]
pub enum FileEvent {
    /// File changed and parsed successfully
    Success {
        /// Path to the changed file
        path: PathBuf,
        /// Parsed document (boxed to reduce enum size)
        document: Box<DampenDocument>,
    },

    /// Parse error (XML syntax or validation)
    ParseError {
        /// Path to the file with error
        path: PathBuf,
        /// Parse error details
        error: ParseError,
        /// File content for error overlay display
        content: String,
    },

    /// File watcher error (permissions, deleted file, etc.)
    WatcherError {
        /// Path to the file
        path: PathBuf,
        /// Error description
        error: String,
    },
}

/// Recipe for creating file watching subscriptions
///
/// This struct implements `iced::subscription::Recipe` to bridge synchronous
/// file system events from `notify` into Iced's async subscription system.
///
/// The recipe creates a unique subscription based on the watched paths and
/// debounce configuration, ensuring that multiple subscriptions with the same
/// configuration share the same underlying file watcher.
#[derive(Debug, Clone)]
pub struct FileWatcherRecipe {
    /// Paths to watch for changes
    pub paths: Vec<PathBuf>,

    /// Debounce interval in milliseconds
    pub debounce_ms: u64,

    /// File extension filter (e.g., ".dampen")
    pub extension_filter: String,

    /// Whether to watch directories recursively
    pub recursive: bool,
}

impl FileWatcherRecipe {
    /// Create a new file watcher recipe
    ///
    /// # Arguments
    /// * `paths` - Paths to watch (directories or specific files)
    /// * `debounce_ms` - Debounce interval in milliseconds
    ///
    /// # Returns
    /// A new FileWatcherRecipe with default settings
    ///
    /// # Example
    /// ```no_run
    /// use dampen_dev::subscription::FileWatcherRecipe;
    /// use std::path::PathBuf;
    ///
    /// let recipe = FileWatcherRecipe::new(
    ///     vec![PathBuf::from("src/ui")],
    ///     100
    /// );
    /// ```
    pub fn new(paths: Vec<PathBuf>, debounce_ms: u64) -> Self {
        Self {
            paths,
            debounce_ms,
            extension_filter: ".dampen".to_string(),
            recursive: true,
        }
    }

    /// Set the file extension filter
    ///
    /// # Arguments
    /// * `extension` - File extension to watch (e.g., ".dampen", ".xml")
    ///
    /// # Returns
    /// Self for method chaining
    pub fn with_extension(mut self, extension: impl Into<String>) -> Self {
        self.extension_filter = extension.into();
        self
    }

    /// Set whether to watch directories recursively
    ///
    /// # Arguments
    /// * `recursive` - If true, watches subdirectories
    ///
    /// # Returns
    /// Self for method chaining
    pub fn with_recursive(mut self, recursive: bool) -> Self {
        self.recursive = recursive;
        self
    }
}

impl Recipe for FileWatcherRecipe {
    type Output = FileEvent;

    fn hash(&self, state: &mut Hasher) {
        // Hash all configuration parameters to create a unique subscription identity
        // This ensures that subscriptions with the same configuration are deduplicated

        // Hash the type discriminant
        std::any::TypeId::of::<Self>().hash(state);

        // Hash paths (sorted to ensure order-independence)
        let mut sorted_paths = self.paths.clone();
        sorted_paths.sort();
        for path in &sorted_paths {
            path.hash(state);
        }

        // Hash configuration
        self.debounce_ms.hash(state);
        self.extension_filter.hash(state);
        self.recursive.hash(state);
    }

    fn stream(
        self: Box<Self>,
        _input: EventStream,
    ) -> futures::stream::BoxStream<'static, Self::Output> {
        // Extract configuration from self
        let paths = self.paths;
        let debounce_ms = self.debounce_ms;
        let extension_filter = self.extension_filter;
        let recursive = self.recursive;

        // Create async channel for bridging sync→async
        // Buffer size of 100 should handle burst file changes
        let (tx, rx) = mpsc::channel(100);

        // Spawn blocking task to run the synchronous file watcher
        // This bridges the sync crossbeam_channel to async tokio channel
        tokio::task::spawn_blocking(move || {
            // Create watcher configuration
            let config = FileWatcherConfig {
                watch_paths: paths.clone(),
                debounce_ms,
                extension_filter,
                recursive,
            };

            eprintln!(
                "[dampen-dev] Creating file watcher with config: paths={:?}, debounce={}ms",
                paths, debounce_ms
            );

            // Create the file watcher
            let mut watcher = match FileWatcher::new(config) {
                Ok(w) => {
                    eprintln!("[dampen-dev] File watcher created successfully");
                    w
                }
                Err(e) => {
                    eprintln!("[dampen-dev] Failed to create file watcher: {}", e);
                    // Send initialization error and return
                    let _ = tx.blocking_send(FileEvent::WatcherError {
                        path: PathBuf::new(),
                        error: format!("Failed to create file watcher: {}", e),
                    });
                    return;
                }
            };

            // Start watching all configured paths
            for path in &paths {
                eprintln!("[dampen-dev] Attempting to watch: {}", path.display());
                if let Err(e) = watcher.watch(path.clone()) {
                    eprintln!("[dampen-dev] Failed to watch {}: {}", path.display(), e);
                    let _ = tx.blocking_send(FileEvent::WatcherError {
                        path: path.clone(),
                        error: format!("Failed to watch path: {}", e),
                    });
                } else {
                    eprintln!("[dampen-dev] Successfully watching: {}", path.display());
                }
            }

            // Read events from the file watcher's channel
            eprintln!("[dampen-dev] File watcher ready, waiting for events...");
            let receiver = watcher.receiver();
            while let Ok(path) = receiver.recv() {
                eprintln!("[dampen-dev] File changed: {}", path.display());
                // Read the file content
                let content = match std::fs::read_to_string(&path) {
                    Ok(c) => c,
                    Err(e) => {
                        // File read error (permissions, deleted, etc.)
                        let _ = tx.blocking_send(FileEvent::WatcherError {
                            path: path.clone(),
                            error: format!("Failed to read file: {}", e),
                        });
                        continue;
                    }
                };

                // Parse the XML content
                match parser::parse(&content) {
                    Ok(document) => {
                        // Success: send parsed document (boxed to reduce enum size)
                        let _ = tx.blocking_send(FileEvent::Success {
                            path: path.clone(),
                            document: Box::new(document),
                        });
                    }
                    Err(error) => {
                        // Parse error: send error with content for overlay
                        let _ = tx.blocking_send(FileEvent::ParseError {
                            path: path.clone(),
                            error,
                            content,
                        });
                    }
                }
            }
        });

        // Convert the tokio receiver into a stream and return it
        Box::pin(ReceiverStream::new(rx))
    }
}

/// Create a subscription that watches files and emits FileEvents
///
/// This is the main public API for creating file watching subscriptions in Iced applications.
/// It creates a `FileWatcherRecipe` that bridges synchronous file system events into Iced's
/// async subscription system.
///
/// # Arguments
/// * `paths` - Paths to watch (directories or files)
/// * `debounce_ms` - Debounce interval in milliseconds (recommended: 100ms)
///
/// # Returns
/// An Iced subscription that produces FileEvent messages
///
/// # Example
/// ```no_run
/// use dampen_dev::subscription::{watch_files, FileEvent};
/// use std::path::PathBuf;
///
/// // Create a subscription that watches UI files
/// let subscription = watch_files(vec![PathBuf::from("src/ui")], 100);
/// // The subscription yields FileEvent values
/// ```
pub fn watch_files<P: AsRef<std::path::Path>>(
    paths: Vec<P>,
    debounce_ms: u64,
) -> Subscription<FileEvent> {
    // Convert paths to PathBuf
    let path_bufs: Vec<PathBuf> = paths.iter().map(|p| p.as_ref().to_path_buf()).collect();

    // Create the recipe
    let recipe = FileWatcherRecipe::new(path_bufs, debounce_ms);

    // Use the advanced API to create a subscription from our Recipe
    use iced::advanced::subscription::from_recipe;
    from_recipe(recipe)
}
