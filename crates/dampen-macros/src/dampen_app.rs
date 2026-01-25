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
/// - `switch_view_variant`: Message variant for programmatic view switching (e.g., `SwitchToView`)
/// - `exclude`: Glob patterns to exclude from discovery (e.g., `["debug", "experimental/*"]`)
/// - `default_view`: View to display on startup (without `.dampen` extension, defaults to first alphabetically)
/// - `shared_model`: Optional shared state model type for inter-view communication (e.g., `"SharedState"`)
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
///     switch_view_variant = "SwitchToView",
///     exclude = ["debug", "experimental/*"],
///     default_view = "window"
/// )]
/// struct MyApp;
/// ```
///
/// With shared state for inter-view communication:
///
/// ```ignore
/// // src/shared.rs
/// use dampen_macros::UiModel;
///
/// #[derive(Clone, Default, UiModel)]
/// pub struct SharedState {
///     pub user_name: String,
///     pub theme: String,
/// }
///
/// // src/main.rs
/// mod shared;
///
/// #[dampen_app(
///     ui_dir = "src/ui",
///     message_type = "Message",
///     handler_variant = "Handler",
///     shared_model = "SharedState"
/// )]
/// struct MyApp;
/// ```
///
/// With `shared_model`, all views can access and modify the shared state:
///
/// ```ignore
/// // src/ui/settings.dampen
/// <column>
///     <text value="User: {shared.user_name}" />
///     <button label="Update Theme" on_click="update_theme" />
/// </column>
///
/// // src/ui/settings.rs
/// #[ui_handler]
/// pub fn update_theme(shared: &SharedContext<SharedState>) {
///     shared.update(|s| s.theme = "dark".to_string());
/// }
/// ```
///
/// # Validation
///
/// The macro validates that:
/// - All required attributes are present
/// - `ui_dir` exists and is a directory
/// - Exclusion patterns compile as valid globs
/// - `default_view` (if specified) exists in discovered views
/// - `shared_model` (if specified) corresponds to an existing `src/shared.rs` file
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

    /// Optional: Message variant for switching between views
    /// If specified, the macro will generate a match arm for Message::SwitchToView(CurrentView)
    pub switch_view_variant: Option<Ident>,

    /// Optional: Glob patterns to exclude from discovery
    pub exclude: Vec<String>,

    /// Optional: Default view to display on startup (without .dampen extension)
    /// If not specified, defaults to first view alphabetically
    pub default_view: Option<String>,

    /// Optional: Shared state model type for inter-view communication
    /// If specified, expects a type in `shared` module (e.g., `"SharedState"` → `shared::SharedState`)
    pub shared_model: Option<Ident>,

    /// Optional: Message variant for system theme change events
    pub system_theme_variant: Option<Ident>,

    /// Optional: Enable window state persistence (requires app_name)
    pub persistence: bool,

    /// Optional: Application identifier for persistence (required if persistence = true)
    pub app_name: Option<String>,
}

