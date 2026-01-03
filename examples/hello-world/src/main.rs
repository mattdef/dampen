use gravity_core::{parse, HandlerRegistry};
use gravity_iced::{GravityWidgetBuilder, HandlerMessage};
use gravity_macros::UiModel;
use iced::{Element, Task};
use serde::{Deserialize, Serialize};
use std::any::Any;

/// Application model (empty for this example)
#[derive(Default, UiModel, Serialize, Deserialize, Clone, Debug)]
struct Model;

/// Messages for the application (using HandlerMessage from gravity-iced)
type Message = HandlerMessage;

/// Application state
struct AppState {
    model: Model,
    document: gravity_core::GravityDocument,
    handler_registry: HandlerRegistry,
}

impl AppState {
    fn new() -> Self {
        // Load and parse the XML file
        let xml = include_str!("../ui/main.gravity");
        let document = parse(xml).expect("Failed to parse XML");

        // Create handler registry
        let handler_registry = HandlerRegistry::new();

        // Register greet handler
        handler_registry.register_simple("greet", |_model: &mut dyn Any| {
            println!("Button clicked!");
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
    GravityWidgetBuilder::new(
        &state.document.root,
        &state.model,
        Some(&state.handler_registry),
    )
    .build()
}

pub fn main() -> iced::Result {
    iced::application(AppState::new, update, view).run()
}
