//! #[dampen_app] procedural macro implementation
//!
//! This module contains:
//! - MacroAttributes parsing from attribute syntax
//! - Main macro entry point
//! - Code generation logic for multi-view applications

use syn::Ident;

/// Parsed attributes from #[dampen_app(...)] annotation
#[derive(Debug, Clone)]
pub struct MacroAttributes {
    /// Required: Directory to scan for .dampen files (relative to crate root)
    pub ui_dir: String,

    /// Required: Name of the user's Message enum
    pub message_type: Ident,

    /// Required: Message variant for HandlerMessage dispatch
    pub handler_variant: Ident,

    /// Optional: Message variant for hot-reload file events
    pub hot_reload_variant: Option<Ident>,

    /// Optional: Message variant for error overlay dismissal
    pub dismiss_error_variant: Option<Ident>,

    /// Optional: Glob patterns to exclude from discovery
    pub exclude: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    // Placeholder test to ensure module compiles
    #[test]
    fn test_macro_attributes_structure() {
        // This test will be replaced with actual parsing tests in T010
        assert!(true);
    }
}
