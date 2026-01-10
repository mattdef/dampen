//! Development mode tooling for Dampen
//!
//! This crate provides hot-reload capabilities, file watching, and error overlays
//! for rapid UI development iteration. It is only used in development/interpreted mode.

#![warn(missing_docs)]
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

pub mod watcher;
pub mod subscription;
pub mod reload;
pub mod overlay;

// Re-export key types for convenience
pub use watcher::{FileWatcher, FileWatcherConfig, FileWatcherState};
pub use subscription::{FileEvent, watch_files};
pub use reload::{HotReloadContext, ReloadResult};
pub use overlay::ErrorOverlay;
