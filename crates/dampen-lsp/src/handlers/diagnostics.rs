//! Diagnostic computation and publishing.
//!
//! Converts Dampen parse errors to LSP diagnostics.

use dampen_core::parser::parse;
use tower_lsp::lsp_types::*;

use crate::converters;
use crate::document::DocumentState;

/// Computes diagnostics for a document.
///
/// Parses the document content and converts any errors to LSP diagnostics.
///
/// # Arguments
///
/// * `doc` - The document state to validate
///
/// # Returns
///
/// Vector of LSP diagnostics
pub fn compute_diagnostics(doc: &DocumentState) -> Vec<Diagnostic> {
    // Use existing parse errors if available, otherwise re-parse
    let errors = if doc.parse_errors.is_empty() {
        match parse(&doc.content) {
            Ok(_) => return vec![],
            Err(error) => vec![error],
        }
    } else {
        doc.parse_errors.clone()
    };

    // Convert parse errors to diagnostics
    errors
        .into_iter()
        .map(|err| converters::parse_error_to_diagnostic(&doc.content, err))
        .collect()
}
