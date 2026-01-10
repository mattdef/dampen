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
pub mod update;
pub mod view;
pub mod bindings;
pub mod handlers;

use crate::DampenDocument;
use crate::HandlerSignature;
use proc_macro2::TokenStream;
use quote::quote;

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

    // Generate Message enum from handlers
    let message_enum = generate_message_enum(handlers);

    // Generate view function
    let view_fn = view::generate_view(document, model_name, message_name)?;

    // Generate update function
    let update_fn = update::generate_update(document, handlers, model_name, message_name)?;

    // Generate Application trait implementation
    let app_impl = application::generate_application_trait(model_name, message_name)?;

    // Combine all generated code
    let combined = quote! {
        #message_enum

        #app_impl

        #view_fn

        #update_fn
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
            let variant_name = h.name.to_string();
            let ident = syn::Ident::new(&variant_name, proc_macro2::Span::call_site());

            if let Some(param_type) = &h.param_type {
                // Handler with value parameter
                let type_ident = syn::Ident::new(param_type, proc_macro2::Span::call_site());
                quote! { #ident(#type_ident) }
            } else if h.returns_command {
                // Handler returning command
                quote! { #ident }
            } else {
                // Simple handler
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

        assert!(code.contains("increment"));
        assert!(code.contains("update_value"));
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
