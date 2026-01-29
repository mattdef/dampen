//! Dampen Language Server Protocol (LSP) implementation.
//!
//! This crate provides a language server for the Dampen UI framework,
//! offering real-time XML validation, intelligent autocompletion, and
//! contextual hover documentation for `.dampen` files.
//!
//! # Architecture
//!
//! The server is built on the `tower-lsp` framework and uses an actor-style
//! architecture with the following components:
//!
//! - **LspServer**: Main orchestrator handling LSP lifecycle
//! - **DocumentCache**: LRU cache of open documents (50 max)
//! - **Analyzer**: Semantic analysis and position-based queries
//! - **Handlers**: LSP method implementations (textDocument/*)
//!
//! # Usage
//!
//! The server communicates via JSON-RPC over stdio, which is the standard
//! LSP transport mechanism. It is started by the editor/client and runs
//! until the connection is closed.

use std::sync::Arc;

use tokio::sync::RwLock;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};
use tracing::{info, warn};
use url::Url;

mod analyzer;
mod capabilities;
mod converters;
mod document;
mod handlers;
mod schema_data;

use document::{DocumentCache, DocumentState};

/// Main LSP server implementation.
///
/// The `LspServer` struct holds the client connection and document cache,
/// coordinating between LSP requests and the Dampen parsing infrastructure.
pub struct LspServer {
    /// LSP client for sending notifications
    client: Client,
    /// LRU cache of open documents
    document_cache: Arc<RwLock<DocumentCache>>,
}

impl LspServer {
    /// Creates a new LSP server instance.
    ///
    /// # Arguments
    ///
    /// * `client` - The LSP client connection
    pub fn new(client: Client) -> Self {
        Self {
            client,
            document_cache: Arc::new(RwLock::new(DocumentCache::new(50))),
        }
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for LspServer {
    /// Handles LSP initialization.
    ///
    /// Advertises server capabilities to the client.
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        info!("Dampen LSP server initializing");

        Ok(InitializeResult {
            capabilities: capabilities::server_capabilities(),
            ..InitializeResult::default()
        })
    }

    /// Handles server shutdown.
    ///
    /// Clears the document cache and prepares for exit.
    async fn shutdown(&self) -> Result<()> {
        info!("Dampen LSP server shutting down");

        let mut cache = self.document_cache.write().await;
        cache.clear();

        Ok(())
    }

    /// Handles document open notification.
    ///
    /// Parses the document and publishes initial diagnostics.
    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri;
        let content = params.text_document.text;
        let version = params.text_document.version;

        info!("Document opened: {}", uri);

        // Create document state and parse
        let doc_state = DocumentState::new(uri.clone(), content, version);

        // Store in cache
        {
            let mut cache = self.document_cache.write().await;
            cache.insert(uri.clone(), doc_state);
        }

        // Publish diagnostics
        self.publish_diagnostics(&uri).await;
    }

    /// Handles document change notification.
    ///
    /// Updates document content and re-publishes diagnostics.
    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri;
        let version = params.text_document.version;

        info!("Document changed: {} (version {})", uri, version);

        // Get current document
        let mut cache = self.document_cache.write().await;

        if let Some(doc) = cache.get(&uri) {
            // Apply changes (full document sync for V1)
            let mut new_content = doc.content.clone();

            for change in params.content_changes {
                if let Some(range) = change.range {
                    // Incremental change
                    let start_offset = converters::position_to_offset(&new_content, range.start);
                    let end_offset = converters::position_to_offset(&new_content, range.end);

                    if let (Some(start), Some(end)) = (start_offset, end_offset) {
                        new_content.replace_range(start..end, &change.text);
                    }
                } else {
                    // Full document change
                    new_content = change.text;
                }
            }

            // Create updated document state
            let updated_doc = DocumentState::new(uri.clone(), new_content, version);
            cache.insert(uri.clone(), updated_doc);

            // Publish diagnostics
            drop(cache);
            self.publish_diagnostics(&uri).await;
        } else {
            warn!("Change received for unknown document: {}", uri);
        }
    }

    /// Handles document close notification.
    ///
    /// Removes document from cache and clears diagnostics.
    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        let uri = params.text_document.uri;

        info!("Document closed: {}", uri);

        // Remove from cache
        {
            let mut cache = self.document_cache.write().await;
            cache.remove(&uri);
        }

        // Clear diagnostics
        self.client.publish_diagnostics(uri, vec![], None).await;
    }

    /// Handles completion request.
    ///
    /// Provides context-aware autocompletion suggestions.
    async fn completion(&self, params: CompletionParams) -> Result<Option<CompletionResponse>> {
        let uri = params.text_document_position.text_document.uri.clone();
        info!("Completion request for: {}", uri);

        // Use write lock because cache.get() updates LRU recency
        let mut cache = self.document_cache.write().await;
        if let Some(doc) = cache.get(&uri) {
            Ok(handlers::completion::completion(doc, params))
        } else {
            warn!("Completion requested for unknown document: {}", uri);
            Ok(None)
        }
    }

    /// Handles hover request.
    ///
    /// Provides contextual documentation for the element under the cursor.
    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        let uri = params
            .text_document_position_params
            .text_document
            .uri
            .clone();
        let position = params.text_document_position_params.position;

        info!("Hover request for: {} at {:?}", uri, position);

        // Track request timing for performance monitoring
        let start_time = std::time::Instant::now();

        // Use write lock because cache.get() updates LRU recency
        let mut cache = self.document_cache.write().await;
        let result = if let Some(doc) = cache.get(&uri) {
            Ok(handlers::hover::hover(doc, position))
        } else {
            warn!("Hover requested for unknown document: {}", uri);
            Ok(None)
        };

        // Log performance metric
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > 200 {
            warn!(
                "Hover response time exceeded 200ms target: {}ms",
                elapsed.as_millis()
            );
        } else {
            info!("Hover response time: {}ms", elapsed.as_millis());
        }

        result
    }
}

impl LspServer {
    /// Publishes diagnostics for a document.
    ///
    /// Parses the document and converts any errors to LSP diagnostics.
    async fn publish_diagnostics(&self, uri: &Url) {
        let mut cache = self.document_cache.write().await;

        if let Some(doc) = cache.get(&uri.clone()) {
            let diagnostics = handlers::diagnostics::compute_diagnostics(doc);
            let version = Some(doc.version);

            drop(cache);

            self.client
                .publish_diagnostics(uri.clone(), diagnostics, version)
                .await;
        }
    }
}

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt().with_env_filter("info").init();

    info!("Starting Dampen LSP server");

    let (stdin, stdout) = (tokio::io::stdin(), tokio::io::stdout());

    let (service, socket) = LspService::new(LspServer::new);

    Server::new(stdin, stdout, socket).serve(service).await;
}
