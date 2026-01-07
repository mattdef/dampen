//! Helper module for the #[gravity_ui] macro.
//!
//! This module contains the implementation details for the gravity_ui macro.

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{ItemMod, LitStr};

/// Process the #[gravity_ui] attribute macro.
///
/// Usage:
/// ```rust,ignore
/// use gravity_macros::gravity_ui;
///
/// #[gravity_ui("app.gravity")]
/// mod app_gravity {}
/// ```
///
/// This generates a module with a `document()` function that returns a cloned GravityDocument.
///
/// # Error Codes
///
/// - **G001**: File not found - The specified .gravity file does not exist
/// - **G002**: Invalid XML - The XML content could not be parsed
/// - **G004**: Parse error - GravityDocument parsing failed
pub fn process_gravity_ui(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the path from macro attributes
    let file_path = if let Ok(path) = syn::parse::<LitStr>(attr) {
        path.value()
    } else {
        return syn::Error::new(
            Span::call_site(),
            "Expected a string literal for the path argument\n\
             Usage: #[gravity_ui(\"app.gravity\")]",
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
            use gravity_core::parse;
            use std::sync::LazyLock;

            fn __load_document() -> gravity_core::GravityDocument {
                let xml = include_str!(#file_path);
                #[allow(clippy::expect_used)]
                parse(xml).expect("Failed to parse Gravity UI file")
            }

            pub static DOCUMENT: LazyLock<gravity_core::GravityDocument> =
                LazyLock::new(__load_document);

            pub fn document() -> gravity_core::GravityDocument {
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
