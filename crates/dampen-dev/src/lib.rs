//! Development mode tooling for Dampen
//!
//! This crate provides hot-reload capabilities, file watching, and error overlays
//! for rapid UI development iteration. It is only used in development/interpreted mode.

#![warn(missing_docs)]
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

pub mod overlay;
pub mod reload;
pub mod subscription;
pub mod watcher;

// Re-export key types for convenience
pub use overlay::ErrorOverlay;
pub use reload::{HotReloadContext, ReloadResult};
pub use subscription::{watch_files, FileEvent, FileWatcherRecipe};
pub use watcher::{FileWatcher, FileWatcherConfig, FileWatcherError, FileWatcherState};
