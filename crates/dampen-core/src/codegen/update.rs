//! Update function generation

use crate::HandlerSignature;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

/// Convert snake_case to UpperCamelCase
fn to_upper_camel_case(s: &str) -> String {
    let mut result = String::new();
    let mut capitalize_next = true;
    for c in s.chars() {
        if c == '_' {
            capitalize_next = true;
        } else if capitalize_next {
            result.push(c.to_ascii_uppercase());
            capitalize_next = false;
        } else {
            result.push(c);
        }
    }
    result
}

/// Generate the update match arms for use in a standalone function
pub fn generate_update_match_arms(
    handlers: &[HandlerSignature],
    message_name: &str,
) -> Result<TokenStream, super::CodegenError> {
    let message_ident = format_ident!("{}", message_name);

    let match_arms: Vec<TokenStream> = handlers
        .iter()
        .map(|handler| {
            let handler_name = format_ident!("{}", handler.name);
            let variant_name = to_upper_camel_case(&handler.name);
            let variant_ident = syn::Ident::new(&variant_name, proc_macro2::Span::call_site());

            if let Some(_param_type) = &handler.param_type {
                quote! {
                    #message_ident::#variant_ident(value) => {
                        ui::window::#handler_name(model, value);
                        iced::Task::none()
                    }
                }
            } else if handler.returns_command {
                quote! {
                    #message_ident::#variant_ident => {
                        ui::window::#handler_name(model)
                    }
                }
            } else {
                quote! {
                    #message_ident::#variant_ident => {
                        ui::window::#handler_name(model);
                        iced::Task::none()
                    }
                }
            }
        })
        .collect();

    Ok(quote! {
        match message {
            #(#match_arms)*
        }
    })
}
