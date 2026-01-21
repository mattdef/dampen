//! Canvas widget builder

use crate::HandlerMessage;
use crate::builder::DampenWidgetBuilder;
use dampen_core::ir::node::{AttributeValue, WidgetNode};
use iced::{Element, Renderer, Theme};

impl<'a> DampenWidgetBuilder<'a> {
    /// T076: Implement Canvas rendering with Program binding evaluation
    /// T077: Implement Canvas click event handling with coordinate passing
    pub(in crate::builder) fn build_canvas(
        &self,
        node: &WidgetNode,
    ) -> Element<'a, HandlerMessage, Theme, Renderer>
    where
        HandlerMessage: Clone + 'static,
    {
        // Parse width and height attributes (validated by parser, so they exist)
        let width = match node.attributes.get("width") {
            Some(AttributeValue::Static(s)) => s.parse::<f32>().ok().unwrap_or(400.0),
            _ => 400.0,
        };

        let height = match node.attributes.get("height") {
            Some(AttributeValue::Static(s)) => s.parse::<f32>().ok().unwrap_or(300.0),
            _ => 300.0,
        };

        #[cfg(debug_assertions)]
        eprintln!(
            "[DampenWidgetBuilder] Building Canvas widget: {}x{}",
            width, height
        );

        // Note: Canvas requires a custom Program implementation
        // For now, we create a placeholder container with a message
        // Real Canvas programs must be implemented in Rust code

        // Get program binding attribute for logging
        if let Some(AttributeValue::Binding(expr)) = node.attributes.get("program") {
            #[cfg(debug_assertions)]
            eprintln!("[DampenWidgetBuilder] Canvas program binding: {:?}", expr);
        }

        // Create a placeholder: container with text explaining Canvas limitation
        let placeholder = iced::widget::container(
            iced::widget::text("Canvas widget requires custom Program implementation in Rust")
                .size(14),
        )
        .width(iced::Length::Fixed(width))
        .height(iced::Length::Fixed(height))
        .center_x(iced::Length::Fill)
        .center_y(iced::Length::Fill)
        .style(|_theme: &iced::Theme| iced::widget::container::Style {
            background: Some(iced::Background::Color(iced::Color::from_rgb(
                0.95, 0.95, 0.95,
            ))),
            border: iced::Border {
                color: iced::Color::from_rgb(0.7, 0.7, 0.7),
                width: 1.0,
                radius: 4.0.into(),
            },
            ..iced::widget::container::Style::default()
        });

        // TODO: When canvas::Program can be accessed from model binding,
        // use: iced::widget::canvas(program)
        // For now, return placeholder
        placeholder.into()
    }
}
