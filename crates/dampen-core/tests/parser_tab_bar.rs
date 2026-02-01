//! Parser tests for TabBar and Tab widgets
//!
//! Tests validation of Tab/TabBar structure and constraints.

use dampen_core::{ir::WidgetKind, parse, parser::error::ParseErrorKind};

#[test]
fn test_parse_valid_tab_bar_with_tabs() {
    let xml = r#"<dampen version="1.1">
        <tab_bar selected="0" on_select="on_tab_selected">
            <tab label="General" />
            <tab label="Appearance" />
            <tab label="Notifications" />
        </tab_bar>
    </dampen>"#;

    let result = parse(xml);
    assert!(
        result.is_ok(),
        "Should parse valid TabBar with Tab children"
    );

    let doc = result.unwrap();
    assert_eq!(doc.root.kind, WidgetKind::TabBar);
    assert_eq!(doc.root.children.len(), 3);

    for child in &doc.root.children {
        assert_eq!(child.kind, WidgetKind::Tab);
    }
}

#[test]
fn test_parse_tab_outside_tab_bar_fails() {
    let xml = r#"<dampen version="1.1">
        <column>
            <tab label="Invalid Tab" />
        </column>
    </dampen>"#;

    let result = parse(xml);
    assert!(result.is_err(), "Tab outside TabBar should fail");

    let err = result.unwrap_err();
    assert_eq!(err.kind, ParseErrorKind::InvalidChild);
    assert!(err.message.contains("Tab must be inside TabBar"));
}

#[test]
fn test_parse_tab_bar_with_non_tab_child_fails() {
    let xml = r#"<dampen version="1.1">
        <tab_bar selected="0" on_select="on_tab_selected">
            <tab label="Valid Tab" />
            <text value="Invalid Child" />
        </tab_bar>
    </dampen>"#;

    let result = parse(xml);
    assert!(result.is_err(), "TabBar with non-Tab child should fail");

    let err = result.unwrap_err();
    assert_eq!(err.kind, ParseErrorKind::InvalidChild);
    assert!(err.message.contains("TabBar can only contain Tab widgets"));
}

#[test]
fn test_parse_tab_bar_with_icon_attribute() {
    let xml = r#"<dampen version="1.1">
        <tab_bar selected="0" on_select="on_tab_selected">
            <tab label="Home" icon="home" />
            <tab label="Settings" icon="settings" />
        </tab_bar>
    </dampen>"#;

    let result = parse(xml);
    assert!(result.is_ok(), "Should parse TabBar with icon attributes");

    let doc = result.unwrap();
    let first_tab = &doc.root.children[0];
    assert!(first_tab.attributes.contains_key("icon"));
    assert!(first_tab.attributes.contains_key("label"));
}

#[test]
fn test_parse_tab_bar_with_binding() {
    let xml = r#"<dampen version="1.1">
        <tab_bar selected="{selected_tab}" on_select="on_tab_selected">
            <tab label="Tab 1" />
            <tab label="Tab 2" />
        </tab_bar>
    </dampen>"#;

    let result = parse(xml);
    assert!(
        result.is_ok(),
        "Should parse TabBar with binding expression"
    );

    let doc = result.unwrap();
    assert!(doc.root.attributes.contains_key("selected"));
}

#[test]
fn test_parse_empty_tab_bar() {
    let xml = r#"<dampen version="1.1">
        <tab_bar selected="0" on_select="on_tab_selected">
        </tab_bar>
    </dampen>"#;

    let result = parse(xml);
    assert!(
        result.is_ok(),
        "Should parse empty TabBar (graceful degradation)"
    );

    let doc = result.unwrap();
    assert_eq!(doc.root.children.len(), 0);
}

#[test]
fn test_parse_tab_with_enabled_attribute() {
    let xml = r#"<dampen version="1.1">
        <tab_bar selected="0" on_select="on_tab_selected">
            <tab label="Public" />
            <tab label="Admin" enabled="{is_admin}" />
        </tab_bar>
    </dampen>"#;

    let result = parse(xml);
    assert!(result.is_ok(), "Should parse Tab with enabled attribute");

    let doc = result.unwrap();
    let admin_tab = &doc.root.children[1];
    assert!(admin_tab.attributes.contains_key("enabled"));
}

#[test]
fn test_tab_bar_requires_v1_1() {
    // TabBar requires v1.1, so using v1.0 should fail
    let xml = r#"<dampen version="1.0">
        <tab_bar selected="0" on_select="on_tab_selected">
            <tab label="Tab 1" />
        </tab_bar>
    </dampen>"#;

    let result = parse(xml);
    assert!(result.is_err(), "TabBar should require schema v1.1");

    let err = result.unwrap_err();
    assert_eq!(err.kind, ParseErrorKind::UnsupportedVersion);
}

#[test]
fn test_tab_requires_v1_1() {
    // Tab requires v1.1, so using v1.0 should fail
    let xml = r#"<dampen version="1.0">
        <tab_bar selected="0" on_select="on_tab_selected">
            <tab label="Tab 1" />
        </tab_bar>
    </dampen>"#;

    let result = parse(xml);
    assert!(result.is_err(), "Tab should require schema v1.1");
}
