//! Expression evaluator for binding expressions

use crate::binding::{BindingValue, UiBindable};
use crate::expr::error::{BindingError, BindingErrorKind};
use crate::expr::{
    BinaryOp, BinaryOpExpr, ConditionalExpr, Expr, FieldAccessExpr, LiteralExpr, MethodCallExpr,
    SharedFieldAccessExpr, UnaryOp, UnaryOpExpr,
};

/// Evaluate an expression against a model
pub fn evaluate_expr(expr: &Expr, model: &dyn UiBindable) -> Result<BindingValue, BindingError> {
    match expr {
        Expr::FieldAccess(field_expr) => evaluate_field_access(field_expr, model),
        Expr::SharedFieldAccess(_) => {
            // SharedFieldAccess requires shared context - use evaluate_expr_with_shared instead
            Err(BindingError {
                kind: BindingErrorKind::InvalidOperation,
                message: "Shared field access requires shared context. Use evaluate_expr_with_shared instead.".to_string(),
                span: crate::ir::span::Span::new(0, 0, 0, 0),
                suggestion: None,
            })
        }
        Expr::MethodCall(method_expr) => evaluate_method_call(method_expr, model),
        Expr::BinaryOp(binary_expr) => evaluate_binary_op(binary_expr, model),
        Expr::UnaryOp(unary_expr) => evaluate_unary_op(unary_expr, model),
        Expr::Conditional(conditional_expr) => evaluate_conditional(conditional_expr, model),
        Expr::Literal(literal_expr) => Ok(evaluate_literal(literal_expr)),
    }
}

/// Evaluate field access: `counter` or `user.name`
fn evaluate_field_access(
    field_expr: &FieldAccessExpr,
    model: &dyn UiBindable,
) -> Result<BindingValue, BindingError> {
    let path: Vec<&str> = field_expr.path.iter().map(|s| s.as_str()).collect();

    model.get_field(&path).ok_or_else(|| {
        let field_name = field_expr.path.join(".");
        BindingError {
            kind: BindingErrorKind::UnknownField,
            message: format!("Field '{}' not found", field_name),
            span: crate::ir::span::Span::new(0, 0, 0, 0), // Will be set by caller
            suggestion: None,
        }
    })
}

/// Evaluate method call: `items.len()` or `name.to_uppercase()`
fn evaluate_method_call(
    method_expr: &MethodCallExpr,
    model: &dyn UiBindable,
) -> Result<BindingValue, BindingError> {
    let receiver = evaluate_expr(&method_expr.receiver, model)?;
    let method = &method_expr.method;

    // Evaluate arguments (currently unused but evaluated for future compatibility)
    let _args: Vec<BindingValue> = method_expr
        .args
        .iter()
        .map(|arg| evaluate_expr(arg, model))
        .collect::<Result<Vec<_>, _>>()?;

    match (receiver.clone(), method.as_str()) {
        // String methods
        (BindingValue::String(s), "len") => Ok(BindingValue::Integer(s.len() as i64)),
        (BindingValue::String(s), "to_uppercase") => Ok(BindingValue::String(s.to_uppercase())),
        (BindingValue::String(s), "to_lowercase") => Ok(BindingValue::String(s.to_lowercase())),
        (BindingValue::String(s), "trim") => Ok(BindingValue::String(s.trim().to_string())),

        // List methods
        (BindingValue::List(l), "len") => Ok(BindingValue::Integer(l.len() as i64)),
        (BindingValue::List(l), "is_empty") => Ok(BindingValue::Bool(l.is_empty())),

        // Integer methods
        (BindingValue::Integer(i), "to_string") => Ok(BindingValue::String(i.to_string())),

        // Float methods
        (BindingValue::Float(f), "to_string") => Ok(BindingValue::String(f.to_string())),
        (BindingValue::Float(f), "round") => Ok(BindingValue::Float(f.round())),
        (BindingValue::Float(f), "floor") => Ok(BindingValue::Float(f.floor())),
        (BindingValue::Float(f), "ceil") => Ok(BindingValue::Float(f.ceil())),

        // Bool methods
        (BindingValue::Bool(b), "to_string") => Ok(BindingValue::String(b.to_string())),

        _ => Err(BindingError {
            kind: BindingErrorKind::UnknownMethod,
            message: format!("Method '{}' not supported on {:?}", method, receiver),
            span: crate::ir::span::Span::new(0, 0, 0, 0),
            suggestion: None,
        }),
    }
}

