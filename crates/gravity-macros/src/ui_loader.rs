//! Helper module for the #[gravity_ui] macro.
//!
//! This module contains the implementation details for the gravity_ui macro.

use proc_macro::TokenStream;
use quote::quote;
use syn::LitStr;

/// Process the gravity_ui macro attributes and generate code.
pub fn process_gravity_ui(attr: TokenStream) -> TokenStream {
    let file_path = if let Ok(path) = syn::parse::<LitStr>(attr) {
        path.value()
    } else {
        return syn::Error::new(
            proc_macro2::Span::call_site(),
            "Expected a string literal for the path argument\n\
             Usage: #[gravity_ui(path = \"ui/app.gravity\")]",
        )
        .to_compile_error()
        .into();
    };

    // Generate the module with document
    let expanded = quote! {
        mod __gravity_ui_module {
            use gravity_core::GravityDocument;

            pub static document: GravityDocument = include!(#file_path);
        }
    };

    TokenStream::from(expanded)
}
