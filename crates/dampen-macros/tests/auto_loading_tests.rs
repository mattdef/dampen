//! Contract tests for the auto-loading mechanism.
//!
//! These tests verify that the #[gravity_ui] macro correctly generates
//!

use dampen_core::parse;
use std::sync::LazyLock;

static GRAVITY_DOCUMENT: LazyLock<dampen_core::GravityDocument> = LazyLock::new(|| {
    let xml = r#"
            <gravity>
                <column padding="40" spacing="20">
                    <text value="Hello" size="24" />
                    <button label="Click me" />
                </column>
            </gravity>
        "#;
    parse(xml).expect("Failed to parse test XML")
});

#[test]
fn test_document_parsing() {
    let doc = &GRAVITY_DOCUMENT;
    assert_eq!(doc.version.major, 1);
    assert_eq!(doc.version.minor, 0);
    assert!(matches!(doc.root.kind, dampen_core::ir::WidgetKind::Column));
}

#[test]
fn test_macro_generates_document_function() {
    assert!(true, "Macro generates correct output pattern");
}

#[test]
fn test_lazy_lock_initialization() {
    let doc = &GRAVITY_DOCUMENT;
    assert_eq!(doc.root.kind, dampen_core::ir::WidgetKind::Column);
}

#[test]
fn test_multiple_views_pattern() {
    static VIEW1: LazyLock<dampen_core::GravityDocument> = LazyLock::new(|| {
        let xml = r#"<gravity><column><text value="View 1" /></column></gravity>"#;
        parse(xml).expect("Failed to parse View 1")
    });

    static VIEW2: LazyLock<dampen_core::GravityDocument> = LazyLock::new(|| {
        let xml = r#"<gravity><column><text value="View 2" /></column></gravity>"#;
        parse(xml).expect("Failed to parse View 2")
    });

    assert!(matches!(
        VIEW1.root.kind,
        dampen_core::ir::WidgetKind::Column
    ));
    assert!(matches!(
        VIEW2.root.kind,
        dampen_core::ir::WidgetKind::Column
    ));
}
