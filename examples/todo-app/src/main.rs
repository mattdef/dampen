use gravity_core::{
    parse, HandlerRegistry, WidgetNode, AttributeValue, InterpolatedPart,
    evaluate_binding_expr, WidgetKind, EventKind,
};
use gravity_macros::{ui_handler, UiModel};
use iced::{Element, Task};
use serde::{Serialize, Deserialize};
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

/// Messages
#[derive(Clone, Debug)]
enum Message {
    Handler(String, Option<String>),
}

/// Event handlers
#[ui_handler]
fn add_item(model: &mut Model) {
    if !model.new_item_text.is_empty() {
        model.items.push(model.new_item_text.clone());
        model.items_done.push(false);
        model.new_item_text.clear();
        model.pending_count += 1;
        println!("Added: {} (Category: {}, Priority: {})", 
            model.items.last().unwrap(), model.selected_category, model.priority);
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
        
        handler_registry.register_with_value("update_new_item", |model: &mut dyn Any, value: Box<dyn Any>| {
            let model = model.downcast_mut::<Model>().unwrap();
            if let Ok(text) = value.downcast::<String>() {
                update_new_item(model, *text);
            }
        });
        
        handler_registry.register_with_value("select_category", |model: &mut dyn Any, value: Box<dyn Any>| {
            let model = model.downcast_mut::<Model>().unwrap();
            if let Ok(text) = value.downcast::<String>() {
                select_category(model, *text);
            }
        });
        
        handler_registry.register_with_value("update_priority", |model: &mut dyn Any, value: Box<dyn Any>| {
            let model = model.downcast_mut::<Model>().unwrap();
            if let Ok(num) = value.downcast::<f32>() {
                update_priority(model, *num);
            }
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
#[allow(clippy::only_used_in_recursion)]
fn render_node<'a>(
    node: &'a WidgetNode,
    model: &Model,
    handler_registry: &HandlerRegistry,
) -> Element<'a, Message> {
    use iced::widget::{button, column, row, text, text_input, toggler, slider, pick_list, container, scrollable, rule, space};
    
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
        WidgetKind::Container => {
            let children: Vec<_> = node.children.iter()
                .map(|child| render_node(child, model, handler_registry))
                .collect();
            if let Some(first) = children.into_iter().next() {
                container(first).into()
            } else {
                container(text("")).into()
            }
        }
        WidgetKind::Scrollable => {
            let children: Vec<_> = node.children.iter()
                .map(|child| render_node(child, model, handler_registry))
                .collect();
            scrollable(column(children)).into()
        }
        WidgetKind::TextInput => {
            let placeholder = node.attributes.get("placeholder")
                .map(|attr| evaluate_attribute(attr, model))
                .unwrap_or_default();
            let value = node.attributes.get("value")
                .map(|attr| evaluate_attribute(attr, model))
                .unwrap_or_default();
            
            let on_input = node.events.iter()
                .find(|e| e.event == EventKind::Input)
                .map(|e| e.handler.clone());
            
            if let Some(handler_name) = on_input {
                // text_input requires a closure that takes String
                let input = text_input(&placeholder, &value);
                input.on_input(move |new_value| Message::Handler(handler_name.clone(), Some(new_value))).into()
            } else {
                text_input(&placeholder, &value).into()
            }
        }
        WidgetKind::Toggler => {
            let label = node.attributes.get("label")
                .map(|attr| evaluate_attribute(attr, model))
                .unwrap_or_default();
            let is_active = node.attributes.get("active")
                .map(|attr| evaluate_attribute(attr, model) == "true")
                .unwrap_or(false);
            
            let on_toggle = node.events.iter()
                .find(|e| e.event == EventKind::Toggle)
                .map(|e| Message::Handler(e.handler.clone(), None));
            
            // Toggler in iced 0.14 has different signature
            // Show as button with state indicator for now
            let btn_label = format!("{} [{}]", label, if is_active { "ON" } else { "OFF" });
            let btn = button(text(btn_label));
            if let Some(msg) = on_toggle {
                btn.on_press(msg).into()
            } else {
                btn.into()
            }
        }
        WidgetKind::Slider => {
            let value = node.attributes.get("value")
                .map(|attr| evaluate_attribute(attr, model))
                .unwrap_or_default();
            
            // Show value as text (slider would need proper message handling)
            text(format!("Priority: {}", value)).into()
        }
        WidgetKind::PickList => {
            let options_str = node.attributes.get("options")
                .map(|attr| evaluate_attribute(attr, model))
                .unwrap_or_default();
            let options: Vec<String> = options_str.split(',').map(|s| s.trim().to_string()).collect();
            let selected = node.attributes.get("selected")
                .map(|attr| evaluate_attribute(attr, model));
            
            let on_select = node.events.iter()
                .find(|e| e.event == EventKind::Select)
                .map(|e| Message::Handler(e.handler.clone(), None));
            
            // Use a placeholder since pick_list has complex type requirements
            if let Some(msg) = on_select {
                // For now, show text with selection info
                let label = format!("Selected: {}", selected.unwrap_or_default());
                text(label).into()
            } else {
                text("PickList").into()
            }
        }
        WidgetKind::Space => {
            space().into()
        }
        WidgetKind::Rule => {
            // Rule is not available in iced 0.14 core, use text as divider
            text("──────────────────────────────────────").into()
        }
        WidgetKind::Stack => {
            let children: Vec<_> = node.children.iter()
                .map(|child| render_node(child, model, handler_registry))
                .collect();
            // Stack not available in iced 0.14 core, use column as fallback
            column(children).into()
        }
        WidgetKind::Image => {
            // Placeholder - would need image feature
            text("[image]").into()
        }
        WidgetKind::Svg => {
            // Placeholder - would need svg feature
            text("[svg]").into()
        }
        _ => column(Vec::new()).into(),
    }
}

/// Update function
fn update(state: &mut AppState, message: Message) -> Task<Message> {
    match message {
        Message::Handler(name, value_opt) => {
            if let Some(value) = value_opt {
                // Handler with value
                if let Some(gravity_core::HandlerEntry::WithValue(h)) = state.handler_registry.get(&name) {
                    h(&mut state.model, Box::new(value));
                }
            } else {
                // Simple handler
                if let Some(gravity_core::HandlerEntry::Simple(h)) = state.handler_registry.get(&name) {
                    h(&mut state.model);
                }
            }
        }
    }
    Task::none()
}

/// View function
fn view(state: &AppState) -> Element<'_, Message> {
    render_node(&state.document.root, &state.model, &state.handler_registry)
}

pub fn main() -> iced::Result {
    iced::application(AppState::new, update, view).run()
}
