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

    // Additional invalid format tests for Phase 4 (T033-T034)

    #[test]
    fn parse_version_suffix() {
        let result = parse_version_string("1.0-beta", dummy_span());
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.kind, ParseErrorKind::InvalidValue);
        assert!(err.message.contains("1.0-beta"));
        assert!(err.message.contains("major.minor"));
    }

    #[test]
    fn parse_version_text() {
        let result = parse_version_string("one.zero", dummy_span());
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.kind, ParseErrorKind::InvalidValue);
        assert!(err.message.contains("one.zero"));
        assert!(err.message.contains("major.minor"));
    }

    // Comprehensive error message validation (T035-T036)

    #[test]
    fn error_messages_include_invalid_input() {
        let test_cases = vec![
            "", "1", "1.0.0", "v1.0", "1.x", "-1.0", "1.0-beta", "one.zero",
        ];

        for input in test_cases {
            let result = parse_version_string(input, dummy_span());
            assert!(result.is_err(), "Expected error for input: {}", input);
            let err = result.unwrap_err();

            // For empty string, message should say "cannot be empty"
            // For others, message should contain the invalid input
            if input.is_empty() {
                assert!(err.message.contains("cannot be empty"));
            } else {
                assert!(
                    err.message.contains(input),
                    "Error message should contain invalid input '{}'. Got: {}",
                    input,
                    err.message
                );
            }
        }
    }

    #[test]
    fn error_messages_include_format_suggestion() {
        let test_cases = vec!["1", "1.0.0", "v1.0", "1.x", "1.0-beta"];

        for input in test_cases {
            let result = parse_version_string(input, dummy_span());
            assert!(result.is_err(), "Expected error for input: {}", input);
            let err = result.unwrap_err();

            // Check suggestion exists and contains format hint
            assert!(
                err.suggestion.is_some(),
                "Should have suggestion for input: {}",
                input
            );
            let suggestion = err.suggestion.unwrap();
            assert!(
                suggestion.contains("1.0") || suggestion.contains("version="),
                "Suggestion should include correct format. Got: {}",
                suggestion
            );
        }
    }
}

#[cfg(test)]
mod validate_version_supported_tests {
    use super::*;

    // Supported version tests (T019-T020)

    #[test]
    fn validate_supported_version_1_0() {
        let version = SchemaVersion { major: 1, minor: 0 };
        let result = validate_version_supported(&version, dummy_span());
        assert!(result.is_ok());
    }

    #[test]
    fn validate_supported_version_0_9() {
        let version = SchemaVersion { major: 0, minor: 9 };
        let result = validate_version_supported(&version, dummy_span());
        assert!(result.is_ok());
    }

    // Unsupported version tests (T021-T023)

    #[test]
    fn validate_supported_version_1_1() {
        let version = SchemaVersion { major: 1, minor: 1 };
        let result = validate_version_supported(&version, dummy_span());
        assert!(result.is_ok());
    }

    #[test]
    fn validate_unsupported_version_2_0() {
        let version = SchemaVersion { major: 2, minor: 0 };
        let result = validate_version_supported(&version, dummy_span());
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.kind, ParseErrorKind::UnsupportedVersion);
        assert!(err.message.contains("2.0"));
        assert!(err.message.contains("not supported"));
    }

    #[test]
    fn validate_error_message_includes_versions() {
        let version = SchemaVersion { major: 2, minor: 0 };
        let result = validate_version_supported(&version, dummy_span());
        assert!(result.is_err());
        let err = result.unwrap_err();

        // Check that declared version is in message
        assert!(err.message.contains("2.0"));

        // Check that max supported version is in message
        assert!(err.message.contains("1.1"));

        // Check suggestion exists
        assert!(err.suggestion.is_some());
        let suggestion = err.suggestion.unwrap();
        assert!(suggestion.contains("Upgrade") || suggestion.contains("use version"));
    }
}

#[cfg(test)]
mod parser_integration_tests {
    use super::*;

