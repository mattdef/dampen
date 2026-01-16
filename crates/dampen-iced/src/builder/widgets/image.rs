//! Image widget builder

use crate::HandlerMessage;
use crate::builder::DampenWidgetBuilder;
use dampen_core::ir::node::WidgetNode;
use iced::{Element, Renderer, Theme};

impl<'a> DampenWidgetBuilder<'a> {
    pub(in crate::builder) fn build_image(
        &self,
        node: &WidgetNode,
    ) -> Element<'a, HandlerMessage, Theme, Renderer>
    where
        HandlerMessage: Clone + 'static,
    {
        // Support both 'src' (standard) and 'path' (legacy) attributes
        let src = node
            .attributes
            .get("src")
            .or_else(|| node.attributes.get("path"))
            .map(|attr| self.evaluate_attribute(attr))
            .unwrap_or_default();

        if src.is_empty() {
            if self.verbose {
                eprintln!("[DampenWidgetBuilder] Image src is empty");
            }
            return iced::widget::text("[Image: no src]").into();
        }

        let handle = iced::widget::image::Handle::from_path(src);

        let mut image = iced::widget::image(handle);

        if let Some(width_attr) = node.attributes.get("width") {
            if let Ok(width) = self.evaluate_attribute(width_attr).parse::<f32>() {
                image = image.width(width);
            }
        }

        if let Some(height_attr) = node.attributes.get("height") {
            if let Ok(height) = self.evaluate_attribute(height_attr).parse::<f32>() {
                image = image.height(height);
            }
        }

        image.into()
    }
}
