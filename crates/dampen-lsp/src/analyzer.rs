//! Semantic analyzer for position-based queries.
//!
//! Provides analysis of document content at specific positions,
//! used for completion and hover functionality.

#![allow(dead_code)]

use tower_lsp::lsp_types::Position;
use tracing::trace;

use crate::converters::position_to_offset;
use crate::document::DocumentState;

/// Context for completion requests.
#[derive(Debug, Clone, PartialEq)]
pub enum CompletionContext {
    /// After `<` - suggesting widget names
    WidgetName,
    /// Inside widget tag - suggesting attributes
    AttributeName { widget: String },
    /// Inside attribute quotes - suggesting values
    AttributeValue { widget: String, attribute: String },
    /// Inside binding expression
    BindingExpression,
    /// Unknown context
    Unknown,
}

/// Analyzes a document at a specific position.
pub struct Analyzer;

impl Analyzer {
    /// Creates a new analyzer instance.
    pub fn new() -> Self {
        Self
    }

    /// Determines the completion context at a position.
    ///
    /// Analyzes the document content around the cursor to determine
    /// what type of completions should be offered.
    ///
    /// # Arguments
    ///
    /// * `doc` - The document state
    /// * `position` - Cursor position
    ///
    /// # Returns
    ///
    /// Detected completion context
    pub fn get_completion_context(
        &self,
        doc: &DocumentState,
        position: Position,
    ) -> CompletionContext {
        trace!("Analyzing completion context at {:?}", position);

        let offset = match position_to_offset(&doc.content, position) {
            Some(offset) => offset,
            None => return CompletionContext::Unknown,
        };

        // Get context around cursor (200 chars before and after should be enough)
        let start = offset.saturating_sub(200);
        let end = (offset + 200).min(doc.content.len());
        let context = &doc.content[start..end];
        let cursor_in_context = offset - start;

        // Check if we're inside a binding expression {|...}
        if self.is_in_binding_expression(context, cursor_in_context) {
            return CompletionContext::BindingExpression;
        }

        // Check if we're inside attribute quotes
        if let Some((widget, attribute)) = self.find_attribute_at_position(doc, position) {
            // Check if cursor is inside the attribute value quotes
            if self.is_inside_attribute_quotes(context, cursor_in_context) {
                return CompletionContext::AttributeValue { widget, attribute };
            }
        }

        // Check if we're inside a widget tag (after widget name, before `>`)
        if let Some(widget) = self.find_widget_at_position(doc, position) {
            // Check if we're inside the tag but not in a value
            if self.is_inside_widget_tag(context, cursor_in_context)
                && !self.is_inside_attribute_quotes(context, cursor_in_context)
            {
                return CompletionContext::AttributeName { widget };
            }
        }

        // Check if we're after `<` (start of tag)
        if self.is_after_open_bracket(context, cursor_in_context) {
            return CompletionContext::WidgetName;
        }

        CompletionContext::Unknown
    }

    /// Finds the widget at a given position.
    ///
    /// Searches backwards from the position to find the enclosing widget tag.
    ///
    /// # Arguments
    ///
    /// * `doc` - The document state
    /// * `position` - Position to check
    ///
    /// # Returns
    ///
    /// Widget name if found
    pub fn find_widget_at_position(
        &self,
        doc: &DocumentState,
        position: Position,
    ) -> Option<String> {
        let offset = position_to_offset(&doc.content, position)?;

        // Search backwards for the nearest opening tag
        let content_before = &doc.content[..offset];

        // Find the last `<` that isn't part of a closing tag
        // Use depth tracking to handle nested tags
        let mut last_tag_start = None;
        let mut depth = 0;

        for (idx, ch) in content_before.char_indices().rev() {
            match ch {
                '>' => depth += 1,
                '<' => {
                    if depth == 0 {
                        // This is an opening tag (not a closing tag of nested content)
                        if content_before.get(idx + 1..idx + 2) != Some("/") {
                            last_tag_start = Some(idx);
                        }
                        break;
                    } else {
                        depth -= 1;
                    }
                }
                _ => {}
            }
        }

        let tag_start = last_tag_start?;

        // Extract widget name from tag
        let after_bracket = &content_before[tag_start + 1..];
        let widget_name: String = after_bracket
            .chars()
            .take_while(|c| c.is_ascii_alphanumeric() || *c == '_')
            .collect();

        if widget_name.is_empty() {
            None
        } else {
            Some(widget_name)
        }
    }

