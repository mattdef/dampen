//! Application trait generation

use proc_macro2::TokenStream;
use quote::quote;

use super::CodegenError;
use super::theme::generate_theme_code;
use crate::ir::theme::ThemeDocument;

/// Generate Application trait implementation
pub fn generate_application_trait(
    model_name: &str,
    message_name: &str,
) -> Result<TokenStream, super::CodegenError> {
    let model_ident = syn::Ident::new(model_name, proc_macro2::Span::call_site());
    let message_ident = syn::Ident::new(message_name, proc_macro2::Span::call_site());

    Ok(quote! {
        impl iced::Application for #model_ident {
            type Executor = iced::executor::Default;
            type Flags = ();
            type Message = #message_ident;

            fn new(_flags: ()) -> (Self, iced::Task<Self::Message>) {
                (Self::default(), iced::Task::none())
            }

            fn title(&self) -> String {
                "Dampen Application".to_string()
            }
        }
    })
}

/// Generate theme code and add it to the application
pub fn generate_application_with_theme(
    model_name: &str,
    message_name: &str,
    theme_document: Option<&ThemeDocument>,
) -> Result<TokenStream, CodegenError> {
    let model_ident = syn::Ident::new(model_name, proc_macro2::Span::call_site());
    let message_ident = syn::Ident::new(message_name, proc_macro2::Span::call_site());

    let theme_code = if let Some(doc) = theme_document {
        match generate_theme_code(doc, "app") {
            Ok(generated) => generated.code,
            Err(e) => {
                return Err(CodegenError::ThemeError(e));
            }
        }
    } else {
        String::new()
    };

    Ok(quote! {
        #theme_code

        impl iced::Application for #model_ident {
            type Executor = iced::executor::Default;
            type Flags = ();
            type Message = #message_ident;

            fn new(_flags: ()) -> (Self, iced::Task<Self::Message>) {
                (Self::default(), iced::Task::none())
            }

            fn title(&self) -> String {
                "Dampen Application".to_string()
            }

            fn theme(&self) -> iced::Theme {
                app_theme()
            }
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::style::Color;
    use crate::ir::theme::{SpacingScale, Theme, ThemeDocument, ThemePalette, Typography};

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
                font_weight: crate::ir::theme::FontWeight::Normal,
                line_height: Some(1.5),
            },
            spacing: SpacingScale { unit: Some(8.0) },
            base_styles: std::collections::HashMap::new(),
            extends: None,
        }
    }

    #[test]
    fn test_application_trait_generation() {
        let result = generate_application_trait("MyModel", "MyMessage").unwrap();
        let code = result.to_string();

        assert!(code.contains("impl") && code.contains("Application") && code.contains("MyModel"));
        assert!(code.contains("Message") && code.contains("MyMessage"));
    }

    #[test]
    fn test_application_with_theme_generation() {
        let doc = ThemeDocument {
            themes: std::collections::HashMap::from([(
                "light".to_string(),
                create_test_theme("light"),
            )]),
            default_theme: Some("light".to_string()),
            follow_system: true,
        };

        let result = generate_application_with_theme("MyModel", "MyMessage", Some(&doc));
        assert!(result.is_ok());
        let code = result.unwrap().to_string();

        assert!(code.contains("impl") && code.contains("Application"));
        assert!(code.contains("app_theme()"));
        assert!(code.contains("fn app_light()"));
    }

    #[test]
    fn test_application_without_theme() {
        let result = generate_application_with_theme("MyModel", "MyMessage", None);
        assert!(result.is_ok());
        let code = result.unwrap().to_string();

        assert!(code.contains("impl") && code.contains("Application"));
    }
}
