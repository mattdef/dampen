//! Dampen Macros - Proc Macros for UI Framework
//!
//! This crate provides procedural macros for the Dampen UI framework.

use proc_macro::TokenStream;

mod dampen_app;
mod discovery;
mod inventory_handlers;
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

/// Attribute macro for auto-discovering and wiring multi-view applications.
///
/// This macro automatically:
/// - Discovers all `.dampen` UI files in the specified directory
/// - Generates a `CurrentView` enum with variants for each view
/// - Generates view switching logic and handler dispatch
/// - Optionally integrates hot-reload subscriptions
///
/// # Required Attributes
///
/// - `ui_dir`: Directory to scan for `.dampen` files (e.g., `"src/ui"`)
/// - `message_type`: Name of your Message enum (e.g., `"Message"`)
/// - `handler_variant`: Message variant for handler dispatch (e.g., `"Handler"`)
///
/// # Optional Attributes
///
/// - `default_view`: View to display on startup (e.g., `"window"` or `"window.dampen"`)
///   - If not specified, uses first view alphabetically
///   - Extension `.dampen` is automatically stripped if provided
/// - `hot_reload_variant`: Message variant for file change events (e.g., `"HotReload"`)
/// - `dismiss_error_variant`: Message variant for error overlay dismissal (e.g., `"DismissError"`)
/// - `exclude`: Array of glob patterns to exclude views (e.g., `["debug_*", "test/*"]`)
///
/// # Example
///
/// ```rust,ignore
/// use dampen_macros::dampen_app;
///
/// #[dampen_app(
///     ui_dir = "src/ui",
///     message_type = "Message",
///     handler_variant = "Handler",
///     default_view = "window",
///     hot_reload_variant = "HotReload",
///     exclude = ["debug_view"]
/// )]
/// struct App;
/// ```
#[proc_macro_attribute]
pub fn dampen_app(attr: TokenStream, item: TokenStream) -> TokenStream {
    match dampen_app::dampen_app_impl(attr.into(), item.into()) {
        Ok(tokens) => tokens.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

/// Declare an inventory of UI handlers for build-time code generation.
///
/// This macro must be used in any module that contains `#[ui_handler]` functions
/// when building with the `codegen` feature. It creates a constant that the build
/// script can use to discover available handlers.
///
/// # Convention
///
/// Each `.rs` file in the `ui/` directory should have a corresponding `.dampen` file
/// with the same name. For example:
/// - `src/ui/window.rs` ↔ `src/ui/window.dampen`
/// - `src/ui/settings.rs` ↔ `src/ui/settings.dampen`
///
/// # Example
///
/// ```rust,ignore
/// use dampen_macros::{ui_handler, inventory_handlers};
///
/// #[ui_handler]
/// fn increment(model: &mut Model) {
///     model.count += 1;
/// }
///
/// #[ui_handler]
/// fn decrement(model: &mut Model) {
///     model.count -= 1;
/// }
///
/// // Declare all handlers in this module
/// inventory_handlers! {
///     increment,
///     decrement
/// }
/// ```
///
/// # Errors
///
/// This macro will fail at compile time if:
/// - No handler names are provided
/// - A referenced handler doesn't have the `#[ui_handler]` attribute
#[proc_macro]
pub fn inventory_handlers(input: TokenStream) -> TokenStream {
    inventory_handlers::process_inventory_handlers(input)
}
