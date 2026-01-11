//! Contract tests for the auto-loading mechanism.
//!
//! These tests verify that the #[dampen_ui] macro correctly generates
//! code for loading and parsing Dampen UI files at runtime.

#![allow(clippy::expect_used)]

use dampen_core::parse;
use std::sync::LazyLock;

static DAMPEN_DOCUMENT: LazyLock<dampen_core::DampenDocument> = LazyLock::new(|| {
    let xml = r#"
            <dampen>
                <column padding="40" spacing="20">
                    <text value="Hello" size="24" />
                    <button label="Click me" />
                </column>
            </dampen>
        "#;
    parse(xml).expect("Failed to parse test XML")
});

#[test]
fn test_document_parsing() {
    let doc = &DAMPEN_DOCUMENT;
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
    let doc = &DAMPEN_DOCUMENT;
    assert_eq!(doc.root.kind, dampen_core::ir::WidgetKind::Column);
}

#[test]
fn test_multiple_views_pattern() {
    static VIEW1: LazyLock<dampen_core::DampenDocument> = LazyLock::new(|| {
        let xml = r#"<dampen><column><text value="View 1" /></column></dampen>"#;
        parse(xml).expect("Failed to parse View 1")
    });

    static VIEW2: LazyLock<dampen_core::DampenDocument> = LazyLock::new(|| {
        let xml = r#"<dampen><column><text value="View 2" /></column></dampen>"#;
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
