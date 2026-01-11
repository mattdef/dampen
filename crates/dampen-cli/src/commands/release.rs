#![allow(clippy::print_stderr, clippy::print_stdout)]

//! Release command - builds production-optimized binaries with codegen mode
//!
//! This command wraps `cargo build --release` with the `codegen` feature flag,
//! providing optimized production builds with zero runtime overhead.

use std::path::Path;
use std::process::Command;

/// Release command arguments
#[derive(clap::Args)]
pub struct ReleaseArgs {
    /// Package to build (if workspace has multiple packages)
    #[arg(short, long)]
    package: Option<String>,

    /// Additional features to enable (beyond codegen)
    #[arg(long, value_delimiter = ',')]
    features: Vec<String>,

    /// Verbose output
    #[arg(short, long)]
    verbose: bool,

    /// Target directory for build artifacts
    #[arg(long)]
    target_dir: Option<String>,
}

/// Execute the release command
///
/// Builds the application in production mode with full optimizations.
///
/// # Mode Behavior
///
/// - **Release Mode**: Compile-time code generation with full optimizations
/// - Zero runtime overhead for maximum performance
/// - Ideal for production deployments
/// - Requires build.rs for code generation
///
/// # Examples
///
/// ```bash
/// # Basic release build
/// dampen release
///
/// # Build specific package in workspace
/// dampen release -p my-app
///
/// # Enable additional features
/// dampen release --features tokio,logging
///
/// # Custom target directory
/// dampen release --target-dir ./dist
/// ```
pub fn execute(args: &ReleaseArgs) -> Result<(), String> {
    // Check if Cargo.toml exists
    if !Path::new("Cargo.toml").exists() {
        return Err("Cargo.toml not found. Are you in a Rust project directory?".to_string());
    }

    // Check if build.rs exists
    if !Path::new("build.rs").exists() {
        return Err(
            "build.rs not found. This project may not be configured for production builds."
                .to_string(),
        );
    }

    if args.verbose {
        eprintln!("Building for production (release mode with codegen)...");
    }

    // Build the cargo command
    let mut cmd = Command::new("cargo");
    cmd.arg("build");
    cmd.arg("--release");

    // Add package specification if provided
    if let Some(ref package) = args.package {
        cmd.arg("-p").arg(package);
    }

    // Add target directory if provided
    if let Some(ref target_dir) = args.target_dir {
        cmd.arg("--target-dir").arg(target_dir);
    }

    if args.verbose {
        cmd.arg("--verbose");
    }

    // Build features list: always include 'codegen', plus user-specified features
    let mut all_features = vec!["codegen".to_string()];
    all_features.extend(args.features.clone());

    // Add features flag
    cmd.arg("--features").arg(all_features.join(","));

    // Execute cargo build --release
    if args.verbose {
        let features_str = all_features.join(",");
        eprintln!(
            "Executing: cargo build --release --features {}",
            features_str
        );
    }

    let status = cmd
        .status()
        .map_err(|e| format!("Failed to execute cargo: {}", e))?;

    if !status.success() {
        return Err("Release build failed".to_string());
    }

    if args.verbose {
        eprintln!("Release build successful! Binary is in target/release/");
    }

    eprintln!("Production build completed successfully!");
    Ok(())
}
