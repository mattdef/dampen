//! Theme end-to-end integration tests
//!
//! These tests verify that the theming system works correctly
//! in integrated scenarios combining multiple components.

use dampen_core::ir::style::Color;
use dampen_core::ir::theme::{SpacingScale, Theme, ThemeDocument, ThemePalette, Typography};
use dampen_core::parse;
use dampen_core::parser::theme_parser::parse_theme_document;
use dampen_core::state::AppState;
use dampen_core::state::ThemeContext;
use std::collections::HashMap;

const SIMPLE_APP_XML: &str = r#"
<dampen version="1.1" encoding="utf-8">
    <column>
        <text value="Hello, Theming!" />
        <button label="Click me" />
    </column>
</dampen>
"#;

const THEME_WITH_LIGHT_DARK: &str = r##"<dampen version="1.1" encoding="utf-8">
    <themes>
        <theme name="light">
            <palette
                primary="#3498db"
                secondary="#2ecc71"
                success="#27ae60"
                warning="#f39c12"
                danger="#e74c3c"
                background="#ecf0f1"
                surface="#ffffff"
                text="#2c3e50"
                text_secondary="#7f8c8d" />
            <typography font_family="sans-serif" font_size_base="16" font_size_small="12" font_size_large="24" font_weight="normal" line_height="1.5" />
            <spacing unit="8" />
        </theme>
        <theme name="dark">
            <palette
                primary="#5dade2"
                secondary="#52be80"
                success="#27ae60"
                warning="#f39c12"
                danger="#ec7063"
                background="#2c3e50"
                surface="#34495e"
                text="#ecf0f1"
                text_secondary="#95a5a6" />
            <typography font_family="sans-serif" font_size_base="16" font_size_small="12" font_size_large="24" font_weight="normal" line_height="1.5" />
            <spacing unit="8" />
        </theme>
    </themes>
    <default_theme name="light" />
    <follow_system enabled="true" />
</dampen>
"##;

fn create_test_palette() -> ThemePalette {
    ThemePalette {
        primary: Some(Color::from_hex("#3498db").unwrap()),
        secondary: Some(Color::from_hex("#2ecc71").unwrap()),
        success: Some(Color::from_hex("#27ae60").unwrap()),
        warning: Some(Color::from_hex("#f39c12").unwrap()),
        danger: Some(Color::from_hex("#e74c3c").unwrap()),
        background: Some(Color::from_hex("#ecf0f1").unwrap()),
        surface: Some(Color::from_hex("#ffffff").unwrap()),
        text: Some(Color::from_hex("#2c3e50").unwrap()),
        text_secondary: Some(Color::from_hex("#7f8c8d").unwrap()),
    }
}

fn create_test_theme(name: &str) -> Theme {
    Theme {
        name: name.to_string(),
        palette: create_test_palette(),
        typography: Typography {
            font_family: Some("sans-serif".to_string()),
            font_size_base: Some(16.0),
            font_size_small: Some(12.0),
            font_size_large: Some(24.0),
            font_weight: dampen_core::ir::theme::FontWeight::Normal,
            line_height: Some(1.5),
        },
        spacing: SpacingScale { unit: Some(8.0) },
        base_styles: HashMap::new(),
        extends: None,
    }
}

fn create_light_dark_document() -> ThemeDocument {
    ThemeDocument {
        themes: HashMap::from([
            ("light".to_string(), create_test_theme("light")),
            ("dark".to_string(), create_test_theme("dark")),
        ]),
        default_theme: Some("light".to_string()),
        follow_system: true,
    }
}

#[cfg(test)]
mod contract_theme_loading_integration {
    use super::*;

    #[test]
    fn integration_app_state_without_theme_context() {
        let document = parse(SIMPLE_APP_XML).unwrap();
        let state: AppState = AppState::new(document);

        assert!(state.theme_context().is_none());
    }

    #[test]
    fn integration_app_state_with_theme_context() {
        let document = parse(SIMPLE_APP_XML).unwrap();

        let theme_doc = ThemeDocument {
            themes: HashMap::from([
                ("light".to_string(), create_test_theme("light")),
                ("dark".to_string(), create_test_theme("dark")),
            ]),
            default_theme: Some("light".to_string()),
            follow_system: true,
        };

        let theme_context = ThemeContext::from_document(theme_doc, None).unwrap();

        let mut state: AppState = AppState::new(document);
        state.set_theme_context(theme_context);

        assert!(state.theme_context().is_some());
        assert_eq!(state.theme_context().unwrap().active_name(), "light");
    }

