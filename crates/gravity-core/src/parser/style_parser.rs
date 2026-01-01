//! Style attribute parsing
//!
//! This module provides parsers for individual style attributes.

use crate::ir::layout::{Alignment, Justification, Length, Padding};
use crate::ir::style::{
    Background, Border, BorderRadius, BorderStyle, Color, Shadow, StyleProperties, Transform,
};
use crate::parser::gradient::parse_gradient;

/// Parse color attribute
pub fn parse_color_attr(s: &str) -> Result<Color, String> {
    Color::parse(s)
}

/// Parse length attribute (width, height, etc.)
pub fn parse_length_attr(s: &str) -> Result<Length, String> {
    Length::parse(s)
}

/// Parse padding attribute
pub fn parse_padding_attr(s: &str) -> Result<Padding, String> {
    Padding::parse(s)
}

/// Parse shadow attribute
pub fn parse_shadow_attr(s: &str) -> Result<Shadow, String> {
    Shadow::parse(s)
}

/// Parse background attribute (color, gradient, or image)
pub fn parse_background_attr(s: &str) -> Result<Background, String> {
    let s = s.trim();

    // Check for gradient
    if s.starts_with("linear-gradient(") || s.starts_with("radial-gradient(") {
        let gradient = parse_gradient(s)?;
        return Ok(Background::Gradient(gradient));
    }

    // Check for image
    if s.starts_with("url(") && s.ends_with(')') {
        let path = &s[4..s.len() - 1];
        return Ok(Background::Image {
            path: path.to_string(),
            fit: crate::ir::style::ImageFit::Cover,
        });
    }

    // Otherwise, parse as color
    let color = Color::parse(s)?;
    Ok(Background::Color(color))
}

/// Parse border width
pub fn parse_border_width(s: &str) -> Result<f32, String> {
    s.parse()
        .map_err(|_| format!("Invalid border width: {}", s))
}

/// Parse border color
pub fn parse_border_color(s: &str) -> Result<Color, String> {
    Color::parse(s)
}

/// Parse border radius
pub fn parse_border_radius(s: &str) -> Result<BorderRadius, String> {
    BorderRadius::parse(s)
}

/// Parse border style
pub fn parse_border_style(s: &str) -> Result<BorderStyle, String> {
    BorderStyle::parse(s)
}

/// Parse opacity
pub fn parse_opacity(s: &str) -> Result<f32, String> {
    let value: f32 = s.parse().map_err(|_| format!("Invalid opacity: {}", s))?;
    if value < 0.0 || value > 1.0 {
        return Err(format!("Opacity must be 0.0-1.0, got {}", value));
    }
    Ok(value)
}

/// Parse transform
pub fn parse_transform(s: &str) -> Result<Transform, String> {
    Transform::parse(s)
}

/// Parse alignment
pub fn parse_alignment(s: &str) -> Result<Alignment, String> {
    Alignment::parse(s)
}

/// Parse justification
pub fn parse_justification(s: &str) -> Result<Justification, String> {
    Justification::parse(s)
}

/// Parse spacing (must be non-negative)
pub fn parse_spacing(s: &str) -> Result<f32, String> {
    let value: f32 = s.parse().map_err(|_| format!("Invalid spacing: {}", s))?;
    if value < 0.0 {
        return Err(format!("Spacing must be non-negative, got {}", value));
    }
    Ok(value)
}

/// Parse min/max constraints
pub fn parse_constraint(s: &str) -> Result<f32, String> {
    let value: f32 = s
        .parse()
        .map_err(|_| format!("Invalid constraint value: {}", s))?;
    if value < 0.0 {
        return Err(format!("Constraint must be non-negative, got {}", value));
    }
    Ok(value)
}

/// Parse fill_portion value
pub fn parse_fill_portion(s: &str) -> Result<u8, String> {
    let value: u8 = s
        .parse()
        .map_err(|_| format!("Invalid fill_portion: {}", s))?;
    if value == 0 || value > 255 {
        return Err(format!("fill_portion must be 1-255, got {}", value));
    }
    Ok(value)
}

/// Parse percentage value
pub fn parse_percentage(s: &str) -> Result<f32, String> {
    if !s.ends_with('%') {
        return Err(format!("Percentage must end with '%', got {}", s));
    }
    let num = &s[..s.len() - 1];
    let value: f32 = num
        .parse()
        .map_err(|_| format!("Invalid percentage: {}", s))?;
    if value < 0.0 || value > 100.0 {
        return Err(format!("Percentage must be 0.0-100.0, got {}", value));
    }
    Ok(value)
}

/// Build StyleProperties from individual attribute values
pub fn build_style_properties(
    background: Option<Background>,
    color: Option<Color>,
    border: Option<Border>,
    shadow: Option<Shadow>,
    opacity: Option<f32>,
    transform: Option<Transform>,
) -> Result<StyleProperties, String> {
    let style = StyleProperties {
        background,
        color,
        border,
        shadow,
        opacity,
        transform,
    };

    style.validate()?;
    Ok(style)
}

/// Build Border from individual attribute values
pub fn build_border(
    width: Option<f32>,
    color: Option<Color>,
    radius: Option<BorderRadius>,
    style: Option<BorderStyle>,
) -> Result<Option<Border>, String> {
    if width.is_none() && color.is_none() && radius.is_none() && style.is_none() {
        return Ok(None);
    }

    let border = Border {
        width: width.unwrap_or(0.0),
        color: color.unwrap_or(Color {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 1.0,
        }),
        radius: radius.unwrap_or(BorderRadius {
            top_left: 0.0,
            top_right: 0.0,
            bottom_right: 0.0,
            bottom_left: 0.0,
        }),
        style: style.unwrap_or(BorderStyle::Solid),
    };

    border.validate()?;
    Ok(Some(border))
}
