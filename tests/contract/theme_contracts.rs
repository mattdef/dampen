//! Theme contract tests
//!
//! Contract tests for the theming system following Constitution Principle V (Test-First Development).
//! These tests define the expected API behavior and MUST fail before implementation is complete.

use dampen_core::ir::theme::{Theme, ThemeDocument, ThemePalette};
use dampen_core::parser::theme_parser::parse_theme_document;
use dampen_core::state::ThemeContext;

const VALID_THEME_XML: &str = include_str!("fixtures/valid_theme.dampen");

fn create_test_palette() -> ThemePalette {
    ThemePalette {
        primary: Some(dampen_core::ir::style::Color::from_hex("#3498db").unwrap()),
        secondary: Some(dampen_core::ir::style::Color::from_hex("#2ecc71").unwrap()),
        success: Some(dampen_core::ir::style::Color::from_hex("#27ae60").unwrap()),
        warning: Some(dampen_core::ir::style::Color::from_hex("#f39c12").unwrap()),
        danger: Some(dampen_core::ir::style::Color::from_hex("#e74c3c").unwrap()),
        background: Some(dampen_core::ir::style::Color::from_hex("#ecf0f1").unwrap()),
        surface: Some(dampen_core::ir::style::Color::from_hex("#ffffff").unwrap()),
        text: Some(dampen_core::ir::style::Color::from_hex("#2c3e50").unwrap()),
        text_secondary: Some(dampen_core::ir::style::Color::from_hex("#7f8c8d").unwrap()),
    }
}

fn create_test_theme() -> Theme {
    Theme {
        name: "light".to_string(),
        palette: create_test_palette(),
        typography: dampen_core::ir::theme::Typography {
            font_family: Some("sans-serif".to_string()),
            font_size_base: Some(16.0),
            font_size_small: Some(12.0),
            font_size_large: Some(24.0),
            font_weight: dampen_core::ir::theme::FontWeight::Normal,
            line_height: Some(1.5),
        },
        spacing: dampen_core::ir::theme::SpacingScale { unit: Some(8.0) },
        base_styles: std::collections::HashMap::new(),
        extends: None,
    }
}

fn create_test_theme_document() -> ThemeDocument {
    ThemeDocument {
        themes: std::collections::HashMap::from([("light".to_string(), create_test_theme())]),
        default_theme: Some("light".to_string()),
        follow_system: true,
    }
}

#[cfg(test)]
mod contract_parse_valid_theme_document {
    use super::*;

    #[test]
    fn contract_parse_valid_theme_document() {
        let result = parse_theme_document(VALID_THEME_XML);

        assert!(result.is_ok(), "Should parse valid theme XML");
        let doc = result.unwrap();

        assert_eq!(doc.themes.len(), 2, "Should have 2 themes");
        assert!(
            doc.themes.contains_key("light"),
            "Should contain 'light' theme"
        );
        assert!(
            doc.themes.contains_key("dark"),
            "Should contain 'dark' theme"
        );
        assert_eq!(doc.default_theme, Some("light".to_string()));
        assert!(doc.follow_system);
    }
}

#[cfg(test)]
mod contract_validation_missing_palette_color {
    use super::*;

    #[test]
    fn contract_validation_missing_palette_color() {
        let xml = r##"
            <dampen version="1.0">
                <themes>
                    <theme name="incomplete">
                        <palette primary="#3498db" />
                    </theme>
                </themes>
            </dampen>
        "##;

        let result = parse_theme_document(xml);
        assert!(result.is_err(), "Should fail with missing palette colors");

        let err = result.unwrap_err();
        assert!(
            err.message.contains("missing") && err.message.contains("required color"),
            "Error should mention missing required color: {}",
            err.message
        );
    }
}

#[cfg(test)]
mod contract_validation_invalid_default_theme {
    use super::*;

    #[test]
    fn contract_validation_invalid_default_theme() {
        let xml = r##"
            <dampen version="1.0">
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
                    </theme>
                </themes>
                <default_theme name="nonexistent" />
            </dampen>
        "##;

        let result = parse_theme_document(xml);
        assert!(result.is_err(), "Should fail with invalid default theme");

        let err = result.unwrap_err();
        assert!(
            err.message.contains("not found"),
            "Error should mention theme not found: {}",
            err.message
        );
    }
}

