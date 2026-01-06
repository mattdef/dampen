// Auto-loaded UI module for hello-world example.
//
// This file is automatically compiled and loads the corresponding app.gravity XML file.

use gravity_core::{parse, AppState, GravityDocument, HandlerRegistry};
use gravity_macros::UiModel;
use serde::{Deserialize, Serialize};
use std::sync::LazyLock;

/// The application model.
#[derive(Default, UiModel, Serialize, Deserialize, Clone, Debug)]
pub struct Model;

fn __load_document() -> GravityDocument {
    let xml = include_str!("app.gravity");
    parse(xml).expect("Failed to parse Gravity UI file")
}

pub static DOCUMENT: LazyLock<GravityDocument> = LazyLock::new(__load_document);

/// Create the AppState for the hello-world example.
pub fn create_app_state() -> AppState<Model> {
    let document = (*DOCUMENT).clone();
    let handler_registry = create_handler_registry();
    AppState::with_handlers(document, handler_registry)
}

/// Create and configure the handler registry.
pub fn create_handler_registry() -> HandlerRegistry {
    let registry = HandlerRegistry::new();

    // Register the greet handler
    registry.register_simple("greet", |model: &mut dyn std::any::Any| {
        if let Some(m) = model.downcast_mut::<Model>() {
            println!("Button clicked! Model: {:?}", m);
        }
    });

    registry
}
