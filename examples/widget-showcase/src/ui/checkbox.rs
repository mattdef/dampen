// Checkbox widget showcase UI module.
//
// This file auto-loads the corresponding checkbox.dampen XML file.

use dampen_core::{AppState, HandlerRegistry};
use dampen_macros::{dampen_ui, UiModel};
use serde::{Deserialize, Serialize};

#[derive(Default, UiModel, Serialize, Deserialize, Clone, Debug)]
pub struct Model {
    pub option1: bool,
    pub option2: bool,
    pub option3: bool,
    pub enable_feature: bool,
    pub agree_terms: bool,
    pub newsletter: bool,
}

#[dampen_ui("checkbox.dampen")]
mod _app {}

pub fn create_app_state() -> AppState<Model> {
    let document = _app::document();
    let handler_registry = create_handler_registry();
    AppState::with_handlers(document, handler_registry)
}

pub fn create_handler_registry() -> HandlerRegistry {
    let registry = HandlerRegistry::new();

    registry.register_simple("toggle_option1", |model: &mut dyn std::any::Any| {
        let model = model.downcast_mut::<Model>().unwrap();
        model.option1 = !model.option1;
    });

    registry.register_simple("toggle_option2", |model: &mut dyn std::any::Any| {
        let model = model.downcast_mut::<Model>().unwrap();
        model.option2 = !model.option2;
    });

    registry.register_simple("toggle_option3", |model: &mut dyn std::any::Any| {
        let model = model.downcast_mut::<Model>().unwrap();
        model.option3 = !model.option3;
    });

    registry.register_simple("toggle_feature", |model: &mut dyn std::any::Any| {
        let model = model.downcast_mut::<Model>().unwrap();
        model.enable_feature = !model.enable_feature;
        println!("Feature enabled: {}", model.enable_feature);
    });

    registry.register_simple("toggle_terms", |model: &mut dyn std::any::Any| {
        let model = model.downcast_mut::<Model>().unwrap();
        model.agree_terms = !model.agree_terms;
    });

    registry.register_simple("toggle_newsletter", |model: &mut dyn std::any::Any| {
        let model = model.downcast_mut::<Model>().unwrap();
        model.newsletter = !model.newsletter;
    });

    registry.register_simple("submit_form", |_model: &mut dyn std::any::Any| {
        println!("Form submitted!");
    });

    registry.register_simple("switch_to_window", |_model: &mut dyn std::any::Any| {
        println!("Switching to main view");
    });

    registry
}