    // Parser integration tests (T026-T029)

    #[test]
    fn parse_document_with_version_1_1() {
        let xml = r#"<dampen version="1.1" encoding="utf-8"><column><text value="Hello" /></column></dampen>"#;
        let result = parse(xml);
        assert!(result.is_ok());
        let doc = result.unwrap();
        assert_eq!(doc.version.major, 1);
        assert_eq!(doc.version.minor, 1);
    }

    #[test]
    fn parse_document_without_version_defaults() {
        let xml = r#"<dampen encoding="utf-8"><column><text value="Hello" /></column></dampen>"#;
        let result = parse(xml);
        assert!(result.is_ok());
        let doc = result.unwrap();
        assert_eq!(doc.version.major, 1);
        assert_eq!(doc.version.minor, 0);
    }

    #[test]
    fn parse_document_with_unsupported_version() {
        let xml = r#"<dampen version="2.0"><column><text value="Hello" /></column></dampen>"#;
        let result = parse(xml);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.kind, ParseErrorKind::UnsupportedVersion);
        assert!(err.message.contains("2.0"));
        assert!(err.message.contains("not supported"));
    }

    #[test]
    fn parse_document_with_invalid_version_format() {
        let xml = r#"<dampen version="invalid"><column><text value="Hello" /></column></dampen>"#;
        let result = parse(xml);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.kind, ParseErrorKind::InvalidValue);
        assert!(err.message.contains("invalid"));
    }
}

#[cfg(test)]
mod widget_minimum_version_tests {
    use dampen_core::WidgetKind;

    // Widget minimum version tests (T052-T054)

    #[test]
    fn widget_kind_column_minimum_version() {
        let widget = WidgetKind::Column;
        let min_version = widget.minimum_version();
        assert_eq!(min_version.major, 1);
        assert_eq!(min_version.minor, 0);
    }

    #[test]
    fn widget_kind_radio_minimum_version() {
        let widget = WidgetKind::Radio;
        let min_version = widget.minimum_version();
        assert_eq!(min_version.major, 1);
        assert_eq!(min_version.minor, 0);
    }

    #[test]
    fn widget_kind_canvas_minimum_version() {
        let widget = WidgetKind::Canvas;
        let min_version = widget.minimum_version();
        assert_eq!(min_version.major, 1);
        assert_eq!(min_version.minor, 1); // Canvas is a v1.1 widget
    }

    // Verify all v1.0 widgets return v1.0 (Canvas is v1.1)
    #[test]
    fn all_v1_0_widgets_minimum_version() {
        let widgets = vec![
            WidgetKind::Column,
            WidgetKind::Row,
            WidgetKind::Text,
            WidgetKind::Button,
            WidgetKind::Checkbox,
            WidgetKind::TextInput,
            WidgetKind::Slider,
            WidgetKind::ProgressBar,
            WidgetKind::Image,
            WidgetKind::Svg,
            WidgetKind::Container,
            WidgetKind::Scrollable,
            WidgetKind::Space,
            WidgetKind::Toggler,
            WidgetKind::PickList,
            WidgetKind::Radio,
            WidgetKind::ComboBox,
            WidgetKind::Stack,
            WidgetKind::Grid,
            WidgetKind::Tooltip,
        ];

        for widget in widgets {
            let min_version = widget.minimum_version();
            assert_eq!(
                min_version.major, 1,
                "Widget {:?} should have major version 1",
                widget
            );
            assert_eq!(
                min_version.minor, 0,
                "Widget {:?} should have minor version 0",
                widget
            );
        }
    }
}

#[cfg(test)]
mod widget_version_validation_tests {
    use dampen_core::{parse, validate_widget_versions};

    // Widget version validation tests (T068)

