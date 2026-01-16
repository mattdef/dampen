//! Button widget builder

use crate::HandlerMessage;
use crate::builder::DampenWidgetBuilder;
use crate::builder::helpers::merge_styles;
use dampen_core::expr::evaluate_binding_expr_with_shared;
use dampen_core::ir::node::{AttributeValue, WidgetNode};
use iced::{Element, Renderer, Theme};

impl<'a> DampenWidgetBuilder<'a> {
    pub(in crate::builder) fn build_button(
        &self,
        node: &WidgetNode,
    ) -> Element<'a, HandlerMessage, Theme, Renderer>
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
                "[DampenWidgetBuilder] Building button with label: '{}'",
                label
            );
        }

        // Get handler from events
        let on_click_event = node
            .events
            .iter()
            .find(|e| e.event == dampen_core::EventKind::Click);

        if self.verbose {
            if let Some(event) = &on_click_event {
                eprintln!(
                    "[DampenWidgetBuilder] Button has click event: handler={}, param={:?}",
                    event.handler, event.param
                );
            } else {
                eprintln!("[DampenWidgetBuilder] Button has no click event");
            }
        }

        let mut btn = iced::widget::button(iced::widget::text(label.clone()));

        // Evaluate enabled attribute (default: true)
        let is_enabled = match node.attributes.get("enabled") {
            None => true,
            Some(AttributeValue::Static(s)) => match s.to_lowercase().as_str() {
                "true" | "1" | "yes" | "on" => true,
                "false" | "0" | "no" | "off" => false,
                _ => true, // Default to enabled for unknown values
            },
            Some(AttributeValue::Binding(expr)) => {
                match evaluate_binding_expr_with_shared(expr, self.model, self.shared_context) {
                    Ok(value) => value.to_bool(),
                    Err(e) => {
                        if self.verbose {
                            eprintln!("[DampenWidgetBuilder] Button enabled binding error: {}", e);
                        }
                        true // Default to enabled on error
                    }
                }
            }
            Some(AttributeValue::Interpolated(_)) => {
                // Interpolated strings in boolean context - check if result is non-empty
                let enabled_attr = node.attributes.get("enabled");
                let result = if let Some(attr) = enabled_attr {
                    self.evaluate_attribute(attr)
                } else {
                    String::new()
                };
                !result.is_empty() && result != "false" && result != "0"
            }
        };

        if self.verbose {
            eprintln!(
                "[DampenWidgetBuilder] Button '{}' enabled: {}",
                label, is_enabled
            );
        }

        // Handle width attribute
        if let Some(width_attr) = node.attributes.get("width") {
            let width_value = self.evaluate_attribute(width_attr);
            if !width_value.is_empty() {
                match width_value.as_str() {
                    "fill" | "100%" => {
                        btn = btn.width(iced::Length::Fill);
                    }
                    _ => {
                        // Try to parse as numeric value (pixels)
                        if let Ok(pixels) = width_value.parse::<f32>() {
                            btn = btn.width(iced::Length::Fixed(pixels));
                        }
                    }
                }
                if self.verbose {
                    eprintln!(
                        "[DampenWidgetBuilder] Button '{}' width: '{}'",
                        label, width_value
                    );
                }
            }
        }

        // Resolve and apply button styles with state-aware styling
        // Get the StyleClass for state variant resolution
        let style_class = if !node.classes.is_empty() {
            self.style_classes.and_then(|classes| {
                // Get the first class for state variant resolution
                // In the future, we could merge state variants from multiple classes
                node.classes.first().and_then(|name| classes.get(name))
            })
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

        if let Some(base_style_props) = resolved_base_style {
            // Clone for move into closure
            let base_style_props = base_style_props.clone();
            let style_class = style_class.cloned();

            btn = btn.style(move |_theme, status| {
                use crate::style_mapping::{
                    map_button_status, merge_style_properties, resolve_state_style,
                };
                use iced::widget::button;
                use iced::{Background, Border, Color};

                // Map Iced button status to WidgetState
                let widget_state = map_button_status(status);

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

                let mut style = button::Style::default();

                // Apply background color
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

                // Apply text color
                if let Some(ref text_color) = final_style_props.color {
                    style.text_color = Color {
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

                // Apply shadow
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
            });
        }

        // Connect event if handler exists AND button is enabled (AFTER style is applied)
        if let Some(event_binding) = on_click_event {
            if self.handler_registry.is_some() && is_enabled {
                let handler_name = event_binding.handler.clone();

                // Evaluate parameter if present
                let param_value = if let Some(param_expr) = &event_binding.param {
                    // Try context first (for {item.id} in for loop)
                    if let Some(value) = self.resolve_from_context(param_expr) {
                        if self.verbose {
                            eprintln!(
                                "[DampenWidgetBuilder] Button param from context: {:?} -> {}",
                                param_expr,
                                value.to_display_string()
                            );
                        }
                        Some(value.to_display_string())
                    } else {
                        // Fallback to model evaluation
                        match evaluate_binding_expr_with_shared(
                            param_expr,
                            self.model,
                            self.shared_context,
                        ) {
                            Ok(value) => {
                                if self.verbose {
                                    eprintln!(
                                        "[DampenWidgetBuilder] Button param from model: {:?} -> {}",
                                        param_expr,
                                        value.to_display_string()
                                    );
                                }
                                Some(value.to_display_string())
                            }
                            Err(e) => {
                                if self.verbose {
                                    eprintln!("[DampenWidgetBuilder] Button param error: {}", e);
                                }
                                None
                            }
                        }
                    }
                } else {
                    if self.verbose {
                        eprintln!("[DampenWidgetBuilder] Button has no param");
                    }
                    None
                };

                if self.verbose {
                    eprintln!(
                        "[DampenWidgetBuilder] Button: Attaching on_press with handler '{}', param: {:?}",
                        handler_name, param_value
                    );
                }

                // Clone param_value explicitly before creating HandlerMessage
                let param_cloned = param_value.clone();
                let handler_cloned = handler_name.clone();

                // Pass the HandlerMessage directly (on_press doesn't support closures)
                btn = btn.on_press(HandlerMessage::Handler(handler_cloned, param_cloned));
            } else if !is_enabled {
                if self.verbose {
                    eprintln!(
                        "[DampenWidgetBuilder] Button '{}' is disabled via enabled attribute",
                        label
                    );
                }
                // Don't call on_press - button will be disabled automatically by Iced
            } else {
                if self.verbose {
                    eprintln!(
                        "[DampenWidgetBuilder] Button: No handler_registry, cannot attach on_press"
                    );
                }
            }
        } else {
            if self.verbose {
                eprintln!("[DampenWidgetBuilder] Button: No on_click event found");
            }
        }

        btn.into()
    }
}
