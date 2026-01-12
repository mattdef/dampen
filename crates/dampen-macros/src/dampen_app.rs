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

/// Configuration attributes parsed from the `#[dampen_app(...)]` macro.
///
/// Contains all user-specified parameters that control code generation for multi-view applications.
/// The macro parses these from the attribute syntax and uses them to discover views and generate code.
///
/// # Required Attributes
///
/// - `ui_dir`: Directory to scan for `.dampen` files (relative to crate root, e.g., `"src/ui"`)
/// - `message_type`: Name of the user's Message enum (e.g., `Message`)
/// - `handler_variant`: Message variant for handler dispatch (e.g., `Handler`)
///
/// # Optional Attributes
///
/// - `hot_reload_variant`: Message variant for hot-reload events (enables file watching in debug builds)
/// - `dismiss_error_variant`: Message variant for error overlay dismissal (enables error overlay in debug builds)
/// - `exclude`: Glob patterns to exclude from discovery (e.g., `["debug", "experimental/*"]`)
/// - `default_view`: View to display on startup (without `.dampen` extension, defaults to first alphabetically)
///
/// # Examples
///
/// Minimal configuration:
///
/// ```ignore
/// #[dampen_app(
///     ui_dir = "src/ui",
///     message_type = "Message",
///     handler_variant = "Handler"
/// )]
/// struct MyApp;
/// ```
///
/// Full configuration with all options:
///
/// ```ignore
/// #[dampen_app(
///     ui_dir = "src/ui",
///     message_type = "Message",
///     handler_variant = "Handler",
///     hot_reload_variant = "HotReload",
///     dismiss_error_variant = "DismissError",
///     exclude = ["debug", "experimental/*"],
///     default_view = "window"
/// )]
/// struct MyApp;
/// ```
///
/// # Validation
///
/// The macro validates that:
/// - All required attributes are present
/// - `ui_dir` exists and is a directory
/// - Exclusion patterns compile as valid globs
/// - `default_view` (if specified) exists in discovered views
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

    /// Optional: Default view to display on startup (without .dampen extension)
    /// If not specified, defaults to first view alphabetically
    pub default_view: Option<String>,
}

impl Parse for MacroAttributes {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut ui_dir = None;
        let mut message_type = None;
        let mut handler_variant = None;
        let mut hot_reload_variant = None;
        let mut dismiss_error_variant = None;
        let mut exclude = Vec::new();
        let mut default_view = None;

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
            } else if key == "default_view" {
                let value: LitStr = input.parse()?;
                let view_name = value.value();
                // Strip .dampen extension if provided
                let view_name = view_name
                    .strip_suffix(".dampen")
                    .unwrap_or(&view_name)
                    .to_string();
                default_view = Some(view_name);
            } else if key == "exclude" {
                // Parse array of string literals: ["debug", "experimental/*"]
                let content;
                syn::bracketed!(content in input);

                while !content.is_empty() {
                    let pattern: LitStr = content.parse()?;
                    exclude.push(pattern.value());

                    // Parse optional comma
                    if content.peek(Token![,]) {
                        content.parse::<Token![,]>()?;
                    }
                }
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

        // Validate exclude patterns
        for pattern in &exclude {
            if let Err(e) = globset::Glob::new(pattern) {
                return Err(syn::Error::new(
                    input.span(),
                    format!(
                        "Invalid exclude pattern '{}': {}\nhelp: Use glob patterns like 'debug' or 'experimental/*'",
                        pattern, e
                    ),
                ));
            }
        }

        Ok(MacroAttributes {
            ui_dir,
            message_type,
            handler_variant,
            hot_reload_variant,
            dismiss_error_variant,
            exclude,
            default_view,
        })
    }
}

