//! Gravity Iced - Iced Backend Implementation

use gravity_core::{AttributeValue, Backend, EventKind, InterpolatedPart, WidgetKind, WidgetNode};
use iced::widget::{button, column, row, text};
use iced::{Element, Renderer, Theme};

/// Iced backend implementation
pub struct IcedBackend {
    message_handler: Box<dyn Fn(String, Option<String>) -> Box<dyn CloneableMessage> + 'static>,
}

impl IcedBackend {
    /// Create a new Iced backend with a message handler
    pub fn new<F>(handler: F) -> Self
    where
        F: Fn(String, Option<String>) -> Box<dyn CloneableMessage> + 'static,
    {
        Self {
            message_handler: Box::new(handler),
        }
    }
}

/// Trait for messages that can be cloned
pub trait CloneableMessage: std::fmt::Debug + Send + Sync + 'static {
    fn clone_box(&self) -> Box<dyn CloneableMessage>;
}

impl<T> CloneableMessage for T
where
    T: Clone + std::fmt::Debug + Send + Sync + 'static,
{
    fn clone_box(&self) -> Box<dyn CloneableMessage> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn CloneableMessage> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

impl Backend for IcedBackend {
    type Widget<'a> = Element<'a, Box<dyn CloneableMessage>, Theme, Renderer>;
    type Message = Box<dyn CloneableMessage>;

    fn text<'a>(&self, content: &str) -> Self::Widget<'a> {
        text(content.to_string()).into()
    }

    fn button<'a>(
        &self,
        label: Self::Widget<'a>,
        on_press: Option<Self::Message>,
    ) -> Self::Widget<'a> {
        let btn = button(label);
        if let Some(msg) = on_press {
            btn.on_press(msg).into()
        } else {
            btn.into()
        }
    }

    fn column<'a>(&self, children: Vec<Self::Widget<'a>>) -> Self::Widget<'a> {
        column(children).into()
    }

    fn row<'a>(&self, children: Vec<Self::Widget<'a>>) -> Self::Widget<'a> {
        row(children).into()
    }
}

/// Render a widget node to an Iced element
///
/// Note: This is a simplified version. In a full implementation, this would receive
/// a model and evaluate bindings. For now, it handles static values.
pub fn render<'a>(
    node: &WidgetNode,
    backend: &IcedBackend,
) -> Element<'a, Box<dyn CloneableMessage>, Theme, Renderer> {
    match node.kind {
        WidgetKind::Text => {
            // Get the value attribute
            let value = match node.attributes.get("value") {
                Some(AttributeValue::Static(v)) => v.clone(),
                Some(AttributeValue::Binding(_)) => {
                    // Binding would be evaluated with model
                    "[binding]".to_string()
                }
                Some(AttributeValue::Interpolated(parts)) => {
                    // Interpolated would be evaluated with model
                    format_interpolated(parts)
                }
                None => String::new(),
            };
            backend.text(&value)
        }
        WidgetKind::Button => {
            // Get label
            let label_text = match node.attributes.get("label") {
                Some(AttributeValue::Static(l)) => l.clone(),
                Some(AttributeValue::Binding(_)) => "[binding]".to_string(),
                Some(AttributeValue::Interpolated(parts)) => format_interpolated(parts),
                None => String::new(),
            };
            let label = backend.text(&label_text);

            // Find click handler
            let on_press = node
                .events
                .iter()
                .find(|e| e.event == EventKind::Click)
                .map(|e| {
                    let handler_name = e.handler.clone();
                    (backend.message_handler)(handler_name, None)
                });

            backend.button(label, on_press)
        }
        WidgetKind::Column => {
            let children: Vec<_> = node
                .children
                .iter()
                .map(|child| render(child, backend))
                .collect();
            backend.column(children)
        }
        WidgetKind::Row => {
            let children: Vec<_> = node
                .children
                .iter()
                .map(|child| render(child, backend))
                .collect();
            backend.row(children)
        }
        _ => {
            // For unsupported widgets, return empty
            backend.column(Vec::new())
        }
    }
}

/// Helper to format interpolated parts (without model evaluation)
fn format_interpolated(parts: &[InterpolatedPart]) -> String {
    let mut result = String::new();
    for part in parts {
        match part {
            InterpolatedPart::Literal(literal) => result.push_str(literal),
            InterpolatedPart::Binding(_) => result.push_str("[binding]"),
        }
    }
    result
}
