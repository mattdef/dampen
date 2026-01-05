//! Gravity Widget Builder - Automatic interpretation of Gravity markup
//!
//! This module provides the GravityWidgetBuilder which automatically converts
//! parsed Gravity UI definitions into Iced widgets with full support for
//! bindings, events, styles, and layouts.
//!
//! # Overview
//!
//! The builder eliminates manual widget rendering by automatically:
//! - Evaluating bindings like `{count}` or `{user.name}`
//! - Connecting event handlers via `on_click`, `on_input`, etc.
//! - Applying styles and layouts from attributes
//! - Recursively processing nested widget trees
//!
//! # Basic Usage
//!
//! ```rust
//! use gravity_core::{parse, HandlerRegistry};
//! use gravity_iced::GravityWidgetBuilder;
//! use gravity_macros::UiModel;
//! use serde::{Deserialize, Serialize};
//!
//! #[derive(UiModel, Serialize, Deserialize, Clone)]
//! struct Model { count: i32 }
//!
//! let xml = r#"<text value="{count}" />"#;
//! let document = parse(xml).unwrap();
//! let model = Model { count: 42 };
//!
//! let element = GravityWidgetBuilder::new(
//!     &document.root,
//!     &model,
//!     None,
//! ).build();
//! ```
//!
//! # Features
//!
//! - **Automatic binding evaluation**: Supports field access, method calls, conditionals
//! - **Event handling**: Connects handlers from `HandlerRegistry`
//! - **Style application**: Applies padding, spacing, colors, borders
//! - **Layout constraints**: Handles width, height, alignment
//! - **Verbose logging**: Debug mode for development
//! - **Error handling**: Graceful degradation with logging
//!
//! # Performance
//!
//! - 100 widgets: ~0.027ms
//! - 1000 widgets: ~0.284ms
//! - Binding evaluation: ~713ns per widget
//!
//! See the [README](../README.md) for complete documentation.

use crate::convert::{map_layout_constraints, map_style_properties};
use crate::state::WidgetStateManager;
use crate::HandlerMessage;
use gravity_core::binding::{BindingValue, UiBindable};
use gravity_core::expr::evaluate_binding_expr;
use gravity_core::handler::HandlerRegistry;
use gravity_core::ir::node::{AttributeValue, InterpolatedPart, WidgetNode};
use gravity_core::ir::theme::StyleClass;
use gravity_core::ir::WidgetKind;
#[allow(unused_imports)]
use iced::widget::{checkbox, image, pick_list, slider, text_input, toggler};
use iced::{Element, Renderer, Theme};
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Builder for creating Iced widgets from Gravity markup
///
/// # Construction
///
/// Use one of these constructors:
/// - [`GravityWidgetBuilder::new()`] - Standard constructor with HandlerMessage
/// - [`GravityWidgetBuilder::from_document()`] - From complete GravityDocument
/// - [`GravityWidgetBuilder::new_with_factory()`] - Custom message factory
///
/// # Configuration
///
/// After construction, chain configuration methods:
/// - [`with_verbose()`] - Enable debug logging
/// - [`with_style_classes()`] - Add theme classes
///
/// # Execution
///
/// Call [`build()`] to render the widget tree.
///
/// # Example
///
/// ```rust
/// use gravity_core::{parse, HandlerRegistry};
/// use gravity_iced::{GravityWidgetBuilder, HandlerMessage};
/// use gravity_macros::{ui_handler, UiModel};
/// use serde::{Deserialize, Serialize};
/// use std::any::Any;
///
/// #[derive(UiModel, Serialize, Deserialize, Clone)]
/// struct Model { count: i32 }
///
/// #[ui_handler]
/// fn increment(model: &mut Model) { model.count += 1; }
///
/// let xml = r#"<button label="+" on_click="increment" />"#;
/// let document = parse(xml).unwrap();
/// let model = Model { count: 0 };
///
/// let registry = HandlerRegistry::new();
/// registry.register_simple("increment", |m: &mut dyn Any| {
///     let model = m.downcast_mut::<Model>().unwrap();
///     increment(model);
/// });
///
/// let element = GravityWidgetBuilder::new(
///     &document.root,
///     &model,
///     Some(®istry),
/// ).build();
/// ```
pub struct GravityWidgetBuilder<'a> {
    /// The root widget node from parsed XML
    node: &'a WidgetNode,

    /// Application state for binding evaluation
    model: &'a dyn UiBindable,

    /// Optional registry for event handler lookup
    handler_registry: Option<&'a HandlerRegistry>,

    /// Optional style classes for theme support
    style_classes: Option<&'a HashMap<String, StyleClass>>,

    /// Enable verbose logging for debugging
    verbose: bool,

    /// Factory function to create messages from handler names
    message_factory: Box<dyn Fn(&str, Option<String>) -> HandlerMessage + 'a>,

    /// Shared state manager for widget state tracking
    state_manager: Arc<Mutex<WidgetStateManager>>,

    /// Binding context stack for <for> loop variables
    /// Each context maps variable names to their BindingValues
    binding_context: RefCell<Vec<HashMap<String, BindingValue>>>,
}

impl<'a> GravityWidgetBuilder<'a> {
    /// Create a new widget builder using the standard HandlerMessage type
    ///
    /// # Arguments
    ///
    /// * `node` - Root widget node from parsed XML
    /// * `model` - Application state implementing `UiBindable`
    /// * `handler_registry` - Optional registry for event handlers
    ///
    /// # Example
    ///
    /// ```rust
    /// use gravity_core::{parse, HandlerRegistry};
    /// use gravity_iced::{GravityWidgetBuilder, HandlerMessage};
    /// use gravity_macros::UiModel;
    /// use serde::{Deserialize, Serialize};
    ///
    /// #[derive(UiModel, Serialize, Deserialize, Clone)]
    /// struct Model { count: i32 }
    ///
    /// let xml = r#"<text value="Hello" />"#;
    /// let document = parse(xml).unwrap();
    /// let model = Model { count: 0 };
    /// let registry = HandlerRegistry::new();
    ///
    /// let builder = GravityWidgetBuilder::new(
    ///     &document.root,
    ///     &model,
    ///     Some(®istry),
    /// );
    /// ```
    pub fn new(
        node: &'a WidgetNode,
        model: &'a dyn UiBindable,
        handler_registry: Option<&'a HandlerRegistry>,
    ) -> Self {
        Self {
            node,
            model,
            handler_registry,
            style_classes: None,
            verbose: false,
            message_factory: Box::new(|name, value| {
                HandlerMessage::Handler(name.to_string(), value)
            }),
            state_manager: Arc::new(Mutex::new(WidgetStateManager::new())),
            binding_context: RefCell::new(Vec::new()),
        }
    }

