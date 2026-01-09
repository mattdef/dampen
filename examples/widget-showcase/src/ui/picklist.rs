// PickList widget showcase UI module.
//
// This file auto-loads the corresponding picklist.dampen XML file.

use dampen_core::{AppState, HandlerRegistry};
use dampen_macros::{dampen_ui, UiModel};
use serde::{Deserialize, Serialize};

#[derive(Default, UiModel, Serialize, Deserialize, Clone, Debug)]
pub struct Model {
    pub filter: String,
}

#[dampen_ui("picklist.dampen")]
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

    registry.register_simple("switch_to_combobox", |_model: &mut dyn std::any::Any| {
        println!("Switching to combobox view");
    });

    registry.register_simple("switch_to_progressbar", |_model: &mut dyn std::any::Any| {
        println!("Switching to progressbar view");
    });

    registry.register_simple("switch_to_tooltip", |_model: &mut dyn std::any::Any| {
        println!("Switching to tooltip view");
    });

    registry.register_simple("switch_to_grid", |_model: &mut dyn std::any::Any| {
        println!("Switching to grid view");
    });

    registry.register_with_value("update_filter", |model: &mut dyn std::any::Any, value| {
        let model = model.downcast_mut::<Model>().unwrap();
        if let Ok(val) = value.downcast::<String>() {
            model.filter = (*val).clone();
        }
    });

    registry
}
