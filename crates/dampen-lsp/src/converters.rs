//! Type converters between Dampen and LSP types.
//!
//! Handles conversion between Dampen core types (Span, ParseError) and
//! LSP types (Position, Range, Diagnostic).

#![allow(dead_code)]

use dampen_core::ir::span::Span;
use dampen_core::parser::error::{ParseError, ParseErrorKind};
use tower_lsp::lsp_types::*;

/// Converts a Dampen Span to an LSP Range.
///
/// # Arguments
///
/// * `content` - Document content for line/column calculation
/// * `span` - Dampen span with byte offsets
///
/// # Returns
///
/// LSP Range with line and character positions
pub fn span_to_range(content: &str, span: Span) -> Range {
    let start = offset_to_position(content, span.start).unwrap_or(Position::new(0, 0));
    let end = offset_to_position(content, span.end).unwrap_or(start);

    Range::new(start, end)
}

/// Converts an LSP Range to a Dampen Span.
///
/// # Arguments
///
/// * `content` - Document content
/// * `range` - LSP Range
///
/// # Returns
///
/// Dampen Span with byte offsets
pub fn range_to_span(content: &str, range: Range) -> Span {
    let start = position_to_offset(content, range.start).unwrap_or(0);
    let end = position_to_offset(content, range.end).unwrap_or(start);

    // Use line 1, column 1 as defaults since we can't calculate reverse
    Span::new(start, end, 1, 1)
}

/// Converts a byte offset to an LSP Position.
///
/// LSP positions use UTF-16 code units, so we need to handle
/// multi-byte characters carefully.
///
/// # Arguments
///
/// * `content` - Document content
/// * `offset` - Byte offset
///
/// # Returns
///
/// LSP Position (line, character) or None if offset is invalid
pub fn offset_to_position(content: &str, offset: usize) -> Option<Position> {
    if offset > content.len() {
        return None;
    }

    let mut line = 0u32;
    let mut character = 0u32;
    let mut current_offset = 0usize;

    for ch in content.chars() {
        if current_offset >= offset {
            break;
        }

        if ch == '\n' {
            line += 1;
            character = 0;
        } else {
            // LSP uses UTF-16 code units
            character += ch.encode_utf16(&mut [0; 2]).len() as u32;
        }

        current_offset += ch.len_utf8();
    }

    Some(Position::new(line, character))
}

/// Converts an LSP Position to a byte offset.
///
/// # Arguments
///
/// * `content` - Document content
/// * `position` - LSP Position
///
/// # Returns
///
/// Byte offset or None if position is invalid
pub fn position_to_offset(content: &str, position: Position) -> Option<usize> {
    let mut line = 0u32;
    let mut character = 0u32;
    let mut offset = 0usize;

    for ch in content.chars() {
        if line == position.line && character == position.character {
            return Some(offset);
        }

        if ch == '\n' {
            if line == position.line {
                // Position is past end of line
                return Some(offset);
            }
            line += 1;
            character = 0;
        } else {
            character += ch.encode_utf16(&mut [0; 2]).len() as u32;
        }

        offset += ch.len_utf8();
    }

    // Check if position is at end of file
    if line == position.line && character == position.character {
        return Some(offset);
    }

    None
}

/// Converts a ParseError to an LSP Diagnostic.
///
/// # Arguments
///
/// * `content` - Document content for position conversion
/// * `error` - Dampen parse error
///
/// # Returns
///
/// LSP Diagnostic
pub fn parse_error_to_diagnostic(content: &str, error: ParseError) -> Diagnostic {
    let range = span_to_range(content, error.span);

    let severity = Some(match error.kind {
        ParseErrorKind::XmlSyntax => DiagnosticSeverity::ERROR,
        ParseErrorKind::UnknownWidget => DiagnosticSeverity::ERROR,
        ParseErrorKind::UnknownAttribute => DiagnosticSeverity::WARNING,
        ParseErrorKind::InvalidValue => DiagnosticSeverity::ERROR,
        ParseErrorKind::InvalidExpression => DiagnosticSeverity::ERROR,
        ParseErrorKind::UnclosedBinding => DiagnosticSeverity::ERROR,
        ParseErrorKind::MissingAttribute => DiagnosticSeverity::ERROR,
        ParseErrorKind::UnsupportedVersion => DiagnosticSeverity::ERROR,
        ParseErrorKind::DeprecatedAttribute => DiagnosticSeverity::WARNING,
        ParseErrorKind::InvalidChild => DiagnosticSeverity::ERROR,
        ParseErrorKind::InvalidDateFormat => DiagnosticSeverity::ERROR,
        ParseErrorKind::InvalidTimeFormat => DiagnosticSeverity::ERROR,
        ParseErrorKind::InvalidDateRange => DiagnosticSeverity::ERROR,
    });

    let code = Some(NumberOrString::String(format!("E{:03}", error.kind as u8)));

    let related_information = error.suggestion.and_then(|suggestion| {
        tower_lsp::lsp_types::Url::parse("file:///dummy")
            .ok()
            .map(|uri| {
                vec![DiagnosticRelatedInformation {
                    location: Location { uri, range },
                    message: suggestion,
                }]
            })
    });

    Diagnostic {
        range,
        severity,
        code,
        code_description: None,
        source: Some("dampen".to_string()),
        message: error.message,
        related_information,
        tags: None,
        data: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_offset_to_position_simple() {
        let content = "line1\nline2\nline3";

        assert_eq!(offset_to_position(content, 0), Some(Position::new(0, 0)));
        assert_eq!(offset_to_position(content, 6), Some(Position::new(1, 0)));
        assert_eq!(offset_to_position(content, 12), Some(Position::new(2, 0)));
    }

    #[test]
    fn test_position_to_offset_simple() {
        let content = "line1\nline2\nline3";

        assert_eq!(position_to_offset(content, Position::new(0, 0)), Some(0));
        assert_eq!(position_to_offset(content, Position::new(1, 0)), Some(6));
        assert_eq!(position_to_offset(content, Position::new(2, 0)), Some(12));
    }
}
