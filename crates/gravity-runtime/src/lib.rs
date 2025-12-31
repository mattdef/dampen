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

pub mod interpreter;
pub mod overlay;
pub mod state;
pub mod watcher;

pub use interpreter::{Interpreter, HotReloadInterpreter, ReloadResult, DispatchError};
pub use overlay::{ErrorOverlay, OverlayManager};
pub use state::{RuntimeState, StateRestoration, StateMigration};
pub use watcher::{FileWatcher, FileEvent};

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
