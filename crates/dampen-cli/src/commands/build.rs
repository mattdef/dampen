#![allow(clippy::print_stderr, clippy::print_stdout)]

//! Build command - generates production Rust code from Dampen UI files

use std::path::Path;

/// Build command arguments
#[derive(clap::Args)]
pub struct BuildArgs {
    /// Input directory containing .dampen files (default: ui/)
    #[arg(short, long, default_value = "ui")]
    input: String,

    /// Output file for generated code (default: src/ui_generated.rs)
    #[arg(short, long, default_value = "src/ui_generated.rs")]
    output: String,

    /// Model struct name (default: Model)
    #[arg(long, default_value = "Model")]
    model: String,

    /// Message enum name (default: Message)
    #[arg(long, default_value = "Message")]
    message: String,

    /// Verbose output
    #[arg(short, long)]
    verbose: bool,

    /// Package to build (if workspace has multiple packages)
    #[arg(short, long)]
    package: Option<String>,

    /// Additional features to enable (beyond codegen)
    #[arg(long, value_delimiter = ',')]
    features: Vec<String>,
}

/// Execute the build command
///
/// Builds the application in debug mode with codegen.
///
/// # Mode Behavior
///
/// - **Debug Mode with Codegen**: Compile-time code generation without optimizations
/// - Faster compilation than release mode
/// - Useful for debugging production codegen behavior
/// - Use `dampen release` for optimized production builds
///
/// # Examples
///
/// ```bash
/// # Basic debug build with codegen
/// dampen build
///
/// # Build specific package in workspace
/// dampen build -p my-app
///
/// # Enable additional features
/// dampen build --features tokio,logging
/// ```
pub fn execute(args: &BuildArgs) -> Result<(), String> {
    // Run debug build with codegen
    execute_production_build(args)
}

/// Execute production build using cargo build
fn execute_production_build(args: &BuildArgs) -> Result<(), String> {
    use std::process::Command;

    if args.verbose {
        eprintln!("Running debug build with codegen...");
    }

    // Check if build.rs exists
    if !Path::new("build.rs").exists() {
        return Err(
            "build.rs not found. This project may not be configured for codegen builds."
                .to_string(),
        );
    }

    // Check if Cargo.toml exists
    if !Path::new("Cargo.toml").exists() {
        return Err("Cargo.toml not found. Are you in a Rust project directory?".to_string());
    }

    // Build the cargo command
    let mut cmd = Command::new("cargo");
    cmd.arg("build");

    // Add package specification if provided
    if let Some(ref package) = args.package {
        cmd.arg("-p").arg(package);
    }

    if args.verbose {
        cmd.arg("--verbose");
    }

    // Build features list: always include 'codegen', plus any user-specified features
    let mut all_features = vec!["codegen".to_string()];
    all_features.extend(args.features.clone());

    // Add features flag
    cmd.arg("--features").arg(all_features.join(","));

    // Execute cargo build
    if args.verbose {
        let features_str = all_features.join(",");
        eprintln!("Executing: cargo build --features {}", features_str);
    }

    let status = cmd
        .status()
        .map_err(|e| format!("Failed to execute cargo: {}", e))?;

    if !status.success() {
        return Err("Build failed".to_string());
    }

    if args.verbose {
        eprintln!("Build successful! Binary is in target/debug/");
    }

    eprintln!("Debug build completed successfully!");
    eprintln!("Use 'dampen release' for optimized production builds.");
    Ok(())
}
