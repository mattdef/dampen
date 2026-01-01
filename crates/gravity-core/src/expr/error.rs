use crate::ir::span::Span;

/// Error during binding evaluation
#[derive(Debug, Clone, PartialEq)]
pub struct BindingError {
    pub kind: BindingErrorKind,
    pub message: String,
    pub span: Span,
    pub suggestion: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BindingErrorKind {
    /// Field does not exist on Model
    UnknownField,

    /// Type mismatch in expression
    TypeMismatch,

    /// Method does not exist
    UnknownMethod,

    /// Invalid operation
    InvalidOperation,
}

impl std::fmt::Display for BindingError {
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
