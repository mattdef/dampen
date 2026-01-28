// TreeView validation for duplicate node IDs and required attributes
use crate::commands::check::errors::CheckError;
use dampen_core::ir::node::{WidgetKind, WidgetNode};
use std::collections::HashMap;
use std::path::PathBuf;

/// Represents a tree node for validation purposes
#[derive(Debug, Clone)]
pub struct TreeNodeInfo {
    pub id: String,
    pub label: String,
    pub file: PathBuf,
    pub line: u32,
    pub col: u32,
}

/// Validator for TreeView widgets
#[derive(Debug, Default)]
pub struct TreeViewValidator {
    /// Map of node IDs to their first occurrence (for duplicate detection)
    node_ids: HashMap<String, TreeNodeInfo>,
    /// List of validation errors
    errors: Vec<CheckError>,
    /// Current file being validated
    current_file: PathBuf,
}

impl TreeViewValidator {
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the current file being validated
    pub fn set_file(&mut self, file: PathBuf) {
        self.current_file = file;
    }

    /// Validate a TreeView widget and all its child nodes
    pub fn validate_tree_view(&mut self, node: &WidgetNode) {
        // Clear previous state for new tree
        self.node_ids.clear();
        self.errors.clear();

        // Validate all tree_node children recursively
        for child in &node.children {
            if child.kind == WidgetKind::TreeNode {
                self.validate_tree_node(child);
            }
        }
    }

    /// Validate a single tree node and its children
    fn validate_tree_node(&mut self, node: &WidgetNode) {
        let span = &node.span;

        // T059: Check required attributes (id, label)
        // Use node.id field since parser extracts it
        let id_value = node.id.clone().unwrap_or_default();
        let label = node.attributes.get("label");

        // Check for missing id
        if node.id.is_none() {
            self.errors.push(CheckError::MissingRequiredAttribute {
                attr: "id".to_string(),
                widget: "tree_node".to_string(),
                file: self.current_file.clone(),
                line: span.line,
                col: span.column,
            });
        }

        // Check for missing label
        if label.is_none() {
            self.errors.push(CheckError::MissingRequiredAttribute {
                attr: "label".to_string(),
                widget: "tree_node".to_string(),
                file: self.current_file.clone(),
                line: span.line,
                col: span.column,
            });
        }

        // T058: Check for duplicate node IDs
        if !id_value.is_empty() {
            if let Some(existing) = self.node_ids.get(&id_value) {
                // Found duplicate
                self.errors.push(CheckError::DuplicateTreeNodeId {
                    id: id_value.clone(),
                    file: self.current_file.clone(),
                    line: span.line,
                    col: span.column,
                    first_file: existing.file.clone(),
                    first_line: existing.line,
                    first_col: existing.col,
                });
            } else {
                // Store first occurrence
                let label_value = label.map_or_else(
                    || id_value.clone(),
                    |attr| match attr {
                        dampen_core::ir::node::AttributeValue::Static(s) => s.clone(),
                        _ => id_value.clone(),
                    },
                );

                self.node_ids.insert(
                    id_value.clone(),
                    TreeNodeInfo {
                        id: id_value.clone(),
                        label: label_value,
                        file: self.current_file.clone(),
                        line: span.line,
                        col: span.column,
                    },
                );
            }
        }

        // Recursively validate children
        for child in &node.children {
            if child.kind == WidgetKind::TreeNode {
                self.validate_tree_node(child);
            }
        }
    }

    /// Get all validation errors
    pub fn errors(&self) -> &[CheckError] {
        &self.errors
    }

    /// Check if validation found any errors
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dampen_core::ir::Span;
    use dampen_core::ir::node::AttributeValue;
    use std::collections::HashMap;

