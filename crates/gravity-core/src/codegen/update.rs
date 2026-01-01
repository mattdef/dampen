//! Update function generation

use crate::{GravityDocument, HandlerSignature};
use proc_macro2::TokenStream;
use quote::quote;

/// Generate the update function
pub fn generate_update(
    _document: &GravityDocument,
    handlers: &[HandlerSignature],
    _model_name: &str,
    message_name: &str,
) -> Result<TokenStream, super::CodegenError> {
    let message_ident = syn::Ident::new(message_name, proc_macro2::Span::call_site());

    let match_arms: Vec<TokenStream> = handlers
        .iter()
        .map(|handler| {
            let handler_name = syn::Ident::new(&handler.name, proc_macro2::Span::call_site());

            if let Some(_param_type) = &handler.param_type {
                // Handler with value parameter
                quote! {
                    #message_ident::#handler_name(value) => {
                        #handler_name(self, value);
                        iced::Task::none()
                    }
                }
            } else if handler.returns_command {
                // Handler returning command
                quote! {
                    #message_ident::#handler_name => {
                        #handler_name(self)
                    }
                }
            } else {
                // Simple handler
                quote! {
                    #message_ident::#handler_name => {
                        #handler_name(self);
                        iced::Task::none()
                    }
                }
            }
        })
        .collect();

    Ok(quote! {
        fn update(&mut self, message: Self::Message) -> iced::Task<Self::Message> {
            match message {
                #(#match_arms)*
            }
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::HandlerSignature;

    #[test]
    fn test_update_generation() {
        let handlers = vec![HandlerSignature {
            name: "increment".to_string(),
            param_type: None,
            returns_command: false,
        }];

        let result = generate_update(
            &crate::GravityDocument::default(),
            &handlers,
            "Model",
            "Message",
        )
        .unwrap();

        let code = result.to_string();
        assert!(code.contains("fn") && code.contains("update"));
        assert!(code.contains("increment"));
    }
}
