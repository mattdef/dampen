#![allow(clippy::print_stderr, clippy::print_stdout)]

//! Release command - builds production-optimized binaries with codegen mode
//!
//! This command wraps `cargo build --release` with the `codegen` feature flag,
//! providing optimized production builds with zero runtime overhead.

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
/// This is an alias for `dampen build --release` that builds the application
/// in production mode with codegen and full optimizations.
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
/// # Basic release build (codegen)
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
///
/// # Note: This is equivalent to `dampen build --release`
/// ```
pub fn execute(args: &ReleaseArgs) -> Result<(), String> {
    // Note: This is an alias for dampen build --release
    // Delegate to build module's release function

    if args.verbose {
        eprintln!("'dampen release' is an alias for 'dampen build --release'");
    }

    crate::commands::build::execute_release_build(
        args.package.clone(),
        args.features.clone(),
        args.verbose,
        args.target_dir.clone(),
    )
}
