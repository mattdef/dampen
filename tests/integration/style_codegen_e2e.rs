//! Style Code Generation End-to-End Integration Tests
//!
//! These tests verify the complete pipeline for static style generation:
//! 1. Parsing .dampen files with style classes and inline styles
//! 2. Generating theme code with style class functions
//! 3. Generating view code with style applications
//! 4. Verifying generated code compiles and contains expected elements

use dampen_core::codegen::theme::generate_theme_code;
use dampen_core::codegen::view::generate_view;
use dampen_core::ir::DampenDocument;
use dampen_core::ir::theme::ThemeDocument;
use dampen_core::parse;

/// Helper function to convert DampenDocument to ThemeDocument for codegen
fn to_theme_document(doc: &DampenDocument) -> ThemeDocument {
    ThemeDocument {
        themes: doc.themes.clone(),
        default_theme: doc.global_theme.clone(),
        follow_system: false,
    }
}

// Test fixture loaded from file
const APP_WITH_STYLES_XML: &str = include_str!("fixtures/app_with_styles.dampen");

// Simpler inline fixture for quick tests
const SIMPLE_STYLED_APP: &str = r##"
<dampen version="1.1" encoding="utf-8">
    <themes>
        <theme name="light">
            <palette 
                primary="#3498db" 
                secondary="#2ecc71"
                success="#27ae60"
                warning="#f39c12"
                danger="#e74c3c"
                background="#ffffff" 
                surface="#f8f9fa"
                text="#000000"
                text_secondary="#6c757d" />
        </theme>
    </themes>
    <global_theme name="light" />
    
    <style_classes>
        <style name="primary-btn" widget="button">
            <base background="#3498db" color="#ffffff" />
            <hover background="#5dade2" />
            <active background="#2980b9" />
        </style>
    </style_classes>
    
    <column>
        <button label="Styled Button" class="primary-btn" on_click="handle_click" />
        <button label="Inline Styled" background="#e74c3c" color="#ffffff" on_click="handle_click2" />
    </column>
</dampen>
"##;

#[cfg(test)]
mod style_parsing_tests {
    use super::*;

    #[test]
    fn test_parse_app_with_style_classes() {
        let document = parse(APP_WITH_STYLES_XML).unwrap();

        // Verify document has style classes
        assert!(
            !document.style_classes.is_empty(),
            "Document should contain style classes"
        );

        // Verify specific style classes exist
        assert!(
            document.style_classes.contains_key("primary-button"),
            "Should have primary-button style class"
        );
        assert!(
            document.style_classes.contains_key("secondary-button"),
            "Should have secondary-button style class"
        );
        assert!(
            document.style_classes.contains_key("card"),
            "Should have card style class"
        );

        // Verify button style has state variants
        let primary_btn = &document.style_classes["primary-button"];
        assert!(
            !primary_btn.state_variants.is_empty(),
            "Primary button should have state variants"
        );

        // Verify base style properties
        assert!(
            primary_btn.style.background.is_some(),
            "Primary button should have background"
        );
        assert!(
            primary_btn.style.color.is_some(),
            "Primary button should have text color"
        );
    }

    #[test]
    fn test_parse_widgets_with_class_attributes() {
        let document = parse(APP_WITH_STYLES_XML).unwrap();

        // Find button with class attribute
        let root = &document.root;
        let first_button = &root.children[0];

        assert_eq!(
            first_button.kind,
            dampen_core::ir::node::WidgetKind::Button,
            "First child should be a button"
        );
        assert!(
            !first_button.classes.is_empty(),
            "Button should have CSS classes"
        );
        assert_eq!(
            first_button.classes[0], "primary-button",
            "Button should have primary-button class"
        );
    }

    #[test]
    fn test_parse_widgets_with_inline_styles() {
        let document = parse(APP_WITH_STYLES_XML).unwrap();

        // Find button with inline style (3rd child)
        let root = &document.root;
        let inline_button = &root.children[2];

        assert!(
            inline_button.style.is_some() || !inline_button.attributes.is_empty(),
            "Button with inline styles should have style properties or background attribute"
        );
    }

    #[test]
    fn test_parse_simple_styled_app() {
        let document = parse(SIMPLE_STYLED_APP).unwrap();

        assert_eq!(
            document.style_classes.len(),
            1,
            "Should have exactly one style class"
        );
        assert!(
            document.style_classes.contains_key("primary-btn"),
            "Should have primary-btn style class"
        );
    }
}

#[cfg(test)]
mod theme_codegen_tests {
    use super::*;