    #[test]
    fn integration_parse_theme_document() {
        let result = parse_theme_document(THEME_WITH_LIGHT_DARK);

        assert!(result.is_ok(), "Should parse valid theme XML");
        let doc = result.unwrap();

        assert_eq!(doc.themes.len(), 2, "Should have 2 themes");
        assert!(doc.themes.contains_key("light"));
        assert!(doc.themes.contains_key("dark"));
        assert_eq!(doc.default_theme, Some("light".to_string()));
        assert!(doc.follow_system);
    }
}

#[cfg(test)]
mod contract_runtime_theme_switching_integration {
    use super::*;

    #[test]
    fn integration_runtime_theme_switch_changes_active() {
        let document = parse(SIMPLE_APP_XML).unwrap();

        let theme_doc = create_light_dark_document();

        let theme_context = ThemeContext::from_document(theme_doc, None).unwrap();

        let mut state: AppState = AppState::new(document);
        state.set_theme_context(theme_context);

        assert_eq!(state.theme_context().unwrap().active_name(), "light");

        state
            .theme_context_mut()
            .unwrap()
            .set_theme("dark")
            .unwrap();
        assert_eq!(state.theme_context().unwrap().active_name(), "dark");

        state
            .theme_context_mut()
            .unwrap()
            .set_theme("light")
            .unwrap();
        assert_eq!(state.theme_context().unwrap().active_name(), "light");
    }

    #[test]
    fn integration_set_theme_nonexistent_theme_fails() {
        let document = parse(SIMPLE_APP_XML).unwrap();

        let theme_doc = ThemeDocument {
            themes: HashMap::from([("light".to_string(), create_test_theme("light"))]),
            default_theme: Some("light".to_string()),
            follow_system: false,
        };

        let theme_context = ThemeContext::from_document(theme_doc, None).unwrap();

        let mut state: AppState = AppState::new(document);
        state.set_theme_context(theme_context);

        let result = state.theme_context_mut().unwrap().set_theme("nonexistent");
        assert!(result.is_err());

        assert_eq!(state.theme_context().unwrap().active_name(), "light");
    }
}

#[cfg(test)]
mod contract_theme_hot_reload_integration {
    use super::*;

    fn create_test_palette_with_primary(primary: &str) -> ThemePalette {
        ThemePalette {
            primary: Some(Color::from_hex(primary).unwrap()),
            secondary: Some(Color::from_hex("#2ecc71").unwrap()),
            success: Some(Color::from_hex("#27ae60").unwrap()),
            warning: Some(Color::from_hex("#f39c12").unwrap()),
            danger: Some(Color::from_hex("#e74c3c").unwrap()),
            background: Some(Color::from_hex("#ecf0f1").unwrap()),
            surface: Some(Color::from_hex("#ffffff").unwrap()),
            text: Some(Color::from_hex("#2c3e50").unwrap()),
            text_secondary: Some(Color::from_hex("#7f8c8d").unwrap()),
        }
    }

    fn create_test_theme_with_primary(name: &str, primary: &str) -> Theme {
        Theme {
            name: name.to_string(),
            palette: create_test_palette_with_primary(primary),
            typography: Typography {
                font_family: Some("sans-serif".to_string()),
                font_size_base: Some(16.0),
                font_size_small: Some(12.0),
                font_size_large: Some(24.0),
                font_weight: dampen_core::ir::theme::FontWeight::Normal,
                line_height: Some(1.5),
            },
            spacing: SpacingScale { unit: Some(8.0) },
            base_styles: HashMap::new(),
            extends: None,
        }
    }

    fn create_light_dark_document_with_primaries(
        light_primary: &str,
        dark_primary: &str,
    ) -> ThemeDocument {
        ThemeDocument {
            themes: HashMap::from([
                (
                    "light".to_string(),
                    create_test_theme_with_primary("light", light_primary),
                ),
                (
                    "dark".to_string(),
                    create_test_theme_with_primary("dark", dark_primary),
                ),
            ]),
            default_theme: Some("light".to_string()),
            follow_system: true,
        }
    }

