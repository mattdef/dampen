use gravity_core::{parse, HandlerRegistry};
use gravity_iced::{GravityWidgetBuilder, HandlerMessage};
use gravity_macros::{ui_handler, UiModel};
use iced::{Element, Task};
use serde::{Deserialize, Serialize};
use std::any::Any;

/// Application model with UiModel macro for data binding
#[derive(Default, UiModel, Serialize, Deserialize, Clone, Debug)]
struct Model {
    count: i32,
}

/// Messages for the application (using HandlerMessage from gravity-iced)
type Message = HandlerMessage;

/// Event handlers using ui_handler macro
#[ui_handler]
fn increment(model: &mut Model) {
    model.count += 1;
}

#[ui_handler]
fn decrement(model: &mut Model) {
    model.count -= 1;
}

#[ui_handler]
fn reset(model: &mut Model) {
    model.count = 0;
}

/// Global application state
struct AppState {
    model: Model,
    document: gravity_core::GravityDocument,
    handler_registry: HandlerRegistry,
}

impl AppState {
    fn new() -> Self {
        let ui_path = std::path::PathBuf::from("examples/styling/ui/main.gravity");
        let xml = match std::fs::read_to_string(&ui_path) {
            Ok(content) => content,
            Err(e) => {
                eprintln!("Error: Failed to read UI file: {}", e);
                r#"<column padding="40" spacing="20">
                    <text value="Error: Could not load ui/main.gravity" size="18" />
                </column>"#
                    .to_string()
            }
        };

        let document = parse(&xml).unwrap_or_else(|e| {
            eprintln!("Error: Failed to parse UI: {}", e);
            gravity_core::GravityDocument::default()
        });

        let handler_registry = HandlerRegistry::new();

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
