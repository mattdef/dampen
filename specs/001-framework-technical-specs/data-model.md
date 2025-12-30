# Data Model: Gravity Framework

**Feature**: 001-framework-technical-specs  
**Date**: 2025-12-30

## Overview

This document defines the core data structures for the Gravity declarative UI framework. These types form the Intermediate Representation (IR) that bridges XML parsing and backend rendering.

---

## Core IR Types

### WidgetNode

The primary structure representing a parsed widget from XML.

```rust
/// A node in the widget tree, representing a single UI element.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WidgetNode {
    /// The widget type (e.g., "button", "text", "column")
    pub kind: WidgetKind,
    
    /// Unique identifier if specified via id="..." attribute
    pub id: Option<String>,
    
    /// Static attributes (non-binding values)
    pub attributes: HashMap<String, AttributeValue>,
    
    /// Event bindings (on_click, on_change, etc.)
    pub events: Vec<EventBinding>,
    
    /// Child widgets for container types
    pub children: Vec<WidgetNode>,
    
    /// Source location for error reporting
    pub span: Span,
}

/// Enumeration of all supported widget types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WidgetKind {
    // Layout widgets
    Column,
    Row,
    Container,
    Scrollable,
    Stack,
    
    // Content widgets
    Text,
    Image,
    Svg,
    
    // Interactive widgets
    Button,
    TextInput,
    Checkbox,
    Slider,
    PickList,
    Toggler,
    
    // Decorative widgets
    Space,
    Rule,
    
    // Custom/unknown (for future extensibility)
    Custom(String),
}
```

### AttributeValue

Represents an attribute that may be static or contain a binding expression.

```rust
/// A value that can be either static or dynamically bound.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AttributeValue {
    /// Static string value
    Static(String),
    
    /// Dynamic binding expression
    Binding(BindingExpr),
    
    /// Mixed content with interpolation: "Hello, {name}!"
    Interpolated(Vec<InterpolatedPart>),
}

/// Part of an interpolated string.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum InterpolatedPart {
    Literal(String),
    Binding(BindingExpr),
}
```

### BindingExpr

The AST for binding expressions like `{counter}` or `{items.len() > 0}`.

```rust
/// A parsed binding expression.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BindingExpr {
    /// The expression AST
    pub expr: Expr,
    
    /// Source span within the attribute value
    pub span: Span,
}

/// Expression AST node.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Expr {
    /// Field access: `counter`, `user.name`
    FieldAccess(FieldAccessExpr),
    
    /// Method call: `items.len()`, `name.to_uppercase()`
    MethodCall(MethodCallExpr),
    
    /// Binary operation: `count > 0`, `a + b`
    BinaryOp(BinaryOpExpr),
    
    /// Unary operation: `!is_valid`, `-offset`
    UnaryOp(UnaryOpExpr),
    
    /// Conditional: `if condition then a else b`
    Conditional(ConditionalExpr),
    
    /// Literal value: `"string"`, `42`, `true`
    Literal(LiteralExpr),
}

/// Field access path.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FieldAccessExpr {
    /// Path segments: ["user", "profile", "name"]
    pub path: Vec<String>,
}

/// Method call with arguments.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MethodCallExpr {
    /// The receiver expression
    pub receiver: Box<Expr>,
    
    /// Method name
    pub method: String,
    
    /// Arguments (may be empty for no-arg methods)
    pub args: Vec<Expr>,
}

/// Binary operation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BinaryOpExpr {
    pub left: Box<Expr>,
    pub op: BinaryOp,
    pub right: Box<Expr>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BinaryOp {
    // Comparison
    Eq,      // ==
    Ne,      // !=
    Lt,      // <
    Le,      // <=
    Gt,      // >
    Ge,      // >=
    
    // Logical
    And,     // &&
    Or,      // ||
    
    // Arithmetic
    Add,     // +
    Sub,     // -
    Mul,     // *
    Div,     // /
}

/// Unary operation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UnaryOpExpr {
    pub op: UnaryOp,
    pub operand: Box<Expr>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UnaryOp {
    Not,     // !
    Neg,     // -
}

/// Conditional expression (ternary-like).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConditionalExpr {
    pub condition: Box<Expr>,
    pub then_branch: Box<Expr>,
    pub else_branch: Box<Expr>,
}

/// Literal value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LiteralExpr {
    String(String),
    Integer(i64),
    Float(f64),
    Bool(bool),
}
```

### EventBinding

Maps XML event attributes to handler references.

```rust
/// An event binding from XML to a Rust handler.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EventBinding {
    /// Event type (click, change, input, submit, etc.)
    pub event: EventKind,
    
    /// Handler name as referenced in XML
    pub handler: String,
    
    /// Source span for error reporting
    pub span: Span,
}

/// Supported event types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EventKind {
    Click,
    Press,
    Release,
    Change,
    Input,
    Submit,
    Select,
    Toggle,
    Scroll,
}
```

