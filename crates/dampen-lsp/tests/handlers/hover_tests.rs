//! Hover handler tests.
//!
//! Tests for the hover functionality providing contextual documentation.

use dampen_lsp::document::DocumentState;
use dampen_lsp::handlers::hover::hover;
use tower_lsp::lsp_types::{HoverContents, MarkupKind, Position, Url};

fn create_test_doc(content: &str) -> DocumentState {
    let uri = Url::parse("file:///test.dampen").unwrap();
    DocumentState::new(uri, content.to_string(), 1)
}

#[test]
fn test_hover_widget_button() {
    let doc = create_test_doc("<button label='Click'/>");
    let position = Position::new(0, 7); // After "button"

    let result = hover(&doc, position);

    assert!(result.is_some(), "Expected hover result for button widget");
    let hover = result.unwrap();
    match hover.contents {
        HoverContents::Markup(content) => {
            assert_eq!(content.kind, MarkupKind::Markdown);
            assert!(
                content.value.contains("Button"),
                "Expected 'Button' in documentation"
            );
        }
        _ => panic!("Expected Markup content"),
    }
}

#[test]
fn test_hover_widget_column() {
    let doc = create_test_doc("<column></column>");
    let position = Position::new(0, 7); // After "column"

    let result = hover(&doc, position);

    assert!(result.is_some(), "Expected hover result for column widget");
    let hover = result.unwrap();
    match hover.contents {
        HoverContents::Markup(content) => {
            assert_eq!(content.kind, MarkupKind::Markdown);
            assert!(
                content.value.contains("Column"),
                "Expected 'Column' in documentation"
            );
        }
        _ => panic!("Expected Markup content"),
    }
}

#[test]
fn test_hover_attribute_on_click() {
    let doc = create_test_doc("<button on_click='handle'/>");
    let position = Position::new(0, 12); // At "on_click"

    let result = hover(&doc, position);

    assert!(
        result.is_some(),
        "Expected hover result for on_click attribute"
    );
    let hover = result.unwrap();
    match hover.contents {
        HoverContents::Markup(content) => {
            assert_eq!(content.kind, MarkupKind::Markdown);
            assert!(
                content.value.contains("on_click") || content.value.contains("event"),
                "Expected on_click documentation"
            );
        }
        _ => panic!("Expected Markup content"),
    }
}

#[test]
fn test_hover_attribute_width() {
    let doc = create_test_doc("<button width='100px'/>");
    let position = Position::new(0, 10); // At "width"

    let result = hover(&doc, position);

    assert!(
        result.is_some(),
        "Expected hover result for width attribute"
    );
    let hover = result.unwrap();
    match hover.contents {
        HoverContents::Markup(content) => {
            assert_eq!(content.kind, MarkupKind::Markdown);
            assert!(
                content.value.contains("width") || content.value.contains("Width"),
                "Expected width documentation"
            );
        }
        _ => panic!("Expected Markup content"),
    }
}

#[test]
fn test_hover_value_boolean_true() {
    let doc = create_test_doc("<button enabled='true'/>");
    let position = Position::new(0, 18); // Inside "true"

    let result = hover(&doc, position);

    assert!(result.is_some(), "Expected hover result for boolean value");
    let hover = result.unwrap();
    match hover.contents {
        HoverContents::Markup(content) => {
            assert_eq!(content.kind, MarkupKind::Markdown);
            assert!(
                content.value.contains("Boolean") || content.value.contains("boolean"),
                "Expected boolean value documentation"
            );
        }
        _ => panic!("Expected Markup content"),
    }
}

#[test]
fn test_hover_value_color() {
    let doc = create_test_doc("<button background='#FF5733'/>");
    let position = Position::new(0, 22); // Inside color value

    let result = hover(&doc, position);

    assert!(result.is_some(), "Expected hover result for color value");
    let hover = result.unwrap();
    match hover.contents {
        HoverContents::Markup(content) => {
            assert_eq!(content.kind, MarkupKind::Markdown);
            assert!(
                content.value.contains("Color") || content.value.contains("color"),
                "Expected color value documentation"
            );
        }
        _ => panic!("Expected Markup content"),
    }
}

#[test]
fn test_hover_value_alignment() {
    let doc = create_test_doc("<column align_items='center'/>");
    let position = Position::new(0, 25); // Inside "center"

    let result = hover(&doc, position);

    assert!(
        result.is_some(),
        "Expected hover result for alignment value"
    );
    let hover = result.unwrap();
    match hover.contents {
        HoverContents::Markup(content) => {
            assert_eq!(content.kind, MarkupKind::Markdown);
            assert!(
                content.value.contains("Alignment") || content.value.contains("alignment"),
                "Expected alignment value documentation"
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
fn test_hover_performance_sc004() {
    // SC-004: Hover must respond within 200ms
    let doc = create_test_doc("<button label='Click' on_click='handle' enabled='true'/>");
    let position = Position::new(0, 7);

    let start = std::time::Instant::now();
    let result = hover(&doc, position);
    let elapsed = start.elapsed();

    assert!(result.is_some());
    assert!(
        elapsed.as_millis() < 200,
        "Hover took {}ms, expected < 200ms per SC-004",
        elapsed.as_millis()
    );
}

#[test]
fn test_hover_multiline_document() {
    let doc =
        create_test_doc("<column>\n  <button label='Click'/>\n  <text value='Hello'/>\n</column>");

    // Hover over button on line 1
    let position = Position::new(1, 10);
    let result = hover(&doc, position);

    assert!(
        result.is_some(),
        "Expected hover result in multiline document"
    );
}

#[test]
fn test_hover_widget_text() {
    let doc = create_test_doc("<text value='Hello'/>");
    let position = Position::new(0, 5); // After "text"

    let result = hover(&doc, position);

    assert!(result.is_some(), "Expected hover result for text widget");
    let hover = result.unwrap();
    match hover.contents {
        HoverContents::Markup(content) => {
            assert_eq!(content.kind, MarkupKind::Markdown);
            assert!(
                content.value.contains("Text"),
                "Expected 'Text' in documentation"
            );
        }
        _ => panic!("Expected Markup content"),
    }
}

#[test]
fn test_hover_attribute_background() {
    let doc = create_test_doc("<container background='#FFF'/>");
    let position = Position::new(0, 15); // At "background"

    let result = hover(&doc, position);

    assert!(
        result.is_some(),
        "Expected hover result for background attribute"
    );
    let hover = result.unwrap();
    match hover.contents {
        HoverContents::Markup(content) => {
            assert_eq!(content.kind, MarkupKind::Markdown);
            assert!(
                content.value.contains("background") || content.value.contains("Background"),
                "Expected background documentation"
            );
        }
        _ => panic!("Expected Markup content"),
    }
}
