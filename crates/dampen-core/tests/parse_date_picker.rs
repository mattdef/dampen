use dampen_core::{WidgetKind, parse};

#[test]
fn test_parse_valid_date_picker() {
    let xml = r#"
        <date_picker>
            <button label="Pick Date" />
        </date_picker>
    "#;
    let doc = parse(xml).expect("Should parse valid XML");
    let root = doc.root;

    assert!(matches!(root.kind, WidgetKind::DatePicker));
    assert_eq!(root.children.len(), 1);
    assert!(matches!(root.children[0].kind, WidgetKind::Button));
}

#[test]
fn test_parse_date_picker_zero_children_error() {
    let xml = r#"<date_picker />"#;
    let result = parse(xml);

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.message.contains("must have exactly one child"));
}

#[test]
fn test_parse_date_picker_multiple_children_error() {
    let xml = r#"
        <date_picker>
            <button label="One" />
            <button label="Two" />
        </date_picker>
    "#;
    let result = parse(xml);

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.message.contains("must have exactly one child"));
}

#[test]
fn test_date_picker_custom_format() {
    let xml = r#"<date_picker value="25/01/2026" format="%d/%m/%Y"><button/></date_picker>"#;
    let doc = parse(xml).expect("Should parse valid XML with custom format");
    let root = doc.root;

    match root.attributes.get("format").unwrap() {
        dampen_core::ir::AttributeValue::Static(s) => assert_eq!(s, "%d/%m/%Y"),
        _ => panic!("Expected static format attribute"),
    }
}

#[test]
fn test_date_picker_invalid_format_error() {
    let xml = r#"<date_picker value="2026-01-25" format="%d/%m/%Y"><button/></date_picker>"#;
    let result = parse(xml);

    // Should fail validation for mismatched date format
    assert!(
        result.is_err(),
        "Should fail validation for mismatched date format"
    );
}

#[test]
fn test_date_picker_binding() {
    let xml = r#"<date_picker value="{selected_date}"><button/></date_picker>"#;
    let doc = parse(xml).expect("Should parse binding");
    let root = doc.root;

    match root.attributes.get("value").unwrap() {
        dampen_core::ir::AttributeValue::Binding(_) => {}
        _ => panic!("Expected binding attribute"),
    }
}

#[test]
fn test_date_picker_min_max_date() {
    let xml = r#"<date_picker min_date="2026-01-01" max_date="2026-12-31"><button/></date_picker>"#;
    let doc = parse(xml).expect("Should parse min/max date");
    let root = doc.root;

    assert!(root.attributes.contains_key("min_date"));
    assert!(root.attributes.contains_key("max_date"));
}

#[test]
fn test_date_picker_invalid_range_error() {
    let xml = r#"<date_picker min_date="2026-12-31" max_date="2026-01-01"><button/></date_picker>"#;
    let result = parse(xml);

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.message.contains("Invalid date range"));
}
