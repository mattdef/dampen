use gravity_core::{parse, HandlerRegistry};
use gravity_iced::{GravityWidgetBuilder, HandlerMessage};
use gravity_macros::{ui_handler, UiModel};
use iced::{Element, Task};
use serde::{Deserialize, Serialize};
use std::any::Any;
use std::time::SystemTime;

/// Application model with comprehensive state
#[derive(UiModel, Serialize, Deserialize, Clone, Debug)]
struct Model {
    // Counter state
    count: i32,

    // Text input state
    user_text: String,

    // List management
    new_item: String,
    items: Vec<String>,
    completed_count: i32,

    // Analytics
    total_clicks: i32,
    start_time: String,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            count: 0,
            user_text: String::new(),
            new_item: String::new(),
            items: Vec::new(),
            completed_count: 0,
            total_clicks: 0,
            start_time: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs()
                .to_string(),
        }
    }
}

/// Messages for the application
type Message = HandlerMessage;

/// Event handlers using ui_handler macro

#[ui_handler]
fn increment(model: &mut Model) {
    model.count += 1;
    model.total_clicks += 1;
    println!("[Handler] Incremented to: {}", model.count);
}

#[ui_handler]
fn decrement(model: &mut Model) {
    model.count -= 1;
    model.total_clicks += 1;
    println!("[Handler] Decremented to: {}", model.count);
}

#[ui_handler]
fn reset(model: &mut Model) {
    model.count = 0;
    model.total_clicks += 1;
    println!("[Handler] Reset counter");
}

#[ui_handler]
fn update_text(model: &mut Model, value: String) {
    model.user_text = value.clone();
    println!("[Handler] Text updated: {}", value);
}

#[ui_handler]
fn update_new_item(model: &mut Model, value: String) {
    model.new_item = value;
    println!("[Handler] New item text: {}", model.new_item);
}

#[ui_handler]
fn add_item(model: &mut Model) {
    if !model.new_item.is_empty() {
        model.items.push(model.new_item.clone());
        model.new_item.clear();
        model.total_clicks += 1;
        println!("[Handler] Added item. Total: {}", model.items.len());
    }
}

#[ui_handler]
fn clear_all(model: &mut Model) {
    let count = model.items.len();
    model.items.clear();
    model.completed_count = 0;
    model.total_clicks += 1;
    println!("[Handler] Cleared {} items", count);
}

#[ui_handler]
fn clear_completed(model: &mut Model) {
    // For demo, just clear half the items as "completed"
    let to_remove = model.items.len().min(model.completed_count as usize);
    if to_remove > 0 {
        model.items.truncate(model.items.len() - to_remove);
        model.completed_count = 0;
        model.total_clicks += 1;
        println!("[Handler] Cleared {} completed items", to_remove);
    }
}

/// Global application state
struct AppState {
    model: Model,
    document: gravity_core::GravityDocument,
    handler_registry: HandlerRegistry,
}

impl AppState {
    fn new() -> Self {
        // Load and parse the UI file
        let xml = include_str!("../ui/main.gravity");
        let document = parse(xml).expect("Failed to parse XML");

        // Create handler registry
        let handler_registry = HandlerRegistry::new();

        // Register all handlers
        handler_registry.register_simple("increment", |m: &mut dyn Any| {
            let model = m.downcast_mut::<Model>().unwrap();
            increment(model);
        });

        handler_registry.register_simple("decrement", |m: &mut dyn Any| {
            let model = m.downcast_mut::<Model>().unwrap();
            decrement(model);
        });

        handler_registry.register_simple("reset", |m: &mut dyn Any| {
            let model = m.downcast_mut::<Model>().unwrap();
            reset(model);
        });

        handler_registry.register_with_value("update_text", |m: &mut dyn Any, v: Box<dyn Any>| {
            let model = m.downcast_mut::<Model>().unwrap();
            if let Ok(text) = v.downcast::<String>() {
                update_text(model, *text);
            }
        });

        handler_registry.register_with_value(
            "update_new_item",
            |m: &mut dyn Any, v: Box<dyn Any>| {
                let model = m.downcast_mut::<Model>().unwrap();
                if let Ok(text) = v.downcast::<String>() {
                    update_new_item(model, *text);
                }
            },
        );

        handler_registry.register_simple("add_item", |m: &mut dyn Any| {
            let model = m.downcast_mut::<Model>().unwrap();
            add_item(model);
        });

        handler_registry.register_simple("clear_all", |m: &mut dyn Any| {
            let model = m.downcast_mut::<Model>().unwrap();
            clear_all(model);
        });

        handler_registry.register_simple("clear_completed", |m: &mut dyn Any| {
            let model = m.downcast_mut::<Model>().unwrap();
            clear_completed(model);
        });

        println!("=== Gravity Builder Demo ===");
        println!("This demo showcases:");
        println!("- Automatic widget building from XML");
        println!("- Binding evaluation (count, text length, conditionals)");
        println!("- Event handling (clicks, text input)");
        println!("- Style classes and themes");
        println!("- Layout and spacing");
        println!("");
        println!("Try interacting with the UI!");
        println!("============================");

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
        HandlerMessage::Handler(name, value_opt) => {
            if let Some(value) = value_opt {
                // Handler with value
                if let Some(gravity_core::HandlerEntry::WithValue(h)) =
                    state.handler_registry.get(&name)
                {
                    h(&mut state.model, Box::new(value));
                }
            } else {
                // Simple handler
                if let Some(gravity_core::HandlerEntry::Simple(h)) =
                    state.handler_registry.get(&name)
                {
                    h(&mut state.model);
                }
            }
        }
    }
    Task::none()
}

/// View function using GravityWidgetBuilder
fn view(state: &AppState) -> Element<'_, Message> {
    // Single line to build the entire UI!
    // The builder automatically:
    // - Evaluates all bindings
    // - Connects all event handlers
    // - Applies styles and layouts
    // - Processes nested widgets recursively
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
