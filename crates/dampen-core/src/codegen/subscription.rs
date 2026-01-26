//! Subscription code generation for production builds
//!
//! This module generates subscription code for system theme detection
//! and other event subscriptions that need to work in production builds.

use proc_macro2::TokenStream;
use quote::quote;

use crate::ir::theme::ThemeDocument;

/// Configuration for subscription code generation
#[derive(Debug, Clone)]
pub struct SubscriptionConfig {
    /// Whether to generate system theme subscription
    pub system_theme: bool,
    /// Name of the message enum
    pub message_name: String,
    /// Variant name for system theme changed message (e.g., "SystemThemeChanged")
    pub system_theme_variant: Option<String>,
}

impl Default for SubscriptionConfig {
    fn default() -> Self {
        Self {
            system_theme: false,
            message_name: "Message".to_string(),
            system_theme_variant: None,
        }
    }
}

impl SubscriptionConfig {
    /// Create subscription config from a theme document
    ///
    /// If the theme document has `follow_system: true`, system theme subscription
    /// will be enabled.
    pub fn from_theme_document(theme_doc: Option<&ThemeDocument>, message_name: &str) -> Self {
        let system_theme = theme_doc.map(|doc| doc.follow_system).unwrap_or(false);

        Self {
            system_theme,
            message_name: message_name.to_string(),
            system_theme_variant: if system_theme {
                Some("SystemThemeChanged".to_string())
            } else {
                None
            },
        }
    }

    /// Set the system theme variant name
    pub fn with_system_theme_variant(mut self, variant: impl Into<String>) -> Self {
        self.system_theme_variant = Some(variant.into());
        self.system_theme = true;
        self
    }
}

/// Generate the subscription function for the application
///
/// This generates a `subscription_model()` function that creates Iced subscriptions
/// for system theme changes when `follow_system` is enabled in the theme configuration.
///
/// # Arguments
///
/// * `config` - Subscription configuration
///
/// # Returns
///
/// TokenStream containing the subscription function
///
/// # Example Output
///
/// ```rust,ignore
/// pub fn subscription_model() -> iced::Subscription<Message> {
///     if app_follows_system() {
///         dampen_iced::watch_system_theme()
///             .map(Message::SystemThemeChanged)
///     } else {
///         iced::Subscription::none()
///     }
/// }
/// ```
pub fn generate_subscription_function(config: &SubscriptionConfig) -> TokenStream {
    let message_ident = syn::Ident::new(&config.message_name, proc_macro2::Span::call_site());

    if let Some(ref variant_name) = config.system_theme_variant {
        let variant_ident = syn::Ident::new(variant_name, proc_macro2::Span::call_site());

        quote! {
            /// Get the application subscription for system events
            ///
            /// This function returns a subscription that monitors system theme changes
            /// when `follow_system` is enabled in the theme configuration.
            pub fn subscription_model() -> iced::Subscription<#message_ident> {
                if app_follows_system() {
                    dampen_iced::watch_system_theme()
                        .map(#message_ident::#variant_ident)
                } else {
                    iced::Subscription::none()
                }
            }
        }
    } else {
        quote! {
            /// Get the application subscription for system events
            ///
            /// Returns no subscription when system theme following is disabled.
            pub fn subscription_model() -> iced::Subscription<#message_ident> {
                iced::Subscription::none()
            }
        }
    }
}

/// Generate the SystemThemeChanged variant for the Message enum
///
/// This should be called when generating the Message enum to add the
/// SystemThemeChanged variant if system theme following is enabled.
///
/// # Arguments
///
/// * `config` - Subscription configuration
///
/// # Returns
///
/// Option containing the TokenStream for the variant, or None if not needed
pub fn generate_system_theme_variant(config: &SubscriptionConfig) -> Option<TokenStream> {
    config.system_theme_variant.as_ref().map(|variant_name| {
        let variant_ident = syn::Ident::new(variant_name, proc_macro2::Span::call_site());
        quote! {
            /// System theme changed event
            #variant_ident(String)
        }
    })
}

