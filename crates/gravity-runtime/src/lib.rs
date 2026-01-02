//! Gravity Runtime - Hot-reload Interpreter and File Watcher
//!
//! This crate provides runtime support for development mode with hot-reload.
//!
//! # Features
//!
//! - **File Watching**: Automatic detection of .gravity file changes
//! - **State Preservation**: Model state persists across hot-reloads
//! - **Error Overlay**: Visual error display in development mode
//! - **Hot-Reload Loop**: Integrated reload management
//!
//! # Example
//!
//! ```rust,ignore
//! use gravity_runtime::{Runtime, HotReloadInterpreter};
//! use gravity_core::HandlerRegistry;
//!
//! let registry = HandlerRegistry::new();
//! let mut interpreter = HotReloadInterpreter::new(registry);
//!
//! // Load initial document
//! interpreter.load_document(xml_content)?;
//!
//! // Watch for changes and reload
//! // (Integrated with file watcher in dev mode)
//! ```

pub mod breakpoint;
pub mod interpreter;
pub mod overlay;
pub mod state;
pub mod style_cascade;
pub mod theme_manager;
pub mod watcher;

pub use breakpoint::{
    get_active_breakpoint, resolve_breakpoint_attributes, resolve_tree_breakpoint_attributes,
    would_change_breakpoint,
};
pub use interpreter::{DispatchError, HotReloadInterpreter, Interpreter, ReloadResult};
pub use overlay::{ErrorOverlay, OverlayManager};
pub use state::{RuntimeState, StateMigration, StateRestoration};
pub use style_cascade::{merge_styles, resolve_layout, StyleCascade};
pub use theme_manager::ThemeManager;
pub use watcher::{FileEvent, FileWatcher};

/// Hot-reload enabled runtime
pub struct Runtime;

impl Runtime {
    /// Create a new runtime instance
    pub fn new() -> Self {
        Self
    }
}

impl Default for Runtime {
    fn default() -> Self {
        Self::new()
    }
}
