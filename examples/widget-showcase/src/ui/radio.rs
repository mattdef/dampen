// Radio widget showcase UI module.
//
// This file auto-loads the corresponding radio.dampen XML file.

use dampen_core::{AppState, HandlerRegistry};
use dampen_macros::{UiModel, dampen_ui};
use serde::{Deserialize, Serialize};

#[derive(Default, UiModel, Serialize, Deserialize, Clone, Debug)]
pub struct Model {
    pub selected_size: Option<String>,
    pub selected_color: Option<String>,
    pub selected_plan: Option<String>,
    pub is_premium_disabled: bool,
}

#[dampen_ui("radio.dampen")]
mod _app {}

pub fn create_app_state() -> AppState<Model> {
    let document = _app::document();
    let handler_registry = create_handler_registry();
    AppState::with_handlers(document, handler_registry)
}

pub fn create_handler_registry() -> HandlerRegistry {
    let registry = HandlerRegistry::new();

    registry.register_with_value(
        "select_size",
        |model: &mut dyn std::any::Any, value: Box<dyn std::any::Any>| {
            let model = model.downcast_mut::<Model>().unwrap();
            if let Ok(val) = value.downcast::<String>() {
                model.selected_size = Some(*val);
            }
        },
    );

    registry.register_with_value(
        "select_color",
        |model: &mut dyn std::any::Any, value: Box<dyn std::any::Any>| {
            let model = model.downcast_mut::<Model>().unwrap();
            if let Ok(val) = value.downcast::<String>() {
                model.selected_color = Some(*val);
            }
        },
    );

    registry.register_with_value(
        "select_plan",
        |model: &mut dyn std::any::Any, value: Box<dyn std::any::Any>| {
            let model = model.downcast_mut::<Model>().unwrap();
            if let Ok(val) = value.downcast::<String>() {
                model.selected_plan = Some(*val);
            }
        },
    );

    registry.register_simple("toggle_premium", |model: &mut dyn std::any::Any| {
        let model = model.downcast_mut::<Model>().unwrap();
        model.is_premium_disabled = !model.is_premium_disabled;
    });

    registry.register_simple("switch_to_window", |_model: &mut dyn std::any::Any| {
        println!("Switching to main view");
    });

    registry
}
