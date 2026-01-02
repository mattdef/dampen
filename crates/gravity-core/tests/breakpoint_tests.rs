//! Contract tests for breakpoint parsing and resolution

use gravity_core::ir::layout::Breakpoint;
use gravity_core::ir::node::AttributeValue;
use gravity_core::parse;

#[test]
fn test_parse_breakpoint_prefixes() {
    let xml = "<column 
        spacing=\"10\" 
        mobile-spacing=\"5\" 
        tablet-spacing=\"15\" 
        desktop-spacing=\"20\">
        <text value=\"Test\" />
    </column>";

    let doc = parse(xml).unwrap();
    let root = &doc.root;

    // Check base attribute
    assert_eq!(
        root.attributes.get("spacing"),
        Some(&AttributeValue::Static("10".to_string()))
    );

    // Check breakpoint attributes
    let mobile_attrs = root.breakpoint_attributes.get(&Breakpoint::Mobile);
    assert!(mobile_attrs.is_some());
    assert_eq!(
        mobile_attrs.unwrap().get("spacing"),
        Some(&AttributeValue::Static("5".to_string()))
    );

    let tablet_attrs = root.breakpoint_attributes.get(&Breakpoint::Tablet);
    assert!(tablet_attrs.is_some());
    assert_eq!(
        tablet_attrs.unwrap().get("spacing"),
        Some(&AttributeValue::Static("15".to_string()))
    );

    let desktop_attrs = root.breakpoint_attributes.get(&Breakpoint::Desktop);
    assert!(desktop_attrs.is_some());
    assert_eq!(
        desktop_attrs.unwrap().get("spacing"),
        Some(&AttributeValue::Static("20".to_string()))
    );
}

#[test]
fn test_parse_multiple_breakpoint_attributes() {
    let xml = "<container 
        width=\"400\"
        mobile-width=\"fill\"
        mobile-padding=\"10\"
        desktop-width=\"600\"
        desktop-padding=\"40\">
        <text value=\"Test\" />
    </container>";

    let doc = parse(xml).unwrap();
    let root = &doc.root;

    // Mobile breakpoint should have width and padding
    let mobile_attrs = root.breakpoint_attributes.get(&Breakpoint::Mobile).unwrap();
    assert_eq!(
        mobile_attrs.get("width"),
        Some(&AttributeValue::Static("fill".to_string()))
    );
    assert_eq!(
        mobile_attrs.get("padding"),
        Some(&AttributeValue::Static("10".to_string()))
    );

    // Desktop breakpoint should have width and padding
    let desktop_attrs = root
        .breakpoint_attributes
        .get(&Breakpoint::Desktop)
        .unwrap();
    assert_eq!(
        desktop_attrs.get("width"),
        Some(&AttributeValue::Static("600".to_string()))
    );
    assert_eq!(
        desktop_attrs.get("padding"),
        Some(&AttributeValue::Static("40".to_string()))
    );

    // Tablet should be empty
    let tablet_attrs = root.breakpoint_attributes.get(&Breakpoint::Tablet);
    assert!(tablet_attrs.is_none() || tablet_attrs.unwrap().is_empty());
}

#[test]
fn test_breakpoint_prefix_validation() {
    // Valid prefixes
    let valid_xml =
        "<column mobile-spacing=\"5\" tablet-spacing=\"10\" desktop-spacing=\"20\">...</column>";
    assert!(parse(valid_xml).is_ok());

    // Invalid prefix should be treated as regular attribute (not stored in breakpoint_attributes)
    let invalid_prefix_xml = "<column invalid-spacing=\"5\">...</column>";
    let doc = parse(invalid_prefix_xml).unwrap();

    // Should be in regular attributes, not breakpoint_attributes
    assert!(doc.root.attributes.contains_key("invalid-spacing"));
    assert!(doc.root.breakpoint_attributes.is_empty());
}

#[test]
fn test_breakpoint_attribute_override() {
    // Should succeed: base attribute and breakpoint attribute for same property
    // Breakpoint attributes override base attributes
    let xml = "<column spacing=\"10\" mobile-spacing=\"5\">...</column>";
    let doc = parse(xml).unwrap();

    // Both should be stored
    assert_eq!(
        doc.root.attributes.get("spacing"),
        Some(&AttributeValue::Static("10".to_string()))
    );
    assert_eq!(
        doc.root
            .breakpoint_attributes
            .get(&Breakpoint::Mobile)
            .unwrap()
            .get("spacing"),
        Some(&AttributeValue::Static("5".to_string()))
    );
}

#[test]
fn test_nested_widget_breakpoint_attributes() {
    let xml = "<column spacing=\"20\" mobile-spacing=\"10\">
        <row spacing=\"15\" desktop-spacing=\"30\">
            <text value=\"Nested\" />
        </row>
    </column>";

    let doc = parse(xml).unwrap();

    // Check root widget
    let root = &doc.root;
    assert!(root
        .breakpoint_attributes
        .get(&Breakpoint::Mobile)
        .is_some());

    // Check nested widget
    let row = &root.children[0];
    assert!(row
        .breakpoint_attributes
        .get(&Breakpoint::Desktop)
        .is_some());
    assert_eq!(
        row.breakpoint_attributes
            .get(&Breakpoint::Desktop)
            .unwrap()
            .get("spacing"),
        Some(&AttributeValue::Static("30".to_string()))
    );
}

