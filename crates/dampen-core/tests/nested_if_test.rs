#[cfg(test)]
mod tests {
    use dampen_core::expr::Expr;
    use dampen_core::ir::node::AttributeValue;
    use dampen_core::parser::parse;

    #[test]
    fn test_parse_nested_if() {
        let xml = r#"<text value="{if a then '1' else if b then '2' else '3'}" />"#;
        let result = parse(xml);
        assert!(
            result.is_ok(),
            "Failed to parse nested if: {:?}",
            result.err()
        );

        let doc = result.unwrap();
        let attr = doc.root.attributes.get("value").unwrap();
        if let AttributeValue::Binding(expr) = attr {
            if let Expr::Conditional(cond) = &expr.expr {
                assert!(
                    matches!(*cond.else_branch, Expr::Conditional(_)),
                    "Else branch should be a conditional"
                );
            } else {
                panic!("Root should be a conditional");
            }
        } else {
            panic!("Expected binding");
        }
    }
}
