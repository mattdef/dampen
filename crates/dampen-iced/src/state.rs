//! Widget state management for Iced backend
//!
//! This module manages widget interaction states (hover, focus, active, disabled)
//! and applies state-based styles during rendering.

use dampen_core::ir::theme::WidgetState;
use std::collections::HashMap;

/// Tracks state for individual widgets
#[derive(Debug, Clone)]
pub struct WidgetStateManager {
    /// Map from widget ID to current state
    states: HashMap<String, WidgetState>,
    /// Track which widgets are disabled
    disabled: HashMap<String, bool>,
}

impl WidgetStateManager {
    pub fn new() -> Self {
        Self {
            states: HashMap::new(),
            disabled: HashMap::new(),
        }
    }

    /// Set state for a widget
    pub fn set_state(&mut self, widget_id: String, state: WidgetState) {
        self.states.insert(widget_id, state);
    }

    /// Get current state for a widget
    pub fn get_state(&self, widget_id: &str) -> Option<WidgetState> {
        if let Some(true) = self.disabled.get(widget_id) {
            return Some(WidgetState::Disabled);
        }
        self.states.get(widget_id).cloned()
    }

    /// Set disabled state
    pub fn set_disabled(&mut self, widget_id: String, disabled: bool) {
        self.disabled.insert(widget_id.clone(), disabled);
        if disabled {
            self.states.insert(widget_id, WidgetState::Disabled);
        }
    }

    /// Handle mouse enter (hover start)
    pub fn on_mouse_enter(&mut self, widget_id: String) {
        if !self.is_disabled(&widget_id) {
            self.states.insert(widget_id, WidgetState::Hover);
        }
    }

    /// Handle mouse leave (hover end)
    pub fn on_mouse_leave(&mut self, widget_id: String) {
        if !self.is_disabled(&widget_id) {
            self.states.remove(&widget_id);
        }
    }

    /// Handle mouse press (active start)
    pub fn on_mouse_press(&mut self, widget_id: String) {
        if !self.is_disabled(&widget_id) {
            self.states.insert(widget_id, WidgetState::Active);
        }
    }

    /// Handle mouse release (active end, goes back to hover)
    pub fn on_mouse_release(&mut self, widget_id: String) {
        if !self.is_disabled(&widget_id) {
            self.states.insert(widget_id, WidgetState::Hover);
        }
    }

    /// Handle focus gain
    pub fn on_focus(&mut self, widget_id: String) {
        if !self.is_disabled(&widget_id) {
            self.states.insert(widget_id, WidgetState::Focus);
        }
    }

    /// Handle focus loss
    pub fn on_blur(&mut self, widget_id: String) {
        if !self.is_disabled(&widget_id) {
            self.states.remove(&widget_id);
        }
    }

    /// Check if widget is disabled
    fn is_disabled(&self, widget_id: &str) -> bool {
        self.disabled.get(widget_id).copied().unwrap_or(false)
    }

    /// Clear all states (e.g., on hot-reload)
    pub fn clear(&mut self) {
        self.states.clear();
        self.disabled.clear();
    }
}

impl Default for WidgetStateManager {
    fn default() -> Self {
        Self::new()
    }
}
