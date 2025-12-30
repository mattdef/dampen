//! Gravity Macros - Proc Macros for UI Framework
//!
//! This crate provides procedural macros for the Gravity UI framework.

use proc_macro::TokenStream;

/// Derive macro to generate UiBindable implementation
#[proc_macro_derive(UiModel)]
pub fn ui_model_derive(input: TokenStream) -> TokenStream {
    // Placeholder implementation
    input
}

/// Attribute macro to mark event handlers
#[proc_macro_attribute]
pub fn ui_handler(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Placeholder implementation
    item
}