#[cfg(test)]
mod contract_validation_no_themes {
    use super::*;

    #[test]
    fn contract_validation_no_themes() {
        let xml = r##"
            <dampen version="1.0">
                <themes>
                </themes>
            </dampen>
        "##;

        let result = parse_theme_document(xml);
        assert!(result.is_err(), "Should fail with no themes");
        let err = result.unwrap_err();
        assert!(
            err.message.contains("At least one theme"),
            "Error should mention no themes defined: {}",
            err.message
        );
    }
}

#[cfg(test)]
mod contract_theme_document_validation {
    use super::*;

    #[test]
    fn contract_theme_document_validate_empty_themes() {
        let doc = ThemeDocument {
            themes: std::collections::HashMap::new(),
            default_theme: None,
            follow_system: true,
        };

        let result = doc.validate();
        assert!(result.is_err(), "Should fail with no themes");
    }

    #[test]
    fn contract_theme_document_validate_missing_default() {
        let doc = ThemeDocument {
            themes: std::collections::HashMap::from([("light".to_string(), create_test_theme())]),
            default_theme: Some("nonexistent".to_string()),
            follow_system: true,
        };

        let result = doc.validate();
        assert!(result.is_err(), "Should fail with missing default theme");
    }

    #[test]
    fn contract_theme_document_validate_success() {
        let doc = create_test_theme_document();
        let result = doc.validate();
        assert!(result.is_ok(), "Should succeed with valid document");
    }

    #[test]
    fn contract_effective_default_with_system() {
        let doc = ThemeDocument {
            themes: std::collections::HashMap::from([
                ("light".to_string(), create_test_theme()),
                ("dark".to_string(), create_test_theme()),
            ]),
            default_theme: Some("light".to_string()),
            follow_system: true,
        };

        assert_eq!(doc.effective_default(Some("dark")), "dark");
        assert_eq!(doc.effective_default(Some("light")), "light");
    }

    #[test]
    fn contract_effective_default_fallback() {
        let doc = ThemeDocument {
            themes: std::collections::HashMap::from([("light".to_string(), create_test_theme())]),
            default_theme: None,
            follow_system: true,
        };

        assert_eq!(doc.effective_default(None), "light");
        assert_eq!(doc.effective_default(Some("dark")), "light");
    }
}

#[cfg(test)]
mod contract_palette_validation {
    use super::*;
    use dampen_core::ir::theme::ThemePalette;

    #[test]
    fn contract_palette_has_all_colors() {
        let palette = create_test_palette();
        assert!(palette.primary.is_some());
        assert!(palette.primary.as_ref().unwrap().validate().is_ok());
        assert!(palette.secondary.is_some());
        assert!(palette.secondary.as_ref().unwrap().validate().is_ok());
        assert!(palette.success.is_some());
        assert!(palette.success.as_ref().unwrap().validate().is_ok());
        assert!(palette.warning.is_some());
        assert!(palette.warning.as_ref().unwrap().validate().is_ok());
        assert!(palette.danger.is_some());
        assert!(palette.danger.as_ref().unwrap().validate().is_ok());
        assert!(palette.background.is_some());
        assert!(palette.background.as_ref().unwrap().validate().is_ok());
        assert!(palette.surface.is_some());
        assert!(palette.surface.as_ref().unwrap().validate().is_ok());
        assert!(palette.text.is_some());
        assert!(palette.text.as_ref().unwrap().validate().is_ok());
        assert!(palette.text_secondary.is_some());
        assert!(palette.text_secondary.as_ref().unwrap().validate().is_ok());
    }
}

#[cfg(test)]
mod contract_color_from_hex {
    use super::*;
    use dampen_core::ir::style::Color;

    #[test]
    fn contract_color_from_hex_short() {
        let color = Color::from_hex("#f00").unwrap();
        assert_eq!(color.r, 1.0);
        assert_eq!(color.g, 0.0);
        assert_eq!(color.b, 0.0);
        assert_eq!(color.a, 1.0);
    }

    #[test]
    fn contract_color_from_hex_long() {
        let color = Color::from_hex("#ff0000").unwrap();
        assert_eq!(color.r, 1.0);
        assert_eq!(color.g, 0.0);
        assert_eq!(color.b, 0.0);
        assert_eq!(color.a, 1.0);
    }

