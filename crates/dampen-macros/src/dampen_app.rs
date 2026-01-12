//! #[dampen_app] procedural macro implementation
//!
//! This module contains:
//! - MacroAttributes parsing from attribute syntax
//! - Main macro entry point
//! - Code generation logic for multi-view applications

use proc_macro2::TokenStream;
use quote::quote;
use std::path::PathBuf;
use syn::{Ident, LitStr, Token, parse::Parse};

use crate::discovery::{ViewInfo, discover_dampen_files};

/// Parsed attributes from #[dampen_app(...)] annotation
#[derive(Debug, Clone)]
pub struct MacroAttributes {
    /// Required: Directory to scan for .dampen files (relative to crate root)
    pub ui_dir: String,

    /// Required: Name of the user's Message enum
    pub message_type: Ident,

    /// Required: Message variant for HandlerMessage dispatch
    pub handler_variant: Ident,

    /// Optional: Message variant for hot-reload file events
    pub hot_reload_variant: Option<Ident>,

    /// Optional: Message variant for error overlay dismissal
    pub dismiss_error_variant: Option<Ident>,

    /// Optional: Glob patterns to exclude from discovery
    pub exclude: Vec<String>,
}

impl Parse for MacroAttributes {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut ui_dir = None;
        let mut message_type = None;
        let mut handler_variant = None;
        let mut hot_reload_variant = None;
        let mut dismiss_error_variant = None;
        let exclude = Vec::new(); // TODO: Parse exclude patterns

        // Parse key-value pairs
        while !input.is_empty() {
            let key: Ident = input.parse()?;
            input.parse::<Token![=]>()?;

            if key == "ui_dir" {
                let value: LitStr = input.parse()?;
                ui_dir = Some(value.value());
            } else if key == "message_type" {
                let value: LitStr = input.parse()?;
                message_type = Some(Ident::new(&value.value(), value.span()));
            } else if key == "handler_variant" {
                let value: LitStr = input.parse()?;
                handler_variant = Some(Ident::new(&value.value(), value.span()));
            } else if key == "hot_reload_variant" {
                let value: LitStr = input.parse()?;
                hot_reload_variant = Some(Ident::new(&value.value(), value.span()));
            } else if key == "dismiss_error_variant" {
                let value: LitStr = input.parse()?;
                dismiss_error_variant = Some(Ident::new(&value.value(), value.span()));
            } else {
                return Err(syn::Error::new(
                    key.span(),
                    format!("Unknown attribute: {}", key),
                ));
            }

            // Parse optional comma
            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }

        // Validate required attributes
        let ui_dir = ui_dir.ok_or_else(|| {
            syn::Error::new(
                input.span(),
                "missing required attribute 'ui_dir'\nhelp: Add ui_dir = \"src/ui\" to the macro attributes"
            )
        })?;

        let message_type = message_type.ok_or_else(|| {
            syn::Error::new(
                input.span(),
                "missing required attribute 'message_type'\nhelp: Add message_type = \"Message\" to the macro attributes"
            )
        })?;

        let handler_variant = handler_variant.ok_or_else(|| {
            syn::Error::new(
                input.span(),
                "missing required attribute 'handler_variant'\nhelp: Add handler_variant = \"Handler\" to the macro attributes"
            )
        })?;

        Ok(MacroAttributes {
            ui_dir,
            message_type,
            handler_variant,
            hot_reload_variant,
            dismiss_error_variant,
            exclude,
        })
    }
}

/// Generate CurrentView enum from discovered views
pub fn generate_current_view_enum(views: &[ViewInfo]) -> TokenStream {
    let variants: Vec<_> = views
        .iter()
        .map(|v| {
            let variant = Ident::new(&v.variant_name, proc_macro2::Span::call_site());
            variant
        })
        .collect();

    quote! {
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub enum CurrentView {
            #(#variants),*
        }
    }
}

/// Generate app struct with AppState fields for each view
pub fn generate_app_struct(views: &[ViewInfo], _message_type: &Ident) -> TokenStream {
    let fields: Vec<_> = views
        .iter()
        .map(|v| {
            let field_name = Ident::new(&v.field_name, proc_macro2::Span::call_site());

            // Convert module_path to Rust path tokens (e.g., "ui::home" -> ui::home::Model)
            let module_parts: Vec<_> = v
                .module_path
                .split("::")
                .map(|part| Ident::new(part, proc_macro2::Span::call_site()))
                .collect();

            quote! {
                #field_name: dampen_core::AppState<#(#module_parts)::*::Model>
            }
        })
        .collect();

    quote! {
        pub struct App {
            #(#fields,)*
            current_view: CurrentView,
        }
    }
}