/// Generates the `CurrentView` enum from discovered view files.
///
/// Creates an enum with one variant per view file, used to track which view is currently active.
/// The enum derives `Debug`, `Clone`, `PartialEq`, and `Eq` for convenient usage.
///
/// # Arguments
///
/// * `views` - Slice of discovered view information (file paths, variant names)
///
/// # Returns
///
/// Token stream containing the enum definition:
///
/// ```ignore
/// #[derive(Debug, Clone, PartialEq, Eq)]
/// pub enum CurrentView {
///     Window,
///     Settings,
///     // ... one variant per view
/// }
/// ```
///
/// # Examples
///
/// Given views `["window.dampen", "settings.dampen"]`, generates:
///
/// ```ignore
/// #[derive(Debug, Clone, PartialEq, Eq)]
/// pub enum CurrentView {
///     Window,
///     Settings,
/// }
/// ```
pub fn generate_current_view_enum(views: &[ViewInfo]) -> TokenStream {
    let variants: Vec<_> = views
        .iter()
        .map(|v| Ident::new(&v.variant_name, proc_macro2::Span::call_site()))
        .collect();

    quote! {
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub enum CurrentView {
            #(#variants),*
        }
    }
}

/// Generates the main application struct with AppState fields for each view.
///
/// Creates a struct with:
/// - One `AppState` field per discovered view
/// - A `current_view: CurrentView` field to track the active view
/// - An `error_overlay` field if `dismiss_error_variant` is specified (debug builds only)
///
/// # Arguments
///
/// * `views` - Slice of discovered view information
/// * `_message_type` - The user's Message enum identifier (currently unused)
/// * `attrs` - Parsed macro attributes containing configuration
/// * `struct_name` - Name of the struct to generate
///
/// # Returns
///
/// Token stream containing the struct definition with appropriate fields.
///
/// # Examples
///
/// For views `["window.dampen", "settings.dampen"]` with struct name `MyApp`:
///
/// ```ignore
/// pub struct MyApp {
///     window_state: dampen_core::AppState<window::Model>,
///     settings_state: dampen_core::AppState<settings::Model>,
///     current_view: CurrentView,
///     #[cfg(debug_assertions)]
///     error_overlay: dampen_dev::ErrorOverlay,
/// }
/// ```
pub fn generate_app_struct(
    views: &[ViewInfo],
    _message_type: &Ident,
    attrs: &MacroAttributes,
    struct_name: &Ident,
) -> TokenStream {
    let fields: Vec<_> = views
        .iter()
        .map(|v| {
            let field_name = Ident::new(&v.field_name, proc_macro2::Span::call_site());

            // Convert module_path to Rust path tokens (e.g., "ui::home" -> ui::home::Model)
            let module_parts: Vec<_> = v
                .module_path
                .split("::")
                .map(|part| Ident::new(part, proc_macro2::Span::call_site()))
                .collect();

            quote! {
                #field_name: dampen_core::AppState<#(#module_parts)::*::Model>
            }
        })
        .collect();

    // Add error_overlay field if dismiss_error_variant is specified
    let error_overlay_field = if attrs.dismiss_error_variant.is_some() {
        Some(quote! {
            #[cfg(debug_assertions)]
            error_overlay: dampen_dev::ErrorOverlay
        })
    } else {
        None
    };

    quote! {
        pub struct #struct_name {
            #(#fields,)*
            current_view: CurrentView,
            #error_overlay_field
        }
    }
}

