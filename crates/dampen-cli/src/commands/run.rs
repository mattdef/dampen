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
/// This command launches the application:
/// - Debug mode (default): Interpreted mode with hot-reload support
/// - Release mode (--release): Codegen mode with full optimizations
///
/// # Mode Behavior
///
/// - **Interpreted Mode** (default): Runtime XML parsing with hot-reload support
/// - **Codegen Mode** (--release): Compile-time code generation, zero runtime overhead
///
/// # Examples
///
/// ```bash
/// # Debug mode (interpreted with hot-reload)
/// dampen run
///
/// # Release mode (codegen optimized)
/// dampen run --release
///
/// # Run specific package
/// dampen run -p my-app
///
/// # Pass application arguments
/// dampen run -- --window-size 800x600
///
/// # Enable additional features
/// dampen run --features tokio,logging
/// ```
pub fn execute(args: &RunArgs) -> Result<(), String> {
    // Run checks first (strict=false so warnings don't block, but errors do)
    if args.verbose {
        eprintln!("Running pre-flight checks...");
    }

    // Resolve UI directory based on package argument if present
    let check_input = if let Some(ref pkg) = args.package {
        crate::commands::check::resolve_package_ui_path(pkg)
            .map(|p| p.to_string_lossy().to_string())
    } else {
        None
    };

    if let Err(e) = crate::commands::check::run_checks(check_input, false, args.verbose) {
        return Err(format!("Pre-flight check failed: {}", e));
    }

    // Check if Cargo.toml exists

    if !Path::new("Cargo.toml").exists() {
        return Err("Cargo.toml not found. Are you in a Rust project directory?".to_string());
    }

    let mode = if args.release {
        "codegen"
    } else {
        "interpreted"
    };

    if args.verbose {
        eprintln!("Running in {} mode...", mode);
        eprintln!(
            "Profile: {}",
            if args.release { "release" } else { "debug" }
        );
    }

    // For release mode, we need to build first then run
    if args.release {
        // Check build.rs exists for codegen mode
        // If package is specified, also check in examples/ directory
        let build_rs_exists = Path::new("build.rs").exists()
            || args
                .package
                .as_ref()
                .is_some_and(|pkg| Path::new("examples").join(pkg).join("build.rs").exists());

        if !build_rs_exists {
            return Err(
                "build.rs not found. Codegen mode requires build.rs for code generation.\n\
                 Tip: Use 'dampen run' (without --release) for interpreted mode,\n\
                 or ensure you're in the correct project directory."
                    .to_string(),
            );
        }

        // Build in release mode with codegen
        let mut build_cmd = Command::new("cargo");
        build_cmd.arg("build");

        if let Some(ref package) = args.package {
            build_cmd.arg("-p").arg(package);
        }

        if args.verbose {
            build_cmd.arg("--verbose");
        }

        // Release mode: codegen with optimizations
        build_cmd.arg("--release");
        build_cmd.arg("--no-default-features");

        let mut features = vec!["codegen".to_string()];
        features.extend(args.features.clone());

        build_cmd.arg("--features").arg(features.join(","));

        if args.verbose {
            let features_str = features.join(",");
            eprintln!(
                "Building: cargo build --release --no-default-features --features {}",
                features_str
            );
        }

        let build_status = build_cmd
            .status()
            .map_err(|e| format!("Failed to execute cargo build: {}", e))?;

        if !build_status.success() {
            return Err("Build failed".to_string());
        }

        if args.verbose {
            eprintln!("Build successful! Now running application...");
        }

        // Run the built binary
        let mut run_cmd = Command::new("cargo");
        run_cmd.arg("run");

        if let Some(ref package) = args.package {
            run_cmd.arg("-p").arg(package);
        }

        if args.verbose {
            run_cmd.arg("--verbose");
        }

        // In release mode, run release binary
        run_cmd.arg("--release");

        // Add application arguments if provided
        if !args.app_args.is_empty() {
            run_cmd.arg("--");
            run_cmd.args(&args.app_args);
        }

        if args.verbose {
            eprintln!("Running: cargo run --release");
            if !args.app_args.is_empty() {
                eprintln!("Application args: {:?}", args.app_args);
            }
        }

        let run_status = run_cmd
            .status()
            .map_err(|e| format!("Failed to execute cargo run: {}", e))?;

        if !run_status.success() {
            return Err("Run command failed".to_string());
        }

        Ok(())
    } else {
        // Debug mode: use interpreted mode
        let mut cmd = Command::new("cargo");
        cmd.arg("run");

        if let Some(ref package) = args.package {
            cmd.arg("-p").arg(package);
        }

        if args.verbose {
            cmd.arg("--verbose");
        }

        // Debug mode: use interpreted
        let mut features = vec!["interpreted".to_string()];
        features.extend(args.features.clone());

        cmd.arg("--features").arg(features.join(","));

        // Add application arguments if provided
        if !args.app_args.is_empty() {
            cmd.arg("--");
            cmd.args(&args.app_args);
        }

        // Execute cargo run
        if args.verbose {
            let features_str = features.join(",");
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
}
