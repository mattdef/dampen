use crate::HandlerMessage;
use crate::builder::DampenWidgetBuilder;
use dampen_core::ir::node::WidgetNode;
use dampen_core::ir::style::Color as DampenColor;
use iced::{Color, Element, Renderer, Theme};
use iced_aw::widgets::color_picker::ColorPicker;

impl<'a> DampenWidgetBuilder<'a> {
    /// Build a color picker widget from Dampen XML definition
    ///
    /// Supports the following attributes:
    /// - `value`: Current color in hex format (#rrggbb or #rrggbbaa), rgb(), rgba(), or named color
    /// - `show`: Boolean binding to control overlay visibility
    /// - `show_alpha`: Boolean to enable alpha channel control
    /// - `enabled`: Boolean to control widget interactivity
    ///
    /// Events:
    /// - `on_submit`: Called when a color is selected, with hex color string payload
    /// - `on_cancel`: Called when the picker is dismissed without selection
    /// - `on_change`: Called when color value changes (for real-time updates)
    ///
    /// # Arguments
    ///
    /// * `node` - The widget node containing attributes and events
    ///
    /// # Returns
    ///
    /// An Iced Element representing the color picker
    pub(in crate::builder) fn build_color_picker(
        &self,
        node: &WidgetNode,
    ) -> Element<'a, HandlerMessage, Theme, Renderer> {
        let show_picker = node
            .attributes
            .get("show")
            .map(|attr| self.evaluate_attribute(attr))
            .map(|v| v == "true" || v == "1")
            .unwrap_or(false);

        let color = if let Some(attr) = node.attributes.get("value") {
            let val = self.evaluate_attribute(attr);
            parse_color(&val).unwrap_or(Color::BLACK)
        } else {
            Color::BLACK
        };

        let show_alpha = node
            .attributes
            .get("show_alpha")
            .map(|attr| self.evaluate_attribute(attr))
            .map(|v| v == "true" || v == "1")
            .unwrap_or(false);

        let _enabled = node
            .attributes
            .get("enabled")
            .map(|attr| self.evaluate_attribute(attr))
            .map(|v| v != "false" && v != "0")
            .unwrap_or(true);

        let on_submit_handler = node
            .events
            .iter()
            .find(|e| e.event == dampen_core::EventKind::Submit)
            .map(|e| e.handler.clone());

        let on_cancel_handler = node
            .events
            .iter()
            .find(|e| e.event == dampen_core::EventKind::Cancel)
            .map(|e| e.handler.clone());

        let child = node.children.first();
        let underlay = if let Some(c) = child {
            self.build_widget(c)
        } else {
            iced::widget::text("ColorPicker requires a child").into()
        };

        let picker = ColorPicker::new(
            show_picker,
            color,
            underlay,
            if let Some(h) = on_cancel_handler {
                HandlerMessage::Handler(h, None)
            } else {
                HandlerMessage::None
            },
            move |color: Color| {
                let hex_str = if show_alpha {
                    color_to_rgba_hex(&color)
                } else {
                    color_to_hex(&color)
                };
                if let Some(ref handler) = on_submit_handler {
                    HandlerMessage::Handler(handler.clone(), Some(hex_str))
                } else {
                    HandlerMessage::None
                }
            },
        );

        picker.into()
    }
}

/// Parse a color string in various CSS formats
fn parse_color(s: &str) -> Option<Color> {
    DampenColor::parse(s)
        .ok()
        .map(|c| Color::from_rgba(c.r, c.g, c.b, c.a))
}

/// Convert a Color to hex string format (#rrggbb)
fn color_to_hex(color: &Color) -> String {
    let dampen_color = DampenColor {
        r: color.r,
        g: color.g,
        b: color.b,
        a: color.a,
    };
    dampen_color.to_hex()
}

/// Convert a Color to hex string format with alpha (#rrggbbaa)
fn color_to_rgba_hex(color: &Color) -> String {
    let dampen_color = DampenColor {
        r: color.r,
        g: color.g,
        b: color.b,
        a: color.a,
    };
    dampen_color.to_rgba_hex()
}
