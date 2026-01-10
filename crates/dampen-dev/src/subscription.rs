//! Iced subscription integration for file watching
//!
//! This module provides an Iced subscription that bridges file system events
//! from the notify crate into Iced's async message system.

use std::path::PathBuf;
use dampen_core::ir::DampenDocument;
use dampen_core::parser::error::ParseError;
use iced::Subscription;

/// Domain event for file watcher subscription output
#[derive(Debug, Clone)]
pub enum FileEvent {
    /// File changed and parsed successfully
    Success {
        /// Path to the changed file
        path: PathBuf,
        /// Parsed document
        document: DampenDocument,
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

/// Create a subscription that watches files and emits FileEvents
///
/// # Arguments
/// * `paths` - Paths to watch (directories or files)
/// * `debounce_ms` - Debounce interval in milliseconds
///
/// # Returns
/// An Iced subscription that produces FileEvent messages
pub fn watch_files<P: AsRef<std::path::Path>>(
    _paths: Vec<P>,
    _debounce_ms: u64,
) -> Subscription<FileEvent> {
    // Stub implementation - will be completed in Phase 4
    Subscription::none()
}
