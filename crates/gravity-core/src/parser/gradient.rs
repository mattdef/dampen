//! Gradient parsing utilities
//!
//! This module provides parsers for CSS-style gradient strings.

use crate::ir::style::{Color, ColorStop, Gradient, RadialShape};

/// Parse a linear or radial gradient from a string
///
/// # Formats
/// - `linear-gradient(<angle>, <color-stop>, <color-stop>, ...)`
/// - `radial-gradient(<shape>, <color-stop>, <color-stop>, ...)`
///
/// # Examples
/// ```rust
/// use gravity_core::parser::gradient::parse_gradient;
///
/// let grad = parse_gradient("linear-gradient(90deg, red, blue)").unwrap();
/// ```
pub fn parse_gradient(s: &str) -> Result<Gradient, String> {
    let s = s.trim();

    if s.starts_with("linear-gradient(") && s.ends_with(')') {
        parse_linear_gradient(s)
    } else if s.starts_with("radial-gradient(") && s.ends_with(')') {
        parse_radial_gradient(s)
    } else {
        Err(format!(
            "Invalid gradient format: '{}'. Expected linear-gradient(...) or radial-gradient(...)",
            s
        ))
    }
}

/// Parse linear gradient: linear-gradient(<angle>, <color-stop>, ...)
fn parse_linear_gradient(s: &str) -> Result<Gradient, String> {
    let inner = &s[16..s.len() - 1]; // Remove "linear-gradient(" and ")"
    let parts: Vec<&str> = inner.split(',').collect();

    if parts.len() < 2 {
        return Err("Linear gradient requires at least angle and one color stop".to_string());
    }

    let angle = parse_angle(parts[0])?;
    let stops = parse_color_stops(&parts[1..])?;

    Ok(Gradient::Linear { angle, stops })
}

/// Parse radial gradient: radial-gradient(<shape>, <color-stop>, ...)
fn parse_radial_gradient(s: &str) -> Result<Gradient, String> {
    let inner = &s[16..s.len() - 1]; // Remove "radial-gradient(" and ")"
    let parts: Vec<&str> = inner.split(',').collect();

    if parts.len() < 2 {
        return Err("Radial gradient requires at least shape and one color stop".to_string());
    }

    let shape = parse_shape(parts[0])?;
    let stops = parse_color_stops(&parts[1..])?;

    Ok(Gradient::Radial { shape, stops })
}

/// Parse angle: "90deg", "1.5rad", "0.25turn"
pub fn parse_angle(s: &str) -> Result<f32, String> {
    let s = s.trim();

    if let Some(num) = s.strip_suffix("deg") {
        let value: f32 = num
            .parse()
            .map_err(|_| format!("Invalid degree value: {}", s))?;
        Ok(value % 360.0)
    } else if let Some(num) = s.strip_suffix("rad") {
        let value: f32 = num
            .parse()
            .map_err(|_| format!("Invalid radian value: {}", s))?;
        Ok(value * 180.0 / std::f32::consts::PI)
    } else if let Some(num) = s.strip_suffix("turn") {
        let value: f32 = num
            .parse()
            .map_err(|_| format!("Invalid turn value: {}", s))?;
        Ok(value * 360.0)
    } else {
        // Try to parse as plain number (degrees)
        let value: f32 = s.parse().map_err(|_| format!("Invalid angle: {}", s))?;
        Ok(value)
    }
}

/// Parse radial shape: "circle" or "ellipse"
fn parse_shape(s: &str) -> Result<RadialShape, String> {
    match s.trim().to_lowercase().as_str() {
        "circle" => Ok(RadialShape::Circle),
        "ellipse" => Ok(RadialShape::Ellipse),
        _ => Err(format!(
            "Invalid radial shape: '{}'. Expected circle or ellipse",
            s
        )),
    }
}

/// Parse color stops: "red", "red 0%", "rgb(255,0,0) 50%"
pub fn parse_color_stops(parts: &[&str]) -> Result<Vec<ColorStop>, String> {
    let mut stops = Vec::new();

    for part in parts {
        let part = part.trim();
        let stop = parse_color_stop(part)?;
        stops.push(stop);
    }

    Ok(stops)
}

/// Parse a single color stop
pub fn parse_color_stop(s: &str) -> Result<ColorStop, String> {
    // Split by whitespace to separate color and optional offset
    let parts: Vec<&str> = s.split_whitespace().collect();

    if parts.is_empty() {
        return Err("Empty color stop".to_string());
    }

    let color_str = parts[0];
    let color = Color::parse(color_str)?;

    // Optional offset
    let offset = if parts.len() > 1 {
        let offset_str = parts[1];
        if let Some(num) = offset_str.strip_suffix('%') {
            let value: f32 = num
                .parse()
                .map_err(|_| format!("Invalid offset: {}", offset_str))?;
            value / 100.0
        } else {
            let value: f32 = offset_str
                .parse()
                .map_err(|_| format!("Invalid offset: {}", offset_str))?;
            value
        }
    } else {
        // No offset specified, will be determined by position
        // For now, we'll use 0.0 and caller should normalize
        0.0
    };

    Ok(ColorStop { color, offset })
}
