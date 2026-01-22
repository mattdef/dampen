//! Toggler widget builder

use crate::HandlerMessage;
use crate::builder::DampenWidgetBuilder;
use dampen_core::ir::node::WidgetNode;
use dampen_core::ir::style::StyleProperties;
use iced::{Element, Renderer, Theme};

/// Convert Dampen StyleProperties to Iced toggler Style
fn apply_toggler_style(props: &StyleProperties) -> iced::widget::toggler::Style {
    use iced::widget::toggler;
    use iced::{Background, Color};

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

    if let Some(ref fg_color) = props.color {
        style.foreground = Background::Color(Color {
            r: fg_color.r,
            g: fg_color.g,
            b: fg_color.b,
            a: fg_color.a,
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
        style.background_border_color = Color {
            r: border.color.r,
            g: border.color.g,
            b: border.color.b,
            a: border.color.a,
        };
        style.background_border_width = border.width;
    }

    style
}

impl<'a> DampenWidgetBuilder<'a> {
    /// Build a toggler widget from Dampen XML definition
    ///
    /// Supports the following attributes:
    /// - `label`: Text label displayed next to toggler
    /// - `toggled`: Boolean binding for toggled state (legacy: `active`)
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

        // Support both 'toggled' (standard) and 'active' (legacy, normalized by parser)
        let active_str = node
            .attributes
            .get("toggled")
            .or_else(|| node.attributes.get("active"))
            .map(|attr| self.evaluate_attribute(attr))
            .unwrap_or_else(|| "false".to_string());

        let is_active = active_str == "true" || active_str == "1";

        #[cfg(debug_assertions)]
        eprintln!(
            "[DampenWidgetBuilder] Building toggler: label='{}', active={}",
            label, is_active
        );

        // Get handler from events
        let on_toggle = node
            .events
            .iter()
            .find(|e| e.event == dampen_core::EventKind::Toggle)
            .map(|e| e.handler.clone());

        #[cfg(debug_assertions)]
        if let Some(handler) = &on_toggle {
            eprintln!(
                "[DampenWidgetBuilder] Toggler has toggle event: handler={}",
                handler
            );
        } else {
            eprintln!("[DampenWidgetBuilder] Toggler has no toggle event");
        }

        let mut toggler = iced::widget::toggler(is_active);

        // Resolve and apply toggler styles with state-aware styling
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
            use crate::style_mapping::map_toggler_status;

            let base_style_props = base_style_props.clone();

            if let Some(style_fn) = create_state_aware_style_fn(
                self,
                node,
                dampen_core::ir::WidgetKind::Toggler,
                style_class,
                base_style_props,
                map_toggler_status,
                apply_toggler_style,
            ) {
                toggler = toggler.style(style_fn);
            }
        }

        // Connect event if handler exists
        if let Some(handler_name) = on_toggle {
            if self.handler_registry.is_some() {
                #[cfg(debug_assertions)]
                eprintln!(
                    "[DampenWidgetBuilder] Toggler: Attaching on_toggle with handler '{}'",
                    handler_name
                );
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
                #[cfg(debug_assertions)]
                eprintln!(
                    "[DampenWidgetBuilder] Toggler: No handler_registry, cannot attach on_toggle"
                );
            }
        }

        let text_widget = iced::widget::text(label);
        let row = iced::widget::row(vec![toggler.into(), text_widget.into()]);
        row.into()
    }
}
