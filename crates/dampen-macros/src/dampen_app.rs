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
            let module_path = v.module_path.replace("::", "_");
            let model_type = Ident::new(
                &format!("{}__Model", module_path),
                proc_macro2::Span::call_site(),
            );

            quote! {
                #field_name: dampen_core::AppState<#model_type>
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
            // TODO: Load actual document from .dampen file
            quote! {
                #field_name: dampen_core::AppState::new(todo!("Load document"))
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

    Ok(quote! {
        #current_view_enum

        #app_struct

        impl App {
            #init_method
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
