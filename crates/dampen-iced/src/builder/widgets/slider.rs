//! Slider widget builder

use crate::HandlerMessage;
use crate::builder::DampenWidgetBuilder;
use dampen_core::ir::node::WidgetNode;
use iced::{Element, Renderer, Theme};

impl<'a> DampenWidgetBuilder<'a> {
    /// Build a slider widget from Dampen XML definition
    ///
    /// Supports the following attributes:
    /// - `min`: Minimum value (default 0.0)
    /// - `max`: Maximum value (default 100.0)
    /// - `value`: Float binding for current value (clamped to [min, max])
    /// - `on_change`: Handler called on change with stringified float value
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

        // TODO: State-aware styling available via map_slider_status() - see style_mapping.rs
        // When implementing, check node.attributes for "disabled" attribute and pass
        // to map_slider_status(status, is_disabled) since Iced slider::Status has no Disabled variant

        slider.into()
    }
}
