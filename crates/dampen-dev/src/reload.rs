//! Hot-reload state preservation and coordination
//!
//! This module handles the hot-reload process, including model snapshotting,
//! state restoration, and error recovery.

use dampen_core::binding::UiBindable;
use dampen_core::parser::error::ParseError;
use dampen_core::state::AppState;
use serde::{de::DeserializeOwned, Serialize};
use std::marker::PhantomData;
use std::time::{Duration, Instant};

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

    /// Snapshot the current model state to JSON
    pub fn snapshot_model(&mut self, model: &M) -> Result<(), String>
    where
        M: Serialize,
    {
        match serde_json::to_string(model) {
            Ok(json) => {
                self.last_model_snapshot = Some(json);
                Ok(())
            }
            Err(e) => Err(format!("Failed to serialize model: {}", e)),
        }
    }

    /// Restore the model from the last snapshot
    pub fn restore_model(&self) -> Result<M, String>
    where
        M: DeserializeOwned,
    {
        match &self.last_model_snapshot {
            Some(json) => serde_json::from_str(json)
                .map_err(|e| format!("Failed to deserialize model: {}", e)),
            None => Err("No model snapshot available".to_string()),
        }
    }

    /// Record a reload attempt
    pub fn record_reload(&mut self, success: bool) {
        self.reload_count += 1;
        self.last_reload_timestamp = Instant::now();
        if !success {
            self.error = Some("Reload failed".to_string());
        } else {
            self.error = None;
        }
    }

    /// Get the latency of the last reload
    pub fn last_reload_latency(&self) -> Duration {
        self.last_reload_timestamp.elapsed()
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
