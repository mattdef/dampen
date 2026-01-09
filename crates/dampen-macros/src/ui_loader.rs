//! Helper module for the #[dampen_ui] macro.
//!
//! This module contains the implementation details for the dampen_ui macro.

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{ItemMod, LitStr};

/// Process the #[dampen_ui] attribute macro.
///
/// Usage:
/// ```rust,ignore
/// use dampen_macros::dampen_ui;
///
/// #[dampen_ui("app.dampen")]
/// mod app_dampen {}
/// ```
///
/// This generates a module with a `document()` function that returns a cloned DampenDocument.
///
/// # Error Codes
///
/// - **D001**: File not found - The specified .dampen file does not exist
/// - **D002**: Invalid XML - The XML content could not be parsed
/// - **D004**: Parse error - DampenDocument parsing failed
pub fn process_dampen_ui(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the path from macro attributes
    let file_path = if let Ok(path) = syn::parse::<LitStr>(attr) {
        path.value()
    } else {
        return syn::Error::new(
            Span::call_site(),
            "Expected a string literal for the path argument\n\
             Usage: #[dampen_ui(\"app.dampen\")]",
        )
        .to_compile_error()
        .into();
    };

    // Parse the input as a module
    #[allow(clippy::expect_used)]
    let input = syn::parse::<ItemMod>(item)
        .map_err(|e| syn::Error::new(Span::call_site(), format!("Expected a module item: {}", e)))
        .expect("Failed to parse module");

    let module_ident = &input.ident;

    // Generate the module with document
    // We use LazyLock for thread-safe lazy initialization of the document
    // since include_str! and parse() cannot be called at compile time
    let expanded = quote! {
        mod #module_ident {
            use dampen_core::parse;
            use std::sync::LazyLock;

            fn __load_document() -> dampen_core::DampenDocument {
                let xml = include_str!(#file_path);
                #[allow(clippy::expect_used)]
                parse(xml).expect("Failed to parse Dampen UI file")
            }

            pub static DOCUMENT: LazyLock<dampen_core::DampenDocument> =
                LazyLock::new(__load_document);

            pub fn document() -> dampen_core::DampenDocument {
                (*DOCUMENT).clone()
            }
        }
    };

    TokenStream::from(expanded)
}

/// Generate a handler for unknown handler references.
///
/// This can be used at compile time to warn about handlers that
/// are referenced in the UI but not registered.
#[allow(dead_code)]
pub fn warn_unknown_handler(handler_name: &str) -> TokenStream {
    let warning = format!(
        "Handler '{}' is not registered in the HandlerRegistry\n\
         help: Register handlers manually with HandlerRegistry::register_simple() or check for typos",
        handler_name
    );

    quote::quote! {
        compile_error!(#warning);
    }
    .into()
}
