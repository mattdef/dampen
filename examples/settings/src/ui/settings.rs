// Settings view UI module.
//
// This file auto-loads the corresponding settings.dampen XML file.

use dampen_core::{AppState, HandlerRegistry};
use dampen_macros::{UiModel, dampen_ui, inventory_handlers, ui_handler};
use serde::{Deserialize, Serialize};

#[dampen_ui("settings.dampen")]
mod _settings {}

#[derive(Default, UiModel, Serialize, Deserialize, Clone, Debug)]
pub struct Model;

#[ui_handler]
pub fn switch_to_main() -> iced::Task<crate::Message> {
    use crate::{CurrentView, Message};
    println!("Switching to main view");
    iced::Task::done(Message::SwitchToView(CurrentView::Window))
}

inventory_handlers! {
    switch_to_main
}

pub fn create_app_state() -> AppState<Model> {
    let document = _settings::document();
    let handler_registry = create_handler_registry();
    AppState::with_handlers(document, handler_registry)
}

pub fn create_handler_registry() -> HandlerRegistry {
    use crate::{CurrentView, Message};

    let registry = HandlerRegistry::new();

    registry.register_with_command("switch_to_main", |_model: &mut dyn std::any::Any| {
        println!("Switching to main view");
        Box::new(iced::Task::done(Message::SwitchToView(CurrentView::Window)))
    });

    registry
}
