//! TreeView widget builder
//!
//! This module implements the TreeView widget for displaying hierarchical data.
//! It supports expand/collapse functionality, node selection, and custom styling.

use crate::HandlerMessage;
use crate::builder::DampenWidgetBuilder;
use dampen_core::binding::BindingValue;
use dampen_core::expr::evaluate_binding_expr_with_shared;
use dampen_core::ir::node::{AttributeValue, WidgetKind, WidgetNode};
use iced::widget::{button, column, container, row, text};
use iced::{Element, Length, Renderer, Theme};

impl<'a> DampenWidgetBuilder<'a> {
    /// Build a TreeView widget from an XML node.
    ///
    /// Renders a hierarchical tree structure with expand/collapse functionality.
    /// Supports both inline XML `tree_node` children and data binding for dynamic trees.
    ///
    /// # Attributes
    /// * `nodes`: Optional data binding to a list of tree nodes.
    /// * `expanded`: Optional binding to a list of IDs of expanded nodes.
    /// * `selected`: Optional binding to the ID of the selected node.
    /// * `indent_size`: Pixels per nesting level (default: 20.0).
    /// * `node_height`: Height of each node row (default: 30.0).
    /// * `expand_icon`: Text for collapsed nodes (default: "▶").
    /// * `collapse_icon`: Text for expanded nodes (default: "▼").
    pub(in crate::builder) fn build_tree_view(
        &self,
        node: &WidgetNode,
    ) -> Element<'a, HandlerMessage, Theme, Renderer>
    where
        HandlerMessage: Clone + 'static,
    {
        // Get tree configuration from attributes
        let indent_size = node
            .attributes
            .get("indent_size")
            .map(|a| self.evaluate_attribute(a))
            .and_then(|s| s.parse::<f32>().ok())
            .unwrap_or(20.0);

        let node_height = node
            .attributes
            .get("node_height")
            .map(|a| self.evaluate_attribute(a))
            .and_then(|s| s.parse::<f32>().ok())
            .unwrap_or(30.0);

        let _icon_size = node
            .attributes
            .get("icon_size")
            .map(|a| self.evaluate_attribute(a))
            .and_then(|s| s.parse::<f32>().ok())
            .unwrap_or(16.0);

        let expand_icon_default = "▶".to_string();
        let collapse_icon_default = "▼".to_string();
        let expand_icon = node
            .attributes
            .get("expand_icon")
            .map(|a| self.evaluate_attribute(a))
            .unwrap_or_else(|| expand_icon_default.clone());

        let collapse_icon = node
            .attributes
            .get("collapse_icon")
            .map(|a| self.evaluate_attribute(a))
            .unwrap_or_else(|| collapse_icon_default.clone());

        let _leaf_icon = node
            .attributes
            .get("leaf_icon")
            .map(|a| self.evaluate_attribute(a));

        // Get expanded node IDs from binding or default to empty
        let expanded_ids: Vec<String> = node
            .attributes
            .get("expanded")
            .map(|attr| match attr {
                AttributeValue::Binding(expr) => {
                    let result = if let Some(ctx_value) = self.resolve_from_context(expr) {
                        Ok(ctx_value)
                    } else {
                        evaluate_binding_expr_with_shared(expr, self.model, self.shared_context)
                    };
                    match result {
                        Ok(BindingValue::List(items)) => items
                            .iter()
                            .filter_map(|item| match item {
                                BindingValue::String(s) => Some(s.clone()),
                                _ => None,
                            })
                            .collect(),
                        _ => Vec::new(),
                    }
                }
                _ => Vec::new(),
            })
            .unwrap_or_default();

        // Get selected node ID from binding
        let selected_id: Option<String> =
            node.attributes.get("selected").and_then(|attr| match attr {
                AttributeValue::Binding(expr) => {
                    let result = if let Some(ctx_value) = self.resolve_from_context(expr) {
                        Ok(ctx_value)
                    } else {
                        evaluate_binding_expr_with_shared(expr, self.model, self.shared_context)
                    };
                    match result {
                        Ok(BindingValue::String(s)) => Some(s),
                        Ok(BindingValue::None) => None,
                        _ => None,
                    }
                }
                _ => None,
            });

        // Check if we have a nodes binding (dynamic tree) or inline children (static tree)
        let has_nodes_binding = node.attributes.contains_key("nodes");

        let tree_nodes: Vec<TreeNodeData> = if has_nodes_binding {
            // Dynamic tree from binding
            node.attributes
                .get("nodes")
                .map(|attr| match attr {
                    AttributeValue::Binding(expr) => {
                        let result = if let Some(ctx_value) = self.resolve_from_context(expr) {
                            Ok(ctx_value)
                        } else {
                            evaluate_binding_expr_with_shared(expr, self.model, self.shared_context)
                        };
                        match result {
                            Ok(BindingValue::List(items)) => {
                                items.iter().filter_map(extract_tree_node_data).collect()
                            }
                            _ => Vec::new(),
                        }
                    }
                    _ => Vec::new(),
                })
                .unwrap_or_default()
        } else {
            // Static tree from inline XML children
            node.children
                .iter()
                .filter(|c| c.kind == WidgetKind::TreeNode)
                .map(|child| {
                    extract_tree_node_from_widget(child, &expanded_ids, selected_id.as_ref())
                })
                .collect()
        };

        // Build the tree UI
        if tree_nodes.is_empty() {
            // Empty state
            return container(text("No items")).into();
        }

        // Build tree recursively
        let tree_elements: Vec<Element<'a, HandlerMessage, Theme, Renderer>> = tree_nodes
            .into_iter()
            .map(|tree_node| {
                self.build_tree_node_recursive(
                    tree_node,
                    &expanded_ids,
                    selected_id.as_ref(),
                    indent_size,
                    node_height,
                    &expand_icon,
                    &collapse_icon,
                    0,
                    node,
                )
            })
            .collect();

        column(tree_elements).spacing(2).into()
    }