    #[test]
    fn integration_hot_reload_updates_theme_colors() {
        let document = parse(SIMPLE_APP_XML).unwrap();

        let doc = create_light_dark_document_with_primaries("#3498db", "#5dade2");
        let mut ctx = ThemeContext::from_document(doc, None).unwrap();

        assert_eq!(ctx.active_name(), "light");
        assert_eq!(
            ctx.active().palette.primary,
            Some(Color::from_hex("#3498db").unwrap())
        );

        let new_doc = create_light_dark_document_with_primaries("#ff0000", "#0000ff");

        ctx.reload(new_doc);

        assert_eq!(ctx.active_name(), "light");
        assert_eq!(
            ctx.active().palette.primary,
            Some(Color::from_hex("#ff0000").unwrap())
        );
    }

    #[test]
    fn integration_hot_reload_preserves_active_theme() {
        let document = parse(SIMPLE_APP_XML).unwrap();

        let doc = create_light_dark_document_with_primaries("#3498db", "#5dade2");
        let mut ctx = ThemeContext::from_document(doc, None).unwrap();

        ctx.set_theme("dark").unwrap();
        assert_eq!(ctx.active_name(), "dark");

        let new_doc = ThemeDocument {
            themes: HashMap::from([
                (
                    "light".to_string(),
                    create_test_theme_with_primary("light", "#ff0000"),
                ),
                (
                    "dark".to_string(),
                    create_test_theme_with_primary("dark", "#00ff00"),
                ),
            ]),
            default_theme: Some("light".to_string()),
            follow_system: true,
        };

        ctx.reload(new_doc);

        assert_eq!(ctx.active_name(), "dark");
        assert_eq!(
            ctx.active().palette.primary,
            Some(Color::from_hex("#00ff00").unwrap())
        );
    }
}

#[cfg(test)]
mod contract_theme_inheritance_integration {
    use super::*;

    const THEME_WITH_INHERITANCE: &str = r##"<dampen version="1.1" encoding="utf-8">
    <themes>
        <theme name="base">
            <palette
                primary="#3498db"
                secondary="#2ecc71"
                success="#27ae60"
                warning="#f39c12"
                danger="#e74c3c"
                background="#ecf0f1"
                surface="#ffffff"
                text="#2c3e50"
                text_secondary="#7f8c8d" />
            <typography font_family="sans-serif" font_size_base="16" font_size_small="12" font_size_large="24" font_weight="normal" line_height="1.5" />
            <spacing unit="8" />
        </theme>
        <theme name="light" extends="base">
            <palette
                background="#ecf0f1"
                surface="#ffffff"
                text="#2c3e50"
                text_secondary="#7f8c8d" />
        </theme>
        <theme name="dark" extends="base">
            <palette
                background="#2c3e50"
                surface="#34495e"
                text="#ecf0f1"
                text_secondary="#95a5a6" />
        </theme>
    </themes>
    <default_theme name="light" />
    <follow_system enabled="true" />
</dampen>
"##;

    #[test]
    fn integration_parse_theme_with_inheritance() {
        let result = parse_theme_document(THEME_WITH_INHERITANCE);

        assert!(result.is_ok(), "Should parse theme with inheritance");
        let doc = result.unwrap();

        assert_eq!(doc.themes.len(), 3, "Should have 3 themes");
        assert!(doc.themes.contains_key("base"));
        assert!(doc.themes.contains_key("light"));
        assert!(doc.themes.contains_key("dark"));

        let light = doc.themes.get("light").unwrap();
        assert_eq!(light.extends, Some("base".to_string()));

        let dark = doc.themes.get("dark").unwrap();
        assert_eq!(dark.extends, Some("base".to_string()));
    }

    #[test]
    fn integration_theme_inheritance_resolution() {
        let result = parse_theme_document(THEME_WITH_INHERITANCE);
        assert!(result.is_ok());

        let doc = result.unwrap();
        let resolved = doc.resolve_inheritance();

        assert!(resolved.contains_key("light"));
        assert!(resolved.contains_key("dark"));

        let light = resolved.get("light").unwrap();
        assert!(light.palette.primary.is_some());
        assert_eq!(light.palette.primary.unwrap().r, 0x34 as f32 / 255.0);

        let dark = resolved.get("dark").unwrap();
        assert!(dark.palette.primary.is_some());
        assert_eq!(dark.palette.primary.unwrap().r, 0x34 as f32 / 255.0);
    }

