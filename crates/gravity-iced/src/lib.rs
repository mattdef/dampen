//! Gravity Iced - Iced Backend Implementation

pub mod style_mapping;
pub mod theme_adapter;
// pub mod widgets; // Placeholder - will be implemented in Phase 10

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

    fn container<'a>(&self, content: Self::Widget<'a>) -> Self::Widget<'a> {
        // Iced 0.14 doesn't have a simple container() helper, use column as placeholder
        // In a full implementation, you'd use iced::widget::container with proper imports
        column(vec![content]).into()
    }

    fn scrollable<'a>(&self, content: Self::Widget<'a>) -> Self::Widget<'a> {
        // Placeholder - Iced 0.14 has scrollable but needs feature flags
        column(vec![content]).into()
    }

    fn stack<'a>(&self, children: Vec<Self::Widget<'a>>) -> Self::Widget<'a> {
        // Stack is not in Iced 0.14 core - use column as placeholder
        column(children).into()
    }

    fn text_input<'a>(
        &self,
        _placeholder: &str,
        _value: &str,
        _on_input: Option<Self::Message>,
    ) -> Self::Widget<'a> {
        // Placeholder - text_input needs proper message handling
        text("[text_input]").into()
    }

    fn checkbox<'a>(
        &self,
        _label: &str,
        _is_checked: bool,
        _on_toggle: Option<Self::Message>,
    ) -> Self::Widget<'a> {
        // Placeholder - checkbox needs proper message handling
        text("[checkbox]").into()
    }

    fn slider<'a>(
        &self,
        _min: f32,
        _max: f32,
        _value: f32,
        _on_change: Option<Self::Message>,
    ) -> Self::Widget<'a> {
        // Placeholder - slider needs proper message handling
        text("[slider]").into()
    }

    fn pick_list<'a>(
        &self,
        _options: Vec<&str>,
        _selected: Option<&str>,
        _on_select: Option<Self::Message>,
    ) -> Self::Widget<'a> {
        // Placeholder - pick_list needs proper message handling
        text("[pick_list]").into()
    }

    fn toggler<'a>(
        &self,
        _label: &str,
        _is_active: bool,
        _on_toggle: Option<Self::Message>,
    ) -> Self::Widget<'a> {
        // Placeholder - toggler needs proper message handling
        text("[toggler]").into()
    }

    fn image<'a>(&self, _path: &str) -> Self::Widget<'a> {
        // Placeholder - image needs feature flag
        text("[image]").into()
    }

    fn svg<'a>(&self, _path: &str) -> Self::Widget<'a> {
        // Placeholder - SVG not in core Iced
        text("[svg]").into()
    }

    fn space<'a>(&self) -> Self::Widget<'a> {
        // Placeholder - space needs proper implementation
        text("").into()
    }

    fn rule<'a>(&self) -> Self::Widget<'a> {
        // Placeholder - rule needs proper implementation
        text("─").into()
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
        WidgetKind::Container => {
            let children: Vec<_> = node
                .children
                .iter()
                .map(|child| render(child, backend))
                .collect();
            if let Some(first_child) = children.into_iter().next() {
                backend.container(first_child)
            } else {
                backend.container(backend.text(""))
            }
        }
        WidgetKind::Scrollable => {
            let children: Vec<_> = node
                .children
                .iter()
                .map(|child| render(child, backend))
                .collect();
            if let Some(first_child) = children.into_iter().next() {
                backend.scrollable(first_child)
            } else {
                backend.scrollable(backend.text(""))
            }
        }
        WidgetKind::Stack => {
            let children: Vec<_> = node
                .children
                .iter()
                .map(|child| render(child, backend))
                .collect();
            backend.stack(children)
        }
        WidgetKind::TextInput => {
            let placeholder = match node.attributes.get("placeholder") {
                Some(AttributeValue::Static(v)) => v.clone(),
                _ => String::new(),
            };
            let value = match node.attributes.get("value") {
                Some(AttributeValue::Static(v)) => v.clone(),
                Some(AttributeValue::Binding(_)) => "[binding]".to_string(),
                Some(AttributeValue::Interpolated(parts)) => format_interpolated(parts),
                None => String::new(),
            };
            // Find input handler
            let on_input = node
                .events
                .iter()
                .find(|e| e.event == EventKind::Input)
                .map(|e| {
                    let handler_name = e.handler.clone();
                    (backend.message_handler)(handler_name, None)
                });
            backend.text_input(&placeholder, &value, on_input)
        }
        WidgetKind::Checkbox => {
            let label = match node.attributes.get("label") {
                Some(AttributeValue::Static(l)) => l.clone(),
                _ => String::new(),
            };
            let is_checked = match node.attributes.get("checked") {
                Some(AttributeValue::Static(v)) => v == "true" || v == "1",
                _ => false,
            };
            // Find toggle handler
            let on_toggle = node
                .events
                .iter()
                .find(|e| e.event == EventKind::Toggle)
                .map(|e| {
                    let handler_name = e.handler.clone();
                    (backend.message_handler)(handler_name, None)
                });
            backend.checkbox(&label, is_checked, on_toggle)
        }
        WidgetKind::Slider => {
            let min = match node.attributes.get("min") {
                Some(AttributeValue::Static(v)) => v.parse::<f32>().unwrap_or(0.0),
                _ => 0.0,
            };
            let max = match node.attributes.get("max") {
                Some(AttributeValue::Static(v)) => v.parse::<f32>().unwrap_or(100.0),
                _ => 100.0,
            };
            let value = match node.attributes.get("value") {
                Some(AttributeValue::Static(v)) => v.parse::<f32>().unwrap_or(50.0),
                _ => 50.0,
            };
            // Find change handler
            let on_change = node
                .events
                .iter()
                .find(|e| e.event == EventKind::Change)
                .map(|e| {
                    let handler_name = e.handler.clone();
                    (backend.message_handler)(handler_name, None)
                });
            backend.slider(min, max, value, on_change)
        }
        WidgetKind::PickList => {
            let options_str = match node.attributes.get("options") {
                Some(AttributeValue::Static(v)) => v.clone(),
                _ => String::new(),
            };
            let options: Vec<&str> = options_str.split(',').collect();
            let selected = match node.attributes.get("selected") {
                Some(AttributeValue::Static(v)) => Some(v.as_str()),
                _ => None,
            };
            // Find select handler
            let on_select = node
                .events
                .iter()
                .find(|e| e.event == EventKind::Select)
                .map(|e| {
                    let handler_name = e.handler.clone();
                    (backend.message_handler)(handler_name, None)
                });
            backend.pick_list(options, selected, on_select)
        }
        WidgetKind::Toggler => {
            let label = match node.attributes.get("label") {
                Some(AttributeValue::Static(l)) => l.clone(),
                _ => String::new(),
            };
            let is_active = match node.attributes.get("active") {
                Some(AttributeValue::Static(v)) => v == "true" || v == "1",
                _ => false,
            };
            // Find toggle handler
            let on_toggle = node
                .events
                .iter()
                .find(|e| e.event == EventKind::Toggle)
                .map(|e| {
                    let handler_name = e.handler.clone();
                    (backend.message_handler)(handler_name, None)
                });
            backend.toggler(&label, is_active, on_toggle)
        }
        WidgetKind::Image => {
            let path = match node.attributes.get("src") {
                Some(AttributeValue::Static(v)) => v.clone(),
                _ => String::new(),
            };
            backend.image(&path)
        }
        WidgetKind::Svg => {
            let path = match node.attributes.get("src") {
                Some(AttributeValue::Static(v)) => v.clone(),
                _ => String::new(),
            };
            backend.svg(&path)
        }
        WidgetKind::Space => backend.space(),
        WidgetKind::Rule => backend.rule(),
        WidgetKind::Custom(_) => {
            // For custom widgets, return empty
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