    #[test]
    fn contract_color_to_hex() {
        let color = Color::from_hex("#ff0000").unwrap();
        assert_eq!(color.to_hex(), "#ff0000");
    }
}

#[cfg(test)]
mod contract_theme_context_creation {
    use super::*;
    use dampen_core::ir::style::Color;
    use dampen_core::ir::theme::{SpacingScale, Typography};

    fn create_test_theme(name: &str) -> Theme {
        Theme {
            name: name.to_string(),
            palette: ThemePalette {
                primary: Some(Color::from_hex("#3498db").unwrap()),
                secondary: Some(Color::from_hex("#2ecc71").unwrap()),
                success: Some(Color::from_hex("#27ae60").unwrap()),
                warning: Some(Color::from_hex("#f39c12").unwrap()),
                danger: Some(Color::from_hex("#e74c3c").unwrap()),
                background: Some(Color::from_hex("#ecf0f1").unwrap()),
                surface: Some(Color::from_hex("#ffffff").unwrap()),
                text: Some(Color::from_hex("#2c3e50").unwrap()),
                text_secondary: Some(Color::from_hex("#7f8c8d").unwrap()),
            },
            typography: Typography {
                font_family: Some("sans-serif".to_string()),
                font_size_base: Some(16.0),
                font_size_small: Some(12.0),
                font_size_large: Some(24.0),
                font_weight: dampen_core::ir::theme::FontWeight::Normal,
                line_height: Some(1.5),
            },
            spacing: SpacingScale { unit: Some(8.0) },
            base_styles: std::collections::HashMap::new(),
            extends: None,
        }
    }

    fn create_light_dark_document() -> ThemeDocument {
        ThemeDocument {
            themes: std::collections::HashMap::from([
                ("light".to_string(), create_test_theme("light")),
                ("dark".to_string(), create_test_theme("dark")),
            ]),
            default_theme: Some("light".to_string()),
            follow_system: true,
        }
    }

    #[test]
    fn contract_theme_context_with_system_preference_dark() {
        let doc = create_light_dark_document();
        let ctx = ThemeContext::from_document(doc, Some("dark"));

        assert!(ctx.is_ok(), "Should create context successfully");
        let ctx = ctx.unwrap();
        assert_eq!(ctx.active_name(), "dark");
    }

    #[test]
    fn contract_theme_context_with_system_preference_light() {
        let doc = create_light_dark_document();
        let ctx = ThemeContext::from_document(doc, Some("light"));

        assert!(ctx.is_ok(), "Should create context successfully");
        let ctx = ctx.unwrap();
        assert_eq!(ctx.active_name(), "light");
    }

    #[test]
    fn contract_theme_context_follows_system_when_enabled() {
        let doc = ThemeDocument {
            themes: std::collections::HashMap::from([
                ("light".to_string(), create_test_theme("light")),
                ("dark".to_string(), create_test_theme("dark")),
            ]),
            default_theme: Some("light".to_string()),
            follow_system: true,
        };

        let ctx = ThemeContext::from_document(doc, Some("dark")).unwrap();
        assert_eq!(ctx.active_name(), "dark");
    }

    #[test]
    fn contract_theme_context_ignores_system_when_disabled() {
        let doc = ThemeDocument {
            themes: std::collections::HashMap::from([
                ("light".to_string(), create_test_theme("light")),
                ("dark".to_string(), create_test_theme("dark")),
            ]),
            default_theme: Some("light".to_string()),
            follow_system: false,
        };

        let ctx = ThemeContext::from_document(doc, Some("dark")).unwrap();
        assert_eq!(ctx.active_name(), "light");
    }

    #[test]
    fn contract_theme_context_fallback_to_light() {
        let doc = ThemeDocument {
            themes: std::collections::HashMap::from([(
                "light".to_string(),
                create_test_theme("light"),
            )]),
            default_theme: None,
            follow_system: true,
        };

        let ctx = ThemeContext::from_document(doc, None).unwrap();
        assert_eq!(ctx.active_name(), "light");
    }

    #[test]
    fn contract_theme_context_no_system_pref_uses_default() {
        let doc = ThemeDocument {
            themes: std::collections::HashMap::from([
                ("light".to_string(), create_test_theme("light")),
                ("dark".to_string(), create_test_theme("dark")),
            ]),
            default_theme: Some("dark".to_string()),
            follow_system: true,
        };

        let ctx = ThemeContext::from_document(doc, None).unwrap();
        assert_eq!(ctx.active_name(), "dark");
    }

