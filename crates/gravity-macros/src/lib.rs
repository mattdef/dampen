//! Gravity Macros - Proc Macros for UI Framework
//!
//! This crate provides procedural macros for the Gravity UI framework.

use proc_macro::TokenStream;

mod ui_handler;
mod ui_model;

/// Derive macro to generate UiBindable implementation
///
/// # Attributes
///
/// - `#[ui_skip]`: Exclude field from binding
/// - `#[ui_bind]`: Explicitly include field (overrides ui_skip)
#[proc_macro_derive(UiModel, attributes(ui_skip, ui_bind))]
pub fn ui_model_derive(input: TokenStream) -> TokenStream {
    ui_model::ui_model_derive(input)
}

/// Attribute macro to mark event handlers
#[proc_macro_attribute]
pub fn ui_handler(attr: TokenStream, item: TokenStream) -> TokenStream {
    ui_handler::ui_handler(attr, item)
}
