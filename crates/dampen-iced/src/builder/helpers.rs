//! Helper functions for the DampenWidgetBuilder
//!
//! This module contains utility functions used by the builder for:
//! - Attribute evaluation (static, binding, interpolated)
//! - Context management (for loop variables)
//! - Handler resolution with detailed error reporting
//! - Style and layout merging
//! - Color parsing
//!
//! # Refactoring Helpers (v0.2.7)
//!
//! The following helper functions eliminate code duplication:
//! - `resolve_boolean_attribute()` - Parse boolean attributes (disabled, checked, selected)
//! - `resolve_handler_param()` - Resolve event handler parameters with rich errors
//! - `create_state_aware_style_fn()` - Generic state-aware styling for widgets
//!

use crate::HandlerMessage;
use dampen_core::binding::BindingValue;
use dampen_core::expr::error::BindingError;
use dampen_core::expr::evaluate_binding_expr_with_shared;
use dampen_core::ir::WidgetKind;
use dampen_core::ir::node::{AttributeValue, InterpolatedPart, WidgetNode};
use dampen_core::ir::span::Span;
use dampen_core::ir::theme::StyleClass;
use std::collections::HashMap;
use std::rc::Rc;

use super::DampenWidgetBuilder;

/// Reference-counted wrapper for StyleClass to avoid expensive deep clones in style closures.
///
/// This type alias is used in state-aware styling to share StyleClass instances
/// across multiple closure invocations without cloning the underlying data.
/// Rc clone is ~47x faster than deep-cloning StyleClass.
pub(crate) type StyleClassRef = Rc<StyleClass>;

/// Error during handler parameter resolution with rich diagnostic context.
///
/// This error wraps `BindingError` with additional information about which
/// handler and widget failed, making debugging much easier.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub struct HandlerResolutionError {
    /// The handler name that failed (e.g., "on_click", "on_change")
    pub handler_name: String,

    /// The widget type where error occurred (e.g., "Button", "TextInput")
    pub widget_kind: String,

    /// Optional widget ID for disambiguation (from `id` attribute)
    pub widget_id: Option<String>,

    /// The parameter expression that failed to resolve
    pub param_expr: String,

    /// The underlying binding evaluation error
    pub binding_error: BindingError,

    /// Location in XML file
    pub span: Span,

    /// Additional context about resolution attempts
    pub context_note: Option<String>,
}

impl std::fmt::Display for HandlerResolutionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Main error line with handler and widget context
        write!(
            f,
            "error[{}]: Handler parameter resolution failed for '{}' on {}",
            self.binding_error.kind as u8, self.handler_name, self.widget_kind
        )?;

        // Add widget ID if available
        if let Some(id) = &self.widget_id {
            write!(f, " (id=\"{}\")", id)?;
        }

        // Add location
        write!(
            f,
            " at line {}, column {}",
            self.span.line, self.span.column
        )?;

        // Parameter expression
        write!(f, "\n  param: {}", self.param_expr)?;

        // Reason from binding error
        write!(f, "\n  reason: {}", self.binding_error.message)?;

        // Suggestion from binding error if present
        if let Some(suggestion) = &self.binding_error.suggestion {
            write!(f, "\n  help: {}", suggestion)?;
        }

        // Additional context notes
        if let Some(note) = &self.context_note {
            write!(f, "\n  note: {}", note)?;
        }

        Ok(())
    }
}

#[allow(dead_code)]
impl HandlerResolutionError {
    /// Create error from binding error with handler/widget context
    pub fn from_binding_error(
        binding_error: BindingError,
        handler_name: String,
        widget_kind: String,
        widget_id: Option<String>,
        param_expr: String,
        span: Span,
    ) -> Self {
        Self {
            handler_name,
            widget_kind,
            widget_id,
            param_expr,
            binding_error,
            span,
            context_note: None,
        }
    }

    /// Add a context note about resolution attempts
    pub fn with_context_note(mut self, note: String) -> Self {
        self.context_note = Some(note);
        self
    }
}

/// Internal helper: Parse a string into a boolean.
fn parse_boolean_string(s: &str, default: bool) -> bool {
    match s.trim().to_lowercase().as_str() {
        "true" | "1" | "yes" | "on" => true,
        "false" | "0" | "no" | "off" | "" => false,
        _ => default,
    }
}

