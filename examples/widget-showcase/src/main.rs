use gravity_core::{parse, HandlerRegistry};
use gravity_iced::{GravityWidgetBuilder, HandlerMessage};
use gravity_macros::UiModel;
use iced::{Element, Task};
use serde::{Deserialize, Serialize};

/// Application model
#[derive(Default, UiModel, Serialize, Deserialize, Clone, Debug)]
struct Model;

/// Messages for application (using HandlerMessage from gravity-iced)
type Message = HandlerMessage;

/// Application state
struct AppState {
    model: Model,
    document: gravity_core::GravityDocument,
    handler_registry: HandlerRegistry,
}

impl AppState {
    fn new() -> Self {
        // Parse simple XML file
        let xml = r#"<?xml version="1.0" encoding="UTF-8" ?>
<text value="Widget Showcase - Coming Soon!" />"#;

        let document = parse(xml).expect("Failed to parse XML");

        // Create handler registry
        let handler_registry = HandlerRegistry::new();

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
        HandlerMessage::Handler(handler_name, _value_opt) => {
            // Simple handler lookup
            if let Some(_entry) = state.handler_registry.get(&handler_name) {
                // For now, just print
                println!("Handler called: {}", handler_name);
            }
            Task::none()
        }
    }
}

/// View function
fn view(state: &AppState) -> Element<'_, Message> {
    GravityWidgetBuilder::new(
        &state.document.root,
        &state.model,
        Some(&state.handler_registry),
    )
    .build()
}

/// Main function
pub fn main() -> iced::Result {
    iced::application(AppState::new, update, view).run()
}