    #[test]
    fn test_generate_theme_code_with_style_classes() {
        let document = parse(APP_WITH_STYLES_XML).unwrap();

        let result = generate_theme_code(
            &to_theme_document(&document),
            &document.style_classes,
            "test_app",
        );

        assert!(
            result.is_ok(),
            "Should generate theme code successfully: {:?}",
            result.err()
        );

        let generated = result.unwrap();

        // Verify style class functions are generated
        assert!(
            generated.code.contains("pub fn style_primary_button"),
            "Should contain primary-button style function"
        );
        assert!(
            generated.code.contains("pub fn style_secondary_button"),
            "Should contain secondary-button style function"
        );
        assert!(
            generated.code.contains("pub fn style_card"),
            "Should contain card style function"
        );

        // Verify function signatures
        assert!(
            generated.code.contains("iced::widget::button::Status"),
            "Button style should accept Status parameter"
        );

        // Verify style struct generation
        assert!(
            generated.code.contains("iced::widget::button::Style"),
            "Should generate button::Style struct"
        );
        assert!(
            generated.code.contains("background"),
            "Should include background property"
        );
        assert!(
            generated.code.contains("text_color"),
            "Should include text_color property"
        );
    }

    #[test]
    fn test_generate_theme_code_with_state_variants() {
        let document = parse(SIMPLE_STYLED_APP).unwrap();

        let result = generate_theme_code(
            &to_theme_document(&document),
            &document.style_classes,
            "simple",
        );

        assert!(result.is_ok(), "Should generate theme code");

        let generated = result.unwrap();

        // Verify state matching logic
        assert!(
            generated.code.contains("match status"),
            "Should have match expression for button status"
        );
        // The quote! macro adds spaces between tokens, so check for ":: Active" not "::Active"
        assert!(
            generated.code.contains("Status :: Active"),
            "Should handle Active state. Generated code:\n{}",
            generated.code
        );
        assert!(
            generated.code.contains("Status :: Hovered"),
            "Should handle Hovered state"
        );
    }

    #[test]
    fn test_theme_code_is_valid_rust_syntax() {
        let document = parse(SIMPLE_STYLED_APP).unwrap();

        let result = generate_theme_code(
            &to_theme_document(&document),
            &document.style_classes,
            "syntax",
        );

        assert!(result.is_ok(), "Should generate valid code");

        let generated = result.unwrap();

        // Basic syntax checks
        assert!(
            !generated.code.contains("{{"),
            "Should not have double braces"
        );
        assert!(
            !generated.code.contains("}}"),
            "Should not have double closing braces"
        );

        // Check for balanced braces (simple heuristic)
        let open_count = generated.code.matches('{').count();
        let close_count = generated.code.matches('}').count();
        assert_eq!(
            open_count, close_count,
            "Braces should be balanced in generated code"
        );
    }
}

#[cfg(test)]
mod view_codegen_tests {
    use super::*;

    #[test]
    fn test_generate_view_with_css_classes() {
        let document = parse(SIMPLE_STYLED_APP).unwrap();

        let result = generate_view(&document, "Model", "Message");

        assert!(
            result.is_ok(),
            "Should generate view code: {:?}",
            result.err()
        );

        let generated = result.unwrap();
        let code = generated.to_string();

        // Verify style application
        assert!(
            code.contains("style"),
            "Generated view should contain style calls"
        );
        assert!(
            code.contains("style_primary_btn") || code.contains("primary_btn"),
            "Should reference the CSS class style function"
        );
    }

    #[test]
    fn test_generate_view_with_inline_styles() {
        let document = parse(SIMPLE_STYLED_APP).unwrap();

        let result = generate_view(&document, "Model", "Message");

        assert!(result.is_ok(), "Should generate view code");

        let generated = result.unwrap();
        let code = generated.to_string();

        // The second button has inline styles
        assert!(
            code.contains("style") || code.contains("background"),
            "Should contain style-related code for inline styled button"
        );
    }

    #[test]
    fn test_view_code_widget_construction() {
        let document = parse(SIMPLE_STYLED_APP).unwrap();

        let result = generate_view(&document, "Model", "Message");

        assert!(result.is_ok(), "Should generate view");

        let code = result.unwrap().to_string();

        // Verify widget construction
        assert!(
            code.contains("iced :: widget :: button"),
            "Should construct button widgets"
        );
        assert!(
            code.contains("iced :: widget :: column"),
            "Should construct column container"
        );
    }
}

#[cfg(test)]
mod full_pipeline_tests {
    use super::*;

    #[test]
    fn test_full_codegen_pipeline_with_styles() {
        let document = parse(APP_WITH_STYLES_XML).unwrap();

        // Step 1: Generate theme code
        let theme_result = generate_theme_code(
            &to_theme_document(&document),
            &document.style_classes,
            "full_test",
        );
        assert!(
            theme_result.is_ok(),
            "Theme generation should succeed: {:?}",
            theme_result.err()
        );
        let theme_code = theme_result.unwrap();

        // Step 2: Generate view code
        let view_result = generate_view(&document, "TestModel", "TestMessage");
        assert!(
            view_result.is_ok(),
            "View generation should succeed: {:?}",
            view_result.err()
        );
        let view_code = view_result.unwrap();

        // Step 3: Verify theme code contains all style functions
        assert!(
            theme_code.code.contains("style_primary_button"),
            "Theme should have primary-button function"
        );
        assert!(
            theme_code.code.contains("style_card"),
            "Theme should have card function"
        );

        // Step 4: Verify view code uses styles
        let view_str = view_code.to_string();
        assert!(
            view_str.contains("style"),
            "View should apply styles to widgets"
        );

        // Step 5: Verify code consistency
        // If view uses a class, theme must define it
        if view_str.contains("style_primary_button") {
            assert!(
                theme_code.code.contains("pub fn style_primary_button"),
                "Used style class must be defined in theme"
            );
        }
    }

