//! Gravity Widget Builder - Automatic interpretation of Gravity markup
//!
//! This module provides the GravityWidgetBuilder which automatically converts
//! parsed Gravity UI definitions into Iced widgets with full support for
//! bindings, events, styles, and layouts.

use crate::convert::{map_layout_constraints, map_style_properties};
use crate::state::WidgetStateManager;
use crate::HandlerMessage;
use gravity_core::binding::UiBindable;
use gravity_core::expr::evaluate_binding_expr;
use gravity_core::handler::HandlerRegistry;
use gravity_core::ir::node::{AttributeValue, InterpolatedPart, WidgetNode};
use gravity_core::ir::theme::StyleClass;
use gravity_core::ir::WidgetKind;
use iced::{Element, Renderer, Theme};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Builder for creating Iced widgets from Gravity markup
pub struct GravityWidgetBuilder<'a, Message> {
    node: &'a WidgetNode,
    model: &'a dyn UiBindable,
    handler_registry: Option<&'a HandlerRegistry>,
    style_classes: Option<&'a HashMap<String, StyleClass>>,
    verbose: bool,
    message_factory: Box<dyn Fn(&str) -> Message + 'a>,
    state_manager: Arc<Mutex<WidgetStateManager>>,
}

impl<'a> GravityWidgetBuilder<'a, HandlerMessage> {
    /// Create a new widget builder using HandlerMessage
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
            message_factory: Box::new(|name| HandlerMessage::Handler(name.to_string(), None)),
            state_manager: Arc::new(Mutex::new(WidgetStateManager::new())),
        }
    }

    /// Create a new widget builder from a complete GravityDocument
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
            message_factory: Box::new(|name| HandlerMessage::Handler(name.to_string(), None)),
            state_manager: Arc::new(Mutex::new(WidgetStateManager::new())),
        }
    }
}

impl<'a, Message> GravityWidgetBuilder<'a, Message> {
    /// Create a new widget builder with custom message factory
    pub fn new_with_factory<F>(
        node: &'a WidgetNode,
        model: &'a dyn UiBindable,
        handler_registry: Option<&'a HandlerRegistry>,
        message_factory: F,
    ) -> Self
    where
        F: Fn(&str) -> Message + 'a,
    {
        Self {
            node,
            model,
            handler_registry,
            style_classes: None,
            verbose: false,
            message_factory: Box::new(message_factory),
            state_manager: Arc::new(Mutex::new(WidgetStateManager::new())),
        }
    }

    /// Add style classes to the builder
    pub fn with_style_classes(mut self, style_classes: &'a HashMap<String, StyleClass>) -> Self {
        self.style_classes = Some(style_classes);
        self
    }

    /// Get access to the state manager (for event handlers)
    pub fn state_manager(&self) -> Arc<Mutex<WidgetStateManager>> {
        self.state_manager.clone()
    }

    /// Enable or disable verbose logging
    pub fn with_verbose(mut self, verbose: bool) -> Self {
        self.verbose = verbose;
        self
    }

    /// Build the widget tree
    pub fn build(self) -> Element<'a, Message, Theme, Renderer>
    where
        Message: Clone + 'static,
    {
        self.build_widget(self.node)
    }

