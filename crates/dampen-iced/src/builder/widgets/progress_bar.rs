//! ProgressBar widget builder

use crate::HandlerMessage;
use crate::builder::DampenWidgetBuilder;
use crate::builder::helpers::parse_length;
use dampen_core::ir::node::{AttributeValue, WidgetNode};
use iced::{Element, Renderer, Theme};

/// Style variants for progress bar
#[derive(Clone, Copy)]
enum ProgressBarStyle {
    Primary,
    Success,
    Warning,
    Danger,
    Secondary,
}

impl ProgressBarStyle {
    fn from_str(s: &str) -> Self {
        match s {
            "success" => Self::Success,
            "warning" => Self::Warning,
            "danger" => Self::Danger,
            "secondary" => Self::Secondary,
            _ => Self::Primary,
        }
    }

    fn get_bar_color(self, theme: &Theme) -> iced::Color {
        let palette = theme.extended_palette();
        match self {
            Self::Success => palette.success.base.color,
            Self::Warning => palette.warning.base.color,
            Self::Danger => palette.danger.base.color,
            Self::Secondary => palette.secondary.base.color,
            Self::Primary => palette.primary.base.color,
        }
    }
}

/// Parse a color string into an iced Color
fn parse_color(color_str: &str) -> Option<iced::Color> {
    // Try hex color (#RRGGBB or #RRGGBBAA)
    if let Some(hex) = color_str.strip_prefix('#') {
        if hex.len() == 6 {
            if let (Ok(r), Ok(g), Ok(b)) = (
                u8::from_str_radix(&hex[0..2], 16),
                u8::from_str_radix(&hex[2..4], 16),
                u8::from_str_radix(&hex[4..6], 16),
            ) {
                return Some(iced::Color::from_rgb(
                    r as f32 / 255.0,
                    g as f32 / 255.0,
                    b as f32 / 255.0,
                ));
            }
        } else if hex.len() == 8
            && let (Ok(r), Ok(g), Ok(b), Ok(a)) = (
                u8::from_str_radix(&hex[0..2], 16),
                u8::from_str_radix(&hex[2..4], 16),
                u8::from_str_radix(&hex[4..6], 16),
                u8::from_str_radix(&hex[6..8], 16),
            )
        {
            return Some(iced::Color::from_rgba(
                r as f32 / 255.0,
                g as f32 / 255.0,
                b as f32 / 255.0,
                a as f32 / 255.0,
            ));
        }
    }

    // Try RGB format: rgb(r,g,b)
    if color_str.starts_with("rgb(") && color_str.ends_with(')') {
        let inner = &color_str[4..color_str.len() - 1];
        let parts: Vec<&str> = inner.split(',').map(|s| s.trim()).collect();
        if parts.len() == 3
            && let (Ok(r), Ok(g), Ok(b)) = (
                parts[0].parse::<u8>(),
                parts[1].parse::<u8>(),
                parts[2].parse::<u8>(),
            )
        {
            return Some(iced::Color::from_rgb(
                r as f32 / 255.0,
                g as f32 / 255.0,
                b as f32 / 255.0,
            ));
        }
    }

    // Try RGBA format: rgba(r,g,b,a)
    if color_str.starts_with("rgba(") && color_str.ends_with(')') {
        let inner = &color_str[5..color_str.len() - 1];
        let parts: Vec<&str> = inner.split(',').map(|s| s.trim()).collect();
        if parts.len() == 4
            && let (Ok(r), Ok(g), Ok(b), Ok(a)) = (
                parts[0].parse::<u8>(),
                parts[1].parse::<u8>(),
                parts[2].parse::<u8>(),
                parts[3].parse::<f32>(),
            )
        {
            return Some(iced::Color::from_rgba(
                r as f32 / 255.0,
                g as f32 / 255.0,
                b as f32 / 255.0,
                a,
            ));
        }
    }

    None
}

impl<'a> DampenWidgetBuilder<'a> {
    pub(in crate::builder) fn build_progress_bar(
        &self,
        node: &WidgetNode,
    ) -> Element<'a, HandlerMessage, Theme, Renderer>
    where
        HandlerMessage: Clone + 'static,
    {
        // Parse min, max, and value attributes
        let min = match node.attributes.get("min") {
            Some(AttributeValue::Static(s)) => s.parse::<f32>().ok().unwrap_or(0.0),
            _ => 0.0,
        };

        let max = match node.attributes.get("max") {
            Some(AttributeValue::Static(s)) => s.parse::<f32>().ok().unwrap_or(1.0),
            _ => 1.0,
        };

        let value_str = match node.attributes.get("value") {
            Some(attr) => self.evaluate_attribute(attr),
            None => "0".to_string(),
        };
        let value = value_str.parse::<f32>().unwrap_or(0.0);

        // Clamp value to [min, max] range
        let clamped_value = value.min(max).max(min);

        // Parse style attribute
        let style = node
            .attributes
            .get("style")
            .and_then(|attr| {
                if let AttributeValue::Static(s) = attr {
                    Some(ProgressBarStyle::from_str(s))
                } else {
                    None
                }
            })
            .unwrap_or(ProgressBarStyle::Primary);

        // Parse custom colors
        // bar_color is bindable - evaluate attribute to support bindings
        let bar_color_str = node
            .attributes
            .get("bar_color")
            .map(|attr| self.evaluate_attribute(attr));
        let bar_color = bar_color_str.as_deref().and_then(parse_color);

        let background_color = node.attributes.get("background_color").and_then(|attr| {
            if let AttributeValue::Static(s) = attr {
                parse_color(s)
            } else {
                None
            }
        });

        // Parse border radius
        let border_radius = node.attributes.get("border_radius").and_then(|attr| {
            if let AttributeValue::Static(s) = attr {
                s.parse::<f32>().ok()
            } else {
                None
            }
        });

        // Parse height (girth)
        let height = node.attributes.get("height").and_then(|attr| {
            if let AttributeValue::Static(s) = attr {
                s.parse::<f32>().ok()
            } else {
                None
            }
        });

        // Parse width
        let width = node.attributes.get("width").and_then(|attr| {
            if let AttributeValue::Static(s) = attr {
                parse_length(s)
            } else {
                None
            }
        });

        // Create progress bar
        let mut progress_bar = iced::widget::progress_bar(min..=max, clamped_value);

        // Apply height if specified
        if let Some(h) = height {
            progress_bar = progress_bar.girth(h);
        }

        // Apply width if specified
        if let Some(w) = width {
            progress_bar = progress_bar.length(w);
        }

        // Apply style
        progress_bar = progress_bar.style(move |theme: &Theme| {
            let palette = theme.extended_palette();

            // Determine bar color (custom or from style)
            let bar = if let Some(color) = bar_color {
                iced::Background::Color(color)
            } else {
                iced::Background::Color(style.get_bar_color(theme))
            };

            // Determine background color (custom or default)
            let background = if let Some(color) = background_color {
                iced::Background::Color(color)
            } else {
                iced::Background::Color(palette.background.weak.color)
            };

            // Build border with optional radius
            let border = if let Some(radius) = border_radius {
                iced::Border::default().rounded(radius)
            } else {
                iced::Border::default()
            };

            iced::widget::progress_bar::Style {
                background,
                bar,
                border,
            }
        });

        progress_bar.into()
    }
}
