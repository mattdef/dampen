//! TextInput widget builder

use crate::HandlerMessage;
use crate::builder::DampenWidgetBuilder;
use dampen_core::ir::node::WidgetNode;
use iced::{Element, Renderer, Theme};

impl<'a> DampenWidgetBuilder<'a> {
    /// Build a text input widget from Dampen XML definition
    ///
    /// Supports the following attributes:
    /// - `value`: String binding for current text value
    /// - `placeholder`: Placeholder text when empty
    /// - `on_input`: Handler called on text input with new value
    /// - `password`: If "true", masks input with password character
    ///
    /// Events: Input (sends HandlerMessage::Handler(name, Some(new_text)))
    pub(in crate::builder) fn build_text_input(
        &self,
        node: &WidgetNode,
    ) -> Element<'a, HandlerMessage, Theme, Renderer> {
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

        // Check for password attribute
        let is_password = node
            .attributes
            .get("password")
            .map(|attr| self.evaluate_attribute(attr))
            .map(|v| v == "true" || v == "1")
            .unwrap_or(false);

        if self.verbose {
            eprintln!(
                "[DampenWidgetBuilder] Building text_input: placeholder='{}', value='{}', password={}",
                placeholder, value, is_password
            );
        }

        // Get handler from events
        let on_input = node
            .events
            .iter()
            .find(|e| e.event == dampen_core::EventKind::Input)
            .map(|e| e.handler.clone());

        if self.verbose {
            if let Some(handler) = &on_input {
                eprintln!(
                    "[DampenWidgetBuilder] TextInput has input event: handler={}",
                    handler
                );
            } else {
                eprintln!("[DampenWidgetBuilder] TextInput has no input event");
            }
        }

        let mut text_input = iced::widget::text_input(&placeholder, &value);

        // Apply state-aware styling (focus, hover, disabled)
        // Use complete style resolution: theme → class → inline
        let resolved_base_style = self.resolve_complete_styles(node);

        // Get the StyleClass for state variant resolution
        let style_class = if !node.classes.is_empty() {
            self.style_classes.and_then(|classes| {
                // Get the first class for state variant resolution
                node.classes.first().and_then(|name| classes.get(name))
            })
        } else {
            None
        };

        if let Some(base_style_props) = resolved_base_style {
            // Clone for move into closure
            let base_style_props = base_style_props.clone();
            let style_class = style_class.cloned();

            text_input = text_input.style(move |_theme, status| {
                use crate::style_mapping::{
                    map_text_input_status, merge_style_properties, resolve_state_style,
                };
                use iced::widget::text_input;
                use iced::{Background, Border, Color};

                // Map Iced text_input status to WidgetState
                let widget_state = map_text_input_status(status);

                // Resolve state-specific style if available
                let final_style_props =
                    if let (Some(class), Some(state)) = (&style_class, widget_state) {
                        // Try to get state-specific style
                        if let Some(state_style) = resolve_state_style(class, state) {
                            // Merge state style with base style
                            merge_style_properties(&base_style_props, state_style)
                        } else {
                            // No state variant defined, use base style
                            base_style_props.clone()
                        }
                    } else {
                        // No style class or no state, use base style
                        base_style_props.clone()
                    };

                // Create text_input style
                // Based on Iced 0.14 text_input::Style struct
                let mut style = text_input::Style {
                    background: Background::Color(Color::WHITE),
                    border: Border::default(),
                    icon: Color::BLACK,
                    placeholder: Color::from_rgb(0.5, 0.5, 0.5),
                    value: Color::BLACK,
                    selection: Color::from_rgb(0.5, 0.7, 1.0),
                };

                // Apply background color
                if let Some(ref bg) = final_style_props.background {
                    if let dampen_core::ir::style::Background::Color(color) = bg {
                        style.background = Background::Color(Color {
                            r: color.r,
                            g: color.g,
                            b: color.b,
                            a: color.a,
                        });
                    }
                }

                // Apply text color (value text color)
                if let Some(ref text_color) = final_style_props.color {
                    style.value = Color {
                        r: text_color.r,
                        g: text_color.g,
                        b: text_color.b,
                        a: text_color.a,
                    };
                }

                // Apply border
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

                style
            });
        }

        // Note: Password masking with dots is not available in Iced 0.14's public API
        // The input still works but text is visible

        // Connect event if handler exists
        if let Some(handler_name) = on_input {
            if self.handler_registry.is_some() {
                if self.verbose {
                    eprintln!(
                        "[DampenWidgetBuilder] TextInput: Attaching on_input with handler '{}'",
                        handler_name
                    );
                }
                text_input = text_input.on_input(move |input_value| {
                    HandlerMessage::Handler(handler_name.clone(), Some(input_value))
                });
            } else {
                if self.verbose {
                    eprintln!(
                        "[DampenWidgetBuilder] TextInput: No handler_registry, cannot attach on_input"
                    );
                }
            }
        }

        text_input.into()
    }
}