### Span

Source location information for error reporting.

```rust
/// Source location in the XML file.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Span {
    /// Byte offset from start of file
    pub start: usize,
    
    /// Byte offset of end (exclusive)
    pub end: usize,
    
    /// Line number (1-based)
    pub line: u32,
    
    /// Column number (1-based)
    pub column: u32,
}

impl Span {
    /// Create a span covering a range
    pub fn new(start: usize, end: usize, line: u32, column: u32) -> Self {
        Self { start, end, line, column }
    }
    
    /// Merge two spans to cover both
    pub fn merge(self, other: Span) -> Span {
        Span {
            start: self.start.min(other.start),
            end: self.end.max(other.end),
            line: self.line.min(other.line),
            column: if self.line < other.line { self.column } else { other.column },
        }
    }
}
```

---

## Document-Level Types

### GravityDocument

The root structure representing a complete parsed `.gravity` file.

```rust
/// A complete parsed Gravity UI document.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GravityDocument {
    /// Schema version (for forward compatibility)
    pub version: SchemaVersion,
    
    /// Root widget (typically a container like Column or Row)
    pub root: WidgetNode,
    
    /// Document-level imports or style references
    pub imports: Vec<Import>,
    
    /// Source file path (for error messages)
    pub source_path: Option<PathBuf>,
}

/// Schema version for compatibility checking.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SchemaVersion {
    pub major: u16,
    pub minor: u16,
}

/// Import declaration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Import {
    /// Import path or URL
    pub path: String,
    
    /// Optional alias
    pub alias: Option<String>,
    
    pub span: Span,
}
```

---

## Handler System Types

### HandlerRegistry

Runtime registry mapping handler names to implementations.

```rust
/// Registry of event handlers.
pub struct HandlerRegistry<Model, Message> {
    handlers: HashMap<String, HandlerEntry<Model, Message>>,
}

/// Entry in the handler registry.
pub enum HandlerEntry<Model, Message> {
    /// Simple handler: fn(&mut Model)
    Simple(Box<dyn Fn(&mut Model) + Send + Sync>),
    
    /// Handler with value: fn(&mut Model, T)
    WithValue(Box<dyn Fn(&mut Model, Box<dyn Any>) + Send + Sync>),
    
    /// Handler returning command: fn(&mut Model) -> Command<Message>
    WithCommand(Box<dyn Fn(&mut Model) -> Command<Message> + Send + Sync>),
}

/// Handler metadata for compile-time validation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HandlerSignature {
    /// Handler name
    pub name: String,
    
    /// Parameter type if applicable
    pub param_type: Option<String>,
    
    /// Whether handler returns Command
    pub returns_command: bool,
}
```

---

## Binding System Types

### UiBindable Trait

Trait implemented by `#[derive(UiModel)]` for binding access.

```rust
/// Trait for types that expose bindable fields.
pub trait UiBindable: Serialize + for<'de> Deserialize<'de> {
    /// Get a field value by path.
    fn get_field(&self, path: &[&str]) -> Option<BindingValue>;
    
    /// List available field paths for error suggestions.
    fn available_fields() -> Vec<String>;
}

/// Value returned from a binding evaluation.
#[derive(Debug, Clone, PartialEq)]
pub enum BindingValue {
    String(String),
    Integer(i64),
    Float(f64),
    Bool(bool),
    List(Vec<BindingValue>),
    None,
}

impl BindingValue {
    /// Convert to display string.
    pub fn to_display_string(&self) -> String {
        match self {
            BindingValue::String(s) => s.clone(),
            BindingValue::Integer(i) => i.to_string(),
            BindingValue::Float(f) => f.to_string(),
            BindingValue::Bool(b) => b.to_string(),
            BindingValue::List(l) => format!("[{} items]", l.len()),
            BindingValue::None => String::new(),
        }
    }
    
    /// Convert to boolean for conditionals.
    pub fn to_bool(&self) -> bool {
        match self {
            BindingValue::Bool(b) => *b,
            BindingValue::String(s) => !s.is_empty(),
            BindingValue::Integer(i) => *i != 0,
            BindingValue::Float(f) => *f != 0.0,
            BindingValue::List(l) => !l.is_empty(),
            BindingValue::None => false,
        }
    }
}
```

---

## Backend Abstraction Types

### Backend Trait

Trait for rendering backends (Iced is the reference implementation).

