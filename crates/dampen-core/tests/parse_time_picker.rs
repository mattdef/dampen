use dampen_core::{AttributeValue, WidgetKind, parse};

#[test]
fn test_parse_valid_time_picker() {
    let xml = r#"
        <time_picker>
            <button label="Pick Time" />
        </time_picker>
    "#;
    let doc = parse(xml).expect("Should parse valid XML");
    let root = doc.root;

    assert!(matches!(root.kind, WidgetKind::TimePicker));
    assert_eq!(root.children.len(), 1);
}

#[test]
fn test_parse_time_picker_attributes() {
    let xml = r#"
        <time_picker use_24h="true" show_seconds="true">
            <button />
        </time_picker>
    "#;
    let doc = parse(xml).unwrap();
    let root = doc.root;

    match root.attributes.get("use_24h").unwrap() {
        AttributeValue::Static(s) => assert_eq!(s, "true"),
        _ => panic!("Expected static attribute"),
    }

    match root.attributes.get("show_seconds").unwrap() {
        AttributeValue::Static(s) => assert_eq!(s, "true"),
        _ => panic!("Expected static attribute"),
    }
}

#[test]
fn test_time_picker_custom_format() {
    let xml = r#"<time_picker value="02:30 PM" format="%I:%M %p"><button/></time_picker>"#;
    let doc = parse(xml).expect("Should parse valid XML with custom format");
    let root = doc.root;

    match root.attributes.get("format").unwrap() {
        AttributeValue::Static(s) => assert_eq!(s, "%I:%M %p"),
        _ => panic!("Expected static format attribute"),
    }
}

#[test]
fn test_time_picker_binding() {
    let xml = r#"<time_picker value="{selected_time}"><button/></time_picker>"#;
    let doc = parse(xml).expect("Should parse binding");
    let root = doc.root;

    match root.attributes.get("value").unwrap() {
        AttributeValue::Binding(_) => {}
        _ => panic!("Expected binding attribute"),
    }
}
