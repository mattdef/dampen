use crate::ir::span::Span;
use std::collections::HashMap;

/// A node in the widget tree
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WidgetNode {
    pub kind: WidgetKind,
    pub id: Option<String>,
    pub attributes: HashMap<String, AttributeValue>,
    pub events: Vec<EventBinding>,
    pub children: Vec<WidgetNode>,
    pub span: Span,
}

/// Enumeration of all supported widget types
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum WidgetKind {
    Column,
    Row,
    Container,
    Scrollable,
    Stack,
    Text,
    Image,
    Svg,
    Button,
    TextInput,
    Checkbox,
    Slider,
    PickList,
    Toggler,
    Space,
    Rule,
    Custom(String),
}

/// A value that can be either static or dynamically bound
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum AttributeValue {
    Static(String),
    Binding(crate::expr::BindingExpr),
    Interpolated(Vec<InterpolatedPart>),
}

/// Part of an interpolated string
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum InterpolatedPart {
    Literal(String),
    Binding(crate::expr::BindingExpr),
}

/// An event binding from XML to a Rust handler
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct EventBinding {
    pub event: EventKind,
    pub handler: String,
    pub span: Span,
}

/// Supported event types
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum EventKind {
    Click,
    Press,
    Release,
    Change,
    Input,
    Submit,
    Select,
    Toggle,
    Scroll,
}
