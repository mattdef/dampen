//! Helper functions for the DampenWidgetBuilder
//!
//! This module contains utility functions used by the builder for:
//! - Attribute evaluation (static, binding, interpolated)
//! - Context management (for loop variables)
//! - Handler resolution
//! - Style and layout merging
//! - Color parsing

use crate::HandlerMessage;
use dampen_core::binding::BindingValue;
use dampen_core::expr::evaluate_binding_expr_with_shared;
use dampen_core::ir::node::{AttributeValue, InterpolatedPart, WidgetNode};
use std::collections::HashMap;

use super::DampenWidgetBuilder;

impl<'a> DampenWidgetBuilder<'a> {
    /// Evaluate an attribute value (handles static, binding, and interpolated)
    ///
    /// # Attribute Types
    ///
    /// - **Static**: Direct string value, returned as-is
    /// - **Binding**: Expression like `{count}`, evaluated via model
    /// - **Interpolated**: Mixed literal and binding parts like `"Count: {count}"`
    ///
    /// # Arguments
    ///
    /// * `attr` - Attribute value to evaluate
    ///
    /// # Returns
    ///
    /// The evaluated string value
    pub(super) fn evaluate_attribute(&self, attr: &AttributeValue) -> String {
        match attr {
            AttributeValue::Static(value) => value.clone(),
            AttributeValue::Binding(expr) => {
                // Dev-mode warning: Check if binding uses shared state but no context provided
                #[cfg(debug_assertions)]
                if expr.uses_shared() && self.shared_context.is_none() {
                    eprintln!(
                        "⚠️  Warning: Binding uses {{shared.}} syntax but no shared context was provided to DampenWidgetBuilder"
                    );
                    eprintln!("    Hint: Use .with_shared(&shared_state) when building widgets");
                }

                // Try context first, then model
                if let Some(value) = self.resolve_from_context(expr) {
                    value.to_display_string()
                } else {
                    match evaluate_binding_expr_with_shared(expr, self.model, self.shared_context) {
                        Ok(value) => {
                            if self.verbose {
                                eprintln!(
                                    "[DampenWidgetBuilder] Binding evaluated to: {}",
                                    value.to_display_string()
                                );
                            }
                            value.to_display_string()
                        }
                        Err(e) => {
                            if self.verbose {
                                eprintln!("[DampenWidgetBuilder] Binding error: {}", e);
                            }
                            String::new()
                        }
                    }
                }
            }
            AttributeValue::Interpolated(parts) => {
                let mut result = String::new();
                for part in parts {
                    match part {
                        InterpolatedPart::Literal(lit) => result.push_str(lit),
                        InterpolatedPart::Binding(expr) => {
                            // Dev-mode warning: Check if binding uses shared state but no context provided
                            #[cfg(debug_assertions)]
                            if expr.uses_shared() && self.shared_context.is_none() {
                                eprintln!(
                                    "⚠️  Warning: Interpolated binding uses {{shared.}} syntax but no shared context was provided"
                                );
                                eprintln!(
                                    "    Hint: Use .with_shared(&shared_state) when building widgets"
                                );
                            }

                            if let Some(value) = self.resolve_from_context(expr) {
                                result.push_str(&value.to_display_string());
                            } else {
                                match evaluate_binding_expr_with_shared(
                                    expr,
                                    self.model,
                                    self.shared_context,
                                ) {
                                    Ok(value) => result.push_str(&value.to_display_string()),
                                    Err(e) => {
                                        if self.verbose {
                                            eprintln!(
                                                "[DampenWidgetBuilder] Interpolated binding error: {}",
                                                e
                                            );
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                result
            }
        }
    }

    /// Push a new binding context for loop variables
    ///
    /// Used by <for> widgets to make loop variables accessible in nested widgets.
    ///
    /// # Arguments
    ///
    /// * `var_name` - Name of the loop variable (e.g., "item")
    /// * `value` - Current value for this iteration
    pub(super) fn push_context(&self, var_name: &str, value: BindingValue) {
        let mut ctx = HashMap::new();
        ctx.insert(var_name.to_string(), value);
        self.binding_context.borrow_mut().push(ctx);
    }

    /// Pop the most recent binding context
    pub(super) fn pop_context(&self) {
        self.binding_context.borrow_mut().pop();
    }

    /// Try to resolve a binding expression from the context stack
    ///
    /// Returns None if the expression doesn't reference a context variable.
    pub(super) fn resolve_from_context(
        &self,
        binding_expr: &dampen_core::expr::BindingExpr,
    ) -> Option<BindingValue> {
        use dampen_core::expr::Expr;

        match &binding_expr.expr {
            Expr::FieldAccess(field_access) => {
                if let Some(first_segment) = field_access.path.first() {
                    // Search context stack in reverse (innermost first)
                    for context in self.binding_context.borrow().iter().rev() {
                        if let Some(value) = context.get(first_segment.as_str()) {
                            // Handle nested access like {item.text}
                            if field_access.path.len() == 1 {
                                return Some(value.clone());
                            } else {
                                // Resolve nested path on the context value
                                return self.resolve_nested_field(value, &field_access.path[1..]);
                            }
                        }
                    }
                }
                None
            }
            _ => None, // Other expression types not supported in context
        }
    }

    /// Resolve nested field access on a BindingValue (e.g., item.text)
    pub(super) fn resolve_nested_field(
        &self,
        value: &BindingValue,
        path: &[String],
    ) -> Option<BindingValue> {
        if path.is_empty() {
            return Some(value.clone());
        }

        match value {
            BindingValue::Object(map) => {
                if let Some(field_value) = map.get(&path[0]) {
                    if path.len() == 1 {
                        Some(field_value.clone())
                    } else {
                        self.resolve_nested_field(field_value, &path[1..])
                    }
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    /// Get optional message from handler name
    ///
    /// Looks up the handler name in the registry and creates a message
    /// using the configured message factory.
    ///
    /// # Arguments
    ///
    /// * `handler_name` - Name of the handler to look up
    ///
    /// # Returns
    ///
    /// `Some(message)` if handler exists, `None` otherwise
    pub(super) fn get_handler_message(&self, handler_name: &str) -> Option<HandlerMessage>
    where
        HandlerMessage: Clone + 'static,
    {
        if let Some(registry) = self.handler_registry {
            if registry.contains(handler_name) {
                if self.verbose {
                    eprintln!("[DampenWidgetBuilder] Handler '{}' found", handler_name);
                }
                return Some((self.message_factory)(handler_name, None));
            }
        }

        if self.verbose {
            eprintln!(
                "[DampenWidgetBuilder] Handler '{}' not found in registry",
                handler_name
            );
        }
        None
    }

    /// Resolve styles from class names
    pub(super) fn resolve_class_styles(
        &self,
        node: &WidgetNode,
    ) -> Option<dampen_core::ir::style::StyleProperties> {
        if node.classes.is_empty() {
            return None;
        }

        let style_classes = self.style_classes?;

        // Merge styles from all classes (in order)
        let mut merged_style = dampen_core::ir::style::StyleProperties::default();

        for class_name in &node.classes {
            if let Some(style_class) = style_classes.get(class_name) {
                // Merge the base style from this class
                merged_style = merge_styles(merged_style, &style_class.style);

                if self.verbose {
                    eprintln!(
                        "[DampenWidgetBuilder] Applied class '{}' to widget",
                        class_name
                    );
                }
            } else if self.verbose {
                eprintln!("[DampenWidgetBuilder] Class '{}' not found", class_name);
            }
        }

        Some(merged_style)
    }

    /// Resolve layout constraints from class names
    pub(super) fn resolve_class_layout(
        &self,
        node: &WidgetNode,
    ) -> Option<dampen_core::ir::layout::LayoutConstraints> {
        if node.classes.is_empty() {
            return None;
        }

        let style_classes = self.style_classes?;

        // Merge layouts from all classes (in order)
        let mut merged_layout: Option<dampen_core::ir::layout::LayoutConstraints> = None;

        for class_name in &node.classes {
            if let Some(style_class) = style_classes.get(class_name) {
                if let Some(class_layout) = &style_class.layout {
                    merged_layout = Some(match merged_layout {
                        Some(existing) => merge_layouts(existing, class_layout),
                        None => class_layout.clone(),
                    });
                }
            }
        }

        merged_layout
    }

    /// Resolve complete layout (class layouts + node layout)
    pub(super) fn resolve_layout(
        &self,
        node: &WidgetNode,
    ) -> Option<dampen_core::ir::layout::LayoutConstraints> {
        match (self.resolve_class_layout(node), &node.layout) {
            (Some(class_layout), Some(node_layout)) => {
                Some(merge_layouts(class_layout, node_layout))
            }
            (Some(class_layout), None) => Some(class_layout),
            (None, Some(node_layout)) => Some(node_layout.clone()),
            (None, None) => None,
        }
    }

    /// Apply style and layout to a widget
    pub(super) fn apply_style_layout<'b, W>(
        &self,
        widget: W,
        node: &WidgetNode,
    ) -> iced::Element<'a, HandlerMessage, iced::Theme, iced::Renderer>
    where
        W: Into<iced::Element<'a, HandlerMessage, iced::Theme, iced::Renderer>>,
        HandlerMessage: Clone + 'static,
    {
        use crate::convert::map_layout_constraints;
        use iced::widget::container;

        let element: iced::Element<'a, HandlerMessage, iced::Theme, iced::Renderer> = widget.into();

        // Resolve styles: class styles first, then node styles override
        let resolved_style = match (self.resolve_class_styles(node), &node.style) {
            (Some(class_style), Some(node_style)) => Some(merge_styles(class_style, node_style)),
            (Some(class_style), None) => Some(class_style),
            (None, Some(node_style)) => Some(node_style.clone()),
            (None, None) => None,
        };

        // Resolve layouts: use the helper to avoid duplication
        let resolved_layout = self.resolve_layout(node);

        // Check if we need to wrap in a container
        // We only wrap if there's style OR layout properties that need a container
        // (spacing doesn't need a container wrapper - it's applied to the widget itself)
        let needs_container_for_layout = if let Some(layout) = &resolved_layout {
            layout.width.is_some()
                || layout.height.is_some()
                || layout.padding.is_some()
                || layout.align_items.is_some()
                || layout.min_width.is_some()
                || layout.max_width.is_some()
                || layout.min_height.is_some()
                || layout.max_height.is_some()
        } else {
            false
        };

        let has_style = resolved_style.is_some();

        if !needs_container_for_layout && !has_style {
            return element;
        }

        let mut container = container(element);

        // Apply layout constraints (includes padding, width, height, etc.)
        if let Some(layout) = &resolved_layout {
            let iced_layout = map_layout_constraints(layout);

            // Apply width/height/padding/alignment
            if layout.width.is_some() {
                container = container.width(iced_layout.width);
            }
            if layout.height.is_some() {
                container = container.height(iced_layout.height);
            }
            container = container.padding(iced_layout.padding);
            if let Some(align) = iced_layout.align_items {
                container = container.align_y(align);
            }
        }

        // Apply resolved style (visual properties)
        if let Some(style) = resolved_style {
            use crate::convert::map_style_properties;
            let iced_style = map_style_properties(&style);
            container = container.style(move |_theme| iced_style);
        }

        container.into()
    }
}

/// Parse a color string (hex format) into a Color
///
/// Supports:
/// - `#RRGGBB` format (e.g., `#FF5733`)
/// - `#RGB` format (e.g., `#F53`)
///
/// Returns `None` if the format is invalid
pub(super) fn parse_color(color_str: &str) -> Option<dampen_core::ir::style::Color> {
    let hex = color_str.trim().trim_start_matches('#');

    let (r, g, b) = if hex.len() == 6 {
        // #RRGGBB
        let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
        let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
        let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
        (r, g, b)
    } else if hex.len() == 3 {
        // #RGB -> #RRGGBB
        let r = u8::from_str_radix(&hex[0..1], 16).ok()?;
        let g = u8::from_str_radix(&hex[1..2], 16).ok()?;
        let b = u8::from_str_radix(&hex[2..3], 16).ok()?;
        (r * 17, g * 17, b * 17) // Expand to full range
    } else {
        return None;
    };

    Some(dampen_core::ir::style::Color {
        r: r as f32 / 255.0,
        g: g as f32 / 255.0,
        b: b as f32 / 255.0,
        a: 1.0,
    })
}

/// Merge two StyleProperties, with the second one taking precedence
pub(super) fn merge_styles(
    base: dampen_core::ir::style::StyleProperties,
    override_style: &dampen_core::ir::style::StyleProperties,
) -> dampen_core::ir::style::StyleProperties {
    use dampen_core::ir::style::StyleProperties;

    StyleProperties {
        background: override_style.background.clone().or(base.background),
        color: override_style.color.or(base.color),
        border: override_style.border.clone().or(base.border),
        shadow: override_style.shadow.or(base.shadow),
        opacity: override_style.opacity.or(base.opacity),
        transform: override_style.transform.clone().or(base.transform),
    }
}

/// Merge two LayoutConstraints, with the second one taking precedence
pub(super) fn merge_layouts(
    base: dampen_core::ir::layout::LayoutConstraints,
    override_layout: &dampen_core::ir::layout::LayoutConstraints,
) -> dampen_core::ir::layout::LayoutConstraints {
    use dampen_core::ir::layout::LayoutConstraints;

    LayoutConstraints {
        width: override_layout.width.clone().or(base.width),
        height: override_layout.height.clone().or(base.height),
        min_width: override_layout.min_width.or(base.min_width),
        max_width: override_layout.max_width.or(base.max_width),
        min_height: override_layout.min_height.or(base.min_height),
        max_height: override_layout.max_height.or(base.max_height),
        padding: override_layout.padding.clone().or(base.padding),
        spacing: override_layout.spacing.or(base.spacing),
        align_items: override_layout.align_items.or(base.align_items),
        justify_content: override_layout.justify_content.or(base.justify_content),
        align_self: override_layout.align_self.or(base.align_self),
        direction: override_layout.direction.or(base.direction),
        position: override_layout.position.or(base.position),
        top: override_layout.top.or(base.top),
        right: override_layout.right.or(base.right),
        bottom: override_layout.bottom.or(base.bottom),
        left: override_layout.left.or(base.left),
        z_index: override_layout.z_index.or(base.z_index),
    }
}
