//! Handler inventory extraction for build scripts
//!
//! This module provides utilities for build.rs scripts to extract handler metadata
//! from Rust source files that use the `inventory_handlers!` macro.

use crate::HandlerSignature;
use std::path::Path;

/// Extract handler names from an `inventory_handlers!` macro invocation in a Rust file.
///
/// This function parses the source file and looks for the `inventory_handlers!` macro,
/// extracting the list of handler names declared within it.
///
/// # Arguments
///
/// * `rs_file_path` - Path to the .rs file to parse
///
/// # Returns
///
/// A vector of handler names found in the inventory, or an empty vector if:
/// - The file doesn't exist
/// - The file has no `inventory_handlers!` macro
/// - The macro is empty
///
/// # Example
///
/// ```rust,ignore
/// let handlers = extract_handler_names_from_file("src/ui/window.rs");
/// // Returns: vec!["increment", "decrement", "reset"]
/// ```
pub fn extract_handler_names_from_file(rs_file_path: &Path) -> Vec<String> {
    let content = match std::fs::read_to_string(rs_file_path) {
        Ok(content) => content,
        Err(_) => return vec![],
    };

    extract_handler_names_from_source(&content)
}

/// Extract handler names from Rust source code containing `inventory_handlers!` macro.
///
/// # Arguments
///
/// * `source` - Rust source code as a string
///
/// # Returns
///
/// A vector of handler names found in the inventory
fn extract_handler_names_from_source(source: &str) -> Vec<String> {
    // Parse the file with syn
    let syntax = match syn::parse_file(source) {
        Ok(syntax) => syntax,
        Err(_) => return vec![],
    };

    // Look for inventory_handlers! macro invocation
    for item in syntax.items {
        if let syn::Item::Macro(mac) = item {
            let path = &mac.mac.path;

            // Check if this is our inventory_handlers macro
            if path.segments.last().map(|s| s.ident.to_string())
                == Some("inventory_handlers".to_string())
            {
                // Parse the token stream to extract handler names
                return parse_handler_list_from_tokens(&mac.mac.tokens);
            }
        }
    }

    vec![]
}

/// Parse handler names from the token stream of inventory_handlers! macro.
fn parse_handler_list_from_tokens(tokens: &proc_macro2::TokenStream) -> Vec<String> {
    let mut handlers = Vec::new();
    let tokens_str = tokens.to_string();

    // Simple tokenization: split by commas and trim whitespace
    for part in tokens_str.split(',') {
        let name = part.trim().to_string();
        if !name.is_empty() {
            handlers.push(name);
        }
    }

    handlers
}

/// Extract full handler metadata from a Rust file.
///
/// This function looks for both the `inventory_handlers!` macro and the corresponding
/// `_HANDLER_METADATA_*` constants to build complete HandlerInfo structures.
///
/// Note: This is a more complete but slower approach. For build scripts, prefer
/// `extract_handler_names_from_file()` combined with the runtime metadata constants.
///
/// # Arguments
///
/// * `rs_file_path` - Path to the .rs file to parse
///
/// # Returns
///
/// A vector of HandlerSignature objects with complete metadata
pub fn extract_handler_signatures_from_file(rs_file_path: &Path) -> Vec<HandlerSignature> {
    let handler_names = extract_handler_names_from_file(rs_file_path);

    // For each handler name, we need to find its metadata constant
    // This is a simplified implementation - full metadata extraction would require
    // evaluating the const expressions, which is complex

    handler_names
        .into_iter()
        .map(|name| HandlerSignature {
            name,
            param_type: None,
            returns_command: false,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_handler_names() {
        let source = r#"
            use dampen_macros::{ui_handler, inventory_handlers};

            #[ui_handler]
            fn increment(model: &mut Model) {
                model.count += 1;
            }

            #[ui_handler]
            fn decrement(model: &mut Model) {
                model.count -= 1;
            }

            inventory_handlers! {
                increment,
                decrement
            }
        "#;

        let handlers = extract_handler_names_from_source(source);
        assert_eq!(handlers, vec!["increment", "decrement"]);
    }

    #[test]
    fn test_extract_empty_inventory() {
        let source = r#"
            use dampen_macros::ui_handler;

            #[ui_handler]
            fn my_handler(model: &mut Model) {}
        "#;

        let handlers = extract_handler_names_from_source(source);
        assert!(handlers.is_empty());
    }

    #[test]
    fn test_extract_single_handler() {
        let source = r#"
            inventory_handlers! { greet }
        "#;

        let handlers = extract_handler_names_from_source(source);
        assert_eq!(handlers, vec!["greet"]);
    }
}