    /// Recursively build a tree node and its children
    fn build_tree_node_recursive(
        &self,
        tree_node: TreeNodeData,
        expanded_ids: &[String],
        selected_id: Option<&String>,
        indent_size: f32,
        node_height: f32,
        expand_icon: &str,
        collapse_icon: &str,
        depth: usize,
        parent_node: &WidgetNode,
    ) -> Element<'a, HandlerMessage, Theme, Renderer>
    where
        HandlerMessage: Clone + 'static,
    {
        // T068, T069: Prevent infinite recursion and handle deep nesting
        if depth > 100 {
            return text("... max depth reached").size(12).into();
        }

        let is_expanded = expanded_ids.contains(&tree_node.id);
        let is_selected = selected_id == Some(&tree_node.id);
        let has_children = !tree_node.children.is_empty();

        // Build the node row
        let indent = (depth as f32) * indent_size;

        // Expand/collapse button or spacer
        let toggle_button: Element<'a, HandlerMessage, Theme, Renderer> = if has_children {
            let icon = if is_expanded {
                collapse_icon.to_string()
            } else {
                expand_icon.to_string()
            };
            let node_id = tree_node.id.clone();

            // Find on_toggle event handler
            if let Some(event) = parent_node
                .events
                .iter()
                .find(|e| matches!(e.event, dampen_core::ir::node::EventKind::Toggle))
            {
                let handler_name = event.handler.clone();
                let message_factory = self.message_factory.clone();

                button(text(icon).size(14))
                    .on_press((message_factory)(
                        &handler_name,
                        Some(format!(
                            "{{\"node_id\":\"{}\",\"expanded\":{}}}",
                            node_id, !is_expanded
                        )),
                    ))
                    .width(Length::Fixed(20.0))
                    .height(Length::Fixed(node_height))
                    .style(
                        |_theme: &Theme, status: iced::widget::button::Status| match status {
                            iced::widget::button::Status::Hovered => iced::widget::button::Style {
                                background: Some(iced::Background::Color(iced::Color::from_rgb(
                                    0.9, 0.9, 0.9,
                                ))),
                                ..Default::default()
                            },
                            iced::widget::button::Status::Pressed => iced::widget::button::Style {
                                background: Some(iced::Background::Color(iced::Color::from_rgb(
                                    0.8, 0.8, 0.8,
                                ))),
                                ..Default::default()
                            },
                            _ => iced::widget::button::Style::default(),
                        },
                    )
                    .into()
            } else {
                text(icon).size(14).into()
            }
        } else {
            // Leaf node - just spacing
            container(text("")).width(Length::Fixed(20.0)).into()
        };

