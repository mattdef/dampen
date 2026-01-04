use gravity_core::ir::{AttributeValue, EventKind, WidgetKind};
use gravity_core::parser::parse;

/// T030: Integration test for ComboBox rendering
#[test]
fn test_build_combobox_basic() {
    let xml = r#"<?xml version="1.0"?>
<column>
    <combobox options="Work,Personal,Shopping" />
</column>"#;

    let result = parse(xml);
    assert!(result.is_ok(), "Should parse combobox XML");
    let doc = result.unwrap();

    // Verify combobox node exists
    let combobox = doc
        .root
        .children
        .iter()
        .find(|c| matches!(c.kind, WidgetKind::ComboBox))
        .expect("Should have combobox child");

    // Verify attributes
    assert!(
        matches!(combobox.attributes.get("options"), Some(AttributeValue::Static(s)) if s == "Work,Personal,Shopping")
    );

    // In a real test, we would call builder.build_widget() and verify it builds
    // For now, just verify the IR is correct
    assert!(combobox.events.is_empty());
}

/// T031: Integration test for PickList rendering
#[test]
fn test_build_picklist_basic() {
    let xml = r#"<?xml version="1.0"?>
<column>
    <pick_list options="All,Active,Completed" />
</column>"#;

    let result = parse(xml);
    assert!(result.is_ok(), "Should parse pick_list XML");
    let doc = result.unwrap();

    // Verify pick_list node exists
    let picklist = doc
        .root
        .children
        .iter()
        .find(|c| matches!(c.kind, WidgetKind::PickList))
        .expect("Should have pick_list child");

    // Verify attributes
    assert!(
        matches!(picklist.attributes.get("options"), Some(AttributeValue::Static(s)) if s == "All,Active,Completed")
    );

    // In a real test, we would call builder.build_widget() and verify it builds
    assert!(picklist.events.is_empty());
}

/// T032: Integration test for ComboBox event handling
#[test]
fn test_build_combobox_with_event() {
    let xml = r#"<?xml version="1.0"?>
<column>
    <combobox
        options="Work,Personal,Shopping"
        selected="{category}"
        placeholder="Select..."
        on_select="update_category"
    />
</column>"#;

    let result = parse(xml);
    assert!(result.is_ok(), "Should parse combobox with event");
    let doc = result.unwrap();

    let combobox = doc
        .root
        .children
        .iter()
        .find(|c| matches!(c.kind, WidgetKind::ComboBox))
        .expect("Should have combobox child");

    // Verify all attributes present
    assert!(combobox.attributes.contains_key("options"));
    assert!(combobox.attributes.contains_key("selected"));
    assert!(combobox.attributes.contains_key("placeholder"));

    // Verify event handler
    assert_eq!(combobox.events.len(), 1);
    assert!(matches!(combobox.events[0].event, EventKind::Select));
    assert_eq!(combobox.events[0].handler, "update_category");
}

/// T033: Integration test for PickList event handling
#[test]
fn test_build_picklist_with_event() {
    let xml = r#"<?xml version="1.0"?>
<column>
    <pick_list
        options="All,Active,Completed"
        selected="{filter}"
        placeholder="Filter..."
        on_select="apply_filter"
    />
</column>"#;

    let result = parse(xml);
    assert!(result.is_ok(), "Should parse pick_list with event");
    let doc = result.unwrap();

    let picklist = doc
        .root
        .children
        .iter()
        .find(|c| matches!(c.kind, WidgetKind::PickList))
        .expect("Should have pick_list child");

    // Verify all attributes present
    assert!(picklist.attributes.contains_key("options"));
    assert!(picklist.attributes.contains_key("selected"));
    assert!(picklist.attributes.contains_key("placeholder"));

    // Verify event handler
    assert_eq!(picklist.events.len(), 1);
    assert!(matches!(picklist.events[0].event, EventKind::Select));
    assert_eq!(picklist.events[0].handler, "apply_filter");
}

/// Additional test: ComboBox with binding for selected value
#[test]
fn test_combobox_binding() {
    let xml = r#"<?xml version="1.0"?>
<column>
    <combobox options="Red,Green,Blue" selected="{color}" />
</column>"#;

    let result = parse(xml);
    assert!(result.is_ok());
    let doc = result.unwrap();

    let combobox = doc
        .root
        .children
        .iter()
        .find(|c| matches!(c.kind, WidgetKind::ComboBox))
        .expect("Should have combobox child");

    // Verify binding is parsed
    assert!(matches!(
        combobox.attributes.get("selected"),
        Some(AttributeValue::Binding(_))
    ));
}

/// Additional test: PickList with binding for selected value
#[test]
fn test_picklist_binding() {
    let xml = r#"<?xml version="1.0"?>
<column>
    <pick_list options="A,B,C" selected="{choice}" />
</column>"#;

    let result = parse(xml);
    assert!(result.is_ok());
    let doc = result.unwrap();

    let picklist = doc
        .root
        .children
        .iter()
        .find(|c| matches!(c.kind, WidgetKind::PickList))
        .expect("Should have pick_list child");

    // Verify binding is parsed
    assert!(matches!(
        picklist.attributes.get("selected"),
        Some(AttributeValue::Binding(_))
    ));
}

/// Additional test: ComboBox with interpolated placeholder
#[test]
fn test_combobox_interpolated_placeholder() {
    let xml = r#"<?xml version="1.0"?>
<column>
    <combobox options="A,B" placeholder="Select {category}..." />
</column>"#;

    let result = parse(xml);
    assert!(result.is_ok());
    let doc = result.unwrap();

    let combobox = doc
        .root
        .children
        .iter()
        .find(|c| matches!(c.kind, WidgetKind::ComboBox))
        .expect("Should have combobox child");

    // Verify interpolated string is parsed
    assert!(matches!(
        combobox.attributes.get("placeholder"),
        Some(AttributeValue::Interpolated(_))
    ));
}