/// Generates the `init()` method to initialize all AppState fields.
///
/// Creates initialization logic that:
/// - Creates an `AppState` for each view by calling `create_{view}_state()`
/// - Sets `current_view` to either the user-specified `default_view` or the first view alphabetically
/// - Initializes the error overlay if `dismiss_error_variant` is specified
///
/// # Arguments
///
/// * `views` - Slice of discovered view information
/// * `attrs` - Parsed macro attributes (for `default_view` and `dismiss_error_variant`)
///
/// # Returns
///
/// Token stream containing the `init()` method implementation.
///
/// # Examples
///
/// With `default_view = "window"`:
///
/// ```ignore
/// pub fn init() -> Self {
///     Self {
///         window_state: create_window_state(),
///         settings_state: create_settings_state(),
///         current_view: CurrentView::Window,  // User-specified
///         #[cfg(debug_assertions)]
///         error_overlay: dampen_dev::ErrorOverlay::new(),
///     }
/// }
/// ```
pub fn generate_init_method(views: &[ViewInfo], attrs: &MacroAttributes) -> TokenStream {
    // Determine the default view variant
    let first_variant = if let Some(ref default_view_name) = attrs.default_view {
        // User specified a default view - find it (validated in dampen_app_impl)
        if let Some(default_view) = views.iter().find(|v| v.view_name == *default_view_name) {
            Ident::new(&default_view.variant_name, proc_macro2::Span::call_site())
        } else {
            // This should never happen due to validation, but handle gracefully
            return quote! {
                compile_error!(concat!("Default view '", #default_view_name, "' not found"));
            };
        }
    } else if let Some(first) = views.first() {
        // No default specified - use first alphabetically
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

            // Convert module_path to Rust path tokens (e.g., "ui::window" -> ui::window)
            let module_parts: Vec<_> = v
                .module_path
                .split("::")
                .map(|part| Ident::new(part, proc_macro2::Span::call_site()))
                .collect();

            quote! {
                #field_name: #(#module_parts)::*::create_app_state()
            }
        })
        .collect();

    // Add error_overlay initialization if dismiss_error_variant is specified
    let error_overlay_init = if attrs.dismiss_error_variant.is_some() {
        Some(quote! {
            #[cfg(debug_assertions)]
            error_overlay: dampen_dev::ErrorOverlay::new(),
        })
    } else {
        None
    };

    quote! {
        pub fn init() -> Self {
            Self {
                #(#field_inits,)*
                current_view: CurrentView::#first_variant,
                #error_overlay_init
            }
        }

        pub fn new() -> Self {
            Self::init()
        }
    }
}

/// Generates convenience `switch_to_*()` methods for each view.
///
/// Creates helper methods that update the `current_view` field, providing an ergonomic API
/// for view switching in user code.
///
/// # Arguments
///
/// * `views` - Slice of discovered view information
///
/// # Returns
///
/// Token stream containing one method per view:
///
/// ```ignore
/// pub fn switch_to_window(&mut self) {
///     self.current_view = CurrentView::Window;
/// }
///
/// pub fn switch_to_settings(&mut self) {
///     self.current_view = CurrentView::Settings;
/// }
/// // ... one method per view
/// ```
///
/// # Examples
///
/// Users can call these methods in their update logic:
///
/// ```ignore
/// fn custom_update(&mut self, msg: Message) {
///     match msg {
///         Message::GoToSettings => self.switch_to_settings(),
///         // ...
///     }
/// }
/// ```
pub fn generate_switch_to_methods(views: &[ViewInfo]) -> TokenStream {
    let methods: Vec<_> = views
        .iter()
        .map(|v| {
            let method_name = Ident::new(
                &format!("switch_to_{}", v.view_name),
                proc_macro2::Span::call_site(),
            );
            let variant = Ident::new(&v.variant_name, proc_macro2::Span::call_site());

            quote! {
                pub fn #method_name(&mut self) {
                    self.current_view = CurrentView::#variant;
                }
            }
        })
        .collect();

    quote! {
        #(#methods)*
    }
}

