//! Handler dispatch code generation
//!
//! This module generates efficient handler dispatch code for production builds,
//! converting handler registry lookups into direct function calls.

use crate::handler::HandlerSignature;
use crate::CodegenError;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

/// Generate handler dispatch code for production mode
///
/// Converts dynamic handler registry lookups into static function calls
/// for zero runtime overhead.
///
/// # Arguments
/// * `handlers` - List of handler signatures
/// * `model_name` - Name of the model struct
/// * `message_name` - Name of the message enum
///
/// # Returns
/// Generated match statement as TokenStream
pub fn generate_handler_dispatch(
    handlers: &[HandlerSignature],
    model_name: &str,
    message_name: &str,
) -> Result<TokenStream, CodegenError> {
    if handlers.is_empty() {
        return Ok(quote! {
            match _handler {
                _ => {}
            }
        });
    }

    let model_ident = format_ident!("{}", model_name);
    let message_ident = format_ident!("{}", message_name);

    let match_arms: Vec<TokenStream> = handlers
        .iter()
        .map(move |handler| {
            let _handler_ident = format_ident!("{}", handler.name);

            match &handler.param_type {
                Some(param_type) if handler.returns_command => generate_handler_with_command(
                    &handler.name,
                    param_type,
                    &model_ident,
                    &message_ident,
                ),
                Some(param_type) => generate_handler_with_value(
                    &handler.name,
                    param_type,
                    &model_ident,
                    &message_ident,
                ),
                None if handler.returns_command => {
                    generate_handler_simple(&handler.name, &model_ident, &message_ident, true)
                }
                None => generate_handler_simple(&handler.name, &model_ident, &message_ident, false),
            }
        })
        .collect();

    Ok(quote! {
        match handler_name {
            #(#match_arms),*
        }
    })
}

/// Generate handler dispatch for simple handlers (no parameters, no return)
fn generate_handler_simple(
    handler_name: &str,
    _model_ident: &syn::Ident,
    message_ident: &syn::Ident,
    _returns_command: bool,
) -> TokenStream {
    let handler_ident = format_ident!("{}", handler_name);
    quote! {
        #handler_name => {
            #message_ident::#handler_ident
        }
    }
}

/// Generate handler dispatch for handlers that take a value parameter
///
/// # Arguments
/// * `handler_name` - Name of the handler
/// * `value_type` - Type of the value parameter
/// * `_model_ident` - Model identifier (unused, kept for API consistency)
/// * `message_ident` - Message identifier
///
/// # Returns
/// Generated match arm with value parameter
fn generate_handler_with_value(
    handler_name: &str,
    value_type: &str,
    _model_ident: &syn::Ident,
    message_ident: &syn::Ident,
) -> TokenStream {
    let handler_ident = format_ident!("{}", handler_name);
    let type_ident = format_ident!("{}", value_type);
    quote! {
        #handler_name => {
            |value: #type_ident| #message_ident::#handler_ident(value)
        }
    }
}

/// Generate handler dispatch for handlers that return commands
///
/// # Arguments
/// * `handler_name` - Name of the handler
/// * `value_type` - Type of the value parameter (if any)
/// * `_model_ident` - Model identifier (unused, kept for API consistency)
/// * `message_ident` - Message identifier
///
/// # Returns
/// Generated match arm with command return
fn generate_handler_with_command(
    handler_name: &str,
    value_type: &str,
    _model_ident: &syn::Ident,
    message_ident: &syn::Ident,
) -> TokenStream {
    let handler_ident = format_ident!("{}", handler_name);
    let type_ident = format_ident!("{}", value_type);
    quote! {
        #handler_name => {
            |value: #type_ident| #message_ident::#handler_ident(value)
        }
    }
}

