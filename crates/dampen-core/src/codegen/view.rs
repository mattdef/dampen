//! View function generation

use crate::{AttributeValue, DampenDocument, InterpolatedPart, WidgetKind};
use proc_macro2::TokenStream;
use quote::quote;

/// Generate the view function from a Dampen document
pub fn generate_view(
    document: &DampenDocument,
    model_name: &str,
    message_name: &str,
) -> Result<TokenStream, super::CodegenError> {
    let model_ident = syn::Ident::new(model_name, proc_macro2::Span::call_site());
    let message_ident = syn::Ident::new(message_name, proc_macro2::Span::call_site());

    let root_widget = generate_widget(&document.root, &model_ident, &message_ident)?;

    Ok(quote! {
        fn view(&self) -> iced::Element<'_, Self::Message> {
            #root_widget
        }
    })
}

/// Generate code for a widget node
fn generate_widget(
    node: &crate::WidgetNode,
    model_ident: &syn::Ident,
    message_ident: &syn::Ident,
) -> Result<TokenStream, super::CodegenError> {
    match node.kind {
        WidgetKind::Text => generate_text(node, model_ident),
        WidgetKind::Button => generate_button(node, model_ident, message_ident),
        WidgetKind::Column => generate_container(node, "column", model_ident, message_ident),
        WidgetKind::Row => generate_container(node, "row", model_ident, message_ident),
        WidgetKind::Container => generate_container(node, "container", model_ident, message_ident),
        _ => {
            // For now, generate a placeholder for unsupported widgets
            Ok(quote! { iced::widget::text("Unsupported widget") })
        }
    }
}