    /// Recursively build a widget
    fn build_widget(&self, node: &WidgetNode) -> Element<'a, Message, Theme, Renderer>
    where
        Message: Clone + 'static,
    {
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
        }
    }

    /// Evaluate an attribute value (handles static, binding, and interpolated)
    fn evaluate_attribute(&self, attr: &AttributeValue) -> String {
        match attr {
            AttributeValue::Static(value) => value.clone(),
            AttributeValue::Binding(expr) => match evaluate_binding_expr(expr, self.model) {
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
            },
            AttributeValue::Interpolated(parts) => {
                let mut result = String::new();
                for part in parts {
                    match part {
                        InterpolatedPart::Literal(lit) => result.push_str(lit),
                        InterpolatedPart::Binding(expr) => {
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
                result
            }
        }
    }

    /// Get optional message from handler name
    fn get_handler_message(&self, handler_name: &str) -> Option<Message>
    where
        Message: Clone + 'static,
    {
        if let Some(registry) = self.handler_registry {
            if registry.contains(handler_name) {
                if self.verbose {
                    eprintln!("[GravityWidgetBuilder] Handler '{}' found", handler_name);
                }
                return Some((self.message_factory)(handler_name));
            }
        }

        if self.verbose {
            eprintln!(
                "[GravityWidgetBuilder] Handler '{}' not found",
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

    /// Apply style and layout to a widget
    fn apply_style_layout<'b, W>(
        &self,
        widget: W,
        node: &WidgetNode,
    ) -> Element<'a, Message, Theme, Renderer>
    where
        W: Into<Element<'a, Message, Theme, Renderer>>,
        Message: Clone + 'static,
    {
        use iced::widget::container;

        let element: Element<'a, Message, Theme, Renderer> = widget.into();

        // Resolve styles: class styles first, then node styles override
        let resolved_style = match (self.resolve_class_styles(node), &node.style) {
            (Some(class_style), Some(node_style)) => Some(merge_styles(class_style, node_style)),
            (Some(class_style), None) => Some(class_style),
            (None, Some(node_style)) => Some(node_style.clone()),
            (None, None) => None,
        };

        let has_layout = node.layout.is_some();
        let has_style = resolved_style.is_some();

        if !has_layout && !has_style {
            return element;
        }

        let mut container = container(element);

        // Apply layout
        if let Some(layout) = &node.layout {
            let iced_layout = map_layout_constraints(layout);
            container = container
                .width(iced_layout.width)
                .height(iced_layout.height)
                .padding(iced_layout.padding);

            if let Some(align) = iced_layout.align_items {
                container = container.align_y(align);
            }
        }

        // Apply resolved style
        if let Some(style) = resolved_style {
            let iced_style = map_style_properties(&style);
            container = container.style(move |_theme| iced_style);
        }

        container.into()
    }

    // Widget builders - will be fully implemented in Phase 3

    fn build_text(&self, node: &WidgetNode) -> Element<'a, Message, Theme, Renderer>
    where
        Message: Clone + 'static,
    {
        let value = node
            .attributes
            .get("value")
            .map(|attr| self.evaluate_attribute(attr))
            .unwrap_or_default();

        iced::widget::text(value).into()
    }

    fn build_button(&self, node: &WidgetNode) -> Element<'a, Message, Theme, Renderer>
    where
        Message: Clone + 'static,
    {
        let label = node
            .attributes
            .get("label")
            .map(|attr| self.evaluate_attribute(attr))
            .unwrap_or_default();

        // Get handler from events
        let on_click = node
            .events
            .iter()
            .find(|e| e.event == gravity_core::EventKind::Click)
            .map(|e| e.handler.clone());

        let mut btn = iced::widget::button(iced::widget::text(label));

        // Connect event if handler exists
        if let Some(handler_name) = on_click {
            if let Some(message) = self.get_handler_message(&handler_name) {
                btn = btn.on_press(message);
            }
        }

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

                style
            });
        }

        btn.into()
    }

    fn build_column(&self, node: &WidgetNode) -> Element<'a, Message, Theme, Renderer>
    where
        Message: Clone + 'static,
    {
        let children: Vec<_> = node
            .children
            .iter()
            .map(|child| self.build_widget(child))
            .collect();

        let column = iced::widget::column(children);
        self.apply_style_layout(column, node)
    }

    fn build_row(&self, node: &WidgetNode) -> Element<'a, Message, Theme, Renderer>
    where
        Message: Clone + 'static,
    {
        let children: Vec<_> = node
            .children
            .iter()
            .map(|child| self.build_widget(child))
            .collect();

        let row = iced::widget::row(children);
        self.apply_style_layout(row, node)
    }

    fn build_container(&self, node: &WidgetNode) -> Element<'a, Message, Theme, Renderer>
    where
        Message: Clone + 'static,
    {
        if let Some(first_child) = node.children.first() {
            let child = self.build_widget(first_child);
            self.apply_style_layout(child, node)
        } else {
            self.apply_style_layout(iced::widget::text(""), node)
        }
    }

    fn build_text_input(&self, _node: &WidgetNode) -> Element<'a, Message, Theme, Renderer>
    where
        Message: Clone + 'static,
    {
        iced::widget::text("[TextInput - T034]").into()
    }

    fn build_checkbox(&self, _node: &WidgetNode) -> Element<'a, Message, Theme, Renderer>
    where
        Message: Clone + 'static,
    {
        iced::widget::text("[Checkbox - T035]").into()
    }

    fn build_slider(&self, _node: &WidgetNode) -> Element<'a, Message, Theme, Renderer>
    where
        Message: Clone + 'static,
    {
        iced::widget::text("[Slider - T036]").into()
    }

    fn build_pick_list(&self, _node: &WidgetNode) -> Element<'a, Message, Theme, Renderer>
    where
        Message: Clone + 'static,
    {
        iced::widget::text("[PickList - T037]").into()
    }

    fn build_toggler(&self, _node: &WidgetNode) -> Element<'a, Message, Theme, Renderer>
    where
        Message: Clone + 'static,
    {
        iced::widget::text("[Toggler - T038]").into()
    }

    fn build_image(&self, _node: &WidgetNode) -> Element<'a, Message, Theme, Renderer>
    where
        Message: Clone + 'static,
    {
        iced::widget::text("[Image - T039]").into()
    }

    fn build_scrollable(&self, node: &WidgetNode) -> Element<'a, Message, Theme, Renderer>
    where
        Message: Clone + 'static,
    {
        if let Some(first_child) = node.children.first() {
            let child = self.build_widget(first_child);
            iced::widget::scrollable(child).into()
        } else {
            iced::widget::scrollable(iced::widget::text("")).into()
        }
    }

    fn build_stack(&self, node: &WidgetNode) -> Element<'a, Message, Theme, Renderer>
    where
        Message: Clone + 'static,
    {
        let children: Vec<_> = node
            .children
            .iter()
            .map(|child| self.build_widget(child))
            .collect();

        // Stack is not directly available, use column as placeholder
        iced::widget::column(children).into()
    }

    fn build_space(&self, _node: &WidgetNode) -> Element<'a, Message, Theme, Renderer>
    where
        Message: Clone + 'static,
    {
        iced::widget::text("").into()
    }

    fn build_rule(&self, _node: &WidgetNode) -> Element<'a, Message, Theme, Renderer>
    where
        Message: Clone + 'static,
    {
        iced::widget::text("─").into()
    }

    fn build_svg(&self, _node: &WidgetNode) -> Element<'a, Message, Theme, Renderer>
    where
        Message: Clone + 'static,
    {
        iced::widget::text("[SVG]").into()
    }

    fn build_custom(&self, _node: &WidgetNode) -> Element<'a, Message, Theme, Renderer>
    where
        Message: Clone + 'static,
    {
        iced::widget::column(vec![]).into()
    }
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
