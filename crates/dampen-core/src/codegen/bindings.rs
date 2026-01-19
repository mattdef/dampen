//! Binding expression inlining for code generation
//!
//! This module provides functions to generate pure Rust code from binding expressions,
//! eliminating runtime interpretation overhead in production builds.

use crate::CodegenError;
use crate::expr::ast::{
    BinaryOp, BinaryOpExpr, ConditionalExpr, Expr, FieldAccessExpr, LiteralExpr, MethodCallExpr,
    SharedFieldAccessExpr, UnaryOp, UnaryOpExpr,
};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

/// Generate Rust code for a binding expression
///
/// This function converts binding expressions from the AST into TokenStream
/// for direct field access without runtime evaluation.
///
/// # Arguments
/// * `expr` - The expression to generate code for
///
/// # Returns
/// Generated code as a TokenStream that produces a String when interpolated
pub fn generate_expr(expr: &Expr) -> TokenStream {
    match expr {
        Expr::FieldAccess(field_access) => generate_field_access(field_access),
        Expr::SharedFieldAccess(shared_access) => generate_shared_field_access(shared_access),
        Expr::MethodCall(method_call) => generate_method_call(method_call),
        Expr::BinaryOp(binary_op) => generate_binary_op(binary_op),
        Expr::UnaryOp(unary_op) => generate_unary_op(unary_op),
        Expr::Conditional(conditional) => generate_conditional(conditional),
        Expr::Literal(literal) => generate_literal(literal),
    }
}

/// Generate Rust code for a boolean expression
///
/// This function is like generate_expr but produces native boolean values
/// instead of converting to String. Use for conditions like `enabled="{count > 0}"`.
///
/// # Arguments
/// * `expr` - The expression to generate code for
///
/// # Returns
/// Generated code as a TokenStream that produces a bool
pub fn generate_bool_expr(expr: &Expr) -> TokenStream {
    match expr {
        Expr::FieldAccess(field_access) => generate_field_access_raw(field_access),
        Expr::SharedFieldAccess(shared_access) => generate_shared_field_access_raw(shared_access),
        Expr::MethodCall(method_call) => generate_method_call_raw(method_call),
        Expr::BinaryOp(binary_op) => generate_binary_op_raw(binary_op),
        Expr::UnaryOp(unary_op) => generate_unary_op_raw(unary_op),
        Expr::Conditional(conditional) => generate_conditional_raw(conditional),
        Expr::Literal(literal) => generate_literal_raw(literal),
    }
}

/// Validate that an expression can be inlined
///
/// Returns Err if the expression contains unsupported constructs for codegen
pub fn validate_expression_inlinable(expr: &Expr) -> Result<(), CodegenError> {
    match expr {
        Expr::FieldAccess(_) => Ok(()),
        Expr::SharedFieldAccess(_) => Ok(()), // Shared field access is inlinable
        Expr::MethodCall(method_expr) => {
            validate_expression_inlinable(&method_expr.receiver)?;
            for arg in &method_expr.args {
                validate_expression_inlinable(arg)?;
            }
            Ok(())
        }
        Expr::BinaryOp(binary_expr) => {
            validate_expression_inlinable(&binary_expr.left)?;
            validate_expression_inlinable(&binary_expr.right)?;
            Ok(())
        }
        Expr::UnaryOp(unary_expr) => {
            validate_expression_inlinable(&unary_expr.operand)?;
            Ok(())
        }
        Expr::Conditional(cond_expr) => {
            validate_expression_inlinable(&cond_expr.condition)?;
            validate_expression_inlinable(&cond_expr.then_branch)?;
            validate_expression_inlinable(&cond_expr.else_branch)?;
            Ok(())
        }
        Expr::Literal(_) => Ok(()),
    }
}

