// Main view UI module.
//
// This file auto-loads the corresponding window.gravity XML file.

use gravity_core::{AppState, HandlerRegistry};
use gravity_macros::{gravity_ui, UiModel};
use serde::{Deserialize, Serialize};

#[derive(Default, UiModel, Serialize, Deserialize, Clone, Debug)]
pub struct Model;

#[gravity_ui("window.gravity")]
mod _app {}

pub fn create_app_state() -> AppState<Model> {
    let document = _app::document();
    let handler_registry = create_handler_registry();
    AppState::with_handlers(document, handler_registry)
}

pub fn create_handler_registry() -> HandlerRegistry {
    let registry = HandlerRegistry::new();

    registry.register_simple("switch_to_button", |_model: &mut dyn std::any::Any| {
        println!("Switching to button view");
    });

    registry.register_simple("switch_to_text", |_model: &mut dyn std::any::Any| {
        println!("Switching to text view");
    });

    registry.register_simple("switch_to_textinput", |_model: &mut dyn std::any::Any| {
        println!("Switching to textinput view");
    });

    registry.register_simple("switch_to_checkbox", |_model: &mut dyn std::any::Any| {
        println!("Switching to checkbox view");
    });

    registry.register_simple("switch_to_slider", |_model: &mut dyn std::any::Any| {
        println!("Switching to slider view");
    });

    registry.register_simple("switch_to_toggler", |_model: &mut dyn std::any::Any| {
        println!("Switching to toggler view");
    });

    registry.register_simple("switch_to_image", |_model: &mut dyn std::any::Any| {
        println!("Switching to image view");
    });

    registry.register_simple("switch_to_svg", |_model: &mut dyn std::any::Any| {
        println!("Switching to svg view");
    });

    registry.register_simple("switch_to_scrollable", |_model: &mut dyn std::any::Any| {
        println!("Switching to scrollable view");
    });

    registry.register_simple("switch_to_stack", |_model: &mut dyn std::any::Any| {
        println!("Switching to stack view");
    });

    registry.register_simple("switch_to_space", |_model: &mut dyn std::any::Any| {
        println!("Switching to space view");
    });

    registry.register_simple("switch_to_layout", |_model: &mut dyn std::any::Any| {
        println!("Switching to layout view");
    });

    registry.register_simple("switch_to_for", |_model: &mut dyn std::any::Any| {
        println!("Switching to for loop view");
    });

    registry.register_simple("switch_to_combobox", |_model: &mut dyn std::any::Any| {
        println!("Switching to combobox view");
    });

    registry.register_simple("switch_to_grid", |_model: &mut dyn std::any::Any| {
        println!("Switching to grid view");
    });

    registry.register_simple("switch_to_picklist", |_model: &mut dyn std::any::Any| {
        println!("Switching to picklist view");
    });

    registry.register_simple("switch_to_progressbar", |_model: &mut dyn std::any::Any| {
        println!("Switching to progressbar view");
    });

    registry.register_simple("switch_to_tooltip", |_model: &mut dyn std::any::Any| {
        println!("Switching to tooltip view");
    });

    registry
}
