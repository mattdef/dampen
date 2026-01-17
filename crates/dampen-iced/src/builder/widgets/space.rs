//! Space widget builder

use crate::HandlerMessage;
use crate::builder::DampenWidgetBuilder;
use dampen_core::ir::node::WidgetNode;
use iced::{Element, Renderer, Theme};

use super::super::helpers::parse_length;

impl<'a> DampenWidgetBuilder<'a> {
    pub(in crate::builder) fn build_space(
        &self,
        node: &WidgetNode,
    ) -> Element<'a, HandlerMessage, Theme, Renderer>
    where
        HandlerMessage: Clone + 'static,
    {
        let mut space = iced::widget::Space::new();

        // Apply width if specified
        if let Some(width_attr) = node.attributes.get("width") {
            let width_str = self.evaluate_attribute(width_attr);
            if let Some(length) = parse_length(&width_str) {
                space = space.width(length);
            }
        }

        // Apply height if specified
        if let Some(height_attr) = node.attributes.get("height") {
            let height_str = self.evaluate_attribute(height_attr);
            if let Some(length) = parse_length(&height_str) {
                space = space.height(length);
            }
        }

        space.into()
    }
}
