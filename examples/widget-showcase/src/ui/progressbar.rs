// ProgressBar widget showcase UI module.
//
// This file auto-loads the corresponding progressbar.dampen XML file.

use crate::{CurrentView, Message};
use dampen_core::{AppState, HandlerRegistry};
use dampen_macros::{UiModel, dampen_ui};
use serde::{Deserialize, Serialize};

#[derive(Default, UiModel, Serialize, Deserialize, Clone, Debug)]
pub struct Model {
    pub primary_progress: f32,
    pub success_progress: f32,
    pub warning_progress: f32,
    pub danger_progress: f32,
    pub secondary_progress: f32,
    pub custom_progress: f32,
}

#[dampen_ui("progressbar.dampen")]
mod _app {}

pub fn create_app_state() -> AppState<Model> {
    let document = _app::document();
    let handler_registry = create_handler_registry();
    AppState::with_handlers(document, handler_registry)
}

pub fn create_handler_registry() -> HandlerRegistry {
    let registry = HandlerRegistry::new();

    registry.register_simple("increment_progress", |model: &mut dyn std::any::Any| {
        let model = model.downcast_mut::<Model>().unwrap();
        model.primary_progress = (model.primary_progress + 10.0).min(100.0);
        model.success_progress = (model.success_progress + 15.0).min(100.0);
        model.warning_progress = (model.warning_progress + 20.0).min(100.0);
        model.danger_progress = (model.danger_progress + 25.0).min(100.0);
        model.secondary_progress = (model.secondary_progress + 5.0).min(100.0);
        model.custom_progress = (model.custom_progress + 5.0).min(50.0);
    });

    registry.register_simple("reset_progress", |model: &mut dyn std::any::Any| {
        let model = model.downcast_mut::<Model>().unwrap();
        model.primary_progress = 0.0;
        model.success_progress = 0.0;
        model.warning_progress = 0.0;
        model.danger_progress = 0.0;
        model.secondary_progress = 0.0;
        model.custom_progress = 0.0;
    });

    registry.register_with_command("switch_to_window", |_model: &mut dyn std::any::Any| {
        Box::new(iced::Task::done(Message::SwitchToView(CurrentView::Window)))
    });

    registry
}
