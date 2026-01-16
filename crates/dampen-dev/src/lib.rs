//! Development mode tooling for Dampen
//!
//! This crate provides hot-reload capabilities, file watching, and error overlays
//! for rapid UI development iteration. It is only used in development/interpreted mode.

#![warn(missing_docs)]
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

pub mod overlay;
pub mod reload;
pub mod subscription;
pub mod theme_loader;
pub mod watcher;

// Re-export key types for convenience
pub use overlay::ErrorOverlay;
pub use reload::{HotReloadContext, ReloadResult};
pub use subscription::{FileEvent, FileWatcherRecipe, watch_files};
pub use theme_loader::{ThemeLoadError, discover_theme_file, load_theme_context};
pub use watcher::{FileWatcher, FileWatcherConfig, FileWatcherError, FileWatcherState};
