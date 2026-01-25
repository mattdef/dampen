//! Canvas and shape parsing logic.

use crate::ir::{AttributeValue, Span, WidgetKind, WidgetNode};
use crate::parser::error::{ParseError, ParseErrorKind};
use std::collections::HashMap;

/// Validate that a widget kind is a valid canvas shape.
pub fn is_canvas_shape(kind: &WidgetKind) -> bool {
    matches!(
        kind,
        WidgetKind::CanvasRect
            | WidgetKind::CanvasCircle
            | WidgetKind::CanvasLine
            | WidgetKind::CanvasText
            | WidgetKind::CanvasGroup
            | WidgetKind::For // Control flow allowed
            | WidgetKind::If // Control flow allowed
    )
}

/// Validate children of a Canvas widget.
///
/// Canvas children must be shapes or control flow elements.
pub fn validate_canvas_children(children: &[WidgetNode], _span: Span) -> Result<(), ParseError> {
    for child in children {
        if !is_canvas_shape(&child.kind) {
            return Err(ParseError {
                kind: ParseErrorKind::InvalidChild,
                message: format!(
                    "Invalid child for Canvas: {:?}. Only shapes (rect, circle, line, text, group) are allowed.",
                    child.kind
                ),
                span: child.span,
                suggestion: Some("Remove this widget or move it outside the <canvas>".to_string()),
            });
        }

        // If child is a group, recursively validate its children
        if child.kind == WidgetKind::CanvasGroup {
            validate_canvas_children(&child.children, child.span)?;
        }
    }
    Ok(())
}

/// Parse and validate a transform string.
///
/// Supported formats:
/// - translate(x, y)
/// - rotate(angle)
/// - scale(factor)
/// - scale(x, y)
/// - matrix(a, b, c, d, e, f)
pub fn parse_transform(value: &str) -> Result<(), String> {
    let value = value.trim();
    if value.starts_with("translate(") && value.ends_with(')') {
        let args = parse_args(&value[10..value.len() - 1])?;
        if args.len() != 2 {
            return Err("translate() requires 2 arguments (x, y)".to_string());
        }
    } else if value.starts_with("rotate(") && value.ends_with(')') {
        let args = parse_args(&value[7..value.len() - 1])?;
        if args.len() != 1 {
            return Err("rotate() requires 1 argument (angle)".to_string());
        }
    } else if value.starts_with("scale(") && value.ends_with(')') {
        let args = parse_args(&value[6..value.len() - 1])?;
        if args.len() != 1 && args.len() != 2 {
            return Err("scale() requires 1 or 2 arguments".to_string());
        }
    } else if value.starts_with("matrix(") && value.ends_with(')') {
        let args = parse_args(&value[7..value.len() - 1])?;
        if args.len() != 6 {
            return Err("matrix() requires 6 arguments".to_string());
        }
    } else {
        return Err(
            "Unknown transform function. Supported: translate, rotate, scale, matrix".to_string(),
        );
    }
    Ok(())
}

fn parse_args(args: &str) -> Result<Vec<f32>, String> {
    args.split(',')
        .map(|s| {
            s.trim()
                .parse::<f32>()
                .map_err(|_| format!("Invalid number: {}", s))
        })
        .collect()
}

/// Validate shape attributes.
pub fn validate_shape_attributes(
    kind: &WidgetKind,
    attributes: &HashMap<String, AttributeValue>,
    span: Span,
) -> Result<(), ParseError> {
    // Validate transform if present (for Group)
    if let Some(AttributeValue::Static(t)) = attributes.get("transform")
        && let Err(e) = parse_transform(t)
    {
        return Err(ParseError {
            kind: ParseErrorKind::InvalidValue,
            message: format!("Invalid transform: {}", e),
            span,
            suggestion: Some(
                "Check transform syntax: translate(x, y), rotate(rad), scale(f)".to_string(),
            ),
        });
    }

    // T088: Validate numeric attributes for non-negative values
    if let Some(AttributeValue::Static(w_str)) = attributes.get("width")
        && let Ok(w) = w_str.parse::<f32>()
        && w < 0.0
    {
        return Err(ParseError {
            kind: ParseErrorKind::InvalidValue,
            message: format!("Width for {:?} cannot be negative: {}", kind, w),
            span,
            suggestion: Some("Use a positive value for width".to_string()),
        });
    }

    if let Some(AttributeValue::Static(h_str)) = attributes.get("height")
        && let Ok(h) = h_str.parse::<f32>()
        && h < 0.0
    {
        return Err(ParseError {
            kind: ParseErrorKind::InvalidValue,
            message: format!("Height for {:?} cannot be negative: {}", kind, h),
            span,
            suggestion: Some("Use a positive value for height".to_string()),
        });
    }

    if let Some(AttributeValue::Static(r_str)) = attributes.get("radius")
        && let Ok(r) = r_str.parse::<f32>()
        && r < 0.0
    {
        return Err(ParseError {
            kind: ParseErrorKind::InvalidValue,
            message: format!("Radius for {:?} cannot be negative: {}", kind, r),
            span,
            suggestion: Some("Use a positive value for radius".to_string()),
        });
    }

    Ok(())
}
