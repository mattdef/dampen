//! Gravity Core - Parser, IR, and Traits
//!
//! This crate contains the core types and traits for the Gravity UI framework.
//! It is backend-agnostic and does not depend on any specific UI framework.

// Module declarations
pub mod binding;
pub mod codegen;
pub mod expr;
pub mod handler;
pub mod ir;
pub mod parser;
pub mod traits;

// Public exports
pub use binding::{BindingValue, ToBindingValue, UiBindable};
pub use expr::{
    evaluate_binding_expr, evaluate_expr, evaluate_formatted, BinaryOp, BinaryOpExpr, BindingError,
    BindingErrorKind, BindingExpr, ConditionalExpr, Expr, FieldAccessExpr, LiteralExpr,
    MethodCallExpr, UnaryOp, UnaryOpExpr,
};
pub use handler::{HandlerEntry, HandlerRegistry, HandlerSignature};
pub use ir::{
    AttributeValue, EventBinding, EventKind, GravityDocument, InterpolatedPart, SchemaVersion,
    Span, WidgetKind, WidgetNode,
};
pub use parser::error::{ParseError, ParseErrorKind};
pub use parser::parse;
pub use traits::Backend;

// Code generation exports
pub use codegen::{generate_application, validate_handlers, CodegenError, CodegenOutput};

// Re-export for convenience
pub use expr::tokenize_binding_expr;

/// Version of the Gravity framework
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
