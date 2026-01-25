//! Code generation for production builds
//!
//! This module generates static Rust code from Dampen UI definitions.
//! The generated code has zero runtime overhead compared to hand-written Iced code.
//!
//! # Overview
//!
//! Code generation is used in production mode to eliminate runtime parsing:
//! 1. Parse `.dampen` files at build time
//! 2. Generate Rust code with `generate_application()`
//! 3. Include generated code via `include!(concat!(env!("OUT_DIR"), "/ui_generated.rs"))`
//!
//! # Example
//!
//! ```rust,ignore
//! // At build time:
//! let doc = parse(ui_xml)?;
//! let output = generate_application(&doc, "Model", "Message", &handlers)?;
//! fs::write("ui_generated.rs", output.code)?;
//!
//! // At runtime:
//! include!("ui_generated.rs");
//! // No XML parsing, no runtime overhead!
//! ```
//!
//! # Generated Code Structure
//!
//! The generated code includes:
//! - Message enum from handler signatures
//! - `impl Application for Model` with `view()` and `update()` methods
//! - Inlined widget tree with evaluated bindings

pub mod application;
pub mod bindings;
pub mod config;
pub mod handlers;
pub mod inventory;
pub mod status_mapping;
pub mod subscription;
pub mod theme;
pub mod update;
pub mod view;

use crate::DampenDocument;
use crate::HandlerSignature;
use proc_macro2::TokenStream;
use quote::quote;
use std::path::PathBuf;
use std::time::SystemTime;

/// Handler signature classification for code generation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HandlerSignatureType {
    /// fn(&mut Model) -> ()
    /// Simple handler with no additional parameters
    Simple,

    /// fn(&mut Model, T) -> ()
    /// Handler that receives a value from the UI element
    WithValue,

    /// fn(&mut Model) -> `Command<Message>`
    /// Handler that returns a command for side effects
    WithCommand,
}

/// Metadata structure emitted by `#[ui_handler]` macro for build-time registration
#[derive(Debug, Clone)]
pub struct HandlerInfo {
    /// Unique handler name referenced in XML
    pub name: &'static str,

    /// Handler signature classification
    pub signature_type: HandlerSignatureType,

    /// Parameter type names for validation
    pub param_types: &'static [&'static str],

    /// Return type name
    pub return_type: &'static str,

    /// Source file location for error messages
    pub source_file: &'static str,

    /// Source line number
    pub source_line: u32,
}

impl HandlerInfo {
    /// Convert HandlerInfo to HandlerSignature for code generation.
    ///
    /// This bridges the metadata emitted by `#[ui_handler]` with the signature
    /// format expected by the code generator.
    ///
    /// # Returns
    ///
    /// A HandlerSignature suitable for use with generate_application()
    pub fn to_signature(&self) -> HandlerSignature {
        // Determine param_type based on signature type
        let param_type = match self.signature_type {
            HandlerSignatureType::Simple => None,
            HandlerSignatureType::WithValue => {
                // Extract the second parameter type (first is &mut Model)
                if self.param_types.len() > 1 {
                    Some(self.param_types[1].to_string())
                } else {
                    None
                }
            }
            HandlerSignatureType::WithCommand => None,
        };

        let returns_command = matches!(self.signature_type, HandlerSignatureType::WithCommand);

        HandlerSignature {
            name: self.name.to_string(),
            param_type,
            returns_command,
        }
    }
}

/// Result of code generation
///
/// Contains the generated Rust code and any warnings or notes.
#[derive(Debug)]
pub struct CodegenOutput {
    /// Generated Rust code
    pub code: String,
    /// Any warnings or notes
    pub warnings: Vec<String>,
}

/// Output structure from code generation
#[derive(Debug, Clone)]
pub struct GeneratedApplication {
    /// Generated Rust code as string
    pub code: String,

    /// List of handler names discovered
    pub handlers: Vec<String>,

    /// List of widget types used
    pub widgets: Vec<String>,

    /// Any warnings generated during code gen
    pub warnings: Vec<String>,
}

/// Container for generated Rust code with metadata
///
/// This structure holds generated code along with metadata about the source
/// file and validation status. It provides methods for validating and formatting
/// the generated code.
#[derive(Debug, Clone)]
pub struct GeneratedCode {
    /// Generated Rust source code
    pub code: String,

    /// Module path (e.g., "ui_window")
    pub module_name: String,

    /// Source .dampen file path
    pub source_file: PathBuf,