/// Evaluate binary operation: `a + b`, `x > 0`, etc.
fn evaluate_binary_op(
    binary_expr: &BinaryOpExpr,
    model: &dyn UiBindable,
) -> Result<BindingValue, BindingError> {
    let left = evaluate_expr(&binary_expr.left, model)?;
    let right = evaluate_expr(&binary_expr.right, model)?;

    match binary_expr.op {
        // Arithmetic
        BinaryOp::Add => match (left, right) {
            (BindingValue::Integer(a), BindingValue::Integer(b)) => {
                Ok(BindingValue::Integer(a + b))
            }
            (BindingValue::Float(a), BindingValue::Float(b)) => Ok(BindingValue::Float(a + b)),
            (BindingValue::String(a), BindingValue::String(b)) => Ok(BindingValue::String(a + &b)),
            _ => Err(BindingError {
                kind: BindingErrorKind::InvalidOperation,
                message: "Cannot add these types".to_string(),
                span: crate::ir::span::Span::new(0, 0, 0, 0),
                suggestion: None,
            }),
        },
        BinaryOp::Sub => match (left, right) {
            (BindingValue::Integer(a), BindingValue::Integer(b)) => {
                Ok(BindingValue::Integer(a - b))
            }
            (BindingValue::Float(a), BindingValue::Float(b)) => Ok(BindingValue::Float(a - b)),
            _ => Err(BindingError {
                kind: BindingErrorKind::InvalidOperation,
                message: "Cannot subtract these types".to_string(),
                span: crate::ir::span::Span::new(0, 0, 0, 0),
                suggestion: None,
            }),
        },
        BinaryOp::Mul => match (left, right) {
            (BindingValue::Integer(a), BindingValue::Integer(b)) => {
                Ok(BindingValue::Integer(a * b))
            }
            (BindingValue::Float(a), BindingValue::Float(b)) => Ok(BindingValue::Float(a * b)),
            _ => Err(BindingError {
                kind: BindingErrorKind::InvalidOperation,
                message: "Cannot multiply these types".to_string(),
                span: crate::ir::span::Span::new(0, 0, 0, 0),
                suggestion: None,
            }),
        },
        BinaryOp::Div => match (left, right) {
            (BindingValue::Integer(a), BindingValue::Integer(b)) if b != 0 => {
                Ok(BindingValue::Integer(a / b))
            }
            (BindingValue::Float(a), BindingValue::Float(b)) if b != 0.0 => {
                Ok(BindingValue::Float(a / b))
            }
            _ => Err(BindingError {
                kind: BindingErrorKind::InvalidOperation,
                message: "Cannot divide these types or division by zero".to_string(),
                span: crate::ir::span::Span::new(0, 0, 0, 0),
                suggestion: None,
            }),
        },

        // Comparison
        BinaryOp::Eq => Ok(BindingValue::Bool(left == right)),
        BinaryOp::Ne => Ok(BindingValue::Bool(left != right)),
        BinaryOp::Lt => Ok(BindingValue::Bool(compare_values(&left, &right, |a, b| {
            a < b
        }))),
        BinaryOp::Le => Ok(BindingValue::Bool(compare_values(&left, &right, |a, b| {
            a <= b
        }))),
        BinaryOp::Gt => Ok(BindingValue::Bool(compare_values(&left, &right, |a, b| {
            a > b
        }))),
        BinaryOp::Ge => Ok(BindingValue::Bool(compare_values(&left, &right, |a, b| {
            a >= b
        }))),

        // Logical
        BinaryOp::And => {
            let left_bool = left.to_bool();
            let right_bool = right.to_bool();
            Ok(BindingValue::Bool(left_bool && right_bool))
        }
        BinaryOp::Or => {
            let left_bool = left.to_bool();
            let right_bool = right.to_bool();
            Ok(BindingValue::Bool(left_bool || right_bool))
        }
    }
}

/// Helper for comparison operations
fn compare_values<F>(left: &BindingValue, right: &BindingValue, cmp: F) -> bool
where
    F: Fn(f64, f64) -> bool,
{
    match (left, right) {
        (BindingValue::Integer(a), BindingValue::Integer(b)) => cmp(*a as f64, *b as f64),
        (BindingValue::Float(a), BindingValue::Float(b)) => cmp(*a, *b),
        (BindingValue::String(a), BindingValue::String(b)) => cmp(a.len() as f64, b.len() as f64),
        (BindingValue::List(a), BindingValue::List(b)) => cmp(a.len() as f64, b.len() as f64),
        _ => false,
    }
}

