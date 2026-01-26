//! Tooltip widget builder

use crate::HandlerMessage;
use crate::builder::DampenWidgetBuilder;
use dampen_core::ir::node::{AttributeValue, WidgetNode};
use iced::{Element, Renderer, Theme};

impl<'a> DampenWidgetBuilder<'a> {
    pub(in crate::builder) fn build_tooltip(
        &self,
        node: &WidgetNode,
    ) -> Element<'a, HandlerMessage, Theme, Renderer>
    where
        HandlerMessage: Clone + 'static,
    {
        // Get message attribute
        let message = match node.attributes.get("message") {
            Some(attr) => self.evaluate_attribute(attr),
            None => "Tooltip".to_string(),
        };

        // Get position attribute (default: FollowCursor)
        let position_str = match node.attributes.get("position") {
            Some(AttributeValue::Static(s)) => s.as_str(),
            _ => "follow_cursor",
        };

        // Map position to Iced Position enum
        let position = match position_str {
            "top" => iced::widget::tooltip::Position::Top,
            "bottom" => iced::widget::tooltip::Position::Bottom,
            "left" => iced::widget::tooltip::Position::Left,
            "right" => iced::widget::tooltip::Position::Right,
            _ => iced::widget::tooltip::Position::FollowCursor,
        };

        // Tooltip must have exactly one child
        if let Some(child) = node.children.first() {
            let child_widget = self.build_widget(child);
            iced::widget::tooltip(child_widget, iced::widget::text(message), position).into()
        } else {
            // No child - return empty text
            iced::widget::text("").into()
        }
    }
}
