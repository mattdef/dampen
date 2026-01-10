//! Dampen Macros - Proc Macros for UI Framework
//!
//! This crate provides procedural macros for the Dampen UI framework.

use proc_macro::TokenStream;

mod ui_handler;
mod ui_loader;
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

/// Attribute macro to automatically load Dampen UI files.
///
/// This macro generates code to load and parse `.dampen` XML files at compile time.
/// The behavior depends on the active feature flags (codegen vs interpreted mode).
///
/// # Example
///
/// ```rust,ignore
/// use dampen_macros::dampen_ui;
///
/// #[dampen_ui("app.dampen")]
/// mod _app {}
///
/// // Use the generated module
/// let document = _app::document();
/// ```
#[proc_macro_attribute]
pub fn dampen_ui(attr: TokenStream, item: TokenStream) -> TokenStream {
    ui_loader::process_dampen_ui(attr, item)
}

/// Attribute macro to mark UI event handlers.
///
/// This macro emits metadata for build-time code generation and handler validation.
///
/// # Example
///
/// ```rust,ignore
/// use dampen_macros::ui_handler;
///
/// #[ui_handler]
/// fn on_click(model: &mut Model) {
///     model.count += 1;
/// }
/// ```
#[proc_macro_attribute]
pub fn ui_handler(attr: TokenStream, item: TokenStream) -> TokenStream {
    ui_handler::process_ui_handler(attr, item)
}
