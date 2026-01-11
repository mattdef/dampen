//! Contract tests for XML schema version parsing and validation
//!
//! Tests the behavior of version string parsing and validation functions
//! against the contracts defined in specs/001-schema-version-validation/contracts/version-api.md

use dampen_core::{
    ParseErrorKind, SchemaVersion, Span, parse, parse_version_string, validate_version_supported,
};

/// Helper function to create a dummy span for testing
fn dummy_span() -> Span {
    Span::new(0, 0, 0, 0)
}

#[cfg(test)]
mod parse_version_string_tests {
    use super::*;

    // Valid input tests (T007-T010)

    #[test]
    fn parse_valid_version_1_0() {
        let result = parse_version_string("1.0", dummy_span());
        assert!(result.is_ok());
        let version = result.unwrap();
        assert_eq!(version.major, 1);
        assert_eq!(version.minor, 0);
    }

    #[test]
    fn parse_valid_version_0_1() {
        let result = parse_version_string("0.1", dummy_span());
        assert!(result.is_ok());
        let version = result.unwrap();
        assert_eq!(version.major, 0);
        assert_eq!(version.minor, 1);
    }

    #[test]
    fn parse_valid_version_with_whitespace() {
        let result = parse_version_string(" 1.0 ", dummy_span());
        assert!(result.is_ok());
        let version = result.unwrap();
        assert_eq!(version.major, 1);
        assert_eq!(version.minor, 0);
    }

    #[test]
    fn parse_valid_version_leading_zeros() {
        let result = parse_version_string("01.00", dummy_span());
        assert!(result.is_ok());
        let version = result.unwrap();
        assert_eq!(version.major, 1);
        assert_eq!(version.minor, 0);
    }

    // Invalid input tests (T011-T016)

    #[test]
    fn parse_invalid_empty_string() {
        let result = parse_version_string("", dummy_span());
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
        assert!(err.message.contains("1"));
        assert!(err.message.contains("major.minor"));
    }

    #[test]
    fn parse_invalid_triple_version() {
        let result = parse_version_string("1.0.0", dummy_span());
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.kind, ParseErrorKind::InvalidValue);
        assert!(err.message.contains("1.0.0"));
    }

    #[test]
    fn parse_invalid_prefix() {
        let result = parse_version_string("v1.0", dummy_span());
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.kind, ParseErrorKind::InvalidValue);
        assert!(err.message.contains("v1.0"));
    }

    #[test]
    fn parse_invalid_non_numeric() {
        let result = parse_version_string("1.x", dummy_span());
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.kind, ParseErrorKind::InvalidValue);
        assert!(err.message.contains("1.x"));
    }

    #[test]
    fn parse_invalid_negative() {
        let result = parse_version_string("-1.0", dummy_span());
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.kind, ParseErrorKind::InvalidValue);
        assert!(err.message.contains("-1.0"));
    }
}

#[cfg(test)]
mod validate_version_supported_tests {
    use super::*;

    // Validation tests will be added here
}

#[cfg(test)]
mod parser_integration_tests {
    use super::*;

    // Integration tests for full document parsing will be added here
}

#[cfg(test)]
mod widget_minimum_version_tests {
    use super::*;

    // Widget minimum version tests will be added here
}