        // Node label with optional icon
        let label_text = if let Some(ref icon) = tree_node.icon {
            format!("{} {}", icon, tree_node.label)
        } else {
            tree_node.label.clone()
        };

        let label = text(label_text).size(14);

        // Selection styling
        let label_element: Element<'a, HandlerMessage, Theme, Renderer> = if let Some(event) =
            parent_node
                .events
                .iter()
                .find(|e| matches!(e.event, dampen_core::ir::node::EventKind::Select))
        {
            let handler_name = event.handler.clone();
            let node_id = tree_node.id.clone();
            let message_factory = self.message_factory.clone();

            button(label)
                .on_press((message_factory)(
                    &handler_name,
                    Some(format!("\"{}\"", node_id)),
                ))
                .style(
                    move |_theme: &Theme, status: iced::widget::button::Status| {
                        match status {
                            iced::widget::button::Status::Active => {
                                if is_selected {
                                    iced::widget::button::Style {
                                        background: Some(iced::Background::Color(
                                            iced::Color::from_rgb(0.0, 0.48, 0.8),
                                        )),
                                        text_color: iced::Color::WHITE,
                                        ..Default::default()
                                    }
                                } else {
                                    iced::widget::button::Style::default()
                                }
                            }
                            iced::widget::button::Status::Hovered => {
                                if is_selected {
                                    // Selected + hovered: slightly lighter blue
                                    iced::widget::button::Style {
                                        background: Some(iced::Background::Color(
                                            iced::Color::from_rgb(0.1, 0.58, 0.9),
                                        )),
                                        text_color: iced::Color::WHITE,
                                        ..Default::default()
                                    }
                                } else {
                                    // Not selected + hovered: light gray background
                                    iced::widget::button::Style {
                                        background: Some(iced::Background::Color(
                                            iced::Color::from_rgb(0.9, 0.9, 0.9),
                                        )),
                                        ..Default::default()
                                    }
                                }
                            }
                            iced::widget::button::Status::Pressed => {
                                // Pressed state: darker blue if selected, gray if not
                                if is_selected {
                                    iced::widget::button::Style {
                                        background: Some(iced::Background::Color(
                                            iced::Color::from_rgb(0.0, 0.38, 0.7),
                                        )),
                                        text_color: iced::Color::WHITE,
                                        ..Default::default()
                                    }
                                } else {
                                    iced::widget::button::Style {
                                        background: Some(iced::Background::Color(
                                            iced::Color::from_rgb(0.8, 0.8, 0.8),
                                        )),
                                        ..Default::default()
                                    }
                                }
                            }
                            iced::widget::button::Status::Disabled => {
                                iced::widget::button::Style::default()
                            }
                        }
                    },
                )
                .into()
        } else {
            label.into()
        };

        // Node row
        let node_row = row(vec![toggle_button, label_element])
            .spacing(4)
            .padding(iced::Padding::from([0.0, indent]));

