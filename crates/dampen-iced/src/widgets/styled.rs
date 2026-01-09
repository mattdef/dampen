//! Styled wrapper for state-based styling
//!
//! This module provides utilities for managing state-based styles.
//! The actual widget integration happens in the render pipeline.

use dampen_core::ir::style::StyleProperties;
use dampen_core::ir::theme::WidgetState;

/// Helper struct to manage state-based styling
pub struct StateStyleManager {
    base_style: StyleProperties,
    state_styles: Vec<(WidgetState, StyleProperties)>,
    current_state: Option<WidgetState>,
    disabled: bool,
}

impl StateStyleManager {
    /// Create a new state style manager
    pub fn new(base_style: StyleProperties) -> Self {
        Self {
            base_style,
            state_styles: Vec::new(),
            current_state: None,
            disabled: false,
        }
    }

    /// Add a state-specific style
    pub fn with_state(mut self, state: WidgetState, style: StyleProperties) -> Self {
        self.state_styles.push((state, style));
        self
    }

    /// Set the current state
    pub fn set_state(&mut self, state: Option<WidgetState>) {
        self.current_state = state;
    }

    /// Set disabled state
    pub fn set_disabled(&mut self, disabled: bool) {
        self.disabled = disabled;
        if disabled {
            self.current_state = Some(WidgetState::Disabled);
        } else if self.current_state == Some(WidgetState::Disabled) {
            self.current_state = None;
        }
    }

    /// Get the effective style for current state
    pub fn effective_style(&self) -> StyleProperties {
        let mut style = self.base_style.clone();

        // If disabled, always use disabled state
        let state_to_apply = if self.disabled {
            Some(WidgetState::Disabled)
        } else {
            self.current_state
        };

        // Apply state styles if state is set
        if let Some(state) = state_to_apply {
            for (s, state_style) in &self.state_styles {
                if s == &state {
                    // Merge state style over base (override specific fields)
                    if let Some(bg) = &state_style.background {
                        style.background = Some(bg.clone());
                    }
                    if let Some(color) = &state_style.color {
                        style.color = Some(*color);
                    }
                    if let Some(border) = &state_style.border {
                        style.border = Some(border.clone());
                    }
                    if let Some(shadow) = &state_style.shadow {
                        style.shadow = Some(*shadow);
                    }
                    if let Some(opacity) = state_style.opacity {
                        style.opacity = Some(opacity);
                    }
                    if let Some(transform) = &state_style.transform {
                        style.transform = Some(transform.clone());
                    }
                }
            }
        }

        style
    }
}
