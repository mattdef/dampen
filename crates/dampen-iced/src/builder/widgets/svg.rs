//! SVG widget builder

use crate::HandlerMessage;
use crate::builder::DampenWidgetBuilder;
use dampen_core::ir::node::WidgetNode;
use iced::{Element, Renderer, Theme};

impl<'a> DampenWidgetBuilder<'a> {
    pub(in crate::builder) fn build_svg(
        &self,
        node: &WidgetNode,
    ) -> Element<'a, HandlerMessage, Theme, Renderer>
    where
        HandlerMessage: Clone + 'static,
    {
        // Support both 'src' and 'path' attributes
        let src = node
            .attributes
            .get("src")
            .or_else(|| node.attributes.get("path"))
            .map(|attr| self.evaluate_attribute(attr))
            .unwrap_or_default();

        if src.is_empty() {
            if self.verbose {
                eprintln!("[DampenWidgetBuilder] SVG src is empty");
            }
            return iced::widget::text("[SVG: no src]").into();
        }

        let handle = iced::widget::svg::Handle::from_path(src);
        let mut svg = iced::widget::svg(handle);

        // Parse optional width
        if let Some(width_attr) = node.attributes.get("width") {
            if let Ok(width) = self.evaluate_attribute(width_attr).parse::<f32>() {
                svg = svg.width(width);
            }
        }

        // Parse optional height
        if let Some(height_attr) = node.attributes.get("height") {
            if let Ok(height) = self.evaluate_attribute(height_attr).parse::<f32>() {
                svg = svg.height(height);
            }
        }

        svg.into()
    }
}
