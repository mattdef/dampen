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

/// Parse a theme node from XML
pub fn parse_theme_from_node(
    node: roxmltree::Node,
    _source: &str,
) -> Result<Theme, crate::parser::error::ParseError> {
    use crate::parser::error::{ParseError, ParseErrorKind};

    let name = node
        .attribute("name")
        .map(|s| s.to_string())
        .unwrap_or_else(|| "default".to_string());

    let mut palette_attrs = HashMap::new();
    let mut typography_attrs = HashMap::new();
    let mut spacing_unit = None;

    // Parse child elements
    for child in node.children() {
        if child.node_type() != roxmltree::NodeType::Element {
            continue;
        }

        let tag = child.tag_name().name();

        if tag == "palette" {
            for attr in child.attributes() {
                palette_attrs.insert(attr.name().to_string(), attr.value().to_string());
            }
        } else if tag == "typography" {
            for attr in child.attributes() {
                typography_attrs.insert(attr.name().to_string(), attr.value().to_string());
            }
        } else if tag == "spacing" {
            if let Some(unit) = child.attribute("unit") {
                spacing_unit = unit.parse::<f32>().ok();
            }
        }
    }

    // Validate required palette colors
    let required_colors = ["primary", "secondary", "background", "text"];
    for color in &required_colors {
        if !palette_attrs.contains_key(*color) {
            return Err(ParseError {
                kind: ParseErrorKind::InvalidValue,
                message: format!("Theme palette missing required color: {}", color),
                span: crate::ir::Span::default(),
                suggestion: None,
            });
        }
    }

    // Parse using existing function
    let theme =
        parse_theme(name, &palette_attrs, &typography_attrs, spacing_unit).map_err(|e| {
            ParseError {
                kind: ParseErrorKind::InvalidValue,
                message: format!("Failed to parse theme: {}", e),
                span: crate::ir::Span::default(),
                suggestion: None,
            }
        })?;

    Ok(theme)
}

/// Parse a style class node from XML
pub fn parse_style_class_from_node(
    node: roxmltree::Node,
    _source: &str,
) -> Result<StyleClass, crate::parser::error::ParseError> {
    use crate::parser::error::{ParseError, ParseErrorKind};

    let name = node
        .attribute("name")
        .map(|s| s.to_string())
        .unwrap_or_default();

    if name.is_empty() {
        return Err(ParseError {
            kind: ParseErrorKind::InvalidValue,
            message: "Style class must have a name".to_string(),
            span: crate::ir::Span::default(),
            suggestion: None,
        });
    }

    // Collect all attributes
    let mut base_attrs = HashMap::new();
    let mut extends = Vec::new();
    let mut state_variants_raw: HashMap<WidgetState, HashMap<String, String>> = HashMap::new();
    let mut layout = None;

    for attr in node.attributes() {
        let key = attr.name();
        let value = attr.value();

        // Check for extends
        if key == "extends" {
            extends = value.split_whitespace().map(|s| s.to_string()).collect();
            continue;
        }

        // Check for state variants
        if let Some((prefix, attr_name)) = split_state_prefix(key) {
            let state = WidgetState::from_prefix(prefix).ok_or_else(|| ParseError {
                kind: ParseErrorKind::InvalidValue,
                message: format!("Invalid state prefix: {}", prefix),
                span: crate::ir::Span::default(),
                suggestion: None,
            })?;

            let state_attr = state_variants_raw.entry(state).or_default();
            state_attr.insert(attr_name.to_string(), value.to_string());
            continue;
        }

        // Check for layout attributes
        let layout_attr_names = [
            "width",
            "height",
            "min_width",
            "max_width",
            "min_height",
            "max_height",
            "padding",
            "spacing",
            "align_items",
            "justify_content",
            "align_self",
            "direction",
        ];

        if layout_attr_names.contains(&key) {
            base_attrs.insert(key.to_string(), value.to_string());
            continue;
        }

        // Regular style attribute
        base_attrs.insert(key.to_string(), value.to_string());
    }

    // Parse layout if any layout attributes present
    if base_attrs.keys().any(|k| {
        matches!(
            k.as_str(),
            "width"
                | "height"
                | "min_width"
                | "max_width"
                | "min_height"
                | "max_height"
                | "padding"
                | "spacing"
                | "align_items"
                | "justify_content"
                | "align_self"
                | "direction"
        )
    }) {
        layout = parse_layout_constraints(&base_attrs).map_err(|e| ParseError {
            kind: ParseErrorKind::InvalidValue,
            message: format!("Failed to parse layout: {}", e),
            span: crate::ir::Span::default(),
            suggestion: None,
        })?;

        // Remove layout attributes from base_attrs
        let layout_keys: Vec<String> = base_attrs
            .keys()
            .filter(|k| {
                matches!(
                    k.as_str(),
                    "width"
                        | "height"
                        | "min_width"
                        | "max_width"
                        | "min_height"
                        | "max_height"
                        | "padding"
                        | "spacing"
                        | "align_items"
                        | "justify_content"
                        | "align_self"
                        | "direction"
                )
            })
            .cloned()
            .collect();

        for key in layout_keys {
            base_attrs.remove(&key);
        }
    }

    // Parse state variants into StyleProperties
    let mut state_variants = HashMap::new();
    for (state, state_attrs) in state_variants_raw {
        let style = parse_style_properties_from_attrs(&state_attrs).map_err(|e| ParseError {
            kind: ParseErrorKind::InvalidValue,
            message: format!("Failed to parse state variant for {:?}: {}", state, e),
            span: crate::ir::Span::default(),
            suggestion: None,
        })?;
        state_variants.insert(state, style);
    }

    // Parse using existing function
    let class =
        parse_style_class(name, &base_attrs, extends, state_variants, layout).map_err(|e| {
            ParseError {
                kind: ParseErrorKind::InvalidValue,
                message: format!("Failed to parse style class: {}", e),
                span: crate::ir::Span::default(),
                suggestion: None,
            }
        })?;

    Ok(class)
}