/// Evaluate unary operation: `!valid` or `-offset`
fn evaluate_unary_op(
    unary_expr: &UnaryOpExpr,
    model: &dyn UiBindable,
) -> Result<BindingValue, BindingError> {
    let operand = evaluate_expr(&unary_expr.operand, model)?;

    match unary_expr.op {
        UnaryOp::Not => Ok(BindingValue::Bool(!operand.to_bool())),
        UnaryOp::Neg => match operand {
            BindingValue::Integer(i) => Ok(BindingValue::Integer(-i)),
            BindingValue::Float(f) => Ok(BindingValue::Float(-f)),
            _ => Err(BindingError {
                kind: BindingErrorKind::InvalidOperation,
                message: "Cannot negate this type".to_string(),
                span: crate::ir::span::Span::new(0, 0, 0, 0),
                suggestion: None,
            }),
        },
    }
}

/// Evaluate conditional: `if condition then a else b`
fn evaluate_conditional(
    conditional_expr: &ConditionalExpr,
    model: &dyn UiBindable,
) -> Result<BindingValue, BindingError> {
    let condition = evaluate_expr(&conditional_expr.condition, model)?;

    if condition.to_bool() {
        evaluate_expr(&conditional_expr.then_branch, model)
    } else {
        evaluate_expr(&conditional_expr.else_branch, model)
    }
}

/// Evaluate literal value
fn evaluate_literal(literal_expr: &LiteralExpr) -> BindingValue {
    match literal_expr {
        LiteralExpr::String(s) => BindingValue::String(s.clone()),
        LiteralExpr::Integer(i) => BindingValue::Integer(*i),
        LiteralExpr::Float(f) => BindingValue::Float(*f),
        LiteralExpr::Bool(b) => BindingValue::Bool(*b),
    }
}

/// Evaluate a binding expression with span information
pub fn evaluate_binding_expr(
    binding_expr: &crate::expr::BindingExpr,
    model: &dyn UiBindable,
) -> Result<BindingValue, BindingError> {
    match evaluate_expr(&binding_expr.expr, model) {
        Ok(result) => Ok(result),
        Err(mut err) => {
            err.span = binding_expr.span;
            Err(err)
        }
    }
}

/// Evaluate formatted string with interpolation: `"Total: {count}"`
pub fn evaluate_formatted(
    parts: &[crate::ir::InterpolatedPart],
    model: &dyn UiBindable,
) -> Result<String, BindingError> {
    let mut result = String::new();

    for part in parts {
        match part {
            crate::ir::InterpolatedPart::Literal(literal) => {
                result.push_str(literal);
            }
            crate::ir::InterpolatedPart::Binding(binding_expr) => {
                let value = evaluate_binding_expr(binding_expr, model)?;
                result.push_str(&value.to_display_string());
            }
        }
    }

    Ok(result)
}

// ==================== Shared-Aware Evaluation ====================

/// Evaluate an expression with access to both local model and shared context
///
/// This is the preferred method when shared state bindings (`{shared.field}`) are used.
/// Falls back to model-only evaluation for expressions that don't use shared state.
///
/// # Arguments
///
/// * `expr` - The expression to evaluate
/// * `model` - The local view model implementing `UiBindable`
/// * `shared` - Optional shared context implementing `UiBindable`
///
/// # Returns
///
/// The evaluated `BindingValue` or an error if evaluation fails.
pub fn evaluate_expr_with_shared(
    expr: &Expr,
    model: &dyn UiBindable,
    shared: Option<&dyn UiBindable>,
) -> Result<BindingValue, BindingError> {
    match expr {
        Expr::FieldAccess(field_expr) => evaluate_field_access(field_expr, model),
        Expr::SharedFieldAccess(shared_expr) => evaluate_shared_field_access(shared_expr, shared),
        Expr::MethodCall(method_expr) => {
            evaluate_method_call_with_shared(method_expr, model, shared)
        }
        Expr::BinaryOp(binary_expr) => evaluate_binary_op_with_shared(binary_expr, model, shared),
        Expr::UnaryOp(unary_expr) => evaluate_unary_op_with_shared(unary_expr, model, shared),
        Expr::Conditional(conditional_expr) => {
            evaluate_conditional_with_shared(conditional_expr, model, shared)
        }
        Expr::Literal(literal_expr) => Ok(evaluate_literal(literal_expr)),
    }
}

/// Evaluate shared field access: `shared.theme` or `shared.user.preferences`
fn evaluate_shared_field_access(
    shared_expr: &SharedFieldAccessExpr,
    shared: Option<&dyn UiBindable>,
) -> Result<BindingValue, BindingError> {
    let Some(shared_ctx) = shared else {
        // No shared context provided - return empty string (graceful degradation)
        return Ok(BindingValue::String(String::new()));
    };

    let path: Vec<&str> = shared_expr.path.iter().map(|s| s.as_str()).collect();

    shared_ctx.get_field(&path).ok_or_else(|| {
        let field_name = format!("shared.{}", shared_expr.path.join("."));
        BindingError {
            kind: BindingErrorKind::UnknownField,
            message: format!("Shared field '{}' not found", field_name),
            span: crate::ir::span::Span::new(0, 0, 0, 0),
            suggestion: None,
        }
    })
}

