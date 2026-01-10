//! File watching functionality for hot-reload
//!
//! This module wraps the `notify` crate to provide file system watching
//! with debouncing and filtering for .dampen files.

use std::path::PathBuf;

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
        paths: Vec<PathBuf>
    },

    /// Error state (watcher failed to initialize)
    Failed {
        /// Error description
        error: String
    },
}

/// File watcher wrapper around notify crate
pub struct FileWatcher {
    // Implementation will be added in Phase 4
    _config: FileWatcherConfig,
}

impl FileWatcher {
    /// Create a new file watcher with the given configuration
    ///
    /// # Arguments
    /// * `config` - File watcher configuration
    ///
    /// # Returns
    /// A new FileWatcher instance
    pub fn new(_config: FileWatcherConfig) -> Self {
        // Stub implementation - will be completed in Phase 4
        Self { _config }
    }
}
