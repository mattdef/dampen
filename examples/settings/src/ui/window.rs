// Main view UI module.
//
// This file auto-loads the corresponding app.dampen XML file.

use dampen_core::{AppState, HandlerRegistry};
use dampen_macros::{dampen_ui, UiModel};
use serde::{Deserialize, Serialize};

#[dampen_ui("window.dampen")]
mod _app {}

#[derive(Default, UiModel, Serialize, Deserialize, Clone, Debug)]
pub struct Model;

pub fn create_app_state() -> AppState<Model> {
    let document = _app::document();
    let handler_registry = create_handler_registry();
    AppState::with_handlers(document, handler_registry)
}

pub fn create_handler_registry() -> HandlerRegistry {
    let registry = HandlerRegistry::new();

    registry.register_simple("switch_to_settings", |_model: &mut dyn std::any::Any| {
        println!("Switching to settings view");
    });

    registry
}
