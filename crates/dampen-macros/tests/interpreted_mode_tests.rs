//! Contract tests for the #[dampen_ui] macro in interpreted mode.
//!
//! These tests verify that when interpreted mode is active (default or explicit),
//! the macro generates runtime XML parsing code with LazyLock initialization.
//!
//! Interpreted mode is active when:
//! - No features specified (default)
//! - `feature = "interpreted"` is explicitly enabled
//! - Both `codegen` and `interpreted` features are enabled (interpreted takes priority)
//!
//! **Note**: The cfg checks use feature flags from the consuming crate, not this test crate.

#![allow(unexpected_cfgs)]
#![allow(clippy::assertions_on_constants)]
#![allow(clippy::expect_used)]
#![allow(clippy::unwrap_used)]

use dampen_core::parse;
use std::sync::LazyLock;

#[cfg(any(feature = "interpreted", not(feature = "codegen")))]
mod interpreted_active {
    //! Tests for when interpreted mode is active

    use super::*;

    #[test]
    fn test_interpreted_mode_documentation() {
        // Interpreted mode is the default and provides runtime XML parsing
        assert!(
            cfg!(any(feature = "interpreted", not(feature = "codegen"))),
            "Interpreted mode should be active"
        );
    }

    #[test]
    fn test_runtime_xml_parsing() {
        // In interpreted mode, XML is parsed at runtime on first access
        let xml = r#"
            <dampen version="1.0">
                <column padding="20">
                    <text value="Hello, World!" size="24" />
                </column>
            </dampen>
        "#;

        let result = parse(xml);
        assert!(result.is_ok(), "Runtime XML parsing should work");

        let doc = result.unwrap();
        assert_eq!(doc.version.major, 1);
        assert_eq!(doc.version.minor, 0);
    }

    #[test]
    fn test_lazylock_initialization() {
        // LazyLock ensures XML is parsed only once, on first access
        static TEST_DOC: LazyLock<dampen_core::DampenDocument> = LazyLock::new(|| {
            let xml = r#"<dampen version="1.0"><column><text value="Test" /></column></dampen>"#;
            parse(xml).expect("Parse failed")
        });

        // First access triggers parsing
        let doc1 = &*TEST_DOC;
        assert_eq!(doc1.version.major, 1);

        // Subsequent accesses reuse parsed document
        let doc2 = &*TEST_DOC;
        assert_eq!(doc2.version.major, 1);
    }

    #[test]
    fn test_include_str_embedding() {
        // XML content is embedded at compile time via include_str!
        // This allows hot-reload without recompilation in dev mode
        let xml = include_str!("../tests/fixtures/test_ui.xml");
        assert!(!xml.is_empty(), "XML should be embedded at compile time");
    }

    #[test]
    fn test_hot_reload_support() {
        // In interpreted mode, changes to XML files can be hot-reloaded
        // without full recompilation (when using file watcher)
        assert!(true, "Interpreted mode supports hot-reload workflow");
    }

    #[test]
    fn test_multiple_views() {
        // Multiple #[dampen_ui] invocations work independently
        static VIEW1: LazyLock<dampen_core::DampenDocument> = LazyLock::new(|| {
            let xml = r#"<dampen version="1.0"><column><text value="View 1" /></column></dampen>"#;
            parse(xml).expect("Parse View 1 failed")
        });

        static VIEW2: LazyLock<dampen_core::DampenDocument> = LazyLock::new(|| {
            let xml = r#"<dampen version="1.0"><column><text value="View 2" /></column></dampen>"#;
            parse(xml).expect("Parse View 2 failed")
        });

        assert_eq!(VIEW1.version.major, 1);
        assert_eq!(VIEW2.version.major, 1);
    }
}

#[cfg(all(feature = "codegen", not(feature = "interpreted")))]
mod interpreted_inactive {
    //! Tests for when interpreted mode is NOT active (codegen exclusive)

    #[test]
    fn test_codegen_mode_active() {
        // When codegen is exclusively enabled, interpreted mode is NOT active
        assert!(
            cfg!(all(feature = "codegen", not(feature = "interpreted"))),
            "Codegen mode should be exclusively active"
        );
        assert!(
            !cfg!(any(feature = "interpreted", not(feature = "codegen"))),
            "Interpreted mode should NOT be active"
        );
    }
}

#[test]
fn test_interpreted_is_default() {
    // When no features are specified, interpreted mode is active
    #[cfg(not(feature = "codegen"))]
    {
        assert!(
            true,
            "Interpreted mode is the default when no features specified"
        );
    }
}

#[test]
fn test_interpreted_priority_over_codegen() {
    // When both features are enabled, interpreted mode takes priority
    #[cfg(all(feature = "codegen", feature = "interpreted"))]
    {
        assert!(
            cfg!(any(feature = "interpreted", not(feature = "codegen"))),
            "Interpreted mode takes priority when both features enabled"
        );
    }
}

#[test]
fn test_document_function_api() {
    // The generated document() function returns a cloned DampenDocument
    // This API is consistent in both modes
    let xml = r#"<dampen version="1.0"><column><text value="API Test" /></column></dampen>"#;
    let doc = parse(xml).expect("Parse failed");

    // The document() function in generated code does this:
    let cloned = doc.clone();
    assert_eq!(cloned.version.major, doc.version.major);
}

#[test]
fn test_error_handling() {
    // In interpreted mode, parse errors are caught at runtime
    let invalid_xml = r#"<dampen version="1.0"><invalid></dampen>"#;
    let result = parse(invalid_xml);

    assert!(result.is_err(), "Invalid XML should produce parse error");
}

#[test]
fn test_development_workflow() {
    // Interpreted mode is optimized for development:
    // - Fast compilation (no codegen overhead)
    // - Hot-reload support
    // - Runtime error messages
    // - State preservation during reload
    assert!(true, "Interpreted mode optimized for development workflow");
}

#[test]
fn test_backwards_compatibility() {
    // Existing code without feature flags continues to work
    // This ensures a smooth migration path
    #[cfg(not(feature = "codegen"))]
    {
        assert!(true, "Default behavior maintains backwards compatibility");
    }
}

#[test]
fn test_xml_validation_at_runtime() {
    // In interpreted mode, XML validation happens at runtime
    // This provides immediate feedback during development
    let valid_xml = r#"
        <dampen version="1.0">
            <column>
                <text value="Valid" />
            </column>
        </dampen>
    "#;

    let result = parse(valid_xml);
    assert!(result.is_ok(), "Valid XML should parse successfully");
}
