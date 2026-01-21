//! Button widget builder

use crate::HandlerMessage;
use crate::builder::DampenWidgetBuilder;
use crate::builder::helpers::{resolve_boolean_attribute, resolve_handler_param};
use dampen_core::ir::node::WidgetNode;
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

        #[cfg(debug_assertions)]
        eprintln!(
            "[DampenWidgetBuilder] Building button with label: '{}'",
            label
        );

        // Get handler from events
        let on_click_event = node
            .events
            .iter()
            .find(|e| e.event == dampen_core::EventKind::Click);

        #[cfg(debug_assertions)]
        if let Some(event) = &on_click_event {
            eprintln!(
                "[DampenWidgetBuilder] Button has click event: handler={}, param={:?}",
                event.handler, event.param
            );
        } else {
            eprintln!("[DampenWidgetBuilder] Button has no click event");
        }

        let mut btn = iced::widget::button(iced::widget::text(label.clone()));

        // Evaluate enabled attribute (default: true)
        let is_enabled = resolve_boolean_attribute(self, node, "enabled", true);

        #[cfg(debug_assertions)]
        eprintln!(
            "[DampenWidgetBuilder] Button '{}' enabled: {}",
            label, is_enabled
        );

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
                #[cfg(debug_assertions)]
                eprintln!(
                    "[DampenWidgetBuilder] Button '{}' width: '{}'",
                    label, width_value
                );
            }
        }

        // Apply theme-aware styling that resolves colors at render time
        // This enables theme switching to work visually
        if let Some(style_closure) = self.create_theme_aware_style_closure(node) {
            btn = btn.style(style_closure);
        }

        // Connect event if handler exists AND button is enabled (AFTER style is applied)
        if let Some(event_binding) = on_click_event {
            if self.handler_registry.is_some() && is_enabled {
                let handler_name = event_binding.handler.clone();

                // Evaluate parameter if present
                let param_value = if let Some(param_expr) = &event_binding.param {
                    match resolve_handler_param(self, param_expr) {
                        Ok(value) => {
                            #[cfg(debug_assertions)]
                            eprintln!(
                                "[DampenWidgetBuilder] Button param resolved: {:?} -> {}",
                                param_expr,
                                value.to_display_string()
                            );
                            Some(value.to_display_string())
                        }
                        Err(e) => {
                            #[cfg(debug_assertions)]
                            eprintln!("[DampenWidgetBuilder] Button param error: {}", e);
                            None
                        }
                    }
                } else {
                    #[cfg(debug_assertions)]
                    eprintln!("[DampenWidgetBuilder] Button has no param");
                    None
                };

                #[cfg(debug_assertions)]
                eprintln!(
                    "[DampenWidgetBuilder] Button: Attaching on_press with handler '{}', param: {:?}",
                    handler_name, param_value
                );

                // Clone param_value explicitly before creating HandlerMessage
                let param_cloned = param_value.clone();
                let handler_cloned = handler_name.clone();

                // Pass the HandlerMessage directly (on_press doesn't support closures)
                btn = btn.on_press(HandlerMessage::Handler(handler_cloned, param_cloned));
            } else if !is_enabled {
                #[cfg(debug_assertions)]
                eprintln!(
                    "[DampenWidgetBuilder] Button '{}' is disabled via enabled attribute",
                    label
                );
                // Don't call on_press - button will be disabled automatically by Iced
            } else {
                #[cfg(debug_assertions)]
                eprintln!(
                    "[DampenWidgetBuilder] Button: No handler_registry, cannot attach on_press"
                );
            }
        } else {
            #[cfg(debug_assertions)]
            eprintln!("[DampenWidgetBuilder] Button: No on_click event found");
        }

        btn.into()
    }
}
