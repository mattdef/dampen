use gravity_cli::commands::check::{attributes::WidgetAttributeSchema, suggestions};
use gravity_core::ir::WidgetKind;

#[test]
fn test_button_unknown_attribute() {
    let schema = WidgetAttributeSchema::for_widget(&WidgetKind::Button);
    let all_valid = schema.all_valid();

    // Valid attribute should be in the set
    assert!(all_valid.contains("on_click"));
    assert!(all_valid.contains("label"));

    // Invalid attribute should not be in the set
    assert!(!all_valid.contains("on_clik")); // typo
    assert!(!all_valid.contains("unknown_attr"));
}

#[test]
fn test_button_unknown_attribute_with_suggestion() {
    let schema = WidgetAttributeSchema::for_widget(&WidgetKind::Button);
    let all_valid_names = schema.all_valid_names();

    // Test suggestion for typo
    let suggestion = suggestions::suggest("on_clik", &all_valid_names, 3);
    assert!(suggestion.contains("on_click"));
    assert!(suggestion.contains("distance: 1"));
}

#[test]
fn test_button_valid_attributes() {
    let schema = WidgetAttributeSchema::for_widget(&WidgetKind::Button);
    let all_valid = schema.all_valid();

    // Common event attributes
    assert!(all_valid.contains("on_click"));
    assert!(all_valid.contains("on_press"));
    assert!(all_valid.contains("on_release"));

    // Optional attributes
    assert!(all_valid.contains("label"));

    // Common style attributes
    assert!(all_valid.contains("background"));
    assert!(all_valid.contains("color"));

    // Common layout attributes
    assert!(all_valid.contains("width"));
    assert!(all_valid.contains("height"));
    assert!(all_valid.contains("padding"));
}

#[test]
fn test_detect_multiple_unknown_attributes() {
    let schema = WidgetAttributeSchema::for_widget(&WidgetKind::Button);
    let all_valid = schema.all_valid();

    let test_attrs = vec!["on_click", "on_clik", "label", "labell", "unknown"];
    let mut unknown_attrs = Vec::new();

    for attr in test_attrs {
        if !all_valid.contains(attr) {
            unknown_attrs.push(attr);
        }
    }

    assert_eq!(unknown_attrs.len(), 3);
    assert!(unknown_attrs.contains(&"on_clik"));
    assert!(unknown_attrs.contains(&"labell"));
    assert!(unknown_attrs.contains(&"unknown"));
}

// T012: Tests for TextInput widget
#[test]
fn test_textinput_unknown_attribute() {
    let schema = WidgetAttributeSchema::for_widget(&WidgetKind::TextInput);
    let all_valid = schema.all_valid();

    // Valid attributes
    assert!(all_valid.contains("placeholder"));
    assert!(all_valid.contains("value"));
    assert!(all_valid.contains("on_input"));
    assert!(all_valid.contains("on_submit"));

    // Invalid attributes
    assert!(!all_valid.contains("placeholdr")); // typo
    assert!(!all_valid.contains("on_submitt")); // typo
}

#[test]
fn test_textinput_unknown_attribute_with_suggestion() {
    let schema = WidgetAttributeSchema::for_widget(&WidgetKind::TextInput);
    let all_valid_names = schema.all_valid_names();

    // Test suggestion for typo in 'placeholder'
    let suggestion = suggestions::suggest("placeholdr", &all_valid_names, 3);
    assert!(suggestion.contains("placeholder"));

    // Test suggestion for typo in 'on_submit'
    let suggestion = suggestions::suggest("on_submitt", &all_valid_names, 3);
    assert!(suggestion.contains("on_submit"));
}

#[test]
fn test_textinput_valid_attributes() {
    let schema = WidgetAttributeSchema::for_widget(&WidgetKind::TextInput);
    let all_valid = schema.all_valid();

    // Widget-specific attributes
    assert!(all_valid.contains("placeholder"));
    assert!(all_valid.contains("value"));
    assert!(all_valid.contains("secure"));

    // Event attributes
    assert!(all_valid.contains("on_input"));
    assert!(all_valid.contains("on_submit"));
    assert!(all_valid.contains("on_change"));

    // Common layout attributes
    assert!(all_valid.contains("width"));
    assert!(all_valid.contains("padding"));
}
