# Contract: Version Parsing API

**Feature**: 001-schema-version-validation  
**Date**: 2026-01-11

## Overview

This contract defines the expected behavior of the version parsing and validation functions in the Dampen parser.

## Contract: parse_version_string

### Valid Inputs

| Input | Expected Output |
|-------|-----------------|
| `"1.0"` | `Ok(SchemaVersion { major: 1, minor: 0 })` |
| `"0.1"` | `Ok(SchemaVersion { major: 0, minor: 1 })` |
| `"2.5"` | `Ok(SchemaVersion { major: 2, minor: 5 })` |
| `" 1.0 "` | `Ok(SchemaVersion { major: 1, minor: 0 })` (trimmed) |
| `"01.00"` | `Ok(SchemaVersion { major: 1, minor: 0 })` (leading zeros parsed) |
| `"99.99"` | `Ok(SchemaVersion { major: 99, minor: 99 })` |

### Invalid Inputs

| Input | Expected Error | Error Kind |
|-------|----------------|------------|
| `""` | "Version attribute cannot be empty" | InvalidValue |
| `" "` | "Version attribute cannot be empty" | InvalidValue |
| `"1"` | "Invalid version format '1'. Expected 'major.minor' (e.g., '1.0')" | InvalidValue |
| `"1.0.0"` | "Invalid version format '1.0.0'. Expected 'major.minor' (e.g., '1.0')" | InvalidValue |
| `"v1.0"` | "Invalid version format 'v1.0'. Expected 'major.minor' (e.g., '1.0')" | InvalidValue |
| `"1.x"` | "Invalid version format '1.x'. Expected 'major.minor' (e.g., '1.0')" | InvalidValue |
| `"-1.0"` | "Invalid version format '-1.0'. Expected 'major.minor' (e.g., '1.0')" | InvalidValue |
| `"1.0-beta"` | "Invalid version format '1.0-beta'. Expected 'major.minor' (e.g., '1.0')" | InvalidValue |
| `"one.zero"` | "Invalid version format 'one.zero'. Expected 'major.minor' (e.g., '1.0')" | InvalidValue |

### Error Requirements

All errors MUST include:
- The invalid input value in the message
- A suggestion with the correct format
- The source span for error positioning

## Contract: validate_version_supported

### Supported Versions (MAX = 1.0)

| Input | Expected Output |
|-------|-----------------|
| `SchemaVersion { major: 1, minor: 0 }` | `Ok(())` |
| `SchemaVersion { major: 0, minor: 9 }` | `Ok(())` |
| `SchemaVersion { major: 0, minor: 0 }` | `Ok(())` |

### Unsupported Versions (MAX = 1.0)

| Input | Expected Error | Error Kind |
|-------|----------------|------------|
| `SchemaVersion { major: 1, minor: 1 }` | "Schema version 1.1 is not supported. Maximum supported version: 1.0" | UnsupportedVersion |
| `SchemaVersion { major: 2, minor: 0 }` | "Schema version 2.0 is not supported. Maximum supported version: 1.0" | UnsupportedVersion |
| `SchemaVersion { major: 99, minor: 0 }` | "Schema version 99.0 is not supported. Maximum supported version: 1.0" | UnsupportedVersion |

### Error Requirements

All unsupported version errors MUST include:
- The declared version in the message
- The maximum supported version
- Suggestion: "Upgrade dampen-core to support vX.Y, or use version=\"1.0\""

## Contract: parse() Function Behavior

### Documents with version Attribute

| XML Input | Expected Version in DampenDocument |
|-----------|-----------------------------------|
| `<dampen version="1.0"><column/></dampen>` | `SchemaVersion { major: 1, minor: 0 }` |
| `<dampen version="0.5"><column/></dampen>` | `SchemaVersion { major: 0, minor: 5 }` |

### Documents without version Attribute

| XML Input | Expected Version in DampenDocument |
|-----------|-----------------------------------|
| `<dampen><column/></dampen>` | `SchemaVersion { major: 1, minor: 0 }` (default) |
| `<dampen xmlns="..."><column/></dampen>` | `SchemaVersion { major: 1, minor: 0 }` (default) |

### Documents with Invalid version Attribute

| XML Input | Expected Result |
|-----------|-----------------|
| `<dampen version="2.0"><column/></dampen>` | `Err(ParseError)` with UnsupportedVersion |
| `<dampen version="invalid"><column/></dampen>` | `Err(ParseError)` with InvalidValue |

### Direct Widget Parsing (No dampen Wrapper)

| XML Input | Expected Version in DampenDocument |
|-----------|-----------------------------------|
| `<column><text value="Hi"/></column>` | `SchemaVersion { major: 1, minor: 0 }` (default) |

## Contract: WidgetKind::minimum_version()

### Current Behavior (v1.0)

| Widget Kind | Expected Minimum Version |
|-------------|-------------------------|
| Column | `SchemaVersion { major: 1, minor: 0 }` |
| Row | `SchemaVersion { major: 1, minor: 0 }` |
| Text | `SchemaVersion { major: 1, minor: 0 }` |
| Button | `SchemaVersion { major: 1, minor: 0 }` |
| Grid | `SchemaVersion { major: 1, minor: 0 }` |
| Tooltip | `SchemaVersion { major: 1, minor: 0 }` |
| ComboBox | `SchemaVersion { major: 1, minor: 0 }` |
| Canvas | `SchemaVersion { major: 1, minor: 0 }` |
| *All other widgets* | `SchemaVersion { major: 1, minor: 0 }` |

### Future Behavior (v1.1+)

When v1.1 is released, certain widgets may require higher versions:
- Implementation will update the match arms in `minimum_version()`
- Existing v1.0 widgets will continue to return v1.0

## Test Implementation Pattern

```rust
#[cfg(test)]
mod version_contract_tests {
    use super::*;

    // Valid parsing tests
    #[test]
    fn parse_valid_version_1_0() {
        let result = parse_version_string("1.0", dummy_span());
        assert_eq!(result.unwrap(), SchemaVersion { major: 1, minor: 0 });
    }

    // Invalid parsing tests
    #[test]
    fn parse_invalid_format_single_number() {
        let result = parse_version_string("1", dummy_span());
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err.kind, ParseErrorKind::InvalidValue));
        assert!(err.message.contains("1.0"));
    }

    // Validation tests
    #[test]
    fn validate_supported_version() {
        let version = SchemaVersion { major: 1, minor: 0 };
        assert!(validate_version_supported(&version, dummy_span()).is_ok());
    }

    #[test]
    fn validate_unsupported_future_version() {
        let version = SchemaVersion { major: 2, minor: 0 };
        let result = validate_version_supported(&version, dummy_span());
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err.kind, ParseErrorKind::UnsupportedVersion));
    }

    // Integration tests
    #[test]
    fn parse_document_with_version() {
        let xml = r#"<dampen version="1.0"><column/></dampen>"#;
        let doc = parse(xml).unwrap();
        assert_eq!(doc.version, SchemaVersion { major: 1, minor: 0 });
    }

    #[test]
    fn parse_document_without_version_defaults() {
        let xml = r#"<dampen><column/></dampen>"#;
        let doc = parse(xml).unwrap();
        assert_eq!(doc.version, SchemaVersion { major: 1, minor: 0 });
    }
}
```