    #[test]
    fn contract_theme_context_has_all_themes() {
        let doc = create_light_dark_document();
        let ctx = ThemeContext::from_document(doc, None).unwrap();

        let themes = ctx.available_themes();
        assert_eq!(themes.len(), 2);
        assert!(themes.contains(&"light"));
        assert!(themes.contains(&"dark"));
    }
}

#[cfg(test)]
mod contract_runtime_theme_switching {
    use super::*;
    use dampen_core::ir::style::Color;
    use dampen_core::ir::theme::{SpacingScale, Theme, ThemeDocument, ThemePalette, Typography};

    fn create_test_theme(name: &str) -> Theme {
        Theme {
            name: name.to_string(),
            palette: ThemePalette {
                primary: Some(Color::from_hex("#3498db").unwrap()),
                secondary: Some(Color::from_hex("#2ecc71").unwrap()),
                success: Some(Color::from_hex("#27ae60").unwrap()),
                warning: Some(Color::from_hex("#f39c12").unwrap()),
                danger: Some(Color::from_hex("#e74c3c").unwrap()),
                background: Some(Color::from_hex("#ecf0f1").unwrap()),
                surface: Some(Color::from_hex("#ffffff").unwrap()),
                text: Some(Color::from_hex("#2c3e50").unwrap()),
                text_secondary: Some(Color::from_hex("#7f8c8d").unwrap()),
            },
            typography: dampen_core::ir::theme::Typography {
                font_family: Some("sans-serif".to_string()),
                font_size_base: Some(16.0),
                font_size_small: Some(12.0),
                font_size_large: Some(24.0),
                font_weight: dampen_core::ir::theme::FontWeight::Normal,
                line_height: Some(1.5),
            },
            spacing: dampen_core::ir::theme::SpacingScale { unit: Some(8.0) },
            base_styles: std::collections::HashMap::new(),
            extends: None,
        }
    }

    fn create_light_dark_document() -> ThemeDocument {
        ThemeDocument {
            themes: std::collections::HashMap::from([
                ("light".to_string(), create_test_theme("light")),
                ("dark".to_string(), create_test_theme("dark")),
            ]),
            default_theme: Some("light".to_string()),
            follow_system: true,
        }
    }

    #[test]
    fn contract_set_theme_changes_active_theme() {
        let doc = create_light_dark_document();
        let mut ctx = ThemeContext::from_document(doc, None).unwrap();

        assert_eq!(ctx.active_name(), "light");

        let result = ctx.set_theme("dark");
        assert!(
            result.is_ok(),
            "set_theme should succeed for existing theme"
        );
        assert_eq!(ctx.active_name(), "dark");
    }

    #[test]
    fn contract_set_theme_returns_error_for_nonexistent() {
        let doc = create_light_dark_document();
        let mut ctx = ThemeContext::from_document(doc, None).unwrap();

        let result = ctx.set_theme("nonexistent");
        assert!(
            result.is_err(),
            "set_theme should fail for nonexistent theme"
        );

        let err = result.unwrap_err();
        assert!(
            err.message.contains("not found") || err.message.contains("THEME_006"),
            "Error should indicate theme not found: {}",
            err.message
        );
    }

    #[test]
    fn contract_set_theme_does_not_change_on_error() {
        let doc = create_light_dark_document();
        let mut ctx = ThemeContext::from_document(doc, None).unwrap();

        assert_eq!(ctx.active_name(), "light");

        let _ = ctx.set_theme("nonexistent");
        assert_eq!(
            ctx.active_name(),
            "light",
            "Active theme should not change on error"
        );
    }

    #[test]
    fn contract_set_theme_multiple_switches() {
        let doc = create_light_dark_document();
        let mut ctx = ThemeContext::from_document(doc, None).unwrap();

        assert_eq!(ctx.active_name(), "light");

        ctx.set_theme("dark").unwrap();
        assert_eq!(ctx.active_name(), "dark");

        ctx.set_theme("light").unwrap();
        assert_eq!(ctx.active_name(), "light");

        ctx.set_theme("dark").unwrap();
        assert_eq!(ctx.active_name(), "dark");
    }