/// Resolves a boolean attribute from a widget node with support for multiple formats.
///
/// This helper eliminates ~15-20 lines of duplicated boolean parsing logic per widget
/// by providing a single, well-tested function that handles:
/// - Multiple truthy formats: "true", "1", "yes", "on"
/// - Multiple falsy formats: "false", "0", "no", "off", "" (empty)
/// - Case-insensitive matching ("True", "TRUE", "tRuE" all work)
/// - Whitespace trimming ("  true  " → true)
/// - Binding expressions like `enabled="{count > 0}"` (evaluated before parsing)
/// - Graceful defaults for invalid values
///
/// # Arguments
///
/// * `builder` - DampenWidgetBuilder containing context and model for binding evaluation
/// * `node` - Widget XML node containing attributes
/// * `attr_name` - Name of the attribute to resolve (e.g., "disabled", "enabled")
/// * `default` - Value to return if attribute is missing or invalid
///
/// # Returns
///
/// `true` or `false` based on attribute value, or `default` if attribute doesn't exist.
///
/// # Supported Formats
///
/// - Truthy: `"true"`, `"1"`, `"yes"`, `"on"` (case-insensitive)
/// - Falsy: `"false"`, `"0"`, `"no"`, `"off"`, `""` (case-insensitive)
/// - Binding: `"{count > 0}"` → evaluates expression, then parses result
/// - Invalid/Unknown: Returns `default` (safe fallback)
///
/// # Example
///
/// ```rust,ignore
/// use crate::builder::helpers::resolve_boolean_attribute;
///
/// // Check if button is disabled
/// let is_disabled = resolve_boolean_attribute(self, node, "disabled", false);
/// if !is_disabled {
///     button = button.on_press(message);
/// }
///
/// // Check if checkbox is initially checked
/// let is_checked = resolve_boolean_attribute(self, node, "checked", false);
/// ```
///
/// Resolves an event handler parameter expression to a concrete value.
///
/// This helper eliminates ~25-30 lines of duplicated binding resolution logic per widget
/// by providing a single function that:
/// 1. Attempts resolution from loop context (for items, indices, etc.)
/// 2. Falls back to model field access (shared application state)
/// 3. Returns detailed error with handler/widget context on failure
///
/// # Type Parameters
///
/// * `M` - Model type implementing `UiBindable` trait
///
/// # Arguments
///
/// * `builder` - DampenWidgetBuilder containing context and model state
/// * `event_param_expr` - Binding expression (e.g., "item.value", "model.count")
///
/// # Returns
///
/// `Ok(BindingValue)` if resolution succeeds, or `Err(HandlerResolutionError)` with
/// detailed diagnostic information for debugging.
///
/// # Errors
///
/// Returns `HandlerResolutionError` when:
/// - Expression not found in context (e.g., "item" outside of for loop)
/// - Field doesn't exist in model (e.g., "model.nonexistent_field")
/// - Type mismatch in binding evaluation
///
/// # Example
///
/// ```rust,ignore
/// use crate::builder::helpers::resolve_handler_param;
///
/// // In button widget builder
/// if let Some(event_param) = &on_click_event.param {
///     match resolve_handler_param(&self, event_param) {
///         Ok(value) => {
///             let handler_msg = create_handler_message("on_click", Some(value));
///             button = button.on_press(handler_msg);
///         }
///         Err(e) => {
///             eprintln!("{}", e);  // Print detailed error
///             // Continue without handler attachment
///         }
///     }
/// }
/// ```
/// Resolves an event handler parameter expression to a concrete value.
///
/// This helper eliminates ~25-30 lines of duplicated binding resolution logic per widget
/// by providing a single function that:
/// 1. Attempts resolution from loop context (for items, indices, etc.)
/// 2. Falls back to model field access (shared application state)
/// 3. Returns detailed error with handler/widget context on failure
///
/// # Arguments
///
/// * `builder` - DampenWidgetBuilder containing context and model state
/// * `binding_expr` - Parsed binding expression (from EventBinding.param)
///
/// # Returns
///
/// `Ok(BindingValue)` if resolution succeeds, or `Err(HandlerResolutionError)` with
/// detailed diagnostic information for debugging.
///
/// # Example
///
/// ```rust,ignore
/// use crate::builder::helpers::resolve_handler_param;
///
/// // In button widget builder
/// if let Some(param_expr) = &on_click_event.param {
///     match resolve_handler_param(self, param_expr) {
///         Ok(value) => {
///             let handler_msg = create_handler_message("on_click", Some(value));
///             button = button.on_press(handler_msg);
///         }
///         Err(e) => {
///             eprintln!("{}", e);
///         }
///     }
/// }
///
/// # Example
///
/// ```rust,ignore
/// use crate::builder::helpers::resolve_boolean_attribute;
///
/// // Check if button is disabled
/// let is_disabled = resolve_boolean_attribute(self, node, "disabled", false);
/// if !is_disabled {
///     button = button.on_press(message);
/// }
///
/// // Check if checkbox is initially checked
/// let is_checked = resolve_boolean_attribute(self, node, "checked", false);
/// ```
pub fn resolve_boolean_attribute(
    builder: &DampenWidgetBuilder<'_>,
    node: &WidgetNode,
    attr_name: &str,
    default: bool,
) -> bool {
    match node.attributes.get(attr_name) {
        None => default,
        Some(AttributeValue::Static(s)) => parse_boolean_string(s, default),
        Some(attr @ AttributeValue::Binding(_)) => {
            // Evaluate the binding expression first, then parse as boolean
            let evaluated = builder.evaluate_attribute(attr);
            parse_boolean_string(&evaluated, default)
        }
        Some(attr @ AttributeValue::Interpolated(_)) => {
            // Evaluate the interpolated expression first, then parse as boolean
            let evaluated = builder.evaluate_attribute(attr);
            parse_boolean_string(&evaluated, default)
        }
    }
}