    #[test]
    fn integration_validate_inheritance() {
        let result = parse_theme_document(THEME_WITH_INHERITANCE);
        assert!(result.is_ok());

        let doc = result.unwrap();
        let validation_result = doc.validate_inheritance();

        assert!(
            validation_result.is_ok(),
            "Inheritance validation should pass: {:?}",
            validation_result
        );
    }

    #[test]
    fn integration_circular_inheritance_detection() {
        const CIRCULAR_THEME: &str = r##"<dampen version="1.1" encoding="utf-8">
    <themes>
        <theme name="theme_a" extends="theme_b">
            <palette primary="#3498db" secondary="#2ecc71" success="#27ae60" warning="#f39c12" danger="#e74c3c" background="#ecf0f1" surface="#ffffff" text="#2c3e50" text_secondary="#7f8c8d" />
        </theme>
        <theme name="theme_b" extends="theme_a">
            <palette primary="#3498db" secondary="#2ecc71" success="#27ae60" warning="#f39c12" danger="#e74c3c" background="#ecf0f1" surface="#ffffff" text="#2c3e50" text_secondary="#7f8c8d" />
        </theme>
    </themes>
    <default_theme name="theme_a" />
    <follow_system enabled="false" />
</dampen>
"##;

        let result = parse_theme_document(CIRCULAR_THEME);
        assert!(result.is_ok());

        let doc = result.unwrap();
        let validation_result = doc.validate_inheritance();

        assert!(
            validation_result.is_err(),
            "Should detect circular inheritance"
        );
        let err = validation_result.unwrap_err();
        assert!(err.message.contains("Circular") || err.message.contains("THEME_007"));
    }

    #[test]
    fn integration_missing_parent_theme_detection() {
        const MISSING_PARENT: &str = r##"<dampen version="1.1" encoding="utf-8">
    <themes>
        <theme name="child" extends="nonexistent">
            <palette primary="#3498db" secondary="#2ecc71" success="#27ae60" warning="#f39c12" danger="#e74c3c" background="#ecf0f1" surface="#ffffff" text="#2c3e50" text_secondary="#7f8c8d" />
        </theme>
    </themes>
    <default_theme name="child" />
    <follow_system enabled="false" />
</dampen>
"##;

        let result = parse_theme_document(MISSING_PARENT);
        assert!(result.is_ok());

        let doc = result.unwrap();
        let validation_result = doc.validate_inheritance();

        assert!(
            validation_result.is_err(),
            "Should detect missing parent theme"
        );
        let err = validation_result.unwrap_err();
        assert!(err.message.contains("not found") || err.message.contains("THEME_006"));
    }
}

#[cfg(test)]
mod contract_widget_level_overrides_integration {
    use super::*;

    const APP_WITH_OVERRIDES: &str = r##"
<dampen version="1.1" encoding="utf-8">
    <style_classes>
        <style name="custom_button"
            background="#ff0000"
            color="#ffff00" />
        <style name="custom_container"
            background="#0000ff" />
    </style_classes>
    <column>
        <button id="theme_default" label="Theme Default" />
        <button id="class_override" class="custom_button" label="Class Override" />
        <button id="inline_override" background="#00ff00" color="#ff00ff" label="Inline Override" />
        <button id="class_and_inline" class="custom_button" background="#00ff00" label="Class + Inline" />
        <container id="theme_container" />
        <container id="class_container" class="custom_container" />
    </column>
</dampen>
"##;

    #[test]
    fn integration_parse_app_with_style_overrides() {
        let document = parse(APP_WITH_OVERRIDES).unwrap();

        assert!(document.style_classes.contains_key("custom_button"));
        assert!(document.style_classes.contains_key("custom_container"));
    }

    #[test]
    fn integration_widget_with_no_override_uses_theme_defaults() {
        let document = parse(APP_WITH_OVERRIDES).unwrap();

        let button_node = document
            .root
            .children
            .iter()
            .find(|n| {
                n.id.as_ref()
                    .map(|id| id == "theme_default")
                    .unwrap_or(false)
            })
            .expect("Should find theme_default button");

        assert!(button_node.classes.is_empty());
        assert!(button_node.style.is_none());
    }