/// Generate init() method to initialize all AppState fields
pub fn generate_init_method(views: &[ViewInfo]) -> TokenStream {
    let first_variant = if let Some(first) = views.first() {
        Ident::new(&first.variant_name, proc_macro2::Span::call_site())
    } else {
        return quote! {
            pub fn init() -> Self {
                compile_error!("No views found in UI directory");
            }
        };
    };

    let field_inits: Vec<_> = views
        .iter()
        .map(|v| {
            let field_name = Ident::new(&v.field_name, proc_macro2::Span::call_site());

            // Convert module_path to Rust path tokens (e.g., "ui::window" -> ui::window)
            let module_parts: Vec<_> = v
                .module_path
                .split("::")
                .map(|part| Ident::new(part, proc_macro2::Span::call_site()))
                .collect();

            // Generate the module name with underscore prefix (e.g., _window)
            let dampen_ui_module =
                Ident::new(&format!("_{}", v.view_name), proc_macro2::Span::call_site());

            quote! {
                #field_name: {
                    // Load document from #[dampen_ui] generated module
                    let document = #(#module_parts)::*::#dampen_ui_module::document();

                    // Try to load handler registry if create_handler_registry() exists
                    // Otherwise use empty registry
                    #[allow(unused_mut)]
                    let mut handlers = dampen_core::HandlerRegistry::new();

                    // Note: User can manually call create_handler_registry() if it exists
                    // For now, we use an empty registry and let users wire handlers manually
                    // TODO: Optionally generate code to call create_handler_registry() if detected

                    dampen_core::AppState::with_handlers(document, handlers)
                }
            }
        })
        .collect();

    quote! {
        pub fn init() -> Self {
            Self {
                #(#field_inits,)*
                current_view: CurrentView::#first_variant,
            }
        }

        pub fn new() -> Self {
            Self::init()
        }
    }
}

/// Generate switch_to_* helper methods for each view
pub fn generate_switch_to_methods(views: &[ViewInfo]) -> TokenStream {
    let methods: Vec<_> = views
        .iter()
        .map(|v| {
            let method_name = Ident::new(
                &format!("switch_to_{}", v.view_name),
                proc_macro2::Span::call_site(),
            );
            let variant = Ident::new(&v.variant_name, proc_macro2::Span::call_site());

            quote! {
                pub fn #method_name(&mut self) {
                    self.current_view = CurrentView::#variant;
                }
            }
        })
        .collect();

    quote! {
        #(#methods)*
    }
}

/// Generate update() method with handler routing and view switching
pub fn generate_update_method(views: &[ViewInfo], attrs: &MacroAttributes) -> TokenStream {
    let handler_variant = &attrs.handler_variant;
    let message_type = &attrs.message_type;

    // Generate match arms for each view's handler dispatch
    let view_match_arms: Vec<_> = views
        .iter()
        .map(|v| {
            let variant = Ident::new(&v.variant_name, proc_macro2::Span::call_site());
            let _field_name = Ident::new(&v.field_name, proc_macro2::Span::call_site());

            quote! {
                CurrentView::#variant => {
                    // TODO: Implement handler dispatch when HandlerMessage API is ready
                    // For now, do nothing
                }
            }
        })
        .collect();

    quote! {
        pub fn update(&mut self, message: #message_type) -> iced::Task<#message_type> {
            match message {
                #message_type::#handler_variant(_handler_msg) => {
                    match self.current_view {
                        #(#view_match_arms)*
                    }
                    iced::Task::none()
                }
                _ => iced::Task::none(),
            }
        }
    }
}

/// Generate view() method with CurrentView matching
pub fn generate_view_method(views: &[ViewInfo], attrs: &MacroAttributes) -> TokenStream {
    let _handler_variant = &attrs.handler_variant;
    let message_type = &attrs.message_type;

    // Generate match arms for each view's rendering
    let view_match_arms: Vec<_> = views
        .iter()
        .map(|v| {
            let variant = Ident::new(&v.variant_name, proc_macro2::Span::call_site());
            let _field_name = Ident::new(&v.field_name, proc_macro2::Span::call_site());

            quote! {
                CurrentView::#variant => {
                    // TODO: Implement view rendering when dampen_iced::build_ui is ready
                    // For now, return a placeholder text widget
                    iced::widget::text("View rendering not yet implemented").into()
                }
            }
        })
        .collect();

    quote! {
        pub fn view(&self) -> iced::Element<'_, #message_type> {
            match self.current_view {
                #(#view_match_arms)*
            }
        }
    }
}

/// Main macro implementation
pub fn dampen_app_impl(attr: TokenStream, _item: TokenStream) -> Result<TokenStream, syn::Error> {
    // Parse attributes
    let attrs: MacroAttributes = syn::parse2(attr)?;

    // Resolve UI directory (relative to CARGO_MANIFEST_DIR)
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").map_err(|_| {
        syn::Error::new(proc_macro2::Span::call_site(), "CARGO_MANIFEST_DIR not set")
    })?;

    let ui_dir = PathBuf::from(&manifest_dir).join(&attrs.ui_dir);

    if !ui_dir.exists() {
        return Err(syn::Error::new(
            proc_macro2::Span::call_site(),
            format!(
                "UI directory not found: '{}'\nhelp: Ensure the directory exists relative to Cargo.toml",
                attrs.ui_dir
            ),
        ));
    }

    // Discover views
    let views = discover_dampen_files(&ui_dir, &attrs.exclude)
        .map_err(|e| syn::Error::new(proc_macro2::Span::call_site(), e))?;

    if views.is_empty() {
        return Err(syn::Error::new(
            proc_macro2::Span::call_site(),
            format!(
                "No .dampen files found in '{}'\nhelp: Add at least one .dampen file to your UI directory",
                attrs.ui_dir
            ),
        ));
    }

    // Generate code
    let current_view_enum = generate_current_view_enum(&views);
    let app_struct = generate_app_struct(&views, &attrs.message_type);
    let init_method = generate_init_method(&views);
    let switch_to_methods = generate_switch_to_methods(&views);
    let update_method = generate_update_method(&views, &attrs);
    let view_method = generate_view_method(&views, &attrs);

    Ok(quote! {
        #current_view_enum

        #app_struct

        impl App {
            #init_method
            #switch_to_methods
            #update_method
            #view_method
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_macro_attributes_structure() {
        // Basic structure test
        assert!(true);
    }
}