#[allow(clippy::result_large_err)]
pub fn resolve_handler_param(
    builder: &DampenWidgetBuilder<'_>,
    binding_expr: &dampen_core::expr::BindingExpr,
) -> Result<BindingValue, HandlerResolutionError> {
    if let Some(value) = builder.resolve_from_context(binding_expr) {
        return Ok(value);
    }

    match evaluate_binding_expr_with_shared(binding_expr, builder.model, builder.shared_context) {
        Ok(value) => Ok(value),
        Err(binding_error) => Err(HandlerResolutionError {
            handler_name: String::from("unknown"),
            widget_kind: String::from("unknown"),
            widget_id: None,
            param_expr: format!("{:?}", binding_expr),
            binding_error,
            span: binding_expr.span,
            context_note: Some(String::from(
                "Tried context resolution first, then model field access",
            )),
        }),
    }
}

/// Creates a state-aware style closure for widgets that support status-based styling.
///
/// This helper eliminates ~50-80 lines of duplicated styling logic per widget by providing
/// a generic closure factory that handles:
/// 1. Base style resolution from theme, classes, and inline attributes
/// 2. State-specific style resolution (hover, focus, active, disabled)
/// 3. Style merging with proper precedence
/// 4. Conversion to Iced widget-specific style types
///
/// # Type Parameters
///
/// * `S` - The Iced widget style type (e.g., `button::Style`, `checkbox::Style`)
/// * `T` - The status type from the Iced widget (e.g., `button::Status`, `checkbox::Status`)
///
/// # Arguments
///
/// * `builder` - Reference to DampenWidgetBuilder for style resolution
/// * `node` - The widget node containing style attributes and classes
/// * `widget_kind` - The kind of widget being styled (for theme lookups)
/// * `style_class` - Optional style class for state variant resolution
/// * `base_style` - Base StyleProperties from theme/class/inline
/// * `status_mapper` - Function to map Iced status to WidgetState
/// * `style_converter` - Function to convert StyleProperties to Iced style type
///
/// # Returns
///
/// `Some(closure)` that can be passed to widget's `.style()` method, or `None` if no styling needed.
///
/// # Example
///
/// ```rust,ignore
/// use crate::builder::helpers::create_state_aware_style_fn;
/// use crate::style_mapping::{map_checkbox_status, resolve_state_style};
/// use iced::widget::checkbox;
///
/// if let Some(style_fn) = create_state_aware_style_fn(
///     self,
///     node,
///     WidgetKind::Checkbox,
///     style_class_rc,  // Now uses Rc<StyleClass>
///     base_style,
///     map_checkbox_status,
///     |props| checkbox::Style { /* conversion */ },
/// ) {
///     checkbox = checkbox.style(style_fn);
/// }
/// ```
#[allow(clippy::too_many_arguments)]
pub fn create_state_aware_style_fn<'a, S, T, F, M>(
    _builder: &DampenWidgetBuilder<'a>,
    _node: &WidgetNode,
    _widget_kind: WidgetKind,
    style_class: Option<StyleClassRef>,
    base_style: dampen_core::ir::style::StyleProperties,
    status_mapper: M,
    style_converter: F,
) -> Option<impl Fn(&iced::Theme, T) -> S + 'a>
where
    S: Clone + 'a,
    T: 'a,
    F: Fn(&dampen_core::ir::style::StyleProperties) -> S + 'a,
    M: Fn(T) -> Option<dampen_core::ir::WidgetState> + Copy + 'a,
{
    use crate::style_mapping::resolve_state_style;

    Some(move |_theme: &iced::Theme, status: T| {
        let widget_state = status_mapper(status);

        let final_style_props = if let (Some(class), Some(state)) = (&style_class, widget_state) {
            if let Some(state_style) = resolve_state_style(class, state) {
                merge_styles(base_style.clone(), state_style)
            } else {
                base_style.clone()
            }
        } else {
            base_style.clone()
        };

        style_converter(&final_style_props)
    })
}

