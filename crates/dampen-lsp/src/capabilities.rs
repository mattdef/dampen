//! LSP server capabilities.
//!
//! Defines the capabilities advertised by the Dampen LSP server.

use tower_lsp::lsp_types::*;

/// Returns the server capabilities.
///
/// These capabilities are advertised to the LSP client during initialization.
/// The client uses this information to determine which features to enable.
pub fn server_capabilities() -> ServerCapabilities {
    ServerCapabilities {
        text_document_sync: Some(TextDocumentSyncCapability::Options(
            TextDocumentSyncOptions {
                open_close: Some(true),
                change: Some(TextDocumentSyncKind::FULL),
                will_save: None,
                will_save_wait_until: None,
                save: None,
            },
        )),
        completion_provider: Some(CompletionOptions {
            resolve_provider: Some(false),
            trigger_characters: Some(vec![
                "<".to_string(),
                " ".to_string(),
                "=".to_string(),
                "{".to_string(),
            ]),
            all_commit_characters: None,
            completion_item: None,
            work_done_progress_options: WorkDoneProgressOptions::default(),
        }),
        hover_provider: Some(HoverProviderCapability::Simple(true)),
        diagnostic_provider: Some(DiagnosticServerCapabilities::Options(DiagnosticOptions {
            identifier: Some("dampen".to_string()),
            inter_file_dependencies: false,
            workspace_diagnostics: false,
            work_done_progress_options: WorkDoneProgressOptions::default(),
        })),
        ..ServerCapabilities::default()
    }
}