    /// Generated at timestamp
    pub timestamp: SystemTime,

    /// Validation status
    pub validated: bool,
}

impl GeneratedCode {
    /// Create a new GeneratedCode instance
    ///
    /// # Arguments
    /// * `code` - The generated Rust source code
    /// * `module_name` - Module name (e.g., "ui_window")
    /// * `source_file` - Path to the source .dampen file
    ///
    /// # Returns
    /// A new GeneratedCode instance with validated set to false
    pub fn new(code: String, module_name: String, source_file: PathBuf) -> Self {
        Self {
            code,
            module_name,
            source_file,
            timestamp: SystemTime::now(),
            validated: false,
        }
    }

    /// Validate syntax by parsing with syn
    ///
    /// # Returns
    /// Ok(()) if the code is valid Rust, Err with message otherwise
    pub fn validate(&mut self) -> Result<(), String> {
        match syn::parse_file(&self.code) {
            Ok(_) => {
                self.validated = true;
                Ok(())
            }
            Err(e) => Err(format!("Syntax validation failed: {}", e)),
        }
    }

    /// Format code with prettyplease
    ///
    /// # Returns
    /// Ok(()) if formatting succeeded, Err with message otherwise
    pub fn format(&mut self) -> Result<(), String> {
        // First parse the code
        match syn::parse_file(&self.code) {
            Ok(syntax_tree) => {
                // Format using prettyplease
                self.code = prettyplease::unparse(&syntax_tree);
                Ok(())
            }
            Err(e) => Err(format!("Failed to parse code for formatting: {}", e)),
        }
    }

    /// Write to output directory
    ///
    /// # Arguments
    /// * `path` - Path to write the generated code to
    ///
    /// # Returns
    /// Ok(()) if write succeeded, Err with IO error otherwise
    pub fn write_to_file(&self, path: &std::path::Path) -> std::io::Result<()> {
        std::fs::write(path, &self.code)
    }
}

