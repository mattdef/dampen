// ComboBox widget showcase UI module.
//
// This file auto-loads the corresponding combobox.gravity XML file.

use gravity_core::{AppState, HandlerRegistry};
use gravity_macros::{gravity_ui, UiModel};
use serde::{Deserialize, Serialize};

#[derive(Default, UiModel, Serialize, Deserialize, Clone, Debug)]
pub struct Model {
    pub selected: String,
}

#[gravity_ui("combobox.gravity")]
mod _settings {}

pub fn create_app_state() -> AppState<Model> {
    let document = _settings::document();
    let handler_registry = create_handler_registry();
    AppState::with_handlers(document, handler_registry)
}

pub fn create_handler_registry() -> HandlerRegistry {
    let registry = HandlerRegistry::new();

    registry.register_simple("switch_to_window", |_model: &mut dyn std::any::Any| {
        println!("Switching to main view");
    });

    registry.register_with_value("select_category", |model: &mut dyn std::any::Any, value| {
        let model = model.downcast_mut::<Model>().unwrap();
        if let Ok(val) = value.downcast::<String>() {
            model.selected = (*val).clone();
        }
    });

    registry
}