    /// Create a new widget builder from a complete GravityDocument
    ///
    /// This constructor automatically extracts the root node and style classes
    /// from the document, providing a convenient way to work with parsed documents.
    ///
    /// # Arguments
    ///
    /// * `document` - Complete GravityDocument from parser
    /// * `model` - Application state implementing `UiBindable`
    /// * `handler_registry` - Optional registry for event handlers
    ///
    /// # Example
    ///
    /// ```rust
    /// use gravity_core::parse;
    /// use gravity_iced::GravityWidgetBuilder;
    /// use gravity_macros::UiModel;
    /// use serde::{Deserialize, Serialize};
    ///
    /// #[derive(UiModel, Serialize, Deserialize, Clone)]
    /// struct Model { count: i32 }
    ///
    /// let xml = r#"<gravity><themes>...</themes><column>...</column></gravity>"#;
    /// let document = parse(xml).unwrap();
    /// let model = Model { count: 0 };
    ///
    /// // Builder automatically uses document.root and document.style_classes
    /// let builder = GravityWidgetBuilder::from_document(
    ///     &document,
    ///     &model,
    ///     None,
    /// );
    /// ```
    pub fn from_document(
        document: &'a gravity_core::GravityDocument,
        model: &'a dyn UiBindable,
        handler_registry: Option<&'a HandlerRegistry>,
    ) -> Self {
        Self {
            node: &document.root,
            model,
            handler_registry,
            style_classes: Some(&document.style_classes),
            verbose: false,
            message_factory: Box::new(|name, value| {
                HandlerMessage::Handler(name.to_string(), value)
            }),
            state_manager: Arc::new(Mutex::new(WidgetStateManager::new())),
            binding_context: RefCell::new(Vec::new()),
        }
    }
}

impl<'a> GravityWidgetBuilder<'a> {
    /// Create a new widget builder with a custom message factory
    ///
    /// This is useful when you need custom message types instead of HandlerMessage.
    ///
    /// # Arguments
    ///
    /// * `node` - Root widget node
    /// * `model` - Application state
    /// * `handler_registry` - Optional event handler registry
    /// * `message_factory` - Function that converts handler names to messages
    ///
    /// # Example
    ///
    /// ```rust
    /// use gravity_core::{parse, HandlerRegistry};
    /// use gravity_iced::GravityWidgetBuilder;
    /// use gravity_macros::UiModel;
    /// use serde::{Deserialize, Serialize};
    ///
    /// #[derive(UiModel, Serialize, Deserialize, Clone)]
    /// struct Model { count: i32 }
    ///
    /// #[derive(Clone, Debug)]
    /// enum MyHandlerMessage {
    ///     Increment,
    ///     Decrement,
    /// }
    ///
    /// let xml = r#"<button label="+" on_click="increment" />"#;
    /// let document = parse(xml).unwrap();
    /// let model = Model { count: 0 };
    ///
    /// let builder = GravityWidgetBuilder::new_with_factory(
    ///     &document.root,
    ///     &model,
    ///     None,
    ///     |name| match name {
    ///         "increment" => MyHandlerMessage::Increment,
    ///         _ => MyHandlerMessage::Decrement,
    ///     },
    /// );
    /// ```
    pub fn new_with_factory<F>(
        node: &'a WidgetNode,
        model: &'a dyn UiBindable,
        handler_registry: Option<&'a HandlerRegistry>,
        message_factory: F,
    ) -> Self
    where
        F: Fn(&str, Option<String>) -> HandlerMessage + 'a,
    {
        Self {
            node,
            model,
            handler_registry,
            style_classes: None,
            verbose: false,
            message_factory: Box::new(message_factory),
            state_manager: Arc::new(Mutex::new(WidgetStateManager::new())),
            binding_context: RefCell::new(Vec::new()),
        }
    }

    /// Add style classes to the builder for theme support
    ///
    /// Style classes allow reusable styling definitions that can be applied
    /// to widgets via the `class` attribute in XML.
    ///
    /// # Arguments
    ///
    /// * `style_classes` - HashMap of style class names to definitions
    ///
    /// # Example
    ///
    /// ```rust
    /// use gravity_core::ir::theme::StyleClass;
    /// use std::collections::HashMap;
    ///
    /// let mut classes = HashMap::new();
    /// classes.insert("primary".to_string(), StyleClass { /* ... */ });
    ///
    /// let builder = GravityWidgetBuilder::new(/* ... */)
    ///     .with_style_classes(&classes);
    /// ```
    pub fn with_style_classes(mut self, style_classes: &'a HashMap<String, StyleClass>) -> Self {
        self.style_classes = Some(style_classes);
        self
    }

    /// Enable verbose logging for debugging
    ///
    /// This prints detailed information about widget building,
    /// event handler attachment, and parameter evaluation.
    ///
    /// # Example
    ///
    /// ```rust
    /// let builder = GravityWidgetBuilder::new(/* ... */)
    ///     .verbose(true);
    /// ```
    pub fn with_verbose(mut self, verbose: bool) -> Self {
        self.verbose = verbose;
        self
    }

    /// Build the widget tree and return an Iced Element
    ///
    /// This is the main entry point that processes the entire widget tree,
    /// evaluates all bindings, connects events, and applies styles.
    ///
    /// # Returns
    ///
    /// An Iced `Element` ready to be used in your application's view
    ///
    /// # Type Requirements
    ///
    /// `HandlerMessage` must implement `Clone` and be `'static`
    ///
    /// # Example
    ///
    /// ```rust
    /// use iced::Element;
    ///
    /// let builder = GravityWidgetBuilder::new(/* ... */);
    /// let element: Element<'_, HandlerMessage> = builder.build();
    /// ```
    pub fn build(self) -> Element<'a, HandlerMessage, Theme, Renderer>
    where
        HandlerMessage: Clone + 'static,
    {
        self.build_widget(self.node)
    }