/// Evaluate method call with shared context support
fn evaluate_method_call_with_shared(
    method_expr: &MethodCallExpr,
    model: &dyn UiBindable,
    shared: Option<&dyn UiBindable>,
) -> Result<BindingValue, BindingError> {
    let receiver = evaluate_expr_with_shared(&method_expr.receiver, model, shared)?;
    let method = &method_expr.method;

    // Evaluate arguments with shared context
    let _args: Vec<BindingValue> = method_expr
        .args
        .iter()
        .map(|arg| evaluate_expr_with_shared(arg, model, shared))
        .collect::<Result<Vec<_>, _>>()?;

    match (receiver.clone(), method.as_str()) {
        // String methods
        (BindingValue::String(s), "len") => Ok(BindingValue::Integer(s.len() as i64)),
        (BindingValue::String(s), "to_uppercase") => Ok(BindingValue::String(s.to_uppercase())),
        (BindingValue::String(s), "to_lowercase") => Ok(BindingValue::String(s.to_lowercase())),
        (BindingValue::String(s), "trim") => Ok(BindingValue::String(s.trim().to_string())),

        // List methods
        (BindingValue::List(l), "len") => Ok(BindingValue::Integer(l.len() as i64)),
        (BindingValue::List(l), "is_empty") => Ok(BindingValue::Bool(l.is_empty())),

        // Integer methods
        (BindingValue::Integer(i), "to_string") => Ok(BindingValue::String(i.to_string())),

        // Float methods
        (BindingValue::Float(f), "to_string") => Ok(BindingValue::String(f.to_string())),
        (BindingValue::Float(f), "round") => Ok(BindingValue::Float(f.round())),
        (BindingValue::Float(f), "floor") => Ok(BindingValue::Float(f.floor())),
        (BindingValue::Float(f), "ceil") => Ok(BindingValue::Float(f.ceil())),

        // Bool methods
        (BindingValue::Bool(b), "to_string") => Ok(BindingValue::String(b.to_string())),

        _ => Err(BindingError {
            kind: BindingErrorKind::UnknownMethod,
            message: format!("Method '{}' not supported on {:?}", method, receiver),
            span: crate::ir::span::Span::new(0, 0, 0, 0),
            suggestion: None,
        }),
    }
}

/// Evaluate binary operation with shared context support
fn evaluate_binary_op_with_shared(
    binary_expr: &BinaryOpExpr,
    model: &dyn UiBindable,
    shared: Option<&dyn UiBindable>,
) -> Result<BindingValue, BindingError> {
    let left = evaluate_expr_with_shared(&binary_expr.left, model, shared)?;
    let right = evaluate_expr_with_shared(&binary_expr.right, model, shared)?;

    match binary_expr.op {
        // Arithmetic
        BinaryOp::Add => match (left, right) {
            (BindingValue::Integer(a), BindingValue::Integer(b)) => {
                Ok(BindingValue::Integer(a + b))
            }
            (BindingValue::Float(a), BindingValue::Float(b)) => Ok(BindingValue::Float(a + b)),
            (BindingValue::String(a), BindingValue::String(b)) => Ok(BindingValue::String(a + &b)),
            _ => Err(BindingError {
                kind: BindingErrorKind::InvalidOperation,
                message: "Cannot add these types".to_string(),
                span: crate::ir::span::Span::new(0, 0, 0, 0),
                suggestion: None,
            }),
        },
        BinaryOp::Sub => match (left, right) {
            (BindingValue::Integer(a), BindingValue::Integer(b)) => {
                Ok(BindingValue::Integer(a - b))
            }
            (BindingValue::Float(a), BindingValue::Float(b)) => Ok(BindingValue::Float(a - b)),
            _ => Err(BindingError {
                kind: BindingErrorKind::InvalidOperation,
                message: "Cannot subtract these types".to_string(),
                span: crate::ir::span::Span::new(0, 0, 0, 0),
                suggestion: None,
            }),
        },
        BinaryOp::Mul => match (left, right) {
            (BindingValue::Integer(a), BindingValue::Integer(b)) => {
                Ok(BindingValue::Integer(a * b))
            }
            (BindingValue::Float(a), BindingValue::Float(b)) => Ok(BindingValue::Float(a * b)),
            _ => Err(BindingError {
                kind: BindingErrorKind::InvalidOperation,
                message: "Cannot multiply these types".to_string(),
                span: crate::ir::span::Span::new(0, 0, 0, 0),
                suggestion: None,
            }),
        },
        BinaryOp::Div => match (left, right) {
            (BindingValue::Integer(a), BindingValue::Integer(b)) if b != 0 => {
                Ok(BindingValue::Integer(a / b))
            }
            (BindingValue::Float(a), BindingValue::Float(b)) if b != 0.0 => {
                Ok(BindingValue::Float(a / b))
            }
            _ => Err(BindingError {
                kind: BindingErrorKind::InvalidOperation,
                message: "Cannot divide these types or division by zero".to_string(),
                span: crate::ir::span::Span::new(0, 0, 0, 0),
                suggestion: None,
            }),
        },

        // Comparison
        BinaryOp::Eq => Ok(BindingValue::Bool(left == right)),
        BinaryOp::Ne => Ok(BindingValue::Bool(left != right)),
        BinaryOp::Lt => Ok(BindingValue::Bool(compare_values(&left, &right, |a, b| {
            a < b
        }))),
        BinaryOp::Le => Ok(BindingValue::Bool(compare_values(&left, &right, |a, b| {
            a <= b
        }))),
        BinaryOp::Gt => Ok(BindingValue::Bool(compare_values(&left, &right, |a, b| {
            a > b
        }))),
        BinaryOp::Ge => Ok(BindingValue::Bool(compare_values(&left, &right, |a, b| {
            a >= b
        }))),

        // Logical
        BinaryOp::And => {
            let left_bool = left.to_bool();
            let right_bool = right.to_bool();
            Ok(BindingValue::Bool(left_bool && right_bool))
        }
        BinaryOp::Or => {
            let left_bool = left.to_bool();
            let right_bool = right.to_bool();
            Ok(BindingValue::Bool(left_bool || right_bool))
        }
    }
}

