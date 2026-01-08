//! Gravity Runtime - Interpreter, State Management, and Error Handling
//!
//! This crate provides runtime support for Gravity UI framework.
//!
//! # Features
//!
//! - **Event Dispatch**: Handler registry and event routing
//! - **State Management**: Serialization/deserialization for application state
//! - **Error Handling**: Error overlay and diagnostic display
//!
//! # Example
//!
//! ```rust,ignore
//! use gravity_runtime::{Runtime, Interpreter};
//! use gravity_core::HandlerRegistry;
//!
//! let registry = HandlerRegistry::new();
//! let interpreter = Interpreter::new(registry);
//! ```

pub mod breakpoint;
pub mod interpreter;
pub mod overlay;
pub mod state;
pub mod style_cascade;
pub mod theme_manager;

pub use breakpoint::{
    get_active_breakpoint, resolve_breakpoint_attributes, resolve_tree_breakpoint_attributes,
    would_change_breakpoint,
};
pub use interpreter::{DispatchError, Interpreter};
pub use overlay::{ErrorOverlay, OverlayManager};
pub use state::{RuntimeState, StateMigration, StateRestoration};
pub use style_cascade::{merge_styles, resolve_layout, StyleCascade};
pub use theme_manager::ThemeManager;

/// Gravity Runtime
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
