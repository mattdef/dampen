//! Gravity Widget Builder - Automatic interpretation of Gravity markup
//!
//! This module provides the GravityWidgetBuilder which automatically converts
//! parsed Gravity UI definitions into Iced widgets with full support for
//! bindings, events, styles, and layouts.

use gravity_core::ir::node::{WidgetNode, AttributeValue, InterpolatedPart};
use gravity_core::ir::WidgetKind;
use gravity_core::binding::UiBindable;
use gravity_core::handler::HandlerRegistry;
use gravity_core::expr::evaluate_binding_expr;
use iced::{Element, Renderer, Theme};
use crate::convert::{map_layout_constraints, map_style_properties};

/// Builder for creating Iced widgets from Gravity markup
pub struct GravityWidgetBuilder<'a, Message> {
    node: &'a WidgetNode,
    model: &'a dyn UiBindable,
    handler_registry: Option<&'a HandlerRegistry>,
    verbose: bool,
    _phantom: std::marker::PhantomData<Message>,
}

impl<'a, Message> GravityWidgetBuilder<'a, Message> {
    /// Create a new widget builder
    pub fn new(
        node: &'a WidgetNode,
        model: &'a dyn UiBindable,
        handler_registry: Option<&'a HandlerRegistry>,
    ) -> Self {
        Self {
            node,
            model,
            handler_registry,
            verbose: false,
            _phantom: std::marker::PhantomData,
        }
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
            AttributeValue::Binding(expr) => {
                match evaluate_binding_expr(expr, self.model) {
                    Ok(value) => {
                        if self.verbose {
                            eprintln!("[GravityWidgetBuilder] Binding evaluated to: {}", value.to_display_string());
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
                                        eprintln!("[GravityWidgetBuilder] Interpolated binding error: {}", e);
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
                // For MVP, we'll need a way to convert handler name to Message
                // This will be handled in Phase 3 with proper message generation
                return None;
            }
        }
        
        if self.verbose {
            eprintln!("[GravityWidgetBuilder] Handler '{}' not found", handler_name);
        }
        None
    }

    /// Apply style and layout to a widget
    fn apply_style_layout<'b, W>(&self, widget: W, node: &WidgetNode) -> Element<'a, Message, Theme, Renderer>
    where
        W: Into<Element<'a, Message, Theme, Renderer>>,
        Message: Clone + 'static,
    {
        use iced::widget::container;

        let element: Element<'a, Message, Theme, Renderer> = widget.into();
        
        let has_layout = node.layout.is_some();
        let has_style = node.style.is_some();

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

        // Apply style
        if let Some(style) = &node.style {
            let iced_style = map_style_properties(style);
            container = container.style(move |_theme| iced_style);
        }

        container.into()
    }

    // Widget builders - will be fully implemented in Phase 3
    
    fn build_text(&self, node: &WidgetNode) -> Element<'a, Message, Theme, Renderer>
    where
        Message: Clone + 'static,
    {
        let value = node.attributes.get("value")
            .map(|attr| self.evaluate_attribute(attr))
            .unwrap_or_default();
        
        iced::widget::text(value).into()
    }

    fn build_button(&self, node: &WidgetNode) -> Element<'a, Message, Theme, Renderer>
    where
        Message: Clone + 'static,
    {
        let label = node.attributes.get("label")
            .map(|attr| self.evaluate_attribute(attr))
            .unwrap_or_default();
        
        // In Phase 3, we'll properly connect events
        iced::widget::button(iced::widget::text(label)).into()
    }

    fn build_column(&self, node: &WidgetNode) -> Element<'a, Message, Theme, Renderer>
    where
        Message: Clone + 'static,
    {
        let children: Vec<_> = node.children
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
        let children: Vec<_> = node.children
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
        let children: Vec<_> = node.children
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