    #[test]
    fn contract_active_returns_correct_theme() {
        let doc = create_light_dark_document();
        let mut ctx = ThemeContext::from_document(doc, None).unwrap();

        assert_eq!(ctx.active().name, "light");

        ctx.set_theme("dark").unwrap();
        assert_eq!(ctx.active().name, "dark");
    }

    #[test]
    fn contract_has_theme_checks_existence() {
        let doc = create_light_dark_document();
        let ctx = ThemeContext::from_document(doc, None).unwrap();

        assert!(ctx.has_theme("light"));
        assert!(ctx.has_theme("dark"));
        assert!(!ctx.has_theme("nonexistent"));
    }
}

#[cfg(test)]
mod contract_hot_reload_theme_update {
    use super::*;
    use dampen_core::ir::style::Color;
    use dampen_core::ir::theme::{SpacingScale, Theme, ThemeDocument, ThemePalette, Typography};

    fn create_test_theme(name: &str, primary_color: &str) -> Theme {
        Theme {
            name: name.to_string(),
            palette: ThemePalette {
                primary: Some(Color::from_hex(primary_color).unwrap()),
                secondary: Some(Color::from_hex("#2ecc71").unwrap()),
                success: Some(Color::from_hex("#27ae60").unwrap()),
                warning: Some(Color::from_hex("#f39c12").unwrap()),
                danger: Some(Color::from_hex("#e74c3c").unwrap()),
                background: Some(Color::from_hex("#ecf0f1").unwrap()),
                surface: Some(Color::from_hex("#ffffff").unwrap()),
                text: Some(Color::from_hex("#2c3e50").unwrap()),
                text_secondary: Some(Color::from_hex("#7f8c8d").unwrap()),
            },
            typography: dampen_core::ir::theme::Typography {
                font_family: Some("sans-serif".to_string()),
                font_size_base: Some(16.0),
                font_size_small: Some(12.0),
                font_size_large: Some(24.0),
                font_weight: dampen_core::ir::theme::FontWeight::Normal,
                line_height: Some(1.5),
            },
            spacing: SpacingScale { unit: Some(8.0) },
            base_styles: std::collections::HashMap::new(),
            extends: None,
        }
    }

    fn create_light_dark_document() -> ThemeDocument {
        ThemeDocument {
            themes: std::collections::HashMap::from([
                ("light".to_string(), create_test_theme("light", "#3498db")),
                ("dark".to_string(), create_test_theme("dark", "#5dade2")),
            ]),
            default_theme: Some("light".to_string()),
            follow_system: true,
        }
    }

    #[test]
    fn contract_hot_reload_updates_theme_colors() {
        let doc = create_light_dark_document();
        let mut ctx = ThemeContext::from_document(doc, None).unwrap();

        assert_eq!(ctx.active_name(), "light");
        assert_eq!(
            ctx.active().palette.primary,
            Some(Color::from_hex("#3498db").unwrap())
        );

        let new_doc = ThemeDocument {
            themes: std::collections::HashMap::from([
                ("light".to_string(), create_test_theme("light", "#ff0000")),
                ("dark".to_string(), create_test_theme("dark", "#0000ff")),
            ]),
            default_theme: Some("light".to_string()),
            follow_system: true,
        };

        ctx.reload(new_doc);

        assert_eq!(ctx.active_name(), "light");
        assert_eq!(
            ctx.active().palette.primary,
            Some(Color::from_hex("#ff0000").unwrap())
        );
    }

    #[test]
    fn contract_hot_reload_falls_back_on_removed_theme() {
        let doc = create_light_dark_document();
        let mut ctx = ThemeContext::from_document(doc, None).unwrap();

        ctx.set_theme("dark").unwrap();
        assert_eq!(ctx.active_name(), "dark");

        let new_doc = ThemeDocument {
            themes: std::collections::HashMap::from([(
                "light".to_string(),
                create_test_theme("light", "#3498db"),
            )]),
            default_theme: Some("light".to_string()),
            follow_system: true,
        };

        ctx.reload(new_doc);

        assert_eq!(ctx.active_name(), "light");
    }

