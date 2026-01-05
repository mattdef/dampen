use gravity_core::{parse, HandlerRegistry};
use gravity_iced::{GravityWidgetBuilder, HandlerMessage};
use gravity_macros::{ui_handler, UiModel};
use iced::{Element, Task};
use serde::{Deserialize, Serialize};
use std::any::Any;

/// Application state
#[derive(Default, UiModel, Serialize, Deserialize, Clone, Debug)]
struct Model {
    count: i32,
}

/// Messages for the application (using HandlerMessage from gravity-iced)
type Message = HandlerMessage;

/// Event handlers
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

/// Global state for the application
struct AppState {
    model: Model,
    document: gravity_core::GravityDocument,
    handler_registry: HandlerRegistry,
}

impl AppState {
    fn new() -> Self {
        // Parse the UI file
        let xml = include_str!("../ui/main.gravity");
        let document = parse(xml).expect("Failed to parse XML");

        // Create handler registry and register handlers
        let handler_registry = HandlerRegistry::new();

        // Register handlers manually
        handler_registry.register_simple("increment", |model: &mut dyn Any| {
            let model = model.downcast_mut::<Model>().unwrap();
            increment(model);
        });

        handler_registry.register_simple("decrement", |model: &mut dyn Any| {
            let model = model.downcast_mut::<Model>().unwrap();
            decrement(model);
        });

        handler_registry.register_simple("reset", |model: &mut dyn Any| {
            let model = model.downcast_mut::<Model>().unwrap();
            reset(model);
        });

        Self {
            model: Model::default(),
            document,
            handler_registry,
        }
    }
}

/// Update function
fn update(state: &mut AppState, message: Message) -> Task<Message> {
    match message {
        HandlerMessage::Handler(handler_name, _value) => {
            // Dispatch to handler registry
            if let Some(gravity_core::HandlerEntry::Simple(h)) =
                state.handler_registry.get(&handler_name)
            {
                h(&mut state.model);
            }
        }
    }
    Task::none()
}

/// View function using GravityWidgetBuilder
fn view(state: &AppState) -> Element<'_, Message> {
    GravityWidgetBuilder::new(&state.document, &state.model, Some(&state.handler_registry)).build()
}

pub fn main() -> iced::Result {
    iced::application(AppState::new, update, view).run()
}
