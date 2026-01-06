//! Helper module for the #[gravity_ui] macro.
//!
//! This module contains the implementation details for the gravity_ui macro.

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::LitStr;

/// Process the gravity_ui macro attributes and generate code.
///
/// The path should be relative to the file containing the `#[gravity_ui]` attribute.
/// For example, if the attribute is in `src/ui/mod.rs` and the gravity file is at
/// `src/ui/app.gravity`, use `app.gravity` as the path.
///
/// # Error Codes
///
/// - **G001**: File not found - The specified .gravity file does not exist
/// - **G002**: Invalid XML - The XML content could not be parsed
/// - **G004**: Parse error - GravityDocument parsing failed
pub fn process_gravity_ui(attr: TokenStream) -> TokenStream {
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

    // Generate the module with document
    // We use LazyLock for thread-safe lazy initialization of the document
    // since include_str! and parse() cannot be called at compile time
    let expanded = quote! {
        pub mod __gravity_ui_module {
            use gravity_core::parse;
            use std::sync::LazyLock;

            fn __load_document() -> gravity_core::GravityDocument {
                let xml = include_str!(#file_path);
                parse(xml).expect("Failed to parse Gravity UI file")
            }

            pub static document: LazyLock<gravity_core::GravityDocument> =
                LazyLock::new(__load_document);
        }

        pub use __gravity_ui_module::document;
    };

    TokenStream::from(expanded)
}

/// Generate a handler for unknown handler references.
///
/// This can be used at compile time to warn about handlers that
/// are referenced in the UI but not registered.
pub fn warn_unknown_handler(handler_name: &str) -> TokenStream {
    let warning = format!(
        "Handler '{}' is not registered in the HandlerRegistry\n\
         help: Add a handler with #[ui_handler] or check for typos",
        handler_name
    );

    quote::quote! {
        compile_error!(#warning);
    }
    .into()
}
