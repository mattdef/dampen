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

    /// Build in release mode with codegen
    #[arg(long)]
    release: bool,
}

/// Execute the build command
///
/// Builds the application:
/// - Debug mode (default): Interpreted mode with hot-reload support
/// - Release mode (--release): Codegen mode with full optimizations
///
/// # Mode Behavior
///
/// - **Debug Mode** (default): Interpreted mode, fast iteration for development
/// - **Release Mode** (--release): Codegen mode with zero runtime overhead
///
/// # Examples
///
/// ```bash
/// # Debug build (interpreted)
/// dampen build
///
/// # Release build (codegen optimized)
/// dampen build --release
///
/// # Build specific package
/// dampen build -p my-app
///
/// # Enable additional features
/// dampen build --features tokio,logging
/// ```
pub fn execute(args: &BuildArgs) -> Result<(), String> {
    execute_production_build(args)
}

fn execute_production_build(args: &BuildArgs) -> Result<(), String> {
    use std::process::Command;

    // Run checks first (strict=false so warnings don't block, but errors do)
    if args.verbose {
        eprintln!("Running pre-flight checks...");
    }

    // Use the input directory specified in args (default is "ui")
    // Or pass None if we want auto-discovery logic (but args.input has a default value)

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

    let mode = if args.release {
        "codegen"
    } else {
        "interpreted"
    };

    if args.verbose {
        eprintln!("Running {} build...", mode);
    }

    // Check build.rs only for codegen mode
    // If package is specified, also check in examples/ directory
    let build_rs_exists = Path::new("build.rs").exists()
        || args
            .package
            .as_ref()
            .is_some_and(|pkg| Path::new("examples").join(pkg).join("build.rs").exists());

    if args.release && !build_rs_exists {
        return Err(
            "build.rs not found. Codegen mode requires build.rs for code generation.\n\
             Tip: Use 'dampen build' (without --release) for interpreted mode,\n\
             or ensure you're in the correct project directory."
                .to_string(),
        );
    }

    if !Path::new("Cargo.toml").exists() {
        return Err("Cargo.toml not found. Are you in a Rust project directory?".to_string());
    }

    let mut cmd = Command::new("cargo");
    cmd.arg("build");

    if let Some(ref package) = args.package {
        cmd.arg("-p").arg(package);
    }

    if args.verbose {
        cmd.arg("--verbose");
    }

    // Build features list based on mode
    let all_features = if args.release {
        // Release mode: codegen with optimizations
        cmd.arg("--release");
        cmd.arg("--no-default-features");
        let mut features = vec!["codegen".to_string()];
        features.extend(args.features.clone());
        features
    } else {
        // Debug mode: interpreted (NEW BEHAVIOR)
        let mut features = vec!["interpreted".to_string()];
        features.extend(args.features.clone());
        features
    };

    cmd.arg("--features").arg(all_features.join(","));

    if args.verbose {
        let features_str = all_features.join(",");
        let cargo_cmd = if args.release {
            format!(
                "cargo build --release --no-default-features --features {}",
                features_str
            )
        } else {
            format!("cargo build --features {}", features_str)
        };
        eprintln!("Executing: {}", cargo_cmd);
    }

    let status = cmd
        .status()
        .map_err(|e| format!("Failed to execute cargo: {}", e))?;

    if !status.success() {
        return Err("Build failed".to_string());
    }

    if args.verbose {
        let output_dir = if args.release {
            "target/release/"
        } else {
            "target/debug/"
        };
        eprintln!("Build successful! Binary is in {}", output_dir);
    }

    if args.release {
        eprintln!("Release build (codegen) completed successfully!");
    } else {
        eprintln!("Debug build (interpreted) completed successfully!");
        eprintln!(
            "Use 'dampen build --release' or 'dampen release' for optimized production builds."
        );
    }

    Ok(())
}

/// Execute build command with release mode enabled
/// This is a helper function for the release command
pub fn execute_release_build(
    package: Option<String>,
    features: Vec<String>,
    verbose: bool,
    target_dir: Option<String>,
) -> Result<(), String> {
    use std::process::Command;

    // Run checks first
    if verbose {
        eprintln!("Running pre-flight checks...");
    }

    // Resolve UI directory based on package argument if present
    let check_input = if let Some(ref pkg) = package {
        crate::commands::check::resolve_package_ui_path(pkg)
            .map(|p| p.to_string_lossy().to_string())
    } else {
        None
    };

    if let Err(e) = crate::commands::check::run_checks(check_input, false, verbose) {
        return Err(format!("Pre-flight check failed: {}", e));
    }

    let mode = "codegen";

    if verbose {
        eprintln!("Running {} build...", mode);
    }

    // Check build.rs only for codegen mode
    // If package is specified, also check in examples/ directory
    let build_rs_exists = Path::new("build.rs").exists()
        || package
            .as_ref()
            .is_some_and(|pkg| Path::new("examples").join(pkg).join("build.rs").exists());

    if !build_rs_exists {
        return Err(
            "build.rs not found. Codegen mode requires build.rs for code generation.\n\
             Tip: Use 'dampen build' (without --release) for interpreted mode,\n\
             or ensure you're in the correct project directory."
                .to_string(),
        );
    }

    if !Path::new("Cargo.toml").exists() {
        return Err("Cargo.toml not found. Are you in a Rust project directory?".to_string());
    }

    let mut cmd = Command::new("cargo");
    cmd.arg("build");
    cmd.arg("--release");

    if let Some(ref pkg) = package {
        cmd.arg("-p").arg(pkg);
    }

    if let Some(ref dir) = target_dir {
        cmd.arg("--target-dir").arg(dir);
    }

    if verbose {
        cmd.arg("--verbose");
    }

    // Release mode: codegen with optimizations
    cmd.arg("--no-default-features");
    let mut all_features = vec!["codegen".to_string()];
    all_features.extend(features);
    cmd.arg("--features").arg(all_features.join(","));

    if verbose {
        let features_str = all_features.join(",");
        eprintln!(
            "Executing: cargo build --release --no-default-features --features {}",
            features_str
        );
    }

    let status = cmd
        .status()
        .map_err(|e| format!("Failed to execute cargo: {}", e))?;

    if !status.success() {
        return Err("Build failed".to_string());
    }

    if verbose {
        eprintln!("Build successful! Binary is in target/release/");
    }

    eprintln!("Release build (codegen) completed successfully!");

    Ok(())
}