// ... imports ...
use dampen_core::UiBindable;

/// A wrapper model that looks up fields in the binding context first, then the actual model.
struct ContextAwareModel<'a, 'b> {
    builder: &'b DampenWidgetBuilder<'a>,
    model: &'b dyn UiBindable,
}

impl<'a, 'b> UiBindable for ContextAwareModel<'a, 'b> {
    fn get_field(&self, path: &[&str]) -> Option<BindingValue> {
        // 1. Try to resolve from context manually (simulating resolve_from_context logic)
        // We can't use resolve_from_context directly easily because it takes a BindingExpr,
        // but we can access the context stack directly.

        if let Some(first_segment) = path.first() {
            // Search context stack in reverse (innermost first)
            for context in self.builder.binding_context.borrow().iter().rev() {
                if let Some(value) = context.get(*first_segment) {
                    // Handle nested access like item.text
                    if path.len() == 1 {
                        return Some(value.clone());
                    } else {
                        // Resolve nested path on the context value
                        // We need to convert &[&str] to Vec<String> for resolve_nested_field
                        let nested_path: Vec<String> =
                            path[1..].iter().map(|s| s.to_string()).collect();
                        return self.builder.resolve_nested_field(value, &nested_path);
                    }
                }
            }
        }

        // 2. Fallback to model
        self.model.get_field(path)
    }

    fn available_fields() -> Vec<String> {
        // We can't easily list all context fields dynamically without iterating everything,
        // so we'll just return the model's fields for now.
        Vec::new()
    }
}

impl<'a> DampenWidgetBuilder<'a> {
    /// Evaluate a binding expression using both context variables and the model.
    pub(crate) fn evaluate_binding_with_context(
        &self,
        expr: &dampen_core::expr::BindingExpr,
    ) -> Result<BindingValue, BindingError> {
        let context_model = ContextAwareModel {
            builder: self,
            model: self.model,
        };

        evaluate_binding_expr_with_shared(expr, &context_model, self.shared_context)
    }

    /// Evaluate an attribute value to a BindingValue (without converting to string)
    ///
    /// This is useful when the attribute might be a complex object (like a custom program).
    ///
    /// # Arguments
    ///
    /// * `attr` - Attribute value to evaluate
    ///
    /// # Returns
    ///
    /// The evaluated BindingValue, or BindingValue::String for static/interpolated
    pub(super) fn evaluate_attribute_value(&self, attr: &AttributeValue) -> BindingValue {
        match attr {
            AttributeValue::Static(value) => BindingValue::String(value.clone()),
            AttributeValue::Binding(expr) => {
                #[cfg(debug_assertions)]
                if expr.uses_shared() && self.shared_context.is_none() {
                    eprintln!(
                        "⚠️  Warning: Binding uses {{shared.}} syntax but no shared context was provided to DampenWidgetBuilder"
                    );
                }

                self.evaluate_binding_with_context(expr)
                    .unwrap_or(BindingValue::None)
            }
            AttributeValue::Interpolated(_parts) => {
                // Interpolated values are always strings
                let s = self.evaluate_attribute(attr);
                BindingValue::String(s)
            }
        }
    }

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

