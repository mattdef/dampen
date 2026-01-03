use gravity_core::{parse, HandlerRegistry};
use gravity_iced::{GravityWidgetBuilder, HandlerMessage};
use gravity_macros::{ui_handler, UiModel};
use iced::{Element, Task};
use serde::{Deserialize, Serialize};
use std::any::Any;

/// Application state
#[derive(UiModel, Debug, Clone, Serialize, Deserialize, Default)]
struct Model {
    items: Vec<String>,
    items_done: Vec<bool>,
    new_item_text: String,
    selected_category: String,
    priority: i32,
    dark_mode: bool,
    completed_count: i32,
    pending_count: i32,
}

/// Messages (using HandlerMessage from gravity-iced)
type Message = HandlerMessage;

/// Event handlers
#[ui_handler]
fn add_item(model: &mut Model) {
    if !model.new_item_text.is_empty() {
        model.items.push(model.new_item_text.clone());
        model.items_done.push(false);
        model.new_item_text.clear();
        model.pending_count += 1;
        println!(
            "Added: {} (Category: {}, Priority: {})",
            model.items.last().unwrap(),
            model.selected_category,
            model.priority
        );
    }
}

#[ui_handler]
fn clear_all(model: &mut Model) {
    let count = model.items.len();
    model.items.clear();
    model.items_done.clear();
    model.completed_count = 0;
    model.pending_count = 0;
    println!("Cleared {} items", count);
}

#[ui_handler]
fn clear_completed(model: &mut Model) {
    let mut new_items = Vec::new();
    let mut new_done = Vec::new();
    let mut completed = 0;
    let mut pending = 0;

    for (i, item) in model.items.iter().enumerate() {
        if !model.items_done[i] {
            new_items.push(item.clone());
            new_done.push(false);
            pending += 1;
        } else {
            completed += 1;
        }
    }

    model.items = new_items;
    model.items_done = new_done;
    model.completed_count = completed;
    model.pending_count = pending;
    println!("Cleared completed items");
}

#[ui_handler]
fn update_new_item(model: &mut Model, value: String) {
    model.new_item_text = value;
}

#[ui_handler]
fn select_category(model: &mut Model, value: String) {
    println!("Category selected: {}", value);
    model.selected_category = value;
}

#[ui_handler]
fn update_priority(model: &mut Model, value: f32) {
    model.priority = value as i32;
    println!("Priority updated: {}", model.priority);
}

#[ui_handler]
fn toggle_dark_mode(model: &mut Model) {
    model.dark_mode = !model.dark_mode;
    println!("Dark mode: {}", model.dark_mode);
}

/// Global state
struct AppState {
    model: Model,
    document: gravity_core::GravityDocument,
    handler_registry: HandlerRegistry,
}

impl AppState {
    fn new() -> Self {
        let xml = include_str!("../ui/main.gravity");
        let document = parse(xml).expect("Failed to parse XML");

        let handler_registry = HandlerRegistry::new();

        // Register handlers
        handler_registry.register_simple("add_item", |model: &mut dyn Any| {
            let model = model.downcast_mut::<Model>().unwrap();
            add_item(model);
        });

        handler_registry.register_simple("clear_all", |model: &mut dyn Any| {
            let model = model.downcast_mut::<Model>().unwrap();
            clear_all(model);
        });

        handler_registry.register_simple("clear_completed", |model: &mut dyn Any| {
            let model = model.downcast_mut::<Model>().unwrap();
            clear_completed(model);
        });

        handler_registry.register_simple("toggle_dark_mode", |model: &mut dyn Any| {
            let model = model.downcast_mut::<Model>().unwrap();
            toggle_dark_mode(model);
        });

        handler_registry.register_with_value(
            "update_new_item",
            |model: &mut dyn Any, value: Box<dyn Any>| {
                let model = model.downcast_mut::<Model>().unwrap();
                if let Ok(text) = value.downcast::<String>() {
                    update_new_item(model, *text);
                }
            },
        );

        handler_registry.register_with_value(
            "select_category",
            |model: &mut dyn Any, value: Box<dyn Any>| {
                let model = model.downcast_mut::<Model>().unwrap();
                if let Ok(text) = value.downcast::<String>() {
                    select_category(model, *text);
                }
            },
        );

        handler_registry.register_with_value(
            "update_priority",
            |model: &mut dyn Any, value: Box<dyn Any>| {
                let model = model.downcast_mut::<Model>().unwrap();
                if let Ok(num) = value.downcast::<f32>() {
                    update_priority(model, *num);
                }
            },
        );

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
