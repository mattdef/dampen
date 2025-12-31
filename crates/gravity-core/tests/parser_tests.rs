use gravity_core::ir::{AttributeValue, EventKind, InterpolatedPart, WidgetKind};
use gravity_core::parser::parse;

#[test]
fn test_parse_valid_simple() {
    let xml = include_str!("fixtures/valid_simple.gravity");
    let result = parse(xml);

    assert!(result.is_ok(), "Should parse valid simple XML");
    let doc = result.unwrap();

    // Check root is a column
    assert!(matches!(doc.root.kind, WidgetKind::Column));

    // Check attributes
    assert_eq!(doc.root.attributes.len(), 2);
    assert!(
        matches!(doc.root.attributes.get("padding"), Some(AttributeValue::Static(s)) if s == "20")
    );
    assert!(
        matches!(doc.root.attributes.get("spacing"), Some(AttributeValue::Static(s)) if s == "10")
    );

    // Check children
    assert_eq!(doc.root.children.len(), 2);

    // First child: text
    let text = &doc.root.children[0];
    assert!(matches!(text.kind, WidgetKind::Text));
    assert!(
        matches!(text.attributes.get("value"), Some(AttributeValue::Static(s)) if s == "Hello, World!")
    );

    // Second child: button
    let button = &doc.root.children[1];
    assert!(matches!(button.kind, WidgetKind::Button));
    assert!(
        matches!(button.attributes.get("label"), Some(AttributeValue::Static(s)) if s == "Click me")
    );
    assert_eq!(button.events.len(), 1);
    assert!(matches!(button.events[0].event, EventKind::Click));
    assert_eq!(button.events[0].handler, "handle_click");
}

#[test]
fn test_parse_valid_nested() {
    let xml = include_str!("fixtures/valid_nested.gravity");
    let result = parse(xml);

    assert!(result.is_ok(), "Should parse nested XML");
    let doc = result.unwrap();

    // Root column
    assert!(matches!(doc.root.kind, WidgetKind::Column));
    assert_eq!(doc.root.children.len(), 2);

    // Second child is a row
    let row = &doc.root.children[1];
    assert!(matches!(row.kind, WidgetKind::Row));
    assert_eq!(row.children.len(), 3);

    // Middle child is a column
    let middle_col = &row.children[1];
    assert!(matches!(middle_col.kind, WidgetKind::Column));
    assert_eq!(middle_col.children.len(), 2);
}

#[test]
fn test_parse_invalid_syntax() {
    let xml = include_str!("fixtures/invalid_syntax.gravity");
    let result = parse(xml);

    // Should fail with parse error
    assert!(result.is_err());
    let err = result.unwrap_err();

    // Should have span information
    assert!(err.span.line > 0);
    assert!(err.span.column > 0);
}

#[test]
fn test_parse_empty_xml() {
    let result = parse("");
    assert!(result.is_err());
}

#[test]
fn test_parse_unknown_widget() {
    let xml = r#"<?xml version="1.0"?>
<unknown>
    <text value="test" />
</unknown>"#;

    let result = parse(xml);
    assert!(result.is_err());
}

#[test]
fn test_parse_button_with_events() {
    let xml = r#"<?xml version="1.0"?>
<column>
    <button label="Submit" on_click="submit" on_press="press" />
</column>"#;

    let result = parse(xml);
    assert!(result.is_ok());

    let doc = result.unwrap();
    let button = &doc.root.children[0];
    assert_eq!(button.events.len(), 2);

    let events: Vec<_> = button
        .events
        .iter()
        .map(|e| (e.event.clone(), e.handler.as_str()))
        .collect();
    assert!(events.contains(&(EventKind::Click, "submit")));
    assert!(events.contains(&(EventKind::Press, "press")));
}

