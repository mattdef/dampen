//! Integration tests for breakpoint resolution in runtime

use gravity_core::ir::layout::Breakpoint;
use gravity_core::parse;
use gravity_runtime::{
    resolve_breakpoint_attributes, resolve_tree_breakpoint_attributes, would_change_breakpoint,
};

#[test]
fn test_resolve_breakpoint_attributes_mobile() {
    let xml = "<column spacing=\"10\" mobile-spacing=\"5\" desktop-spacing=\"20\">...</column>";
    let doc = parse(xml).unwrap();

    // Mobile viewport (600px)
    let attrs = resolve_breakpoint_attributes(&doc.root, 600.0);

    // Should use mobile spacing
    assert_eq!(
        attrs.get("spacing"),
        Some(&gravity_core::ir::node::AttributeValue::Static(
            "5".to_string()
        ))
    );
}

#[test]
fn test_resolve_breakpoint_attributes_tablet() {
    let xml = "<column spacing=\"10\" mobile-spacing=\"5\" tablet-spacing=\"15\" desktop-spacing=\"20\">...</column>";
    let doc = parse(xml).unwrap();

    // Tablet viewport (800px)
    let attrs = resolve_breakpoint_attributes(&doc.root, 800.0);

    // Should use tablet spacing
    assert_eq!(
        attrs.get("spacing"),
        Some(&gravity_core::ir::node::AttributeValue::Static(
            "15".to_string()
        ))
    );
}

#[test]
fn test_resolve_breakpoint_attributes_desktop() {
    let xml = "<column spacing=\"10\" mobile-spacing=\"5\" desktop-spacing=\"20\">...</column>";
    let doc = parse(xml).unwrap();

    // Desktop viewport (1200px)
    let attrs = resolve_breakpoint_attributes(&doc.root, 1200.0);

    // Should use desktop spacing
    assert_eq!(
        attrs.get("spacing"),
        Some(&gravity_core::ir::node::AttributeValue::Static(
            "20".to_string()
        ))
    );
}

#[test]
fn test_resolve_breakpoint_attributes_fallback() {
    let xml = "<column spacing=\"10\" mobile-spacing=\"5\">...</column>";
    let doc = parse(xml).unwrap();

    // Desktop viewport - no desktop-specific attribute
    let attrs = resolve_breakpoint_attributes(&doc.root, 1200.0);

    // Should fallback to base attribute
    assert_eq!(
        attrs.get("spacing"),
        Some(&gravity_core::ir::node::AttributeValue::Static(
            "10".to_string()
        ))
    );
}

#[test]
fn test_resolve_tree_breakpoint_attributes() {
    let xml = "<column spacing=\"20\" mobile-spacing=\"10\">
        <row spacing=\"15\" desktop-spacing=\"30\">
            <text value=\"Test\" />
        </row>
    </column>";
    let doc = parse(xml).unwrap();

    // Resolve for mobile
    let resolved = resolve_tree_breakpoint_attributes(&doc.root, 600.0);

    // Root should have mobile spacing
    assert_eq!(
        resolved.attributes.get("spacing"),
        Some(&gravity_core::ir::node::AttributeValue::Static(
            "10".to_string()
        ))
    );

    // Child row should have base spacing (no mobile override)
    let child = &resolved.children[0];
    assert_eq!(
        child.attributes.get("spacing"),
        Some(&gravity_core::ir::node::AttributeValue::Static(
            "15".to_string()
        ))
    );

    // Resolve for desktop
    let resolved_desktop = resolve_tree_breakpoint_attributes(&doc.root, 1200.0);

    // Root should have base spacing
    assert_eq!(
        resolved_desktop.attributes.get("spacing"),
        Some(&gravity_core::ir::node::AttributeValue::Static(
            "20".to_string()
        ))
    );

    // Child row should have desktop spacing
    let child_desktop = &resolved_desktop.children[0];
    assert_eq!(
        child_desktop.attributes.get("spacing"),
        Some(&gravity_core::ir::node::AttributeValue::Static(
            "30".to_string()
        ))
    );
}

#[test]
fn test_would_change_breakpoint() {
    // Mobile to mobile
    assert!(!would_change_breakpoint(100.0, 500.0));

    // Mobile to tablet
    assert!(would_change_breakpoint(600.0, 700.0));

    // Tablet to desktop
    assert!(would_change_breakpoint(1000.0, 1100.0));

    // Desktop to tablet
    assert!(would_change_breakpoint(1100.0, 900.0));

    // Tablet to mobile
    assert!(would_change_breakpoint(700.0, 600.0));

    // Desktop to mobile (large change)
    assert!(would_change_breakpoint(1500.0, 500.0));
}