        // If expanded and has children, render them recursively
        if is_expanded && has_children {
            let child_elements: Vec<Element<'a, HandlerMessage, Theme, Renderer>> = tree_node
                .children
                .into_iter()
                .map(|child| {
                    self.build_tree_node_recursive(
                        child,
                        expanded_ids,
                        selected_id,
                        indent_size,
                        node_height,
                        expand_icon,
                        collapse_icon,
                        depth + 1,
                        parent_node,
                    )
                })
                .collect();

            column(vec![
                node_row.into(),
                column(child_elements).spacing(2).into(),
            ])
            .spacing(2)
            .into()
        } else {
            node_row.into()
        }
    }
}

/// Data structure representing a tree node
#[derive(Debug, Clone)]
#[allow(dead_code)]
struct TreeNodeData {
    id: String,
    label: String,
    icon: Option<String>,
    expanded: Option<bool>,
    selected: Option<bool>,
    disabled: Option<bool>,
    children: Vec<TreeNodeData>,
}

/// Extract tree node data from a BindingValue
fn extract_tree_node_data(value: &BindingValue) -> Option<TreeNodeData> {
    match value {
        BindingValue::Object(map) => {
            let id = map.get("id").and_then(|v| match v {
                BindingValue::String(s) => Some(s.clone()),
                _ => None,
            })?;

            let label = map
                .get("label")
                .and_then(|v| match v {
                    BindingValue::String(s) => Some(s.clone()),
                    _ => None,
                })
                .unwrap_or_else(|| id.clone());

            let icon = map.get("icon").and_then(|v| match v {
                BindingValue::String(s) => Some(s.clone()),
                _ => None,
            });

            let expanded = map.get("expanded").and_then(|v| match v {
                BindingValue::Bool(b) => Some(*b),
                _ => None,
            });

            let selected = map.get("selected").and_then(|v| match v {
                BindingValue::Bool(b) => Some(*b),
                _ => None,
            });

            let disabled = map.get("disabled").and_then(|v| match v {
                BindingValue::Bool(b) => Some(*b),
                _ => None,
            });

            let children = map
                .get("children")
                .and_then(|v| match v {
                    BindingValue::List(items) => {
                        Some(items.iter().filter_map(extract_tree_node_data).collect())
                    }
                    _ => Some(Vec::new()),
                })
                .unwrap_or_default();

            Some(TreeNodeData {
                id,
                label,
                icon,
                expanded,
                selected,
                disabled,
                children,
            })
        }
        _ => None,
    }
}

/// Extract tree node data from a WidgetNode (inline XML)
fn extract_tree_node_from_widget(
    node: &WidgetNode,
    _expanded_ids: &[String],
    _selected_id: Option<&String>,
) -> TreeNodeData {
    let id = node
        .attributes
        .get("id")
        .map(|a| match a {
            AttributeValue::Static(s) => s.clone(),
            _ => "unknown".to_string(),
        })
        .unwrap_or_else(|| "unknown".to_string());

    let label = node
        .attributes
        .get("label")
        .map(|a| match a {
            AttributeValue::Static(s) => s.clone(),
            _ => id.clone(),
        })
        .unwrap_or_else(|| id.clone());

    let icon = node.attributes.get("icon").and_then(|a| match a {
        AttributeValue::Static(s) => Some(s.clone()),
        _ => None,
    });

    let expanded = node.attributes.get("expanded").and_then(|a| match a {
        AttributeValue::Static(s) => s.parse::<bool>().ok(),
        _ => None,
    });

    let selected = node.attributes.get("selected").and_then(|a| match a {
        AttributeValue::Static(s) => s.parse::<bool>().ok(),
        _ => None,
    });

    let disabled = node.attributes.get("disabled").and_then(|a| match a {
        AttributeValue::Static(s) => s.parse::<bool>().ok(),
        _ => None,
    });

    let children: Vec<TreeNodeData> = node
        .children
        .iter()
        .filter(|c| c.kind == WidgetKind::TreeNode)
        .map(|child| extract_tree_node_from_widget(child, _expanded_ids, _selected_id))
        .collect();

    TreeNodeData {
        id,
        label,
        icon,
        expanded,
        selected,
        disabled,
        children,
    }
}
