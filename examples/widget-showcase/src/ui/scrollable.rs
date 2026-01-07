// Scrollable widget showcase UI module.
//
// This file auto-loads the corresponding scrollable.gravity XML file.

use gravity_core::{AppState, HandlerRegistry};
use gravity_macros::{gravity_ui, UiModel};
use serde::{Deserialize, Serialize};

#[derive(Default, UiModel, Serialize, Deserialize, Clone, Debug)]
pub struct Model;

#[gravity_ui("scrollable.gravity")]
mod _app {}

pub fn create_app_state() -> AppState<Model> {
    let document = _app::document();
    let handler_registry = create_handler_registry();
    AppState::with_handlers(document, handler_registry)
}

pub fn create_handler_registry() -> HandlerRegistry {
    let registry = HandlerRegistry::new();

    registry.register_simple("switch_to_window", |_model: &mut dyn std::any::Any| {
        println!("Switching to main view");
    });

    registry.register_simple("click_button1", |_model: &mut dyn std::any::Any| {
        println!("Button 1 clicked!");
    });

    registry.register_simple("click_button2", |_model: &mut dyn std::any::Any| {
        println!("Button 2 clicked!");
    });

    registry.register_simple("click_button3", |_model: &mut dyn std::any::Any| {
        println!("Button 3 clicked!");
    });

    registry.register_simple("click_button4", |_model: &mut dyn std::any::Any| {
        println!("Button 4 clicked!");
    });

    registry.register_simple("click_button5", |_model: &mut dyn std::any::Any| {
        println!("Button 5 clicked!");
    });

    registry.register_simple("click_button6", |_model: &mut dyn std::any::Any| {
        println!("Button 6 clicked!");
    });

    registry
}
