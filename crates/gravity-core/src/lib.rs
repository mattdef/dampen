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
pub use binding::{BindingValue, UiBindable};
pub use expr::{
    BinaryOp, BinaryOpExpr, BindingError, BindingErrorKind, BindingExpr, ConditionalExpr, Expr,
    FieldAccessExpr, LiteralExpr, MethodCallExpr, UnaryOp, UnaryOpExpr,
};
pub use ir::{
    AttributeValue, EventBinding, EventKind, GravityDocument, SchemaVersion, Span, WidgetKind,
    WidgetNode,
};
pub use parser::{parse, ParseError, ParseErrorKind};
pub use traits::Backend;

/// Version of the Gravity framework
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