/// Generates the `update()` method with handler routing, view switching, and error handling.
///
/// Creates update logic that:
/// - Routes `Handler` messages to the appropriate view's handler registry
/// - Supports hot-reload file change events (debug builds only)
/// - Supports error overlay dismissal (debug builds only)
/// - Returns `iced::Task` for asynchronous operations
///
/// # Arguments
///
/// * `views` - Slice of discovered view information
/// * `attrs` - Parsed macro attributes (for handler_variant, hot_reload_variant, dismiss_error_variant)
///
/// # Returns
///
/// Token stream containing the `update()` method implementation.
///
/// # Examples
///
/// Generated update method structure:
///
/// ```ignore
/// pub fn update(&mut self, message: Message) -> iced::Task<Message> {
///     match message {
///         Message::Handler(handler_msg) => {
///             match self.current_view {
///                 CurrentView::Window => self.window_state.update(handler_msg),
///                 CurrentView::Settings => self.settings_state.update(handler_msg),
///             }
///         }
///         #[cfg(debug_assertions)]
///         Message::HotReload(path) => { /* reload logic */ }
///         #[cfg(debug_assertions)]
///         Message::DismissError => { /* dismiss error overlay */ }
///     }
/// }
/// ```
pub fn generate_update_method(views: &[ViewInfo], attrs: &MacroAttributes) -> TokenStream {
    let handler_variant = &attrs.handler_variant;
    let message_type = &attrs.message_type;

    // Generate match arms for each view's handler dispatch
    let view_match_arms: Vec<_> = views
        .iter()
        .map(|v| {
            let variant = Ident::new(&v.variant_name, proc_macro2::Span::call_site());
            let _field_name = Ident::new(&v.field_name, proc_macro2::Span::call_site());

            quote! {
                CurrentView::#variant => {
                    // TODO: Implement handler dispatch when HandlerMessage API is ready
                    // For now, do nothing
                }
            }
        })
        .collect();

    // Generate hot-reload file matching arms if hot_reload_variant is specified
    let hot_reload_match_arms: Vec<_> =
        if attrs.hot_reload_variant.is_some() && attrs.dismiss_error_variant.is_some() {
            views
                .iter()
                .map(|v| {
                    let _field_name = Ident::new(&v.field_name, proc_macro2::Span::call_site());
                    let dampen_file_name = v
                        .dampen_file
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or(&v.view_name);

                    quote! {
                        if path_str.ends_with(#dampen_file_name) {
                            // Reload succeeded, clear any error overlay
                            #[cfg(debug_assertions)]
                            {
                                self.error_overlay.hide();
                            }
                            return iced::Task::none();
                        }
                    }
                })
                .collect()
        } else if attrs.hot_reload_variant.is_some() {
            // No error overlay, just match files without clearing overlay
            views
                .iter()
                .map(|v| {
                    let _field_name = Ident::new(&v.field_name, proc_macro2::Span::call_site());
                    let dampen_file_name = v
                        .dampen_file
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or(&v.view_name);

                    quote! {
                        if path_str.ends_with(#dampen_file_name) {
                            return iced::Task::none();
                        }
                    }
                })
                .collect()
        } else {
            vec![]
        };

    // Generate HotReload match arm if hot_reload_variant is specified
    let hot_reload_arm = if let Some(hot_reload_variant) = &attrs.hot_reload_variant {
        let parse_error_handling = if attrs.dismiss_error_variant.is_some() {
            quote! {
                // Show error overlay
                #[cfg(debug_assertions)]
                {
                    self.error_overlay.show(error);
                }
                iced::Task::none()
            }
        } else {
            quote! {
                // No error overlay configured, just log and ignore
                iced::Task::none()
            }
        };

        Some(quote! {
            #[cfg(debug_assertions)]
            #message_type::#hot_reload_variant(event) => {
                match event {
                    dampen_dev::subscription::FileEvent::Success { path, document } => {
                        // Match path to corresponding view and update its AppState
                        if let Some(path_str) = path.to_str() {
                            #(#hot_reload_match_arms)*
                        }
                        iced::Task::none()
                    }
                    dampen_dev::subscription::FileEvent::ParseError { path, error, content: _ } => {
                        #parse_error_handling
                    }
                    dampen_dev::subscription::FileEvent::WatcherError { path: _, error: _ } => {
                        // Ignore watcher errors for now (permissions, etc.)
                        iced::Task::none()
                    }
                }
            }
        })
    } else {
        None
    };

    // Generate DismissError match arm if dismiss_error_variant is specified
    let dismiss_error_arm = attrs
        .dismiss_error_variant
        .as_ref()
        .map(|dismiss_error_variant| {
            quote! {
                #[cfg(debug_assertions)]
                #message_type::#dismiss_error_variant => {
                    self.error_overlay.hide();
                    iced::Task::none()
                }
            }
        });

    quote! {
        pub fn update(&mut self, message: #message_type) -> iced::Task<#message_type> {
            match message {
                #message_type::#handler_variant(_handler_msg) => {
                    match self.current_view {
                        #(#view_match_arms)*
                    }
                    iced::Task::none()
                }
                #hot_reload_arm
                #dismiss_error_arm
                _ => iced::Task::none(),
            }
        }
    }
}