    /// Recursively build a widget from a node
    ///
    /// This is the core dispatcher that routes to widget-specific builders
    /// based on the node's `WidgetKind`.
    ///
    /// # Arguments
    ///
    /// * `node` - Widget node to build
    ///
    /// # Returns
    ///
    /// An Iced Element representing the widget
    fn build_widget(&self, node: &WidgetNode) -> Element<'a, HandlerMessage, Theme, Renderer>
    where
        HandlerMessage: Clone + 'static,
    {
        if self.verbose {
            eprintln!("[GravityWidgetBuilder] Building widget: {:?}", node.kind);
        }
        match node.kind {
            WidgetKind::Text => self.build_text(node),
            WidgetKind::Button => self.build_button(node),
            WidgetKind::Column => self.build_column(node),
            WidgetKind::Row => self.build_row(node),
            WidgetKind::Container => self.build_container(node),
            WidgetKind::TextInput => self.build_text_input(node),
            WidgetKind::Checkbox => self.build_checkbox(node),
            WidgetKind::Slider => self.build_slider(node),
            WidgetKind::PickList => self.build_pick_list(node),
            WidgetKind::Toggler => self.build_toggler(node),
            WidgetKind::Image => self.build_image(node),
            WidgetKind::Scrollable => self.build_scrollable(node),
            WidgetKind::Stack => self.build_stack(node),
            WidgetKind::Space => self.build_space(node),
            WidgetKind::Rule => self.build_rule(node),
            WidgetKind::Svg => self.build_svg(node),
            WidgetKind::Custom(_) => self.build_custom(node),
            WidgetKind::ComboBox => todo!("ComboBox not yet implemented"),
            WidgetKind::ProgressBar => self.build_progress_bar(node),
            WidgetKind::Tooltip => self.build_tooltip(node),
            WidgetKind::Grid => self.build_grid(node),
            WidgetKind::Canvas => self.build_canvas(node),
            WidgetKind::Float => todo!("Float not yet implemented"),
            WidgetKind::For => self.build_for(node),
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
    fn evaluate_attribute(&self, attr: &AttributeValue) -> String {
        match attr {
            AttributeValue::Static(value) => value.clone(),
            AttributeValue::Binding(expr) => {
                // Try context first, then model
                if let Some(value) = self.resolve_from_context(expr) {
                    value.to_display_string()
                } else {
                    match evaluate_binding_expr(expr, self.model) {
                        Ok(value) => {
                            if self.verbose {
                                eprintln!(
                                    "[GravityWidgetBuilder] Binding evaluated to: {}",
                                    value.to_display_string()
                                );
                            }
                            value.to_display_string()
                        }
                        Err(e) => {
                            if self.verbose {
                                eprintln!("[GravityWidgetBuilder] Binding error: {}", e);
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
                            if let Some(value) = self.resolve_from_context(expr) {
                                result.push_str(&value.to_display_string());
                            } else {
                                match evaluate_binding_expr(expr, self.model) {
                                    Ok(value) => result.push_str(&value.to_display_string()),
                                    Err(e) => {
                                        if self.verbose {
                                            eprintln!(
                                                "[GravityWidgetBuilder] Interpolated binding error: {}",
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
    fn push_context(&self, var_name: &str, value: BindingValue) {
        let mut ctx = HashMap::new();
        ctx.insert(var_name.to_string(), value);
        self.binding_context.borrow_mut().push(ctx);
    }

    /// Pop the most recent binding context
    fn pop_context(&self) {
        self.binding_context.borrow_mut().pop();
    }

    /// Try to resolve a binding expression from the context stack
    ///
    /// Returns None if the expression doesn't reference a context variable.
    fn resolve_from_context(
        &self,
        binding_expr: &gravity_core::expr::BindingExpr,
    ) -> Option<BindingValue> {
        use gravity_core::expr::Expr;

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
    fn resolve_nested_field(&self, value: &BindingValue, path: &[String]) -> Option<BindingValue> {
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
    fn get_handler_message(&self, handler_name: &str) -> Option<HandlerMessage>
    where
        HandlerMessage: Clone + 'static,
    {
        if let Some(registry) = self.handler_registry {
            if registry.contains(handler_name) {
                if self.verbose {
                    eprintln!("[GravityWidgetBuilder] Handler '{}' found", handler_name);
                }
                return Some((self.message_factory)(handler_name, None));
            }
        }

        if self.verbose {
            eprintln!(
                "[GravityWidgetBuilder] Handler '{}' not found in registry",
                handler_name
            );
        }
        None
    }

    /// Resolve styles from class names
    fn resolve_class_styles(
        &self,
        node: &WidgetNode,
    ) -> Option<gravity_core::ir::style::StyleProperties> {
        if node.classes.is_empty() {
            return None;
        }

        let style_classes = self.style_classes?;

        // Merge styles from all classes (in order)
        let mut merged_style = gravity_core::ir::style::StyleProperties::default();

        for class_name in &node.classes {
            if let Some(style_class) = style_classes.get(class_name) {
                // Merge the base style from this class
                merged_style = merge_styles(merged_style, &style_class.style);

                if self.verbose {
                    eprintln!(
                        "[GravityWidgetBuilder] Applied class '{}' to widget",
                        class_name
                    );
                }
            } else if self.verbose {
                eprintln!("[GravityWidgetBuilder] Class '{}' not found", class_name);
            }
        }

        Some(merged_style)
    }

    /// Resolve layout constraints from class names
    fn resolve_class_layout(
        &self,
        node: &WidgetNode,
    ) -> Option<gravity_core::ir::layout::LayoutConstraints> {
        if node.classes.is_empty() {
            return None;
        }

        let style_classes = self.style_classes?;

        // Merge layouts from all classes (in order)
        let mut merged_layout: Option<gravity_core::ir::layout::LayoutConstraints> = None;

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
    fn resolve_layout(
        &self,
        node: &WidgetNode,
    ) -> Option<gravity_core::ir::layout::LayoutConstraints> {
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
    fn apply_style_layout<'b, W>(
        &self,
        widget: W,
        node: &WidgetNode,
    ) -> Element<'a, HandlerMessage, Theme, Renderer>
    where
        W: Into<Element<'a, HandlerMessage, Theme, Renderer>>,
        HandlerMessage: Clone + 'static,
    {
        use iced::widget::container;

        let element: Element<'a, HandlerMessage, Theme, Renderer> = widget.into();

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
            let iced_style = map_style_properties(&style);
            container = container.style(move |_theme| iced_style);
        }

        container.into()
    }

    // Widget builders - will be fully implemented in Phase 3

    fn build_text(&self, node: &WidgetNode) -> Element<'a, HandlerMessage, Theme, Renderer>
    where
        HandlerMessage: Clone + 'static,
    {
        let value = node
            .attributes
            .get("value")
            .map(|attr| self.evaluate_attribute(attr))
            .unwrap_or_default();

        let mut text_widget = iced::widget::text(value);

        // Resolve and apply text styles
        let resolved_style = match (self.resolve_class_styles(node), &node.style) {
            (Some(class_style), Some(node_style)) => Some(merge_styles(class_style, node_style)),
            (Some(class_style), None) => Some(class_style),
            (None, Some(node_style)) => Some(node_style.clone()),
            (None, None) => None,
        };

        // Apply color from styles
        if let Some(style_props) = resolved_style {
            if let Some(ref color) = style_props.color {
                text_widget = text_widget.color(iced::Color {
                    r: color.r,
                    g: color.g,
                    b: color.b,
                    a: color.a,
                });
            }
        }

        // Check for direct attributes (size, weight, color) that override styles
        if let Some(color_attr) = node.attributes.get("color") {
            let color_str = self.evaluate_attribute(color_attr);
            if let Some(color) = parse_color(&color_str) {
                text_widget = text_widget.color(iced::Color {
                    r: color.r,
                    g: color.g,
                    b: color.b,
                    a: color.a,
                });
            }
        }

        if let Some(size_attr) = node.attributes.get("size") {
            let size_str = self.evaluate_attribute(size_attr);
            if let Ok(size) = size_str.parse::<f32>() {
                text_widget = text_widget.size(size);
            }
        }

        if let Some(weight_attr) = node.attributes.get("weight") {
            let weight_str = self.evaluate_attribute(weight_attr);
            if weight_str == "bold" {
                text_widget = text_widget.font(iced::Font {
                    weight: iced::font::Weight::Bold,
                    ..Default::default()
                });
            }
        }

        text_widget.into()
    }

    fn build_button(&self, node: &WidgetNode) -> Element<'a, HandlerMessage, Theme, Renderer>
    where
        HandlerMessage: Clone + 'static,
    {
        let label = node
            .attributes
            .get("label")
            .map(|attr| self.evaluate_attribute(attr))
            .unwrap_or_default();

        if self.verbose {
            eprintln!(
                "[GravityWidgetBuilder] Building button with label: '{}'",
                label
            );
        }

        // Get handler from events
        let on_click_event = node
            .events
            .iter()
            .find(|e| e.event == gravity_core::EventKind::Click);

        if self.verbose {
            if let Some(event) = &on_click_event {
                eprintln!(
                    "[GravityWidgetBuilder] Button has click event: handler={}, param={:?}",
                    event.handler, event.param
                );
            } else {
                eprintln!("[GravityWidgetBuilder] Button has no click event");
            }
        }

        let mut btn = iced::widget::button(iced::widget::text(label));

        // Resolve and apply button styles using button-specific style function
        let resolved_style = match (self.resolve_class_styles(node), &node.style) {
            (Some(class_style), Some(node_style)) => Some(merge_styles(class_style, node_style)),
            (Some(class_style), None) => Some(class_style),
            (None, Some(node_style)) => Some(node_style.clone()),
            (None, None) => None,
        };

        if let Some(style_props) = resolved_style {
            btn = btn.style(move |_theme, _status| {
                use iced::widget::button;
                use iced::{Background, Border, Color};

                let mut style = button::Style::default();

                // Apply background color
                if let Some(ref bg) = style_props.background {
                    if let gravity_core::ir::style::Background::Color(ref color) = bg {
                        style.background = Some(Background::Color(Color {
                            r: color.r,
                            g: color.g,
                            b: color.b,
                            a: color.a,
                        }));
                    }
                }

                // Apply text color
                if let Some(ref text_color) = style_props.color {
                    style.text_color = Color {
                        r: text_color.r,
                        g: text_color.g,
                        b: text_color.b,
                        a: text_color.a,
                    };
                }

                // Apply border
                if let Some(ref border) = style_props.border {
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

                // Apply shadow
                if let Some(ref shadow) = style_props.shadow {
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
            });
        }

        // Connect event if handler exists (AFTER style is applied)
        if let Some(event_binding) = on_click_event {
            if self.handler_registry.is_some() {
                let handler_name = event_binding.handler.clone();

                // Evaluate parameter if present
                let param_value = if let Some(param_expr) = &event_binding.param {
                    // Try context first (for {item.id} in for loop)
                    if let Some(value) = self.resolve_from_context(param_expr) {
                        if self.verbose {
                            eprintln!(
                                "[GravityWidgetBuilder] Button param from context: {:?} -> {}",
                                param_expr,
                                value.to_display_string()
                            );
                        }
                        Some(value.to_display_string())
                    } else {
                        // Fallback to model evaluation
                        match evaluate_binding_expr(param_expr, self.model) {
                            Ok(value) => {
                                if self.verbose {
                                    eprintln!("[GravityWidgetBuilder] Button param from model: {:?} -> {}", 
                                             param_expr, value.to_display_string());
                                }
                                Some(value.to_display_string())
                            }
                            Err(e) => {
                                if self.verbose {
                                    eprintln!("[GravityWidgetBuilder] Button param error: {}", e);
                                }
                                None
                            }
                        }
                    }
                } else {
                    if self.verbose {
                        eprintln!("[GravityWidgetBuilder] Button has no param");
                    }
                    None
                };

                if self.verbose {
                    eprintln!("[GravityWidgetBuilder] Button: Attaching on_press with handler '{}', param: {:?}",
                             handler_name, param_value);
                }

                // Clone param_value explicitly before creating HandlerMessage
                let param_cloned = param_value.clone();
                let handler_cloned = handler_name.clone();

                // Pass the HandlerMessage directly (on_press doesn't support closures)
                btn = btn.on_press(HandlerMessage::Handler(handler_cloned, param_cloned));
            } else {
                if self.verbose {
                    eprintln!("[GravityWidgetBuilder] Button: No handler_registry, cannot attach on_press");
                }
            }
        } else {
            if self.verbose {
                eprintln!("[GravityWidgetBuilder] Button: No on_click event found");
            }
        }

        btn.into()
    }

    fn build_column(&self, node: &WidgetNode) -> Element<'a, HandlerMessage, Theme, Renderer>
    where
        HandlerMessage: Clone + 'static,
    {
        let children: Vec<_> = node
            .children
            .iter()
            .map(|child| self.build_widget(child))
            .collect();

        let mut column = iced::widget::column(children);

        // Apply spacing and width/height from resolved layout
        if let Some(layout) = self.resolve_layout(node) {
            if let Some(spacing) = layout.spacing {
                column = column.spacing(spacing);
            }
            // Apply width and height directly to the column
            if layout.width.is_some() {
                let iced_layout = map_layout_constraints(&layout);
                column = column.width(iced_layout.width);
            }
            if layout.height.is_some() {
                let iced_layout = map_layout_constraints(&layout);
                column = column.height(iced_layout.height);
            }
        }

        self.apply_style_layout(column, node)
    }

    fn build_row(&self, node: &WidgetNode) -> Element<'a, HandlerMessage, Theme, Renderer>
    where
        HandlerMessage: Clone + 'static,
    {
        let children: Vec<_> = node
            .children
            .iter()
            .map(|child| self.build_widget(child))
            .collect();

        let mut row = iced::widget::row(children);

        // Apply spacing and width/height from resolved layout
        if let Some(layout) = self.resolve_layout(node) {
            if let Some(spacing) = layout.spacing {
                row = row.spacing(spacing);
            }
            // Apply width and height directly to the row
            if layout.width.is_some() {
                let iced_layout = map_layout_constraints(&layout);
                row = row.width(iced_layout.width);
            }
            if layout.height.is_some() {
                let iced_layout = map_layout_constraints(&layout);
                row = row.height(iced_layout.height);
            }
        }

        self.apply_style_layout(row, node)
    }

    fn build_container(&self, node: &WidgetNode) -> Element<'a, HandlerMessage, Theme, Renderer>
    where
        HandlerMessage: Clone + 'static,
    {
        // Container can have multiple children - wrap them in a column if needed
        match node.children.len() {
            0 => {
                // Empty container - use empty space
                self.apply_style_layout(iced::widget::Space::new(), node)
            }
            1 => {
                // Single child - use it directly
                let child = self.build_widget(&node.children[0]);
                self.apply_style_layout(child, node)
            }
            _ => {
                // Multiple children - wrap in a column
                let children: Vec<_> = node
                    .children
                    .iter()
                    .map(|child| self.build_widget(child))
                    .collect();
                let column = iced::widget::column(children);
                self.apply_style_layout(column, node)
            }
        }
    }

    /// Build a text input widget from Gravity XML definition
    ///
    /// Supports the following attributes:
    /// - `value`: String binding for current text value
    /// - `placeholder`: Placeholder text when empty
    /// - `on_input`: Handler called on text input with new value
    ///
    /// Events: Input (sends HandlerMessage::Handler(name, Some(new_text)))
    fn build_text_input(&self, node: &WidgetNode) -> Element<'a, HandlerMessage, Theme, Renderer> {
        let placeholder = node
            .attributes
            .get("placeholder")
            .map(|attr| self.evaluate_attribute(attr))
            .unwrap_or_default();

        let value = node
            .attributes
            .get("value")
            .map(|attr| self.evaluate_attribute(attr))
            .unwrap_or_default();

        if self.verbose {
            eprintln!(
                "[GravityWidgetBuilder] Building text_input: placeholder='{}', value='{}'",
                placeholder, value
            );
        }

        // Get handler from events
        let on_input = node
            .events
            .iter()
            .find(|e| e.event == gravity_core::EventKind::Input)
            .map(|e| e.handler.clone());

        if self.verbose {
            if let Some(handler) = &on_input {
                eprintln!(
                    "[GravityWidgetBuilder] TextInput has input event: handler={}",
                    handler
                );
            } else {
                eprintln!("[GravityWidgetBuilder] TextInput has no input event");
            }
        }

        let mut text_input = iced::widget::text_input(&placeholder, &value);

        // Connect event if handler exists
        if let Some(handler_name) = on_input {
            if self.handler_registry.is_some() {
                if self.verbose {
                    eprintln!(
                        "[GravityWidgetBuilder] TextInput: Attaching on_input with handler '{}'",
                        handler_name
                    );
                }
                text_input = text_input.on_input(move |input_value| {
                    HandlerMessage::Handler(handler_name.clone(), Some(input_value))
                });
            } else {
                if self.verbose {
                    eprintln!("[GravityWidgetBuilder] TextInput: No handler_registry, cannot attach on_input");
                }
            }
        }

        text_input.into()
    }

    /// Build a checkbox widget from Gravity XML definition
    ///
    /// Supports the following attributes:
    /// - `label`: Text label displayed next to checkbox
    /// - `checked`: Boolean binding for checked state
    /// - `on_toggle`: Handler called on toggle with "true"/"false"
    ///
    /// Events: Toggle (sends HandlerMessage::Handler(name, Some("true"|"false")))
    fn build_checkbox(&self, node: &WidgetNode) -> Element<'a, HandlerMessage, Theme, Renderer> {
        let label = node
            .attributes
            .get("label")
            .map(|attr| self.evaluate_attribute(attr))
            .unwrap_or_default();

        let checked_str = node
            .attributes
            .get("checked")
            .map(|attr| self.evaluate_attribute(attr))
            .unwrap_or_else(|| "false".to_string());

        let is_checked = checked_str == "true" || checked_str == "1";

        if self.verbose {
            eprintln!(
                "[GravityWidgetBuilder] Building checkbox: label='{}', checked={}",
                label, is_checked
            );
        }

        // Get handler from events
        let on_toggle_event = node
            .events
            .iter()
            .find(|e| e.event == gravity_core::EventKind::Toggle);

        if self.verbose {
            if let Some(event) = &on_toggle_event {
                eprintln!(
                    "[GravityWidgetBuilder] Checkbox has toggle event: handler={}, param={:?}",
                    event.handler, event.param
                );
            } else {
                eprintln!("[GravityWidgetBuilder] Checkbox has no toggle event");
            }
        }

        let mut checkbox = iced::widget::checkbox(is_checked);

        // Connect event if handler exists
        if let Some(event_binding) = on_toggle_event {
            if self.handler_registry.is_some() {
                let handler_name = event_binding.handler.clone();

                // Evaluate parameter if present, otherwise use toggle state
                let param_value = if let Some(param_expr) = &event_binding.param {
                    // Evaluate the parameter expression with context
                    if let Some(value) = self.resolve_from_context(param_expr) {
                        if self.verbose {
                            eprintln!(
                                "[GravityWidgetBuilder] Checkbox param from context: {:?} -> {}",
                                param_expr,
                                value.to_display_string()
                            );
                        }
                        Some(value.to_display_string())
                    } else {
                        match evaluate_binding_expr(param_expr, self.model) {
                            Ok(value) => {
                                if self.verbose {
                                    eprintln!("[GravityWidgetBuilder] Checkbox param from model: {:?} -> {}", 
                                             param_expr, value.to_display_string());
                                }
                                Some(value.to_display_string())
                            }
                            Err(e) => {
                                if self.verbose {
                                    eprintln!("[GravityWidgetBuilder] Checkbox param error: {}", e);
                                }
                                None
                            }
                        }
                    }
                } else {
                    if self.verbose {
                        eprintln!("[GravityWidgetBuilder] Checkbox has no param");
                    }
                    None
                };

                if self.verbose {
                    eprintln!("[GravityWidgetBuilder] Checkbox: Attaching on_toggle with handler '{}', param: {:?}",
                             handler_name, param_value);
                }

                checkbox = checkbox.on_toggle(move |new_checked| {
                    HandlerMessage::Handler(
                        handler_name.clone(),
                        param_value.clone().or_else(|| {
                            Some(if new_checked {
                                "true".to_string()
                            } else {
                                "false".to_string()
                            })
                        }),
                    )
                });
            } else {
                if self.verbose {
                    eprintln!("[GravityWidgetBuilder] Checkbox: No handler_registry, cannot attach on_toggle");
                }
            }
        }

        let text_widget = iced::widget::text(label);
        let row = iced::widget::row(vec![checkbox.into(), text_widget.into()]);
        row.into()
    }

    /// Build a slider widget from Gravity XML definition
    ///
    /// Supports the following attributes:
    /// - `min`: Minimum value (default 0.0)
    /// - `max`: Maximum value (default 100.0)
    /// - `value`: Float binding for current value (clamped to [min, max])
    /// - `on_change`: Handler called on change with stringified float value
    ///
    /// Events: Change (sends HandlerMessage::Handler(name, Some(value.to_string())))
    fn build_slider(&self, node: &WidgetNode) -> Element<'a, HandlerMessage, Theme, Renderer> {
        let min = node
            .attributes
            .get("min")
            .map(|attr| self.evaluate_attribute(attr))
            .unwrap_or_else(|| "0.0".to_string())
            .parse::<f32>()
            .unwrap_or(0.0);

        let max = node
            .attributes
            .get("max")
            .map(|attr| self.evaluate_attribute(attr))
            .unwrap_or_else(|| "100.0".to_string())
            .parse::<f32>()
            .unwrap_or(100.0);

        let value_str = node
            .attributes
            .get("value")
            .map(|attr| self.evaluate_attribute(attr))
            .unwrap_or_else(|| "50.0".to_string());

        let mut value = value_str.parse::<f32>().unwrap_or(50.0);

        // Clamp value to [min, max]
        value = value.max(min).min(max);

        // Get handler from events
        let on_change = node
            .events
            .iter()
            .find(|e| e.event == gravity_core::EventKind::Change)
            .map(|e| e.handler.clone());

        let slider = if let Some(handler_name) = on_change {
            if self.handler_registry.is_some() {
                iced::widget::slider(min..=max, value, move |new_value| {
                    HandlerMessage::Handler(handler_name.clone(), Some(new_value.to_string()))
                })
            } else {
                iced::widget::slider(min..=max, value, |_| {
                    HandlerMessage::Handler("dummy".to_string(), None)
                })
            }
        } else {
            iced::widget::slider(min..=max, value, |_| {
                HandlerMessage::Handler("dummy".to_string(), None)
            })
        };

        slider.into()
    }

    /// Build a pick list widget from Gravity XML definition
    ///
    /// Supports the following attributes:
    /// - `options`: Comma-separated list of options
    /// - `selected`: String binding for selected option
    /// - `placeholder`: Placeholder text (currently unused)
    /// - `on_select`: Handler called on selection with selected value
    ///
    /// Events: Select (sends HandlerMessage::Handler(name, Some(selected_value)))
    fn build_pick_list(&self, node: &WidgetNode) -> Element<'a, HandlerMessage, Theme, Renderer> {
        let options_str = node
            .attributes
            .get("options")
            .map(|attr| self.evaluate_attribute(attr))
            .unwrap_or_default();

        let options: Vec<String> = options_str
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        let selected_str = node
            .attributes
            .get("selected")
            .map(|attr| self.evaluate_attribute(attr))
            .unwrap_or_default();

        let selected = if selected_str.is_empty() {
            None
        } else {
            options.iter().find(|o| *o == &selected_str).cloned()
        };

        if self.verbose {
            eprintln!(
                "[GravityWidgetBuilder] Building pick_list: options={:?}, selected={:?}",
                options, selected
            );
        }

        // Get handler from events
        let on_select = node
            .events
            .iter()
            .find(|e| e.event == gravity_core::EventKind::Select)
            .map(|e| e.handler.clone());

        if self.verbose {
            if let Some(handler) = &on_select {
                eprintln!(
                    "[GravityWidgetBuilder] PickList has select event: handler={}",
                    handler
                );
            } else {
                eprintln!("[GravityWidgetBuilder] PickList has no select event");
            }
        }

        let pick_list = if let Some(handler_name) = on_select {
            if self.handler_registry.is_some() {
                if self.verbose {
                    eprintln!(
                        "[GravityWidgetBuilder] PickList: Attaching on_select with handler '{}'",
                        handler_name
                    );
                }
                iced::widget::pick_list(options, selected, move |selected_value| {
                    HandlerMessage::Handler(handler_name.clone(), Some(selected_value))
                })
            } else {
                if self.verbose {
                    eprintln!("[GravityWidgetBuilder] PickList: No handler_registry, cannot attach on_select");
                }
                iced::widget::pick_list(options, selected, |_| {
                    HandlerMessage::Handler("dummy".to_string(), None)
                })
            }
        } else {
            // If no handler, still need to provide one, but since no event, perhaps use a dummy
            iced::widget::pick_list(options, selected, |_| {
                HandlerMessage::Handler("dummy".to_string(), None)
            })
        };

        pick_list.into()
    }

    /// Build a toggler widget from Gravity XML definition
    ///
    /// Supports the following attributes:
    /// - `label`: Text label displayed next to toggler
    /// - `active`: Boolean binding for active state
    /// - `on_toggle`: Handler called on toggle with "true"/"false"
    ///
    /// Events: Toggle (sends HandlerMessage::Handler(name, Some("true"|"false")))
    fn build_toggler(&self, node: &WidgetNode) -> Element<'a, HandlerMessage, Theme, Renderer> {
        let label = node
            .attributes
            .get("label")
            .map(|attr| self.evaluate_attribute(attr))
            .unwrap_or_default();

        let active_str = node
            .attributes
            .get("active")
            .map(|attr| self.evaluate_attribute(attr))
            .unwrap_or_else(|| "false".to_string());

        let is_active = active_str == "true" || active_str == "1";

        if self.verbose {
            eprintln!(
                "[GravityWidgetBuilder] Building toggler: label='{}', active={}",
                label, is_active
            );
        }

        // Get handler from events
        let on_toggle = node
            .events
            .iter()
            .find(|e| e.event == gravity_core::EventKind::Toggle)
            .map(|e| e.handler.clone());

        if self.verbose {
            if let Some(handler) = &on_toggle {
                eprintln!(
                    "[GravityWidgetBuilder] Toggler has toggle event: handler={}",
                    handler
                );
            } else {
                eprintln!("[GravityWidgetBuilder] Toggler has no toggle event");
            }
        }

        let mut toggler = iced::widget::toggler(is_active);

        // Connect event if handler exists
        if let Some(handler_name) = on_toggle {
            if self.handler_registry.is_some() {
                if self.verbose {
                    eprintln!(
                        "[GravityWidgetBuilder] Toggler: Attaching on_toggle with handler '{}'",
                        handler_name
                    );
                }
                toggler = toggler.on_toggle(move |new_active| {
                    HandlerMessage::Handler(
                        handler_name.clone(),
                        Some(if new_active {
                            "true".to_string()
                        } else {
                            "false".to_string()
                        }),
                    )
                });
            } else {
                if self.verbose {
                    eprintln!("[GravityWidgetBuilder] Toggler: No handler_registry, cannot attach on_toggle");
                }
            }
        }

        let text_widget = iced::widget::text(label);
        let row = iced::widget::row(vec![toggler.into(), text_widget.into()]);
        row.into()
    }

    /// Build an image widget from Gravity XML definition
    ///
    /// Supports the following attributes:
    /// - `src`: Path to image file (required)
    /// - `width`: Display width in pixels
    /// - `height`: Display height in pixels
    ///
    /// No events - display only widget
    fn build_image(&self, node: &WidgetNode) -> Element<'a, HandlerMessage, Theme, Renderer> {
        // Support both 'src' (standard) and 'path' (legacy) attributes
        let src = node
            .attributes
            .get("src")
            .or_else(|| node.attributes.get("path"))
            .map(|attr| self.evaluate_attribute(attr))
            .unwrap_or_default();

        if src.is_empty() {
            if self.verbose {
                eprintln!("[GravityWidgetBuilder] Image src is empty");
            }
            return iced::widget::text("[Image: no src]").into();
        }

        let handle = iced::widget::image::Handle::from_path(src);

        let mut image = iced::widget::image(handle);

        if let Some(width_attr) = node.attributes.get("width") {
            if let Ok(width) = self.evaluate_attribute(width_attr).parse::<f32>() {
                image = image.width(width);
            }
        }

        if let Some(height_attr) = node.attributes.get("height") {
            if let Ok(height) = self.evaluate_attribute(height_attr).parse::<f32>() {
                image = image.height(height);
            }
        }

        image.into()
    }

    fn build_scrollable(&self, node: &WidgetNode) -> Element<'a, HandlerMessage, Theme, Renderer>
    where
        HandlerMessage: Clone + 'static,
    {
        if let Some(first_child) = node.children.first() {
            let child = self.build_widget(first_child);
            iced::widget::scrollable(child).into()
        } else {
            iced::widget::scrollable(iced::widget::text("")).into()
        }
    }

    fn build_stack(&self, node: &WidgetNode) -> Element<'a, HandlerMessage, Theme, Renderer>
    where
        HandlerMessage: Clone + 'static,
    {
        let children: Vec<_> = node
            .children
            .iter()
            .map(|child| self.build_widget(child))
            .collect();

        // Stack is not directly available, use column as placeholder
        iced::widget::column(children).into()
    }

    fn build_space(&self, _node: &WidgetNode) -> Element<'a, HandlerMessage, Theme, Renderer>
    where
        HandlerMessage: Clone + 'static,
    {
        iced::widget::text("").into()
    }

    fn build_rule(&self, _node: &WidgetNode) -> Element<'a, HandlerMessage, Theme, Renderer>
    where
        HandlerMessage: Clone + 'static,
    {
        // Create a horizontal rule using a container with a border
        iced::widget::container(iced::widget::text(""))
            .width(iced::Length::Fill)
            .height(iced::Length::Fixed(1.0))
            .style(|_theme: &iced::Theme| iced::widget::container::Style {
                background: Some(iced::Background::Color(iced::Color::from_rgb(
                    0.7, 0.7, 0.7,
                ))),
                ..Default::default()
            })
            .into()
    }

    fn build_svg(&self, node: &WidgetNode) -> Element<'a, HandlerMessage, Theme, Renderer>
    where
        HandlerMessage: Clone + 'static,
    {
        // Support both 'src' and 'path' attributes
        let src = node
            .attributes
            .get("src")
            .or_else(|| node.attributes.get("path"))
            .map(|attr| self.evaluate_attribute(attr))
            .unwrap_or_default();

        if src.is_empty() {
            if self.verbose {
                eprintln!("[GravityWidgetBuilder] SVG src is empty");
            }
            return iced::widget::text("[SVG: no src]").into();
        }

        let handle = iced::widget::svg::Handle::from_path(src);
        let mut svg = iced::widget::svg(handle);

        // Parse optional width
        if let Some(width_attr) = node.attributes.get("width") {
            if let Ok(width) = self.evaluate_attribute(width_attr).parse::<f32>() {
                svg = svg.width(width);
            }
        }

        // Parse optional height
        if let Some(height_attr) = node.attributes.get("height") {
            if let Ok(height) = self.evaluate_attribute(height_attr).parse::<f32>() {
                svg = svg.height(height);
            }
        }

        svg.into()
    }

    fn build_custom(&self, _node: &WidgetNode) -> Element<'a, HandlerMessage, Theme, Renderer>
    where
        HandlerMessage: Clone + 'static,
    {
        iced::widget::column(vec![]).into()
    }

    /// T054: Implement ProgressBar rendering with value clamping
    fn build_progress_bar(&self, node: &WidgetNode) -> Element<'a, HandlerMessage, Theme, Renderer>
    where
        HandlerMessage: Clone + 'static,
    {
        // Parse min, max, and value attributes
        let min = match node.attributes.get("min") {
            Some(AttributeValue::Static(s)) => s.parse::<f32>().ok().unwrap_or(0.0),
            _ => 0.0,
        };

        let max = match node.attributes.get("max") {
            Some(AttributeValue::Static(s)) => s.parse::<f32>().ok().unwrap_or(1.0),
            _ => 1.0,
        };

        let value_str = match node.attributes.get("value") {
            Some(attr) => self.evaluate_attribute(attr),
            None => "0".to_string(),
        };
        let value = value_str.parse::<f32>().unwrap_or(0.0);

        // Clamp value to [min, max] range
        let clamped_value = value.min(max).max(min);

        // Create progress bar
        let progress_bar = iced::widget::progress_bar(min..=max, clamped_value);

        progress_bar.into()
    }

    /// T055: Implement Tooltip rendering as wrapper widget
    fn build_tooltip(&self, node: &WidgetNode) -> Element<'a, HandlerMessage, Theme, Renderer>
    where
        HandlerMessage: Clone + 'static,
    {
        // Get message attribute
        let message = match node.attributes.get("message") {
            Some(attr) => self.evaluate_attribute(attr),
            None => "Tooltip".to_string(),
        };

        // Get position attribute (default: FollowCursor)
        let position_str = match node.attributes.get("position") {
            Some(AttributeValue::Static(s)) => s.as_str(),
            _ => "follow_cursor",
        };

        // Map position to Iced Position enum
        let position = match position_str {
            "top" => iced::widget::tooltip::Position::Top,
            "bottom" => iced::widget::tooltip::Position::Bottom,
            "left" => iced::widget::tooltip::Position::Left,
            "right" => iced::widget::tooltip::Position::Right,
            _ => iced::widget::tooltip::Position::FollowCursor,
        };

        // Tooltip must have exactly one child
        if let Some(child) = node.children.first() {
            let child_widget = self.build_widget(child);
            iced::widget::tooltip(child_widget, iced::widget::text(message), position).into()
        } else {
            // No child - return empty text
            iced::widget::text("").into()
        }
    }

    /// T076: Implement Canvas rendering with Program binding evaluation
    /// T077: Implement Canvas click event handling with coordinate passing
    fn build_canvas(&self, node: &WidgetNode) -> Element<'a, HandlerMessage, Theme, Renderer>
    where
        HandlerMessage: Clone + 'static,
    {
        // Parse width and height attributes (validated by parser, so they exist)
        let width = match node.attributes.get("width") {
            Some(AttributeValue::Static(s)) => s.parse::<f32>().ok().unwrap_or(400.0),
            _ => 400.0,
        };

        let height = match node.attributes.get("height") {
            Some(AttributeValue::Static(s)) => s.parse::<f32>().ok().unwrap_or(300.0),
            _ => 300.0,
        };

        if self.verbose {
            eprintln!(
                "[GravityWidgetBuilder] Building Canvas widget: {}x{}",
                width, height
            );
        }

        // Note: Canvas requires a custom Program implementation
        // For now, we create a placeholder container with a message
        // Real Canvas programs must be implemented in Rust code

        // Get program binding attribute for logging
        if let Some(AttributeValue::Binding(expr)) = node.attributes.get("program") {
            if self.verbose {
                eprintln!("[GravityWidgetBuilder] Canvas program binding: {:?}", expr);
            }
        }

        // Create a placeholder: container with text explaining Canvas limitation
        let placeholder = iced::widget::container(
            iced::widget::text("Canvas widget requires custom Program implementation in Rust")
                .size(14),
        )
        .width(iced::Length::Fixed(width))
        .height(iced::Length::Fixed(height))
        .center_x(iced::Length::Fill)
        .center_y(iced::Length::Fill)
        .style(|_theme: &iced::Theme| iced::widget::container::Style {
            background: Some(iced::Background::Color(iced::Color::from_rgb(
                0.95, 0.95, 0.95,
            ))),
            border: iced::Border {
                color: iced::Color::from_rgb(0.7, 0.7, 0.7),
                width: 1.0,
                radius: 4.0.into(),
            },
            ..iced::widget::container::Style::default()
        });

        // TODO: When canvas::Program can be accessed from model binding,
        // use: iced::widget::canvas(program)
        // For now, return placeholder
        placeholder.into()
    }

    /// Build a <for> loop widget
    ///
    /// Iterates over a collection and renders the child widgets for each item,
    /// with the loop variable available in the binding context.
    ///
    /// # Example XML
    ///
    /// ```xml
    /// <for each="item" in="{items}">
    ///     <text value="{item.text}" />
    /// </for>
    /// ```
    fn build_for(&self, node: &WidgetNode) -> Element<'a, HandlerMessage, Theme, Renderer>
    where
        HandlerMessage: Clone + 'static,
    {
        // Get variable name
        let var_name = match node.attributes.get("each") {
            Some(AttributeValue::Static(name)) => name.clone(),
            _ => {
                if self.verbose {
                    eprintln!("[GravityWidgetBuilder] For loop missing 'each' attribute");
                }
                return iced::widget::column(vec![]).into();
            }
        };

        // Evaluate collection binding
        let collection_values = match node.attributes.get("in") {
            Some(AttributeValue::Binding(expr)) => {
                // Try context first, then model
                let binding_result = if let Some(ctx_value) = self.resolve_from_context(expr) {
                    Ok(ctx_value)
                } else {
                    evaluate_binding_expr(expr, self.model)
                };

                match binding_result {
                    Ok(BindingValue::List(items)) => items,
                    Ok(other) => {
                        if self.verbose {
                            eprintln!(
                                "[GravityWidgetBuilder] For loop 'in' is not a list: {:?}",
                                other
                            );
                        }
                        return iced::widget::column(vec![]).into();
                    }
                    Err(e) => {
                        if self.verbose {
                            eprintln!("[GravityWidgetBuilder] For loop evaluation error: {}", e);
                        }
                        return iced::widget::column(vec![]).into();
                    }
                }
            }
            _ => {
                if self.verbose {
                    eprintln!("[GravityWidgetBuilder] For loop missing 'in' binding");
                }
                return iced::widget::column(vec![]).into();
            }
        };

        if self.verbose {
            eprintln!(
                "[GravityWidgetBuilder] For loop rendering {} items as '{}'",
                collection_values.len(),
                var_name
            );
        }

        // Render children for each item
        let mut rendered_children = Vec::new();

        for (index, item_value) in collection_values.iter().enumerate() {
            // Push context
            self.push_context(&var_name, item_value.clone());
            self.push_context("index", BindingValue::Integer(index as i64));

            // Render all template children
            for child in &node.children {
                rendered_children.push(self.build_widget(child));
            }

            // Pop context
            self.pop_context(); // index
            self.pop_context(); // item
        }

        // Return as column
        iced::widget::column(rendered_children).into()
    }

    /// Build a grid widget
    ///
    /// Creates a grid layout by grouping children into rows based on the `columns` attribute.
    ///
    /// # Example XML
    ///
    /// ```xml
    /// <grid columns="3" spacing="10">
    ///     <text value="Cell 1" />
    ///     <text value="Cell 2" />
    ///     <text value="Cell 3" />
    ///     <text value="Cell 4" />
    /// </grid>
    /// ```
    fn build_grid(&self, node: &WidgetNode) -> Element<'a, HandlerMessage, Theme, Renderer>
    where
        HandlerMessage: Clone + 'static,
    {
        // Parse columns attribute (validated by parser, so it exists)
        let columns = match node.attributes.get("columns") {
            Some(AttributeValue::Static(s)) => s.parse::<usize>().unwrap_or(1),
            _ => 1,
        };

        // Parse spacing attribute
        let spacing = match node.attributes.get("spacing") {
            Some(attr) => self.evaluate_attribute(attr).parse::<f32>().unwrap_or(10.0),
            None => 10.0,
        };

        if self.verbose {
            eprintln!(
                "[GravityWidgetBuilder] Building Grid: {} columns, spacing {}",
                columns, spacing
            );
        }

        // Group child nodes into rows and build widgets
        let mut rows = Vec::new();

        for chunk in node.children.chunks(columns) {
            let row_widgets: Vec<_> = chunk.iter().map(|child| self.build_widget(child)).collect();

            let row = iced::widget::row(row_widgets).spacing(spacing);
            rows.push(row.into());
        }

        iced::widget::column(rows).spacing(spacing).into()
    }
}

/// Parse a color string (#RRGGBB or #RGB format)
fn parse_color(color_str: &str) -> Option<gravity_core::ir::style::Color> {
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

    Some(gravity_core::ir::style::Color {
        r: r as f32 / 255.0,
        g: g as f32 / 255.0,
        b: b as f32 / 255.0,
        a: 1.0,
    })
}

/// Merge two StyleProperties, with the second one taking precedence
fn merge_styles(
    base: gravity_core::ir::style::StyleProperties,
    override_style: &gravity_core::ir::style::StyleProperties,
) -> gravity_core::ir::style::StyleProperties {
    use gravity_core::ir::style::StyleProperties;

    StyleProperties {
        background: override_style.background.clone().or(base.background),
        color: override_style.color.clone().or(base.color),
        border: override_style.border.clone().or(base.border),
        shadow: override_style.shadow.clone().or(base.shadow),
        opacity: override_style.opacity.or(base.opacity),
        transform: override_style.transform.clone().or(base.transform),
    }
}

/// Merge two LayoutConstraints, with the second one taking precedence
fn merge_layouts(
    base: gravity_core::ir::layout::LayoutConstraints,
    override_layout: &gravity_core::ir::layout::LayoutConstraints,
) -> gravity_core::ir::layout::LayoutConstraints {
    use gravity_core::ir::layout::LayoutConstraints;

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
