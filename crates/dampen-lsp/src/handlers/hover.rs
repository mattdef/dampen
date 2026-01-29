//! Hover request handler.
//!
//! Provides contextual documentation on hover.

#![allow(dead_code)]

use tower_lsp::lsp_types::*;

use crate::analyzer::{Analyzer, CompletionContext};
use crate::converters::position_to_offset;
use crate::document::DocumentState;
use crate::schema_data::{get_attribute_documentation, get_widget_documentation};

/// Handles hover requests.
///
/// Returns hover information for the element at the given position.
/// Analyzes the document to determine if the user is hovering over a widget,
/// attribute, or value, and returns appropriate documentation.
///
/// # Arguments
///
/// * `doc` - The document state
/// * `position` - Cursor position
///
/// # Returns
///
/// Optional hover information with Markdown documentation
///
/// # Performance
///
/// This function completes within 200ms as per SC-004.
pub fn hover(doc: &DocumentState, position: Position) -> Option<Hover> {
    let analyzer = Analyzer::new();

    // Get completion context to understand what we're hovering over
    let context = analyzer.get_completion_context(doc, position);

    match context {
        CompletionContext::WidgetName => {
            // Try to find the widget name at this position
            if let Some(widget_name) = analyzer.find_widget_at_position(doc, position) {
                return hover_widget(&widget_name, position);
            }
        }
        CompletionContext::AttributeName { widget } => {
            // Check if we're hovering over an attribute name
            if let Some((_, attr_name)) = analyzer.find_attribute_at_position(doc, position) {
                return hover_attribute(&widget, &attr_name, position);
            }
            // If not on a specific attribute, show widget documentation
            return hover_widget(&widget, position);
        }
        CompletionContext::AttributeValue { widget, attribute } => {
            // We're inside an attribute value - show value documentation
            return hover_value(&widget, &attribute, position, doc);
        }
        CompletionContext::BindingExpression => {
            // Hovering over a binding expression
            return Some(Hover {
                contents: HoverContents::Markup(MarkupContent {
                    kind: MarkupKind::Markdown,
                    value: "**Binding Expression**\n\n\
Dynamic value bound to application state.\n\n\
Syntax: `{|expression|}`"
                        .to_string(),
                }),
                range: Some(Range {
                    start: position,
                    end: Position {
                        line: position.line,
                        character: position.character + 1,
                    },
                }),
            });
        }
        CompletionContext::Unknown => {
            // Try to detect widget or attribute without context
            if let Some(widget_name) = analyzer.find_widget_at_position(doc, position) {
                // Check if we're on an attribute
                if let Some((_, attr_name)) = analyzer.find_attribute_at_position(doc, position) {
                    return hover_attribute(&widget_name, &attr_name, position);
                }
                return hover_widget(&widget_name, position);
            }
        }
    }

    None
}

/// Generates hover information for a widget.
///
/// Retrieves widget documentation from the schema data and formats it
/// as Markdown for display in the editor.
///
/// # Arguments
///
/// * `widget_name` - Name of the widget
/// * `position` - Cursor position for range calculation
///
/// # Returns
///
/// Hover information with widget documentation in Markdown format
fn hover_widget(widget_name: &str, position: Position) -> Option<Hover> {
    let documentation = get_widget_documentation(widget_name)?;

    // Calculate range for the widget name
    // This helps the editor highlight the correct token
    let range = Some(Range {
        start: Position {
            line: position.line,
            character: position.character.saturating_sub(widget_name.len() as u32),
        },
        end: Position {
            line: position.line,
            character: position.character + 1,
        },
    });

    Some(Hover {
        contents: HoverContents::Markup(MarkupContent {
            kind: MarkupKind::Markdown,
            value: documentation.to_string(),
        }),
        range,
    })
}

/// Generates hover information for an attribute.
///
/// Retrieves attribute documentation from the schema data and formats it
/// as Markdown for display in the editor.
///
/// # Arguments
///
/// * `widget_name` - Name of the containing widget
/// * `attr_name` - Name of the attribute
/// * `position` - Cursor position for range calculation
///
/// # Returns
///
/// Hover information with attribute documentation in Markdown format
fn hover_attribute(widget_name: &str, attr_name: &str, position: Position) -> Option<Hover> {
    let documentation = get_attribute_documentation(widget_name, attr_name)?;

    // Calculate range for the attribute name
    let range = Some(Range {
        start: Position {
            line: position.line,
            character: position.character.saturating_sub(attr_name.len() as u32),
        },
        end: position,
    });

    Some(Hover {
        contents: HoverContents::Markup(MarkupContent {
            kind: MarkupKind::Markdown,
            value: documentation.to_string(),
        }),
        range,
    })
}

