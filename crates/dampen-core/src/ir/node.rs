use crate::ir::layout::{Breakpoint, LayoutConstraints};
use crate::ir::span::Span;
use crate::ir::style::StyleProperties;
use crate::ir::theme::WidgetState;
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
    pub theme_ref: Option<AttributeValue>,
    pub classes: Vec<String>,
    pub breakpoint_attributes: HashMap<Breakpoint, HashMap<String, AttributeValue>>,
    /// State-specific styles from inline attributes (e.g., hover:background="#ff0000")
    #[serde(default)]
    pub inline_state_variants: HashMap<WidgetState, StyleProperties>,
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
    Radio,
    // Advanced widgets
    ComboBox,
    ProgressBar,
    Tooltip,
    Grid,
    Canvas,
    CanvasRect,
    CanvasCircle,
    CanvasLine,
    CanvasText,
    CanvasGroup,
    /// Date selection widget with calendar overlay
    DatePicker,
    /// Time selection widget with hour/minute/second picker
    TimePicker,
    /// Color selection widget with color picker overlay
    ColorPicker,
    Menu,
    MenuItem,
    MenuSeparator,
    ContextMenu,
    Float,
    // Data display
    DataTable,
    DataColumn,
    // Tree widget
    TreeView,
    TreeNode,
    // Control flow
    For,
    If,
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
    /// Optional parameter expression (e.g., for on_click="delete:{item.id}")
    pub param: Option<crate::expr::BindingExpr>,
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
    CanvasClick,
    CanvasDrag,
    CanvasMove,
    CanvasRelease,
    RowClick,
    Cancel,
    Open,
    Close,
}

impl std::fmt::Display for WidgetKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            WidgetKind::Column => "column",
            WidgetKind::Row => "row",
            WidgetKind::Container => "container",
            WidgetKind::Scrollable => "scrollable",
            WidgetKind::Stack => "stack",
            WidgetKind::Text => "text",
            WidgetKind::Image => "image",
            WidgetKind::Svg => "svg",
            WidgetKind::Button => "button",
            WidgetKind::TextInput => "text_input",
            WidgetKind::Checkbox => "checkbox",
            WidgetKind::Slider => "slider",
            WidgetKind::PickList => "pick_list",
            WidgetKind::Toggler => "toggler",
            WidgetKind::Space => "space",
            WidgetKind::Rule => "rule",
            WidgetKind::Radio => "radio",
            WidgetKind::ComboBox => "combobox",
            WidgetKind::ProgressBar => "progress_bar",
            WidgetKind::Tooltip => "tooltip",
            WidgetKind::Grid => "grid",
            WidgetKind::Canvas => "canvas",
            WidgetKind::CanvasRect => "rect",
            WidgetKind::CanvasCircle => "circle",
            WidgetKind::CanvasLine => "line",
            WidgetKind::CanvasText => "canvas_text",
            WidgetKind::CanvasGroup => "group",
            WidgetKind::DatePicker => "date_picker",
            WidgetKind::TimePicker => "time_picker",
            WidgetKind::ColorPicker => "color_picker",
            WidgetKind::Menu => "menu",
            WidgetKind::MenuItem => "menu_item",
            WidgetKind::MenuSeparator => "menu_separator",
            WidgetKind::ContextMenu => "context_menu",
            WidgetKind::Float => "float",
            WidgetKind::DataTable => "data_table",
            WidgetKind::DataColumn => "data_column",
            WidgetKind::TreeView => "tree_view",
            WidgetKind::TreeNode => "tree_node",
            WidgetKind::For => "for",
            WidgetKind::If => "if",
            WidgetKind::Custom(name) => return write!(f, "{}", name),
        };
        write!(f, "{}", name)
    }
}

