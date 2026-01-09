//! Breakpoint resolution and attribute merging for responsive layouts

use dampen_core::ir::layout::Breakpoint;
use dampen_core::ir::node::{AttributeValue, WidgetNode};
use std::collections::HashMap;

/// Resolve breakpoint-specific attributes for a widget
///
/// This function merges base attributes with breakpoint-specific attributes
/// based on the current viewport width. Breakpoint attributes take precedence
/// over base attributes.
///
/// # Arguments
///
/// * `node` - The widget node to resolve attributes for
/// * `viewport_width` - Current viewport width in pixels
///
/// # Returns
///
/// A map of all resolved attributes (base + breakpoint-specific)
///
/// # Example
///
/// ```rust,ignore
/// use dampen_runtime::breakpoint::resolve_breakpoint_attributes;
/// use dampen_core::parse;
///
/// let xml = r#"<column spacing="10" mobile:spacing="5" desktop:spacing="20">...</column>"#;
/// let doc = parse(xml).unwrap();
/// let attrs = resolve_breakpoint_attributes(&doc.root, 800.0);
/// // Returns: {"spacing": "10"} for tablet (640-1024)
/// // Returns: {"spacing": "5"} for mobile (<640)
/// // Returns: {"spacing": "20"} for desktop (>=1024)
/// ```
pub fn resolve_breakpoint_attributes(
    node: &WidgetNode,
    viewport_width: f32,
) -> HashMap<String, AttributeValue> {
    let breakpoint = Breakpoint::from_viewport_width(viewport_width);

    // Start with base attributes
    let mut resolved = node.attributes.clone();

    // Override with breakpoint-specific attributes if they exist
    if let Some(bp_attrs) = node.breakpoint_attributes.get(&breakpoint) {
        for (key, value) in bp_attrs {
            resolved.insert(key.clone(), value.clone());
        }
    }

    resolved
}

/// Check if viewport width change would trigger a breakpoint change
///
/// # Arguments
///
/// * `old_width` - Previous viewport width
/// * `new_width` - New viewport width
///
/// # Returns
///
/// `true` if the breakpoint would change, `false` otherwise
pub fn would_change_breakpoint(old_width: f32, new_width: f32) -> bool {
    let old_bp = Breakpoint::from_viewport_width(old_width);
    let new_bp = Breakpoint::from_viewport_width(new_width);
    old_bp != new_bp
}

/// Get the active breakpoint for a given viewport width
pub fn get_active_breakpoint(width: f32) -> Breakpoint {
    Breakpoint::from_viewport_width(width)
}

/// Resolve all attributes for a widget tree recursively
///
/// This is useful for rendering the entire tree with responsive attributes applied
pub fn resolve_tree_breakpoint_attributes(node: &WidgetNode, viewport_width: f32) -> WidgetNode {
    let mut resolved_node = node.clone();

    // Resolve attributes for this node
    resolved_node.attributes = resolve_breakpoint_attributes(node, viewport_width);

    // Recursively resolve children
    resolved_node.children = node
        .children
        .iter()
        .map(|child| resolve_tree_breakpoint_attributes(child, viewport_width))
        .collect();

    resolved_node
}
