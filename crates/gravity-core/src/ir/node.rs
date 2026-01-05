use crate::ir::layout::{Breakpoint, LayoutConstraints};
use crate::ir::span::Span;
use crate::ir::style::StyleProperties;
use std::collections::HashMap;

/// A node in the widget tree
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct WidgetNode {
    pub kind: WidgetKind,
    pub id: Option<String>,
    pub attributes: HashMap<String, AttributeValue>,
    pub events: Vec<EventBinding>,
    pub children: Vec<WidgetNode>,
    pub span: Span,

    // Styling extensions
    pub style: Option<StyleProperties>,
    pub layout: Option<LayoutConstraints>,
    pub theme_ref: Option<String>,
    pub classes: Vec<String>,
    pub breakpoint_attributes: HashMap<Breakpoint, HashMap<String, AttributeValue>>,
}

/// Enumeration of all supported widget types
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize, Default)]
pub enum WidgetKind {
    #[default]
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
    // Advanced widgets
    ComboBox,
    ProgressBar,
    Tooltip,
    Grid,
    Canvas,
    Float,
    // Control flow
    For,
    Custom(String),
}

/// A value that can be either static or dynamically bound
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum AttributeValue {
    Static(String),
    Binding(crate::expr::BindingExpr),
    Interpolated(Vec<InterpolatedPart>),
}

impl Default for AttributeValue {
    fn default() -> Self {
        AttributeValue::Static(String::new())
    }
}

/// Part of an interpolated string
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum InterpolatedPart {
    Literal(String),
    Binding(crate::expr::BindingExpr),
}

impl Default for InterpolatedPart {
    fn default() -> Self {
        InterpolatedPart::Literal(String::new())
    }
}

/// Attribute structures for advanced widgets
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ComboBoxAttributes {
    pub options: Vec<String>,
    pub selected: Option<crate::expr::BindingExpr>,
    pub placeholder: Option<String>,
    pub on_select: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PickListAttributes {
    pub options: Vec<String>,
    pub selected: Option<crate::expr::BindingExpr>,
    pub placeholder: Option<String>,
    pub on_select: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CanvasAttributes {
    pub width: f32,
    pub height: f32,
    pub program: Option<crate::expr::BindingExpr>,
    pub on_click: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ProgressBarAttributes {
    pub min: Option<f32>,
    pub max: Option<f32>,
    pub value: crate::expr::BindingExpr,
    pub style: Option<ProgressBarStyle>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ProgressBarStyle {
    Primary,
    Success,
    Warning,
    Danger,
    Secondary,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TooltipAttributes {
    pub message: String,
    pub position: Option<TooltipPosition>,
    pub delay: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum TooltipPosition {
    FollowCursor,
    Top,
    Bottom,
    Left,
    Right,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GridAttributes {
    pub columns: u32,
    pub spacing: Option<f32>,
    pub padding: Option<f32>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FloatAttributes {
    pub position: Option<FloatPosition>,
    pub offset_x: Option<f32>,
    pub offset_y: Option<f32>,
    pub z_index: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum FloatPosition {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

/// An event binding from XML to a Rust handler
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct EventBinding {
    pub event: EventKind,
    pub handler: String,
    pub span: Span,
}

/// Supported event types
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize, Default)]
pub enum EventKind {
    #[default]
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
