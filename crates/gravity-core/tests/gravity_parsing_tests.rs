//! Tests for <gravity> document parsing

use gravity_core::parse;

#[test]
fn test_parse_gravity_with_themes() {
    let xml = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<gravity>\n    <themes>\n        <theme name=\"custom\">\n            <palette primary=\"#3498db\" secondary=\"#2ecc71\" success=\"#27ae60\" warning=\"#f39c12\" danger=\"#e74c3c\" background=\"#ecf0f1\" surface=\"#ffffff\" text=\"#2c3e50\" text_secondary=\"#7f8c8d\" />\n        </theme>\n    </themes>\n    <global_theme name=\"custom\" />\n    <column padding=\"40\" spacing=\"20\">\n        <text value=\"Test\" />\n    </column>\n</gravity>";

    let doc = parse(xml).expect("Should parse successfully");
    
    // Verify structure
    assert_eq!(doc.themes.len(), 1, "Should have 1 theme");
    assert!(doc.global_theme.is_some(), "Should have global theme");
    assert_eq!(doc.global_theme.unwrap(), "custom", "Global theme should be 'custom'");
    assert!(doc.themes.contains_key("custom"), "Should have 'custom' theme");
    assert_eq!(doc.root.kind, gravity_core::WidgetKind::Column, "Root should be column");
}

#[test]
fn test_parse_gravity_without_themes() {
    let xml = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<gravity>\n    <column padding=\"20\">\n        <text value=\"Hello\" />\n    </column>\n</gravity>";

    let doc = parse(xml).expect("Should parse successfully");
    
    assert_eq!(doc.themes.len(), 0, "Should have no themes");
    assert!(doc.global_theme.is_none(), "Should have no global theme");
    assert_eq!(doc.root.kind, gravity_core::WidgetKind::Column, "Root should be column");
}

#[test]
fn test_parse_backward_compatibility() {
    // Old format without <gravity> should still work
    let xml = "<column padding=\"20\"><text value=\"Test\" /></column>";
    
    let doc = parse(xml).expect("Should parse successfully");
    
    assert_eq!(doc.themes.len(), 0, "Should have no themes");
    assert_eq!(doc.root.kind, gravity_core::WidgetKind::Column, "Root should be column");
}

#[test]
fn test_parse_gravity_with_style_classes() {
    let xml = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<gravity>\n    <style_classes>\n        <class name=\"highlight\" background=\"#ffff00\" color=\"#000000\" />\n    </style_classes>\n    <column>\n        <text value=\"Test\" class=\"highlight\" />\n    </column>\n</gravity>";

    let doc = parse(xml).expect("Should parse successfully");
    
    assert_eq!(doc.style_classes.len(), 1, "Should have 1 style class");
    assert!(doc.style_classes.contains_key("highlight"), "Should have 'highlight' class");
}

#[test]
fn test_parse_gravity_multiple_widgets_error() {
    let xml = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<gravity>\n    <column><text value=\"First\" /></column>\n    <row><text value=\"Second\" /></row>\n</gravity>";

    let result = parse(xml);
    assert!(result.is_err(), "Should fail with multiple root widgets");
}

#[test]
fn test_parse_gravity_no_root_widget_error() {
    let xml = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<gravity>\n    <themes>\n        <theme name=\"custom\">\n            <palette primary=\"#3498db\" secondary=\"#2ecc71\" success=\"#27ae60\" warning=\"#f39c12\" danger=\"#e74c3c\" background=\"#ecf0f1\" surface=\"#ffffff\" text=\"#2c3e50\" text_secondary=\"#7f8c8d\" />\n        </theme>\n    </themes>\n</gravity>";

    let result = parse(xml);
    assert!(result.is_err(), "Should fail with no root widget");
}
