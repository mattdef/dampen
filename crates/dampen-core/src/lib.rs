//! Dampen Core - Parser, IR, and Traits
//!
//! This crate contains the core types and traits for the Dampen UI framework.
//!
//! # Overview
//!
//! Dampen Core provides:
//! - **XML Parser**: Parse `.gravity` files into an Intermediate Representation (IR)
//! - **IR Types**: Structured representation of UI widgets and bindings
//! - **Expression Engine**: Evaluate binding expressions like `{counter}` or `{if x > 0}`
//! - **Handler Registry**: Manage event handlers for UI interactions
//! - **Code Generation**: Generate static Rust code for production builds
//! - **Backend Traits**: Abstract interface for rendering backends
//!
//! # Quick Start
//!
//! ```rust
//! use dampen_core::parse;
//!
//! let xml = r#"<column><text value="Hello!" /></column>"#;
//! let doc = parse(xml).unwrap();
//! println!("Parsed {} widgets", doc.root.children.len());
//! ```
//!
//! # Core Concepts
//!
//! ## Intermediate Representation (IR)
//!
//! The IR bridges XML parsing and backend rendering:
//! - [`DampenDocument`] - Root document structure
//! - [`WidgetNode`] - Individual UI widgets
//! - [`WidgetKind`] - Types of widgets (button, text, etc.)
//! - [`AttributeValue`] - Static or bound attribute values
//!
//! ## Binding Expressions
//!
//! Expressions in `{braces}` are parsed into [`Expr`] AST nodes:
//! - Field access: `{user.name}`
//! - Method calls: `{items.len()}`
//! - Conditionals: `{if active then 'yes' else 'no'}`
//! - Binary operations: `{count > 0}`
//!
//! ## Event Handlers
//!
//! Handlers connect UI events to Rust functions:
//! - [`HandlerRegistry`] - Runtime handler storage
//! - [`HandlerSignature`] - Compile-time handler metadata
//! - [`HandlerEntry`] - Different handler types (simple, with value, with command)
//!
//! # Architecture
//!
//! This crate is **backend-agnostic**. It defines traits and types but doesn't
//! implement any specific UI framework. See `dampen-iced` for the Iced backend.
//!
//! # Features
//!
//! - **Zero-copy parsing** with `roxmltree`
//! - **Type-safe** expression evaluation
//! - **Error recovery** with span information
//! - **Code generation** for production builds
//!
//! # Re-exports
//!
//! This crate re-exports all public types from its submodules for convenience.
//! See individual modules for detailed documentation.

// Module declarations
pub mod binding;
pub mod codegen;
pub mod expr;
pub mod handler;
pub mod ir;
pub mod parser;
pub mod state;
pub mod traits;

// Public exports

/// Binding value types and the `UiBindable` trait for data models.
///
/// This module provides the core abstraction for data binding in Dampen.
/// Types implementing `UiBindable` can have their fields accessed from
/// binding expressions in XML.
pub use binding::{BindingValue, ToBindingValue, UiBindable};

/// Expression evaluation and AST types.
///
/// This module handles parsing and evaluating binding expressions like
/// `{counter}`, `{items.len()}`, and `{if x > 0 then 'yes' else 'no'}`.
pub use expr::{
    BinaryOp, BinaryOpExpr, BindingError, BindingErrorKind, BindingExpr, ConditionalExpr, Expr,
    FieldAccessExpr, LiteralExpr, MethodCallExpr, UnaryOp, UnaryOpExpr, evaluate_binding_expr,
    evaluate_expr, evaluate_formatted,
};

/// Event handler management and signatures.
///
/// This module provides the registry for event handlers and signature
/// validation for compile-time checking.
pub use handler::{HandlerEntry, HandlerRegistry, HandlerSignature};

/// Intermediate Representation (IR) types.
///
/// This module contains all types representing the parsed structure of
/// a Dampen UI document, suitable for rendering or code generation.
pub use ir::{
    AttributeValue, DampenDocument, EventBinding, EventKind, InterpolatedPart, SchemaVersion, Span,
    WidgetKind, WidgetNode,
};

/// XML parsing and error types.
///
/// This module provides the parser that converts XML markup into the IR.
pub use parser::error::{ParseError, ParseErrorKind};
pub use parser::parse;

/// Backend abstraction traits.
///
/// This module defines the `Backend` trait that rendering implementations
/// must provide.
pub use traits::Backend;

/// Code generation for production builds.
///
/// This module generates static Rust code from Dampen documents,
/// eliminating runtime parsing overhead.
pub use codegen::{CodegenError, CodegenOutput, generate_application, validate_handlers};

/// Application state container for UI views.
///
/// This module provides the [`AppState`](state::AppState) struct that combines
/// a parsed UI document with application state and event handlers.
pub use state::AppState;

/// Tokenize a binding expression for debugging or custom processing.
pub use expr::tokenize_binding_expr;

/// Version of the Dampen framework
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
