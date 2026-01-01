pub mod node;
pub mod span;

pub use node::InterpolatedPart;
pub use node::{AttributeValue, EventBinding, EventKind, WidgetKind, WidgetNode};
pub use span::Span;

/// A complete parsed Gravity UI document.
///
/// This is the root structure returned by the parser. It contains
/// the document's schema version and the root widget tree.
///
/// # Example
///
/// ```rust
/// use gravity_core::{parse, GravityDocument};
///
/// let xml = r#"<column><text value="Hello" /></column>"#;
/// let doc: GravityDocument = parse(xml).unwrap();
/// assert_eq!(doc.version.major, 1);
/// ```
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GravityDocument {
    /// Schema version for compatibility checking
    pub version: SchemaVersion,

    /// Root widget of the UI tree
    pub root: WidgetNode,
}

impl Default for GravityDocument {
    /// Creates a default document with version 1.0 and an empty root column.
    fn default() -> Self {
        Self {
            version: SchemaVersion { major: 1, minor: 0 },
            root: WidgetNode::default(),
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
