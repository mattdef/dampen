use crate::ir::span::Span;
use proc_macro2::TokenStream;
use quote::quote;

/// Error during parsing
#[derive(Debug, Clone, PartialEq)]
pub struct ParseError {
    pub kind: ParseErrorKind,
    pub message: String,
    pub span: Span,
    pub suggestion: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParseErrorKind {
    XmlSyntax,
    UnknownWidget,
    UnknownAttribute,
    InvalidValue,
    InvalidExpression,
    UnclosedBinding,
    MissingAttribute,
    /// Schema version is newer than the framework supports
    UnsupportedVersion,
    /// Attribute is deprecated in favor of a standardized alternative
    DeprecatedAttribute,
    /// Child widget is not allowed in this context
    InvalidChild,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "error[{}]: {} at line {}, column {}",
            self.kind as u8, self.message, self.span.line, self.span.column
        )?;
        if let Some(suggestion) = &self.suggestion {
            write!(f, "\n  help: {}", suggestion)?;
        }
        Ok(())
    }
}

impl ParseError {
    /// Convert this error into a compile_error! macro invocation.
    ///
    /// This is used by procedural macros to emit compile-time errors
    /// with proper location information and helpful suggestions.
    ///
    /// # Returns
    ///
    /// A `TokenStream` containing a `compile_error!` macro invocation.
    pub fn to_compile_error(&self) -> TokenStream {
        let message = format!(
            "Dampen parsing error: {}\n  at line {}, column {}",
            self.message, self.span.line, self.span.column
        );

        let mut tokens = quote! {
            compile_error!(#message);
        };

        if let Some(ref suggestion) = self.suggestion {
            let help = format!("help: {}", suggestion);
            tokens.extend(quote! {
                compile_error!(#help);
            });
        }

        tokens
    }
}
