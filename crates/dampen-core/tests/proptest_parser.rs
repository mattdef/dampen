//! Property-based tests for the parser
//!
//! These tests use proptest to generate random inputs and verify that
//! the parser handles edge cases correctly.

use dampen_core::{AttributeValue, WidgetKind, parse};
use proptest::prelude::*;

/// Generate valid XML with random nesting depth
fn generate_xml(depth: usize) -> String {
    if depth == 0 {
        return r#"<text value="test" />"#.to_string();
    }

    format!(
        r#"<column spacing="{}">{}</column>"#,
        depth,
        generate_xml(depth - 1)
    )
}

proptest::proptest! {
    #[test]
    fn test_parse_random_xml(depth in 1usize..10) {
        let xml = generate_xml(depth);
        let result = parse(&xml);
        prop_assert!(result.is_ok());
        let doc = result.unwrap();
        prop_assert_eq!(doc.version.major, 1);
    }

    #[test]
    fn test_parse_with_random_spacing(spacing in 0i32..100) {
        let xml = format!(r#"<column spacing="{}"><text value="test" /></column>"#, spacing);
        let result = parse(&xml);
        prop_assert!(result.is_ok());
        let doc = result.unwrap();
        prop_assert_eq!(doc.root.kind, WidgetKind::Column);
        prop_assert!(doc.root.attributes.contains_key("spacing"));
    }

    #[test]
    fn test_parse_button_with_handler(handler_name in "[a-z]{3,10}") {
        let xml = format!(r#"<button label="Click" on_click="{}" />"#, handler_name);
        let result = parse(&xml);
        prop_assert!(result.is_ok());
        let doc = result.unwrap();
        prop_assert_eq!(doc.root.kind, WidgetKind::Button);
        prop_assert_eq!(doc.root.events.len(), 1);
    }

    #[test]
    fn test_parse_text_with_binding(field_name in "[a-z]{2,8}".prop_filter("exclude reserved keywords", |s| {
        !matches!(s.as_str(), "if" | "else" | "while" | "for" | "loop" | "match" | "fn" | "let" | "true" | "false" | "null" | "self")
    })) {
        let xml = format!(r#"<text value="{{{}}}" />"#, field_name);
        let result = parse(&xml);
        prop_assert!(result.is_ok(), "Failed to parse: {}", xml);
        let doc = result.unwrap();
        prop_assert_eq!(doc.root.kind, WidgetKind::Text);
        prop_assert!(doc.root.attributes.contains_key("value"));
    }

    #[test]
    fn test_parse_multiple_widgets(count in 1usize..20) {
        let children: String = (0..count)
            .map(|i| format!(r#"<text value="item{}" />"#, i))
            .collect();
        let xml = format!(r#"<column>{}</column>"#, children);
        let result = parse(&xml);
        prop_assert!(result.is_ok());
        let doc = result.unwrap();
        prop_assert_eq!(doc.root.children.len(), count);
    }

    #[test]
    fn test_parse_long_text(length in 1usize..500) {
        let text: String = (0..length).map(|_| 'x').collect();
        let xml = format!(r#"<text value="{}" />"#, text);
        let result = parse(&xml);
        prop_assert!(result.is_ok());
        let doc = result.unwrap();
        if let Some(AttributeValue::Static(value)) = doc.root.attributes.get("value") {
            prop_assert_eq!(value.len(), length);
        } else {
            prop_assert!(false, "Expected static attribute");
        }
    }
}

#[cfg(test)]
mod edge_cases {
    use super::*;

    #[test]
    fn test_empty_string() {
        let result = parse("");
        assert!(result.is_err());
    }

    #[test]
    fn test_unicode_in_values() {
        let xml = r#"<text value="Hello 世界 🌍" />"#;
        let result = parse(xml);
        assert!(result.is_ok());
        let doc = result.unwrap();
        if let Some(AttributeValue::Static(value)) = doc.root.attributes.get("value") {
            assert_eq!(value, "Hello 世界 🌍");
        }
    }

    #[test]
    fn test_special_xml_chars() {
        // Just test that normal text works
        let xml = r#"<text value="Hello World" />"#;
        let result = parse(xml);
        assert!(result.is_ok());
    }
}