impl WidgetKind {
    /// Returns a list of all standard widget tag names.
    pub fn all_standard() -> &'static [&'static str] {
        &[
            "column",
            "row",
            "container",
            "scrollable",
            "stack",
            "text",
            "image",
            "svg",
            "button",
            "text_input",
            "checkbox",
            "slider",
            "pick_list",
            "toggler",
            "space",
            "rule",
            "radio",
            "combobox",
            "progress_bar",
            "tooltip",
            "grid",
            "canvas",
            "rect",
            "circle",
            "line",
            "canvas_text",
            "group",
            "date_picker",
            "time_picker",
            "color_picker",
            "menu",
            "menu_item",
            "menu_separator",
            "context_menu",
            "float",
            "data_table",
            "data_column",
            "tree_view",
            "tree_node",
            "for",
            "if",
        ]
    }

    /// Returns true if this is a custom widget.
    pub fn is_custom(&self) -> bool {
        matches!(self, WidgetKind::Custom(_))
    }

    /// Returns the minimum schema version required for this widget type.
    ///
    /// This method provides infrastructure for version-gating widgets in future releases.
    /// Currently, all widgets return version 1.0 as they are part of the initial release.
    ///
    /// # Future Usage
    ///
    /// When new widgets are added in future schema versions (e.g., 1.1, 1.2), this method
    /// will be updated to return the appropriate minimum version for those widgets.
    /// The parser can then validate that documents declaring older schema versions
    /// do not use widgets that were introduced in later versions.
    ///
    /// # Examples
    ///
    /// ```
    /// use dampen_core::{WidgetKind, SchemaVersion};
    ///
    /// let column = WidgetKind::Column;
    /// assert_eq!(column.minimum_version(), SchemaVersion { major: 1, minor: 0 });
    /// ```
    ///
    /// # Returns
    ///
    /// The minimum `SchemaVersion` required to use this widget type.
    pub fn minimum_version(&self) -> crate::ir::SchemaVersion {
        // Canvas is a v1.1 widget (experimental, not fully functional)
        // All other widgets are part of v1.0
        match self {
            WidgetKind::Canvas => crate::ir::SchemaVersion { major: 1, minor: 1 },
            WidgetKind::DatePicker
            | WidgetKind::TimePicker
            | WidgetKind::ColorPicker
            | WidgetKind::Menu
            | WidgetKind::MenuItem
            | WidgetKind::MenuSeparator
            | WidgetKind::ContextMenu
            | WidgetKind::DataTable
            | WidgetKind::DataColumn
            | WidgetKind::TreeView
            | WidgetKind::TreeNode => crate::ir::SchemaVersion { major: 1, minor: 1 },
            _ => crate::ir::SchemaVersion { major: 1, minor: 0 },
        }
    }

    /// Returns the validation schema for this widget type.
    pub fn schema(&self) -> crate::schema::WidgetSchema {
        crate::schema::get_widget_schema(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::style::StyleProperties;
    use crate::ir::theme::WidgetState;

    #[test]
    fn test_widget_node_default_has_empty_inline_state_variants() {
        let node = WidgetNode::default();
        assert!(node.inline_state_variants.is_empty());
    }

    #[test]
    fn test_widget_node_inline_state_variants_serialization() {
        let mut node = WidgetNode {
            kind: WidgetKind::Button,
            id: Some("test-button".to_string()),
            attributes: Default::default(),
            events: Default::default(),
            children: Default::default(),
            span: Default::default(),
            style: Default::default(),
            layout: Default::default(),
            theme_ref: Default::default(),
            classes: Default::default(),
            breakpoint_attributes: Default::default(),
            inline_state_variants: Default::default(),
        };

        // Add state variant
        node.inline_state_variants.insert(
            WidgetState::Hover,
            StyleProperties {
                opacity: Some(0.8),
                ..Default::default()
            },
        );

        // Serialize and deserialize
        let json = serde_json::to_string(&node).expect("Should serialize");
        let deserialized: WidgetNode = serde_json::from_str(&json).expect("Should deserialize");

        // Verify field preserved
        assert_eq!(deserialized.inline_state_variants.len(), 1);
        assert!(
            deserialized
                .inline_state_variants
                .contains_key(&WidgetState::Hover)
        );
    }

    #[test]
    fn test_widget_node_inline_state_variants_multiple_states() {
        let mut node = WidgetNode::default();

        node.inline_state_variants.insert(
            WidgetState::Hover,
            StyleProperties {
                opacity: Some(0.9),
                ..Default::default()
            },
        );

        node.inline_state_variants.insert(
            WidgetState::Active,
            StyleProperties {
                opacity: Some(0.7),
                ..Default::default()
            },
        );

        node.inline_state_variants.insert(
            WidgetState::Disabled,
            StyleProperties {
                opacity: Some(0.5),
                ..Default::default()
            },
        );

        assert_eq!(node.inline_state_variants.len(), 3);
        assert!(node.inline_state_variants.contains_key(&WidgetState::Hover));
        assert!(
            node.inline_state_variants
                .contains_key(&WidgetState::Active)
        );
        assert!(
            node.inline_state_variants
                .contains_key(&WidgetState::Disabled)
        );
    }
}

// Canvas Shape Types

/// A declarative representation of a shape in a canvas widget.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum CanvasShape {
    /// A rectangle shape.
    Rect(RectShape),
    /// A circle shape.
    Circle(CircleShape),
    /// A line segment.
    Line(LineShape),
    /// Text drawn on the canvas.
    Text(TextShape),
    /// A group of shapes with a common transformation.
    Group(GroupShape),
}

