// Auto-loaded UI module for counter example.

use gravity_core::{AppState, HandlerRegistry};
use gravity_macros::{gravity_ui, ui_handler, UiModel};
use serde::{Deserialize, Serialize};

#[derive(Default, UiModel, Serialize, Deserialize, Clone, Debug)]
pub struct Model {
    pub count: i32,
}

#[ui_handler]
fn increment(model: &mut Model) {
    model.count += 1;
    println!("Incremented to: {}", model.count);
}

#[ui_handler]
fn decrement(model: &mut Model) {
    model.count -= 1;
    println!("Decremented to: {}", model.count);
}

#[ui_handler]
fn reset(model: &mut Model) {
    model.count = 0;
    println!("Reset to: {}", model.count);
}

#[gravity_ui("app.gravity")]
mod _app {}

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
