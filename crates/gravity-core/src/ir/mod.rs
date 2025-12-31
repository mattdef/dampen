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

/// Schema version for compatibility checking
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct SchemaVersion {
    pub major: u16,
    pub minor: u16,
}