    /// Finds the attribute at a given position.
    ///
    /// # Arguments
    ///
    /// * `doc` - The document state
    /// * `position` - Position to check
    ///
    /// # Returns
    ///
    /// (widget_name, attribute_name) if found
    pub fn find_attribute_at_position(
        &self,
        doc: &DocumentState,
        position: Position,
    ) -> Option<(String, String)> {
        let offset = position_to_offset(&doc.content, position)?;

        // First find the enclosing widget
        let widget = self.find_widget_at_position(doc, position)?;

        // Search backwards for the nearest attribute
        let content_before = &doc.content[..offset];

        // Find the last attribute name before position
        let mut last_attr = None;

        // First, determine if we're inside a string by counting quotes
        // Odd count means we're inside a double-quoted string
        let double_quote_count = content_before.chars().filter(|c| *c == '"').count();
        let single_quote_count = content_before.chars().filter(|c| *c == '\'').count();

        // Start with the correct state based on quote count
        let mut in_double_quote = double_quote_count % 2 == 1;
        let mut in_single_quote = single_quote_count % 2 == 1;

        for (idx, ch) in content_before.char_indices().rev() {
            // Handle quotes - toggle state when we encounter a quote
            if ch == '"' && !in_single_quote {
                in_double_quote = !in_double_quote;
                continue;
            }
            if ch == '\'' && !in_double_quote {
                in_single_quote = !in_single_quote;
                continue;
            }

            // Skip if we're inside a string
            if in_double_quote || in_single_quote {
                continue;
            }

            match ch {
                '=' => {
                    // Found an equals sign, extract the attribute name before it
                    let before_equals = &content_before[..idx];
                    let attr_name: String = before_equals
                        .chars()
                        .rev()
                        .take_while(|c| c.is_ascii_alphanumeric() || *c == '_' || *c == '-')
                        .collect::<String>()
                        .chars()
                        .rev()
                        .collect();

                    if !attr_name.is_empty() {
                        last_attr = Some(attr_name);
                    }
                    break;
                }
                '<' => {
                    // Hit the start of tag, stop searching
                    break;
                }
                _ => {}
            }
        }

        last_attr.map(|attr| (widget, attr))
    }

    /// Checks if the cursor is after an opening bracket `<`.
    fn is_after_open_bracket(&self, context: &str, cursor: usize) -> bool {
        // Look backwards from cursor for `<` without intervening `>` or whitespace
        let before_cursor = &context[..cursor.min(context.len())];

        for ch in before_cursor.chars().rev() {
            match ch {
                '<' => return true,
                '>' | ' ' | '\t' | '\n' | '\r' => return false,
                _ => continue,
            }
        }

        false
    }

    /// Checks if the cursor is inside a widget tag (between `<widget` and `>`).
    fn is_inside_widget_tag(&self, context: &str, cursor: usize) -> bool {
        let before_cursor = &context[..cursor.min(context.len())];

        // Look for `<` without a matching `>` after it
        let mut found_open = false;
        let mut in_string = false;
        let mut string_char = '\0';

        for ch in before_cursor.chars().rev() {
            if in_string {
                if ch == string_char {
                    in_string = false;
                }
                continue;
            }

            match ch {
                '"' | '\'' => {
                    in_string = true;
                    string_char = ch;
                }
                '>' => return false,
                '<' => {
                    found_open = true;
                    break;
                }
                _ => {}
            }
        }

        if !found_open {
            return false;
        }

        // Check that we're still before the closing `>`
        let after_cursor = &context[cursor.min(context.len())..];
        for ch in after_cursor.chars() {
            match ch {
                '>' => return true,
                '<' => return false,
                _ => {}
            }
        }

        // If we get here, there's no closing `>` or opening `<` after cursor
        // This means we're at the end of an unclosed tag, so we're inside it
        true
    }

    /// Checks if the cursor is inside attribute quotes.
    fn is_inside_attribute_quotes(&self, context: &str, cursor: usize) -> bool {
        let before_cursor = &context[..cursor.min(context.len())];

        // Count unclosed quotes before cursor
        let mut in_double_quote = false;
        let mut in_single_quote = false;

        for ch in before_cursor.chars() {
            match ch {
                '"' if !in_single_quote => in_double_quote = !in_double_quote,
                '\'' if !in_double_quote => in_single_quote = !in_single_quote,
                _ => {}
            }
        }

        in_double_quote || in_single_quote
    }

    /// Checks if the cursor is inside a binding expression `{|...}`.
    fn is_in_binding_expression(&self, context: &str, cursor: usize) -> bool {
        let before_cursor = &context[..cursor.min(context.len())];

        // Find the last `{` before cursor
        let mut last_brace = None;
        let mut in_string = false;
        let mut string_char = '\0';

        for (idx, ch) in before_cursor.char_indices().rev() {
            if in_string {
                if ch == string_char {
                    in_string = false;
                }
                continue;
            }

            match ch {
                '"' | '\'' | '`' => {
                    in_string = true;
                    string_char = ch;
                }
                '}' => {
                    // Found a closing brace, we're not in a binding
                    return false;
                }
                '{' => {
                    last_brace = Some(idx);
                    break;
                }
                _ => {}
            }
        }

        last_brace.is_some()
    }
}

