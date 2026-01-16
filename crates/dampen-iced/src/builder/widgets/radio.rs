//! Radio widget builder

use crate::HandlerMessage;
use crate::builder::DampenWidgetBuilder;
use crate::builder::helpers::merge_styles;
use dampen_core::expr::evaluate_binding_expr_with_shared;
use dampen_core::ir::node::{AttributeValue, WidgetNode};
use iced::{Element, Renderer, Theme};

impl<'a> DampenWidgetBuilder<'a> {
    /// Build a radio button widget (placeholder implementation)
    pub(in crate::builder) fn build_radio(
        &self,
        node: &WidgetNode,
    ) -> Element<'a, HandlerMessage, Theme, Renderer> {
        let label = node
            .attributes
            .get("label")
            .map(|attr| self.evaluate_attribute(attr))
            .unwrap_or_else(|| String::from(""));

        let value = node
            .attributes
            .get("value")
            .map(|attr| self.evaluate_attribute(attr))
            .unwrap_or_else(|| String::from(""));

        // Get the currently selected value (if any)
        let selected = node
            .attributes
            .get("selected")
            .map(|attr| self.evaluate_attribute(attr));

        if self.verbose {
            eprintln!(
                "[DampenWidgetBuilder] Building radio: label='{}', value='{}', selected={:?}",
                label, value, selected
            );
        }

        // Get handler from events
        let on_select_event = node
            .events
            .iter()
            .find(|e| e.event == dampen_core::EventKind::Select);

        if self.verbose {
            if let Some(event) = &on_select_event {
                eprintln!(
                    "[DampenWidgetBuilder] Radio has select event: handler={}, param={:?}",
                    event.handler, event.param
                );
            } else {
                eprintln!("[DampenWidgetBuilder] Radio has no select event");
            }
        }

        // Determine if this radio is currently selected
        let is_selected = selected.as_ref().map(|s| s == &value).unwrap_or(false);

        // Evaluate disabled attribute (default: false)
        let is_disabled = match node.attributes.get("disabled") {
            None => false,
            Some(AttributeValue::Static(s)) => match s.to_lowercase().as_str() {
                "true" | "1" | "yes" | "on" => true,
                "false" | "0" | "no" | "off" => false,
                _ => false, // Default to enabled for unknown values
            },
            Some(AttributeValue::Binding(expr)) => {
                match evaluate_binding_expr_with_shared(expr, self.model, self.shared_context) {
                    Ok(value) => value.to_bool(),
                    Err(e) => {
                        if self.verbose {
                            eprintln!("[DampenWidgetBuilder] Radio disabled binding error: {}", e);
                        }
                        false // Default to enabled on error
                    }
                }
            }
            Some(AttributeValue::Interpolated(_)) => {
                // Interpolated strings in boolean context - check if result is "true"
                let disabled_attr = node.attributes.get("disabled");
                let result = if let Some(attr) = disabled_attr {
                    self.evaluate_attribute(attr)
                } else {
                    String::new()
                };
                result == "true" || result == "1"
            }
        };

        if self.verbose {
            eprintln!(
                "[DampenWidgetBuilder] Radio '{}' disabled: {}",
                label, is_disabled
            );
        }

        // Create the radio widget using Iced's radio API
        // Note: Iced radio requires Copy types, so we use a unique ID as the value
        // and map it back to the string value in the message handler

        // For Iced radio, we need to use a copyable type. We'll use a hash of the value
        // to create a unique u64 identifier for each radio option
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut hasher = DefaultHasher::new();
        value.hash(&mut hasher);
        let value_id = hasher.finish();

        // Track the radio ID for debugging (unused but helpful for future enhancements)
        let _radio_id = format!("{}_{}", node.id.as_deref().unwrap_or("radio"), value);

        // Create the currently selected value_id if this is the selected option
        let selected_id = if is_selected { Some(value_id) } else { None };

