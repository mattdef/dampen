use dampen_core::{AppState, HandlerRegistry};
use dampen_macros::{UiModel, dampen_ui};
use serde::{Deserialize, Serialize};

/// Model for the main window (local state)
#[derive(Default, Clone, Serialize, Deserialize, UiModel)]
pub struct WindowModel {
    pub status: String,
}

#[dampen_ui("window.dampen")]
mod _window {}

pub fn create_app_state() -> AppState<WindowModel, ()> {
    let document = _window::document();
    let handlers = create_handler_registry();
    AppState::with_handlers(document, handlers)
}

pub fn create_handler_registry() -> HandlerRegistry {
    let mut registry = HandlerRegistry::new();

    // Placeholder for open_settings handler
    // This will be implemented when multi-window support is added
    registry.register_simple("open_settings", |_model: &mut dyn std::any::Any| {
        println!("Opening settings window...");
    });

    registry
}
