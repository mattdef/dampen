use dampen_core::ir::{AttributeValue, EventKind, WidgetKind};
use dampen_core::parser::parse;

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
        on_select="update_filter"
    />
</column>"#;

    let result = parse(xml);
    assert!(result.is_ok(), "Should parse picklist with all attributes");
    let doc = result.unwrap();

    let picklist = doc
        .root
        .children
        .iter()
        .find(|c| matches!(c.kind, WidgetKind::PickList))
        .expect("Should have pick_list child");

    assert_eq!(picklist.events.len(), 1);
    assert!(matches!(picklist.events[0].event, EventKind::Select));
    assert_eq!(picklist.events[0].handler, "update_filter");
}

/// T033: Contract test for ColorPicker XML parsing
#[test]
fn parse_color_picker_basic() {
    let xml = r##"<?xml version="1.1"?>
<column>
    <color_picker value="#ff0000" show="{picker_open}" on_submit="color_selected">
        <button label="Pick Color" />
    </color_picker>
</column>"##;

    let result = parse(xml);
    assert!(
        result.is_ok(),
        "Should parse valid color_picker: {:?}",
        result.err()
    );
    let doc = result.unwrap();

    let picker = doc
        .root
        .children
        .iter()
        .find(|c| matches!(c.kind, WidgetKind::ColorPicker))
        .expect("Should have color_picker child");

    assert_eq!(picker.attributes.len(), 2);
    assert!(
        matches!(picker.attributes.get("value"), Some(AttributeValue::Static(s)) if s == "#ff0000")
    );
    assert!(picker.attributes.get("show").is_some());

    assert_eq!(picker.events.len(), 1);
    assert!(matches!(picker.events[0].event, EventKind::Submit));
    assert_eq!(picker.events[0].handler, "color_selected");
}

/// T034: Test invalid color formats in ColorPicker
#[test]
fn test_invalid_color_format() {
    let xml = r##"<?xml version="1.1"?>
<column>
    <color_picker value="#invalid" show="{picker_open}">
        <button label="Pick Color" />
    </color_picker>
</column>"##;

    let result = parse(xml);
    assert!(result.is_err(), "Should fail to parse invalid color format");
}