#[test]
fn test_resolve_multiple_attributes() {
    let xml = "<container 
        width=\"400\" padding=\"20\"
        mobile-width=\"fill\" mobile-padding=\"10\"
        desktop-width=\"600\" desktop-padding=\"40\"
        background=\"#ffffff\">
        <text value=\"Test\" />
    </container>";
    let doc = parse(xml).unwrap();

    // Mobile resolution
    let mobile_attrs = resolve_breakpoint_attributes(&doc.root, 600.0);
    assert_eq!(
        mobile_attrs.get("width"),
        Some(&gravity_core::ir::node::AttributeValue::Static(
            "fill".to_string()
        ))
    );
    assert_eq!(
        mobile_attrs.get("padding"),
        Some(&gravity_core::ir::node::AttributeValue::Static(
            "10".to_string()
        ))
    );
    assert_eq!(
        mobile_attrs.get("background"),
        Some(&gravity_core::ir::node::AttributeValue::Static(
            "#ffffff".to_string()
        ))
    );

    // Desktop resolution
    let desktop_attrs = resolve_breakpoint_attributes(&doc.root, 1200.0);
    assert_eq!(
        desktop_attrs.get("width"),
        Some(&gravity_core::ir::node::AttributeValue::Static(
            "600".to_string()
        ))
    );
    assert_eq!(
        desktop_attrs.get("padding"),
        Some(&gravity_core::ir::node::AttributeValue::Static(
            "40".to_string()
        ))
    );
    assert_eq!(
        desktop_attrs.get("background"),
        Some(&gravity_core::ir::node::AttributeValue::Static(
            "#ffffff".to_string()
        ))
    );

    // Tablet resolution (fallback)
    let tablet_attrs = resolve_breakpoint_attributes(&doc.root, 800.0);
    assert_eq!(
        tablet_attrs.get("width"),
        Some(&gravity_core::ir::node::AttributeValue::Static(
            "400".to_string()
        ))
    );
    assert_eq!(
        tablet_attrs.get("padding"),
        Some(&gravity_core::ir::node::AttributeValue::Static(
            "20".to_string()
        ))
    );
}

#[test]
fn test_resolve_with_binding_expressions() {
    let xml = "<column 
        spacing=\"{count}\" 
        mobile-spacing=\"{mobile_count}\">
        <text value=\"Test\" />
    </column>";
    let doc = parse(xml).unwrap();

    // Mobile resolution
    let mobile_attrs = resolve_breakpoint_attributes(&doc.root, 600.0);

    // Should have binding for mobile spacing
    assert!(matches!(
        mobile_attrs.get("spacing"),
        Some(gravity_core::ir::node::AttributeValue::Binding(_))
    ));

    // Desktop resolution
    let desktop_attrs = resolve_breakpoint_attributes(&doc.root, 1200.0);

    // Should have binding for base spacing
    assert!(matches!(
        desktop_attrs.get("spacing"),
        Some(gravity_core::ir::node::AttributeValue::Binding(_))
    ));
}

#[test]
#[test]
fn test_resolve_empty_breakpoint_attributes() {
    let xml = "<column spacing=\"10\">...</column>";
    let doc = parse(xml).unwrap();

    // Should just return base attributes
    let attrs = resolve_breakpoint_attributes(&doc.root, 600.0);
    assert_eq!(
        attrs.get("spacing"),
        Some(&gravity_core::ir::node::AttributeValue::Static(
            "10".to_string()
        ))
    );
}

#[test]
fn test_resolve_tree_preserves_structure() {
    let xml = "<column spacing=\"20\">
        <text value=\"Child 1\" />
        <text value=\"Child 2\" />
    </column>";
    let doc = parse(xml).unwrap();

    let resolved = resolve_tree_breakpoint_attributes(&doc.root, 600.0);

    // Should preserve children count
    assert_eq!(resolved.children.len(), 2);

    // Should preserve child content
    assert_eq!(
        resolved.children[0].attributes.get("value"),
        Some(&gravity_core::ir::node::AttributeValue::Static(
            "Child 1".to_string()
        ))
    );
    assert_eq!(
        resolved.children[1].attributes.get("value"),
        Some(&gravity_core::ir::node::AttributeValue::Static(
            "Child 2".to_string()
        ))
    );
}

#[test]
fn test_breakpoint_boundaries() {
    // Test exact boundary values
    assert_eq!(Breakpoint::from_viewport_width(639.9), Breakpoint::Mobile);
    assert_eq!(Breakpoint::from_viewport_width(640.0), Breakpoint::Tablet);
    assert_eq!(Breakpoint::from_viewport_width(1023.9), Breakpoint::Tablet);
    assert_eq!(Breakpoint::from_viewport_width(1024.0), Breakpoint::Desktop);
}
