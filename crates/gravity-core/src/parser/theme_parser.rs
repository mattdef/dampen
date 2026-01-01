//! Theme and style class parsing
//!
//! This module provides parsers for theme definitions and style classes.

use crate::ir::layout::LayoutConstraints;
use crate::ir::style::{Color, StyleProperties};
use crate::ir::theme::{
    FontWeight, SpacingScale, StyleClass, Theme, ThemePalette, Typography, WidgetState,
};
use std::collections::HashMap;

/// Parse a theme definition from XML attributes
pub fn parse_theme(
    name: String,
    palette_attrs: &HashMap<String, String>,
    typography_attrs: &HashMap<String, String>,
    spacing_unit: Option<f32>,
) -> Result<Theme, String> {
    let palette = parse_palette(palette_attrs)?;
    let typography = parse_typography(typography_attrs)?;
    let spacing = SpacingScale {
        unit: spacing_unit.unwrap_or(4.0),
    };

    let theme = Theme {
        name,
        palette,
        typography,
        spacing,
        base_styles: HashMap::new(),
    };

    theme.validate()?;
    Ok(theme)
}

/// Parse theme palette from attributes
pub fn parse_palette(attrs: &HashMap<String, String>) -> Result<ThemePalette, String> {
    let get_color = |key: &str| -> Result<Color, String> {
        let value = attrs
            .get(key)
            .ok_or_else(|| format!("Missing required palette color: {}", key))?;
        Color::parse(value)
    };

    Ok(ThemePalette {
        primary: get_color("primary")?,
        secondary: get_color("secondary")?,
        success: get_color("success")?,
        warning: get_color("warning")?,
        danger: get_color("danger")?,
        background: get_color("background")?,
        surface: get_color("surface")?,
        text: get_color("text")?,
        text_secondary: get_color("text_secondary")?,
    })
}

/// Parse typography from attributes
pub fn parse_typography(attrs: &HashMap<String, String>) -> Result<Typography, String> {
    let font_family = attrs
        .get("font_family")
        .cloned()
        .unwrap_or_else(|| "sans-serif".to_string());

    let font_size_base: f32 = attrs
        .get("font_size_base")
        .unwrap_or(&"16.0".to_string())
        .parse()
        .map_err(|_| "Invalid font_size_base")?;

    let font_size_small: f32 = attrs
        .get("font_size_small")
        .unwrap_or(&"12.0".to_string())
        .parse()
        .map_err(|_| "Invalid font_size_small")?;

    let font_size_large: f32 = attrs
        .get("font_size_large")
        .unwrap_or(&"20.0".to_string())
        .parse()
        .map_err(|_| "Invalid font_size_large")?;

    let font_weight = match attrs.get("font_weight") {
        Some(w) => FontWeight::parse(w)?,
        None => FontWeight::Normal,
    };

    let line_height: f32 = attrs
        .get("line_height")
        .unwrap_or(&"1.5".to_string())
        .parse()
        .map_err(|_| "Invalid line_height")?;

    Ok(Typography {
        font_family,
        font_size_base,
        font_size_small,
        font_size_large,
        font_weight,
        line_height,
    })
}

/// Parse a style class definition
pub fn parse_style_class(
    name: String,
    base_attrs: &HashMap<String, String>,
    extends: Vec<String>,
    state_variants: HashMap<WidgetState, StyleProperties>,
    layout: Option<LayoutConstraints>,
) -> Result<StyleClass, String> {
    let style = parse_style_properties_from_attrs(base_attrs)?;

    let class = StyleClass {
        name,
        style,
        layout,
        extends,
        state_variants,
    };

    Ok(class)
}

/// Parse style properties from a map of attributes
pub fn parse_style_properties_from_attrs(
    attrs: &HashMap<String, String>,
) -> Result<StyleProperties, String> {
    use crate::parser::style_parser::*;

    let mut background = None;
    let mut color = None;
    let mut shadow = None;
    let mut opacity = None;
    let mut transform = None;

    // Parse background
    if let Some(value) = attrs.get("background") {
        background = Some(parse_background_attr(value)?);
    }

    // Parse color
    if let Some(value) = attrs.get("color") {
        color = Some(parse_color_attr(value)?);
    }

    // Parse border properties
    let border_width = attrs
        .get("border_width")
        .map(|v| parse_border_width(v))
        .transpose()?;
    let border_color = attrs
        .get("border_color")
        .map(|v| parse_border_color(v))
        .transpose()?;
    let border_radius = attrs
        .get("border_radius")
        .map(|v| parse_border_radius(v))
        .transpose()?;
    let border_style = attrs
        .get("border_style")
        .map(|v| parse_border_style(v))
        .transpose()?;

    let border = build_border(border_width, border_color, border_radius, border_style)?;

    // Parse shadow
    if let Some(value) = attrs.get("shadow") {
        shadow = Some(parse_shadow_attr(value)?);
    }

    // Parse opacity
    if let Some(value) = attrs.get("opacity") {
        opacity = Some(parse_opacity(value)?);
    }

    // Parse transform
    if let Some(value) = attrs.get("transform") {
        transform = Some(parse_transform(value)?);
    }

    build_style_properties(background, color, border, shadow, opacity, transform)
}