/// Evaluate unary operation with shared context support
fn evaluate_unary_op_with_shared(
    unary_expr: &UnaryOpExpr,
    model: &dyn UiBindable,
    shared: Option<&dyn UiBindable>,
) -> Result<BindingValue, BindingError> {
    let operand = evaluate_expr_with_shared(&unary_expr.operand, model, shared)?;

    match unary_expr.op {
        UnaryOp::Not => Ok(BindingValue::Bool(!operand.to_bool())),
        UnaryOp::Neg => match operand {
            BindingValue::Integer(i) => Ok(BindingValue::Integer(-i)),
            BindingValue::Float(f) => Ok(BindingValue::Float(-f)),
            _ => Err(BindingError {
                kind: BindingErrorKind::InvalidOperation,
                message: "Cannot negate this type".to_string(),
                span: crate::ir::span::Span::new(0, 0, 0, 0),
                suggestion: None,
            }),
        },
    }
}

/// Evaluate conditional with shared context support
fn evaluate_conditional_with_shared(
    conditional_expr: &ConditionalExpr,
    model: &dyn UiBindable,
    shared: Option<&dyn UiBindable>,
) -> Result<BindingValue, BindingError> {
    let condition = evaluate_expr_with_shared(&conditional_expr.condition, model, shared)?;

    if condition.to_bool() {
        evaluate_expr_with_shared(&conditional_expr.then_branch, model, shared)
    } else {
        evaluate_expr_with_shared(&conditional_expr.else_branch, model, shared)
    }
}

/// Evaluate a binding expression with shared context support
pub fn evaluate_binding_expr_with_shared(
    binding_expr: &crate::expr::BindingExpr,
    model: &dyn UiBindable,
    shared: Option<&dyn UiBindable>,
) -> Result<BindingValue, BindingError> {
    match evaluate_expr_with_shared(&binding_expr.expr, model, shared) {
        Ok(result) => Ok(result),
        Err(mut err) => {
            err.span = binding_expr.span;
            Err(err)
        }
    }
}

/// Evaluate formatted string with interpolation and shared context support
pub fn evaluate_formatted_with_shared(
    parts: &[crate::ir::InterpolatedPart],
    model: &dyn UiBindable,
    shared: Option<&dyn UiBindable>,
) -> Result<String, BindingError> {
    let mut result = String::new();

    for part in parts {
        match part {
            crate::ir::InterpolatedPart::Literal(literal) => {
                result.push_str(literal);
            }
            crate::ir::InterpolatedPart::Binding(binding_expr) => {
                let value = evaluate_binding_expr_with_shared(binding_expr, model, shared)?;
                result.push_str(&value.to_display_string());
            }
        }
    }

    Ok(result)
}
