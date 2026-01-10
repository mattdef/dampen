//! Hot-reload state preservation and coordination
//!
//! This module handles the hot-reload process, including model snapshotting,
//! state restoration, and error recovery.

use std::marker::PhantomData;
use std::time::Instant;
use dampen_core::state::AppState;
use dampen_core::binding::UiBindable;
use dampen_core::parser::error::ParseError;

/// Tracks hot-reload state and history for debugging
pub struct HotReloadContext<M> {
    /// Last successful model snapshot (JSON)
    last_model_snapshot: Option<String>,

    /// Timestamp of last reload
    last_reload_timestamp: Instant,

    /// Reload count (for metrics)
    reload_count: usize,

    /// Current error state (if any)
    error: Option<String>,

    _marker: PhantomData<M>,
}

impl<M: UiBindable> HotReloadContext<M> {
    /// Create a new hot-reload context
    pub fn new() -> Self {
        Self {
            last_model_snapshot: None,
            last_reload_timestamp: Instant::now(),
            reload_count: 0,
            error: None,
            _marker: PhantomData,
        }
    }
}

impl<M: UiBindable> Default for HotReloadContext<M> {
    fn default() -> Self {
        Self::new()
    }
}

/// Result type for hot-reload attempts with detailed error information
#[derive(Debug)]
pub enum ReloadResult<M: UiBindable> {
    /// Reload succeeded
    Success(AppState<M>),

    /// XML parse error (reject reload)
    ParseError(ParseError),

    /// Schema validation error (reject reload)
    ValidationError(Vec<String>),

    /// Model deserialization failed, using default (accept reload with warning)
    StateRestoreWarning(AppState<M>, String),
}