/// Generate the update match arm for SystemThemeChanged
///
/// This generates the match arm that handles the SystemThemeChanged message
/// by updating the current theme based on the system preference.
///
/// # Arguments
///
/// * `config` - Subscription configuration
///
/// # Returns
///
/// Option containing the TokenStream for the match arm, or None if not needed
///
/// # Example Output
///
/// ```rust,ignore
/// Message::SystemThemeChanged(theme_name) => {
///     // Theme is automatically selected via app_theme_named()
///     // The application should store the current theme name if needed
///     iced::Task::none()
/// }
/// ```
pub fn generate_system_theme_update_arm(config: &SubscriptionConfig) -> Option<TokenStream> {
    let message_ident = syn::Ident::new(&config.message_name, proc_macro2::Span::call_site());

    config.system_theme_variant.as_ref().map(|variant_name| {
        let variant_ident = syn::Ident::new(variant_name, proc_macro2::Span::call_site());

        quote! {
            #message_ident::#variant_ident(theme_name) => {
                // Update current theme based on system preference
                app_set_current_theme(&theme_name);
                iced::Task::none()
            }
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::theme::{SpacingScale, Theme, ThemeDocument, ThemePalette, Typography};
    use std::collections::HashMap;

    fn create_test_theme_document(follow_system: bool) -> ThemeDocument {
        let mut themes = HashMap::new();
        themes.insert(
            "light".to_string(),
            Theme {
                name: "light".to_string(),
                palette: ThemePalette::light(),
                typography: Typography {
                    font_family: None,
                    font_size_base: Some(16.0),
                    font_size_small: Some(12.0),
                    font_size_large: Some(24.0),
                    font_weight: crate::ir::theme::FontWeight::Normal,
                    line_height: Some(1.5),
                },
                spacing: SpacingScale { unit: Some(8.0) },
                base_styles: HashMap::new(),
                extends: None,
            },
        );

        ThemeDocument {
            themes,
            default_theme: Some("light".to_string()),
            follow_system,
        }
    }

    #[test]
    fn test_subscription_config_from_theme_document() {
        let doc = create_test_theme_document(true);
        let config = SubscriptionConfig::from_theme_document(Some(&doc), "Message");

        assert!(config.system_theme);
        assert_eq!(
            config.system_theme_variant,
            Some("SystemThemeChanged".to_string())
        );
    }

    #[test]
    fn test_subscription_config_no_follow_system() {
        let doc = create_test_theme_document(false);
        let config = SubscriptionConfig::from_theme_document(Some(&doc), "Message");

        assert!(!config.system_theme);
        assert_eq!(config.system_theme_variant, None);
    }

    #[test]
    fn test_generate_subscription_function_with_system_theme() {
        let config = SubscriptionConfig {
            system_theme: true,
            message_name: "Message".to_string(),
            system_theme_variant: Some("SystemThemeChanged".to_string()),
        };

        let tokens = generate_subscription_function(&config);
        let code = tokens.to_string();

        assert!(code.contains("subscription_model"), "code: {}", code);
        assert!(code.contains("app_follows_system"), "code: {}", code);
        // quote! generates "dampen_iced :: watch_system_theme" with spaces
        assert!(code.contains("watch_system_theme"), "code: {}", code);
        assert!(code.contains("SystemThemeChanged"), "code: {}", code);
    }

    #[test]
    fn test_generate_subscription_function_without_system_theme() {
        let config = SubscriptionConfig::default();

        let tokens = generate_subscription_function(&config);
        let code = tokens.to_string();

        assert!(code.contains("subscription_model"), "code: {}", code);
        // quote! generates "Subscription :: none" with spaces
        assert!(
            code.contains("Subscription") && code.contains("none"),
            "code: {}",
            code
        );
        assert!(!code.contains("watch_system_theme"), "code: {}", code);
    }

    #[test]
    fn test_generate_system_theme_variant() {
        let config = SubscriptionConfig {
            system_theme: true,
            message_name: "Message".to_string(),
            system_theme_variant: Some("SystemThemeChanged".to_string()),
        };

        let tokens = generate_system_theme_variant(&config);
        assert!(tokens.is_some());

        let code = tokens.unwrap().to_string();
        assert!(code.contains("SystemThemeChanged"));
        assert!(code.contains("String"));
    }

    #[test]
    fn test_generate_system_theme_update_arm() {
        let config = SubscriptionConfig {
            system_theme: true,
            message_name: "Message".to_string(),
            system_theme_variant: Some("SystemThemeChanged".to_string()),
        };

        let tokens = generate_system_theme_update_arm(&config);
        assert!(tokens.is_some());

        let code = tokens.unwrap().to_string();
        // quote! generates "Message :: SystemThemeChanged" with spaces
        assert!(
            code.contains("Message") && code.contains("SystemThemeChanged"),
            "code: {}",
            code
        );
        assert!(code.contains("theme_name"), "code: {}", code);
        // quote! generates "Task :: none" with spaces
        assert!(
            code.contains("Task") && code.contains("none"),
            "code: {}",
            code
        );
    }
}
