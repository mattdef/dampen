//! LSP method handlers.
//!
//! This module contains implementations for LSP protocol methods,
//! organized by category (text document, diagnostics, completion, hover).

pub mod completion;
pub mod diagnostics;
pub mod hover;
pub mod text_document;
