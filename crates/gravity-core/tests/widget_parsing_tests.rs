use gravity_core::ir::{AttributeValue, EventKind, WidgetKind};
use gravity_core::parser::parse;

/// T024: Contract test for ComboBox XML parsing
#[test]
fn parse_combobox_basic() {
    let xml = r#"<?xml version="1.0"?>
<column>
    <combobox options="Work,Personal,Shopping" />
</column>"#;

    let result = parse(xml);
    assert!(result.is_ok(), "Should parse valid combobox");
    let doc = result.unwrap();

    // Find combobox child
    let combobox = doc
        .root
        .children
        .iter()
        .find(|c| matches!(c.kind, WidgetKind::ComboBox))
        .expect("Should have combobox child");

    assert_eq!(combobox.attributes.len(), 1);
    assert!(
        matches!(combobox.attributes.get("options"), Some(AttributeValue::Static(s)) if s == "Work,Personal,Shopping")
    );
}

/// T025: Contract test for PickList XML parsing
#[test]
fn parse_picklist_basic() {
    let xml = r#"<?xml version="1.0"?>
<column>
    <pick_list options="All,Active,Completed" />
</column>"#;

    let result = parse(xml);
    assert!(result.is_ok(), "Should parse valid pick_list");
    let doc = result.unwrap();

    // Find pick_list child
    let picklist = doc
        .root
        .children
        .iter()
        .find(|c| matches!(c.kind, WidgetKind::PickList))
        .expect("Should have pick_list child");

    assert_eq!(picklist.attributes.len(), 1);
    assert!(
        matches!(picklist.attributes.get("options"), Some(AttributeValue::Static(s)) if s == "All,Active,Completed")
    );
}

/// T026: Contract test for ComboBox with all attributes
#[test]
fn parse_combobox_with_all_attributes() {
    let xml = r#"<?xml version="1.0"?>
<column>
    <combobox 
        options="Work,Personal,Shopping,Other"
        selected="{category}"
        placeholder="Select category..."
        on_select="update_category"
    />
</column>"#;

    let result = parse(xml);
    assert!(result.is_ok(), "Should parse combobox with all attributes");
    let doc = result.unwrap();

    let combobox = doc
        .root
        .children
        .iter()
        .find(|c| matches!(c.kind, WidgetKind::ComboBox))
        .expect("Should have combobox child");

    // Check options
    assert!(
        matches!(combobox.attributes.get("options"), Some(AttributeValue::Static(s)) if s == "Work,Personal,Shopping,Other")
    );

    // Check selected binding
    assert!(combobox.attributes.get("selected").is_some());

    // Check placeholder
    assert!(
        matches!(combobox.attributes.get("placeholder"), Some(AttributeValue::Static(s)) if s == "Select category...")
    );

    // Check event handler
    assert_eq!(combobox.events.len(), 1);
    assert!(matches!(combobox.events[0].event, EventKind::Select));
    assert_eq!(combobox.events[0].handler, "update_category");
}

/// T027: Contract test for PickList with all attributes
#[test]
fn parse_picklist_with_all_attributes() {
    let xml = r#"<?xml version="1.0"?>
<column>
    <pick_list 
        options="All,Active,Completed"
        selected="{filter}"
        placeholder="Filter tasks..."
        on_select="apply_filter"
    />
</column>"#;

    let result = parse(xml);
    assert!(result.is_ok(), "Should parse pick_list with all attributes");
    let doc = result.unwrap();

    let picklist = doc
        .root
        .children
        .iter()
        .find(|c| matches!(c.kind, WidgetKind::PickList))
        .expect("Should have pick_list child");

    // Check options
    assert!(
        matches!(picklist.attributes.get("options"), Some(AttributeValue::Static(s)) if s == "All,Active,Completed")
    );

    // Check selected binding
    assert!(picklist.attributes.get("selected").is_some());

    // Check placeholder
    assert!(
        matches!(picklist.attributes.get("placeholder"), Some(AttributeValue::Static(s)) if s == "Filter tasks...")
    );

    // Check event handler
    assert_eq!(picklist.events.len(), 1);
    assert!(matches!(picklist.events[0].event, EventKind::Select));
    assert_eq!(picklist.events[0].handler, "apply_filter");
}

/// T028: Contract test for ComboBox missing required attributes
#[test]
fn parse_combobox_missing_options_errors() {
    let xml = r#"<?xml version="1.0"?>
<column>
    <combobox />
</column>"#;

    let result = parse(xml);

    // Should fail with parse error about missing required attribute
    assert!(result.is_err(), "Should fail when options is missing");
    let err = result.unwrap_err();
    assert!(err.to_string().to_lowercase().contains("options"));
}

/// T029: Contract test for PickList missing required attributes
#[test]
fn parse_picklist_missing_options_errors() {
    let xml = r#"<?xml version="1.0"?>
<column>
    <pick_list />
</column>"#;

    let result = parse(xml);

    // Should fail with parse error about missing required attribute
    assert!(result.is_err(), "Should fail when options is missing");
    let err = result.unwrap_err();
    assert!(err.to_string().to_lowercase().contains("options"));
}

/// Additional test: ComboBox with empty options should error
#[test]
fn parse_combobox_empty_options_errors() {
    let xml = r#"<?xml version="1.0"?>
<column>
    <combobox options="" />
</column>"#;

    let result = parse(xml);
    assert!(result.is_err(), "Should fail when options is empty");
}

/// Additional test: PickList with single option
#[test]
fn parse_picklist_single_option() {
    let xml = r#"<?xml version="1.0"?>
<column>
    <pick_list options="OnlyOption" />
</column>"#;

    let result = parse(xml);
    assert!(result.is_ok(), "Should parse pick_list with single option");
    let doc = result.unwrap();

    let picklist = doc
        .root
        .children
        .iter()
        .find(|c| matches!(c.kind, WidgetKind::PickList))
        .expect("Should have pick_list child");

    assert!(
        matches!(picklist.attributes.get("options"), Some(AttributeValue::Static(s)) if s == "OnlyOption")
    );
}

/// Additional test: ComboBox with many options
#[test]
fn parse_combobox_many_options() {
    let options = (1..=50)
        .map(|i| format!("Option{}", i))
        .collect::<Vec<_>>()
        .join(",");
    let xml = format!(
        r#"<?xml version="1.0"?>
<column>
    <combobox options="{}" />
</column>"#,
        options
    );

    let result = parse(&xml);
    assert!(result.is_ok(), "Should parse combobox with many options");
    let doc = result.unwrap();

    let combobox = doc
        .root
        .children
        .iter()
        .find(|c| matches!(c.kind, WidgetKind::ComboBox))
        .expect("Should have combobox child");

    assert!(combobox.attributes.get("options").is_some());
}