/// Parse layout constraints from attributes
pub fn parse_layout_constraints(
    attrs: &HashMap<String, String>,
) -> Result<Option<LayoutConstraints>, String> {
    use crate::parser::style_parser::*;

    let mut constraints = LayoutConstraints::default();
    let mut has_any = false;

    // Parse sizing
    if let Some(value) = attrs.get("width") {
        constraints.width = Some(parse_length_attr(value)?);
        has_any = true;
    }

    if let Some(value) = attrs.get("height") {
        constraints.height = Some(parse_length_attr(value)?);
        has_any = true;
    }

    // Parse constraints
    if let Some(value) = attrs.get("min_width") {
        constraints.min_width = Some(parse_constraint(value)?);
        has_any = true;
    }

    if let Some(value) = attrs.get("max_width") {
        constraints.max_width = Some(parse_constraint(value)?);
        has_any = true;
    }

    if let Some(value) = attrs.get("min_height") {
        constraints.min_height = Some(parse_constraint(value)?);
        has_any = true;
    }

    if let Some(value) = attrs.get("max_height") {
        constraints.max_height = Some(parse_constraint(value)?);
        has_any = true;
    }

    // Parse layout
    if let Some(value) = attrs.get("padding") {
        constraints.padding = Some(parse_padding_attr(value)?);
        has_any = true;
    }

    if let Some(value) = attrs.get("spacing") {
        constraints.spacing = Some(parse_spacing(value)?);
        has_any = true;
    }

    // Parse alignment
    if let Some(value) = attrs.get("align_items") {
        constraints.align_items = Some(parse_alignment(value)?);
        has_any = true;
    }

    if let Some(value) = attrs.get("justify_content") {
        constraints.justify_content = Some(parse_justification(value)?);
        has_any = true;
    }

    if let Some(value) = attrs.get("align_self") {
        constraints.align_self = Some(parse_alignment(value)?);
        has_any = true;
    }

    // Parse direction
    if let Some(value) = attrs.get("direction") {
        constraints.direction = Some(crate::ir::layout::Direction::parse(value)?);
        has_any = true;
    }

    if has_any {
        constraints.validate()?;
        Ok(Some(constraints))
    } else {
        Ok(None)
    }
}

/// Parse state-prefixed attributes into state variants
pub fn parse_state_variants(
    attrs: &HashMap<String, String>,
) -> Result<HashMap<WidgetState, StyleProperties>, String> {
    let mut variants: HashMap<WidgetState, HashMap<String, String>> = HashMap::new();

    for (key, value) in attrs {
        // Check if key has state prefix
        if let Some((prefix, attr_name)) = split_state_prefix(key) {
            let state = WidgetState::from_prefix(prefix)
                .ok_or_else(|| format!("Invalid state prefix: {}", prefix))?;

            variants
                .entry(state)
                .or_default()
                .insert(attr_name.to_string(), value.to_string());
        }
    }

    // Parse each state's properties
    let mut result = HashMap::new();
    for (state, state_attrs) in variants {
        let style = parse_style_properties_from_attrs(&state_attrs)?;
        result.insert(state, style);
    }

    Ok(result)
}

/// Split a state-prefixed attribute name
/// e.g., "hover:background" -> Some(("hover", "background"))
fn split_state_prefix(key: &str) -> Option<(&str, &str)> {
    // Handle combined states: "hover:active:background"
    if let Some(idx) = key.find(':') {
        let prefix = &key[..idx];
        let rest = &key[idx + 1..];

        // Check if rest also has state prefix (combined states)
        if rest.contains(':') {
            // For now, we only support single state prefixes
            // Combined states like "hover:active:background" would need recursive parsing
            return None;
        }

        return Some((prefix, rest));
    }
    None
}