/// Generates the `view()` method with CurrentView matching and error overlay rendering.
///
/// Creates view rendering logic that:
/// - Matches on `current_view` to render the appropriate AppState's UI
/// - Wraps the Message in the user's `Handler` variant
/// - Shows error overlay on top if visible (debug builds only)
///
/// # Arguments
///
/// * `views` - Slice of discovered view information
/// * `attrs` - Parsed macro attributes (for message_type and dismiss_error_variant)
///
/// # Returns
///
/// Token stream containing the `view()` method implementation.
///
/// # Examples
///
/// Generated view method structure:
///
/// ```ignore
/// pub fn view(&self) -> iced::Element<Message> {
///     #[cfg(debug_assertions)]
///     if self.error_overlay.is_visible() {
///         return self.error_overlay.render(Message::DismissError);
///     }
///
///     match self.current_view {
///         CurrentView::Window => self.window_state.view().map(Message::Handler),
///         CurrentView::Settings => self.settings_state.view().map(Message::Handler),
///     }
/// }
/// ```
pub fn generate_view_method(views: &[ViewInfo], attrs: &MacroAttributes) -> TokenStream {
    let _handler_variant = &attrs.handler_variant;
    let message_type = &attrs.message_type;

    // Generate match arms for each view's rendering
    let view_match_arms: Vec<_> = views
        .iter()
        .map(|v| {
            let variant = Ident::new(&v.variant_name, proc_macro2::Span::call_site());
            let _field_name = Ident::new(&v.field_name, proc_macro2::Span::call_site());

            quote! {
                CurrentView::#variant => {
                    dampen_iced::DampenWidgetBuilder::from_app_state(&self.#_field_name)
                        .build()
                        .map(#message_type::#_handler_variant)
                }
            }
        })
        .collect();

    // Generate error overlay rendering if dismiss_error_variant is specified
    let error_overlay_check = attrs
        .dismiss_error_variant
        .as_ref()
        .map(|dismiss_error_variant| {
            quote! {
                // Show error overlay if visible (debug builds only)
                #[cfg(debug_assertions)]
                if self.error_overlay.is_visible() {
                    return self.error_overlay.render(#message_type::#dismiss_error_variant);
                }
            }
        });

    quote! {
        pub fn view(&self) -> iced::Element<'_, #message_type> {
            #error_overlay_check

            match self.current_view {
                #(#view_match_arms)*
            }
        }
    }
}

/// Generates the `subscription()` method for hot-reload file watching (debug builds only).
///
/// Creates subscription logic that:
/// - Watches all `.dampen` files for changes (debug builds only)
/// - Sends `HotReload` messages when files change
/// - Returns `iced::Subscription::none()` in release builds
/// - Only generated if `hot_reload_variant` is specified in attributes
///
/// # Arguments
///
/// * `views` - Slice of discovered view information
/// * `attrs` - Parsed macro attributes (for hot_reload_variant and ui_dir)
/// * `message_type` - The user's Message enum identifier
///
/// # Returns
///
/// Token stream containing the `subscription()` method implementation.
///
/// # Examples
///
/// Generated subscription method:
///
/// ```ignore
/// pub fn subscription(&self) -> iced::Subscription<Message> {
///     #[cfg(debug_assertions)]
///     {
///         let paths = vec![
///             std::path::PathBuf::from("src/ui/window.dampen"),
///             std::path::PathBuf::from("src/ui/settings.dampen"),
///         ];
///         dampen_dev::watch_files(paths).map(Message::HotReload)
///     }
///     #[cfg(not(debug_assertions))]
///     iced::Subscription::none()
/// }
/// ```
pub fn generate_subscription_method(
    views: &[ViewInfo],
    attrs: &MacroAttributes,
) -> Option<TokenStream> {
    let hot_reload_variant = attrs.hot_reload_variant.as_ref()?;
    let message_type = &attrs.message_type;

    // Collect all .dampen file paths from views
    let watch_paths: Vec<_> = views
        .iter()
        .map(|v| {
            let path = v.dampen_file.to_string_lossy().to_string();
            quote! { std::path::PathBuf::from(#path) }
        })
        .collect();

    Some(quote! {
        #[cfg(debug_assertions)]
        pub fn subscription(&self) -> iced::Subscription<#message_type> {
            dampen_dev::subscription::watch_files(
                vec![#(#watch_paths),*],
                100  // 100ms debounce
            ).map(#message_type::#hot_reload_variant)
        }
    })
}

/// Main macro implementation
/// Implementation of the `#[dampen_app]` procedural macro.
///
/// This is the main entry point that:
/// 1. Parses macro attributes (`ui_dir`, `message_type`, etc.)
/// 2. Discovers all `.dampen` files in the specified directory
/// 3. Validates discovered views and configuration
/// 4. Generates all necessary code (enum, struct, methods)
///
/// # Arguments
///
/// * `attr` - Token stream containing macro attributes (e.g., `ui_dir = "src/ui"`)
/// * `item` - Token stream containing the annotated struct
///
/// # Returns
///
/// Result containing the generated token stream or a compile error.
///
/// # Errors
///
/// Returns `syn::Error` if:
/// - Required attributes are missing
/// - `ui_dir` doesn't exist
/// - No `.dampen` files are discovered
/// - View names conflict or are invalid Rust identifiers
/// - Corresponding `.rs` module files don't exist
/// - `default_view` is specified but doesn't exist in discovered views
///
/// # Examples
///
/// Input:
///
/// ```ignore
/// #[dampen_app(
///     ui_dir = "src/ui",
///     message_type = "Message",
///     handler_variant = "Handler"
/// )]
/// struct MyApp;
/// ```
///
/// Generates struct definition with fields, `init()`, `update()`, `view()`, `subscription()`, etc.
#[doc(hidden)] // Not part of public API, but accessible to tests via #[path]
pub fn dampen_app_impl(attr: TokenStream, item: TokenStream) -> Result<TokenStream, syn::Error> {
    // Parse attributes
    let attrs: MacroAttributes = syn::parse2(attr)?;

    // Parse the input struct to extract its name
    let input_struct: syn::ItemStruct = syn::parse2(item)?;
    let struct_name = &input_struct.ident;

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

    // Validate default_view if specified
    if let Some(ref default_view_name) = attrs.default_view {
        let view_exists = views.iter().any(|v| v.view_name == *default_view_name);
        if !view_exists {
            let available_views: Vec<_> = views.iter().map(|v| v.view_name.as_str()).collect();
            return Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                format!(
                    "default_view '{}' not found in discovered views\nhelp: Available views: {}\nhelp: Use default_view = \"{}\" (without .dampen extension)",
                    default_view_name,
                    available_views.join(", "),
                    available_views.first().unwrap_or(&"window")
                ),
            ));
        }
    }

    // Generate code
    let current_view_enum = generate_current_view_enum(&views);
    let app_struct = generate_app_struct(&views, &attrs.message_type, &attrs, struct_name);
    let init_method = generate_init_method(&views, &attrs);
    let switch_to_methods = generate_switch_to_methods(&views);
    let update_method = generate_update_method(&views, &attrs);
    let view_method = generate_view_method(&views, &attrs);
    let subscription_method = generate_subscription_method(&views, &attrs);

    // Build impl block with optional subscription method
    let impl_methods = if let Some(subscription) = subscription_method {
        quote! {
            impl #struct_name {
                #init_method
                #switch_to_methods
                #update_method
                #view_method
                #subscription
            }
        }
    } else {
        quote! {
            impl #struct_name {
                #init_method
                #switch_to_methods
                #update_method
                #view_method
            }
        }
    };

    Ok(quote! {
        #current_view_enum

        #app_struct

        #impl_methods
    })
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_macro_attributes_structure() {
        // Basic structure test
        assert!(true);
    }
}
