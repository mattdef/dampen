//! Slider widget builder

use crate::HandlerMessage;
use crate::builder::DampenWidgetBuilder;
use crate::builder::helpers::{create_state_aware_style_fn, resolve_boolean_attribute};
use dampen_core::ir::node::WidgetNode;
use dampen_core::ir::style::StyleProperties;
use iced::{Element, Renderer, Theme};

/// Convert Dampen StyleProperties to Iced slider Style
fn apply_slider_style(props: &StyleProperties) -> iced::widget::slider::Style {
    use iced::widget::slider;
    use iced::{Background, Border, Color};

    let mut rail_bg = (
        Background::Color(Color::from_rgb(0.6, 0.6, 0.6)),
        Background::Color(Color::from_rgb(0.2, 0.6, 1.0)),
    );
    let mut rail_width = 4.0;
    let mut rail_border = Border::default();

    if let Some(ref color) = props.color {
        rail_bg = (
            Background::Color(Color {
                r: color.r * 0.5,
                g: color.g * 0.5,
                b: color.b * 0.5,
                a: 1.0,
            }),
            Background::Color(Color {
                r: color.r,
                g: color.g,
                b: color.b,
                a: color.a,
            }),
        );
    }

    if let Some(ref border) = props.border {
        rail_border = Border {
            color: Color {
                r: border.color.r,
                g: border.color.g,
                b: border.color.b,
                a: border.color.a,
            },
            width: border.width,
            radius: iced::border::Radius::new(0.0),
        };
        rail_width = border.width;
    }

    slider::Style {
        rail: slider::Rail {
            backgrounds: rail_bg,
            width: rail_width,
            border: rail_border,
        },
        handle: slider::Handle {
            shape: slider::HandleShape::Circle { radius: 8.0 },
            background: Background::Color(if let Some(ref c) = props.color {
                Color {
                    r: c.r,
                    g: c.g,
                    b: c.b,
                    a: c.a,
                }
            } else {
                Color::from_rgb(0.4, 0.4, 0.8)
            }),
            border_width: 1.0,
            border_color: if let Some(ref c) = props.color {
                Color {
                    r: (c.r * 0.7).min(1.0),
                    g: (c.g * 0.7).min(1.0),
                    b: (c.b * 0.7).min(1.0),
                    a: 1.0,
                }
            } else {
                Color::from_rgb(0.3, 0.3, 0.7)
            },
        },
    }
}

impl<'a> DampenWidgetBuilder<'a> {
    /// Build a slider widget from Dampen XML definition
    ///
    /// Supports the following attributes:
    /// - `min`: Minimum value (default 0.0)
    /// - `max`: Maximum value (default 100.0)
    /// - `value`: Float binding for current value (clamped to [min, max])
    /// - `on_change`: Handler called on change with stringified float value
    /// - `disabled`: Boolean attribute for disabled state
    ///
    /// Events: Change (sends HandlerMessage::Handler(name, Some(value.to_string())))
    pub(in crate::builder) fn build_slider(
        &self,
        node: &WidgetNode,
    ) -> Element<'a, HandlerMessage, Theme, Renderer> {
        let min = node
            .attributes
            .get("min")
            .map(|attr| self.evaluate_attribute(attr))
            .unwrap_or_else(|| "0.0".to_string())
            .parse::<f32>()
            .unwrap_or(0.0);

        let max = node
            .attributes
            .get("max")
            .map(|attr| self.evaluate_attribute(attr))
            .unwrap_or_else(|| "100.0".to_string())
            .parse::<f32>()
            .unwrap_or(100.0);

        let value_str = node
            .attributes
            .get("value")
            .map(|attr| self.evaluate_attribute(attr))
            .unwrap_or_else(|| "50.0".to_string());

        let mut value = value_str.parse::<f32>().unwrap_or(50.0);

        // Clamp value to [min, max]
        value = value.max(min).min(max);

        // Get optional step value
        let step = node
            .attributes
            .get("step")
            .map(|attr| self.evaluate_attribute(attr))
            .and_then(|s| s.parse::<f32>().ok());

        // Get handler from events
        let on_change = node
            .events
            .iter()
            .find(|e| e.event == dampen_core::EventKind::Change)
            .map(|e| e.handler.clone());

        let slider = if let Some(handler_name) = on_change {
            if self.handler_registry.is_some() {
                let mut slider = iced::widget::slider(min..=max, value, move |new_value| {
                    HandlerMessage::Handler(handler_name.clone(), Some(new_value.to_string()))
                });
                if let Some(step_val) = step {
                    slider = slider.step(step_val);
                }
                slider
            } else {
                let mut slider = iced::widget::slider(min..=max, value, |_| {
                    HandlerMessage::Handler("dummy".to_string(), None)
                });
                if let Some(step_val) = step {
                    slider = slider.step(step_val);
                }
                slider
            }
        } else {
            let mut slider = iced::widget::slider(min..=max, value, |_| {
                HandlerMessage::Handler("dummy".to_string(), None)
            });
            if let Some(step_val) = step {
                slider = slider.step(step_val);
            }
            slider
        };

        // Apply state-aware styling
        // Note: Iced slider::Status has no Disabled variant, so we check disabled attribute
        let is_disabled = resolve_boolean_attribute(self, node, "disabled", false);
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

        let slider = if let Some(base_style_props) = resolved_base_style {
            use crate::style_mapping::map_slider_status;

            let base_style_props = base_style_props.clone();
            let status_mapper =
                move |status: iced::widget::slider::Status| map_slider_status(status, is_disabled);

            if let Some(style_fn) = create_state_aware_style_fn(
                self,
                node,
                dampen_core::ir::WidgetKind::Slider,
                style_class,
                base_style_props,
                status_mapper,
                apply_slider_style,
            ) {
                slider.style(style_fn)
            } else {
                slider
            }
        } else {
            slider
        };

        slider.into()
    }
}