        // Create the radio widget
        let radio_widget = if let Some(event_binding) = on_select_event {
            if self.handler_registry.is_some() && !is_disabled {
                let handler_name = event_binding.handler.clone();
                let value_clone = value.clone();

                if self.verbose {
                    eprintln!(
                        "[DampenWidgetBuilder] Radio: Attaching on_select with handler '{}', value='{}' (id={})",
                        handler_name, value_clone, value_id
                    );
                }

                // Create radio with handler - sends the string value when selected
                iced::widget::radio(label, value_id, selected_id, move |_selected_id| {
                    HandlerMessage::Handler(handler_name.clone(), Some(value_clone.clone()))
                })
            } else {
                if self.verbose {
                    if is_disabled {
                        eprintln!(
                            "[DampenWidgetBuilder] Radio: Disabled, creating non-interactive radio"
                        );
                    } else {
                        eprintln!(
                            "[DampenWidgetBuilder] Radio: No handler_registry, creating read-only radio"
                        );
                    }
                }
                // Disabled or no handler registry - create read-only radio
                iced::widget::radio(label, value_id, selected_id, |_| {
                    HandlerMessage::Handler(String::new(), None)
                })
            }
        } else {
            if self.verbose {
                eprintln!(
                    "[DampenWidgetBuilder] Radio: No on_select event, creating read-only radio"
                );
            }
            // No event handler - create read-only radio
            iced::widget::radio(label, value_id, selected_id, |_| {
                HandlerMessage::Handler(String::new(), None)
            })
        };

        // Resolve and apply radio styles with state-aware styling
        // Get the StyleClass for state variant resolution
        let style_class = if !node.classes.is_empty() {
            self.style_classes
                .and_then(|classes| node.classes.first().and_then(|name| classes.get(name)))
        } else {
            None
        };

        // Resolve base styles (class + inline)
        let resolved_base_style = match (self.resolve_class_styles(node), &node.style) {
            (Some(class_style), Some(node_style)) => Some(merge_styles(class_style, node_style)),
            (Some(class_style), None) => Some(class_style),
            (None, Some(node_style)) => Some(node_style.clone()),
            (None, None) => None,
        };

        let radio_widget = if let Some(base_style_props) = resolved_base_style {
            // Clone for move into closure
            let base_style_props = base_style_props.clone();
            let style_class = style_class.cloned();

            radio_widget.style(move |_theme, status| {
                use crate::style_mapping::{
                    map_radio_status, merge_style_properties, resolve_state_style,
                };
                use dampen_core::ir::WidgetState;
                use iced::widget::radio;
                use iced::{Background, Color};

                // Map Iced radio status to WidgetState
                // Note: Radio doesn't have Status::Disabled in Iced 0.14,
                // so we need to manually check the disabled attribute
                let widget_state = if is_disabled {
                    Some(WidgetState::Disabled)
                } else {
                    map_radio_status(status)
                };

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

                // Create radio style with defaults
                let mut style = radio::Style {
                    background: Background::Color(Color::WHITE),
                    dot_color: Color::BLACK,
                    border_width: 1.0,
                    border_color: Color::from_rgb(0.5, 0.5, 0.5),
                    text_color: None,
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

                // Apply text color
                if let Some(ref text_color) = final_style_props.color {
                    style.text_color = Some(Color {
                        r: text_color.r,
                        g: text_color.g,
                        b: text_color.b,
                        a: text_color.a,
                    });
                }

                // Apply border
                if let Some(ref border) = final_style_props.border {
                    style.border_color = Color {
                        r: border.color.r,
                        g: border.color.g,
                        b: border.color.b,
                        a: border.color.a,
                    };
                    style.border_width = border.width;
                }

                // Apply dot color if specified
                if let Some(ref dot_color) = final_style_props.color {
                    style.dot_color = Color {
                        r: dot_color.r,
                        g: dot_color.g,
                        b: dot_color.b,
                        a: dot_color.a,
                    };
                }

                style
            })
        } else {
            radio_widget
        };

        radio_widget.into()
    }
}
