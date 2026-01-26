//! Stack widget builder

use crate::HandlerMessage;
use crate::builder::DampenWidgetBuilder;
use dampen_core::ir::node::WidgetNode;
use iced::{Element, Renderer, Theme};

impl<'a> DampenWidgetBuilder<'a> {
    pub(in crate::builder) fn build_stack(
        &self,
        node: &WidgetNode,
    ) -> Element<'a, HandlerMessage, Theme, Renderer>
    where
        HandlerMessage: Clone + 'static,
    {
        let children: Vec<Element<'a, HandlerMessage, Theme, Renderer>> = node
            .children
            .iter()
            .map(|child| self.build_widget(child))
            .collect();

        // Stack children in a container
        let content: Element<'a, HandlerMessage, Theme, Renderer> = if children.is_empty() {
            iced::widget::text("").into()
        } else {
            iced::widget::column(children).into()
        };

        let mut container = iced::widget::container(content);

        // Handle width attribute
        if let Some(width_attr) = node.attributes.get("width") {
            let width_value = self.evaluate_attribute(width_attr);
            if !width_value.is_empty() {
                match width_value.as_str() {
                    "fill" | "100%" => {
                        container = container.width(iced::Length::Fill);
                    }
                    _ => {
                        if let Ok(pixels) = width_value.parse::<f32>() {
                            container = container.width(iced::Length::Fixed(pixels));
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
                        container = container.height(iced::Length::Fill);
                    }
                    _ => {
                        if let Ok(pixels) = height_value.parse::<f32>() {
                            container = container.height(iced::Length::Fixed(pixels));
                        }
                    }
                }
            }
        }

        container.into()
    }
}
