// Auto-loaded UI module for counter example.

use dampen_core::{AppState, HandlerRegistry};
use dampen_macros::{UiModel, dampen_ui, inventory_handlers, ui_handler};
use serde::{Deserialize, Serialize};

#[dampen_ui("window.dampen")]
mod _app {}

#[derive(Default, UiModel, Serialize, Deserialize, Clone, Debug)]
pub struct Model {
    pub count: i32,
}

#[ui_handler]
pub fn increment(model: &mut Model) {
    model.count += 1;
    println!("Incremented to: {}", model.count);
}

#[ui_handler]
pub fn decrement(model: &mut Model) {
    model.count -= 1;
    println!("Decremented to: {}", model.count);
}

#[ui_handler]
pub fn reset(model: &mut Model) {
    model.count = 0;
    println!("Reset to: {}", model.count);
}

// Declare all handlers in this module for build-time code generation
inventory_handlers! {
    increment,
    decrement,
    reset
}

pub fn create_app_state() -> AppState<Model> {
    let document = _app::document();
    let handler_registry = create_handler_registry();
    AppState::with_handlers(document, handler_registry)
}

pub fn create_handler_registry() -> HandlerRegistry {
    let registry = HandlerRegistry::new();

    registry.register_simple("increment", |model: &mut dyn std::any::Any| {
        if let Some(m) = model.downcast_mut::<Model>() {
            increment(m);
        }
    });

    registry.register_simple("decrement", |model: &mut dyn std::any::Any| {
        if let Some(m) = model.downcast_mut::<Model>() {
            decrement(m);
        }
    });

    registry.register_simple("reset", |model: &mut dyn std::any::Any| {
        if let Some(m) = model.downcast_mut::<Model>() {
            reset(m);
        }
    });

    registry
}
