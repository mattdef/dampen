//! Tests for radio widget XML parsing

use gravity_core::parse;
use gravity_core::WidgetKind;

#[test]
fn test_parse_single_radio() {
    let xml = r#"<radio label="Option A" value="a" />"#;
    let doc = parse(xml).unwrap();

    assert_eq!(doc.root.kind, WidgetKind::Radio);
    assert_eq!(doc.root.children.len(), 0);
}

#[test]
fn test_parse_radio_with_label() {
    let xml = r#"<radio label="My Option" value="my_value" />"#;
    let doc = parse(xml).unwrap();

    let label = doc.root.attributes.get("label").unwrap();
    match label {
        gravity_core::AttributeValue::Static(s) => assert_eq!(s, "My Option"),
        _ => panic!("Expected static label"),
    }
}

#[test]
fn test_parse_radio_with_value() {
    let xml = r#"<radio label="Option" value="test_value" />"#;
    let doc = parse(xml).unwrap();

    let value = doc.root.attributes.get("value").unwrap();
    match value {
        gravity_core::AttributeValue::Static(s) => assert_eq!(s, "test_value"),
        _ => panic!("Expected static value"),
    }
}

#[test]
fn test_parse_radio_group() {
    let xml = r#"
        <column>
            <radio label="Small" value="small" />
            <radio label="Medium" value="medium" />
            <radio label="Large" value="large" />
        </column>
    "#;
    let doc = parse(xml).unwrap();

    assert_eq!(doc.root.kind, WidgetKind::Column);
    assert_eq!(doc.root.children.len(), 3);

    for (i, child) in doc.root.children.iter().enumerate() {
        assert_eq!(child.kind, WidgetKind::Radio);
    }
}

#[test]
fn test_parse_radio_with_on_select() {
    let xml = r#"<radio label="Option" value="opt" on_select="handleSelect" />"#;
    let doc = parse(xml).unwrap();

    assert_eq!(doc.root.events.len(), 1);
    assert_eq!(doc.root.events[0].event, gravity_core::EventKind::Select);
    assert_eq!(doc.root.events[0].handler, "handleSelect");
}

#[test]
fn test_parse_radio_with_selected_binding() {
    let xml = r#"<radio label="Option" value="opt" selected="{selection}" />"#;
    let doc = parse(xml).unwrap();

    let selected = doc.root.attributes.get("selected").unwrap();
    match selected {
        gravity_core::AttributeValue::Binding(_) => {
            // Binding expression - test passes
        }
        _ => panic!("Expected binding for selected attribute"),
    }
}

#[test]
fn test_parse_radio_with_disabled_binding() {
    let xml = r#"<radio label="Option" value="opt" disabled="{is_disabled}" />"#;
    let doc = parse(xml).unwrap();

    let disabled = doc.root.attributes.get("disabled").unwrap();
    match disabled {
        gravity_core::AttributeValue::Binding(_) => {
            // Binding expression - test passes
        }
        _ => panic!("Expected binding for disabled attribute"),
    }
}

#[test]
fn test_parse_radio_in_row() {
    let xml = r#"
        <row spacing="10">
            <radio label="A" value="a" />
            <radio label="B" value="b" />
            <radio label="C" value="c" />
        </row>
    "#;
    let doc = parse(xml).unwrap();

    assert_eq!(doc.root.kind, WidgetKind::Row);
    assert_eq!(doc.root.children.len(), 3);
}