/// Generate text widget
fn generate_text(
    node: &crate::WidgetNode,
    model_ident: &syn::Ident,
) -> Result<TokenStream, super::CodegenError> {
    let value_attr = node.attributes.get("value").ok_or_else(|| {
        super::CodegenError::InvalidWidget("text requires value attribute".to_string())
    })?;

    let value_expr = generate_attribute_value(value_attr, model_ident);

    Ok(quote! {
        iced::widget::text(#value_expr)
    })
}

/// Generate button widget
fn generate_button(
    node: &crate::WidgetNode,
    model_ident: &syn::Ident,
    message_ident: &syn::Ident,
) -> Result<TokenStream, super::CodegenError> {
    let label_attr = node.attributes.get("label").ok_or_else(|| {
        super::CodegenError::InvalidWidget("button requires label attribute".to_string())
    })?;

    let label_expr = generate_attribute_value(label_attr, model_ident);

    // Find click handler
    let on_click = node
        .events
        .iter()
        .find(|e| e.event == crate::EventKind::Click);

    if let Some(event) = on_click {
        let handler_ident = syn::Ident::new(&event.handler, proc_macro2::Span::call_site());

        Ok(quote! {
            iced::widget::button(#label_expr)
                .on_press(#message_ident::#handler_ident)
        })
    } else {
        Ok(quote! {
            iced::widget::button(#label_expr)
        })
    }
}

/// Generate container widget (column, row, container)
fn generate_container(
    node: &crate::WidgetNode,
    widget_type: &str,
    model_ident: &syn::Ident,
    message_ident: &syn::Ident,
) -> Result<TokenStream, super::CodegenError> {
    let children: Vec<TokenStream> = node
        .children
        .iter()
        .map(|child| generate_widget(child, model_ident, message_ident))
        .collect::<Result<_, _>>()?;

    let widget_ident = syn::Ident::new(widget_type, proc_macro2::Span::call_site());

    // Apply spacing if specified
    let spacing = node.attributes.get("spacing").and_then(|attr| {
        if let AttributeValue::Static(s) = attr {
            s.parse::<f32>().ok()
        } else {
            None
        }
    });

    let padding = node.attributes.get("padding").and_then(|attr| {
        if let AttributeValue::Static(s) = attr {
            s.parse::<f32>().ok()
        } else {
            None
        }
    });

    let mut widget = quote! {
            iced::widget::#widget_ident(vec
    ![#(#children),*])
        };

    if let Some(s) = spacing {
        widget = quote! { #widget.spacing(#s) };
    }

    if let Some(p) = padding {
        widget = quote! { #widget.padding(#p) };
    }

    Ok(widget)
}

/// Generate attribute value expression
fn generate_attribute_value(attr: &AttributeValue, model_ident: &syn::Ident) -> TokenStream {
    match attr {
        AttributeValue::Static(s) => {
            quote! { #s.to_string() }
        }
        AttributeValue::Binding(expr) => generate_binding_expr(expr, model_ident),
        AttributeValue::Interpolated(parts) => {
            let parts_tokens: Vec<TokenStream> = parts
                .iter()
                .map(|part| match part {
                    InterpolatedPart::Literal(s) => quote! { #s.to_string() },
                    InterpolatedPart::Binding(expr) => generate_binding_expr(expr, model_ident),
                })
                .collect();

            quote! {
                {
                    let mut result = String::new();
                    #(result.push_str(&#parts_tokens);)*
                    result
                }
            }
        }
    }
}

/// Generate binding expression as Rust code
fn generate_binding_expr(expr: &crate::BindingExpr, model_ident: &syn::Ident) -> TokenStream {
    // This generates the expression that will be evaluated at runtime
    // For production mode, we inline the evaluation logic

    match &expr.expr {
        crate::Expr::FieldAccess(field_expr) => {
            let path = &field_expr.path;
            if path.is_empty() {
                return quote! { String::new() };
            }

            // Generate field access
            let field_name = syn::Ident::new(&path[0], proc_macro2::Span::call_site());

            if path.len() == 1 {
                // Simple field access
                quote! {
                    #model_ident::#field_name.to_binding_value().to_display_string()
                }
            } else {
                // Nested field access - for now, just handle first level
                quote! {
                    #model_ident::#field_name.to_binding_value().to_display_string()
                }
            }
        }
        crate::Expr::MethodCall(method_expr) => {
            // Generate method call
            let method = syn::Ident::new(&method_expr.method, proc_macro2::Span::call_site());
            let receiver = generate_expr(&method_expr.receiver, model_ident);

            quote! {
                #receiver.#method().to_binding_value().to_display_string()
            }
        }
        crate::Expr::BinaryOp(bin_expr) => {
            let left = generate_expr(&bin_expr.left, model_ident);
            let right = generate_expr(&bin_expr.right, model_ident);
            let op = generate_binary_op(&bin_expr.op);

            quote! {
                if #left #op #right {
                    "true".to_string()
                } else {
                    "false".to_string()
                }
            }
        }
        crate::Expr::Conditional(cond_expr) => {
            let cond = generate_expr(&cond_expr.condition, model_ident);
            let then = generate_expr(&cond_expr.then_branch, model_ident);
            let else_ = generate_expr(&cond_expr.else_branch, model_ident);

            quote! {
                if #cond.to_binding_value().to_bool() {
                    #then
                } else {
                    #else_
                }
            }
        }
        crate::Expr::Literal(lit_expr) => match lit_expr {
            crate::LiteralExpr::String(s) => quote! { #s.to_string() },
            crate::LiteralExpr::Integer(i) => quote! { #i.to_string() },
            crate::LiteralExpr::Float(f) => quote! { #f.to_string() },
            crate::LiteralExpr::Bool(b) => quote! { #b.to_string() },
        },
        _ => quote! { String::new() },
    }
}

fn generate_expr(expr: &crate::Expr, model_ident: &syn::Ident) -> TokenStream {
    match expr {
        crate::Expr::FieldAccess(field_expr) => {
            let path = &field_expr.path;
            if path.is_empty() {
                return quote! { 0 };
            }
            let field_name = syn::Ident::new(&path[0], proc_macro2::Span::call_site());
            quote! { #model_ident::#field_name }
        }
        crate::Expr::MethodCall(method_expr) => {
            let method = syn::Ident::new(&method_expr.method, proc_macro2::Span::call_site());
            let receiver = generate_expr(&method_expr.receiver, model_ident);
            quote! { #receiver.#method() }
        }
        crate::Expr::BinaryOp(bin_expr) => {
            let left = generate_expr(&bin_expr.left, model_ident);
            let right = generate_expr(&bin_expr.right, model_ident);
            let op = generate_binary_op(&bin_expr.op);
            quote! { #left #op #right }
        }
        crate::Expr::Literal(lit_expr) => match lit_expr {
            crate::LiteralExpr::Integer(i) => quote! { #i },
            crate::LiteralExpr::Float(f) => quote! { #f },
            crate::LiteralExpr::Bool(b) => quote! { #b },
            crate::LiteralExpr::String(s) => quote! { #s.to_string() },
        },
        _ => quote! { 0 },
    }
}

fn generate_binary_op(op: &crate::BinaryOp) -> TokenStream {
    match op {
        crate::BinaryOp::Eq => quote! { == },
        crate::BinaryOp::Ne => quote! { != },
        crate::BinaryOp::Lt => quote! { < },
        crate::BinaryOp::Le => quote! { <= },
        crate::BinaryOp::Gt => quote! { > },
        crate::BinaryOp::Ge => quote! { >= },
        crate::BinaryOp::And => quote! { && },
        crate::BinaryOp::Or => quote! { || },
        crate::BinaryOp::Add => quote! { + },
        crate::BinaryOp::Sub => quote! { - },
        crate::BinaryOp::Mul => quote! { * },
        crate::BinaryOp::Div => quote! { / },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse;

    #[test]
    fn test_view_generation() {
        let xml = r#"<column><text value="Hello" /></column>"#;
        let doc = parse(xml).unwrap();

        let result = generate_view(&doc, "Model", "Message").unwrap();
        let code = result.to_string();

        assert!(code.contains("fn") && code.contains("view"));
        assert!(code.contains("text"));
    }
}
