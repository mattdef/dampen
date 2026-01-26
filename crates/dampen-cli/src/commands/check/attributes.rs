use dampen_core::ir::WidgetKind;
use std::collections::HashSet;

/// Schema defining valid attributes for a widget type.
///
/// This is a wrapper around `dampen_core::schema::WidgetSchema` to maintain
/// backward compatibility with existing CLI tests and logic.
#[derive(Debug, Clone)]
pub struct WidgetAttributeSchema {
    pub required: HashSet<&'static str>,
    pub optional: HashSet<&'static str>,
    pub events: HashSet<&'static str>,
    pub style_attributes: HashSet<&'static str>,
    pub layout_attributes: HashSet<&'static str>,
}

impl WidgetAttributeSchema {
    /// Returns the schema for a specific widget kind.
    pub fn for_widget(kind: &WidgetKind) -> Self {
        let core_schema = kind.schema();

        // Helper to convert slice to HashSet
        let to_set = |slice: &'static [&'static str]| -> HashSet<&'static str> {
            slice.iter().copied().collect()
        };

        Self {
            required: to_set(core_schema.required),
            optional: to_set(core_schema.optional),
            events: to_set(core_schema.events),
            style_attributes: to_set(core_schema.style_attributes),
            layout_attributes: to_set(core_schema.layout_attributes),
        }
    }

    /// Returns all valid attributes (required + optional + events + style + layout).
    pub fn all_valid(&self) -> HashSet<&'static str> {
        let mut all = HashSet::new();
        all.extend(self.required.iter().copied());
        all.extend(self.optional.iter().copied());
        all.extend(self.events.iter().copied());
        all.extend(self.style_attributes.iter().copied());
        all.extend(self.layout_attributes.iter().copied());
        all
    }

    /// Returns a list of all valid attribute names as a vector.
    pub fn all_valid_names(&self) -> Vec<&'static str> {
        self.all_valid().into_iter().collect()
    }
}

/// Validates widget attributes and detects unknown attributes.
///
/// Returns a list of unknown attributes with suggestions.
pub fn validate_widget_attributes(
    widget_kind: &WidgetKind,
    attributes: &[String],
) -> Vec<(String, Option<String>)> {
    use crate::commands::check::suggestions;

    let schema = WidgetAttributeSchema::for_widget(widget_kind);
    let valid_attrs = schema.all_valid();
    let valid_names = schema.all_valid_names();

    let mut unknown_attrs = Vec::new();

    for attr in attributes {
        if !valid_attrs.contains(attr.as_str()) {
            // Generate suggestion using Levenshtein distance
            let suggestion = suggestions::find_closest_match(attr, &valid_names, 3)
                .map(|(matched, _)| matched.to_string());

            unknown_attrs.push((attr.clone(), suggestion));
        }
    }

    unknown_attrs
}

/// Checks if an attribute is valid for a widget.
pub fn is_valid_attribute(widget_kind: &WidgetKind, attribute: &str) -> bool {
    let schema = WidgetAttributeSchema::for_widget(widget_kind);
    schema.all_valid().contains(attribute)
}