/// Generate complete application code from a Dampen document
///
/// This is the main entry point for code generation. It orchestrates
/// the generation of all necessary components.
///
/// # Arguments
///
/// * `document` - Parsed Dampen document
/// * `model_name` - Name of the model struct (e.g., "Model")
/// * `message_name` - Name of the message enum (e.g., "Message")
/// * `handlers` - List of handler signatures
///
/// # Returns
///
/// `Ok(CodegenOutput)` with generated code and warnings
///
/// # Errors
///
/// Returns `CodegenError` if:
/// - Handler validation fails
/// - Widget generation fails
pub fn generate_application(
    document: &DampenDocument,
    model_name: &str,
    message_name: &str,
    handlers: &[HandlerSignature],
) -> Result<CodegenOutput, CodegenError> {
    let warnings = Vec::new();

    let message_enum = generate_message_enum(handlers);

    let view_fn = view::generate_view(document, model_name, message_name)?;

    let update_arms = update::generate_arms(handlers, message_name)?;

    let model_ident = syn::Ident::new(model_name, proc_macro2::Span::call_site());
    let message_ident = syn::Ident::new(message_name, proc_macro2::Span::call_site());

    let combined = quote! {
        use iced::{Element, Task};
        use crate::ui::window::*;

        #message_enum

        pub fn new_model() -> (#model_ident, Task<#message_ident>) {
            (#model_ident::default(), Task::none())
        }

        pub fn update_model(model: &mut #model_ident, message: #message_ident) -> Task<#message_ident> {
            match message {
                #update_arms
            }
        }

        pub fn view_model(model: &#model_ident) -> Element<'_, #message_ident> {
            #view_fn
        }
    };

    Ok(CodegenOutput {
        code: combined.to_string(),
        warnings,
    })
}

use crate::ir::theme::ThemeDocument;
pub use config::PersistenceConfig;

/// Generate complete application code with theme and subscription support
///
/// This is the full-featured entry point for code generation that includes:
/// - Message enum with system theme variant (if follow_system is enabled)
/// - Subscription function for system theme detection
/// - Theme code generation
/// - View and update functions
///
/// # Arguments
///
/// * `document` - Parsed Dampen document
/// * `model_name` - Name of the model struct (e.g., "Model")
/// * `message_name` - Name of the message enum (e.g., "Message")
/// * `handlers` - List of handler signatures
/// * `theme_document` - Optional theme document for theme code generation
///
/// # Returns
///
/// `Ok(CodegenOutput)` with generated code and warnings
pub fn generate_application_with_theme_and_subscriptions(
    document: &DampenDocument,
    model_name: &str,
    message_name: &str,
    handlers: &[HandlerSignature],
    theme_document: Option<&ThemeDocument>,
) -> Result<CodegenOutput, CodegenError> {
    let warnings = Vec::new();

    // Create subscription config from theme document
    let sub_config =
        subscription::SubscriptionConfig::from_theme_document(theme_document, message_name);

    // Generate message enum with system theme variant if needed
    let message_enum = generate_message_enum_with_subscription(handlers, Some(&sub_config));

    let view_fn = view::generate_view(document, model_name, message_name)?;

    let update_arms = update::generate_arms(handlers, message_name)?;

    // Generate system theme update arm if needed
    let system_theme_arm = subscription::generate_system_theme_update_arm(&sub_config);

    let model_ident = syn::Ident::new(model_name, proc_macro2::Span::call_site());
    let message_ident = syn::Ident::new(message_name, proc_macro2::Span::call_site());

    // Generate theme code if theme document is provided
    let theme_code = if let Some(theme_doc) = theme_document {
        match theme::generate_theme_code(theme_doc, &document.style_classes, "app") {
            Ok(generated) => {
                let code_str = generated.code;
                syn::parse_str::<TokenStream>(&code_str).unwrap_or_default()
            }
            Err(e) => {
                return Err(CodegenError::ThemeError(e));
            }
        }
    } else {
        TokenStream::new()
    };

    // Generate subscription function
    let subscription_fn = subscription::generate_subscription_function(&sub_config);

    let has_theme = theme_document.is_some();
    let theme_method = if has_theme {
        quote! {
            pub fn theme(_model: &#model_ident) -> iced::Theme {
                app_theme()
            }
        }
    } else {
        quote! {}
    };

    let combined = quote! {
        use iced::{Element, Task, Theme};
        use crate::ui::window::*;
        use std::collections::HashMap;
        use dampen_core::handler::CanvasEvent;

        #theme_code


        #message_enum

        pub fn new_model() -> (#model_ident, Task<#message_ident>) {
            (#model_ident::default(), Task::none())
        }

        pub fn update_model(model: &mut #model_ident, message: #message_ident) -> Task<#message_ident> {
            match message {
                #update_arms
                #system_theme_arm
            }
        }

        pub fn view_model(model: &#model_ident) -> Element<'_, #message_ident> {
            #view_fn
        }

        #theme_method

        #subscription_fn
    };

    Ok(CodegenOutput {
        code: combined.to_string(),
        warnings,
    })
}

/// Generate complete application code with theme, subscription, and persistence support
///
/// This is the most complete entry point for code generation that includes:
/// - Message enum with system theme variant (if follow_system is enabled)
/// - Subscription function for system theme detection
/// - Theme code generation
/// - View and update functions
/// - Window settings function for persistence (if persistence config is provided)
/// - Full window event handling for persistence (tracking size, position, save on close)
///
/// # Arguments
///
/// * `document` - Parsed Dampen document
/// * `model_name` - Name of the model struct (e.g., "Model")
/// * `message_name` - Name of the message enum (e.g., "Message")
/// * `handlers` - List of handler signatures
/// * `theme_document` - Optional theme document for theme code generation
/// * `persistence` - Optional persistence configuration for window state
///
/// # Returns
///
/// `Ok(CodegenOutput)` with generated code and warnings
pub fn generate_application_full(
    document: &DampenDocument,
    model_name: &str,
    message_name: &str,
    handlers: &[HandlerSignature],
    theme_document: Option<&ThemeDocument>,
    persistence: Option<&PersistenceConfig>,
) -> Result<CodegenOutput, CodegenError> {
    let warnings = Vec::new();

    // Create subscription config from theme document
    let sub_config =
        subscription::SubscriptionConfig::from_theme_document(theme_document, message_name);

    // Determine if we need window events (for persistence)
    let has_persistence = persistence.is_some();

    // Generate message enum with system theme variant and window events if needed
    let message_enum = generate_message_enum_full(handlers, Some(&sub_config), has_persistence);

    let view_fn = view::generate_view(document, model_name, message_name)?;

    let update_arms = update::generate_arms(handlers, message_name)?;

    // Generate system theme update arm if needed
    let system_theme_arm = subscription::generate_system_theme_update_arm(&sub_config);

    let model_ident = syn::Ident::new(model_name, proc_macro2::Span::call_site());
    let message_ident = syn::Ident::new(message_name, proc_macro2::Span::call_site());

    // Generate theme code if theme document is provided
    let theme_code = if let Some(theme_doc) = theme_document {
        match theme::generate_theme_code(theme_doc, &document.style_classes, "app") {
            Ok(generated) => {
                let code_str = generated.code;
                syn::parse_str::<TokenStream>(&code_str).unwrap_or_default()
            }
            Err(e) => {
                return Err(CodegenError::ThemeError(e));
            }
        }
    } else {
        TokenStream::new()
    };

    let has_theme = theme_document.is_some();
    let theme_method = if has_theme {
        quote! {
            pub fn theme(_model: &AppModel) -> iced::Theme {
                app_theme()
            }
        }
    } else {
        quote! {}
    };

    // Generate persistence-related code
    let (
        wrapper_struct,
        new_model_fn,
        update_model_fn,
        view_model_fn,
        subscription_fn,
        window_settings_fn,
    ) = if let Some(config) = persistence {
        let app_name = &config.app_name;

        // Generate wrapper struct that includes persisted_window_state
        let wrapper = quote! {
            /// Application model wrapper with persistence state
            pub struct AppModel {
                /// User's model
                pub inner: #model_ident,
                /// Persisted window state for saving on close
                persisted_window_state: dampen_dev::persistence::WindowState,
            }
        };

        // Generate new_model that initializes the wrapper
        let new_model = quote! {
            pub fn new_model() -> (AppModel, Task<#message_ident>) {
                let persisted_state = dampen_dev::persistence::load_or_default(#app_name, 800, 600);
                (
                    AppModel {
                        inner: #model_ident::default(),
                        persisted_window_state: persisted_state,
                    },
                    Task::none(),
                )
            }
        };

        // Generate update arms that access model.inner for user handlers
        let update_arms_inner = generate_update_arms_for_inner(handlers, message_name)?;

        // Generate update_model with window event handling
        let update_model = quote! {
            pub fn update_model(model: &mut AppModel, message: #message_ident) -> Task<#message_ident> {
                match message {
                    #update_arms_inner
                    #system_theme_arm
                    #message_ident::Window(id, event) => {
                        match event {
                            iced::window::Event::Opened { .. } => {
                                // Window is already created with correct size via window_settings().
                                // Only need to maximize if that was the saved state.
                                if model.persisted_window_state.maximized {
                                    iced::window::maximize(id, true)
                                } else {
                                    Task::none()
                                }
                            }
                            iced::window::Event::Resized(size) => {
                                // Update persisted state with new size
                                model.persisted_window_state.width = size.width as u32;
                                model.persisted_window_state.height = size.height as u32;
                                Task::none()
                            }
                            iced::window::Event::Moved(position) => {
                                model.persisted_window_state.x = Some(position.x as i32);
                                model.persisted_window_state.y = Some(position.y as i32);
                                Task::none()
                            }
                            iced::window::Event::CloseRequested => {
                                let _ = dampen_dev::persistence::save_window_state(
                                    #app_name,
                                    &model.persisted_window_state,
                                );
                                iced::window::close(id)
                            }
                            _ => Task::none(),
                        }
                    }
                }
            }
        };

        // Generate view_model that accesses inner model
        // Note: view_fn uses `model` as the variable name, so we shadow it with the inner model
        let view_model = quote! {
            pub fn view_model(app_model: &AppModel) -> Element<'_, #message_ident> {
                let model = &app_model.inner;
                #view_fn
            }
        };

        // Generate subscription with window events
        let subscription = if sub_config.system_theme_variant.is_some() {
            let variant_name = sub_config
                .system_theme_variant
                .as_ref()
                .map(|v| syn::Ident::new(v, proc_macro2::Span::call_site()));
            quote! {
                pub fn subscription_model(_model: &AppModel) -> iced::Subscription<#message_ident> {
                    let window_events = iced::window::events()
                        .map(|(id, e)| #message_ident::Window(id, e));

                    if app_follows_system() {
                        let system_theme = dampen_iced::watch_system_theme()
                            .map(#message_ident::#variant_name);
                        iced::Subscription::batch(vec![window_events, system_theme])
                    } else {
                        window_events
                    }
                }
            }
        } else {
            quote! {
                pub fn subscription_model(_model: &AppModel) -> iced::Subscription<#message_ident> {
                    iced::window::events()
                        .map(|(id, e)| #message_ident::Window(id, e))
                }
            }
        };

        // Generate window_settings function
        let window_settings = quote! {
            /// Returns a WindowSettingsBuilder for configuring the initial window state.
            ///
            /// This function loads persisted window state if available, or uses defaults
            /// for first launch.
            ///
            /// # Example
            ///
            /// ```ignore
            /// iced::application(...)
            ///     .window(window::window_settings()
            ///         .default_size(800, 600)
            ///         .min_size(400, 300)
            ///         .build())
            ///     .run()
            /// ```
            pub fn window_settings() -> dampen_dev::persistence::WindowSettingsBuilder {
                dampen_dev::persistence::WindowSettingsBuilder::new(#app_name)
            }
        };

        (
            wrapper,
            new_model,
            update_model,
            view_model,
            subscription,
            window_settings,
        )
    } else {
        // No persistence - generate simpler code without wrapper
        let wrapper = TokenStream::new();

        let new_model = quote! {
            pub fn new_model() -> (#model_ident, Task<#message_ident>) {
                (#model_ident::default(), Task::none())
            }
        };

        let update_model = quote! {
            pub fn update_model(model: &mut #model_ident, message: #message_ident) -> Task<#message_ident> {
                match message {
                    #update_arms
                    #system_theme_arm
                }
            }
        };

        let view_model = quote! {
            pub fn view_model(model: &#model_ident) -> Element<'_, #message_ident> {
                #view_fn
            }
        };

        let subscription = subscription::generate_subscription_function(&sub_config);

        let window_settings = TokenStream::new();

        (
            wrapper,
            new_model,
            update_model,
            view_model,
            subscription,
            window_settings,
        )
    };

    let combined = quote! {
        use iced::{Element, Task, Theme};
        use crate::ui::window::*;
        use std::collections::HashMap;
        use dampen_core::handler::CanvasEvent;

        #theme_code


        #message_enum

        #wrapper_struct

        #new_model_fn

        #update_model_fn

        #view_model_fn

        #theme_method

        #subscription_fn

        #window_settings_fn
    };

    Ok(CodegenOutput {
        code: combined.to_string(),
        warnings,
    })
}

/// Generate Message enum from handler signatures
fn generate_message_enum(handlers: &[HandlerSignature]) -> TokenStream {
    generate_message_enum_with_subscription(handlers, None)
}

/// Generate Message enum from handler signatures with optional subscription config
fn generate_message_enum_with_subscription(
    handlers: &[HandlerSignature],
    sub_config: Option<&subscription::SubscriptionConfig>,
) -> TokenStream {
    generate_message_enum_full(handlers, sub_config, false)
}

/// Generate Message enum with all optional variants
fn generate_message_enum_full(
    handlers: &[HandlerSignature],
    sub_config: Option<&subscription::SubscriptionConfig>,
    include_window_events: bool,
) -> TokenStream {
    let handler_variants: Vec<_> = handlers
        .iter()
        .map(|h| {
            // Convert snake_case to UpperCamelCase
            let variant_name = to_upper_camel_case(&h.name);
            let ident = syn::Ident::new(&variant_name, proc_macro2::Span::call_site());

            if let Some(param_type) = &h.param_type {
                // If the parameter type is "&str", use "String" for the enum variant
                // because enum variants cannot hold references without lifetimes
                let effective_type = if param_type == "&str" {
                    "String"
                } else {
                    param_type
                };
                let type_path: syn::Type =
                    syn::parse_str(effective_type).expect("Failed to parse parameter type");
                quote! { #ident(#type_path) }
            } else {
                quote! { #ident }
            }
        })
        .collect();

    // Add system theme variant if configured
    let system_theme_variant = sub_config.and_then(subscription::generate_system_theme_variant);

    // Add window events variant if persistence is enabled
    let window_variant = if include_window_events {
        Some(quote! {
            /// Window events for persistence
            Window(iced::window::Id, iced::window::Event)
        })
    } else {
        None
    };

    let all_variants: Vec<TokenStream> = handler_variants
        .into_iter()
        .chain(system_theme_variant)
        .chain(window_variant)
        .collect();

    if all_variants.is_empty() {
        return quote! {
            #[derive(Clone, Debug)]
            pub enum Message {}
        };
    }

    quote! {
        #[derive(Clone, Debug)]
        pub enum Message {
            #(#all_variants),*
        }
    }
}

/// Convert snake_case to UpperCamelCase
fn to_upper_camel_case(s: &str) -> String {
    let mut result = String::new();
    let mut capitalize_next = true;
    for c in s.chars() {
        if c == '_' {
            capitalize_next = true;
        } else if capitalize_next {
            result.push(c.to_ascii_uppercase());
            capitalize_next = false;
        } else {
            result.push(c);
        }
    }
    result
}

/// Generate update match arms that access `model.inner` instead of `model`
///
/// This is used when the model is wrapped in an AppModel struct for persistence.
/// The handlers expect the user's model type, not the wrapper.
fn generate_update_arms_for_inner(
    handlers: &[HandlerSignature],
    message_name: &str,
) -> Result<TokenStream, CodegenError> {
    use quote::format_ident;

    let message_ident = format_ident!("{}", message_name);

    let match_arms: Vec<TokenStream> = handlers
        .iter()
        .map(|handler| {
            let handler_name = format_ident!("{}", handler.name);
            let variant_name = to_upper_camel_case(&handler.name);
            let variant_ident = syn::Ident::new(&variant_name, proc_macro2::Span::call_site());

            if let Some(_param_type) = &handler.param_type {
                quote! {
                    #message_ident::#variant_ident(value) => {
                        #handler_name(&mut model.inner, value);
                        iced::Task::none()
                    }
                }
            } else if handler.returns_command {
                quote! {
                    #message_ident::#variant_ident => {
                        #handler_name(&mut model.inner)
                    }
                }
            } else {
                quote! {
                    #message_ident::#variant_ident => {
                        #handler_name(&mut model.inner);
                        iced::Task::none()
                    }
                }
            }
        })
        .collect();

    Ok(quote! {
        #(#match_arms)*
    })
}

/// Basic constant folding optimizations for generated code
///
/// Performs simple optimizations:
/// - Removes consecutive blank lines
/// - Trims trailing whitespace from lines
/// - Normalizes multiple spaces to single space
pub fn constant_folding(code: &str) -> String {
    let mut result = String::with_capacity(code.len());

    for line in code.lines() {
        let trimmed = line.trim_end();
        if !trimmed.is_empty() {
            result.push_str(trimmed);
            result.push('\n');
        }
    }

    result
}

/// Validate that all handlers referenced in the document exist
pub fn validate_handlers(
    document: &DampenDocument,
    available_handlers: &[HandlerSignature],
) -> Result<(), CodegenError> {
    let handler_names: Vec<_> = available_handlers.iter().map(|h| h.name.clone()).collect();

    // Collect all handler references from the document
    fn collect_handlers(node: &crate::WidgetNode, handlers: &mut Vec<String>) {
        for event in &node.events {
            handlers.push(event.handler.clone());
        }
        for child in &node.children {
            collect_handlers(child, handlers);
        }
    }

    let mut referenced_handlers = Vec::new();
    collect_handlers(&document.root, &mut referenced_handlers);

    // Check each referenced handler exists
    for handler in referenced_handlers {
        if !handler_names.contains(&handler) {
            return Err(CodegenError::MissingHandler(handler));
        }
    }

    Ok(())
}

/// Errors that can occur during code generation
#[derive(Debug, thiserror::Error)]
pub enum CodegenError {
    #[error("Handler '{0}' is referenced but not defined")]
    MissingHandler(String),

    #[error("Invalid widget kind for code generation: {0}")]
    InvalidWidget(String),

    #[error("Binding expression error: {0}")]
    BindingError(String),

    #[error("Theme code generation error: {0}")]
    ThemeError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse;

    #[test]
    fn test_message_enum_generation() {
        let handlers = vec![
            HandlerSignature {
                name: "increment".to_string(),
                param_type: None,
                returns_command: false,
            },
            HandlerSignature {
                name: "update_value".to_string(),
                param_type: Some("String".to_string()),
                returns_command: false,
            },
        ];

        let tokens = generate_message_enum(&handlers);
        let code = tokens.to_string();

        assert!(code.contains("Increment"));
        assert!(code.contains("UpdateValue"));
    }

    #[test]
    fn test_handler_validation() {
        let xml = r#"<column><button on_click="increment" /></column>"#;
        let doc = parse(xml).unwrap();

        let handlers = vec![HandlerSignature {
            name: "increment".to_string(),
            param_type: None,
            returns_command: false,
        }];

        assert!(validate_handlers(&doc, &handlers).is_ok());

        // Missing handler should fail
        let handlers_empty: Vec<HandlerSignature> = vec![];
        assert!(validate_handlers(&doc, &handlers_empty).is_err());
    }
}