#[test]
fn test_breakpoint_from_viewport_width() {
    assert_eq!(Breakpoint::from_viewport_width(0.0), Breakpoint::Mobile);
    assert_eq!(Breakpoint::from_viewport_width(639.0), Breakpoint::Mobile);
    assert_eq!(Breakpoint::from_viewport_width(640.0), Breakpoint::Tablet);
    assert_eq!(Breakpoint::from_viewport_width(1023.0), Breakpoint::Tablet);
    assert_eq!(Breakpoint::from_viewport_width(1024.0), Breakpoint::Desktop);
    assert_eq!(Breakpoint::from_viewport_width(2000.0), Breakpoint::Desktop);
}

#[test]
fn test_breakpoint_parse_from_string() {
    assert_eq!(Breakpoint::parse("mobile").unwrap(), Breakpoint::Mobile);
    assert_eq!(Breakpoint::parse("MOBILE").unwrap(), Breakpoint::Mobile);
    assert_eq!(Breakpoint::parse("tablet").unwrap(), Breakpoint::Tablet);
    assert_eq!(Breakpoint::parse("TABLET").unwrap(), Breakpoint::Tablet);
    assert_eq!(Breakpoint::parse("desktop").unwrap(), Breakpoint::Desktop);
    assert_eq!(Breakpoint::parse("DESKTOP").unwrap(), Breakpoint::Desktop);

    // Invalid
    assert!(Breakpoint::parse("laptop").is_err());
    assert!(Breakpoint::parse("").is_err());
}

#[test]
fn test_breakpoint_with_binding_expressions() {
    let xml = "<column 
        spacing=\"{count}\" 
        mobile-spacing=\"{mobile_count}\"
        desktop-spacing=\"{desktop_count}\">
        <text value=\"Test\" />
    </column>";

    let doc = parse(xml).unwrap();
    let root = &doc.root;

    // Base attribute should be a binding
    assert!(matches!(
        root.attributes.get("spacing"),
        Some(AttributeValue::Binding(_))
    ));

    // Breakpoint attributes should also support bindings
    let mobile_attrs = root.breakpoint_attributes.get(&Breakpoint::Mobile).unwrap();
    assert!(matches!(
        mobile_attrs.get("spacing"),
        Some(AttributeValue::Binding(_))
    ));
}

#[test]
fn test_breakpoint_with_complex_attributes() {
    let xml = "<container 
        background=\"#ffffff\"
        mobile-background=\"#000000\"
        border_width=\"2\"
        mobile-border_width=\"1\"
        desktop-border_width=\"4\">
        <text value=\"Test\" />
    </container>";

    let doc = parse(xml).unwrap();
    let root = &doc.root;

    // Check base attributes
    assert!(root.attributes.contains_key("background"));
    assert!(root.attributes.contains_key("border_width"));

    // Check mobile breakpoint
    let mobile_attrs = root.breakpoint_attributes.get(&Breakpoint::Mobile).unwrap();
    assert!(mobile_attrs.contains_key("background"));
    assert!(mobile_attrs.contains_key("border_width"));

    // Check desktop breakpoint
    let desktop_attrs = root
        .breakpoint_attributes
        .get(&Breakpoint::Desktop)
        .unwrap();
    assert!(desktop_attrs.contains_key("border_width"));
}

#[test]
fn test_breakpoint_empty_attributes() {
    let xml = "<column spacing=\"10\">...</column>";
    let doc = parse(xml).unwrap();

    // Should have no breakpoint attributes
    assert!(doc.root.breakpoint_attributes.is_empty());
}

#[test]
fn test_breakpoint_multiple_widgets_independent() {
    let xml = "<gravity>
        <column mobile-spacing=\"5\" desktop-spacing=\"20\">
            <text value=\"First\" />
            <text value=\"Second\" mobile-color=\"#ff0000\" desktop-color=\"#00ff00\" />
        </column>
    </gravity>";

    let doc = parse(xml).unwrap();

    // Root column
    let column = &doc.root;
    assert!(column
        .breakpoint_attributes
        .get(&Breakpoint::Mobile)
        .is_some());
    assert!(column
        .breakpoint_attributes
        .get(&Breakpoint::Desktop)
        .is_some());

    // Second text child
    let text = &column.children[1];
    assert!(text
        .breakpoint_attributes
        .get(&Breakpoint::Mobile)
        .is_some());
    assert!(text
        .breakpoint_attributes
        .get(&Breakpoint::Desktop)
        .is_some());

    // First text child should have no breakpoint attributes
    let first_text = &column.children[0];
    assert!(first_text.breakpoint_attributes.is_empty());
}

#[test]
fn test_breakpoint_serialization() {
    use serde_json;

    let xml = "<column mobile-spacing=\"5\" desktop-spacing=\"20\">...</column>";
    let doc = parse(xml).unwrap();

    // Serialize to JSON
    let json = serde_json::to_string(&doc.root.breakpoint_attributes).unwrap();

    // Should contain both breakpoints
    assert!(json.contains("Mobile"));
    assert!(json.contains("Desktop"));
    assert!(json.contains("5"));
    assert!(json.contains("20"));
}

#[test]
fn test_breakpoint_deserialization() {
    use serde_json;

    let xml = "<column mobile-spacing=\"5\" desktop-spacing=\"20\">...</column>";
    let doc = parse(xml).unwrap();

    // Serialize and deserialize
    let json = serde_json::to_string(&doc.root.breakpoint_attributes).unwrap();
    let deserialized: std::collections::HashMap<
        Breakpoint,
        std::collections::HashMap<String, AttributeValue>,
    > = serde_json::from_str(&json).unwrap();

    // Verify
    assert_eq!(deserialized.len(), 2);
    assert!(deserialized.contains_key(&Breakpoint::Mobile));
    assert!(deserialized.contains_key(&Breakpoint::Desktop));
}