impl Default for Analyzer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tower_lsp::lsp_types::Url;

    fn test_doc(content: &str) -> DocumentState {
        DocumentState::new(
            Url::parse("file:///test.dampen").unwrap(),
            content.to_string(),
            1,
        )
    }

    fn pos(line: u32, character: u32) -> Position {
        Position::new(line, character)
    }

    #[test]
    fn test_find_widget_at_position_simple() {
        let doc = test_doc("<button label=\"Click\">");
        let analyzer = Analyzer::new();

        // Position after "button"
        let widget = analyzer.find_widget_at_position(&doc, pos(0, 7));
        assert_eq!(widget, Some("button".to_string()));
    }

    #[test]
    fn test_find_widget_at_position_with_attribute() {
        let doc = test_doc("<button label=\"Click\" />");
        let analyzer = Analyzer::new();

        // Position inside attribute value - after "label=\"C"
        let widget = analyzer.find_widget_at_position(&doc, pos(0, 16));
        assert_eq!(widget, Some("button".to_string()));
    }

    #[test]
    fn test_find_widget_at_position_nested() {
        let doc = test_doc("<column><button /></column>");
        let analyzer = Analyzer::new();

        // Position at button
        let widget = analyzer.find_widget_at_position(&doc, pos(0, 15));
        assert_eq!(widget, Some("button".to_string()));
    }

    #[test]
    fn test_find_attribute_at_position() {
        let doc = test_doc("<button label=\"Click\" />");
        let analyzer = Analyzer::new();

        // Position inside attribute value - after "label=\"C"
        let result = analyzer.find_attribute_at_position(&doc, pos(0, 16));
        assert_eq!(result, Some(("button".to_string(), "label".to_string())));
    }

    #[test]
    fn test_get_completion_context_widget_name() {
        let doc = test_doc("<");
        let analyzer = Analyzer::new();

        let context = analyzer.get_completion_context(&doc, pos(0, 1));
        assert_eq!(context, CompletionContext::WidgetName);
    }

    #[test]
    fn test_get_completion_context_attribute_name() {
        let doc = test_doc("<button ");
        let analyzer = Analyzer::new();

        // Position after "button " - should be inside tag
        let context = analyzer.get_completion_context(&doc, pos(0, 8));
        assert!(
            matches!(&context,
                CompletionContext::AttributeName { widget } if widget == "button"
            ),
            "Expected AttributeName, got {:?}",
            context
        );
    }

    #[test]
    fn test_get_completion_context_attribute_value() {
        let doc = test_doc("<button label=\"");
        let analyzer = Analyzer::new();

        // Position inside the quotes after "label=\""
        let context = analyzer.get_completion_context(&doc, pos(0, 15));
        assert!(
            matches!(&context,
                CompletionContext::AttributeValue { widget, attribute }
                if widget == "button" && attribute == "label"
            ),
            "Expected AttributeValue, got {:?}",
            context
        );
    }

    #[test]
    fn test_is_after_open_bracket() {
        let analyzer = Analyzer::new();

        // Position 1 is right after `<` - should be true
        assert!(analyzer.is_after_open_bracket("<button>", 1));
        // Position 8 is after `>` - should be false
        assert!(!analyzer.is_after_open_bracket("<button>", 8));
        // Position 4 is in middle of "button" - should be true (still after `<`)
        assert!(analyzer.is_after_open_bracket("<button>", 4));
    }

    #[test]
    fn test_is_inside_widget_tag() {
        let analyzer = Analyzer::new();

        assert!(analyzer.is_inside_widget_tag("<button >", 7));
        assert!(!analyzer.is_inside_widget_tag("<button>", 8));
        assert!(!analyzer.is_inside_widget_tag("text", 2));
    }

    #[test]
    fn test_is_inside_attribute_quotes() {
        let analyzer = Analyzer::new();

        assert!(analyzer.is_inside_attribute_quotes("label=\"val", 8));
        assert!(!analyzer.is_inside_attribute_quotes("label=\"val\"", 12));
        assert!(analyzer.is_inside_attribute_quotes("label='val", 8));
    }

    #[test]
    fn test_is_in_binding_expression() {
        let analyzer = Analyzer::new();

        assert!(analyzer.is_in_binding_expression("{|count", 2));
        assert!(!analyzer.is_in_binding_expression("{|count|}", 9));
        assert!(!analyzer.is_in_binding_expression("text", 2));
    }

    #[test]
    fn test_find_widget_no_widget() {
        let doc = test_doc("just text");
        let analyzer = Analyzer::new();

        let widget = analyzer.find_widget_at_position(&doc, pos(0, 5));
        assert_eq!(widget, None);
    }

    #[test]
    fn test_find_attribute_no_attribute() {
        let doc = test_doc("<button>");
        let analyzer = Analyzer::new();

        let result = analyzer.find_attribute_at_position(&doc, pos(0, 5));
        assert_eq!(result, None);
    }

    #[test]
    fn test_multiline_document() {
        let doc = test_doc("<column>\n  <button label=\"Click\" />\n</column>");
        let analyzer = Analyzer::new();

        // Line 1: "  <button label=\"Click\" />"
        // Position at button on line 1 - character 10 is after "button"
        let widget = analyzer.find_widget_at_position(&doc, pos(1, 10));
        assert_eq!(widget, Some("button".to_string()));

        // Position inside attribute value on line 1 - character 18 is after "label=\"C"
        let result = analyzer.find_attribute_at_position(&doc, pos(1, 18));
        assert_eq!(result, Some(("button".to_string(), "label".to_string())));
    }
}
