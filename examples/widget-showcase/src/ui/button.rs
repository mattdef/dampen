// Button widget showcase UI module.
//
// This file auto-loads the corresponding button.dampen XML file.

use dampen_core::{AppState, HandlerRegistry};
use dampen_macros::{UiModel, dampen_ui};
use serde::{Deserialize, Serialize};

#[derive(Default, UiModel, Serialize, Deserialize, Clone, Debug)]
pub struct Model {
    pub click_count: i32,
}

#[dampen_ui("button.dampen")]
mod _app {}

pub fn create_app_state() -> AppState<Model> {
    let document = _app::document();
    let handler_registry = create_handler_registry();
    AppState::with_handlers(document, handler_registry)
}

pub fn create_handler_registry() -> HandlerRegistry {
    let registry = HandlerRegistry::new();

    registry.register_simple("handle_click", |_model: &mut dyn std::any::Any| {
        println!("Button clicked!");
    });

    registry.register_simple("increment_count", |model: &mut dyn std::any::Any| {
        let model = model.downcast_mut::<Model>().unwrap();
        model.click_count += 1;
        println!("Count: {}", model.click_count);
    });

    registry.register_simple("switch_to_window", |_model: &mut dyn std::any::Any| {
        println!("Switching to main view");
    });

    registry
}
