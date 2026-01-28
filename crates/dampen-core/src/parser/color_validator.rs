use crate::ir::Span;
use crate::parser::error::{ParseError, ParseErrorKind};

/// Validate color format strings in widget attributes
///
/// Supports #rgb, #rgba, #rrggbb, #rrggbbaa, and basic functional syntax rgb(), rgba().
///
/// # Arguments
///
/// * `value` - The raw color string to validate
/// * `span` - Source location for error reporting
///
/// # Returns
///
/// `Ok(())` if the format is valid, or `Err(ParseError)` otherwise.
pub fn validate_color_format(value: &str, span: Span) -> Result<(), ParseError> {
    // If it's a binding, we can't validate at compile time
    if value.starts_with('{') && value.ends_with('}') {
        return Ok(());
    }

    // Check if it's a hex color
    if let Some(hex) = value.strip_prefix('#') {
        match hex.len() {
            3 | 4 | 6 | 8 => {
                if hex.chars().all(|c| c.is_ascii_hexdigit()) {
                    return Ok(());
                }
            }
            _ => {}
        }
        return Err(ParseError {
            kind: ParseErrorKind::InvalidValue,
            message: format!("Invalid hex color format: {}", value),
            span,
            suggestion: Some("Use format: #rgb, #rgba, #rrggbb, or #rrggbbaa".to_string()),
        });
    }

    // Check for rgb/rgba function syntax
    if value.starts_with("rgb(") || value.starts_with("rgba(") {
        if !value.ends_with(')') {
            return Err(ParseError {
                kind: ParseErrorKind::InvalidValue,
                message: "Unclosed color function".to_string(),
                span,
                suggestion: Some("Ensure color function ends with ')'".to_string()),
            });
        }
        // Basic syntax check passes, csscolorparser will handle details
        return Ok(());
    }

    // Assume named color or other valid format handled by parser
    // We could add a list of valid named colors here if stricter validation is needed

    Ok(())
}
