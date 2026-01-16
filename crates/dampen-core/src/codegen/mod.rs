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

    /// fn(&mut Model) -> Command<Message>
    /// Handler that returns a command for side effects
    WithCommand,
}

/// Metadata structure emitted by #[ui_handler] macro for build-time registration
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

    let update_match_arms = update::generate_update_match_arms(handlers, message_name)?;

    let model_ident = syn::Ident::new(model_name, proc_macro2::Span::call_site());
    let message_ident = syn::Ident::new(message_name, proc_macro2::Span::call_site());

    let combined = quote! {
        use crate::ui::window::{self, #model_ident};
        use iced::{Element, Task, executor};

        #message_enum

        pub fn new_model() -> (#model_ident, Task<#message_ident>) {
            (#model_ident::default(), Task::none())
        }

        pub fn update_model(model: &mut #model_ident, message: #message_ident) -> Task<#message_ident> {
            #update_match_arms
        }

        pub fn view_model(model: &#model_ident) -> Element<'_, #message_ident> {
            let count = &model.count;
            #view_fn
        }
    };

    Ok(CodegenOutput {
        code: combined.to_string(),
        warnings,
    })
}

/// Generate Message enum from handler signatures
fn generate_message_enum(handlers: &[HandlerSignature]) -> TokenStream {
    if handlers.is_empty() {
        return quote! {
            #[derive(Clone, Debug)]
            pub enum Message {}
        };
    }

    let variants: Vec<_> = handlers
        .iter()
        .map(|h| {
            // Convert snake_case to UpperCamelCase
            let variant_name = to_upper_camel_case(&h.name);
            let ident = syn::Ident::new(&variant_name, proc_macro2::Span::call_site());

            if let Some(param_type) = &h.param_type {
                let type_ident = syn::Ident::new(param_type, proc_macro2::Span::call_site());
                quote! { #ident(#type_ident) }
            } else {
                quote! { #ident }
            }
        })
        .collect();

    quote! {
        #[derive(Clone, Debug)]
        pub enum Message {
            #(#variants),*
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

/// Optimize constant expressions in generated code
pub fn constant_folding(code: &str) -> String {
    // This is a placeholder for constant folding optimization
    // In a full implementation, this would:
    // - Evaluate constant expressions at compile time
    // - Remove dead code
    // - Inline small functions
    // - Optimize string concatenations

    code.to_string()
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
