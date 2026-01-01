use gravity_core::{
    parse, evaluate_binding_expr, HandlerRegistry, WidgetNode, WidgetKind, EventKind,
    AttributeValue, InterpolatedPart,
};
use gravity_macros::{ui_handler, UiModel};
use iced::widget::{column, row, text, button};
use iced::{Element, Task};
use serde::{Serialize, Deserialize};
use std::any::Any;

/// Application state
#[derive(Default, UiModel, Serialize, Deserialize, Clone, Debug)]
struct Model {
    count: i32,
}

/// Messages for the application
#[derive(Clone, Debug)]
enum Message {
    Handler(String, Option<String>),
}

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
        Message::Handler(handler_name, _value) => {
            // Dispatch to handler registry
            if let Some(gravity_core::HandlerEntry::Simple(h)) = state.handler_registry.get(&handler_name) {
                h(&mut state.model);
            }
        }
    }
    Task::none()
}

/// Helper to render a widget node
#[allow(clippy::only_used_in_recursion)]
fn render_node<'a>(node: &'a WidgetNode, model: &'a Model, handler_registry: &'a HandlerRegistry) -> Element<'a, Message> {
    match node.kind {
        WidgetKind::Text => {
            let value = match node.attributes.get("value") {
                Some(AttributeValue::Static(v)) => v.clone(),
                Some(AttributeValue::Binding(expr)) => {
                    match evaluate_binding_expr(expr, model) {
                        Ok(v) => v.to_display_string(),
                        Err(_) => "[error]".to_string(),
                    }
                }
                Some(AttributeValue::Interpolated(parts)) => {
                    let mut result = String::new();
                    for part in parts {
                        match part {
                            InterpolatedPart::Literal(l) => result.push_str(l),
                            InterpolatedPart::Binding(expr) => {
                                match evaluate_binding_expr(expr, model) {
                                    Ok(v) => result.push_str(&v.to_display_string()),
                                    Err(_) => result.push_str("[error]"),
                                }
                            }
                        }
                    }
                    result
                }
                None => String::new(),
            };
            text(value).into()
        }
        WidgetKind::Button => {
            let label = match node.attributes.get("label") {
                Some(AttributeValue::Static(l)) => l.clone(),
                Some(AttributeValue::Binding(expr)) => {
                    match evaluate_binding_expr(expr, model) {
                        Ok(v) => v.to_display_string(),
                        Err(_) => "[error]".to_string(),
                    }
                }
                Some(AttributeValue::Interpolated(parts)) => {
                    let mut result = String::new();
                    for part in parts {
                        match part {
                            InterpolatedPart::Literal(l) => result.push_str(l),
                            InterpolatedPart::Binding(expr) => {
                                match evaluate_binding_expr(expr, model) {
                                    Ok(v) => result.push_str(&v.to_display_string()),
                                    Err(_) => result.push_str("[error]"),
                                }
                            }
                        }
                    }
                    result
                }
                None => String::new(),
            };
            
            // Find click handler
            let on_click = node.events.iter()
                .find(|e| e.event == EventKind::Click)
                .map(|e| e.handler.clone());
            
            let btn = button(text(label));
            if let Some(handler_name) = on_click {
                btn.on_press(Message::Handler(handler_name, None)).into()
            } else {
                btn.into()
            }
        }
        WidgetKind::Column => {
            let children: Vec<_> = node.children.iter()
                .map(|child| render_node(child, model, handler_registry))
                .collect();
            let mut col = column(children);
            
            // Handle attributes
            if let Some(AttributeValue::Static(padding)) = node.attributes.get("padding") {
                if let Ok(p) = padding.parse::<f32>() {
                    col = col.padding(p);
                }
            }
            if let Some(AttributeValue::Static(spacing)) = node.attributes.get("spacing") {
                if let Ok(s) = spacing.parse::<f32>() {
                    col = col.spacing(s);
                }
            }
            
            col.into()
        }
        WidgetKind::Row => {
            let children: Vec<_> = node.children.iter()
                .map(|child| render_node(child, model, handler_registry))
                .collect();
            let mut row_widget = row(children);
            
            // Handle attributes
            if let Some(AttributeValue::Static(spacing)) = node.attributes.get("spacing") {
                if let Ok(s) = spacing.parse::<f32>() {
                    row_widget = row_widget.spacing(s);
                }
            }
            
            row_widget.into()
        }
        _ => column(Vec::new()).into(),
    }
}

/// View function
fn view(state: &AppState) -> Element<'_, Message> {
    render_node(&state.document.root, &state.model, &state.handler_registry)
}

pub fn main() -> iced::Result {
    iced::application(AppState::new, update, view).run()
}
