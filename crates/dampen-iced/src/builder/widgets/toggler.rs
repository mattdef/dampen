//! Toggler widget builder

use crate::HandlerMessage;
use crate::builder::DampenWidgetBuilder;
use crate::builder::helpers::merge_styles;
use dampen_core::ir::node::WidgetNode;
use iced::{Element, Renderer, Theme};

impl<'a> DampenWidgetBuilder<'a> {
    /// Build a toggler widget from Dampen XML definition
    ///
    /// Supports the following attributes:
    /// - `label`: Text label displayed next to toggler
    /// - `active`: Boolean binding for active state
    /// - `on_toggle`: Handler called on toggle with "true"/"false"
    ///
    /// Events: Toggle (sends HandlerMessage::Handler(name, Some("true"|"false")))
    pub(in crate::builder) fn build_toggler(
        &self,
        node: &WidgetNode,
    ) -> Element<'a, HandlerMessage, Theme, Renderer> {
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
                "[DampenWidgetBuilder] Building toggler: label='{}', active={}",
                label, is_active
            );
        }

        // Get handler from events
        let on_toggle = node
            .events
            .iter()
            .find(|e| e.event == dampen_core::EventKind::Toggle)
            .map(|e| e.handler.clone());

        if self.verbose {
            if let Some(handler) = &on_toggle {
                eprintln!(
                    "[DampenWidgetBuilder] Toggler has toggle event: handler={}",
                    handler
                );
            } else {
                eprintln!("[DampenWidgetBuilder] Toggler has no toggle event");
            }
        }

        let mut toggler = iced::widget::toggler(is_active);

        // Resolve and apply toggler styles with state-aware styling
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

        if let Some(base_style_props) = resolved_base_style {
            // Clone for move into closure
            let base_style_props = base_style_props.clone();
            let style_class = style_class.cloned();

            toggler = toggler.style(move |_theme, status| {
                use crate::style_mapping::{
                    map_toggler_status, merge_style_properties, resolve_state_style,
                };
                use iced::widget::toggler;
                use iced::{Background, Color};

                // Map Iced toggler status to WidgetState
                let widget_state = map_toggler_status(status);

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

                // Create toggler style with defaults
                let mut style = toggler::Style {
                    background: Background::Color(Color::from_rgb(0.7, 0.7, 0.7)),
                    background_border_width: 1.0,
                    background_border_color: Color::TRANSPARENT,
                    foreground: Background::Color(Color::WHITE),
                    foreground_border_width: 0.0,
                    foreground_border_color: Color::TRANSPARENT,
                    border_radius: None,
                    padding_ratio: 0.2,
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

                // Apply foreground color (the toggle indicator)
                if let Some(ref fg_color) = final_style_props.color {
                    style.foreground = Background::Color(Color {
                        r: fg_color.r,
                        g: fg_color.g,
                        b: fg_color.b,
                        a: fg_color.a,
                    });
                }

                // Apply text color if specified
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
                    style.background_border_color = Color {
                        r: border.color.r,
                        g: border.color.g,
                        b: border.color.b,
                        a: border.color.a,
                    };
                    style.background_border_width = border.width;
                }

                style
            });
        }

        // Connect event if handler exists
        if let Some(handler_name) = on_toggle {
            if self.handler_registry.is_some() {
                if self.verbose {
                    eprintln!(
                        "[DampenWidgetBuilder] Toggler: Attaching on_toggle with handler '{}'",
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
                    eprintln!(
                        "[DampenWidgetBuilder] Toggler: No handler_registry, cannot attach on_toggle"
                    );
                }
            }
        }

        let text_widget = iced::widget::text(label);
        let row = iced::widget::row(vec![toggler.into(), text_widget.into()]);
        row.into()
    }
}
