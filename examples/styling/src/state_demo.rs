use gravity_core::{
    evaluate_binding_expr, parse, AttributeValue, EventKind, HandlerRegistry, InterpolatedPart,
    WidgetKind, WidgetNode,
};
use gravity_macros::{ui_handler, UiModel};
use iced::widget::{button, column, container, row, text};
use iced::{Border, Color, Element, Length, Padding, Task};
use serde::{Deserialize, Serialize};
use std::any::Any;

/// Application model with UiModel macro for data binding
#[derive(UiModel, Serialize, Deserialize, Clone, Debug)]
pub struct Model {
    click_count: i32,
    last_message: String,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            click_count: 0,
            last_message: String::from("No interactions yet"),
        }
    }
}

/// Messages for the application
#[derive(Clone, Debug)]
pub enum Message {
    Handler(String, Option<String>),
}

/// Event handlers using ui_handler macro
#[ui_handler]
fn click(model: &mut Model) {
    model.click_count += 1;
    model.last_message = format!("Button clicked! Count: {}", model.click_count);
}

#[ui_handler]
fn reset(model: &mut Model) {
    model.click_count = 0;
    model.last_message = String::from("Counter reset!");
}

/// Global application state
pub struct AppState {
    model: Model,
    document: gravity_core::GravityDocument,
    handler_registry: HandlerRegistry,
}

impl AppState {
    fn new() -> Self {
        let ui_path = std::path::PathBuf::from("examples/styling/ui/state_demo.gravity");
        let xml = match std::fs::read_to_string(&ui_path) {
            Ok(content) => content,
            Err(e) => {
                eprintln!("Error: Failed to read UI file: {}", e);
                r#"<column padding="40" spacing="20">
                    <text value="Error: Could not load ui/state_demo.gravity" size="18" />
                </column>"#
                    .to_string()
            }
        };

        let document = parse(&xml).unwrap_or_else(|e| {
            eprintln!("Error: Failed to parse UI: {}", e);
            gravity_core::GravityDocument::default()
        });

        let handler_registry = HandlerRegistry::new();

        handler_registry.register_simple("click", |model: &mut dyn Any| {
            let model = model.downcast_mut::<Model>().unwrap();
            click(model);
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
        Message::Handler(handler_name, _value) => {
            if let Some(gravity_core::HandlerEntry::Simple(h)) =
                state.handler_registry.get(&handler_name)
            {
                h(&mut state.model);
            }
        }
    }
    Task::none()
}

// ============================================================================
// Helper functions to convert Gravity IR types to Iced types
// ============================================================================

fn to_iced_color(color: &gravity_core::Color) -> Color {
    Color {
        r: color.r,
        g: color.g,
        b: color.b,
        a: color.a,
    }
}

fn to_iced_length(length: &gravity_core::Length) -> Length {
    match length {
        gravity_core::Length::Fixed(pixels) => Length::Fixed(*pixels),
        gravity_core::Length::Fill => Length::Fill,
        gravity_core::Length::Shrink => Length::Shrink,
        gravity_core::Length::FillPortion(n) => Length::FillPortion(*n as u16),
        gravity_core::Length::Percentage(pct) => Length::FillPortion((pct / 10.0).max(1.0) as u16),
    }
}

fn to_iced_padding(padding: &gravity_core::Padding) -> Padding {
    Padding {
        top: padding.top,
        right: padding.right,
        bottom: padding.bottom,
        left: padding.left,
    }
}

fn to_iced_radius(radius: &gravity_core::BorderRadius) -> iced::border::Radius {
    iced::border::Radius {
        top_left: radius.top_left,
        top_right: radius.top_right,
        bottom_right: radius.bottom_right,
        bottom_left: radius.bottom_left,
    }
}

// ============================================================================
// Rendering functions
// ============================================================================

#[allow(clippy::only_used_in_recursion)]
fn render_node<'a>(
    node: &'a WidgetNode,
    model: &'a Model,
    _handler_registry: &'a HandlerRegistry,
) -> Element<'a, Message> {
    match node.kind {
        WidgetKind::Text => render_text(node, model),
        WidgetKind::Button => render_button(node, model),
        WidgetKind::Column => render_column(node, model, _handler_registry),
        WidgetKind::Row => render_row(node, model, _handler_registry),
        WidgetKind::Container => render_container(node, model, _handler_registry),
        _ => text("").into(),
    }
}

