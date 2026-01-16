//! Checkbox widget builder

use crate::HandlerMessage;
use crate::builder::DampenWidgetBuilder;
use dampen_core::expr::evaluate_binding_expr_with_shared;
use dampen_core::ir::node::WidgetNode;
use iced::{Element, Renderer, Theme};

impl<'a> DampenWidgetBuilder<'a> {
    /// Build a checkbox widget from Dampen XML definition
    ///
    /// Supports the following attributes:
    /// - `label`: Text label displayed next to checkbox
    /// - `checked`: Boolean binding for checked state
    /// - `on_toggle`: Handler called on toggle with "true"/"false"
    ///
    /// Events: Toggle (sends HandlerMessage::Handler(name, Some("true"|"false")))
    pub(in crate::builder) fn build_checkbox(
        &self,
        node: &WidgetNode,
    ) -> Element<'a, HandlerMessage, Theme, Renderer> {
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
                "[DampenWidgetBuilder] Building checkbox: label='{}', checked={}",
                label, is_checked
            );
        }

        // Get handler from events (accept both on_toggle and on_change)
        let on_toggle_event = node.events.iter().find(|e| {
            e.event == dampen_core::EventKind::Toggle || e.event == dampen_core::EventKind::Change
        });

        if self.verbose {
            if let Some(event) = &on_toggle_event {
                eprintln!(
                    "[DampenWidgetBuilder] Checkbox has toggle/change event: handler={}, param={:?}",
                    event.handler, event.param
                );
            } else {
                eprintln!("[DampenWidgetBuilder] Checkbox has no toggle/change event");
            }
        }

        let mut checkbox = iced::widget::checkbox(is_checked);

        // Resolve and apply checkbox styles with state-aware styling
        // Use complete style resolution: theme → class → inline
        let resolved_base_style = self.resolve_complete_styles(node);

        // Get the StyleClass for state variant resolution
        let style_class = if !node.classes.is_empty() {
            self.style_classes
                .and_then(|classes| node.classes.first().and_then(|name| classes.get(name)))
        } else {
            None
        };

        if let Some(base_style_props) = resolved_base_style {
            // Clone for move into closure
            let base_style_props = base_style_props.clone();
            let style_class = style_class.cloned();

            checkbox = checkbox.style(move |_theme, status| {
                use crate::style_mapping::{
                    map_checkbox_status, merge_style_properties, resolve_state_style,
                };
                use iced::widget::checkbox;
                use iced::{Background, Border, Color};

                // Map Iced checkbox status to WidgetState
                let widget_state = map_checkbox_status(status);

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

                // Create checkbox style with defaults
                let mut style = checkbox::Style {
                    background: Background::Color(Color::WHITE),
                    icon_color: Color::BLACK,
                    border: Border::default(),
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

                // Apply icon color if specified
                if let Some(ref icon_color) = final_style_props.color {
                    style.icon_color = Color {
                        r: icon_color.r,
                        g: icon_color.g,
                        b: icon_color.b,
                        a: icon_color.a,
                    };
                }

                style
            });
        }

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
                                "[DampenWidgetBuilder] Checkbox param from context: {:?} -> {}",
                                param_expr,
                                value.to_display_string()
                            );
                        }
                        Some(value.to_display_string())
                    } else {
                        match evaluate_binding_expr_with_shared(
                            param_expr,
                            self.model,
                            self.shared_context,
                        ) {
                            Ok(value) => {
                                if self.verbose {
                                    eprintln!(
                                        "[DampenWidgetBuilder] Checkbox param from model: {:?} -> {}",
                                        param_expr,
                                        value.to_display_string()
                                    );
                                }
                                Some(value.to_display_string())
                            }
                            Err(e) => {
                                if self.verbose {
                                    eprintln!("[DampenWidgetBuilder] Checkbox param error: {}", e);
                                }
                                None
                            }
                        }
                    }
                } else {
                    if self.verbose {
                        eprintln!("[DampenWidgetBuilder] Checkbox has no param");
                    }
                    None
                };

                if self.verbose {
                    eprintln!(
                        "[DampenWidgetBuilder] Checkbox: Attaching on_toggle with handler '{}', param: {:?}",
                        handler_name, param_value
                    );
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
                    eprintln!(
                        "[DampenWidgetBuilder] Checkbox: No handler_registry, cannot attach on_toggle"
                    );
                }
            }
        }

        let text_widget = iced::widget::text(label);
        let row = iced::widget::row(vec![checkbox.into(), text_widget.into()]);
        row.into()
    }
}