    #[test]
    fn integration_widget_with_class_override() {
        let document = parse(APP_WITH_OVERRIDES).unwrap();

        let button_node = document
            .root
            .children
            .iter()
            .find(|n| {
                n.id.as_ref()
                    .map(|id| id == "class_override")
                    .unwrap_or(false)
            })
            .expect("Should find class_override button");

        assert_eq!(button_node.classes, vec!["custom_button"]);
        assert!(button_node.style.is_none());
    }

    #[test]
    fn integration_widget_with_inline_override() {
        let document = parse(APP_WITH_OVERRIDES).unwrap();

        let button_node = document
            .root
            .children
            .iter()
            .find(|n| {
                n.id.as_ref()
                    .map(|id| id == "inline_override")
                    .unwrap_or(false)
            })
            .expect("Should find inline_override button");

        assert!(button_node.classes.is_empty());
        assert!(button_node.style.is_some());
    }

    #[test]
    fn integration_widget_with_class_and_inline_override() {
        let document = parse(APP_WITH_OVERRIDES).unwrap();

        let button_node = document
            .root
            .children
            .iter()
            .find(|n| {
                n.id.as_ref()
                    .map(|id| id == "class_and_inline")
                    .unwrap_or(false)
            })
            .expect("Should find class_and_inline button");

        assert_eq!(button_node.classes, vec!["custom_button"]);
        assert!(button_node.style.is_some());
    }

    #[test]
    fn integration_container_style_overrides() {
        let document = parse(APP_WITH_OVERRIDES).unwrap();

        let theme_container = document
            .root
            .children
            .iter()
            .find(|n| {
                n.id.as_ref()
                    .map(|id| id == "theme_container")
                    .unwrap_or(false)
            })
            .expect("Should find theme_container");

        let class_container = document
            .root
            .children
            .iter()
            .find(|n| {
                n.id.as_ref()
                    .map(|id| id == "class_container")
                    .unwrap_or(false)
            })
            .expect("Should find class_container");

        assert!(theme_container.classes.is_empty());
        assert_eq!(class_container.classes, vec!["custom_container"]);
    }

    #[test]
    fn integration_style_class_defined_in_document() {
        let document = parse(APP_WITH_OVERRIDES).unwrap();

        let custom_button = document
            .style_classes
            .get("custom_button")
            .expect("Should have custom_button style class");

        let bg_color = custom_button
            .style
            .background
            .as_ref()
            .expect("Should have background");
        match bg_color {
            dampen_core::ir::style::Background::Color(c) => {
                assert_eq!(c.r, 1.0); // #ff0000
                assert_eq!(c.g, 0.0);
                assert_eq!(c.b, 0.0);
            }
            _ => panic!("Expected Color background"),
        }
    }
}

#[cfg(test)]
mod contract_theme_codegen_integration {
    use super::*;
    use dampen_core::codegen::theme::generate_theme_code;
    use dampen_core::ir::style::Color;
    use dampen_core::ir::theme::{SpacingScale, Theme, ThemeDocument, ThemePalette, Typography};

    fn create_test_palette_with_hex(hex: &str) -> ThemePalette {
        ThemePalette {
            primary: Some(Color::from_hex(hex).unwrap()),
            secondary: Some(Color::from_hex("#2ecc71").unwrap()),
            success: Some(Color::from_hex("#27ae60").unwrap()),
            warning: Some(Color::from_hex("#f39c12").unwrap()),
            danger: Some(Color::from_hex("#e74c3c").unwrap()),
            background: Some(Color::from_hex("#ecf0f1").unwrap()),
            surface: Some(Color::from_hex("#ffffff").unwrap()),
            text: Some(Color::from_hex("#2c3e50").unwrap()),
            text_secondary: Some(Color::from_hex("#7f8c8d").unwrap()),
        }
    }

    fn create_test_theme(name: &str, primary_hex: &str) -> Theme {
        Theme {
            name: name.to_string(),
            palette: create_test_palette_with_hex(primary_hex),
            typography: Typography {
                font_family: Some("sans-serif".to_string()),
                font_size_base: Some(16.0),
                font_size_small: Some(12.0),
                font_size_large: Some(24.0),
                font_weight: dampen_core::ir::theme::FontWeight::Normal,
                line_height: Some(1.5),
            },
            spacing: SpacingScale { unit: Some(8.0) },
            base_styles: HashMap::new(),
            extends: None,
        }
    }

