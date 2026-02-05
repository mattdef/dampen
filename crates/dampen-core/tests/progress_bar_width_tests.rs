//! Tests for progress_bar width attribute

#[cfg(test)]
mod tests {
    use dampen_core::{HandlerSignature, generate_application, parse};

    #[test]
    fn test_progress_bar_with_width_fixed() {
        let xml = r#"
            <dampen version="1.1">
                <progress_bar value="50" width="200" />
            </dampen>
        "#;

        let doc = parse(xml).unwrap();
        let handlers: Vec<HandlerSignature> = vec![];

        let result = generate_application(&doc, "Model", "Message", &handlers);

        assert!(
            result.is_ok(),
            "Codegen should succeed for progress_bar with width"
        );

        let output = result.unwrap();
        let code = output.code.to_string();

        // Check for the pattern with spaces (quote! formatting)
        assert!(
            code.contains(". length (iced :: Length :: Fixed (200f32))"),
            "Generated code should contain length with Fixed: {}",
            code
        );
    }

    #[test]
    fn test_progress_bar_with_width_fill() {
        let xml = r#"
            <dampen version="1.1">
                <progress_bar value="50" width="fill" />
            </dampen>
        "#;

        let doc = parse(xml).unwrap();
        let handlers: Vec<HandlerSignature> = vec![];

        let result = generate_application(&doc, "Model", "Message", &handlers);

        assert!(
            result.is_ok(),
            "Codegen should succeed for progress_bar with fill width"
        );

        let output = result.unwrap();
        let code = output.code.to_string();

        assert!(
            code.contains(". length (iced :: Length :: Fill)"),
            "Generated code should contain length with Fill: {}",
            code
        );
    }

    #[test]
    fn test_progress_bar_with_width_shrink() {
        let xml = r#"
            <dampen version="1.1">
                <progress_bar value="50" width="shrink" />
            </dampen>
        "#;

        let doc = parse(xml).unwrap();
        let handlers: Vec<HandlerSignature> = vec![];

        let result = generate_application(&doc, "Model", "Message", &handlers);

        assert!(
            result.is_ok(),
            "Codegen should succeed for progress_bar with shrink width"
        );

        let output = result.unwrap();
        let code = output.code.to_string();

        assert!(
            code.contains(". length (iced :: Length :: Shrink)"),
            "Generated code should contain length with Shrink: {}",
            code
        );
    }

    #[test]
    fn test_progress_bar_with_width_percentage() {
        let xml = r#"
            <dampen version="1.1">
                <progress_bar value="50" width="50%" />
            </dampen>
        "#;

        let doc = parse(xml).unwrap();
        let handlers: Vec<HandlerSignature> = vec![];

        let result = generate_application(&doc, "Model", "Message", &handlers);

        assert!(
            result.is_ok(),
            "Codegen should succeed for progress_bar with percentage width"
        );

        let output = result.unwrap();
        let code = output.code.to_string();

        assert!(
            code.contains(". length (iced :: Length :: FillPortion ("),
            "Generated code should contain length with FillPortion for percentage: {}",
            code
        );
    }

    #[test]
    fn test_progress_bar_without_width() {
        let xml = r#"
            <dampen version="1.1">
                <progress_bar value="50" />
            </dampen>
        "#;

        let doc = parse(xml).unwrap();
        let handlers: Vec<HandlerSignature> = vec![];

        let result = generate_application(&doc, "Model", "Message", &handlers);

        assert!(
            result.is_ok(),
            "Codegen should succeed for progress_bar without width"
        );

        let output = result.unwrap();
        let code = output.code.to_string();

        // Should not contain .length() call when not specified
        assert!(
            !code.contains(". length (iced :: Length ::"),
            "Generated code should not contain length when not specified: {}",
            code
        );
    }

    #[test]
    fn test_progress_bar_with_width_and_other_attributes() {
        let xml = r#"
            <dampen version="1.1">
                <progress_bar value="75" width="300" height="10" style="success" />
            </dampen>
        "#;

        let doc = parse(xml).unwrap();
        let handlers: Vec<HandlerSignature> = vec![];

        let result = generate_application(&doc, "Model", "Message", &handlers);

        assert!(
            result.is_ok(),
            "Codegen should succeed for progress_bar with multiple attributes"
        );

        let output = result.unwrap();
        let code = output.code.to_string();

        assert!(
            code.contains(". length (iced :: Length :: Fixed (300f32))"),
            "Generated code should contain length: {}",
            code
        );
        assert!(
            code.contains(". girth (10f32)"),
            "Generated code should contain girth: {}",
            code
        );
    }
}
