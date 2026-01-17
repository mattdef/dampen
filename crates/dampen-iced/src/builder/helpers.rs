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
use dampen_core::ir::WidgetKind;
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

    /// Resolve the theme name from a theme_ref attribute value
    ///
    /// Supports:
    /// - Static values: `theme="dark"` → returns "dark"
    /// - Binding expressions: `theme="{model.theme}"` → evaluates binding
    /// - Interpolated: `theme="custom-{model.variant}"` → combines literal and binding
    ///
    /// # Arguments
    ///
    /// * `theme_ref` - The theme reference attribute value (may be None)
    ///
    /// # Returns
    ///
    /// The resolved theme name as a string, or None if no theme is specified
    #[allow(dead_code)]
    pub(super) fn resolve_theme(&self, theme_ref: &Option<AttributeValue>) -> Option<String> {
        match theme_ref {
            None => None,
            Some(AttributeValue::Static(name)) => Some(name.clone()),
            Some(AttributeValue::Binding(expr)) => {
                // Try context first, then model
                if let Some(value) = self.resolve_from_context(expr) {
                    Some(value.to_display_string())
                } else {
                    match evaluate_binding_expr_with_shared(expr, self.model, self.shared_context) {
                        Ok(value) => Some(value.to_display_string()),
                        Err(_) => {
                            if self.verbose {
                                eprintln!("[DampenWidgetBuilder] Theme binding error");
                            }
                            None
                        }
                    }
                }
            }
            Some(AttributeValue::Interpolated(parts)) => {
                let mut result = String::new();
                for part in parts {
                    match part {
                        InterpolatedPart::Literal(lit) => result.push_str(lit),
                        InterpolatedPart::Binding(expr) => {
                            if let Some(value) = self.resolve_from_context(expr) {
                                result.push_str(&value.to_display_string());
                            } else {
                                match evaluate_binding_expr_with_shared(
                                    expr,
                                    self.model,
                                    self.shared_context,
                                ) {
                                    Ok(value) => result.push_str(&value.to_display_string()),
                                    Err(_) => {
                                        if self.verbose {
                                            eprintln!(
                                                "[DampenWidgetBuilder] Theme binding error in interpolated value"
                                            );
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                if result.is_empty() {
                    None
                } else {
                    Some(result)
                }
            }
        }
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

    /// Resolve theme-based styles from the active theme context
    ///
    /// This extracts color defaults from the theme palette for use as widget base styles.
    /// Returns `None` if no theme context is available.
    ///
    /// # Theme Colors Used
    ///
    /// - `primary` → background for buttons, interactive elements
    /// - `text` → text color for labels
    /// - `background` → container backgrounds
    /// - `surface` → card/pane backgrounds
    ///
    /// # Arguments
    ///
    /// * `widget_kind` - The kind of widget being styled (affects which colors to use)
    ///
    /// # Returns
    ///
    /// StyleProperties with theme colors, or None if no theme context
    pub(super) fn resolve_theme_styles(
        &self,
        widget_kind: WidgetKind,
    ) -> Option<dampen_core::ir::style::StyleProperties> {
        let theme_ctx = self.theme_context?;
        let theme = theme_ctx.active();
        let palette = &theme.palette;

        // Determine which theme colors to use based on widget type
        let mut style = dampen_core::ir::style::StyleProperties::default();

        match widget_kind {
            WidgetKind::Button => {
                // Buttons use primary color for background
                if let Some(ref primary) = palette.primary {
                    style.background = Some(dampen_core::ir::style::Background::Color(*primary));
                }
                // Buttons use text color
                if let Some(ref text) = palette.text {
                    style.color = Some(*text);
                }
            }
            WidgetKind::Container => {
                // Containers use background/surface colors
                if let Some(ref surface) = palette.surface {
                    style.background = Some(dampen_core::ir::style::Background::Color(*surface));
                }
            }
            WidgetKind::Text => {
                // Text widgets use text color
                if let Some(ref text) = palette.text {
                    style.color = Some(*text);
                }
            }
            _ => {
                // Other widgets: use text color if available
                if let Some(ref text) = palette.text {
                    style.color = Some(*text);
                }
            }
        }

        // Check if we actually got any theme colors
        if style.background.is_none() && style.color.is_none() {
            None
        } else {
            Some(style)
        }
    }

    /// Create a style closure that resolves theme colors at render time.
    ///
    /// This is the key to making theme switching work visually. Instead of
    /// resolving theme colors at build time (which would capture them permanently),
    /// this method returns a closure that looks up colors from the active theme
    /// each time Iced renders the widget.
    ///
    /// # Arguments
    ///
    /// * `node` - The widget node being styled (for inline styles and classes)
    ///
    /// # Returns
    ///
    /// A closure that can be passed to Iced widget's `.style()` method
    pub(super) fn create_theme_aware_style_closure(
        &self,
        node: &WidgetNode,
    ) -> Option<
        impl Fn(&iced::Theme, iced::widget::button::Status) -> iced::widget::button::Style + 'a,
    > {
        let theme_context = self.theme_context?;
        let widget_kind = node.kind.clone();

        // Get class styles (these are static, not from theme)
        let class_styles = if !node.classes.is_empty() {
            self.style_classes.and_then(|classes| {
                node.classes
                    .first()
                    .and_then(|name| classes.get(name))
                    .cloned()
            })
        } else {
            None
        };

        // Get inline styles (also static)
        let inline_style = node.style.clone();

        Some(
            move |_theme: &iced::Theme, status: iced::widget::button::Status| {
                use crate::style_mapping::{
                    map_button_status, merge_style_properties, resolve_state_style,
                };
                use iced::widget::button;
                use iced::{Background, Border, Color};

                // Get the active theme at RUNTIME (not build time!)
                let active_theme = theme_context.active();
                let palette = &active_theme.palette;

                // Resolve theme colors for this widget type at render time
                let mut theme_style = dampen_core::ir::style::StyleProperties::default();

                match widget_kind {
                    WidgetKind::Container => {
                        if let Some(ref surface) = palette.surface {
                            theme_style.background =
                                Some(dampen_core::ir::style::Background::Color(*surface));
                        }
                    }
                    WidgetKind::Button => {
                        if let Some(ref primary) = palette.primary {
                            theme_style.background =
                                Some(dampen_core::ir::style::Background::Color(*primary));
                        }
                        if let Some(ref text) = palette.text {
                            theme_style.color = Some(*text);
                        }
                    }
                    WidgetKind::Text => {
                        if let Some(ref text) = palette.text {
                            theme_style.color = Some(*text);
                        }
                    }
                    _ => {
                        if let Some(ref text) = palette.text {
                            theme_style.color = Some(*text);
                        }
                    }
                }

                // Merge theme with class styles (theme is base, class overrides)
                let mut merged = theme_style;
                if let Some(ref class_style) = class_styles {
                    merged = merge_styles(merged, &class_style.style);
                }

                // Merge with inline styles (highest precedence)
                if let Some(ref inline) = inline_style {
                    merged = merge_styles(merged, inline);
                }

                // Handle state variants
                let final_style_props = if let (Some(class), Some(state)) =
                    (&class_styles, map_button_status(status))
                {
                    if let Some(state_style) = resolve_state_style(class, state) {
                        merge_style_properties(&merged, state_style)
                    } else {
                        merged
                    }
                } else {
                    merged
                };

                // Convert to Iced style
                let mut style = button::Style::default();

                if let Some(ref bg) = final_style_props.background {
                    if let dampen_core::ir::style::Background::Color(color) = bg {
                        style.background = Some(Background::Color(Color {
                            r: color.r,
                            g: color.g,
                            b: color.b,
                            a: color.a,
                        }));
                    }
                }

                if let Some(ref text_color) = final_style_props.color {
                    style.text_color = Color {
                        r: text_color.r,
                        g: text_color.g,
                        b: text_color.b,
                        a: text_color.a,
                    };
                }

                if let Some(ref border) = final_style_props.border {
                    style.border = Border {
                        color: Color {
                            r: border.color.r,
                            g: border.color.g,
                            b: border.color.b,
                            a: border.color.a,
                        },
                        width: border.width,
                        radius: iced::border::Radius {
                            top_left: border.radius.top_left,
                            top_right: border.radius.top_right,
                            bottom_right: border.radius.bottom_right,
                            bottom_left: border.radius.bottom_left,
                        },
                    };
                }

                if let Some(ref shadow) = final_style_props.shadow {
                    style.shadow = iced::Shadow {
                        color: Color {
                            r: shadow.color.r,
                            g: shadow.color.g,
                            b: shadow.color.b,
                            a: shadow.color.a,
                        },
                        offset: iced::Vector {
                            x: shadow.offset_x,
                            y: shadow.offset_y,
                        },
                        blur_radius: shadow.blur_radius,
                    };
                }

                style
            },
        )
    }

    /// Resolve complete styles with proper precedence: theme → class → inline
    ///
    /// This is the main entry point for style resolution, combining:
    /// 1. Theme palette colors (base defaults)
    /// 2. Style class definitions (override theme)
    /// 3. Inline node styles (override class)
    ///
    /// # Arguments
    ///
    /// * `node` - The widget node being styled
    ///
    /// # Returns
    ///
    /// Merged StyleProperties, or None if no styles are defined
    pub(super) fn resolve_complete_styles(
        &self,
        node: &WidgetNode,
    ) -> Option<dampen_core::ir::style::StyleProperties> {
        // Layer 1: Theme styles (base)
        let theme_styles = self.resolve_theme_styles(node.kind.clone())?;

        // Layer 2: Class styles (override theme)
        let class_styles = self.resolve_class_styles(node);

        // Layer 3: Inline styles (override class)
        let inline_style = &node.style;

        // Merge all layers: theme → class → inline
        let merged = match (class_styles, inline_style) {
            (Some(class_style), Some(inline_style)) => {
                // Merge class on top of theme, then inline on top of class
                let theme_then_class = merge_styles(theme_styles, &class_style);
                merge_styles(theme_then_class, inline_style)
            }
            (Some(class_style), None) => {
                // Only theme + class
                merge_styles(theme_styles, &class_style)
            }
            (None, Some(inline_style)) => {
                // Only theme + inline
                merge_styles(theme_styles, inline_style)
            }
            (None, None) => theme_styles,
        };

        Some(merged)
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

        // Resolve class and inline styles (static, not from theme)
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
        let has_theme_context = self.theme_context.is_some();

        if !needs_container_for_layout && !has_style && !has_theme_context {
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
            if let Some(align_x) = iced_layout.align_x {
                container = container.align_x(align_x);
            }
            if let Some(align_y) = iced_layout.align_y {
                container = container.align_y(align_y);
            }
        }

        // Apply resolved style (visual properties) with theme-aware styling
        if self.theme_context.is_some() {
            // Use theme-aware styling that resolves colors at render time
            let widget_kind = node.kind.clone();
            let resolved_style = resolved_style.clone();
            let theme_context = self.theme_context;

            container = container.style(move |_theme: &iced::Theme| {
                use crate::convert::map_style_properties;

                // Get the active theme at RUNTIME (not build time!)
                let ctx = match theme_context {
                    Some(ctx) => ctx,
                    None => {
                        return map_style_properties(&resolved_style.clone().unwrap_or_default());
                    }
                };
                let active_theme = ctx.active();
                let palette = &active_theme.palette;

                // Resolve theme colors for this widget type at render time
                let mut theme_style = dampen_core::ir::style::StyleProperties::default();

                if widget_kind == WidgetKind::Container {
                    if let Some(ref surface) = palette.surface {
                        theme_style.background =
                            Some(dampen_core::ir::style::Background::Color(*surface));
                    }
                }

                // Merge theme with static styles (theme is base, static overrides)
                let merged = match resolved_style.as_ref() {
                    Some(static_style) => merge_styles(theme_style, static_style),
                    None => theme_style,
                };

                map_style_properties(&merged)
            });
        } else if let Some(style) = resolved_style {
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

/// Parse a length value from string (e.g., "fill", "shrink", "100", "50%")
///
/// # Supported Formats
///
/// - `"fill"` → `Length::Fill`
/// - `"shrink"` → `Length::Shrink`
/// - `"100"` → `Length::Fixed(100.0)`
/// - `"50%"` → `Length::FillPortion(8)` (approximation)
///
/// # Arguments
///
/// * `s` - Length string to parse
///
/// # Returns
///
/// Parsed `iced::Length` or `None` if invalid
pub(super) fn parse_length(s: &str) -> Option<iced::Length> {
    let s = s.trim().to_lowercase();
    if s == "fill" {
        Some(iced::Length::Fill)
    } else if s == "shrink" {
        Some(iced::Length::Shrink)
    } else if let Some(pct) = s.strip_suffix('%') {
        if let Ok(p) = pct.parse::<f32>() {
            // Iced doesn't have a direct percentage, use FillPortion as approximation
            let portion = ((p / 100.0) * 16.0).round() as u16;
            let portion = portion.max(1);
            Some(iced::Length::FillPortion(portion))
        } else {
            None
        }
    } else if let Ok(px) = s.parse::<f32>() {
        Some(iced::Length::Fixed(px))
    } else {
        None
    }
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
        align_x: override_layout.align_x.or(base.align_x),
        align_y: override_layout.align_y.or(base.align_y),
        direction: override_layout.direction.or(base.direction),
        position: override_layout.position.or(base.position),
        top: override_layout.top.or(base.top),
        right: override_layout.right.or(base.right),
        bottom: override_layout.bottom.or(base.bottom),
        left: override_layout.left.or(base.left),
        z_index: override_layout.z_index.or(base.z_index),
    }
}
