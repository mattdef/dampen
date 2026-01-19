// Main view UI module.
//
// This file auto-loads the corresponding app.dampen XML file.

use dampen_core::{AppState, HandlerRegistry};
use dampen_macros::{UiModel, dampen_ui, inventory_handlers, ui_handler};
use serde::{Deserialize, Serialize};

#[dampen_ui("window.dampen")]
mod _app {}

#[derive(Default, UiModel, Serialize, Deserialize, Clone, Debug)]
pub struct Model;

#[ui_handler]
pub fn switch_to_settings() -> iced::Task<crate::Message> {
    use crate::{CurrentView, Message};
    println!("Switching to settings view");
    iced::Task::done(Message::SwitchToView(CurrentView::Settings))
}

inventory_handlers! {
    switch_to_settings
}

pub fn create_app_state() -> AppState<Model> {
    let document = _app::document();
    let handler_registry = create_handler_registry();
    AppState::with_handlers(document, handler_registry)
}

pub fn create_handler_registry() -> HandlerRegistry {
    use crate::{CurrentView, Message};

    let registry = HandlerRegistry::new();

    registry.register_with_command("switch_to_settings", |_model: &mut dyn std::any::Any| {
        println!("Switching to settings view");
        Box::new(iced::Task::done(Message::SwitchToView(
            CurrentView::Settings,
        )))
    });

    registry
}
