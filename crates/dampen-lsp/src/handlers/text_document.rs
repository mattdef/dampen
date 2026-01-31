//! Text document synchronization handlers.
//!
//! Handles `textDocument/didOpen`, `textDocument/didChange`, and
//! `textDocument/didClose` notifications.

#![allow(dead_code)]

use tower_lsp::lsp_types::*;

/// Applies content changes to a document.
///
/// For V1, we use full document sync. This function handles both
/// incremental and full document changes.
///
/// # Arguments
///
/// * `content` - Current document content
/// * `changes` - List of content changes
///
/// # Returns
///
/// Updated document content
pub fn apply_content_changes(
    content: &str,
    changes: Vec<TextDocumentContentChangeEvent>,
) -> String {
    let mut result = content.to_string();

    for change in changes {
        if let Some(_range) = change.range {
            // Incremental change - would need position mapping
            // For V1, we treat this as full document change
            result = change.text;
        } else {
            // Full document change
            result = change.text;
        }
    }

    result
}
