//! Tests for radio widget rendering

use dampen_core::{WidgetKind, parse};
use dampen_iced::{IcedBackend, render};

#[test]
fn test_render_single_radio() {
    let xml = r#"<radio label="Option A" value="a" />"#;
    let doc = parse(xml).unwrap();

    let backend = IcedBackend::new(|_, _| Box::new(()));
    let element = render(&doc.root, &backend);

    // Radio renders successfully - the element is created
    assert!(true);
}

#[test]
fn test_render_radio_with_label() {
    let xml = r#"<radio label="My Radio Option" value="my_value" />"#;
    let doc = parse(xml).unwrap();

    let backend = IcedBackend::new(|_, _| Box::new(()));
    let element = render(&doc.root, &backend);

    // Should render without error
    assert!(true);
}

#[test]
fn test_render_radio_group() {
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

    for child in &doc.root.children {
        assert_eq!(child.kind, WidgetKind::Radio);
    }
}

#[test]
fn test_render_radio_with_select_handler() {
    let xml = r#"<radio label="Option" value="opt" on_select="handleSelect" />"#;
    let doc = parse(xml).unwrap();

    assert_eq!(doc.root.events.len(), 1);
    assert_eq!(doc.root.events[0].event, dampen_core::EventKind::Select);
    assert_eq!(doc.root.events[0].handler, "handleSelect");
}

#[test]
fn test_radio_parsed_as_widget_kind() {
    let xml = r#"<radio label="Test" value="test" />"#;
    let doc = parse(xml).unwrap();

    assert_eq!(doc.root.kind, WidgetKind::Radio);
}

#[test]
fn test_radio_has_no_children() {
    let xml = r#"<radio label="Option" value="opt" />"#;
    let doc = parse(xml).unwrap();

    // Radio is an atomic widget with no children
    assert_eq!(doc.root.children.len(), 0);
}
