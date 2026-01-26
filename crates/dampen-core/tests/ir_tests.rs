use dampen_core::expr::{BindingExpr, Expr, FieldAccessExpr, LiteralExpr};
use dampen_core::ir::{
    AttributeValue, DampenDocument, EventBinding, EventKind, SchemaVersion, Span, WidgetKind,
    WidgetNode,
};
use std::collections::HashMap;

#[test]
fn test_ir_serialization() {
    let doc = DampenDocument {
        version: SchemaVersion { major: 1, minor: 0 },
        root: WidgetNode {
            kind: WidgetKind::Column,
            id: Some("root".to_string()),
            attributes: {
                let mut map = HashMap::new();
                map.insert(
                    "padding".to_string(),
                    AttributeValue::Static("20".to_string()),
                );
                map
            },
            events: vec![],
            children: vec![WidgetNode {
                kind: WidgetKind::Text,
                id: None,
                attributes: {
                    let mut map = HashMap::new();
                    map.insert(
                        "value".to_string(),
                        AttributeValue::Static("Hello".to_string()),
                    );
                    map
                },
                events: vec![],
                children: vec![],
                span: Span::new(0, 0, 1, 1),
                style: None,
                layout: None,
                theme_ref: None,
                classes: vec![],
                breakpoint_attributes: HashMap::new(),
                inline_state_variants: HashMap::new(),
            }],
            span: Span::new(0, 0, 1, 1),
            style: None,
            layout: None,
            theme_ref: None,
            classes: vec![],
            breakpoint_attributes: HashMap::new(),
            inline_state_variants: HashMap::new(),
        },
        themes: HashMap::new(),
        style_classes: HashMap::new(),
        global_theme: None,
        follow_system: true,
    };

    // Test serialization
    let json = serde_json::to_string(&doc).unwrap();
    assert!(json.contains("Column"));
    assert!(json.contains("Hello"));

    // Test deserialization
    let deserialized: DampenDocument = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.version.major, 1);
    assert!(matches!(deserialized.root.kind, WidgetKind::Column));
}

#[test]
fn test_span_serialization() {
    let span = Span::new(10, 20, 3, 5);
    let json = serde_json::to_string(&span).unwrap();
    let deserialized: Span = serde_json::from_str(&json).unwrap();

    assert_eq!(deserialized.start, 10);
    assert_eq!(deserialized.end, 20);
    assert_eq!(deserialized.line, 3);
    assert_eq!(deserialized.column, 5);
}

#[test]
fn test_widget_kind_serialization() {
    let kind = WidgetKind::Button;
    let json = serde_json::to_string(&kind).unwrap();
    let deserialized: WidgetKind = serde_json::from_str(&json).unwrap();

    assert!(matches!(deserialized, WidgetKind::Button));
}

#[test]
fn test_expression_serialization() {
    let expr = Expr::Literal(LiteralExpr::String("test".to_string()));
    let json = serde_json::to_string(&expr).unwrap();
    let deserialized: Expr = serde_json::from_str(&json).unwrap();

    assert!(matches!(deserialized, Expr::Literal(_)));
}

#[test]
fn test_binding_expr_serialization() {
    let binding = BindingExpr {
        expr: Expr::FieldAccess(FieldAccessExpr {
            path: vec!["user".to_string(), "name".to_string()],
        }),
        span: Span::new(0, 0, 1, 1),
    };

    let json = serde_json::to_string(&binding).unwrap();
    let deserialized: BindingExpr = serde_json::from_str(&json).unwrap();

    if let Expr::FieldAccess(fa) = deserialized.expr {
        assert_eq!(fa.path, vec!["user", "name"]);
    } else {
        panic!("Expected FieldAccess");
    }
}

#[test]
fn test_event_binding_serialization() {
    let event = EventBinding {
        event: EventKind::Click,
        handler: "handle_click".to_string(),
        param: None,
        span: Span::new(0, 0, 1, 1),
    };

    let json = serde_json::to_string(&event).unwrap();
    let deserialized: EventBinding = serde_json::from_str(&json).unwrap();

    assert!(matches!(deserialized.event, EventKind::Click));
    assert_eq!(deserialized.handler, "handle_click");
}
