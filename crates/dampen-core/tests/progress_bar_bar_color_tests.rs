//! Tests for bindable bar_color attribute on progress_bar

#[cfg(test)]
mod tests {
    use dampen_core::{HandlerSignature, generate_application, parse};

    #[test]
    fn test_progress_bar_with_static_bar_color() {
        // Build XML string without using # in raw string
        let xml = String::from(
            r#"
            <dampen version="1.1">
                <progress_bar value="50" bar_color="" />
            </dampen>
        "#,
        )
        .replace("bar_color=\"\"", "bar_color=\"\u{23}FF5733\"");

        let doc = parse(&xml).unwrap();
        let handlers: Vec<HandlerSignature> = vec![];

        let result = generate_application(&doc, "Model", "Message", &handlers);

        assert!(
            result.is_ok(),
            "Codegen should succeed for progress_bar with static bar_color"
        );

        let output = result.unwrap();
        let code = output.code.to_string();

        // Static color should be parsed at build time and inlined
        assert!(
            code.contains("iced :: Color :: from_rgb"),
            "Generated code should contain inlined color: {}",
            code
        );
    }

    #[test]
    fn test_progress_bar_with_bound_bar_color() {
        let xml = r#"
            <dampen version="1.1">
                <progress_bar value="50" bar_color="{progress_color}" />
            </dampen>
        "#;

        let doc = parse(xml).unwrap();
        let handlers: Vec<HandlerSignature> = vec![];

        let result = generate_application(&doc, "Model", "Message", &handlers);

        assert!(
            result.is_ok(),
            "Codegen should succeed for progress_bar with bound bar_color"
        );

        let output = result.unwrap();
        let code = output.code.to_string();

        // Bound color should generate runtime parsing code
        assert!(
            code.contains("let color_str"),
            "Generated code should evaluate color binding: {}",
            code
        );
        assert!(
            code.contains("strip_prefix") || code.contains("starts_with"),
            "Generated code should include color parsing logic: {}",
            code
        );
    }

    #[test]
    fn test_progress_bar_with_interpolated_bar_color() {
        // Build XML string without using # in raw string
        let xml = String::from(
            r#"
            <dampen version="1.1">
                <progress_bar value="50" bar_color="" />
            </dampen>
        "#,
        )
        .replace("bar_color=\"\"", "bar_color=\"\u{23}{color_hex}\"");

        let doc = parse(&xml).unwrap();
        let handlers: Vec<HandlerSignature> = vec![];

        let result = generate_application(&doc, "Model", "Message", &handlers);

        assert!(
            result.is_ok(),
            "Codegen should succeed for progress_bar with interpolated bar_color"
        );

        let output = result.unwrap();
        let code = output.code.to_string();

        // Interpolated color should generate format! and runtime parsing
        assert!(
            code.contains("format !"),
            "Generated code should use format! for interpolation: {}",
            code
        );
    }

    #[test]
    fn test_progress_bar_without_bar_color() {
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
            "Codegen should succeed for progress_bar without bar_color"
        );

        let output = result.unwrap();
        let code = output.code.to_string();

        // Without bar_color, should use palette color
        assert!(
            code.contains("palette . primary . base . color"),
            "Generated code should use palette color: {}",
            code
        );
    }

    #[test]
    fn test_progress_bar_with_bar_color_and_style() {
        // Build XML string without using # in raw string
        let xml = String::from(
            r#"
            <dampen version="1.1">
                <progress_bar value="50" bar_color="" style="success" />
            </dampen>
        "#,
        )
        .replace("bar_color=\"\"", "bar_color=\"\u{23}FF5733\"");

        let doc = parse(&xml).unwrap();
        let handlers: Vec<HandlerSignature> = vec![];

        let result = generate_application(&doc, "Model", "Message", &handlers);

        assert!(
            result.is_ok(),
            "Codegen should succeed for progress_bar with bar_color and style"
        );

        let output = result.unwrap();
        let code = output.code.to_string();

        // bar_color should override style
        assert!(
            code.contains("iced :: Color :: from_rgb"),
            "Generated code should use bar_color, not style: {}",
            code
        );
    }
}
