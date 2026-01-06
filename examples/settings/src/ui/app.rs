// Main view UI module.
//
// This file auto-loads the corresponding app.gravity XML file.

use gravity_core::{AppState, HandlerRegistry};
use gravity_macros::{gravity_ui, UiModel};
use serde::{Deserialize, Serialize};

#[derive(Default, UiModel, Serialize, Deserialize, Clone, Debug)]
pub struct Model;

#[gravity_ui("app.gravity")]
mod _app {}

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
