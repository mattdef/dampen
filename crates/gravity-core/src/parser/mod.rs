pub mod error;
pub mod lexer;

use crate::ir::span::Span;
use crate::ir::GravityDocument;

/// Parse XML into a GravityDocument
pub fn parse(_xml: &str) -> Result<GravityDocument, ParseError> {
    unimplemented!("Parser not yet implemented")
}

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
