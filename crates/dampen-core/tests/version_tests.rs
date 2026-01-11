//! Contract tests for XML schema version parsing and validation
//!
//! These tests implement the contracts defined in:
//! `specs/001-schema-version-validation/contracts/version-api.md`

use dampen_core::ir::{SchemaVersion, Span};
use dampen_core::parse;
use dampen_core::parser::error::ParseErrorKind;
use dampen_core::parser::{
    MAX_SUPPORTED_VERSION, parse_version_string, validate_version_supported,
};

/// Create a dummy span for testing
fn dummy_span() -> Span {
    Span::new(0, 0, 1, 1)
}

// ============================================================================
// Contract: parse_version_string - Valid Inputs
// ============================================================================

#[test]
fn parse_valid_version_1_0() {
    let result = parse_version_string("1.0", dummy_span());
    assert_eq!(result.unwrap(), SchemaVersion { major: 1, minor: 0 });
}

#[test]
fn parse_valid_version_0_1() {
    let result = parse_version_string("0.1", dummy_span());
    assert_eq!(result.unwrap(), SchemaVersion { major: 0, minor: 1 });
}

#[test]
fn parse_valid_version_2_5() {
    let result = parse_version_string("2.5", dummy_span());
    assert_eq!(result.unwrap(), SchemaVersion { major: 2, minor: 5 });
}

#[test]
fn parse_valid_version_with_whitespace() {
    let result = parse_version_string(" 1.0 ", dummy_span());
    assert_eq!(result.unwrap(), SchemaVersion { major: 1, minor: 0 });
}

#[test]
fn parse_valid_version_leading_zeros() {
    let result = parse_version_string("01.00", dummy_span());
    assert_eq!(result.unwrap(), SchemaVersion { major: 1, minor: 0 });
}

#[test]
fn parse_valid_version_large_numbers() {
    let result = parse_version_string("99.99", dummy_span());
    assert_eq!(
        result.unwrap(),
        SchemaVersion {
            major: 99,
            minor: 99
        }
    );
}

// ============================================================================
// Contract: parse_version_string - Invalid Inputs
// ============================================================================

#[test]
fn parse_invalid_empty_string() {
    let result = parse_version_string("", dummy_span());
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.kind, ParseErrorKind::InvalidValue);
    assert!(err.message.contains("cannot be empty"));
}

#[test]
fn parse_invalid_whitespace_only() {
    let result = parse_version_string("   ", dummy_span());
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.kind, ParseErrorKind::InvalidValue);
    assert!(err.message.contains("cannot be empty"));
}

#[test]
fn parse_invalid_single_number() {
    let result = parse_version_string("1", dummy_span());
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.kind, ParseErrorKind::InvalidValue);
    assert!(err.message.contains("Invalid version format"));
    assert!(err.message.contains("'1'"));
    assert!(err.suggestion.as_ref().unwrap().contains("1.0"));
}

#[test]
fn parse_invalid_triple_version() {
    let result = parse_version_string("1.0.0", dummy_span());
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.kind, ParseErrorKind::InvalidValue);
    assert!(err.message.contains("Invalid version format"));
    assert!(err.message.contains("'1.0.0'"));
}

#[test]
fn parse_invalid_prefix() {
    let result = parse_version_string("v1.0", dummy_span());
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.kind, ParseErrorKind::InvalidValue);
    assert!(err.message.contains("Invalid version format"));
    assert!(err.message.contains("'v1.0'"));
}

#[test]
fn parse_invalid_non_numeric_minor() {
    let result = parse_version_string("1.x", dummy_span());
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.kind, ParseErrorKind::InvalidValue);
    assert!(err.message.contains("'1.x'"));
}

#[test]
fn parse_invalid_negative() {
    let result = parse_version_string("-1.0", dummy_span());
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.kind, ParseErrorKind::InvalidValue);
}

#[test]
fn parse_invalid_suffix() {
    let result = parse_version_string("1.0-beta", dummy_span());
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.kind, ParseErrorKind::InvalidValue);
    assert!(err.message.contains("'1.0-beta'"));
}

#[test]
fn parse_invalid_text() {
    let result = parse_version_string("one.zero", dummy_span());
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.kind, ParseErrorKind::InvalidValue);
}

// ============================================================================
// Contract: validate_version_supported - Supported Versions
// ============================================================================

#[test]
fn validate_supported_version_1_0() {
    let version = SchemaVersion { major: 1, minor: 0 };
    assert!(validate_version_supported(&version, dummy_span()).is_ok());
}