/// Attributes for a declarative rectangle shape.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RectShape {
    /// The X coordinate of the top-left corner.
    pub x: AttributeValue,
    /// The Y coordinate of the top-left corner.
    pub y: AttributeValue,
    /// The width of the rectangle.
    pub width: AttributeValue,
    /// The height of the rectangle.
    pub height: AttributeValue,
    /// The fill color.
    pub fill: Option<AttributeValue>,
    /// The stroke color.
    pub stroke: Option<AttributeValue>,
    /// The width of the stroke.
    pub stroke_width: Option<AttributeValue>,
    /// The corner radius for rounded rectangles.
    pub radius: Option<AttributeValue>,
}

/// Attributes for a declarative circle shape.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CircleShape {
    /// The X coordinate of the center.
    pub cx: AttributeValue,
    /// The Y coordinate of the center.
    pub cy: AttributeValue,
    /// The radius of the circle.
    pub radius: AttributeValue,
    /// The fill color.
    pub fill: Option<AttributeValue>,
    /// The stroke color.
    pub stroke: Option<AttributeValue>,
    /// The width of the stroke.
    pub stroke_width: Option<AttributeValue>,
}

/// Attributes for a declarative line shape.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LineShape {
    /// The X coordinate of the starting point.
    pub x1: AttributeValue,
    /// The Y coordinate of the starting point.
    pub y1: AttributeValue,
    /// The X coordinate of the ending point.
    pub x2: AttributeValue,
    /// The Y coordinate of the ending point.
    pub y2: AttributeValue,
    /// The stroke color.
    pub stroke: Option<AttributeValue>,
    /// The width of the stroke.
    pub stroke_width: Option<AttributeValue>,
}

/// Attributes for a declarative text element.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TextShape {
    /// The X coordinate of the text baseline.
    pub x: AttributeValue,
    /// The Y coordinate of the text baseline.
    pub y: AttributeValue,
    /// The content to be displayed.
    pub content: AttributeValue,
    /// The font size.
    pub size: Option<AttributeValue>,
    /// The text color.
    pub color: Option<AttributeValue>,
}

/// Attributes for a group of canvas shapes.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GroupShape {
    /// An optional transformation to apply to all child shapes.
    pub transform: Option<Transform>,
    /// The child shapes within this group.
    pub children: Vec<CanvasShape>,
}

/// Geometric transformations for canvas groups.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Transform {
    /// Translate by (x, y).
    Translate(f32, f32),
    /// Rotate by an angle in radians.
    Rotate(f32),
    /// Uniformly scale.
    Scale(f32),
    /// Scale non-uniformly by (x, y).
    ScaleXY(f32, f32),
    /// A 2D transformation matrix.
    Matrix([f32; 6]),
}
