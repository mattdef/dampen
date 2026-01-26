//! Scrollable widget builder

use crate::HandlerMessage;
use crate::builder::DampenWidgetBuilder;
use dampen_core::ir::node::WidgetNode;
use iced::{Element, Renderer, Theme};

impl<'a> DampenWidgetBuilder<'a> {
    pub(in crate::builder) fn build_scrollable(
        &self,
        node: &WidgetNode,
    ) -> Element<'a, HandlerMessage, Theme, Renderer>
    where
        HandlerMessage: Clone + 'static,
    {
        let content = if let Some(first_child) = node.children.first() {
            self.build_widget(first_child)
        } else {
            iced::widget::text("").into()
        };

        let mut scrollable = iced::widget::scrollable(content);

        // Handle width attribute
        if let Some(width_attr) = node.attributes.get("width") {
            let width_value = self.evaluate_attribute(width_attr);
            if !width_value.is_empty() {
                match width_value.as_str() {
                    "fill" | "100%" => {
                        scrollable = scrollable.width(iced::Length::Fill);
                    }
                    _ => {
                        if let Ok(pixels) = width_value.parse::<f32>() {
                            scrollable = scrollable.width(iced::Length::Fixed(pixels));
                        }
                    }
                }
            }
        }

        // Handle height attribute
        if let Some(height_attr) = node.attributes.get("height") {
            let height_value = self.evaluate_attribute(height_attr);
            if !height_value.is_empty() {
                match height_value.as_str() {
                    "fill" | "100%" => {
                        scrollable = scrollable.height(iced::Length::Fill);
                    }
                    _ => {
                        if let Ok(pixels) = height_value.parse::<f32>() {
                            scrollable = scrollable.height(iced::Length::Fixed(pixels));
                        }
                    }
                }
            }
        }

        scrollable.into()
    }
}