/// Generate code for a field access expression
///
/// # Arguments
/// * `expr` - Field access with path components
///
/// # Returns
/// TokenStream generating `model.field.to_string()`
fn generate_field_access(expr: &FieldAccessExpr) -> TokenStream {
    if expr.path.is_empty() {
        return quote! { String::new() };
    }

    let field_access: Vec<_> = expr.path.iter().map(|s| format_ident!("{}", s)).collect();

    quote! { model.#(#field_access).*.to_string() }
}

/// Generate code for a shared field access expression
///
/// # Arguments
/// * `expr` - Shared field access with path components (after "shared.")
///
/// # Returns
/// TokenStream generating `shared.field.to_string()` where `shared` refers to
/// the shared context's read guard
fn generate_shared_field_access(expr: &SharedFieldAccessExpr) -> TokenStream {
    if expr.path.is_empty() {
        return quote! { String::new() };
    }

    // Generate access to the shared context
    // The generated code assumes a `shared` variable is in scope (the read guard)
    let field_access: Vec<_> = expr.path.iter().map(|s| format_ident!("{}", s)).collect();

    quote! { shared.#(#field_access).*.to_string() }
}

/// Generate code for a method call expression
///
/// # Arguments
/// * `expr` - Method call with receiver and arguments
///
/// # Returns
/// TokenStream generating `model.receiver.method(args).to_string()`
fn generate_method_call(expr: &MethodCallExpr) -> TokenStream {
    let receiver_tokens = generate_expr(&expr.receiver);
    let method_ident = format_ident!("{}", expr.method);

    if expr.args.is_empty() {
        quote! { #receiver_tokens.#method_ident().to_string() }
    } else {
        let arg_tokens: Vec<TokenStream> = expr.args.iter().map(generate_expr).collect();
        quote! { #receiver_tokens.#method_ident(#(#arg_tokens),*).to_string() }
    }
}

/// Generate code for a binary operation expression
///
/// # Arguments
/// * `expr` - Binary operation with left, op, right
///
/// # Returns
/// TokenStream generating `(left op right).to_string()`
fn generate_binary_op(expr: &BinaryOpExpr) -> TokenStream {
    let left = generate_expr(&expr.left);
    let right = generate_expr(&expr.right);
    let op = match expr.op {
        BinaryOp::Eq => quote! { == },
        BinaryOp::Ne => quote! { != },
        BinaryOp::Lt => quote! { < },
        BinaryOp::Le => quote! { <= },
        BinaryOp::Gt => quote! { > },
        BinaryOp::Ge => quote! { >= },
        BinaryOp::And => quote! { && },
        BinaryOp::Or => quote! { || },
        BinaryOp::Add => quote! { + },
        BinaryOp::Sub => quote! { - },
        BinaryOp::Mul => quote! { * },
        BinaryOp::Div => quote! { / },
    };

    quote! { (#left #op #right).to_string() }
}

/// Generate code for a unary operation expression
///
/// # Arguments
/// * `expr` - Unary operation with operator and operand
///
/// # Returns
/// TokenStream generating `(!operand).to_string()` or `(-operand).to_string()`
fn generate_unary_op(expr: &UnaryOpExpr) -> TokenStream {
    let operand = generate_expr(&expr.operand);
    let op = match expr.op {
        UnaryOp::Not => quote! { ! },
        UnaryOp::Neg => quote! { - },
    };

    quote! { (#op #operand).to_string() }
}

/// Generate code for a conditional expression
///
/// # Arguments
/// * `expr` - Conditional with condition, then_branch, else_branch
///
/// # Returns
/// TokenStream generating `if condition { then } else { else }.to_string()`
fn generate_conditional(expr: &ConditionalExpr) -> TokenStream {
    let condition = generate_expr(&expr.condition);
    let then_branch = generate_expr(&expr.then_branch);
    let else_branch = generate_expr(&expr.else_branch);

    quote! {
        {
            let __cond = #condition;
            let __then = #then_branch;
            let __else = #else_branch;
            if __cond.trim() == "true" || __cond.parse::<bool>().unwrap_or(false) {
                __then
            } else {
                __else
            }
        }
    }
}

/// Generate code for a literal expression
///
/// # Arguments
/// * `expr` - Literal value
///
/// # Returns
/// TokenStream generating the literal value as a string
fn generate_literal(expr: &LiteralExpr) -> TokenStream {
    match expr {
        LiteralExpr::String(s) => {
            let lit = proc_macro2::Literal::string(s);
            quote! { #lit.to_string() }
        }
        LiteralExpr::Integer(n) => {
            let lit = proc_macro2::Literal::i64_unsuffixed(*n);
            quote! { #lit.to_string() }
        }
        LiteralExpr::Float(f) => {
            let lit = proc_macro2::Literal::f64_unsuffixed(*f);
            quote! { #lit.to_string() }
        }
        LiteralExpr::Bool(b) => {
            let val = if *b { "true" } else { "false" };
            let lit = proc_macro2::Literal::string(val);
            quote! { #lit.to_string() }
        }
    }
}

/// Generate Rust code for interpolated strings
///
/// Converts interpolated strings like "Count: {count}" into format! macro calls.
///
/// # Arguments
/// * `parts` - Alternating literal strings and binding expressions
///
/// # Returns
/// TokenStream generating format! macro invocation
///
/// # Examples
/// ```ignore
/// // "Count: {count}"
/// generate_interpolated(...) -> quote! { format!("Count: {}", count) }
/// ```
pub fn generate_interpolated(parts: &[String]) -> TokenStream {
    if parts.is_empty() {
        return quote! { String::new() };
    }

    let mut format_args = Vec::new();
    let mut arg_exprs = Vec::new();

    for part in parts {
        if part.starts_with('{') && part.ends_with('}') {
            let binding_name = &part[1..part.len() - 1];
            let field_parts: Vec<_> = binding_name
                .split('.')
                .map(|s| format_ident!("{}", s))
                .collect();
            format_args.push("{}");
            arg_exprs.push(quote! { #(#field_parts).*.to_string() });
        } else {
            format_args.push(part);
        }
    }

    let format_string = format_args.join("");
    let lit = proc_macro2::Literal::string(&format_string);

    quote! { format!(#lit, #(#arg_exprs),*) }
}

// ============================================================================
// Raw value generation (without .to_string() conversion)
// Used for boolean expressions in conditions like enabled="{count > 0}"
// ============================================================================

/// Generate field access without .to_string() conversion
fn generate_field_access_raw(expr: &FieldAccessExpr) -> TokenStream {
    if expr.path.is_empty() {
        return quote! { false };
    }

    let field_access: Vec<_> = expr.path.iter().map(|s| format_ident!("{}", s)).collect();
    quote! { model.#(#field_access).* }
}

/// Generate shared field access without .to_string() conversion
fn generate_shared_field_access_raw(expr: &SharedFieldAccessExpr) -> TokenStream {
    if expr.path.is_empty() {
        return quote! { false };
    }

    let field_access: Vec<_> = expr.path.iter().map(|s| format_ident!("{}", s)).collect();
    quote! { shared.#(#field_access).* }
}

/// Generate method call without .to_string() conversion
fn generate_method_call_raw(expr: &MethodCallExpr) -> TokenStream {
    let receiver_tokens = generate_bool_expr(&expr.receiver);
    let method_ident = format_ident!("{}", &expr.method);
    let arg_tokens: Vec<_> = expr.args.iter().map(generate_bool_expr).collect();

    quote! { #receiver_tokens.#method_ident(#(#arg_tokens),*) }
}

/// Generate binary operation without .to_string() conversion
fn generate_binary_op_raw(expr: &BinaryOpExpr) -> TokenStream {
    let left = generate_bool_expr(&expr.left);
    let right = generate_bool_expr(&expr.right);
    let op = match expr.op {
        BinaryOp::Eq => quote! { == },
        BinaryOp::Ne => quote! { != },
        BinaryOp::Lt => quote! { < },
        BinaryOp::Le => quote! { <= },
        BinaryOp::Gt => quote! { > },
        BinaryOp::Ge => quote! { >= },
        BinaryOp::And => quote! { && },
        BinaryOp::Or => quote! { || },
        BinaryOp::Add => quote! { + },
        BinaryOp::Sub => quote! { - },
        BinaryOp::Mul => quote! { * },
        BinaryOp::Div => quote! { / },
    };

    quote! { #left #op #right }
}

/// Generate unary operation without .to_string() conversion
fn generate_unary_op_raw(expr: &UnaryOpExpr) -> TokenStream {
    let operand = generate_bool_expr(&expr.operand);
    let op = match expr.op {
        UnaryOp::Not => quote! { ! },
        UnaryOp::Neg => quote! { - },
    };

    quote! { #op #operand }
}

/// Generate conditional expression without .to_string() conversion
fn generate_conditional_raw(expr: &ConditionalExpr) -> TokenStream {
    let condition = generate_bool_expr(&expr.condition);
    let then_branch = generate_bool_expr(&expr.then_branch);
    let else_branch = generate_bool_expr(&expr.else_branch);

    quote! {
        if #condition {
            #then_branch
        } else {
            #else_branch
        }
    }
}

/// Generate literal without .to_string() conversion
fn generate_literal_raw(expr: &LiteralExpr) -> TokenStream {
    match expr {
        LiteralExpr::String(s) => {
            let lit = proc_macro2::Literal::string(s);
            quote! { #lit }
        }
        LiteralExpr::Integer(n) => {
            let lit = proc_macro2::Literal::i64_unsuffixed(*n);
            quote! { #lit }
        }
        LiteralExpr::Float(f) => {
            let lit = proc_macro2::Literal::f64_unsuffixed(*f);
            quote! { #lit }
        }
        LiteralExpr::Bool(b) => {
            quote! { #b }
        }
    }
}
