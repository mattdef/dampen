//! ProgressBar widget builder

use crate::HandlerMessage;
use crate::builder::DampenWidgetBuilder;
use dampen_core::ir::node::{AttributeValue, WidgetNode};
use iced::{Element, Renderer, Theme};

impl<'a> DampenWidgetBuilder<'a> {
    pub(in crate::builder) fn build_progress_bar(
        &self,
        node: &WidgetNode,
    ) -> Element<'a, HandlerMessage, Theme, Renderer>
    where
        HandlerMessage: Clone + 'static,
    {
        // Parse min, max, and value attributes
        let min = match node.attributes.get("min") {
            Some(AttributeValue::Static(s)) => s.parse::<f32>().ok().unwrap_or(0.0),
            _ => 0.0,
        };

        let max = match node.attributes.get("max") {
            Some(AttributeValue::Static(s)) => s.parse::<f32>().ok().unwrap_or(1.0),
            _ => 1.0,
        };

        let value_str = match node.attributes.get("value") {
            Some(attr) => self.evaluate_attribute(attr),
            None => "0".to_string(),
        };
        let value = value_str.parse::<f32>().unwrap_or(0.0);

        // Clamp value to [min, max] range
        let clamped_value = value.min(max).max(min);

        // Create progress bar
        let progress_bar = iced::widget::progress_bar(min..=max, clamped_value);

        progress_bar.into()
    }
}
