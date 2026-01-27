//! Attribute standardization and validation for unified naming across widgets.
//!
//! This module enforces the standard attribute contract to ensure parity between
//! Interpreted and Codegen modes.

use crate::ir::Span;
use crate::ir::WidgetKind;
use crate::parser::error::{ParseError, ParseErrorKind};
use std::collections::HashMap;

/// Deprecated attributes that should be warned about or migrated.
const DEPRECATED_ATTRIBUTES: &[(&str, &str, &str)] = &[
    // (old_name, new_name, widget_applicable)
    ("path", "src", "image,svg"),
    ("active", "toggled", "toggler"),
    ("is_toggled", "toggled", "toggler"),
    ("secure", "password", "text_input"),
];

/// Validate and normalize attributes for a widget node.
///
/// # Arguments
///
/// * `widget_kind` - The type of widget being validated
/// * `attributes` - The attributes map to validate
/// * `span` - Source location for error reporting
///
/// # Returns
///
/// Returns `Ok(())` if attributes are valid, or `Err(ParseError)` with
/// suggestions for deprecated or invalid attributes.
pub fn validate_attributes(
    widget_kind: &WidgetKind,
    attributes: &HashMap<String, crate::ir::AttributeValue>,
    span: Span,
) -> Result<(), ParseError> {
    // Check for deprecated attributes and suggest alternatives
    for (old_name, new_name, applicable_widgets) in DEPRECATED_ATTRIBUTES {
        if attributes.contains_key(*old_name) {
            let widget_name = widget_kind.to_string();

            // Check if this deprecation applies to this widget
            if applicable_widgets
                .split(',')
                .any(|w| w == widget_name.as_str())
            {
                return Err(ParseError {
                    kind: ParseErrorKind::DeprecatedAttribute,
                    message: format!(
                        "Attribute '{}' is deprecated for {} widgets",
                        old_name, widget_name
                    ),
                    span,
                    suggestion: Some(format!("Use '{}' instead: {}=\"...\"", new_name, new_name)),
                });
            }
        }
    }

    Ok(())
}

/// Normalize deprecated attribute names to their standard equivalents.
///
/// This function should be called early in parsing to automatically migrate
/// deprecated attributes without breaking existing files.
///
/// # Arguments
///
/// * `widget_kind` - The type of widget
/// * `attributes` - Mutable reference to the attributes map
///
/// # Returns
///
/// Returns a vector of warnings (old_name, new_name) for deprecated attributes
/// that were automatically migrated.
pub fn normalize_attributes(
    widget_kind: &WidgetKind,
    attributes: &mut HashMap<String, crate::ir::AttributeValue>,
) -> Vec<(String, String)> {
    let mut warnings = Vec::new();
    let widget_name = widget_kind.to_string();

    for (old_name, new_name, applicable_widgets) in DEPRECATED_ATTRIBUTES {
        if attributes.contains_key(*old_name) {
            // Check if this applies to this widget
            if applicable_widgets
                .split(',')
                .any(|w| w == widget_name.as_str())
            {
                // Move the value from old_name to new_name
                if let Some(value) = attributes.remove(*old_name) {
                    attributes.insert(new_name.to_string(), value);
                    warnings.push((old_name.to_string(), new_name.to_string()));
                }
            }
        }
    }

    warnings
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::AttributeValue;

    #[test]
    fn test_normalize_image_path_to_src() {
        let mut attributes = HashMap::new();
        attributes.insert(
            "path".to_string(),
            AttributeValue::Static("icon.png".to_string()),
        );

        let warnings = normalize_attributes(&WidgetKind::Image, &mut attributes);

        assert_eq!(warnings.len(), 1);
        assert_eq!(warnings[0], ("path".to_string(), "src".to_string()));
        assert!(attributes.contains_key("src"));
        assert!(!attributes.contains_key("path"));
    }

    #[test]
    fn test_normalize_toggler_active_to_toggled() {
        let mut attributes = HashMap::new();
        attributes.insert(
            "active".to_string(),
            AttributeValue::Static("true".to_string()),
        );

        let warnings = normalize_attributes(&WidgetKind::Toggler, &mut attributes);

        assert_eq!(warnings.len(), 1);
        assert_eq!(warnings[0], ("active".to_string(), "toggled".to_string()));
        assert!(attributes.contains_key("toggled"));
        assert!(!attributes.contains_key("active"));
    }

    #[test]
    fn test_normalize_textinput_secure_to_password() {
        let mut attributes = HashMap::new();
        attributes.insert(
            "secure".to_string(),
            AttributeValue::Static("true".to_string()),
        );

        let warnings = normalize_attributes(&WidgetKind::TextInput, &mut attributes);

        assert_eq!(warnings.len(), 1);
        assert_eq!(warnings[0], ("secure".to_string(), "password".to_string()));
        assert!(attributes.contains_key("password"));
        assert!(!attributes.contains_key("secure"));
    }

    #[test]
    fn test_normalize_no_change_for_standard_attributes() {
        let mut attributes = HashMap::new();
        attributes.insert(
            "src".to_string(),
            AttributeValue::Static("icon.png".to_string()),
        );

        let warnings = normalize_attributes(&WidgetKind::Image, &mut attributes);

        assert_eq!(warnings.len(), 0);
        assert!(attributes.contains_key("src"));
    }

    #[test]
    fn test_normalize_ignores_deprecated_on_wrong_widget() {
        let mut attributes = HashMap::new();
        attributes.insert(
            "path".to_string(),
            AttributeValue::Static("some_path".to_string()),
        );

        // "path" is only deprecated for Image/Svg, not Button
        let warnings = normalize_attributes(&WidgetKind::Button, &mut attributes);

        assert_eq!(warnings.len(), 0);
        assert!(attributes.contains_key("path")); // Not normalized
    }
}
