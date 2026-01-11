#![allow(clippy::print_stderr, clippy::print_stdout)]

//! Run command - launches development mode with interpreted execution
//!
//! This command wraps `cargo run` with the `interpreted` feature flag enabled,
//! providing fast iteration with hot-reload capabilities.

use std::path::Path;
use std::process::Command;

/// Run command arguments
#[derive(clap::Args)]
pub struct RunArgs {
    /// Package to run (if workspace has multiple binaries)
    #[arg(short, long)]
    package: Option<String>,

    /// Arguments to pass to the application
    #[arg(last = true)]
    app_args: Vec<String>,

    /// Verbose output
    #[arg(short, long)]
    verbose: bool,

    /// Run with release optimizations
    #[arg(long)]
    release: bool,

    /// Additional features to enable (beyond interpreted)
    #[arg(long, value_delimiter = ',')]
    features: Vec<String>,
}

/// Execute the run command
///
/// This command launches the application in development/interpreted mode
/// by invoking `cargo run` with the `interpreted` feature flag.
///
/// # Mode Behavior
///
/// - **Interpreted Mode** (default): Runtime XML parsing with hot-reload support
/// - Fast compilation and iteration
/// - Preserves application state during UI changes
/// - Ideal for development and prototyping
///
/// # Examples
///
/// ```bash
/// # Basic run in interpreted mode
/// dampen run
///
/// # Run specific package in workspace
/// dampen run -p my-app
///
/// # Pass arguments to the application
/// dampen run -- --window-size 800x600
///
/// # Enable additional features
/// dampen run --features tokio,logging
/// ```
pub fn execute(args: &RunArgs) -> Result<(), String> {
    // Check if Cargo.toml exists
    if !Path::new("Cargo.toml").exists() {
        return Err("Cargo.toml not found. Are you in a Rust project directory?".to_string());
    }

    if args.verbose {
        eprintln!("Running in development mode (interpreted)...");
        eprintln!("Mode: {}", if args.release { "release" } else { "debug" });
    }

    // Build the cargo command
    let mut cmd = Command::new("cargo");
    cmd.arg("run");

    // Add package specification if provided
    if let Some(ref package) = args.package {
        cmd.arg("-p").arg(package);
    }

    // Add release flag if requested
    if args.release {
        cmd.arg("--release");
    }

    // Add verbose flag if requested
    if args.verbose {
        cmd.arg("--verbose");
    }

    // Build features list: always include 'interpreted', plus any user-specified features
    let mut all_features = vec!["interpreted".to_string()];
    all_features.extend(args.features.clone());

    // Add features flag
    cmd.arg("--features").arg(all_features.join(","));

    // Add application arguments if provided
    if !args.app_args.is_empty() {
        cmd.arg("--");
        cmd.args(&args.app_args);
    }

    // Execute cargo run
    if args.verbose {
        let features_str = all_features.join(",");
        eprintln!("Executing: cargo run --features {}", features_str);
        if !args.app_args.is_empty() {
            eprintln!("Application args: {:?}", args.app_args);
        }
    }

    let status = cmd
        .status()
        .map_err(|e| format!("Failed to execute cargo: {}", e))?;

    if !status.success() {
        return Err("Run command failed".to_string());
    }

    Ok(())
}
