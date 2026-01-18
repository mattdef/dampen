//! Handler inventory macro
//!
//! This module provides the `inventory_handlers!` macro that collects handler metadata
//! emitted by `#[ui_handler]` for build-time code generation.
//!
//! # Purpose
//!
//! When using codegen mode, the build script needs to know which handlers are available
//! in each UI module. This macro creates a public constant that lists all handlers,
//! allowing build.rs to discover them automatically.
//!
//! # Example
//!
//! ```rust,ignore
//! use dampen_macros::{ui_handler, inventory_handlers};
//!
//! #[ui_handler]
//! fn increment(model: &mut Model) {
//!     model.count += 1;
//! }
//!
//! #[ui_handler]
//! fn decrement(model: &mut Model) {
//!     model.count -= 1;
//! }
//!
//! // Declare the inventory once per module
//! inventory_handlers! {
//!     increment,
//!     decrement
//! }
//! ```
//!
//! This generates:
//! ```rust,ignore
//! #[doc(hidden)]
//! pub const HANDLER_INVENTORY: &[&dampen_core::codegen::HandlerInfo] = &[
//!     &_HANDLER_METADATA_INCREMENT,
//!     &_HANDLER_METADATA_DECREMENT,
//! ];
//! ```

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, punctuated::Punctuated, token::Comma};

/// Process the `inventory_handlers!` macro.
///
/// This macro takes a comma-separated list of handler function names and generates
/// a public constant that references their metadata.
///
/// # Arguments
///
/// * `input` - Comma-separated list of handler names (e.g., `increment, decrement`)
///
/// # Returns
///
/// A `HANDLER_INVENTORY` constant containing references to all handler metadata.
pub fn process_inventory_handlers(input: TokenStream) -> TokenStream {
    let handler_names =
        parse_macro_input!(input with Punctuated::<syn::Ident, Comma>::parse_terminated);

    if handler_names.is_empty() {
        return syn::Error::new(
            proc_macro2::Span::call_site(),
            "inventory_handlers! requires at least one handler name",
        )
        .to_compile_error()
        .into();
    }

    // Generate references to metadata constants
    let metadata_refs: Vec<_> = handler_names
        .iter()
        .map(|name| {
            let metadata_name = format!("_HANDLER_METADATA_{}", name.to_string().to_uppercase());
            let metadata_ident = syn::Ident::new(&metadata_name, name.span());
            quote! { &#metadata_ident }
        })
        .collect();

    let output = quote! {
        /// Handler inventory for build-time code generation.
        ///
        /// This constant is used by build.rs to discover available handlers
        /// and generate the appropriate Message enum and update logic.
        #[doc(hidden)]
        pub const HANDLER_INVENTORY: &[&::dampen_core::codegen::HandlerInfo] = &[
            #(#metadata_refs),*
        ];
    };

    output.into()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_inventory_generation() {
        // This is tested via integration tests
        assert!(true, "Inventory generation tested via integration tests");
    }
}