/// Generates hover information for an attribute value.
///
/// Provides context-aware documentation for attribute values based on
/// the attribute type (boolean, color, enum, etc.).
///
/// # Arguments
///
/// * `widget_name` - Name of the containing widget
/// * `attr_name` - Name of the attribute
/// * `position` - Cursor position
/// * `doc` - Document state to extract the current value
///
/// # Returns
///
/// Hover information with value documentation in Markdown format
fn hover_value(
    widget_name: &str,
    attr_name: &str,
    position: Position,
    doc: &DocumentState,
) -> Option<Hover> {
    // Get the current value at the position
    let offset = position_to_offset(&doc.content, position)?;

    // Extract the value from the document
    // Look for the value between quotes around the cursor position
    let value = extract_value_at_position(&doc.content, offset);

    // Generate documentation based on attribute type
    let documentation = match attr_name {
        "enabled" | "checked" | "visible" | "show" | "toggled" | "selected" | "active"
        | "password" | "close_on_select" | "use_24h" | "show_seconds" | "show_alpha" => {
            format!(
                "**{}** - Boolean Value\n\n\
Type: `boolean`\n\n\
Current value: `{}`\n\n\
Valid values:\n\
- `true` - Enabled/on\n\
- `false` - Disabled/off",
                attr_name,
                value.as_deref().unwrap_or("unknown")
            )
        }
        "align" | "align_items" | "align_self" | "align_x" | "align_y" => "**Alignment Value**\n\n\
Type: `enum`\n\n\
Valid values:\n\
- `start` - Align to start (left/top)\n\
- `center` - Center alignment\n\
- `end` - Align to end (right/bottom)\n\
- `stretch` - Stretch to fill available space"
            .to_string(),
        "justify_content" => "**Justify Content Value**\n\n\
Type: `enum`\n\n\
Valid values:\n\
- `start` - Pack items at start\n\
- `center` - Pack items at center\n\
- `end` - Pack items at end\n\
- `space_between` - Even space between items\n\
- `space_around` - Even space around items\n\
- `space_evenly` - Truly even spacing"
            .to_string(),
        "direction" => "**Direction Value**\n\n\
Type: `enum`\n\n\
Valid values:\n\
- `row` - Left to right\n\
- `column` - Top to bottom\n\
- `row_reverse` - Right to left\n\
- `column_reverse` - Bottom to top"
            .to_string(),
        _ if attr_name.contains("color")
            || attr_name == "background"
            || attr_name == "fill"
            || attr_name == "stroke" =>
        {
            format!(
                "**{}** - Color Value\n\n\
Type: `color`\n\n\
Current value: `{}`\n\n\
Valid formats:\n\
- Hex: `#FF5733` or `#FF5733FF` (with alpha)\n\
- CSS names: `red`, `blue`, `transparent`\n\
- RGBA: `rgba(255, 87, 51, 0.5)`",
                attr_name,
                value.as_deref().unwrap_or("unknown")
            )
        }
        _ => {
            // Default value documentation
            format!(
                "**{}** Attribute Value\n\n\
Widget: `{}`\n\n\
Current value: `{}`\n\n\
See attribute documentation for valid values.",
                attr_name,
                widget_name,
                value.as_deref().unwrap_or("unknown")
            )
        }
    };

    Some(Hover {
        contents: HoverContents::Markup(MarkupContent {
            kind: MarkupKind::Markdown,
            value: documentation,
        }),
        range: Some(Range {
            start: position,
            end: Position {
                line: position.line,
                character: position.character + 1,
            },
        }),
    })
}

