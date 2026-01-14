// Shared state model for the shared-state example
// Demonstrates inter-window communication via SharedContext

use dampen_macros::UiModel;
use serde::{Deserialize, Serialize};

/// Shared application state that multiple windows can access and modify
#[derive(Default, Clone, Serialize, Deserialize, UiModel)]
pub struct SharedState {
    /// Current theme: "light" or "dark"
    pub theme: String,

    /// User's preferred language
    pub language: String,

    /// Whether notifications are enabled
    pub notifications_enabled: bool,
}

impl SharedState {
    pub fn new() -> Self {
        Self {
            theme: "light".to_string(),
            language: "en".to_string(),
            notifications_enabled: true,
        }
    }

    pub fn toggle_theme(&mut self) {
        self.theme = if self.theme == "light" {
            "dark".to_string()
        } else {
            "light".to_string()
        };
    }

    pub fn toggle_notifications(&mut self) {
        self.notifications_enabled = !self.notifications_enabled;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dampen_core::UiBindable;

    #[test]
    fn test_shared_state_default() {
        let state = SharedState::default();
        assert_eq!(state.theme, "");
        assert_eq!(state.language, "");
        assert!(!state.notifications_enabled);
    }

    #[test]
    fn test_shared_state_new() {
        let state = SharedState::new();
        assert_eq!(state.theme, "light");
        assert_eq!(state.language, "en");
        assert!(state.notifications_enabled);
    }

    #[test]
    fn test_toggle_theme() {
        let mut state = SharedState::new();
        assert_eq!(state.theme, "light");

        state.toggle_theme();
        assert_eq!(state.theme, "dark");

        state.toggle_theme();
        assert_eq!(state.theme, "light");
    }

    #[test]
    fn test_toggle_notifications() {
        let mut state = SharedState::new();
        assert!(state.notifications_enabled);

        state.toggle_notifications();
        assert!(!state.notifications_enabled);

        state.toggle_notifications();
        assert!(state.notifications_enabled);
    }

    #[test]
    fn test_ui_bindable_theme() {
        let state = SharedState::new();
        let value = state.get_field(&["theme"]);
        assert!(value.is_some());
        match value.unwrap() {
            dampen_core::BindingValue::String(s) => assert_eq!(s, "light"),
            _ => panic!("Expected String binding value"),
        }
    }

    #[test]
    fn test_ui_bindable_language() {
        let state = SharedState::new();
        let value = state.get_field(&["language"]);
        assert!(value.is_some());
        match value.unwrap() {
            dampen_core::BindingValue::String(s) => assert_eq!(s, "en"),
            _ => panic!("Expected String binding value"),
        }
    }

    #[test]
    fn test_ui_bindable_notifications() {
        let state = SharedState::new();
        let value = state.get_field(&["notifications_enabled"]);
        assert!(value.is_some());
        match value.unwrap() {
            dampen_core::BindingValue::Bool(b) => assert_eq!(b, true),
            _ => panic!("Expected Bool binding value"),
        }
    }
}
