//! Radio widget builder

use crate::HandlerMessage;
use crate::builder::DampenWidgetBuilder;
use crate::builder::helpers::resolve_boolean_attribute;
use dampen_core::ir::node::WidgetNode;
use dampen_core::ir::style::StyleProperties;
use iced::{Element, Renderer, Theme};

/// Convert Dampen StyleProperties to Iced radio Style
fn apply_radio_style(props: &StyleProperties) -> iced::widget::radio::Style {
    use iced::widget::radio;
    use iced::{Background, Color};

    let mut style = radio::Style {
        background: Background::Color(Color::WHITE),
        dot_color: Color::BLACK,
        border_width: 1.0,
        border_color: Color::from_rgb(0.5, 0.5, 0.5),
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
        style.border_color = Color {
            r: border.color.r,
            g: border.color.g,
            b: border.color.b,
            a: border.color.a,
        };
        style.border_width = border.width;
    }

    if let Some(ref dot_color) = props.color {
        style.dot_color = Color {
            r: dot_color.r,
            g: dot_color.g,
            b: dot_color.b,
            a: dot_color.a,
        };
    }

    style
}

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

        #[cfg(debug_assertions)]
        eprintln!(
            "[DampenWidgetBuilder] Building radio: label='{}', value='{}', selected={:?}",
            label, value, selected
        );

        // Get handler from events
        let on_select_event = node
            .events
            .iter()
            .find(|e| e.event == dampen_core::EventKind::Select);

        #[cfg(debug_assertions)]
        if let Some(event) = &on_select_event {
            eprintln!(
                "[DampenWidgetBuilder] Radio has select event: handler={}, param={:?}",
                event.handler, event.param
            );
        } else {
            eprintln!("[DampenWidgetBuilder] Radio has no select event");
        }

        // Determine if this radio is currently selected
        let is_selected = selected.as_ref().map(|s| s == &value).unwrap_or(false);

        // Evaluate disabled attribute (default: false)
        let is_disabled = resolve_boolean_attribute(self, node, "disabled", false);

        #[cfg(debug_assertions)]
        eprintln!(
            "[DampenWidgetBuilder] Radio '{}' disabled: {}",
            label, is_disabled
        );

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

                #[cfg(debug_assertions)]
                eprintln!(
                    "[DampenWidgetBuilder] Radio: Attaching on_select with handler '{}', value='{}' (id={})",
                    handler_name, value_clone, value_id
                );

                // Create radio with handler - sends the string value when selected
                iced::widget::radio(label, value_id, selected_id, move |_selected_id| {
                    HandlerMessage::Handler(handler_name.clone(), Some(value_clone.clone()))
                })
            } else {
                #[cfg(debug_assertions)]
                if is_disabled {
                    eprintln!(
                        "[DampenWidgetBuilder] Radio: Disabled, creating non-interactive radio"
                    );
                } else {
                    eprintln!(
                        "[DampenWidgetBuilder] Radio: No handler_registry, creating read-only radio"
                    );
                }
                // Disabled or no handler registry - create read-only radio
                iced::widget::radio(label, value_id, selected_id, |_| {
                    HandlerMessage::Handler(String::new(), None)
                })
            }
        } else {
            #[cfg(debug_assertions)]
            eprintln!("[DampenWidgetBuilder] Radio: No on_select event, creating read-only radio");
            // No event handler - create read-only radio
            iced::widget::radio(label, value_id, selected_id, |_| {
                HandlerMessage::Handler(String::new(), None)
            })
        };

        // Resolve and apply radio styles with state-aware styling
        // Use complete style resolution: theme → class → inline
        let resolved_base_style = self.resolve_complete_styles(node);

        // Get the StyleClass for state variant resolution, wrapped in Rc for efficient cloning
        let style_class = if !node.classes.is_empty() {
            self.style_classes
                .and_then(|classes| node.classes.first().and_then(|name| classes.get(name)))
                .cloned()
                .map(std::rc::Rc::new)
        } else {
            None
        };

        // Apply state-aware styling using generic helper
        // Note: Radio doesn't have Status::Disabled in Iced 0.14, so we handle it manually
        let radio_widget = if let Some(base_style_props) = resolved_base_style {
            use crate::builder::helpers::create_state_aware_style_fn;
            use crate::style_mapping::map_radio_status;
            use dampen_core::ir::WidgetState;

            let base_style_props = base_style_props.clone();

            let status_mapper = move |status: iced::widget::radio::Status| {
                if is_disabled {
                    Some(WidgetState::Disabled)
                } else {
                    map_radio_status(status)
                }
            };

            if let Some(style_fn) = create_state_aware_style_fn(
                self,
                node,
                dampen_core::ir::WidgetKind::Radio,
                style_class,
                base_style_props,
                status_mapper,
                apply_radio_style,
            ) {
                radio_widget.style(style_fn)
            } else {
                radio_widget
            }
        } else {
            radio_widget
        };

        radio_widget.into()
    }
}