    #[test]
    fn validate_all_v1_0_widgets_pass() {
        let xml = r#"<dampen version="1.1" encoding="utf-8">
            <column spacing="10">
                <text value="test" />
                <button label="Click" on_click="handler" />
                <row>
                    <checkbox checked="{active}" />
                    <text_input value="{input}" />
                </row>
            </column>
        </dampen>"#;
        let doc = parse(xml).unwrap();
        let warnings = validate_widget_versions(&doc);
        assert!(
            warnings.is_empty(),
            "No warnings for v1.0 widgets in v1.0 document"
        );
    }

    #[test]
    fn validate_canvas_in_v1_0_document_produces_warning() {
        // Use implicit v1.0 document (no <dampen> tag) to bypass strict validation in parse()
        // so we can test the warning generation
        let xml = r#"
            <column>
                <canvas width="400" height="200" program="{chart}" />
            </column>
        "#;
        let doc = parse(xml).unwrap();
        let warnings = validate_widget_versions(&doc);

        assert_eq!(
            warnings.len(),
            1,
            "Canvas should produce 1 warning in v1.0 document"
        );

        let warning = &warnings[0];
        assert_eq!(warning.widget_kind, dampen_core::WidgetKind::Canvas);
        assert_eq!(warning.declared_version.major, 1);
        assert_eq!(warning.declared_version.minor, 0);
        assert_eq!(warning.required_version.major, 1);
        assert_eq!(warning.required_version.minor, 1);
    }

    #[test]
    fn validate_canvas_in_v1_1_document_no_warning() {
        let xml = r#"<dampen version="1.1" encoding="utf-8">
            <canvas width="400" height="200" program="{chart}" />
        </dampen>"#;
        let doc = parse(xml).unwrap();
        let warnings = validate_widget_versions(&doc);

        // Should produce no warnings because document is v1.1
        assert!(warnings.is_empty());
    }

    #[test]
    fn validate_nested_canvas_produces_warning() {
        // Use implicit v1.0 document (no <dampen> tag)
        let xml = r#"
            <column>
                <row>
                    <container>
                        <canvas width="200" height="100" program="{mini_chart}" />
                    </container>
                </row>
                <canvas width="400" height="200" program="{main_chart}" />
            </column>
        "#;
        let doc = parse(xml).unwrap();
        let warnings = validate_widget_versions(&doc);

        assert_eq!(warnings.len(), 2, "Should warn about both Canvas widgets");
        for warning in warnings {
            assert_eq!(warning.widget_kind, dampen_core::WidgetKind::Canvas);
        }
    }

    #[test]
    fn validate_warning_format_message() {
        // Use implicit v1.0 document (no <dampen> tag)
        let xml = r#"<canvas width="400" height="200" program="{chart}" />"#;
        let doc = parse(xml).unwrap();
        let warnings = validate_widget_versions(&doc);

        let warning = &warnings[0];
        let message = warning.format_message();

        assert!(
            message.contains("canvas"),
            "Message should mention widget name"
        );
        assert!(
            message.contains("1.1"),
            "Message should mention required version"
        );
        assert!(
            message.contains("1.0"),
            "Message should mention declared version"
        );
    }

    #[test]
    fn validate_warning_suggestion() {
        // Use implicit v1.0 document (no <dampen> tag)
        let xml = r#"<canvas width="400" height="200" program="{chart}" />"#;
        let doc = parse(xml).unwrap();
        let warnings = validate_widget_versions(&doc);

        let warning = &warnings[0];
        let suggestion = warning.suggestion();

        assert!(
            suggestion.contains("1.1"),
            "Suggestion should mention required version"
        );
        assert!(
            suggestion.contains("dampen version"),
            "Suggestion should show how to update"
        );
    }

    #[test]
    fn validate_empty_document_no_warnings() {
        let xml = r#"<dampen version="1.1" encoding="utf-8">
            <column />
        </dampen>"#;
        let doc = parse(xml).unwrap();
        let warnings = validate_widget_versions(&doc);

        assert!(
            warnings.is_empty(),
            "Empty document should have no warnings"
        );
    }
}
