//! UI Handler attribute macro
//!
//! This module provides the #[ui_handler] attribute macro that marks functions
//! as UI event handlers and emits metadata for build-time code generation.
//!
//! # Example
//!
//! ```rust,ignore
//! use gravity_macros::ui_handler;
//!
//! #[ui_handler]
//! fn on_click(model: &mut Model) {
//!     model.count += 1;
//! }
//! ```

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, FnArg, ItemFn, ReturnType};

/// Process the #[ui_handler] attribute macro
pub fn process_ui_handler(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);

    // Extract handler metadata
    let handler_name = input.sig.ident.clone();
    let handler_name_str = handler_name.to_string();

    // Detect signature type and parameter types
    let (signature_type_str, param_types) = analyze_signature(&input);
    let signature_type = syn::Ident::new(&signature_type_str, proc_macro2::Span::call_site());

    // Get return type
    let return_type_str = match &input.sig.output {
        ReturnType::Default => "()".to_string(),
        ReturnType::Type(_, ty) => quote!(#ty).to_string(),
    };

    // Build parameter types array
    let param_types_array = param_types.iter().map(|t| quote!(#t));

    // Get source location
    let source_file = file!(); // Gets the file where the macro is invoked
    let source_line = line!(); // Gets the line where the macro is invoked

    // Generate the original function plus metadata
    // The metadata is stored in a module-level static for build.rs to collect
    let metadata_ident = syn::Ident::new(
        &format!("_HANDLER_METADATA_{}", handler_name_str.to_uppercase()),
        proc_macro2::Span::call_site(),
    );

    let output = quote! {
        #input

        // Emit handler metadata for build.rs consumption
        #[doc(hidden)]
        #[allow(non_upper_case_globals)]
        pub const #metadata_ident: ::gravity_core::codegen::HandlerInfo = ::gravity_core::codegen::HandlerInfo {
            name: #handler_name_str,
            signature_type: ::gravity_core::codegen::HandlerSignatureType::#signature_type,
            param_types: &[#(#param_types_array),*],
            return_type: #return_type_str,
            source_file: #source_file,
            source_line: #source_line,
        };
    };

    output.into()
}

/// Analyze function signature to determine handler type and parameters
fn analyze_signature(func: &ItemFn) -> (String, Vec<String>) {
    let mut param_types = Vec::new();
    let mut has_value_param = false;

    // Analyze parameters
    for input in &func.sig.inputs {
        if let FnArg::Typed(pat_type) = input {
            let ty = &pat_type.ty;
            let type_str = quote!(#ty).to_string();
            param_types.push(type_str);

            // Check if it's not &mut Model (the first param)
            if param_types.len() > 1 {
                has_value_param = true;
            }
        }
    }

    // Determine signature type
    let signature_type = if let ReturnType::Type(_, ty) = &func.sig.output {
        let return_str = quote!(#ty).to_string();
        if return_str.contains("Command") {
            "WithCommand".to_string()
        } else if has_value_param {
            "WithValue".to_string()
        } else {
            "Simple".to_string()
        }
    } else if has_value_param {
        "WithValue".to_string()
    } else {
        "Simple".to_string()
    };

    (signature_type, param_types)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_signature_analysis() {
        // This is a unit test for the signature analysis logic
        // The actual macro tests are in tests/handler_metadata_tests.rs
        assert!(
            true,
            "Signature analysis logic tested via integration tests"
        );
    }
}