fn render_text<'a>(node: &'a WidgetNode, model: &'a Model) -> Element<'a, Message> {
    let value = match node.attributes.get("value") {
        Some(AttributeValue::Static(v)) => v.clone(),
        Some(AttributeValue::Binding(expr)) => match evaluate_binding_expr(expr, model) {
            Ok(v) => v.to_display_string(),
            Err(_) => "[error]".to_string(),
        },
        Some(AttributeValue::Interpolated(parts)) => {
            let mut result = String::new();
            for part in parts {
                match part {
                    InterpolatedPart::Literal(l) => result.push_str(l),
                    InterpolatedPart::Binding(expr) => match evaluate_binding_expr(expr, model) {
                        Ok(v) => result.push_str(&v.to_display_string()),
                        Err(_) => result.push_str("[error]"),
                    },
                }
            }
            result
        }
        None => String::new(),
    };

    let mut txt = text(value);

    // Apply style from node.style (already parsed!)
    if let Some(ref style) = node.style {
        if let Some(ref color) = style.color {
            txt = txt.color(to_iced_color(color));
        }
    }

    // Size and weight from attributes (not in IR yet)
    if let Some(AttributeValue::Static(size_str)) = node.attributes.get("size") {
        if let Ok(size) = size_str.parse::<f32>() {
            txt = txt.size(size);
        }
    }

    txt.into()
}

fn render_button<'a>(node: &'a WidgetNode, model: &'a Model) -> Element<'a, Message> {
    let label = match node.attributes.get("label") {
        Some(AttributeValue::Static(l)) => l.clone(),
        _ => String::new(),
    };

    let on_click = node
        .events
        .iter()
        .find(|e| e.event == EventKind::Click)
        .map(|e| e.handler.clone());

    let btn = button(text(label));
    if let Some(handler_name) = on_click {
        btn.on_press(Message::Handler(handler_name, None)).into()
    } else {
        btn.into()
    }
}

fn render_column<'a>(
    node: &'a WidgetNode,
    model: &'a Model,
    handler_registry: &'a HandlerRegistry,
) -> Element<'a, Message> {
    let children: Vec<_> = node
        .children
        .iter()
        .map(|child| render_node(child, model, handler_registry))
        .collect();

    let mut col = column(children);

    // Apply layout from node.layout (already parsed!)
    if let Some(ref layout) = node.layout {
        if let Some(ref padding) = layout.padding {
            col = col.padding(to_iced_padding(padding));
        }
        if let Some(spacing) = layout.spacing {
            col = col.spacing(spacing);
        }
    }

    col.into()
}

fn render_row<'a>(
    node: &'a WidgetNode,
    model: &'a Model,
    handler_registry: &'a HandlerRegistry,
) -> Element<'a, Message> {
    let children: Vec<_> = node
        .children
        .iter()
        .map(|child| render_node(child, model, handler_registry))
        .collect();

    let mut r = row(children);

    // Apply layout from node.layout (already parsed!)
    if let Some(ref layout) = node.layout {
        if let Some(ref padding) = layout.padding {
            r = r.padding(to_iced_padding(padding));
        }
        if let Some(spacing) = layout.spacing {
            r = r.spacing(spacing);
        }
    }

    r.into()
}

fn render_container<'a>(
    node: &'a WidgetNode,
    model: &'a Model,
    handler_registry: &'a HandlerRegistry,
) -> Element<'a, Message> {
    let child = if !node.children.is_empty() {
        render_node(&node.children[0], model, handler_registry)
    } else {
        text("").into()
    };

    let mut cont = container(child);

    // Apply layout from node.layout (already parsed!)
    if let Some(ref layout) = node.layout {
        if let Some(ref padding) = layout.padding {
            cont = cont.padding(to_iced_padding(padding));
        }
        if let Some(ref width) = layout.width {
            cont = cont.width(to_iced_length(width));
        }
    }

    // Apply style from node.style (already parsed!)
    if let Some(ref style) = node.style {
        let has_style = style.background.is_some() || style.border.is_some();

        if has_style {
            let bg = style.background.as_ref().and_then(|bg| match bg {
                gravity_core::Background::Color(c) => Some(to_iced_color(c)),
                _ => None,
            });

            let (border_width, border_color, border_radius) = if let Some(ref border) = style.border
            {
                (
                    border.width,
                    to_iced_color(&border.color),
                    to_iced_radius(&border.radius),
                )
            } else {
                (0.0, Color::TRANSPARENT, iced::border::Radius::default())
            };

            cont = cont.style(move |_theme| container::Style {
                background: bg.map(iced::Background::Color),
                border: Border {
                    width: border_width,
                    color: border_color,
                    radius: border_radius,
                },
                text_color: None,
                shadow: iced::Shadow::default(),
                snap: false,
            });
        }
    }

    cont.into()
}

fn view(state: &AppState) -> Element<'_, Message> {
    render_node(&state.document.root, &state.model, &state.handler_registry)
}

pub fn main() -> iced::Result {
    iced::application(AppState::new, update, view).run()
}
