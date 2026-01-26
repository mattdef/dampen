//! Theme code generation for production builds
//!
//! This module generates static Rust code from theme definitions.
//! The generated code allows themes to be compiled directly into the binary
//! with zero runtime parsing overhead.

use super::GeneratedCode;
use crate::ir::style::{
    Background, Border, BorderRadius, Color, Gradient, Shadow, StyleProperties,
};
use crate::ir::theme::{StyleClass, ThemeDocument, WidgetState};
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::HashMap;

// ============================================================================
// Helper Functions for Style Code Generation
// ============================================================================

/// Generate TokenStream for a Color IR type to iced::Color
fn generate_color_expr(color: &Color) -> TokenStream {
    let r = color.r;
    let g = color.g;
    let b = color.b;
    let a = color.a;
    quote! {
        iced::Color::from_rgba(#r, #g, #b, #a)
    }
}

/// Generate TokenStream for a Background IR type to iced::Background
fn generate_background_expr(bg: &Background) -> TokenStream {
    match bg {
        Background::Color(color) => {
            let color_expr = generate_color_expr(color);
            quote! { iced::Background::Color(#color_expr) }
        }
        Background::Gradient(gradient) => generate_gradient_expr(gradient),
        Background::Image { .. } => {
            // Iced doesn't support image backgrounds directly
            quote! { iced::Background::Color(iced::Color::TRANSPARENT) }
        }
    }
}

/// Generate TokenStream for a Gradient IR type to iced::Gradient
fn generate_gradient_expr(gradient: &Gradient) -> TokenStream {
    match gradient {
        Gradient::Linear { angle, stops } => {
            let radians = angle * (std::f32::consts::PI / 180.0);

            let stop_exprs: Vec<TokenStream> = stops
                .iter()
                .take(8)
                .map(|stop| {
                    let offset = stop.offset;
                    let color_expr = generate_color_expr(&stop.color);
                    quote! { .add_stop(#offset, #color_expr) }
                })
                .collect();

            quote! {
                iced::Background::Gradient(
                    iced::Gradient::Linear(
                        iced::gradient::Linear::new(iced::Radians(#radians))
                            #(#stop_exprs)*
                    )
                )
            }
        }
        Gradient::Radial { stops, .. } => {
            // Iced 0.14 only supports linear gradients, convert to linear as fallback
            let radians = 0.0f32;
            let stop_exprs: Vec<TokenStream> = stops
                .iter()
                .take(8)
                .map(|stop| {
                    let offset = stop.offset;
                    let color_expr = generate_color_expr(&stop.color);
                    quote! { .add_stop(#offset, #color_expr) }
                })
                .collect();

            quote! {
                iced::Background::Gradient(
                    iced::Gradient::Linear(
                        iced::gradient::Linear::new(iced::Radians(#radians))
                            #(#stop_exprs)*
                    )
                )
            }
        }
    }
}

/// Generate TokenStream for a Border IR type to iced::Border
fn generate_border_expr(border: &Border) -> TokenStream {
    let width = border.width;
    let color_expr = generate_color_expr(&border.color);
    let radius_expr = generate_border_radius_expr(&border.radius);

    quote! {
        iced::Border {
            width: #width,
            color: #color_expr,
            radius: #radius_expr,
        }
    }
}

/// Generate TokenStream for a BorderRadius IR type to iced::border::Radius
fn generate_border_radius_expr(radius: &BorderRadius) -> TokenStream {
    let tl = radius.top_left;
    let tr = radius.top_right;
    let br = radius.bottom_right;
    let bl = radius.bottom_left;

    quote! {
        iced::border::Radius::from(#tl)
            .top_right(#tr)
            .bottom_right(#br)
            .bottom_left(#bl)
    }
}

/// Generate TokenStream for a Shadow IR type to iced::Shadow
fn generate_shadow_expr(shadow: &Shadow) -> TokenStream {
    let offset_x = shadow.offset_x;
    let offset_y = shadow.offset_y;
    let blur = shadow.blur_radius;
    let color_expr = generate_color_expr(&shadow.color);

    quote! {
        iced::Shadow {
            color: #color_expr,
            offset: iced::Vector::new(#offset_x, #offset_y),
            blur_radius: #blur,
        }
    }
}

/// Generate a button::Style struct from StyleProperties
fn generate_button_style_struct(style: &StyleProperties) -> TokenStream {
    let background_expr = if let Some(ref bg) = style.background {
        let bg_expr = generate_background_expr(bg);
        quote! { Some(#bg_expr) }
    } else {
        quote! { None }
    };

    let text_color_expr = if let Some(ref color) = style.color {
        generate_color_expr(color)
    } else {
        quote! { _theme.extended_palette().background.base.text }
    };

    let border_expr = if let Some(ref border) = style.border {
        generate_border_expr(border)
    } else {
        quote! { iced::Border::default() }
    };

    let shadow_expr = if let Some(ref shadow) = style.shadow {
        generate_shadow_expr(shadow)
    } else {
        quote! { iced::Shadow::default() }
    };

    quote! {
        iced::widget::button::Style {
            background: #background_expr,
            text_color: #text_color_expr,
            border: #border_expr,
            shadow: #shadow_expr,
            snap: false,
        }
    }
}

/// Generate a container::Style struct from StyleProperties
fn generate_container_style_struct(style: &StyleProperties) -> TokenStream {
    let background_expr = if let Some(ref bg) = style.background {
        let bg_expr = generate_background_expr(bg);
        quote! { Some(#bg_expr) }
    } else {
        quote! { None }
    };

    let text_color_expr = if let Some(ref color) = style.color {
        let color_expr = generate_color_expr(color);
        quote! { Some(#color_expr) }
    } else {
        quote! { None }
    };

    let border_expr = if let Some(ref border) = style.border {
        generate_border_expr(border)
    } else {
        quote! { iced::Border::default() }
    };

    let shadow_expr = if let Some(ref shadow) = style.shadow {
        generate_shadow_expr(shadow)
    } else {
        quote! { iced::Shadow::default() }
    };

    quote! {
        iced::widget::container::Style {
            background: #background_expr,
            text_color: #text_color_expr,
            border: #border_expr,
            shadow: #shadow_expr,
            snap: false,
        }
    }
}

/// Merge base style with state override
fn merge_style_properties(
    base: &StyleProperties,
    override_props: &StyleProperties,
) -> StyleProperties {
    StyleProperties {
        background: override_props
            .background
            .clone()
            .or_else(|| base.background.clone()),
        color: override_props.color.or(base.color),
        border: override_props
            .border
            .clone()
            .or_else(|| base.border.clone()),
        shadow: override_props.shadow.or(base.shadow),
        opacity: override_props.opacity.or(base.opacity),
        transform: override_props
            .transform
            .clone()
            .or_else(|| base.transform.clone()),
    }
}

/// Determine which Iced widget type this style class targets
fn infer_widget_type_from_class(style_class: &StyleClass) -> &'static str {
    // For now, default to button if we have state variants (interactive)
    // Otherwise default to container (static styling)
    if !style_class.state_variants.is_empty() {
        "button"
    } else {
        "container"
    }
}

/// Generate a match block for state-aware styling
fn generate_state_match_for_button(style_class: &StyleClass) -> TokenStream {
    let mut match_arms = Vec::new();

    // Base state (Active)
    let base_style_expr = generate_button_style_struct(&style_class.style);
    match_arms.push(quote! {
        iced::widget::button::Status::Active => #base_style_expr
    });

    // State overrides
    for (state, override_style) in &style_class.state_variants {
        let merged_style = merge_style_properties(&style_class.style, override_style);
        let style_expr = generate_button_style_struct(&merged_style);

        let status_variant = match state {
            WidgetState::Hover => quote! { iced::widget::button::Status::Hovered },
            WidgetState::Active => quote! { iced::widget::button::Status::Pressed },
            WidgetState::Disabled => quote! { iced::widget::button::Status::Disabled },
            WidgetState::Focus => {
                // Button doesn't have Focus in Iced, skip or map to Active
                continue;
            }
        };

        match_arms.push(quote! {
            #status_variant => #style_expr
        });
    }

    // Add wildcard arm to catch any other states (use base style)
    let fallback_style = generate_button_style_struct(&style_class.style);
    match_arms.push(quote! {
        _ => #fallback_style
    });

    quote! {
        match status {
            #(#match_arms),*
        }
    }
}

/// Generate a style class function (String output for easier concatenation)
fn generate_style_class_function(
    class_name: &str,
    style_class: &StyleClass,
) -> Result<String, String> {
    let fn_name = format!("style_{}", class_name.replace("-", "_").replace(":", "_"));
    let widget_type = infer_widget_type_from_class(style_class);

    let mut code = String::new();
    code.push_str(&format!("/// Style function for class '{}'\n", class_name));

    if widget_type == "button" && !style_class.state_variants.is_empty() {
        // State-aware button style
        code.push_str(&format!(
            "pub fn {}(_theme: &iced::Theme, status: iced::widget::button::Status) -> iced::widget::button::Style {{\n",
            fn_name
        ));

        let match_expr = generate_state_match_for_button(style_class);
        let match_str = match_expr.to_string();
        code.push_str("    ");
        code.push_str(&match_str);
        code.push('\n');
    } else {
        // Static container style
        code.push_str(&format!(
            "pub fn {}(_theme: &iced::Theme) -> iced::widget::container::Style {{\n",
            fn_name
        ));

        let style_expr = generate_container_style_struct(&style_class.style);
        let style_str = style_expr.to_string();
        code.push_str("    ");
        code.push_str(&style_str);
        code.push('\n');
    }

    code.push_str("}\n\n");

    Ok(code)
}

// ============================================================================
// Main Theme Code Generation
// ============================================================================

/// Generate Rust code for a theme document
///
/// This function generates a Rust module containing functions to access
/// themes at runtime without any parsing overhead. It also generates style
/// class functions for widget styling.
///
/// # Arguments
///
/// * `document` - The parsed theme document
/// * `style_classes` - Style class definitions from the Dampen document
/// * `module_name` - Name for the generated module (e.g., "app" → app_theme module)
///
/// # Returns
///
/// Ok(GeneratedCode) with the generated Rust code, or an error if validation fails
///
/// # Example Output
///
/// ```rust,ignore
/// // Generated theme code
/// pub fn app_theme() -> iced::Theme {
///     app_default_theme()
/// }
///
/// pub fn app_themes() -> HashMap<&'static str, iced::Theme> {
///     let mut themes = HashMap::new();
///     themes.insert("light", app_theme_light());
///     themes.insert("dark", app_theme_dark());
///     themes
/// }
///
/// fn app_theme_light() -> iced::Theme {
///     iced::Theme::custom(
///         "light".to_string(),
///         iced::theme::Palette {
///             background: iced::Color::from_rgb8(0xEC, 0xF0, 0xF1),
///             text: iced::Color::from_rgb8(0x2C, 0x3E, 0x50),
///             primary: iced::Color::from_rgb8(0x34, 0x98, 0xDB),
///             success: iced::Color::from_rgb8(0x27, 0xAE, 0x60),
///             warning: iced::Color::from_rgb8(0xF3, 0x9C, 0x12),
///             danger: iced::Color::from_rgb8(0xE7, 0x4C, 0x3C),
///         }
///     )
/// }
///
/// // Style class functions
/// pub fn style_primary_button(_theme: &iced::Theme, status: iced::widget::button::Status) -> iced::widget::button::Style {
///     match status {
///         iced::widget::button::Status::Active => { /* ... */ }
///         iced::widget::button::Status::Hovered => { /* ... */ }
///         _ => iced::widget::button::Style::default()
///     }
/// }
/// ```
pub fn generate_theme_code(
    document: &ThemeDocument,
    style_classes: &HashMap<String, StyleClass>,
    module_name: &str,
) -> Result<GeneratedCode, String> {
    if document.themes.is_empty() {
        return Err("THEME_001: At least one theme must be defined".to_string());
    }

    let mut code = String::new();

    code.push_str("// Generated theme code - DO NOT EDIT\n");
    code.push_str("// This file is auto-generated by the dampen codegen.\n\n");

    // Add thread-local storage for current theme name
    code.push_str("use std::cell::RefCell;\n\n");
    code.push_str("thread_local! {\n");
    code.push_str(
        "    static CURRENT_THEME: RefCell<Option<String>> = const { RefCell::new(None) };\n",
    );
    code.push_str("}\n\n");

    code.push_str("/// Set the current theme by name\n");
    code.push_str(&format!(
        "pub fn {}_set_current_theme(name: &str) {{\n",
        module_name
    ));
    code.push_str("    CURRENT_THEME.with(|t| {\n");
    code.push_str("        *t.borrow_mut() = Some(name.to_string());\n");
    code.push_str("    });\n");
    code.push_str("}\n\n");

    code.push_str("/// Get the current theme name\n");
    code.push_str(&format!(
        "pub fn {}_current_theme_name() -> String {{\n",
        module_name
    ));
    code.push_str("    CURRENT_THEME.with(|t| {\n");
    code.push_str("        t.borrow().clone().unwrap_or_else(|| {\n");
    let effective_default = document.effective_default(None);
    code.push_str(&format!(
        "            \"{}\".to_string()\n",
        effective_default
    ));
    code.push_str("        })\n");
    code.push_str("    })\n");
    code.push_str("}\n\n");

    code.push_str(
        "/// Get the current theme (respects system preference when follow_system is enabled)\n",
    );
    code.push_str(&format!("pub fn {}_theme() -> Theme {{\n", module_name));
    code.push_str(&format!(
        "    let name = {}_current_theme_name();\n",
        module_name
    ));
    code.push_str(&format!(
        "    {}_theme_named(&name).unwrap_or_else(|| {}_default_theme())\n",
        module_name, module_name
    ));
    code.push_str("}\n\n");

    code.push_str("/// Get a specific theme by name\n");
    code.push_str(&format!(
        "pub fn {}_theme_named(name: &str) -> Option<Theme> {{\n",
        module_name
    ));
    code.push_str(&format!("    let themes = {}_themes();\n", module_name));
    code.push_str("    themes.get(name).cloned()\n");
    code.push_str("}\n\n");

    code.push_str("/// Get all available themes\n");
    code.push_str(&format!(
        "pub fn {}_themes() -> HashMap<&'static str, Theme> {{\n",
        module_name
    ));
    code.push_str("    let mut themes = HashMap::new();\n");

    let mut theme_names: Vec<&str> = document.themes.keys().map(|s| s.as_str()).collect();
    theme_names.sort();

    for theme_name in &theme_names {
        code.push_str(&format!(
            "    themes.insert(\"{}\", {}_{}());\n",
            theme_name, module_name, theme_name
        ));
    }

    code.push_str("    themes\n");
    code.push_str("}\n\n");

    code.push_str("/// Get the default theme\n");
    code.push_str(&format!(
        "pub fn {}_default_theme() -> Theme {{\n",
        module_name
    ));
    code.push_str(&format!("    {}_{}()\n", module_name, effective_default));
    code.push_str("}\n\n");

    code.push_str("/// Get the default theme name as a string\n");
    code.push_str(&format!(
        "pub fn {}_default_theme_name() -> &'static str {{\n",
        module_name
    ));
    code.push_str(&format!("    \"{}\"\n", effective_default));
    code.push_str("}\n\n");

    code.push_str("/// Get whether the theme follows system preference\n");
    code.push_str(&format!(
        "pub fn {}_follows_system() -> bool {{\n",
        module_name
    ));
    code.push_str(&format!("    {}\n", document.follow_system));
    code.push_str("}\n\n");

    for theme_name in &theme_names {
        let theme = match document.themes.get(*theme_name) {
            Some(t) => t,
            None => continue,
        };

        let theme_fn_name = format!("{}_{}", module_name, theme_name);
        code.push_str(&format!("/// Theme: {}\n", theme_name));
        code.push_str("fn ");
        code.push_str(&theme_fn_name);
        code.push_str("() -> Theme {\n");

        let palette = &theme.palette;
        let primary = color_to_rgb8_tuple(palette.primary.as_ref());
        let background = color_to_rgb8_tuple(palette.background.as_ref());
        let text = color_to_rgb8_tuple(palette.text.as_ref());
        let success = color_to_rgb8_tuple(palette.success.as_ref());
        let warning = color_to_rgb8_tuple(palette.warning.as_ref());
        let danger = color_to_rgb8_tuple(palette.danger.as_ref());

        code.push_str("    Theme::custom(\n");
        code.push_str(&format!("        \"{}\".to_string(),\n", theme_name));
        code.push_str("        iced::theme::Palette {\n");
        code.push_str(&format!(
            "            background: iced::Color::from_rgb8(0x{:02X}, 0x{:02X}, 0x{:02X}),\n",
            (background.0 * 255.0) as u8,
            (background.1 * 255.0) as u8,
            (background.2 * 255.0) as u8
        ));
        code.push_str(&format!(
            "            text: iced::Color::from_rgb8(0x{:02X}, 0x{:02X}, 0x{:02X}),\n",
            (text.0 * 255.0) as u8,
            (text.1 * 255.0) as u8,
            (text.2 * 255.0) as u8
        ));
        code.push_str(&format!(
            "            primary: iced::Color::from_rgb8(0x{:02X}, 0x{:02X}, 0x{:02X}),\n",
            (primary.0 * 255.0) as u8,
            (primary.1 * 255.0) as u8,
            (primary.2 * 255.0) as u8
        ));
        code.push_str(&format!(
            "            success: iced::Color::from_rgb8(0x{:02X}, 0x{:02X}, 0x{:02X}),\n",
            (success.0 * 255.0) as u8,
            (success.1 * 255.0) as u8,
            (success.2 * 255.0) as u8
        ));
        code.push_str(&format!(
            "            warning: iced::Color::from_rgb8(0x{:02X}, 0x{:02X}, 0x{:02X}),\n",
            (warning.0 * 255.0) as u8,
            (warning.1 * 255.0) as u8,
            (warning.2 * 255.0) as u8
        ));
        code.push_str(&format!(
            "            danger: iced::Color::from_rgb8(0x{:02X}, 0x{:02X}, 0x{:02X}),\n",
            (danger.0 * 255.0) as u8,
            (danger.1 * 255.0) as u8,
            (danger.2 * 255.0) as u8
        ));
        code.push_str("        }\n");
        code.push_str("    )\n");
        code.push_str("}\n\n");
    }

    // ========================================
    // Style Class Functions
    // ========================================
    if !style_classes.is_empty() {
        code.push_str("// ========================================\n");
        code.push_str("// Style Class Functions\n");
        code.push_str("// ========================================\n\n");

        let mut class_names: Vec<&str> = style_classes.keys().map(|s| s.as_str()).collect();
        class_names.sort();

        for class_name in class_names {
            if let Some(style_class) = style_classes.get(class_name) {
                let class_fn_code = generate_style_class_function(class_name, style_class)?;
                code.push_str(&class_fn_code);
            }
        }
    }

    let source_file = format!("{}/theme.dampen", module_name);
    Ok(GeneratedCode::new(
        code,
        format!("{}_theme", module_name),
        std::path::PathBuf::from(source_file),
    ))
}

/// Convert a color to RGB tuple (0.0-1.0 range)
fn color_to_rgb8_tuple(color: Option<&Color>) -> (f32, f32, f32) {
    match color {
        Some(c) => (c.r, c.g, c.b),
        None => (0.0, 0.0, 0.0),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::style::Color;
    use crate::ir::theme::{SpacingScale, Theme, ThemePalette, Typography};

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
                font_weight: crate::ir::theme::FontWeight::Normal,
                line_height: Some(1.5),
            },
            spacing: SpacingScale { unit: Some(8.0) },
            base_styles: std::collections::HashMap::new(),
            extends: None,
        }
    }

    #[test]
    fn test_generate_theme_code_basic() {
        let doc = ThemeDocument {
            themes: std::collections::HashMap::from([(
                "light".to_string(),
                create_test_theme("light", "#3498db"),
            )]),
            default_theme: Some("light".to_string()),
            follow_system: false,
        };

        let style_classes = HashMap::new();
        let result = generate_theme_code(&doc, &style_classes, "test");

        assert!(result.is_ok());
        let code = result.unwrap().code;

        assert!(code.contains("pub fn test_theme()"));
        assert!(code.contains("pub fn test_themes()"));
        assert!(code.contains("pub fn test_default_theme()"));
        assert!(code.contains("fn test_light()"));
        assert!(code.contains("Theme::custom"));
        assert!(code.contains("Color::from_rgb8"));
    }

    #[test]
    fn test_generate_theme_code_multiple_themes() {
        let doc = ThemeDocument {
            themes: std::collections::HashMap::from([
                ("light".to_string(), create_test_theme("light", "#3498db")),
                ("dark".to_string(), create_test_theme("dark", "#5dade2")),
            ]),
            default_theme: Some("light".to_string()),
            follow_system: true,
        };

        let style_classes = HashMap::new();
        let result = generate_theme_code(&doc, &style_classes, "app");

        assert!(result.is_ok());
        let code = result.unwrap().code;

        assert!(code.contains("fn app_light()"));
        assert!(code.contains("fn app_dark()"));
        assert!(code.contains("themes.insert(\"light\""));
        assert!(code.contains("themes.insert(\"dark\""));
    }

    #[test]
    fn test_generate_theme_code_empty_themes_error() {
        let doc = ThemeDocument {
            themes: std::collections::HashMap::new(),
            default_theme: None,
            follow_system: true,
        };

        let style_classes = HashMap::new();
        let result = generate_theme_code(&doc, &style_classes, "app");

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.contains("THEME_001") || err.contains("no themes"));
    }

    #[test]
    fn test_generate_theme_code_valid_rust_syntax() {
        let doc = ThemeDocument {
            themes: std::collections::HashMap::from([(
                "test".to_string(),
                create_test_theme("test", "#ff0000"),
            )]),
            default_theme: Some("test".to_string()),
            follow_system: false,
        };

        let style_classes = HashMap::new();
        let result = generate_theme_code(&doc, &style_classes, "test");

        assert!(result.is_ok());
        let code = result.unwrap().code;

        let parsed = syn::parse_file(&code);
        assert!(
            parsed.is_ok(),
            "Generated code should be valid Rust syntax: {:?}",
            parsed.err()
        );
    }

    #[test]
    fn test_generate_theme_code_contains_color_values() {
        let doc = ThemeDocument {
            themes: std::collections::HashMap::from([(
                "custom".to_string(),
                create_test_theme("custom", "#AABBCC"),
            )]),
            default_theme: Some("custom".to_string()),
            follow_system: false,
        };

        let style_classes = HashMap::new();
        let result = generate_theme_code(&doc, &style_classes, "myapp");

        assert!(result.is_ok());
        let code = result.unwrap().code;

        assert!(
            code.contains("0xAA") || code.contains("0xBB") || code.contains("0xCC"),
            "Generated code should contain the color values"
        );
    }

    #[test]
    fn test_generate_style_class_simple() {
        let style_class = StyleClass {
            name: "primary-button".to_string(),
            style: StyleProperties {
                background: Some(Background::Color(Color::from_rgb8(52, 152, 219))),
                color: Some(Color::from_rgb8(255, 255, 255)),
                border: None,
                shadow: None,
                opacity: None,
                transform: None,
            },
            layout: None,
            extends: vec![],
            state_variants: HashMap::new(),
            combined_state_variants: HashMap::new(),
        };

        let mut style_classes = HashMap::new();
        style_classes.insert("primary-button".to_string(), style_class);

        let theme_doc = ThemeDocument {
            themes: HashMap::from([("light".to_string(), create_test_theme("light", "#3498db"))]),
            default_theme: Some("light".to_string()),
            follow_system: false,
        };

        let result = generate_theme_code(&theme_doc, &style_classes, "test");
        assert!(result.is_ok());

        let code = result.unwrap().code;
        assert!(code.contains("pub fn style_primary_button"));
        assert!(code.contains("Style Class Functions"));
    }

    #[test]
    fn test_generate_style_with_hover() {
        let mut state_variants = HashMap::new();
        state_variants.insert(
            WidgetState::Hover,
            StyleProperties {
                background: Some(Background::Color(Color::from_rgb8(74, 172, 239))),
                color: None,
                border: None,
                shadow: None,
                opacity: None,
                transform: None,
            },
        );

        let style_class = StyleClass {
            name: "hover-button".to_string(),
            style: StyleProperties {
                background: Some(Background::Color(Color::from_rgb8(52, 152, 219))),
                color: Some(Color::from_rgb8(255, 255, 255)),
                border: None,
                shadow: None,
                opacity: None,
                transform: None,
            },
            layout: None,
            extends: vec![],
            state_variants,
            combined_state_variants: HashMap::new(),
        };

        let mut style_classes = HashMap::new();
        style_classes.insert("hover-button".to_string(), style_class);

        let theme_doc = ThemeDocument {
            themes: HashMap::from([("light".to_string(), create_test_theme("light", "#3498db"))]),
            default_theme: Some("light".to_string()),
            follow_system: false,
        };

        let result = generate_theme_code(&theme_doc, &style_classes, "test");
        assert!(result.is_ok());

        let code = result.unwrap().code;
        assert!(code.contains("style_hover_button"));
        assert!(code.contains("Status :: Active"));
        assert!(code.contains("Status :: Hovered"));
    }
}
