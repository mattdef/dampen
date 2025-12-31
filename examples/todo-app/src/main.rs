use gravity_core::{parse, HandlerRegistry, WidgetNode, AttributeValue, InterpolatedPart, evaluate_binding_expr, WidgetKind, EventKind};
use gravity_macros::{ui_handler, UiModel};
use iced::{Application, Element, Task};
use iced::widget::{column, row, text, button};
use serde::{Serialize, Deserialize};
use std::any::Any;

/// Application state
#[derive(UiModel, Debug, Clone, Serialize, Deserialize, Default)]
struct Model {
    items: Vec<String>,
}

/// Messages
#[derive(Clone, Debug)]
enum Message {
    AddItem,
    ClearAll,
    Handler(String, Option<String>),
}

/// Event handlers
#[ui_handler]
fn add_item(model: &mut Model) {
    let count = model.items.len() + 1;
    model.items.push(format!("Item {}", count));
    println!("Added item. Total: {}", model.items.len());
}

#[ui_handler]
fn clear_all(model: &mut Model) {
    model.items.clear();
    println!("Cleared all items");
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
        
        Self {
            model: Model::default(),
            document,
            handler_registry,
        }
    }
}

/// Helper to evaluate bindings in attributes
fn evaluate_attribute(
    attr: &AttributeValue,
    model: &Model,
) -> String {
    match attr {
        AttributeValue::Static(s) => s.clone(),
        AttributeValue::Binding(binding_expr) => {
            match evaluate_binding_expr(binding_expr, model) {
                Ok(value) => value.to_display_string(),
                Err(_) => "[error]".to_string(),
            }
        }
        AttributeValue::Interpolated(parts) => {
            let mut result = String::new();
            for part in parts {
                match part {
                    InterpolatedPart::Literal(literal) => result.push_str(literal),
                    InterpolatedPart::Binding(binding_expr) => {
                        match evaluate_binding_expr(binding_expr, model) {
                            Ok(value) => result.push_str(&value.to_display_string()),
                            Err(_) => result.push_str("[error]"),
                        }
                    }
                }
            }
            result
        }
    }
}

/// Helper to render a node with binding evaluation
fn render_node<'a>(
    node: &'a WidgetNode,
    model: &Model,
    handler_registry: &HandlerRegistry,
) -> Element<'a, Message> {
    use iced::widget::{column, row, text, button};
    
    match node.kind {
        WidgetKind::Text => {
            let value = node.attributes.get("value")
                .map(|attr| evaluate_attribute(attr, model))
                .unwrap_or_default();
            text(value).into()
        }
        WidgetKind::Button => {
            let label = node.attributes.get("label")
                .map(|attr| evaluate_attribute(attr, model))
                .unwrap_or_default();
            
            let on_click = node.events.iter()
                .find(|e| e.event == EventKind::Click)
                .map(|e| Message::Handler(e.handler.clone(), None));
            
            let btn = button(text(label));
            if let Some(msg) = on_click {
                btn.on_press(msg).into()
            } else {
                btn.into()
            }
        }
        WidgetKind::Column => {
            let children: Vec<_> = node.children.iter()
                .map(|child| render_node(child, model, handler_registry))
                .collect();
            column(children).into()
        }
        WidgetKind::Row => {
            let children: Vec<_> = node.children.iter()
                .map(|child| render_node(child, model, handler_registry))
                .collect();
            row(children).into()
        }
        _ => column(Vec::new()).into(),
    }
}

/// Update function
fn update(state: &mut AppState, message: Message) -> Task<Message> {
    match message {
        Message::AddItem => {
            if let Some(handler) = state.handler_registry.get("add_item") {
                if let gravity_core::HandlerEntry::Simple(h) = handler {
                    h(&mut state.model);
                }
            }
        }
        Message::ClearAll => {
            if let Some(handler) = state.handler_registry.get("clear_all") {
                if let gravity_core::HandlerEntry::Simple(h) = handler {
                    h(&mut state.model);
                }
            }
        }
        Message::Handler(name, _value) => {
            if let Some(handler) = state.handler_registry.get(&name) {
                match handler {
                    gravity_core::HandlerEntry::Simple(h) => {
                        h(&mut state.model);
                    }
                    _ => {}
                }
            }
        }
    }
    Task::none()
}

/// View function
fn view(state: &AppState) -> Element<Message> {
    render_node(&state.document.root, &state.model, &state.handler_registry)
}

pub fn main() -> iced::Result {
    iced::application(AppState::new, update, view).run()
}