    fn create_test_node(id: &str, label: &str, line: u32) -> WidgetNode {
        let mut attributes = HashMap::new();
        attributes.insert(
            "label".to_string(),
            AttributeValue::Static(label.to_string()),
        );

        WidgetNode {
            kind: WidgetKind::TreeNode,
            id: Some(id.to_string()),
            attributes,
            events: vec![],
            children: vec![],
            span: Span::new(0, 0, line, 1),
            style: None,
            layout: None,
            theme_ref: None,
            classes: vec![],
            breakpoint_attributes: HashMap::new(),
            inline_state_variants: HashMap::new(),
        }
    }

    fn create_node_without_id(line: u32) -> WidgetNode {
        let mut attributes = HashMap::new();
        attributes.insert(
            "label".to_string(),
            AttributeValue::Static("Test".to_string()),
        );

        WidgetNode {
            kind: WidgetKind::TreeNode,
            id: None,
            attributes,
            events: vec![],
            children: vec![],
            span: Span::new(0, 0, line, 1),
            style: None,
            layout: None,
            theme_ref: None,
            classes: vec![],
            breakpoint_attributes: HashMap::new(),
            inline_state_variants: HashMap::new(),
        }
    }

    fn create_node_without_label(line: u32) -> WidgetNode {
        WidgetNode {
            kind: WidgetKind::TreeNode,
            id: Some("test".to_string()),
            attributes: HashMap::new(),
            events: vec![],
            children: vec![],
            span: Span::new(0, 0, line, 1),
            style: None,
            layout: None,
            theme_ref: None,
            classes: vec![],
            breakpoint_attributes: HashMap::new(),
            inline_state_variants: HashMap::new(),
        }
    }

    #[test]
    fn test_unique_node_ids() {
        let mut validator = TreeViewValidator::new();
        validator.set_file(PathBuf::from("test.dampen"));

        let tree_view = WidgetNode {
            kind: WidgetKind::TreeView,
            id: None,
            attributes: HashMap::new(),
            events: vec![],
            children: vec![
                create_test_node("node1", "Node 1", 10),
                create_test_node("node2", "Node 2", 15),
                create_test_node("node3", "Node 3", 20),
            ],
            span: Span::new(0, 0, 1, 1),
            style: None,
            layout: None,
            theme_ref: None,
            classes: vec![],
            breakpoint_attributes: HashMap::new(),
            inline_state_variants: HashMap::new(),
        };

        validator.validate_tree_view(&tree_view);
        assert!(!validator.has_errors());
    }

    #[test]
    fn test_duplicate_node_ids() {
        let mut validator = TreeViewValidator::new();
        validator.set_file(PathBuf::from("test.dampen"));

        let tree_view = WidgetNode {
            kind: WidgetKind::TreeView,
            id: None,
            attributes: HashMap::new(),
            events: vec![],
            children: vec![
                create_test_node("node1", "Node 1", 10),
                create_test_node("node1", "Node 1 Duplicate", 15),
            ],
            span: Span::new(0, 0, 1, 1),
            style: None,
            layout: None,
            theme_ref: None,
            classes: vec![],
            breakpoint_attributes: HashMap::new(),
            inline_state_variants: HashMap::new(),
        };

        validator.validate_tree_view(&tree_view);
        assert!(validator.has_errors());
        assert_eq!(validator.errors().len(), 1);
    }

    #[test]
    fn test_missing_required_attributes() {
        let mut validator = TreeViewValidator::new();
        validator.set_file(PathBuf::from("test.dampen"));

        let tree_view = WidgetNode {
            kind: WidgetKind::TreeView,
            id: None,
            attributes: HashMap::new(),
            events: vec![],
            children: vec![create_node_without_id(10), create_node_without_label(15)],
            span: Span::new(0, 0, 1, 1),
            style: None,
            layout: None,
            theme_ref: None,
            classes: vec![],
            breakpoint_attributes: HashMap::new(),
            inline_state_variants: HashMap::new(),
        };

        validator.validate_tree_view(&tree_view);
        assert!(validator.has_errors());
        assert_eq!(validator.errors().len(), 2);
    }
}
