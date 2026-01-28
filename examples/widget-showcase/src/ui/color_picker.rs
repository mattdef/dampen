use crate::{CurrentView, Message};
use dampen_core::{AppState, HandlerRegistry};
use dampen_macros::{UiModel, dampen_ui, inventory_handlers, ui_handler};
use serde::{Deserialize, Serialize};

#[dampen_ui("color_picker.dampen")]
mod _app {}

#[derive(Default, UiModel, Serialize, Deserialize, Clone, Debug)]
pub struct Model {
    pub color1: String,
    pub color2: String,
    pub color3: String,
    pub color4: String,
    pub show_picker1: bool,
    pub show_picker2: bool,
    pub show_picker3: bool,
    pub show_picker4: bool,
}

#[ui_handler]
pub fn toggle_picker1(model: &mut Model) {
    model.show_picker1 = !model.show_picker1;
}

#[ui_handler]
pub fn toggle_picker2(model: &mut Model) {
    model.show_picker2 = !model.show_picker2;
}

#[ui_handler]
pub fn toggle_picker3(model: &mut Model) {
    model.show_picker3 = !model.show_picker3;
}

#[ui_handler]
pub fn toggle_picker4(model: &mut Model) {
    model.show_picker4 = !model.show_picker4;
}

#[ui_handler]
pub fn handle_color1_selected(model: &mut Model, color: &str) {
    model.color1 = color.to_string();
    model.show_picker1 = false;
}

#[ui_handler]
pub fn handle_color2_selected(model: &mut Model, color: &str) {
    model.color2 = color.to_string();
    model.show_picker2 = false;
}

#[ui_handler]
pub fn handle_color3_selected(model: &mut Model, color: &str) {
    model.color3 = color.to_string();
    model.show_picker3 = false;
}

#[ui_handler]
pub fn handle_color3_change(model: &mut Model, color: &str) {
    model.color3 = color.to_string();
}

#[ui_handler]
pub fn handle_color4_selected(model: &mut Model, color: &str) {
    model.color4 = color.to_string();
    model.show_picker4 = false;
}

#[ui_handler]
pub fn close_picker1(model: &mut Model) {
    model.show_picker1 = false;
}

#[ui_handler]
pub fn close_picker2(model: &mut Model) {
    model.show_picker2 = false;
}

#[ui_handler]
pub fn close_picker3(model: &mut Model) {
    model.show_picker3 = false;
}

inventory_handlers! {
    toggle_picker1,
    toggle_picker2,
    toggle_picker3,
    toggle_picker4,
    handle_color1_selected,
    handle_color2_selected,
    handle_color3_selected,
    handle_color3_change,
    handle_color4_selected,
    close_picker1,
    close_picker2,
    close_picker3
}

pub fn create_app_state() -> AppState<Model> {
    let document = _app::document();
    let handler_registry = create_handler_registry();
    let model = Model {
        color1: "#3498db".to_string(),
        color2: "#e74c3cff".to_string(),
        color3: "#2ecc71".to_string(),
        color4: "#4169e1".to_string(),
        ..Model::default()
    };
    AppState::with_all(document, model, handler_registry)
}

pub fn create_handler_registry() -> HandlerRegistry {
    let registry = HandlerRegistry::new();

    registry.register_simple("toggle_picker1", |m| {
        if let Some(m) = m.downcast_mut::<Model>() {
            toggle_picker1(m);
        }
    });
    registry.register_simple("toggle_picker2", |m| {
        if let Some(m) = m.downcast_mut::<Model>() {
            toggle_picker2(m);
        }
    });
    registry.register_simple("toggle_picker3", |m| {
        if let Some(m) = m.downcast_mut::<Model>() {
            toggle_picker3(m);
        }
    });
    registry.register_simple("toggle_picker4", |m| {
        if let Some(m) = m.downcast_mut::<Model>() {
            toggle_picker4(m);
        }
    });

    registry.register_with_value(
        "handle_color1_selected",
        |m: &mut dyn std::any::Any, p: Box<dyn std::any::Any>| {
            if let Some(m) = m.downcast_mut::<Model>()
                && let Some(s) = p.downcast_ref::<String>()
            {
                handle_color1_selected(m, s);
            }
        },
    );
    registry.register_with_value(
        "handle_color2_selected",
        |m: &mut dyn std::any::Any, p: Box<dyn std::any::Any>| {
            if let Some(m) = m.downcast_mut::<Model>()
                && let Some(s) = p.downcast_ref::<String>()
            {
                handle_color2_selected(m, s);
            }
        },
    );
    registry.register_with_value(
        "handle_color3_selected",
        |m: &mut dyn std::any::Any, p: Box<dyn std::any::Any>| {
            if let Some(m) = m.downcast_mut::<Model>()
                && let Some(s) = p.downcast_ref::<String>()
            {
                handle_color3_selected(m, s);
            }
        },
    );
    registry.register_with_value(
        "handle_color3_change",
        |m: &mut dyn std::any::Any, p: Box<dyn std::any::Any>| {
            if let Some(m) = m.downcast_mut::<Model>()
                && let Some(s) = p.downcast_ref::<String>()
            {
                handle_color3_change(m, s);
            }
        },
    );
    registry.register_with_value(
        "handle_color4_selected",
        |m: &mut dyn std::any::Any, p: Box<dyn std::any::Any>| {
            if let Some(m) = m.downcast_mut::<Model>()
                && let Some(s) = p.downcast_ref::<String>()
            {
                handle_color4_selected(m, s);
            }
        },
    );

    registry.register_simple("close_picker1", |m| {
        if let Some(m) = m.downcast_mut::<Model>() {
            close_picker1(m);
        }
    });
    registry.register_simple("close_picker2", |m| {
        if let Some(m) = m.downcast_mut::<Model>() {
            close_picker2(m);
        }
    });
    registry.register_simple("close_picker3", |m| {
        if let Some(m) = m.downcast_mut::<Model>() {
            close_picker3(m);
        }
    });

    registry.register_with_command("switch_to_window", |_model: &mut dyn std::any::Any| {
        Box::new(iced::Task::done(Message::SwitchToView(CurrentView::Window)))
    });

    registry
}
