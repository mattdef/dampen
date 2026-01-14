//! Shared state model for the macro-shared-state example.
//!
//! This module defines the SharedState struct that will be automatically
//! wrapped in SharedContext by the #[dampen_app] macro.

use dampen_macros::UiModel;

/// Shared state accessible across all views.
///
/// This state is automatically wrapped in SharedContext<SharedState>
/// by the #[dampen_app] macro when shared_model = "SharedState" is specified.
#[derive(Clone, Debug, UiModel)]
pub struct SharedState {
    /// Current theme name (e.g., "Light", "Dark", "Solarized")
    pub theme: String,

    /// Current username
    pub username: String,

    /// Current language
    pub language: String,

    /// Number of notifications (using i64 for UiBindable compatibility)
    pub notification_count: i64,
}

impl Default for SharedState {
    fn default() -> Self {
        Self {
            theme: "Light".to_string(),
            username: "Guest".to_string(),
            language: "English".to_string(),
            notification_count: 0,
        }
    }
}
