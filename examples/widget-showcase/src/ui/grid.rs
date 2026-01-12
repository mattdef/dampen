// Grid widget showcase UI module.
//
// This file auto-loads the corresponding grid.dampen XML file.

use crate::{CurrentView, Message};
use dampen_core::{AppState, HandlerRegistry};
use dampen_macros::{UiModel, dampen_ui};
use serde::{Deserialize, Serialize};

#[derive(Default, UiModel, Serialize, Deserialize, Clone, Debug)]
pub struct Model;

#[dampen_ui("grid.dampen")]
mod _app {}

pub fn create_app_state() -> AppState<Model> {
    let document = _app::document();
    let handler_registry = create_handler_registry();
    AppState::with_handlers(document, handler_registry)
}

pub fn create_handler_registry() -> HandlerRegistry {
    let registry = HandlerRegistry::new();

    registry.register_with_command("switch_to_window", |_model: &mut dyn std::any::Any| {
        Box::new(iced::Task::done(Message::SwitchToView(CurrentView::Window)))
    });

    registry
}
