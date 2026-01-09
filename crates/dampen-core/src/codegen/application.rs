//! Application trait generation

use proc_macro2::TokenStream;
use quote::quote;

/// Generate Application trait implementation
pub fn generate_application_trait(
    model_name: &str,
    message_name: &str,
) -> Result<TokenStream, super::CodegenError> {
    let model_ident = syn::Ident::new(model_name, proc_macro2::Span::call_site());
    let message_ident = syn::Ident::new(message_name, proc_macro2::Span::call_site());

    Ok(quote! {
        impl iced::Application for #model_ident {
            type Executor = iced::executor::Default;
            type Flags = ();
            type Message = #message_ident;

            fn new(_flags: ()) -> (Self, iced::Task<Self::Message>) {
                (Self::default(), iced::Task::none())
            }

            fn title(&self) -> String {
                "Gravity Application".to_string()
            }
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_application_trait_generation() {
        let result = generate_application_trait("MyModel", "MyMessage").unwrap();
        let code = result.to_string();

        assert!(code.contains("impl") && code.contains("Application") && code.contains("MyModel"));
        assert!(code.contains("Message") && code.contains("MyMessage"));
    }
}
