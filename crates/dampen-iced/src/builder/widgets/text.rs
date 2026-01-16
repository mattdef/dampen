//! Text widget builder

use crate::HandlerMessage;
use crate::builder::DampenWidgetBuilder;
use crate::builder::helpers::{merge_styles, parse_color};
use dampen_core::ir::node::WidgetNode;
use iced::{Element, Renderer, Theme};

impl<'a> DampenWidgetBuilder<'a> {
    pub(in crate::builder) fn build_text(
        &self,
        node: &WidgetNode,
    ) -> Element<'a, HandlerMessage, Theme, Renderer>
    where
        HandlerMessage: Clone + 'static,
    {
        let value = node
            .attributes
            .get("value")
            .map(|attr| self.evaluate_attribute(attr))
            .unwrap_or_default();

        let mut text_widget = iced::widget::text(value);

        // Resolve and apply text styles
        let resolved_style = match (self.resolve_class_styles(node), &node.style) {
            (Some(class_style), Some(node_style)) => Some(merge_styles(class_style, node_style)),
            (Some(class_style), None) => Some(class_style),
            (None, Some(node_style)) => Some(node_style.clone()),
            (None, None) => None,
        };

        // Apply color from styles
        if let Some(style_props) = resolved_style {
            if let Some(ref color) = style_props.color {
                text_widget = text_widget.color(iced::Color {
                    r: color.r,
                    g: color.g,
                    b: color.b,
                    a: color.a,
                });
            }
        }

        // Check for direct attributes (size, weight, color) that override styles
        if let Some(color_attr) = node.attributes.get("color") {
            let color_str = self.evaluate_attribute(color_attr);
            if let Some(color) = parse_color(&color_str) {
                text_widget = text_widget.color(iced::Color {
                    r: color.r,
                    g: color.g,
                    b: color.b,
                    a: color.a,
                });
            }
        }

        if let Some(size_attr) = node.attributes.get("size") {
            let size_str = self.evaluate_attribute(size_attr);
            if let Ok(size) = size_str.parse::<f32>() {
                text_widget = text_widget.size(size);
            }
        }

        if let Some(weight_attr) = node.attributes.get("weight") {
            let weight_str = self.evaluate_attribute(weight_attr);
            if weight_str == "bold" {
                text_widget = text_widget.font(iced::Font {
                    weight: iced::font::Weight::Bold,
                    ..Default::default()
                });
            }
        }

        text_widget.into()
    }
}
