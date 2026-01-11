// TextInput widget showcase UI module.
//
// This file auto-loads the corresponding textinput.dampen XML file.

use dampen_core::{AppState, HandlerRegistry};
use dampen_macros::{UiModel, dampen_ui};
use serde::{Deserialize, Serialize};

#[derive(Default, UiModel, Serialize, Deserialize, Clone, Debug)]
pub struct Model {
    pub name: String,
    pub email: String,
    pub password: String,
    pub search_text: String,
}

#[dampen_ui("textinput.dampen")]
mod _app {}

pub fn create_app_state() -> AppState<Model> {
    let document = _app::document();
    let handler_registry = create_handler_registry();
    AppState::with_handlers(document, handler_registry)
}

pub fn create_handler_registry() -> HandlerRegistry {
    let registry = HandlerRegistry::new();

    registry.register_with_value("update_name", |model: &mut dyn std::any::Any, value| {
        let model = model.downcast_mut::<Model>().unwrap();
        if let Ok(name) = value.downcast::<String>() {
            model.name = *name;
        }
    });

    registry.register_with_value("update_email", |model: &mut dyn std::any::Any, value| {
        let model = model.downcast_mut::<Model>().unwrap();
        if let Ok(email) = value.downcast::<String>() {
            model.email = *email;
        }
    });

    registry.register_with_value("update_search", |model: &mut dyn std::any::Any, value| {
        let model = model.downcast_mut::<Model>().unwrap();
        if let Ok(search) = value.downcast::<String>() {
            model.search_text = *search;
        }
    });

    registry.register_with_value("update_password", |model: &mut dyn std::any::Any, value| {
        let model = model.downcast_mut::<Model>().unwrap();
        if let Ok(pwd) = value.downcast::<String>() {
            model.password = *pwd;
        }
    });

    registry.register_simple("switch_to_window", |_model: &mut dyn std::any::Any| {
        println!("Switching to main view");
    });

    registry
}
