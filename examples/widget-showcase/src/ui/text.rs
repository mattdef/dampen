// Text widget showcase UI module.
//
// This file auto-loads the corresponding text.dampen XML file.

use crate::{CurrentView, Message};
use dampen_core::{AppState, HandlerRegistry};
use dampen_macros::{UiModel, dampen_ui};
use serde::{Deserialize, Serialize};

#[derive(Default, UiModel, Serialize, Deserialize, Clone, Debug)]
pub struct Model {
    pub name: String,
    pub count: i32,
    pub status: String,
    pub status_color: String,
    pub is_active: bool,
}

#[dampen_ui("text.dampen")]
mod _app {}

pub fn create_app_state() -> AppState<Model> {
    let document = _app::document();
    let handler_registry = create_handler_registry();

    // Créer le model avec des valeurs initiales
    let model = Model {
        name: "Matt".to_string(),
        count: 3,
        status: "active".to_string(),
        status_color: "green".to_string(),
        is_active: true,
    };

    AppState::with_all(document, model, handler_registry)
}

pub fn create_handler_registry() -> HandlerRegistry {
    let registry = HandlerRegistry::new();

    registry.register_with_command("switch_to_window", |_model: &mut dyn std::any::Any| {
        Box::new(iced::Task::done(Message::SwitchToView(CurrentView::Window)))
    });

    registry
}
