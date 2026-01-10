//! Error overlay UI components for displaying parse errors
//!
//! This module provides UI widgets for displaying error overlays during
//! hot-reload when XML parsing or validation fails.

use dampen_core::parser::error::ParseError;
use std::time::Instant;

/// UI state for displaying parse errors during hot-reload
#[derive(Debug, Clone)]
pub struct ErrorOverlay {
    /// Parse error details
    pub error: Option<ParseError>,

    /// Whether overlay is visible
    pub visible: bool,

    /// Timestamp when error occurred
    pub timestamp: Instant,
}

impl ErrorOverlay {
    /// Create a new error overlay (initially hidden)
    pub fn new() -> Self {
        Self {
            error: None,
            visible: false,
            timestamp: Instant::now(),
        }
    }

    /// Show the overlay with an error
    ///
    /// # Arguments
    /// * `error` - The parse error to display
    pub fn show(&mut self, error: ParseError) {
        self.error = Some(error);
        self.visible = true;
        self.timestamp = Instant::now();
    }

    /// Hide the overlay
    pub fn hide(&mut self) {
        self.visible = false;
    }

    /// Check if the overlay is currently visible
    pub fn is_visible(&self) -> bool {
        self.visible
    }
}

impl Default for ErrorOverlay {
    fn default() -> Self {
        Self::new()
    }
}
