use crate::ir::span::Span;

/// A parsed binding expression
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BindingExpr {
    pub expr: Expr,
    pub span: Span,
}

/// Expression AST node
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Expr {
    /// Field access on the local model: `{field}` or `{model.field.subfield}`
    FieldAccess(FieldAccessExpr),
    /// Field access on the shared context: `{shared.field}` or `{shared.field.subfield}`
    SharedFieldAccess(SharedFieldAccessExpr),
    MethodCall(MethodCallExpr),
    BinaryOp(BinaryOpExpr),
    UnaryOp(UnaryOpExpr),
    Conditional(ConditionalExpr),
    Literal(LiteralExpr),
}

/// Field access path
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FieldAccessExpr {
    pub path: Vec<String>,
}

/// Shared field access path
///
/// Represents access to shared state fields via `{shared.field}` syntax.
/// The path does NOT include the "shared" prefix (it's implied).
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SharedFieldAccessExpr {
    /// The path segments after "shared." (e.g., `["theme"]` for `{shared.theme}`)
    pub path: Vec<String>,
}

/// Method call with arguments
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MethodCallExpr {
    pub receiver: Box<Expr>,
    pub method: String,
    pub args: Vec<Expr>,
}

/// Binary operation
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BinaryOpExpr {
    pub left: Box<Expr>,
    pub op: BinaryOp,
    pub right: Box<Expr>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum BinaryOp {
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    And,
    Or,
    Add,
    Sub,
    Mul,
    Div,
}

/// Unary operation
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UnaryOpExpr {
    pub op: UnaryOp,
    pub operand: Box<Expr>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum UnaryOp {
    Not,
    Neg,
}

/// Conditional expression
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ConditionalExpr {
    pub condition: Box<Expr>,
    pub then_branch: Box<Expr>,
    pub else_branch: Box<Expr>,
}

/// Literal value
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum LiteralExpr {
    String(String),
    Integer(i64),
    Float(f64),
    Bool(bool),
}

// ============================================
// Expr helper methods
// ============================================

impl Expr {
    /// Check if this expression accesses shared state.
    ///
    /// Returns `true` if this expression or any of its sub-expressions
    /// reference shared state via `{shared.field}` syntax.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dampen_core::expr::{Expr, SharedFieldAccessExpr, FieldAccessExpr};
    ///
    /// // Shared field access
    /// let shared_expr = Expr::SharedFieldAccess(SharedFieldAccessExpr {
    ///     path: vec!["theme".to_string()],
    /// });
    /// assert!(shared_expr.uses_shared());
    ///
    /// // Regular field access
    /// let regular_expr = Expr::FieldAccess(FieldAccessExpr {
    ///     path: vec!["count".to_string()],
    /// });
    /// assert!(!regular_expr.uses_shared());
    /// ```
    pub fn uses_shared(&self) -> bool {
        match self {
            Expr::SharedFieldAccess(_) => true,
            Expr::FieldAccess(_) => false,
            Expr::Literal(_) => false,
            Expr::MethodCall(m) => {
                m.receiver.uses_shared() || m.args.iter().any(|a| a.uses_shared())
            }
            Expr::BinaryOp(b) => b.left.uses_shared() || b.right.uses_shared(),
            Expr::UnaryOp(u) => u.operand.uses_shared(),
            Expr::Conditional(c) => {
                c.condition.uses_shared()
                    || c.then_branch.uses_shared()
                    || c.else_branch.uses_shared()
            }
        }
    }

    /// Check if this expression accesses the local model.
    ///
    /// Returns `true` if this expression or any of its sub-expressions
    /// reference local model fields via `{field}` syntax.
    pub fn uses_model(&self) -> bool {
        match self {
            Expr::FieldAccess(_) => true,
            Expr::SharedFieldAccess(_) => false,
            Expr::Literal(_) => false,
            Expr::MethodCall(m) => m.receiver.uses_model() || m.args.iter().any(|a| a.uses_model()),
            Expr::BinaryOp(b) => b.left.uses_model() || b.right.uses_model(),
            Expr::UnaryOp(u) => u.operand.uses_model(),
            Expr::Conditional(c) => {
                c.condition.uses_model() || c.then_branch.uses_model() || c.else_branch.uses_model()
            }
        }
    }
}

impl BindingExpr {
    /// Check if this binding expression accesses shared state.
    ///
    /// Convenience method that delegates to `Expr::uses_shared()`.
    pub fn uses_shared(&self) -> bool {
        self.expr.uses_shared()
    }

    /// Check if this binding expression accesses the local model.
    ///
    /// Convenience method that delegates to `Expr::uses_model()`.
    pub fn uses_model(&self) -> bool {
        self.expr.uses_model()
    }
}
