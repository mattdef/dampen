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

pub mod analyzer;
pub mod capabilities;
pub mod converters;
pub mod document;
pub mod handlers;
pub mod schema_data;

// Re-export main types for convenience
pub use document::{DocumentCache, DocumentState};
