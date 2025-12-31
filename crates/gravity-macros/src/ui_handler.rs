use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, FnArg, ItemFn, ReturnType, Type};

/// Attribute macro to mark event handlers
///
/// # Supported Signatures
///
/// ## Simple handler
/// ```ignore
/// #[ui_handler]
/// fn increment(model: &mut Model) {
///     model.count += 1;
/// }
/// ```
///
/// ## Handler with value parameter
/// ```ignore
/// #[ui_handler]
/// fn update_name(model: &mut Model, value: String) {
///     model.name = value;
/// }
/// ```
///
/// ## Handler with Command return
/// ```ignore
/// #[ui_handler]
/// fn fetch_data(model: &mut Model) -> Command<Message> {
///     Command::perform(...)
/// }
/// ```
pub fn ui_handler(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);

    // Validate and extract handler information
    let handler_info = match validate_and_extract_handler(&input_fn) {
        Ok(info) => info,
        Err(err) => return err.to_compile_error().into(),
    };

    // Generate the handler registration code
    let expanded = generate_handler_code(&input_fn, &handler_info);

    expanded.into()
}

/// Information extracted from a handler function
struct HandlerInfo {
    _handler_name: String,
    model_type: Type,
    signature_type: HandlerSignatureType,
}

enum HandlerSignatureType {
    Simple,
    WithValue(Type),   // The value parameter type
    WithCommand(Type), // The Command<Message> type
}

/// Validates handler signature and extracts information
fn validate_and_extract_handler(sig: &ItemFn) -> Result<HandlerInfo, syn::Error> {
    let signature = &sig.sig;

    // Must be a function
    if signature.constness.is_some() {
        return Err(syn::Error::new_spanned(
            signature.constness,
            "Handler cannot be const",
        ));
    }

    // Check for async - handlers can be async but return Command
    let is_async = signature.asyncness.is_some();

    // Extract parameters
    let mut params = signature.inputs.iter();

    // First parameter must be &mut Model
    let first_param = params.next().ok_or_else(|| {
        syn::Error::new_spanned(
            signature,
            "Handler must have at least one parameter: &mut Model",
        )
    })?;

    let model_type = extract_model_type(first_param)?;

    // Check second parameter and return type
    let second_param = params.next();
    let return_type = &signature.output;

    // Determine signature type
    let signature_type = match (second_param, return_type, is_async) {
        // Simple: no second param, no return
        (None, ReturnType::Default, false) => HandlerSignatureType::Simple,

        // With value: has second param, no return
        (Some(param), ReturnType::Default, false) => {
            let value_type = extract_value_type(param)?;
            HandlerSignatureType::WithValue(value_type)
        }

        // With command: no second param, has return
        (None, ReturnType::Type(_, ret_type), false) => {
            let cmd_type = extract_command_type(ret_type)?;
            HandlerSignatureType::WithCommand(cmd_type)
        }

        // Async with command
        (None, ReturnType::Type(_, ret_type), true) => {
            let cmd_type = extract_command_type(ret_type)?;
            HandlerSignatureType::WithCommand(cmd_type)
        }

        _ => {
            return Err(syn::Error::new_spanned(
                signature,
                "Invalid handler signature. Supported:\n\
                 - fn(&mut Model)\n\
                 - fn(&mut Model, T)\n\
                 - fn(&mut Model) -> Command<Message>",
            ));
        }
    };

    let handler_name = signature.ident.to_string();

    Ok(HandlerInfo {
        _handler_name: handler_name,
        model_type,
        signature_type,
    })
}

/// Extract model type from first parameter
fn extract_model_type(arg: &FnArg) -> Result<Type, syn::Error> {
    match arg {
        FnArg::Typed(pat_type) => {
            // Check it's &mut Model
            if let Type::Reference(type_ref) = &*pat_type.ty {
                if type_ref.mutability.is_some() {
                    return Ok((*type_ref.elem).clone());
                }
            }
            Err(syn::Error::new_spanned(
                arg,
                "First parameter must be &mut Model",
            ))
        }
        FnArg::Receiver(_) => Err(syn::Error::new_spanned(
            arg,
            "Handler cannot be a method (must be free function)",
        )),
    }
}

/// Extract value type from second parameter
fn extract_value_type(arg: &FnArg) -> Result<Type, syn::Error> {
    match arg {
        FnArg::Typed(pat_type) => Ok((*pat_type.ty).clone()),
        FnArg::Receiver(_) => Err(syn::Error::new_spanned(
            arg,
            "Second parameter must be a value type",
        )),
    }
}

/// Extract Command type from return type
fn extract_command_type(ret_type: &Type) -> Result<Type, syn::Error> {
    // For now, just return the type as-is
    // In a full implementation, we'd validate it's Command<Message>
    Ok(ret_type.clone())
}

/// Generate the handler code
fn generate_handler_code(input_fn: &ItemFn, info: &HandlerInfo) -> proc_macro2::TokenStream {
    let fn_name = &input_fn.sig.ident;
    let fn_name_str = fn_name.to_string();
    let model_type = &info.model_type;

    // Generate unique static name
    let static_name = syn::Ident::new(
        &format!("__GRAVITY_HANDLER_{}", fn_name_str.to_uppercase()),
        fn_name.span(),
    );

    // Generate the original function
    let original_fn = quote! {
        #input_fn
    };

    // Generate registration code based on signature type
    let registration = match &info.signature_type {
        HandlerSignatureType::Simple => {
            quote! {
                // Register simple handler at module initialization
                #[allow(non_upper_case_globals)]
                static #static_name: () = {
                    use std::any::Any;
                    use gravity_core::handler::HandlerRegistry;

                    // This will be called when the registry is initialized
                    fn register(registry: &HandlerRegistry) {
                        registry.register_simple(
                            #fn_name_str,
                            |model: &mut dyn Any| {
                                let model = model.downcast_mut::<#model_type>().unwrap();
                                #fn_name(model);
                            }
                        );
                    }
                };
            }
        }
        HandlerSignatureType::WithValue(value_type) => {
            quote! {
                #[allow(non_upper_case_globals)]
                static #static_name: () = {
                    use std::any::Any;
                    use gravity_core::handler::HandlerRegistry;

                    fn register(registry: &HandlerRegistry) {
                        registry.register_with_value(
                            #fn_name_str,
                            |model: &mut dyn Any, value: Box<dyn Any>| {
                                let model = model.downcast_mut::<#model_type>().unwrap();
                                let value = *value.downcast::<#value_type>().unwrap();
                                #fn_name(model, value);
                            }
                        );
                    }
                };
            }
        }
        HandlerSignatureType::WithCommand(_cmd_type) => {
            quote! {
                #[allow(non_upper_case_globals)]
                static #static_name: () = {
                    use std::any::Any;
                    use gravity_core::handler::HandlerRegistry;

                    fn register(registry: &HandlerRegistry) {
                        registry.register_with_command(
                            #fn_name_str,
                            |model: &mut dyn Any| -> Box<dyn Any> {
                                let model = model.downcast_mut::<#model_type>().unwrap();
                                let command = #fn_name(model);
                                Box::new(command)
                            }
                        );
                    }
                };
            }
        }
    };

    // Combine original function with registration
    quote! {
        #original_fn

        #registration
    }
}
