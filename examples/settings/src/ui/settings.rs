// Settings view UI module.
//
// This file auto-loads the corresponding settings.dampen XML file.

use dampen_core::{AppState, HandlerRegistry};
use dampen_macros::{UiModel, dampen_ui};
use serde::{Deserialize, Serialize};

#[dampen_ui("settings.dampen")]
mod _settings {}

#[derive(Default, UiModel, Serialize, Deserialize, Clone, Debug)]
pub struct Model;

pub fn create_app_state() -> AppState<Model> {
    let document = _settings::document();
    let handler_registry = create_handler_registry();
    AppState::with_handlers(document, handler_registry)
}

pub fn create_handler_registry() -> HandlerRegistry {
    let registry = HandlerRegistry::new();

    registry.register_simple("switch_to_main", |_model: &mut dyn std::any::Any| {
        println!("Switching to main view");
    });

    registry
}