/// Extracts the value at a given position in the document.
///
/// Looks for quoted values around the cursor position.
///
/// # Arguments
///
/// * `content` - Document content
/// * `offset` - Byte offset in the document
///
/// # Returns
///
/// The extracted value if found
fn extract_value_at_position(content: &str, offset: usize) -> Option<String> {
    // Get a window around the position
    let start = offset.saturating_sub(100);
    let end = (offset + 100).min(content.len());
    let window = &content[start..end];
    let cursor_in_window = offset - start;

    // Find the nearest quotes around the cursor
    let before_cursor = &window[..cursor_in_window];
    let after_cursor = &window[cursor_in_window..];

    // Find opening quote
    let quote_start = before_cursor.rfind(['"', '\''])?;

    // Find closing quote after cursor
    let quote_end = after_cursor.find(['"', '\''])?;

    // Extract value between quotes
    let value = &window[quote_start + 1..cursor_in_window + quote_end];

    Some(value.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::document::DocumentState;
    use tower_lsp::lsp_types::Url;

    fn create_test_doc(content: &str) -> DocumentState {
        let uri = Url::parse("file:///test.dampen").unwrap();
        DocumentState::new(uri, content.to_string(), 1)
    }

    #[test]
    fn test_hover_widget() {
        let doc = create_test_doc("<button label='Click'/>");
        // Position after "button" (character 7)
        let position = Position::new(0, 7);

        let result = hover(&doc, position);

        assert!(
            result.is_some(),
            "Expected hover result for widget at position (0, 7)"
        );
        let hover = result.unwrap();
        match hover.contents {
            HoverContents::Markup(content) => {
                assert_eq!(content.kind, MarkupKind::Markdown);
                assert!(
                    content.value.contains("Button"),
                    "Expected 'Button' in documentation, got: {}",
                    content.value
                );
            }
            _ => panic!("Expected Markup content"),
        }
    }

    #[test]
    fn test_hover_attribute() {
        let doc = create_test_doc("<button on_click='handle'/>");
        // Position at "on_click" (character 8-16)
        let position = Position::new(0, 12);

        let result = hover(&doc, position);

        assert!(
            result.is_some(),
            "Expected hover result for attribute at position (0, 12)"
        );
        let hover = result.unwrap();
        match hover.contents {
            HoverContents::Markup(content) => {
                assert_eq!(content.kind, MarkupKind::Markdown);
                // Should contain attribute documentation
                assert!(
                    content.value.contains("on_click") || content.value.contains("event"),
                    "Expected documentation for on_click attribute, got: {}",
                    content.value
                );
            }
            _ => panic!("Expected Markup content"),
        }
    }

    #[test]
    fn test_hover_value_boolean() {
        let doc = create_test_doc("<button enabled='true'/>");
        // Position inside the value "true" (character 18)
        let position = Position::new(0, 18);

        let result = hover(&doc, position);

        assert!(
            result.is_some(),
            "Expected hover result for value at position (0, 18)"
        );
        let hover = result.unwrap();
        match hover.contents {
            HoverContents::Markup(content) => {
                assert_eq!(content.kind, MarkupKind::Markdown);
                assert!(
                    content.value.contains("Boolean") || content.value.contains("true"),
                    "Expected boolean value documentation, got: {}",
                    content.value
                );
            }
            _ => panic!("Expected Markup content"),
        }
    }

    #[test]
    fn test_hover_value_color() {
        let doc = create_test_doc("<button background='#FF5733'/>");
        // Position inside the color value
        let position = Position::new(0, 22);

        let result = hover(&doc, position);

        assert!(
            result.is_some(),
            "Expected hover result for color value at position (0, 22)"
        );
        let hover = result.unwrap();
        match hover.contents {
            HoverContents::Markup(content) => {
                assert_eq!(content.kind, MarkupKind::Markdown);
                assert!(
                    content.value.contains("Color") || content.value.contains("color"),
                    "Expected color value documentation, got: {}",
                    content.value
                );
            }
            _ => panic!("Expected Markup content"),
        }
    }

    #[test]
    fn test_hover_unknown_widget() {
        let doc = create_test_doc("<unknown_widget/>");
        let position = Position::new(0, 2);

        let result = hover(&doc, position);

        // Should return None for unknown widgets
        assert!(result.is_none());
    }

    #[test]
    fn test_hover_no_context() {
        let doc = create_test_doc("just plain text");
        let position = Position::new(0, 5);

        let result = hover(&doc, position);

        // Should return None when not hovering over anything
        assert!(result.is_none());
    }

    #[test]
    fn test_extract_value_at_position() {
        let content = r#"enabled="true""#;
        let offset = 10; // Position at 'r' in "true"

        let value = extract_value_at_position(content, offset);

        // Should extract the full value between quotes
        assert_eq!(value, Some("true".to_string()));
    }

    #[test]
    fn test_hover_performance() {
        let doc = create_test_doc("<button label='Click' on_click='handle' enabled='true'/>");
        let position = Position::new(0, 7);

        let start = std::time::Instant::now();
        let result = hover(&doc, position);
        let elapsed = start.elapsed();

        assert!(result.is_some());
        // Should complete within 200ms (SC-004)
        assert!(
            elapsed.as_millis() < 200,
            "Hover took {}ms, expected < 200ms",
            elapsed.as_millis()
        );
    }
}
