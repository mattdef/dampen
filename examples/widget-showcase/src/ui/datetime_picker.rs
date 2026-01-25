use crate::{CurrentView, Message};
use dampen_core::{AppState, HandlerRegistry};
use dampen_macros::{UiModel, dampen_ui, inventory_handlers, ui_handler};
use serde::{Deserialize, Serialize};

#[dampen_ui("datetime_picker.dampen")]
mod _app {}

#[derive(Default, UiModel, Serialize, Deserialize, Clone, Debug)]
pub struct Model {
    pub selected_date: String,
    pub selected_time: String,
    pub show_date_picker: bool,
    pub show_time_picker: bool,
}

#[ui_handler]
pub fn toggle_date_picker(model: &mut Model) {
    model.show_date_picker = !model.show_date_picker;
}

#[ui_handler]
pub fn toggle_time_picker(model: &mut Model) {
    model.show_time_picker = !model.show_time_picker;
}

#[ui_handler]
pub fn handle_date_selected(model: &mut Model, date: &str) {
    model.selected_date = date.to_string();
    model.show_date_picker = false;
}

#[ui_handler]
pub fn handle_time_selected(model: &mut Model, time: &str) {
    model.selected_time = time.to_string();
    model.show_time_picker = false;
}

#[ui_handler]
pub fn close_date_picker(model: &mut Model) {
    model.show_date_picker = false;
}

#[ui_handler]
pub fn close_time_picker(model: &mut Model) {
    model.show_time_picker = false;
}

inventory_handlers! {
    toggle_date_picker,
    toggle_time_picker,
    handle_date_selected,
    handle_time_selected,
    close_date_picker,
    close_time_picker
}

pub fn create_app_state() -> AppState<Model> {
    let document = _app::document();
    let handler_registry = create_handler_registry();
    let model = Model {
        selected_date: "2026-01-25".to_string(),
        selected_time: "12:00:00".to_string(),
        ..Model::default()
    };
    AppState::with_all(document, model, handler_registry)
}

pub fn create_handler_registry() -> HandlerRegistry {
    let registry = HandlerRegistry::new();

    registry.register_simple("toggle_date_picker", |m| {
        if let Some(m) = m.downcast_mut::<Model>() {
            toggle_date_picker(m);
        }
    });
    registry.register_simple("toggle_time_picker", |m| {
        if let Some(m) = m.downcast_mut::<Model>() {
            toggle_time_picker(m);
        }
    });
    registry.register_with_value(
        "handle_date_selected",
        |m: &mut dyn std::any::Any, p: Box<dyn std::any::Any>| {
            if let Some(m) = m.downcast_mut::<Model>()
                && let Some(s) = p.downcast_ref::<String>()
            {
                handle_date_selected(m, s);
            }
        },
    );
    registry.register_with_value(
        "handle_time_selected",
        |m: &mut dyn std::any::Any, p: Box<dyn std::any::Any>| {
            if let Some(m) = m.downcast_mut::<Model>()
                && let Some(s) = p.downcast_ref::<String>()
            {
                handle_time_selected(m, s);
            }
        },
    );
    registry.register_simple("close_date_picker", |m| {
        if let Some(m) = m.downcast_mut::<Model>() {
            close_date_picker(m);
        }
    });
    registry.register_simple("close_time_picker", |m| {
        if let Some(m) = m.downcast_mut::<Model>() {
            close_time_picker(m);
        }
    });

    registry.register_with_command("switch_to_window", |_model: &mut dyn std::any::Any| {
        Box::new(iced::Task::done(Message::SwitchToView(CurrentView::Window)))
    });

    registry
}
