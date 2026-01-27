use dampen_core::{AppState, HandlerRegistry};
use dampen_macros::{UiModel, dampen_ui, inventory_handlers, ui_handler};
use serde::{Deserialize, Serialize};

#[dampen_ui("window.dampen")]
mod _app {}

#[derive(UiModel, Serialize, Deserialize, Default, Clone, Debug)]
pub struct Model {
    pub status: String,
}

#[ui_handler]
pub fn new_file(model: &mut Model) {
    model.status = "New File clicked".to_string();
    println!("Status: {}", model.status);
}

#[ui_handler]
pub fn exit_app(_model: &mut Model) {
    std::process::exit(0);
}

#[ui_handler]
pub fn action1(model: &mut Model) {
    model.status = "Action 1 triggered".to_string();
    println!("Status: {}", model.status);
}

#[ui_handler]
pub fn action2(model: &mut Model) {
    model.status = "Action 2 triggered".to_string();
    println!("Status: {}", model.status);
}

inventory_handlers! {
    new_file,
    exit_app,
    action1,
    action2
}

pub fn create_app_state() -> AppState<Model> {
    let document = _app::document();
    let handler_registry = create_handler_registry();
    AppState::with_handlers(document, handler_registry)
}

pub fn create_handler_registry() -> HandlerRegistry {
    let registry = HandlerRegistry::new();

    registry.register_simple("new_file", |model: &mut dyn std::any::Any| {
        if let Some(m) = model.downcast_mut::<Model>() {
            new_file(m);
        }
    });

    registry.register_simple("exit_app", |model: &mut dyn std::any::Any| {
        if let Some(m) = model.downcast_mut::<Model>() {
            exit_app(m);
        }
    });

    registry.register_simple("action1", |model: &mut dyn std::any::Any| {
        if let Some(m) = model.downcast_mut::<Model>() {
            action1(m);
        }
    });

    registry.register_simple("action2", |model: &mut dyn std::any::Any| {
        if let Some(m) = model.downcast_mut::<Model>() {
            action2(m);
        }
    });

    registry
}
