pub mod layout;
pub mod node;
pub mod span;
pub mod style;
pub mod theme;

use std::collections::HashMap;

pub use layout::{
    Alignment, Breakpoint, Direction, Justification, LayoutConstraints, Length, Padding,
};
pub use node::InterpolatedPart;
pub use node::{AttributeValue, EventBinding, EventKind, WidgetKind, WidgetNode};
pub use span::Span;
pub use style::{
    Background, Border, BorderRadius, BorderStyle, Color, Gradient, ImageFit, Shadow,
    StyleProperties, Transform,
};
pub use theme::{
    FontWeight, IcedPaletteColors, SpacingScale, StateSelector, StyleClass, Theme, ThemeDocument,
    ThemeError, ThemeErrorKind, ThemePalette, Typography, WidgetState,
};

/// A complete parsed Dampen UI document.
///
/// This is the root structure returned by the parser. It contains
/// the document's schema version and the root widget tree.
///
/// # Example
///
/// ```rust
/// use dampen_core::{parse, DampenDocument};
///
/// let xml = r#"<dampen><column><text value="Hello" /></column></dampen>"#;
/// let doc: DampenDocument = parse(xml).unwrap();
/// assert_eq!(doc.version.major, 1);
/// ```
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DampenDocument {
    /// Schema version for compatibility checking
    pub version: SchemaVersion,

    /// Root widget of the UI tree
    pub root: WidgetNode,

    /// Theme definitions
    pub themes: HashMap<String, crate::ir::theme::Theme>,

    /// Style class definitions
    pub style_classes: HashMap<String, crate::ir::theme::StyleClass>,

    /// Global theme name
    pub global_theme: Option<String>,
}

impl Default for DampenDocument {
    /// Creates a default document with version 1.0 and an empty root column.
    fn default() -> Self {
        Self {
            version: SchemaVersion { major: 1, minor: 0 },
            root: WidgetNode::default(),
            themes: HashMap::new(),
            style_classes: HashMap::new(),
            global_theme: None,
        }
    }
}

/// Schema version for compatibility checking.
///
/// Versions follow semantic versioning:
/// - Major: Breaking changes
/// - Minor: Backward-compatible additions
///
/// Files without an explicit version default to 1.0.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct SchemaVersion {
    /// Major version number
    pub major: u16,
    /// Minor version number
    pub minor: u16,
}

impl Default for SchemaVersion {
    /// Default version is 1.0
    fn default() -> Self {
        Self { major: 1, minor: 0 }
    }
}
