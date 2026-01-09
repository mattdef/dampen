// Auto-loaded UI module for hello-world example.
//
// This file is automatically compiled and loads the corresponding app.dampen XML file.

use dampen_core::{AppState, HandlerRegistry};
use dampen_macros::{dampen_ui, UiModel};
use serde::{Deserialize, Serialize};

/// The application model.
#[derive(Default, UiModel, Serialize, Deserialize, Clone, Debug)]
pub struct Model {
    pub message: String,
}

/// Auto-load the app.dampen XML file.
/// Path is relative to this file (src/ui/).
#[dampen_ui("window.dampen")]
mod _app {}

/// Create the AppState for the hello-world example.
pub fn create_app_state() -> AppState<Model> {
    let document = _app::document();
    let handler_registry = create_handler_registry();
    AppState::with_handlers(document, handler_registry)
}

/// Create and configure the handler registry.
pub fn create_handler_registry() -> HandlerRegistry {
    let registry = HandlerRegistry::new();

    // Register the greet handler
    registry.register_simple("greet", |model: &mut dyn std::any::Any| {
        if let Some(m) = model.downcast_mut::<Model>() {
            m.message = "Hello World!".to_string();
        }
    });

    registry
}