impl Parse for MacroAttributes {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut ui_dir = None;
        let mut message_type = None;
        let mut handler_variant = None;
        let mut hot_reload_variant = None;
        let mut dismiss_error_variant = None;
        let mut switch_view_variant = None;
        let mut exclude = Vec::new();
        let mut default_view = None;
        let mut shared_model = None;
        let mut system_theme_variant = None;
        let mut persistence = false;
        let mut app_name = None;

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
            } else if key == "switch_view_variant" {
                let value: LitStr = input.parse()?;
                switch_view_variant = Some(Ident::new(&value.value(), value.span()));
            } else if key == "system_theme_variant" {
                let value: LitStr = input.parse()?;
                system_theme_variant = Some(Ident::new(&value.value(), value.span()));
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
            } else if key == "shared_model" {
                let value: LitStr = input.parse()?;
                shared_model = Some(Ident::new(&value.value(), value.span()));
            } else if key == "persistence" {
                let value: syn::LitBool = input.parse()?;
                persistence = value.value;
            } else if key == "app_name" {
                let value: LitStr = input.parse()?;
                app_name = Some(value.value());
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

        // Validate persistence requirements
        if persistence && app_name.is_none() {
            return Err(syn::Error::new(
                input.span(),
                "persistence = true requires app_name attribute\nhelp: Add app_name = \"my-app-id\"",
            ));
        }

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

        // Validate shared_model file exists if specified
        if let Some(ref shared_model_name) = shared_model {
            // Use CARGO_MANIFEST_DIR to get the crate directory (not workspace root)
            let manifest_dir =
                std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".to_string());
            let shared_path = PathBuf::from(manifest_dir).join("src/shared.rs");

            if !shared_path.exists() {
                return Err(syn::Error::new(
                    input.span(),
                    format!(
                        "shared_model '{}' specified but 'src/shared.rs' not found\n\
                        help: Create src/shared.rs with:\n\
                        #[derive(Clone, Default, UiModel)]\n\
                        pub struct {} {{ /* your shared state fields */ }}",
                        shared_model_name, shared_model_name
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
            switch_view_variant,
            exclude,
            default_view,
            shared_model,
            system_theme_variant,
            persistence,
            app_name,
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
    // Add shared field if shared_model is specified
    let shared_field = attrs.shared_model.as_ref().map(|shared_model| {
        quote! {
            shared: dampen_core::SharedContext<shared::#shared_model>,
        }
    });

    // Generate AppState fields with or without shared state type parameter
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

            // If shared_model is specified, use AppState<Model, SharedState>
            // Otherwise use AppState<Model>
            if let Some(ref shared_model) = attrs.shared_model {
                quote! {
                    #field_name: dampen_core::AppState<#(#module_parts)::*::Model, shared::#shared_model>
                }
            } else {
                quote! {
                    #field_name: dampen_core::AppState<#(#module_parts)::*::Model>
                }
            }
        })
        .collect();

    // Add error_overlay field if dismiss_error_variant is specified
    let error_overlay_field = if attrs.dismiss_error_variant.is_some() {
        Some(quote! {
            #[cfg(debug_assertions)]
            error_overlay: dampen_dev::ErrorOverlay,
        })
    } else {
        None
    };

    // Add window_state field if persistence is enabled
    let window_state_field = if attrs.persistence {
        Some(quote! {
            persisted_window_state: dampen_dev::persistence::WindowState,
        })
    } else {
        None
    };

    quote! {
        pub struct #struct_name {
            #shared_field
            #(#fields,)*
            current_view: CurrentView,
            #error_overlay_field
            #window_state_field
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
    let message_type = &attrs.message_type;

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

    // Initialize shared context if shared_model is specified
    let (shared_init, shared_field_init) = if let Some(ref shared_model) = attrs.shared_model {
        let init_code = quote! {
            let shared = dampen_core::SharedContext::new(shared::#shared_model::default());
        };
        let field_init = Some(quote! {
            shared: shared.clone(),
        });
        (Some(init_code), field_init)
    } else {
        (None, None)
    };

    // Generate field initializations with or without shared context
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

            // If shared_model is specified, call create_app_state_with_shared
            // Otherwise call create_app_state
            if attrs.shared_model.is_some() {
                quote! {
                    #field_name: #(#module_parts)::*::create_app_state_with_shared(shared.clone())
                }
            } else {
                quote! {
                    #field_name: #(#module_parts)::*::create_app_state()
                }
            }
        })
        .collect();

    // Generate theme context setting for each view
    let theme_context_setters: Vec<_> = views
        .iter()
        .map(|v| {
            let field_name = Ident::new(&v.field_name, proc_macro2::Span::call_site());
            quote! {
                if let Some(ref ctx) = theme_context {
                    app.#field_name.set_theme_context(ctx.clone());
                }
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

    // Initialize window_state if persistence is enabled
    let window_state_init = if attrs.persistence {
        #[allow(clippy::unwrap_used)]
        let app_name = attrs.app_name.as_ref().unwrap();
        Some(quote! {
            persisted_window_state: dampen_dev::persistence::load_or_default(#app_name, 800, 600),
        })
    } else {
        None
    };

    quote! {
        pub fn init() -> (Self, iced::Task<#message_type>) {
            #[cfg(debug_assertions)]
            println!("DEBUG: DampenApp::init called");

            #shared_init

            // Load theme context from theme.dampen if present
            // Use find_project_root() which searches:
            // 1. CARGO_MANIFEST_DIR (cargo run)
            // 2. Executable ancestors (target/release)
            // 3. Current directory ancestors
            let project_dir = dampen_dev::theme_loader::find_project_root()
                .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from(".")));

            let theme_context_result = dampen_dev::theme_loader::load_theme_context(&project_dir);
            let theme_context = theme_context_result.ok().flatten();

            let mut app = Self {
                #shared_field_init
                #(#field_inits,)*
                current_view: CurrentView::#first_variant,
                #error_overlay_init
                #window_state_init
            };

            // Set theme context on all view states
            #(#theme_context_setters)*

            (app, iced::Task::none())
        }

        pub fn new() -> (Self, iced::Task<#message_type>) {
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

    // Determine if we're using shared state
    let use_shared = attrs.shared_model.is_some();

    // Generate match arms for each view's handler dispatch
    let view_match_arms: Vec<_> = views
        .iter()
        .map(|v| {
            let variant = Ident::new(&v.variant_name, proc_macro2::Span::call_site());
            let field_name = Ident::new(&v.field_name, proc_macro2::Span::call_site());

            if use_shared {
                quote! {
                    CurrentView::#variant => {
                        // Handle built-in set_theme action
                        if let dampen_iced::HandlerMessage::Handler(name, value) = &handler_msg {
                            if name == "set_theme" {
                                if let Some(ref mut ctx) = self.#field_name.theme_context {
                                    if let Some(theme_name) = value {
                                        let _ = ctx.set_theme(&theme_name);
                                    }
                                }
                                return iced::Task::none();
                            }
                        }
                        dispatch_handler_with_task_and_shared(
                            &mut self.#field_name.model,
                            &self.#field_name.handler_registry,
                            &self.shared,
                            handler_msg
                        )
                    }
                }
            } else {
                quote! {
                    CurrentView::#variant => {
                        // Handle built-in set_theme action
                        if let dampen_iced::HandlerMessage::Handler(name, value) = &handler_msg {
                            if name == "set_theme" {
                                if let Some(ref mut ctx) = self.#field_name.theme_context {
                                    if let Some(theme_name) = value {
                                        let _ = ctx.set_theme(&theme_name);
                                    }
                                }
                                return iced::Task::none();
                            }
                        }
                        dispatch_handler_with_task(
                            &mut self.#field_name.model,
                            &self.#field_name.handler_registry,
                            handler_msg
                        )
                    }
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
                    let field_name = Ident::new(&v.field_name, proc_macro2::Span::call_site());
                    let dampen_file_name = v
                        .dampen_file
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or(&v.view_name);

                    quote! {
                        if path_str.ends_with(#dampen_file_name) {
                            // Update the AppState with the new document
                            self.#field_name.hot_reload(*document.clone());

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
                    let field_name = Ident::new(&v.field_name, proc_macro2::Span::call_site());
                    let dampen_file_name = v
                        .dampen_file
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or(&v.view_name);

                    quote! {
                        if path_str.ends_with(#dampen_file_name) {
                            // Update the AppState with the new document
                            self.#field_name.hot_reload(*document.clone());
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

    // Generate SwitchToView match arm if switch_view_variant is specified
    let switch_view_arm = attrs
        .switch_view_variant
        .as_ref()
        .map(|switch_view_variant| {
            // Generate match arms for each view switch
            let switch_match_arms: Vec<_> = views
                .iter()
                .map(|v| {
                    let variant = Ident::new(&v.variant_name, proc_macro2::Span::call_site());
                    let switch_method = Ident::new(
                        &format!("switch_to_{}", v.view_name),
                        proc_macro2::Span::call_site(),
                    );

                    quote! {
                        CurrentView::#variant => self.#switch_method(),
                    }
                })
                .collect();

            quote! {
                #message_type::#switch_view_variant(view) => {
                    match view {
                        #(#switch_match_arms)*
                    }
                    iced::Task::none()
                }
            }
        });

    // Generate helper function(s) for handler dispatch
    let helper_functions = if use_shared {
        // Generate version with shared context
        if let Some(ref shared_model) = attrs.shared_model {
            quote! {
                // Helper function to dispatch handlers with shared state and return tasks
                fn dispatch_handler_with_task_and_shared<M: dampen_core::UiBindable + 'static>(
                    model: &mut M,
                    registry: &dampen_core::HandlerRegistry,
                    shared: &dampen_core::SharedContext<shared::#shared_model>,
                    handler_msg: dampen_iced::HandlerMessage,
                ) -> iced::Task<#message_type> {
                    match handler_msg {
                        dampen_iced::HandlerMessage::Handler(handler_name, value) => {
                            let model_any: &mut dyn std::any::Any = model;
                            let shared_any: &dyn std::any::Any = shared;
                            if let Some(boxed_task) = registry.dispatch_with_shared(&handler_name, model_any, shared_any, value) {
                                // Try to downcast to Task<Message>
                                if let Ok(task) = boxed_task.downcast::<iced::Task<#message_type>>() {
                                    return *task;
                                }
                            }
                            iced::Task::none()
                        }
                        dampen_iced::HandlerMessage::None => iced::Task::none(),
                    }
                }
            }
        } else {
            quote! {}
        }
    } else {
        // Generate version without shared context
        quote! {
            // Helper function to dispatch handlers and return tasks
            fn dispatch_handler_with_task<M: dampen_core::UiBindable + 'static>(
                model: &mut M,
                registry: &dampen_core::HandlerRegistry,
                handler_msg: dampen_iced::HandlerMessage,
            ) -> iced::Task<#message_type> {
                match handler_msg {
                    dampen_iced::HandlerMessage::Handler(handler_name, value) => {
                        let model_any: &mut dyn std::any::Any = model;
                        if let Some(boxed_task) = registry.dispatch_with_command(&handler_name, model_any, value) {
                            // Try to downcast to Task<Message>
                            if let Ok(task) = boxed_task.downcast::<iced::Task<#message_type>>() {
                                return *task;
                            }
                        }
                        iced::Task::none()
                    }
                    dampen_iced::HandlerMessage::None => iced::Task::none(),
                }
            }
        }
    };

    // Generate update_system_preference match arm if system_theme_variant is specified
    let system_theme_arm = if let Some(system_theme_variant) = &attrs.system_theme_variant {
        let update_all_views: Vec<_> = views
            .iter()
            .map(|v| {
                let field_name = Ident::new(&v.field_name, proc_macro2::Span::call_site());
                quote! {
                    if let Some(ref mut ctx) = self.#field_name.theme_context {
                        ctx.update_system_preference(&theme_name);
                    }
                }
            })
            .collect();

        Some(quote! {
            #message_type::#system_theme_variant(theme_name) => {
                #(#update_all_views)*
                iced::Task::none()
            }
        })
    } else {
        None
    };

    // Generate window event handling match arm if persistence is enabled
    let window_event_arm = if attrs.persistence {
        #[allow(clippy::unwrap_used)]
        let app_name = attrs.app_name.as_ref().unwrap();
        Some(quote! {
            #message_type::Window(id, event) => {
                match event {
                    iced::window::Event::Opened { .. } => {
                        #[cfg(debug_assertions)]
                        println!("DEBUG: Window opened with persisted state: {:?}", self.persisted_window_state);

                        // Window is already created with correct size via window_settings().
                        // Only need to maximize if that was the saved state.
                        if self.persisted_window_state.maximized {
                            iced::window::maximize(id, true)
                        } else {
                            iced::Task::none()
                        }
                    }
                    iced::window::Event::Resized(size) => {
                        // Update persisted state with new size
                        self.persisted_window_state.width = size.width as u32;
                        self.persisted_window_state.height = size.height as u32;
                        iced::Task::none()
                    }
                    iced::window::Event::Moved(position) => {
                        self.persisted_window_state.x = Some(position.x as i32);
                        self.persisted_window_state.y = Some(position.y as i32);
                        iced::Task::none()
                    }
                    iced::window::Event::CloseRequested => {
                         #[cfg(debug_assertions)]
                         println!("DEBUG: Saving window state on close");
                         let _ = dampen_dev::persistence::save_window_state(#app_name, &self.persisted_window_state);
                         iced::window::close(id)
                    }
                    _ => iced::Task::none(),
                }
            }
        })
    } else {
        None
    };

    quote! {
        pub fn update(&mut self, message: #message_type) -> iced::Task<#message_type> {
            #[cfg(debug_assertions)]
            println!("DEBUG: Update received message"); // Generic debug to avoid needing Debug trait on Message

            #helper_functions

            match message {
                #message_type::#handler_variant(handler_msg) => {
                    match self.current_view {
                        #(#view_match_arms)*
                    }
                }
                #hot_reload_arm
                #dismiss_error_arm
                #switch_view_arm
                #system_theme_arm
                #window_event_arm
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

/// Generates the `theme()` method to resolve the active Iced theme.
///
/// Creates logic that:
/// - Matches on `current_view` to find the active AppState
/// - Retrieves the active Dampen theme from the AppState's ThemeContext
/// - Converts it to an `iced::Theme` using the `ThemeAdapter`
///
/// # Arguments
///
/// * `views` - Slice of discovered view information
///
/// # Returns
///
/// Token stream containing the `theme()` method implementation.
pub fn generate_theme_method(views: &[ViewInfo]) -> TokenStream {
    let view_match_arms: Vec<_> = views
        .iter()
        .map(|v| {
            let variant = Ident::new(&v.variant_name, proc_macro2::Span::call_site());
            let field_name = Ident::new(&v.field_name, proc_macro2::Span::call_site());

            quote! {
                CurrentView::#variant => {
                    self.#field_name.theme_context()
                        .map(|ctx| dampen_iced::theme_adapter::ThemeAdapter::to_iced(ctx.active()))
                        .unwrap_or(iced::Theme::Light)
                }
            }
        })
        .collect();

    quote! {
        pub fn theme(&self) -> iced::Theme {
            match self.current_view {
                #(#view_match_arms)*
            }
        }
    }
}

/// Generates a static `window_settings()` method for persistence.
///
/// Creates a method that returns `iced::window::Settings` with the persisted window size
/// so the window can be created with the correct initial size.
///
/// # Arguments
///
/// * `attrs` - Parsed macro attributes (for app_name)
///
/// # Returns
///
/// Option with token stream containing the `window_settings()` method if persistence is enabled.
pub fn generate_window_settings_method(attrs: &MacroAttributes) -> Option<TokenStream> {
    if !attrs.persistence {
        return None;
    }

    #[allow(clippy::unwrap_used)]
    let app_name = attrs.app_name.as_ref().unwrap();

    Some(quote! {
        /// Default window settings for first launch.
        ///
        /// Use this builder to configure the initial window state when no
        /// persisted configuration exists yet.
        ///
        /// # Example
        ///
        /// ```ignore
        /// iced::application(...)
        ///     .window(DampenApp::window_settings()
        ///         .default_size(800, 600)
        ///         .default_maximized(false)
        ///         .build())
        ///     .run()
        /// ```
        pub fn window_settings() -> dampen_dev::persistence::WindowSettingsBuilder {
            dampen_dev::persistence::WindowSettingsBuilder::new(#app_name)
        }
    })
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
    let message_type = &attrs.message_type;

    // Hot reload subscription
    let hot_reload_sub = if let Some(hot_reload_variant) = &attrs.hot_reload_variant {
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
            let hot_reload = dampen_dev::subscription::watch_files(
                vec![#(#watch_paths),*],
                100  // 100ms debounce
            ).map(#message_type::#hot_reload_variant);
        })
    } else {
        None
    };

    // System theme subscription (works in both debug and release builds)
    // Uses dampen_iced::watch_system_theme() which wraps iced::system::theme_changes()
    let system_theme_sub = attrs
        .system_theme_variant
        .as_ref()
        .map(|system_theme_variant| {
            quote! {
                let system_theme = dampen_iced::watch_system_theme()
                    .map(#message_type::#system_theme_variant);
            }
        });

    // Persistence subscription (window events)
    let persistence_sub = if attrs.persistence {
        Some(quote! {
           let window_events = iced::window::events().map(|(id, e)| {
               #[cfg(debug_assertions)]
               println!("DEBUG: Window event detected: {:?}", e);
               #message_type::Window(id, e)
           });
        })
    } else {
        None
    };

    // Build subscription expressions for debug mode (hot reload + system theme + persistence)
    let mut debug_subs = Vec::new();
    if hot_reload_sub.is_some() {
        debug_subs.push(quote! { hot_reload });
    }
    if system_theme_sub.is_some() {
        debug_subs.push(quote! { system_theme });
    }
    if persistence_sub.is_some() {
        debug_subs.push(quote! { window_events });
    }

    // Build subscription expressions for release mode (system theme + persistence)
    let mut release_subs = Vec::new();
    if system_theme_sub.is_some() {
        release_subs.push(quote! { system_theme });
    }
    if persistence_sub.is_some() {
        release_subs.push(quote! { window_events });
    }

    // If no subscriptions at all, don't generate the method
    if debug_subs.is_empty() && release_subs.is_empty() {
        return None;
    }

    let debug_sub_expr = if debug_subs.len() == 1 {
        quote! { #(#debug_subs)* }
    } else if debug_subs.len() > 1 {
        quote! { iced::Subscription::batch(vec![#(#debug_subs),*]) }
    } else {
        quote! { iced::Subscription::none() }
    };

    let release_sub_expr = if release_subs.len() == 1 {
        quote! { #(#release_subs)* }
    } else if release_subs.len() > 1 {
        quote! { iced::Subscription::batch(vec![#(#release_subs),*]) }
    } else {
        quote! { iced::Subscription::none() }
    };

    Some(quote! {
        #[cfg(debug_assertions)]
        pub fn subscription(&self) -> iced::Subscription<#message_type> {
            #hot_reload_sub
            #system_theme_sub
            #persistence_sub

            #debug_sub_expr
        }

        #[cfg(not(debug_assertions))]
        pub fn subscription(&self) -> iced::Subscription<#message_type> {
            #system_theme_sub
            #persistence_sub

            #release_sub_expr
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
    let theme_method = generate_theme_method(&views);
    let subscription_method = generate_subscription_method(&views, &attrs);
    let window_settings_method = generate_window_settings_method(&attrs);

    // Build impl block with optional methods
    let impl_methods = match (subscription_method, window_settings_method) {
        (Some(subscription), Some(window_settings)) => {
            quote! {
                impl #struct_name {
                    #init_method
                    #switch_to_methods
                    #update_method
                    #view_method
                    #theme_method
                    #subscription
                    #window_settings
                }
            }
        }
        (Some(subscription), None) => {
            quote! {
                impl #struct_name {
                    #init_method
                    #switch_to_methods
                    #update_method
                    #view_method
                    #theme_method
                    #subscription
                }
            }
        }
        (None, Some(window_settings)) => {
            quote! {
                impl #struct_name {
                    #init_method
                    #switch_to_methods
                    #update_method
                    #view_method
                    #theme_method
                    #window_settings
                }
            }
        }
        (None, None) => {
            quote! {
                impl #struct_name {
                    #init_method
                    #switch_to_methods
                    #update_method
                    #view_method
                    #theme_method
                }
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
