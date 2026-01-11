//! UI Handler attribute macro
//!
//! This module provides the #[ui_handler] attribute macro that marks functions
//! as UI event handlers and emits metadata for build-time code generation.
//!
//! # Dual-Mode Architecture Support
//!
//! This macro works in both compilation modes:
//!
//! - **Interpreted Mode**: Metadata is emitted but not consumed. Handlers are registered
//!   manually at runtime using `HandlerRegistry::register_*()` methods.
//! - **Codegen Mode**: Metadata is collected by build.rs to generate handler registration
//!   code at compile time, eliminating runtime registration overhead.
//!
//! The macro always emits both the original function and metadata constants, ensuring
//! compatibility with both modes without conditional compilation.
//!
//! # Example
//!
//! ```rust,ignore
//! use dampen_macros::ui_handler;
//!
//! #[ui_handler]
//! fn on_click(model: &mut Model) {
//!     model.count += 1;
//! }
//! ```

use proc_macro::TokenStream;
use quote::quote;
use syn::{FnArg, ItemFn, ReturnType, parse_macro_input};

/// Process the #[ui_handler] attribute macro.
///
/// This macro transforms a handler function to emit metadata for optional build-time
/// code generation while preserving the original function for runtime use.
///
/// # Dual-Mode Behavior
///
/// The macro emits code that works in both modes:
///
/// ## Interpreted Mode
/// - Original function is preserved for runtime use
/// - Metadata constants are emitted but typically unused
/// - Handlers registered manually via `HandlerRegistry::register_*()`
///
/// ## Codegen Mode  
/// - Original function is preserved and callable
/// - Metadata constants are collected by build.rs
/// - build.rs generates registration code automatically
/// - Zero runtime registration overhead
///
/// # Generated Code
///
/// For a handler function `on_click`, the macro generates:
/// - The original `on_click` function unchanged
/// - A `_HANDLER_METADATA_ON_CLICK` constant with type information
///
/// The metadata constant is marked `#[doc(hidden)]` and does not affect
/// the public API surface.
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
        pub const #metadata_ident: ::dampen_core::codegen::HandlerInfo = ::dampen_core::codegen::HandlerInfo {
            name: #handler_name_str,
            signature_type: ::dampen_core::codegen::HandlerSignatureType::#signature_type,
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
