//! Text widget builder

use crate::HandlerMessage;
use crate::builder::DampenWidgetBuilder;
use crate::builder::helpers::parse_color;
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

        // Resolve text color with theme awareness
        // Priority: direct color attribute > inline style color > class color > theme color > default
        let mut theme_color_applied = false;

        if let Some(theme_ctx) = self.theme_context {
            // Get theme text color at render time
            let active_theme = theme_ctx.active();
            if let Some(ref text_color) = active_theme.palette.text {
                // Check if there's no direct color attribute, inline style, or class color overriding it
                let has_direct_color = node.attributes.contains_key("color");
                let has_inline_color = node.style.as_ref().and_then(|s| s.color.as_ref()).is_some();
                let has_class_color = self
                    .resolve_class_styles(node)
                    .and_then(|s| s.color)
                    .is_some();

                if !has_direct_color && !has_inline_color && !has_class_color {
                    text_widget = text_widget.color(iced::Color {
                        r: text_color.r,
                        g: text_color.g,
                        b: text_color.b,
                        a: text_color.a,
                    });
                    theme_color_applied = true;
                }
            }
        }

        // Resolve and apply text styles with complete resolution: theme → class → inline
        // (but skip theme colors since we already applied them above if appropriate)
        let resolved_style = self.resolve_complete_styles(node);

        // Apply color from styles (only if not already applied from theme)
        if let Some(style_props) = resolved_style {
            if let Some(ref color) = style_props.color {
                if !theme_color_applied {
                    text_widget = text_widget.color(iced::Color {
                        r: color.r,
                        g: color.g,
                        b: color.b,
                        a: color.a,
                    });
                }
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

        // Note: align_x and align_y are NOT applied here.
        // These layout properties are handled by apply_style_layout which wraps
        // the text in a container when width/height/padding are specified.
        // The text widget in Iced only supports horizontal/vertical alignment
        // when it has a fixed size, which is managed at the container level.

        self.apply_style_layout(text_widget, node)
    }
}
