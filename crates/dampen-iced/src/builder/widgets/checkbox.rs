//! Checkbox widget builder

use crate::HandlerMessage;
use crate::builder::DampenWidgetBuilder;
use crate::builder::helpers::{resolve_boolean_attribute, resolve_handler_param};
use dampen_core::ir::node::WidgetNode;
use dampen_core::ir::style::StyleProperties;
use iced::{Element, Renderer, Theme};

/// Convert Dampen StyleProperties to Iced checkbox Style
fn apply_checkbox_style(props: &StyleProperties) -> iced::widget::checkbox::Style {
    use iced::widget::checkbox;
    use iced::{Background, Border, Color};

    let mut style = checkbox::Style {
        background: Background::Color(Color::WHITE),
        icon_color: Color::BLACK,
        border: Border::default(),
        text_color: None,
    };

    if let Some(ref bg) = props.background
        && let dampen_core::ir::style::Background::Color(color) = bg
    {
        style.background = Background::Color(Color {
            r: color.r,
            g: color.g,
            b: color.b,
            a: color.a,
        });
    }

    if let Some(ref text_color) = props.color {
        style.text_color = Some(Color {
            r: text_color.r,
            g: text_color.g,
            b: text_color.b,
            a: text_color.a,
        });
    }

    if let Some(ref border) = props.border {
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

    if let Some(ref icon_color) = props.color {
        style.icon_color = Color {
            r: icon_color.r,
            g: icon_color.g,
            b: icon_color.b,
            a: icon_color.a,
        };
    }

    style
}

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

        let is_checked = resolve_boolean_attribute(self, node, "checked", false);

        // Parse size attribute
        let size = node
            .attributes
            .get("size")
            .map(|attr| self.evaluate_attribute(attr))
            .and_then(|s| s.parse::<f32>().ok());

        #[cfg(debug_assertions)]
        eprintln!(
            "[DampenWidgetBuilder] Building checkbox: label='{}', checked={}, size={:?}",
            label, is_checked, size
        );

        // Get handler from events (accept both on_toggle and on_change)
        let on_toggle_event = node.events.iter().find(|e| {
            e.event == dampen_core::EventKind::Toggle || e.event == dampen_core::EventKind::Change
        });

        #[cfg(debug_assertions)]
        if let Some(event) = &on_toggle_event {
            eprintln!(
                "[DampenWidgetBuilder] Checkbox has toggle/change event: handler={}, param={:?}",
                event.handler, event.param
            );
        } else {
            eprintln!("[DampenWidgetBuilder] Checkbox has no toggle/change event");
        }

        let mut checkbox = iced::widget::checkbox(is_checked);

        if let Some(s) = size {
            checkbox = checkbox.size(s);
        }

        // Resolve and apply checkbox styles with state-aware styling
        // Use complete style resolution: theme → class → inline
        let resolved_base_style = self.resolve_complete_styles(node);

        // Get the StyleClass for state variant resolution, wrapped in Rc for efficient cloning
        let classes = self.resolve_active_classes(node);
        let style_class = if !classes.is_empty() {
            self.style_classes
                .and_then(|cls| classes.first().and_then(|name| cls.get(name)))
                .cloned()
                .map(std::rc::Rc::new)
        } else {
            None
        };

        // Apply state-aware styling using generic helper
        if let Some(base_style_props) = resolved_base_style {
            use crate::builder::helpers::create_state_aware_style_fn;
            use crate::style_mapping::map_checkbox_status;

            let base_style_props = base_style_props.clone();

            if let Some(style_fn) = create_state_aware_style_fn(
                self,
                node,
                dampen_core::ir::WidgetKind::Checkbox,
                style_class,
                base_style_props,
                map_checkbox_status,
                apply_checkbox_style,
            ) {
                checkbox = checkbox.style(style_fn);
            }
        }

        // Connect event if handler exists
        if let Some(event_binding) = on_toggle_event {
            if self.handler_registry.is_some() {
                let handler_name = event_binding.handler.clone();

                // Evaluate parameter if present, otherwise use toggle state
                let param_value = if let Some(param_expr) = &event_binding.param {
                    match resolve_handler_param(self, param_expr) {
                        Ok(value) => {
                            #[cfg(debug_assertions)]
                            eprintln!(
                                "[DampenWidgetBuilder] Checkbox param resolved: {:?} -> {}",
                                param_expr,
                                value.to_display_string()
                            );
                            Some(value.to_display_string())
                        }
                        Err(e) => {
                            #[cfg(debug_assertions)]
                            eprintln!("[DampenWidgetBuilder] Checkbox param error: {}", e);
                            None
                        }
                    }
                } else {
                    #[cfg(debug_assertions)]
                    eprintln!("[DampenWidgetBuilder] Checkbox has no param");
                    None
                };

                #[cfg(debug_assertions)]
                eprintln!(
                    "[DampenWidgetBuilder] Checkbox: Attaching on_toggle with handler '{}', param: {:?}",
                    handler_name, param_value
                );

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
                #[cfg(debug_assertions)]
                eprintln!(
                    "[DampenWidgetBuilder] Checkbox: No handler_registry, cannot attach on_toggle"
                );
            }
        }

        let text_widget = iced::widget::text(label);
        let row = iced::widget::row(vec![checkbox.into(), text_widget.into()]);
        row.into()
    }
}