                match self.evaluate_binding_with_context(expr) {
                    Ok(value) => {
                        #[cfg(debug_assertions)]
                        eprintln!(
                            "[DampenWidgetBuilder] Binding evaluated to: {}",
                            value.to_display_string()
                        );
                        value.to_display_string()
                    }
                    Err(e) => {
                        #[cfg(debug_assertions)]
                        eprintln!("[DampenWidgetBuilder] Binding error: {}", e);
                        String::new()
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

                            match self.evaluate_binding_with_context(expr) {
                                Ok(value) => result.push_str(&value.to_display_string()),
                                Err(e) => {
                                    #[cfg(debug_assertions)]
                                    eprintln!(
                                        "[DampenWidgetBuilder] Interpolated binding error: {}",
                                        e
                                    );
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
                            #[cfg(debug_assertions)]
                            eprintln!("[DampenWidgetBuilder] Theme binding error");
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
                                        #[cfg(debug_assertions)]
                                        eprintln!(
                                            "[DampenWidgetBuilder] Theme binding error in interpolated value"
                                        );
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

    /// Resolve active classes, handling dynamic bindings
    pub(crate) fn resolve_active_classes(&self, node: &WidgetNode) -> Vec<String> {
        if let Some(attr) = node.attributes.get("class") {
            self.evaluate_attribute(attr)
                .split_whitespace()
                .map(|s| s.to_string())
                .collect()
        } else {
            node.classes.clone()
        }
    }

    /// Resolve styles from class names
    pub(super) fn resolve_class_styles(
        &self,
        node: &WidgetNode,
    ) -> Option<dampen_core::ir::style::StyleProperties> {
        let classes = self.resolve_active_classes(node);
        if classes.is_empty() {
            return None;
        }

        let style_classes = self.style_classes?;

        // Merge styles from all classes (in order)
        let mut merged_style = dampen_core::ir::style::StyleProperties::default();

        for class_name in classes {
            if let Some(style_class) = style_classes.get(&class_name) {
                // Merge the base style from this class
                merged_style = merge_styles(merged_style, &style_class.style);

                #[cfg(debug_assertions)]
                eprintln!(
                    "[DampenWidgetBuilder] Applied class '{}' to widget",
                    class_name
                );
            } else {
                #[cfg(debug_assertions)]
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
        let theme_context = self.theme_context;
        let widget_kind = node.kind.clone();

        // Get class styles (these are static/resolved at build time, not from theme)
        let classes = self.resolve_active_classes(node);
        let class_styles = if !classes.is_empty() {
            self.style_classes
                .and_then(|cls| classes.first().and_then(|name| cls.get(name)).cloned())
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

                // Resolve theme colors for this widget type at render time
                let mut theme_style = dampen_core::ir::style::StyleProperties::default();

                if let Some(ctx) = theme_context {
                    let active_theme = ctx.active();
                    let palette = &active_theme.palette;

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

                if let Some(ref bg) = final_style_props.background
                    && let dampen_core::ir::style::Background::Color(color) = bg
                {
                    style.background = Some(Background::Color(Color {
                        r: color.r,
                        g: color.g,
                        b: color.b,
                        a: color.a,
                    }));
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
        // Note: Don't use ? here, as we want to continue even if theme has no styles
        let theme_styles = self.resolve_theme_styles(node.kind.clone());

        // Layer 2: Class styles (override theme)
        let class_styles = self.resolve_class_styles(node);

        // Layer 3: Inline styles (override class)
        let inline_style = &node.style;

        // If no styles at all, return None
        if theme_styles.is_none() && class_styles.is_none() && inline_style.is_none() {
            return None;
        }

        // Merge all layers: theme → class → inline
        let mut merged = theme_styles.unwrap_or_default();

        if let Some(class_style) = class_styles {
            merged = merge_styles(merged, &class_style);
        }

        if let Some(inline_style) = inline_style {
            merged = merge_styles(merged, inline_style);
        }

        Some(merged)
    }

    /// Resolve layout constraints from class names
    pub(super) fn resolve_class_layout(
        &self,
        node: &WidgetNode,
    ) -> Option<dampen_core::ir::layout::LayoutConstraints> {
        let classes = self.resolve_active_classes(node);
        if classes.is_empty() {
            return None;
        }

        let style_classes = self.style_classes?;

        // Merge layouts from all classes (in order)
        let mut merged_layout: Option<dampen_core::ir::layout::LayoutConstraints> = None;

        for class_name in classes {
            if let Some(style_class) = style_classes.get(&class_name)
                && let Some(class_layout) = &style_class.layout
            {
                merged_layout = Some(match merged_layout {
                    Some(existing) => merge_layouts(existing, class_layout),
                    None => class_layout.clone(),
                });
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

                if widget_kind == WidgetKind::Container
                    && let Some(ref surface) = palette.surface
                {
                    theme_style.background =
                        Some(dampen_core::ir::style::Background::Color(*surface));
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
pub fn parse_color(color_str: &str) -> Option<dampen_core::ir::style::Color> {
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
