// Toggler widget showcase UI module.
//
// This file auto-loads the corresponding toggler.gravity XML file.

use gravity_core::{AppState, HandlerRegistry};
use gravity_macros::{gravity_ui, UiModel};
use serde::{Deserialize, Serialize};

#[derive(Default, UiModel, Serialize, Deserialize, Clone, Debug)]
pub struct Model {
    pub dark_mode: bool,
    pub notifications: bool,
    pub auto_save: bool,
    pub is_active: bool,
    pub wifi: bool,
    pub bluetooth: bool,
    pub airplane: bool,
}

#[gravity_ui("toggler.gravity")]
mod _app {}

pub fn create_app_state() -> AppState<Model> {
    let document = _app::document();
    let handler_registry = create_handler_registry();
    AppState::with_handlers(document, handler_registry)
}

pub fn create_handler_registry() -> HandlerRegistry {
    let registry = HandlerRegistry::new();

    registry.register_simple("toggle_dark", |model: &mut dyn std::any::Any| {
        let model = model.downcast_mut::<Model>().unwrap();
        model.dark_mode = !model.dark_mode;
    });

    registry.register_simple("toggle_notifications", |model: &mut dyn std::any::Any| {
        let model = model.downcast_mut::<Model>().unwrap();
        model.notifications = !model.notifications;
    });

    registry.register_simple("toggle_autosave", |model: &mut dyn std::any::Any| {
        let model = model.downcast_mut::<Model>().unwrap();
        model.auto_save = !model.auto_save;
    });

    registry.register_simple("toggle_active", |model: &mut dyn std::any::Any| {
        let model = model.downcast_mut::<Model>().unwrap();
        model.is_active = !model.is_active;
    });

    registry.register_simple("toggle_wifi", |model: &mut dyn std::any::Any| {
        let model = model.downcast_mut::<Model>().unwrap();
        model.wifi = !model.wifi;
    });

    registry.register_simple("toggle_bluetooth", |model: &mut dyn std::any::Any| {
        let model = model.downcast_mut::<Model>().unwrap();
        model.bluetooth = !model.bluetooth;
    });

    registry.register_simple("toggle_airplane", |model: &mut dyn std::any::Any| {
        let model = model.downcast_mut::<Model>().unwrap();
        model.airplane = !model.airplane;
    });

    registry.register_simple("switch_to_window", |_model: &mut dyn std::any::Any| {
        println!("Switching to main view");
    });

    registry
}
