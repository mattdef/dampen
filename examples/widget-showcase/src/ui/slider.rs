// Slider widget showcase UI module.
//
// This file auto-loads the corresponding slider.dampen XML file.

use crate::{CurrentView, Message};
use dampen_core::{AppState, HandlerRegistry};
use dampen_macros::{UiModel, dampen_ui};
use serde::{Deserialize, Serialize};

#[derive(Default, UiModel, Serialize, Deserialize, Clone, Debug)]
pub struct Model {
    pub slider_value: f32,
    pub volume: f32,
    pub temperature: f32,
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}

#[dampen_ui("slider.dampen")]
mod _app {}

pub fn create_app_state() -> AppState<Model> {
    let document = _app::document();
    let handler_registry = create_handler_registry();
    AppState::with_handlers(document, handler_registry)
}

pub fn create_handler_registry() -> HandlerRegistry {
    let registry = HandlerRegistry::new();

    registry.register_with_value(
        "update_slider",
        |model: &mut dyn std::any::Any, value: Box<dyn std::any::Any>| {
            let model = model.downcast_mut::<Model>().unwrap();
            if let Ok(val) = value.downcast::<String>() {
                if let Ok(new_value) = val.parse::<f32>() {
                    model.slider_value = new_value;
                }
            }
        },
    );

    registry.register_with_value(
        "update_volume",
        |model: &mut dyn std::any::Any, value: Box<dyn std::any::Any>| {
            let model = model.downcast_mut::<Model>().unwrap();
            if let Ok(val) = value.downcast::<String>() {
                if let Ok(new_value) = val.parse::<f32>() {
                    model.volume = new_value;
                }
            }
        },
    );

    registry.register_with_value(
        "update_temperature",
        |model: &mut dyn std::any::Any, value: Box<dyn std::any::Any>| {
            let model = model.downcast_mut::<Model>().unwrap();
            if let Ok(val) = value.downcast::<String>() {
                if let Ok(new_value) = val.parse::<f32>() {
                    model.temperature = new_value;
                }
            }
        },
    );

    registry.register_with_value(
        "update_red",
        |model: &mut dyn std::any::Any, value: Box<dyn std::any::Any>| {
            let model = model.downcast_mut::<Model>().unwrap();
            if let Ok(val) = value.downcast::<String>() {
                if let Ok(new_value) = val.parse::<f32>() {
                    model.red = new_value;
                }
            }
        },
    );

    registry.register_with_value(
        "update_green",
        |model: &mut dyn std::any::Any, value: Box<dyn std::any::Any>| {
            let model = model.downcast_mut::<Model>().unwrap();
            if let Ok(val) = value.downcast::<String>() {
                if let Ok(new_value) = val.parse::<f32>() {
                    model.green = new_value;
                }
            }
        },
    );

    registry.register_with_value(
        "update_blue",
        |model: &mut dyn std::any::Any, value: Box<dyn std::any::Any>| {
            let model = model.downcast_mut::<Model>().unwrap();
            if let Ok(val) = value.downcast::<String>() {
                if let Ok(new_value) = val.parse::<f32>() {
                    model.blue = new_value;
                }
            }
        },
    );

    registry.register_with_command("switch_to_window", |_model: &mut dyn std::any::Any| {
        Box::new(iced::Task::done(Message::SwitchToView(CurrentView::Window)))
    });

    registry
}
