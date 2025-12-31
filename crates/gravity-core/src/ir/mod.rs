pub mod node;
pub mod span;

pub use node::InterpolatedPart;
pub use node::{AttributeValue, EventBinding, EventKind, WidgetKind, WidgetNode};
pub use span::Span;

/// A complete parsed Gravity UI document
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GravityDocument {
    pub version: SchemaVersion,
    pub root: WidgetNode,
}

impl Default for GravityDocument {
    fn default() -> Self {
        Self {
            version: SchemaVersion { major: 1, minor: 0 },
            root: WidgetNode::default(),
        }
    }
}

/// Schema version for compatibility checking
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct SchemaVersion {
    pub major: u16,
    pub minor: u16,
}

impl Default for SchemaVersion {
    fn default() -> Self {
        Self { major: 1, minor: 0 }
    }
}