/// Validates that all required attributes are present for a widget.
///
/// Returns a list of missing required attributes.
pub fn validate_required_attributes(
    widget_kind: &WidgetKind,
    attributes: &[String],
) -> Vec<String> {
    let schema = WidgetAttributeSchema::for_widget(widget_kind);

    // Find all required attributes that are not present in the provided attributes
    schema
        .required
        .iter()
        .filter(|&&req| !attributes.iter().any(|attr| attr == req))
        .map(|&s| s.to_string())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_widget_schema() {
        let schema = WidgetAttributeSchema::for_widget(&WidgetKind::Text);
        assert!(schema.required.contains("value"));
        assert!(schema.optional.contains("size"));
        assert!(!schema.required.contains("size"));
    }

    #[test]
    fn test_image_widget_schema() {
        let schema = WidgetAttributeSchema::for_widget(&WidgetKind::Image);
        assert!(schema.required.contains("src"));
        assert!(schema.optional.contains("width"));
    }

    #[test]
    fn test_button_widget_schema() {
        let schema = WidgetAttributeSchema::for_widget(&WidgetKind::Button);
        assert!(schema.events.contains("on_click"));
        assert!(schema.optional.contains("label"));
    }

    #[test]
    fn test_radio_widget_schema() {
        let schema = WidgetAttributeSchema::for_widget(&WidgetKind::Radio);
        assert!(schema.required.contains("label"));
        assert!(schema.required.contains("value"));
        assert!(schema.optional.contains("selected"));
        assert!(schema.events.contains("on_select"));
    }

    #[test]
    fn test_all_valid_includes_all_categories() {
        let schema = WidgetAttributeSchema::for_widget(&WidgetKind::Text);
        let all = schema.all_valid();

        // Should include required
        assert!(all.contains("value"));

        // Should include optional
        assert!(all.contains("size"));

        // Should include style
        assert!(all.contains("background"));

        // Should include layout
        assert!(all.contains("width"));

        // Should include events
        assert!(all.contains("on_click"));
    }

    #[test]
    fn test_all_valid_names_returns_vec() {
        let schema = WidgetAttributeSchema::for_widget(&WidgetKind::Button);
        let names = schema.all_valid_names();

        assert!(!names.is_empty());
        assert!(names.contains(&"on_click"));
    }

    #[test]
    fn test_validate_widget_attributes_valid() {
        let attrs = vec!["on_click".to_string(), "label".to_string()];
        let unknown = validate_widget_attributes(&WidgetKind::Button, &attrs);
        assert!(unknown.is_empty());
    }

    #[test]
    fn test_validate_widget_attributes_unknown() {
        let attrs = vec!["on_clik".to_string(), "unknown".to_string()];
        let unknown = validate_widget_attributes(&WidgetKind::Button, &attrs);
        assert_eq!(unknown.len(), 2);

        // First should have suggestion for "on_click"
        assert_eq!(unknown[0].0, "on_clik");
        assert!(unknown[0].1.is_some());
        assert_eq!(unknown[0].1.as_ref().unwrap(), "on_click");

        // Second might not have a good suggestion
        assert_eq!(unknown[1].0, "unknown");
    }

    #[test]
    fn test_is_valid_attribute() {
        assert!(is_valid_attribute(&WidgetKind::Button, "on_click"));
        assert!(is_valid_attribute(&WidgetKind::Button, "label"));
        assert!(!is_valid_attribute(&WidgetKind::Button, "on_clik"));
    }

    #[test]
    fn test_validate_required_attributes_all_present() {
        let attrs = vec!["value".to_string(), "size".to_string()];
        let missing = validate_required_attributes(&WidgetKind::Text, &attrs);
        assert!(missing.is_empty());
    }

    #[test]
    fn test_validate_required_attributes_missing_value() {
        let attrs = vec!["size".to_string(), "color".to_string()];
        let missing = validate_required_attributes(&WidgetKind::Text, &attrs);
        assert_eq!(missing.len(), 1);
        assert_eq!(missing[0], "value");
    }

    #[test]
    fn test_validate_required_attributes_image_missing_src() {
        let attrs = vec!["width".to_string(), "height".to_string()];
        let missing = validate_required_attributes(&WidgetKind::Image, &attrs);
        assert_eq!(missing.len(), 1);
        assert_eq!(missing[0], "src");
    }

    #[test]
    fn test_validate_required_attributes_radio_missing_both() {
        let attrs = vec!["selected".to_string()];
        let missing = validate_required_attributes(&WidgetKind::Radio, &attrs);
        assert_eq!(missing.len(), 2);
        assert!(missing.contains(&"label".to_string()));
        assert!(missing.contains(&"value".to_string()));
    }

    #[test]
    fn test_validate_required_attributes_button_no_required() {
        let attrs = vec!["on_click".to_string()];
        let missing = validate_required_attributes(&WidgetKind::Button, &attrs);
        assert!(missing.is_empty());
    }
}
