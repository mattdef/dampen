//! Window state persistence module.
//!
//! This module handles saving and restoring window state (size, position, maximized)
//! across application restarts.

/// Persistence error types.
pub mod error;
/// Monitor validation utilities.
pub mod monitor;
/// Storage utilities for saving/loading window state.
pub mod storage;
/// Window state data structure.
pub mod window_state;

pub use error::PersistenceError;
pub use monitor::position_is_reasonable;
pub use storage::{WindowSettingsBuilder, get_config_path, load_or_default, save_window_state};
pub use window_state::WindowState;