```rust
/// Backend for rendering IR to a specific UI framework.
pub trait Backend {
    /// The widget type produced by this backend.
    type Widget<'a>;
    
    /// The message type for events.
    type Message: Clone + 'static;
    
    /// Create a text widget.
    fn text<'a>(&self, content: &str) -> Self::Widget<'a>;
    
    /// Create a button widget.
    fn button<'a>(
        &self, 
        label: Self::Widget<'a>, 
        on_press: Option<Self::Message>
    ) -> Self::Widget<'a>;
    
    /// Create a column layout.
    fn column<'a>(&self, children: Vec<Self::Widget<'a>>) -> Self::Widget<'a>;
    
    /// Create a row layout.
    fn row<'a>(&self, children: Vec<Self::Widget<'a>>) -> Self::Widget<'a>;
    
    /// Create a container.
    fn container<'a>(&self, content: Self::Widget<'a>) -> Self::Widget<'a>;
    
    /// Create a text input.
    fn text_input<'a>(
        &self,
        placeholder: &str,
        value: &str,
        on_input: impl Fn(String) -> Self::Message + 'static,
    ) -> Self::Widget<'a>;
    
    /// Create a checkbox.
    fn checkbox<'a>(
        &self,
        label: &str,
        is_checked: bool,
        on_toggle: impl Fn(bool) -> Self::Message + 'static,
    ) -> Self::Widget<'a>;
    
    /// Create a slider.
    fn slider<'a>(
        &self,
        range: std::ops::RangeInclusive<f32>,
        value: f32,
        on_change: impl Fn(f32) -> Self::Message + 'static,
    ) -> Self::Widget<'a>;
    
    // ... additional widget methods
}
```

---

## Error Types

### ParseError

Errors from XML parsing and expression parsing.

```rust
/// Error during parsing.
#[derive(Debug, Clone, PartialEq)]
pub struct ParseError {
    /// Error kind
    pub kind: ParseErrorKind,
    
    /// Human-readable message
    pub message: String,
    
    /// Source location
    pub span: Span,
    
    /// Suggestion for fixing the error
    pub suggestion: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParseErrorKind {
    /// Invalid XML syntax
    XmlSyntax,
    
    /// Unknown widget element
    UnknownWidget,
    
    /// Unknown attribute
    UnknownAttribute,
    
    /// Invalid attribute value
    InvalidValue,
    
    /// Malformed binding expression
    InvalidExpression,
    
    /// Unclosed binding brace
    UnclosedBinding,
    
    /// Missing required attribute
    MissingAttribute,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f, 
            "error[{}]: {} at line {}, column {}",
            self.kind,
            self.message,
            self.span.line,
            self.span.column
        )?;
        if let Some(suggestion) = &self.suggestion {
            write!(f, "\n  help: {}", suggestion)?;
        }
        Ok(())
    }
}
```

### BindingError

Errors during binding evaluation (dev mode).

```rust
/// Error during binding evaluation.
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
```

---

## State Management Types

### RuntimeState

State container for hot-reload preservation.

```rust
/// Wrapper for serializable runtime state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeState<T> {
    /// Serialized model data
    pub model: T,
    
    /// Schema version for migration
    pub version: u32,
    
    /// Timestamp of last save
    pub saved_at: u64,
}

/// Result of attempting to restore state.
pub enum StateRestoration<T> {
    /// Full restoration successful
    Restored(T),
    
    /// Partial restoration with default values for new fields
    Partial { 
        model: T, 
        missing_fields: Vec<String> 
    },
    
    /// Cannot restore, using defaults
    Default { 
        model: T, 
        reason: String 
    },
}
```

---

## Entity Relationships

```text
GravityDocument
    └── WidgetNode (root)
            ├── WidgetKind
            ├── attributes: HashMap<String, AttributeValue>
            │       └── AttributeValue
            │               ├── Static(String)
            │               ├── Binding(BindingExpr)
            │               │       └── Expr (AST)
            │               └── Interpolated(Vec<InterpolatedPart>)
            ├── events: Vec<EventBinding>
            │       ├── EventKind
            │       └── handler: String → HandlerRegistry
            ├── children: Vec<WidgetNode> (recursive)
            └── span: Span

HandlerRegistry<Model, Message>
    └── HandlerEntry
            ├── Simple
            ├── WithValue
            └── WithCommand

Backend (trait)
    └── IcedBackend (implementation)
            └── Renders WidgetNode → iced::Element
```

---

## Validation Rules

| Entity | Rule | Enforced At |
|--------|------|-------------|
| WidgetNode.kind | Must be valid WidgetKind | Parse time |
| EventBinding.handler | Must exist in HandlerRegistry | Compile time (prod), Runtime (dev) |
| BindingExpr.path | Must resolve to Model field | Compile time (prod), Runtime (dev) |
| WidgetNode.children | Only valid for container widgets | Parse time |
| Span | Line/column must be positive | Parse time |