#[test]
fn validate_supported_version_0_9() {
    let version = SchemaVersion { major: 0, minor: 9 };
    assert!(validate_version_supported(&version, dummy_span()).is_ok());
}

#[test]
fn validate_supported_version_0_0() {
    let version = SchemaVersion { major: 0, minor: 0 };
    assert!(validate_version_supported(&version, dummy_span()).is_ok());
}

// ============================================================================
// Contract: validate_version_supported - Unsupported Versions
// ============================================================================

#[test]
fn validate_unsupported_version_1_1() {
    let version = SchemaVersion { major: 1, minor: 1 };
    let result = validate_version_supported(&version, dummy_span());
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.kind, ParseErrorKind::UnsupportedVersion);
    assert!(err.message.contains("1.1"));
    assert!(err.message.contains("1.0"));
}

#[test]
fn validate_unsupported_version_2_0() {
    let version = SchemaVersion { major: 2, minor: 0 };
    let result = validate_version_supported(&version, dummy_span());
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.kind, ParseErrorKind::UnsupportedVersion);
    assert!(err.message.contains("2.0"));
    assert!(err.message.contains("Maximum supported version: 1.0"));
}

#[test]
fn validate_unsupported_version_99_0() {
    let version = SchemaVersion {
        major: 99,
        minor: 0,
    };
    let result = validate_version_supported(&version, dummy_span());
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.kind, ParseErrorKind::UnsupportedVersion);
    assert!(err.message.contains("99.0"));
}

#[test]
fn validate_unsupported_version_has_suggestion() {
    let version = SchemaVersion { major: 2, minor: 0 };
    let result = validate_version_supported(&version, dummy_span());
    let err = result.unwrap_err();
    assert!(err.suggestion.is_some());
    let suggestion = err.suggestion.unwrap();
    assert!(suggestion.contains("Upgrade dampen-core") || suggestion.contains("use version"));
}

// ============================================================================
// Contract: parse() Function - Documents with version Attribute
// ============================================================================

#[test]
fn parse_document_with_version_1_0() {
    let xml = r#"<dampen version="1.0"><column/></dampen>"#;
    let doc = parse(xml).unwrap();
    assert_eq!(doc.version, SchemaVersion { major: 1, minor: 0 });
}

#[test]
fn parse_document_with_version_0_5() {
    let xml = r#"<dampen version="0.5"><column/></dampen>"#;
    let doc = parse(xml).unwrap();
    assert_eq!(doc.version, SchemaVersion { major: 0, minor: 5 });
}

// ============================================================================
// Contract: parse() Function - Documents without version Attribute
// ============================================================================

#[test]
fn parse_document_without_version_defaults_to_1_0() {
    let xml = r#"<dampen><column/></dampen>"#;
    let doc = parse(xml).unwrap();
    assert_eq!(doc.version, SchemaVersion { major: 1, minor: 0 });
}

#[test]
fn parse_document_with_xmlns_defaults_to_1_0() {
    let xml = r#"<dampen xmlns="http://example.com"><column/></dampen>"#;
    let doc = parse(xml).unwrap();
    assert_eq!(doc.version, SchemaVersion { major: 1, minor: 0 });
}

// ============================================================================
// Contract: parse() Function - Documents with Invalid version Attribute
// ============================================================================

#[test]
fn parse_document_with_unsupported_version() {
    let xml = r#"<dampen version="2.0"><column/></dampen>"#;
    let result = parse(xml);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.kind, ParseErrorKind::UnsupportedVersion);
}

#[test]
fn parse_document_with_invalid_version_format() {
    let xml = r#"<dampen version="invalid"><column/></dampen>"#;
    let result = parse(xml);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.kind, ParseErrorKind::InvalidValue);
}

// ============================================================================
// Contract: Direct Widget Parsing (No dampen Wrapper)
// ============================================================================

#[test]
fn parse_direct_widget_defaults_to_1_0() {
    let xml = r#"<column><text value="Hi"/></column>"#;
    let doc = parse(xml).unwrap();
    assert_eq!(doc.version, SchemaVersion { major: 1, minor: 0 });
}

// ============================================================================
// Constant Tests
// ============================================================================

#[test]
fn max_supported_version_is_1_0() {
    assert_eq!(MAX_SUPPORTED_VERSION.major, 1);
    assert_eq!(MAX_SUPPORTED_VERSION.minor, 0);
}