    #[test]
    fn integration_codegen_light_dark_themes() {
        let doc = ThemeDocument {
            themes: HashMap::from([
                ("light".to_string(), create_test_theme("light", "#3498db")),
                ("dark".to_string(), create_test_theme("dark", "#5dade2")),
            ]),
            default_theme: Some("light".to_string()),
            follow_system: true,
        };

        let result =
            generate_theme_code(&doc, &std::collections::HashMap::new(), "integration_test");

        assert!(result.is_ok(), "Codegen should succeed: {:?}", result.err());
        let code = result.unwrap().code;

        assert!(code.contains("pub fn integration_test_theme()"));
        assert!(code.contains("pub fn integration_test_themes()"));
        assert!(code.contains("fn integration_test_light()"));
        assert!(code.contains("fn integration_test_dark()"));
    }

    #[test]
    fn integration_codegen_creates_valid_rust() {
        let doc = ThemeDocument {
            themes: HashMap::from([("test".to_string(), create_test_theme("test", "#ff0000"))]),
            default_theme: Some("test".to_string()),
            follow_system: false,
        };

        let result = generate_theme_code(&doc, &std::collections::HashMap::new(), "syntax_test");

        assert!(result.is_ok());
        let code = result.unwrap().code;

        assert!(code.contains("pub fn syntax_test_theme()"));
        assert!(code.contains("fn syntax_test_test()"));
    }

    #[test]
    fn integration_codegen_includes_all_themes() {
        let doc = ThemeDocument {
            themes: HashMap::from([
                ("light".to_string(), create_test_theme("light", "#3498db")),
                ("dark".to_string(), create_test_theme("dark", "#5dade2")),
                (
                    "high-contrast".to_string(),
                    create_test_theme("high-contrast", "#000000"),
                ),
            ]),
            default_theme: Some("dark".to_string()),
            follow_system: false,
        };

        let result = generate_theme_code(&doc, &std::collections::HashMap::new(), "multi_theme");

        assert!(result.is_ok());
        let code = result.unwrap().code;

        assert!(code.contains("themes.insert(\"light\""));
        assert!(code.contains("themes.insert(\"dark\""));
        assert!(code.contains("themes.insert(\"high-contrast\""));
        assert!(
            code.contains("\"dark\""),
            "Should have dark as default theme"
        );
    }

    #[test]
    fn integration_codegen_single_theme() {
        let doc = ThemeDocument {
            themes: HashMap::from([("brand".to_string(), create_test_theme("brand", "#FF6B35"))]),
            default_theme: Some("brand".to_string()),
            follow_system: false,
        };

        let result = generate_theme_code(&doc, &std::collections::HashMap::new(), "brand_theme");

        assert!(result.is_ok());
        let code = result.unwrap().code;

        assert!(code.contains("fn brand_theme_brand()"));
        assert!(code.contains("\"brand\""));
    }

    #[test]
    fn integration_codegen_empty_themes_fails() {
        let doc = ThemeDocument {
            themes: HashMap::new(),
            default_theme: None,
            follow_system: true,
        };

        let result = generate_theme_code(&doc, &std::collections::HashMap::new(), "empty_test");

        assert!(result.is_err(), "Should fail with no themes");
        let err = result.unwrap_err();
        assert!(
            err.contains("THEME_001") || err.contains("no themes"),
            "Error should mention THEME_001: {}",
            err
        );
    }

    #[test]
    fn integration_codegen_follow_system_flag() {
        let doc_follows = ThemeDocument {
            themes: HashMap::from([("light".to_string(), create_test_theme("light", "#3498db"))]),
            default_theme: Some("light".to_string()),
            follow_system: true,
        };

        let result_follows =
            generate_theme_code(&doc_follows, &std::collections::HashMap::new(), "follows");
        assert!(result_follows.is_ok());
        assert!(result_follows.unwrap().code.contains("true"));

        let doc_no_follow = ThemeDocument {
            themes: HashMap::from([("light".to_string(), create_test_theme("light", "#3498db"))]),
            default_theme: Some("light".to_string()),
            follow_system: false,
        };

        let result_no_follow = generate_theme_code(
            &doc_no_follow,
            &std::collections::HashMap::new(),
            "no_follow",
        );
        assert!(result_no_follow.is_ok());
        assert!(result_no_follow.unwrap().code.contains("false"));
    }
}
