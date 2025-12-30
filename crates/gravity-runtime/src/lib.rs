//! Gravity Runtime - Hot-reload Interpreter and File Watcher
//!
//! This crate provides runtime support for development mode with hot-reload.

pub mod interpreter;
pub mod overlay;
pub mod state;
pub mod watcher;

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
