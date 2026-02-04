//! TabBar widget demo handlers
#![allow(dead_code)]

use crate::{CurrentView, Message};
use dampen_core::{AppState, HandlerRegistry};
use dampen_macros::{UiModel, dampen_ui};
use serde::{Deserialize, Serialize};

#[derive(Default, UiModel, Serialize, Deserialize, Clone, Debug)]
pub struct Model {
    pub selected_tab: i32,
    pub auto_save: bool,
    pub show_welcome: bool,
    pub theme: String,
    pub accent_color: String,
    pub email_notifications: bool,
    pub push_notifications: bool,
    pub sms_notifications: bool,
}

#[dampen_ui("tab_bar.dampen")]
mod _app {}

pub fn create_app_state() -> AppState<Model> {
    let document = _app::document();
    let handler_registry = create_handler_registry();
    AppState::with_handlers(document, handler_registry)
}

pub fn create_handler_registry() -> HandlerRegistry {
    let registry = HandlerRegistry::new();

    registry.register_with_value(
        "on_tab_selected",
        |model: &mut dyn std::any::Any, value: Box<dyn std::any::Any>| {
            let model = model.downcast_mut::<Model>().unwrap();
            if let Some(idx_str) = value.downcast_ref::<String>()
                && let Ok(idx) = idx_str.parse::<i32>()
            {
                model.selected_tab = idx;
                println!("Tab selected: {}", model.selected_tab);
            }
        },
    );

    registry.register_simple("toggle_auto_save", |model: &mut dyn std::any::Any| {
        let model = model.downcast_mut::<Model>().unwrap();
        model.auto_save = !model.auto_save;
        println!("Auto-save: {}", model.auto_save);
    });

    registry.register_simple("toggle_welcome", |model: &mut dyn std::any::Any| {
        let model = model.downcast_mut::<Model>().unwrap();
        model.show_welcome = !model.show_welcome;
        println!("Show welcome: {}", model.show_welcome);
    });

    registry.register_simple("change_theme", |_model: &mut dyn std::any::Any| {
        println!("Theme changed");
    });

    registry.register_simple("change_accent", |_model: &mut dyn std::any::Any| {
        println!("Accent color changed");
    });

    registry.register_simple("toggle_email", |model: &mut dyn std::any::Any| {
        let model = model.downcast_mut::<Model>().unwrap();
        model.email_notifications = !model.email_notifications;
        println!("Email notifications: {}", model.email_notifications);
    });

    registry.register_simple("toggle_push", |model: &mut dyn std::any::Any| {
        let model = model.downcast_mut::<Model>().unwrap();
        model.push_notifications = !model.push_notifications;
        println!("Push notifications: {}", model.push_notifications);
    });

    registry.register_simple("toggle_sms", |model: &mut dyn std::any::Any| {
        let model = model.downcast_mut::<Model>().unwrap();
        model.sms_notifications = !model.sms_notifications;
        println!("SMS notifications: {}", model.sms_notifications);
    });

    registry.register_with_command("switch_to_window", |_model: &mut dyn std::any::Any| {
        Box::new(iced::Task::done(Message::SwitchToView(CurrentView::Window)))
    });

    registry
}
