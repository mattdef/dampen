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

    /// Release build (use cargo build --release)
    #[arg(long)]
    release: bool,
}

/// Execute the build command
pub fn execute(args: &BuildArgs) -> Result<(), String> {
    // By default, run production build (like cargo build)
    execute_production_build(args)
}

/// Execute production build using cargo build
fn execute_production_build(args: &BuildArgs) -> Result<(), String> {
    use std::process::Command;

    if args.verbose {
        eprintln!("Running production build...");
        eprintln!("Mode: {}", if args.release { "release" } else { "debug" });
    }

    // Check if build.rs exists
    if !Path::new("build.rs").exists() {
        return Err(
            "build.rs not found. This project may not be configured for production builds."
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

    if args.release {
        cmd.arg("--release");
    }

    if args.verbose {
        cmd.arg("--verbose");
    }

    // Execute cargo build
    if args.verbose {
        eprintln!(
            "Executing: cargo build{}",
            if args.release { " --release" } else { "" }
        );
    }

    let status = cmd
        .status()
        .map_err(|e| format!("Failed to execute cargo: {}", e))?;

    if !status.success() {
        return Err("Build failed".to_string());
    }

    if args.verbose {
        let binary_path = if args.release {
            "target/release"
        } else {
            "target/debug"
        };
        eprintln!("Build successful! Binary is in {}/", binary_path);
    }

    eprintln!("Production build completed successfully!");
    Ok(())
}
