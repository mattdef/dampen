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

// T050: Unit test for missing required attribute on Text widget
#[test]
fn test_text_missing_required_value_attribute() {
    let schema = WidgetAttributeSchema::for_widget(&WidgetKind::Text);

    // Text widget requires 'value' attribute
    assert!(schema.required.contains("value"));

    // Test that an attribute set without 'value' is missing required
    let test_attrs = vec!["size", "color", "width"];
    let missing_required: Vec<_> = schema
        .required
        .iter()
        .filter(|&&req| !test_attrs.contains(&req))
        .collect();

    assert_eq!(missing_required.len(), 1);
    assert_eq!(*missing_required[0], "value");
}

#[test]
fn test_text_with_all_required_attributes() {
    let schema = WidgetAttributeSchema::for_widget(&WidgetKind::Text);

    // Test that an attribute set with 'value' has all required
    let test_attrs = vec!["value", "size", "color"];
    let missing_required: Vec<_> = schema
        .required
        .iter()
        .filter(|&&req| !test_attrs.contains(&req))
        .collect();

    assert_eq!(missing_required.len(), 0);
}

// T052: Unit test for missing required attribute on Radio widget
#[test]
fn test_radio_missing_required_label_attribute() {
    let schema = WidgetAttributeSchema::for_widget(&WidgetKind::Radio);

    // Radio widget requires 'label' and 'value' attributes
    assert!(schema.required.contains("label"));
    assert!(schema.required.contains("value"));

    // Test that an attribute set without 'label' is missing required
    let test_attrs = vec!["value", "selected", "on_select"];
    let missing_required: Vec<_> = schema
        .required
        .iter()
        .filter(|&&req| !test_attrs.contains(&req))
        .collect();

    assert_eq!(missing_required.len(), 1);
    assert_eq!(*missing_required[0], "label");
}

#[test]
fn test_radio_missing_required_value_attribute() {
    let schema = WidgetAttributeSchema::for_widget(&WidgetKind::Radio);

    // Test that an attribute set without 'value' is missing required
    let test_attrs = vec!["label", "selected", "on_select"];
    let missing_required: Vec<_> = schema
        .required
        .iter()
        .filter(|&&req| !test_attrs.contains(&req))
        .collect();

    assert_eq!(missing_required.len(), 1);
    assert_eq!(*missing_required[0], "value");
}

#[test]
fn test_radio_missing_both_required_attributes() {
    let schema = WidgetAttributeSchema::for_widget(&WidgetKind::Radio);

    // Test that an attribute set without both 'label' and 'value' is missing both
    let test_attrs = vec!["selected", "on_select", "disabled"];
    let missing_required: Vec<_> = schema
        .required
        .iter()
        .filter(|&&req| !test_attrs.contains(&req))
        .collect();

    assert_eq!(missing_required.len(), 2);
}

#[test]
fn test_radio_with_all_required_attributes() {
    let schema = WidgetAttributeSchema::for_widget(&WidgetKind::Radio);

    // Test that an attribute set with both 'label' and 'value' has all required
    let test_attrs = vec!["label", "value", "selected"];
    let missing_required: Vec<_> = schema
        .required
        .iter()
        .filter(|&&req| !test_attrs.contains(&req))
        .collect();

    assert_eq!(missing_required.len(), 0);
}

// T051: Unit test for missing required attribute on Image widget
#[test]
fn test_image_missing_required_src_attribute() {
    let schema = WidgetAttributeSchema::for_widget(&WidgetKind::Image);

    // Image widget requires 'src' attribute
    assert!(schema.required.contains("src"));

    // Test that an attribute set without 'src' is missing required
    let test_attrs = vec!["width", "height", "fit"];
    let missing_required: Vec<_> = schema
        .required
        .iter()
        .filter(|&&req| !test_attrs.contains(&req))
        .collect();

    assert_eq!(missing_required.len(), 1);
    assert_eq!(*missing_required[0], "src");
}

#[test]
fn test_image_with_all_required_attributes() {
    let schema = WidgetAttributeSchema::for_widget(&WidgetKind::Image);

    // Test that an attribute set with 'src' has all required
    let test_attrs = vec!["src", "width", "height"];
    let missing_required: Vec<_> = schema
        .required
        .iter()
        .filter(|&&req| !test_attrs.contains(&req))
        .collect();

    assert_eq!(missing_required.len(), 0);
}