/// Generate the update function body
///
/// # Arguments
/// * `handlers` - List of handler signatures
/// * `model_name` - Name of the model struct
/// * `message_name` - Name of the message enum
///
/// # Returns
/// Generated update function as TokenStream
pub fn generate_update_function(
    handlers: &[HandlerSignature],
    model_name: &str,
    message_name: &str,
) -> Result<TokenStream, CodegenError> {
    let message_ident = format_ident!("{}", message_name);

    if handlers.is_empty() {
        return Ok(quote! {
            fn update(&mut self, _message: Self::Message) {}
        });
    }

    let arms: Vec<TokenStream> = handlers
        .iter()
        .map(|handler| {
            let _model_ident = format_ident!("{}", model_name);
            let _handler_ident = format_ident!("{}", handler.name);
            let handler_name_str = handler.name.clone();

            match (&handler.param_type, handler.returns_command) {
                (None, false) => {
                    quote! {
                        #message_ident::#_handler_ident => {
                            self.handler_registry.call_simple(&mut self.model, #handler_name_str);
                        }
                    }
                }
                (Some(param_type), false) => {
                    let type_ident = format_ident!("{}", param_type);
                    quote! {
                        #message_ident::#_handler_ident(value) => {
                            self.handler_registry.call_with_value::<#type_ident>(&mut self.model, #handler_name_str, value);
                        }
                    }
                }
                (None, true) => {
                    quote! {
                        #message_ident::#_handler_ident => {
                            if let Some(cmd) = self.handler_registry.call_with_command(&mut self.model, #handler_name_str) {
                                return cmd;
                            }
                        }
                    }
                }
                (Some(param_type), true) => {
                    let type_ident = format_ident!("{}", param_type);
                    quote! {
                        #message_ident::#_handler_ident(value) => {
                            if let Some(cmd) = self.handler_registry.call_with_command::<#type_ident>(&mut self.model, #handler_name_str, value) {
                                return cmd;
                            }
                        }
                    }
                }
            }
        })
        .collect();

    let catch_all = quote! {
        _ => {}
    };

    Ok(quote! {
        fn update(&mut self, message: Self::Message) -> Option<iced::Command<Self::Message>> {
            match message {
                #(#arms)*
                #catch_all
            }
            None
        }
    })
}

/// Validate that expressions can be inlined
///
/// Returns Err if the expression contains unsupported constructs
pub fn validate_expression_inlinable(expr: &crate::Expr) -> Result<(), CodegenError> {
    match expr {
        crate::Expr::FieldAccess(_) => Ok(()),
        crate::Expr::MethodCall(method_expr) => {
            validate_expression_inlinable(&method_expr.receiver)?;
            for arg in &method_expr.args {
                validate_expression_inlinable(arg)?;
            }
            Ok(())
        }
        crate::Expr::BinaryOp(binary_expr) => {
            validate_expression_inlinable(&binary_expr.left)?;
            validate_expression_inlinable(&binary_expr.right)?;
            Ok(())
        }
        crate::Expr::UnaryOp(unary_expr) => {
            validate_expression_inlinable(&unary_expr.operand)?;
            Ok(())
        }
        crate::Expr::Conditional(cond_expr) => {
            validate_expression_inlinable(&cond_expr.condition)?;
            validate_expression_inlinable(&cond_expr.then_branch)?;
            validate_expression_inlinable(&cond_expr.else_branch)?;
            Ok(())
        }
        crate::Expr::Literal(_) => Ok(()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handler_dispatch_simple() {
        let handlers = vec![HandlerSignature {
            name: "increment".to_string(),
            param_type: None,
            returns_command: false,
        }];

        let result = generate_handler_dispatch(&handlers, "Model", "Message").unwrap();
        let code = result.to_string();
        assert!(code.contains("increment"));
    }

    #[test]
    fn test_handler_dispatch_with_value() {
        let handlers = vec![HandlerSignature {
            name: "set_value".to_string(),
            param_type: Some("String".to_string()),
            returns_command: false,
        }];

        let result = generate_handler_dispatch(&handlers, "Model", "Message").unwrap();
        let code = result.to_string();
        assert!(code.contains("set_value"));
        assert!(code.contains("String"));
    }

    #[test]
    fn test_handler_dispatch_with_command() {
        let handlers = vec![HandlerSignature {
            name: "save".to_string(),
            param_type: None,
            returns_command: true,
        }];

        let result = generate_handler_dispatch(&handlers, "Model", "Message").unwrap();
        let code = result.to_string();
        assert!(code.contains("save"));
    }

    #[test]
    fn test_update_function_generation() {
        let handlers = vec![
            HandlerSignature {
                name: "increment".to_string(),
                param_type: None,
                returns_command: false,
            },
            HandlerSignature {
                name: "set_value".to_string(),
                param_type: Some("String".to_string()),
                returns_command: false,
            },
        ];

        let result = generate_update_function(&handlers, "Model", "Message").unwrap();
        let code = result.to_string();
        assert!(code.contains("update"));
        assert!(code.contains("increment"));
        assert!(code.contains("set_value"));
    }

    #[test]
    fn test_expression_validation() {
        use crate::expr::ast::{BinaryOp, BinaryOpExpr, Expr, FieldAccessExpr, LiteralExpr};

        let expr = Expr::BinaryOp(BinaryOpExpr {
            left: Box::new(Expr::FieldAccess(FieldAccessExpr {
                path: vec!["count".to_string()],
            })),
            op: BinaryOp::Add,
            right: Box::new(Expr::Literal(LiteralExpr::Integer(1))),
        });

        assert!(validate_expression_inlinable(&expr).is_ok());
    }
}
