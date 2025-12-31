use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

pub fn ui_model_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;

    // Extract fields from the struct
    let fields = match &input.data {
        Data::Struct(data) => &data.fields,
        _ => {
            return syn::Error::new_spanned(input.ident, "UiModel can only be derived for structs")
                .to_compile_error()
                .into();
        }
    };

    // Generate get_field implementation
    let get_field_impl = generate_get_field(name, fields);

    // Generate available_fields implementation
    let available_fields_impl = generate_available_fields(name, fields);

    let expanded = quote! {
        impl gravity_core::binding::UiBindable for #name {
            #get_field_impl
            #available_fields_impl
        }
    };

    expanded.into()
}

fn generate_get_field(_name: &syn::Ident, fields: &Fields) -> proc_macro2::TokenStream {
    let mut arms = Vec::new();

    if let Fields::Named(fields_named) = fields {
        for field in &fields_named.named {
            let field_name = match field.ident.as_ref() {
                Some(name) => name,
                None => continue,
            };
            let field_name_str = field_name.to_string();

            // Check for #[ui_skip] and #[ui_bind] attributes
            let skip = field
                .attrs
                .iter()
                .any(|attr| attr.path().is_ident("ui_skip"));
            let bind = field
                .attrs
                .iter()
                .any(|attr| attr.path().is_ident("ui_bind"));

            if skip && !bind {
                continue;
            }

            // Generate single-field match arm
            arms.push(quote! {
                [#field_name_str] => {
                    use gravity_core::binding::ToBindingValue;
                    Some(gravity_core::binding::BindingValue::from_value(&self.#field_name))
                }
            });

            // Generate nested-field match arms (for fields that are UiBindable)
            // This is a simplified approach - in reality, we'd need to detect if the type
            // implements UiBindable, which requires more complex macro logic
            // For now, we'll just handle single-level paths
        }
    }

    quote! {
        fn get_field(&self, path: &[&str]) -> Option<gravity_core::binding::BindingValue> {
            if path.is_empty() {
                return None;
            }

            match path {
                #(#arms)*
                _ => None,
            }
        }
    }
}

fn generate_available_fields(_name: &syn::Ident, fields: &Fields) -> proc_macro2::TokenStream {
    let field_names: Vec<String> = match fields {
        Fields::Named(fields_named) => fields_named
            .named
            .iter()
            .filter_map(|field| {
                let skip = field
                    .attrs
                    .iter()
                    .any(|attr| attr.path().is_ident("ui_skip"));

                let bind = field
                    .attrs
                    .iter()
                    .any(|attr| attr.path().is_ident("ui_bind"));

                if skip && !bind {
                    None
                } else {
                    let field_name = field.ident.as_ref()?;
                    Some(field_name.to_string())
                }
            })
            .collect(),
        _ => Vec::new(),
    };

    quote! {
        fn available_fields() -> Vec<String> {
            vec![#(#field_names.to_string()),*]
        }
    }
}
