//! Build script for Dampen projects
//!
//! This build script scans the ui/ directory for .dampen files and generates
//! Rust code from them for production builds using handler metadata from
//! the `inventory_handlers!` macro.

fn main() {
    // Only generate code in codegen mode
    #[cfg(feature = "codegen")]
    {
        generate_ui_code();
    }

    #[cfg(not(feature = "codegen"))]
    {
        // Interpreted mode - no code generation needed
        // The #[dampen_ui] macro handles runtime XML loading
        println!("cargo:rerun-if-changed=src/ui/");
    }
}

#[cfg(feature = "codegen")]
fn generate_ui_code() {
    use dampen_core::codegen::{generate_application_with_theme_and_subscriptions, inventory};
    use dampen_core::parser;
    use dampen_core::parser::theme_parser::parse_theme_document;
    use dampen_core::parser::theme_parser::parse_theme_document;
    use std::env;
    use std::fs;
    use std::path::{Path, PathBuf};

    // Get output directory for generated code
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR not set");
    let out_path = PathBuf::from(&out_dir);

    // Find all .dampen files in src/ui/
    let ui_dir = PathBuf::from("src/ui");
    if !ui_dir.exists() {
        eprintln!("Warning: src/ui/ directory not found, skipping code generation");
        return;
    }

    println!("cargo:rerun-if-changed=src/ui/");

    // Parse theme file if it exists
    let theme_path = ui_dir.join("theme/theme.dampen");
    let theme_document = if theme_path.exists() {
        println!("cargo:rerun-if-changed={}", theme_path.display());
        match fs::read_to_string(&theme_path) {
            Ok(content) => match parse_theme_document(&content) {
                Ok(doc) => Some(doc),
                Err(e) => {
                    eprintln!("Warning: Failed to parse theme file: {}", e);
                    None
                }
            },
            Err(e) => {
                eprintln!("Warning: Failed to read theme file: {}", e);
                None
            }
        }
    } else {
        None
    };

    // Find all .dampen files
    let dampen_files = find_dampen_files(&ui_dir);

    if dampen_files.is_empty() {
        eprintln!("Warning: No .dampen files found in src/ui/");
        return;
    }

    // Generate code for each .dampen file
    let mut all_generated = String::new();
    all_generated.push_str("// Auto-generated - DO NOT EDIT\n");
    all_generated.push_str("// Regenerate with: cargo build --features codegen\n\n");

    for dampen_file in &dampen_files {
        println!("cargo:rerun-if-changed={}", dampen_file.display());

        // Find corresponding .rs file (same name)
        let rs_file = dampen_file.with_extension("rs");
        if !rs_file.exists() {
            eprintln!(
                "Warning: No corresponding .rs file found for {}",
                dampen_file.display()
            );
            continue;
        }

        println!("cargo:rerun-if-changed={}", rs_file.display());

        // Extract handler names from the .rs file using inventory_handlers! macro
        let handler_names = inventory::extract_handler_names_from_file(&rs_file);

        if handler_names.is_empty() {
            eprintln!(
                "Warning: No handlers found in {}. Did you forget to add inventory_handlers! macro?",
                rs_file.display()
            );
            // In strict mode, we continue but warn - handlers might be optional for this view
        }

        // Convert handler names to HandlerSignature objects
        // Note: For now, we create simple signatures. In the future, we could extract
        // full metadata from the _HANDLER_METADATA_* constants
        let handlers: Vec<_> = handler_names
            .iter()
            .map(|name| dampen_core::HandlerSignature {
                name: name.clone(),
                param_type: None,
                returns_command: false,
            })
            .collect();

        // Read and parse the .dampen file
        let dampen_content = match fs::read_to_string(&dampen_file) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Error: Failed to read {}: {}", dampen_file.display(), e);
                continue;
            }
        };

        let document = match parser::parse(&dampen_content) {
            Ok(doc) => doc,
            Err(e) => {
                eprintln!("Error: Failed to parse {}: {}", dampen_file.display(), e);
                continue;
            }
        };

        // Generate the application code
        let output = match generate_application_with_theme_and_subscriptions(
            &document,
            "Model",
            "Message",
            &handlers,
            theme_document.as_ref(),
        ) {
            Ok(output) => output,
            Err(e) => {
                eprintln!(
                    "Error: Code generation failed for {}: {}",
                    dampen_file.display(),
                    e
                );
                continue;
            }
        };

        // Add module for this generated code
        let module_name = dampen_file
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown");

        all_generated.push_str(&format!("// Generated from: {}\n", dampen_file.display()));
        all_generated.push_str(&format!("pub mod {} {{\n", module_name));
        all_generated.push_str(&output.code);
        all_generated.push_str("\n}\n\n");

        // Print any warnings
        for warning in &output.warnings {
            println!("cargo:warning={}: {}", dampen_file.display(), warning);
        }
    }

    // Write generated code
    let output_file = out_path.join("ui_generated.rs");
    if let Err(e) = fs::write(&output_file, &all_generated) {
        eprintln!("Error: Failed to write generated code: {}", e);
        return;
    }

    // Expose path to generated code
    println!("cargo:rustc-env=DAMPEN_GENERATED={}", output_file.display());

    println!(
        "Generated UI code successfully for {} files",
        dampen_files.len()
    );
}

/// Find all .dampen files in a directory recursively
#[cfg(feature = "codegen")]
fn find_dampen_files(dir: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();

    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                files.extend(find_dampen_files(&path));
            } else if path.extension().and_then(|s| s.to_str()) == Some("dampen") {
                files.push(path);
            }
        }
    }

    files
}