    #[test]
    fn contract_hot_reload_all_available_themes_updated() {
        let doc = create_light_dark_document();
        let mut ctx = ThemeContext::from_document(doc.clone(), None).unwrap();

        let new_doc = ThemeDocument {
            themes: std::collections::HashMap::from([
                ("light".to_string(), create_test_theme("light", "#111111")),
                ("dark".to_string(), create_test_theme("dark", "#222222")),
                ("custom".to_string(), create_test_theme("custom", "#333333")),
            ]),
            default_theme: Some("light".to_string()),
            follow_system: true,
        };

        ctx.reload(new_doc);

        let themes = ctx.available_themes();
        assert_eq!(themes.len(), 3);
        assert!(themes.contains(&"light"));
        assert!(themes.contains(&"dark"));
        assert!(themes.contains(&"custom"));
    }
}

#[cfg(test)]
mod contract_theme_codegen {
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
            base_styles: std::collections::HashMap::new(),
            extends: None,
        }
    }

    fn create_light_dark_document() -> ThemeDocument {
        ThemeDocument {
            themes: std::collections::HashMap::from([
                ("light".to_string(), create_test_theme("light", "#3498db")),
                ("dark".to_string(), create_test_theme("dark", "#5dade2")),
            ]),
            default_theme: Some("light".to_string()),
            follow_system: true,
        }
    }

    #[test]
    fn contract_codegen_generates_valid_rust_code() {
        let doc = create_light_dark_document();
        let result = generate_theme_code(&doc, &std::collections::HashMap::new(), "app");

        assert!(
            result.is_ok(),
            "Should generate valid Rust code: {:?}",
            result.err()
        );
        let code = result.unwrap();

        assert!(
            code.code.contains("pub fn app_theme"),
            "Generated code should contain app_theme function"
        );
        assert!(
            code.code.contains("pub fn app_themes"),
            "Generated code should contain app_themes function"
        );
        // Accept either "Theme" or "iced::Theme" since the generated code imports Theme
        assert!(
            code.code.contains("-> Theme") || code.code.contains("iced::Theme"),
            "Generated code should use Theme type"
        );
    }

    #[test]
    fn contract_codegen_includes_theme_colors() {
        let doc = create_light_dark_document();
        let result = generate_theme_code(&doc, &std::collections::HashMap::new(), "app");

        assert!(result.is_ok());
        let code = result.unwrap();

        assert!(
            code.code.contains("#3498db")
                || code.code.contains("0x34")
                || code.code.contains("0.204"),
            "Generated code should contain light theme primary color"
        );
        assert!(
            code.code.contains("#5dade2")
                || code.code.contains("0x5D")
                || code.code.contains("0.365"),
            "Generated code should contain dark theme primary color"
        );
    }

    #[test]
    fn contract_codegen_single_theme() {
        let doc = ThemeDocument {
            themes: std::collections::HashMap::from([(
                "custom".to_string(),
                create_test_theme("custom", "#ff0000"),
            )]),
            default_theme: Some("custom".to_string()),
            follow_system: false,
        };

        let result = generate_theme_code(&doc, &std::collections::HashMap::new(), "my_app");

        assert!(result.is_ok());
        let code = result.unwrap();

        assert!(
            code.code.contains("my_app_theme"),
            "Generated code should use custom module name"
        );
        assert!(
            code.code.contains("custom"),
            "Generated code should contain theme name"
        );
    }

    #[test]
    fn contract_codegen_empty_themes_returns_error() {
        let doc = ThemeDocument {
            themes: std::collections::HashMap::new(),
            default_theme: None,
            follow_system: true,
        };

        let result = generate_theme_code(&doc, &std::collections::HashMap::new(), "app");

        assert!(result.is_err(), "Should fail with no themes");
        let err = result.unwrap_err();
        assert!(
            err.contains("no themes") || err.contains("THEME_001"),
            "Error should mention no themes: {}",
            err
        );
    }

    #[test]
    fn contract_codegen_default_theme_in_generated_code() {
        let doc = create_light_dark_document();
        let result = generate_theme_code(&doc, &std::collections::HashMap::new(), "app");

        assert!(result.is_ok());
        let code = result.unwrap();

        assert!(
            code.code.contains("pub fn app_default_theme"),
            "Generated code should contain default theme function"
        );
        assert!(
            code.code.contains("\"light\""),
            "Generated code should contain default theme name"
        );
    }
}