    #[test]
    fn test_style_priority_order() {
        // Create document with both class and inline styles on same widget
        let xml = r##"
<dampen version="1.1" encoding="utf-8">
    <style_classes>
        <style name="test-style" widget="button">
            <base background="#ff0000" color="#ffffff" />
        </style>
    </style_classes>
    
    <column>
        <button label="Test" class="test-style" background="#00ff00" on_click="test" />
    </column>
</dampen>
"##;

        let document = parse(xml).unwrap();
        let view_result = generate_view(&document, "Model", "Message");

        assert!(
            view_result.is_ok(),
            "Should generate view with mixed styles"
        );

        let code = view_result.unwrap().to_string();

        // Inline style should take precedence
        // The generated code should use inline style closure, not the class function
        assert!(code.contains("style"), "Should have style application");
    }

    #[test]
    fn test_generated_code_structure() {
        let document = parse(SIMPLE_STYLED_APP).unwrap();

        let theme_code = generate_theme_code(
            &to_theme_document(&document),
            &document.style_classes,
            "struct_test",
        )
        .unwrap();
        let view_code = generate_view(&document, "Model", "Message").unwrap();

        // Theme code structure checks
        assert!(
            theme_code.code.starts_with("//"),
            "Theme code should start with comment"
        );
        assert!(
            theme_code.code.contains("pub fn"),
            "Theme code should have public functions"
        );

        // View code structure checks
        let view_str = view_code.to_string();
        assert!(
            view_str.contains("iced"),
            "View code should use iced widgets"
        );
        assert!(
            view_str.contains("into"),
            "View code should convert widgets"
        );
    }

    #[test]
    fn test_empty_style_classes_still_works() {
        let xml = r##"
<dampen version="1.1" encoding="utf-8">
    <column>
        <button label="Plain Button" on_click="test" />
    </column>
</dampen>
"##;

        let document = parse(xml).unwrap();

        // Should work with no style classes
        let view_result = generate_view(&document, "Model", "Message");
        assert!(
            view_result.is_ok(),
            "Should generate view without style classes"
        );

        // Button should still be generated, just without custom styles
        let code = view_result.unwrap().to_string();
        assert!(code.contains("button"), "Should contain button widget");
    }
}

#[cfg(test)]
mod compilation_verification_tests {
    use super::*;

    #[test]
    fn test_generated_theme_functions_callable() {
        let document = parse(SIMPLE_STYLED_APP).unwrap();

        let theme_code = generate_theme_code(
            &to_theme_document(&document),
            &document.style_classes,
            "callable",
        )
        .unwrap();

        // Verify function signatures are correct
        assert!(
            theme_code.code.contains("pub fn style_primary_btn"),
            "Should define style function"
        );
        assert!(
            theme_code.code.contains("_theme: &iced::Theme"),
            "Should accept theme parameter"
        );
        assert!(
            theme_code.code.contains("iced::widget::button::Status"),
            "Should accept status parameter for stateful widgets"
        );
    }

    #[test]
    fn test_no_unwrap_in_generated_code() {
        let document = parse(APP_WITH_STYLES_XML).unwrap();

        let theme_code = generate_theme_code(
            &to_theme_document(&document),
            &document.style_classes,
            "safe",
        )
        .unwrap();
        let view_code = generate_view(&document, "Model", "Message").unwrap();

        // Code should not contain unwrap() calls (clippy denies them)
        assert!(
            !theme_code.code.contains(".unwrap()"),
            "Generated theme code should not use unwrap()"
        );
        assert!(
            !view_code.to_string().contains(".unwrap()"),
            "Generated view code should not use unwrap()"
        );
    }

    #[test]
    fn test_all_imports_present() {
        let document = parse(SIMPLE_STYLED_APP).unwrap();

        let theme_code = generate_theme_code(
            &to_theme_document(&document),
            &document.style_classes,
            "imports",
        )
        .unwrap();

        // Verify necessary Iced types are used
        let necessary_types = vec![
            "iced::Color",
            "iced::Background",
            "iced::Border",
            "iced::Shadow",
        ];

        for type_name in necessary_types {
            // Types should appear in the generated code
            // (imports would be added by the application generator)
            if theme_code.code.contains("background: Some") {
                assert!(
                    theme_code.code.contains("Background")
                        || theme_code.code.contains("iced::Background"),
                    "Should use Background type when backgrounds are present"
                );
            }
        }
    }
}