#[test]
fn test_parse_binding_expressions() {
    let xml = r#"<text value="Total: {count} items" />"#;
    let result = parse(xml);

    assert!(result.is_ok());
    let doc = result.unwrap();

    if let WidgetKind::Text = doc.root.kind {
        if let Some(AttributeValue::Interpolated(parts)) = doc.root.attributes.get("value") {
            assert_eq!(parts.len(), 3);

            // Check literal parts
            if let InterpolatedPart::Literal(lit) = &parts[0] {
                assert_eq!(lit, "Total: ");
            } else {
                panic!("Expected literal");
            }

            // Check binding part
            if let InterpolatedPart::Binding(expr) = &parts[1] {
                // Should be a field access to "count"
                if let gravity_core::expr::Expr::FieldAccess(field) = &expr.expr {
                    assert_eq!(field.path, vec!["count"]);
                } else {
                    panic!("Expected field access");
                }
            } else {
                panic!("Expected binding");
            }

            // Check final literal
            if let InterpolatedPart::Literal(lit) = &parts[2] {
                assert_eq!(lit, " items");
            } else {
                panic!("Expected literal");
            }
        } else {
            panic!("Expected interpolated attribute");
        }
    } else {
        panic!("Expected text widget");
    }
}

#[test]
fn test_parse_simple_binding() {
    let xml = r#"<text value="{count}" />"#;
    let result = parse(xml);

    assert!(result.is_ok());
    let doc = result.unwrap();

    if let WidgetKind::Text = doc.root.kind {
        if let Some(AttributeValue::Binding(expr)) = doc.root.attributes.get("value") {
            if let gravity_core::expr::Expr::FieldAccess(field) = &expr.expr {
                assert_eq!(field.path, vec!["count"]);
            } else {
                panic!("Expected field access");
            }
        } else {
            panic!("Expected binding attribute");
        }
    } else {
        panic!("Expected text widget");
    }
}
#[test]
fn test_binding_eval_with_method() {
    use gravity_core::{
        evaluate_binding_expr, parse, AttributeValue, BindingValue, InterpolatedPart, UiBindable,
        WidgetKind,
    };

    #[derive(Debug, Clone, Default)]
    struct Model {
        items: Vec<String>,
    }

    impl UiBindable for Model {
        fn get_field(&self, path: &[&str]) -> Option<BindingValue> {
            match path {
                ["items"] => Some(BindingValue::from_value(&self.items)),
                _ => None,
            }
        }

        fn available_fields() -> Vec<String> {
            vec!["items".to_string()]
        }
    }

    let xml = r#"<text value="Total: {items.len()} items" />"#;
    let doc = parse(xml).unwrap();

    let model = Model {
        items: vec!["a".to_string(), "b".to_string()],
    };

    if let WidgetKind::Text = doc.root.kind {
        if let Some(AttributeValue::Interpolated(parts)) = doc.root.attributes.get("value") {
            let mut result = String::new();
            for part in parts {
                match part {
                    InterpolatedPart::Literal(literal) => result.push_str(literal),
                    InterpolatedPart::Binding(binding_expr) => {
                        match evaluate_binding_expr(binding_expr, &model) {
                            Ok(value) => result.push_str(&value.to_display_string()),
                            Err(_) => result.push_str("[error]"),
                        }
                    }
                }
            }
            assert_eq!(result, "Total: 2 items");
        }
    }
}

#[test]
fn test_conditional_binding() {
    use gravity_core::{evaluate_binding_expr, BindingValue, UiBindable};

    #[derive(Debug, Clone, Default)]
    struct Model {
        items: Vec<String>,
    }

    impl UiBindable for Model {
        fn get_field(&self, path: &[&str]) -> Option<BindingValue> {
            match path {
                ["items"] => Some(BindingValue::from_value(&self.items)),
                _ => None,
            }
        }

        fn available_fields() -> Vec<String> {
            vec!["items".to_string()]
        }
    }

    let xml = r#"<text value="{if items.len() == 0 then 'No items yet!' else ''}" />"#;
    let doc = parse(xml).unwrap();

    let model = Model { items: Vec::new() };

    if let WidgetKind::Text = doc.root.kind {
        if let Some(AttributeValue::Binding(binding_expr)) = doc.root.attributes.get("value") {
            match evaluate_binding_expr(binding_expr, &model) {
                Ok(value) => {
                    assert_eq!(value.to_display_string(), "No items yet!");
                }
                Err(e) => {
                    panic!("Evaluation error: {}", e.message);
                }
            }
        }
    }
}

#[test]
fn test_tokenizer_conditional() {
    use gravity_core::expr::tokenize_binding_expr;

    let expr = "if items.len() == 0 then 'No items yet!' else ''";
    let result = tokenize_binding_expr(expr, 0, 1, 1);

    match result {
        Ok(tokens) => {
            println!("Tokens: {:?}", tokens);
        }
        Err(e) => {
            panic!("Tokenization error: {}", e);
        }
    }
}
