#![allow(clippy::print_stderr, clippy::print_stdout)]

//! Test command - runs test suite
//!
//! This command wraps `cargo test` to provide a consistent CLI experience
//! for running tests in Dampen applications.

use std::path::Path;
use std::process::Command;

/// Test command arguments
#[derive(clap::Args)]
pub struct TestArgs {
    /// Package to test (if workspace has multiple packages)
    #[arg(short, long)]
    package: Option<String>,

    /// Test name filter (runs tests matching this string)
    #[arg(value_name = "TESTNAME")]
    test_filter: Option<String>,

    /// Arguments to pass to the test binary
    #[arg(last = true)]
    test_args: Vec<String>,

    /// Run tests in release mode
    #[arg(long)]
    release: bool,

    /// Run ignored tests
    #[arg(long)]
    ignored: bool,

    /// Run only ignored tests (implies --ignored)
    #[arg(long)]
    only_ignored: bool,

    /// Display one character per test instead of one line
    #[arg(long)]
    quiet: bool,

    /// Verbose output
    #[arg(short, long)]
    verbose: bool,

    /// Additional features to enable
    #[arg(long, value_delimiter = ',')]
    features: Vec<String>,
}

/// Execute the test command
///
/// Runs the project's test suite using cargo test.
///
/// # Examples
///
/// ```bash
/// # Run all tests
/// dampen test
///
/// # Run specific test
/// dampen test my_test_name
///
/// # Run tests for specific package
/// dampen test -p my-app
///
/// # Run with verbose output
/// dampen test --verbose
///
/// # Pass arguments to test binary
/// dampen test -- --nocapture
///
/// # Run tests in release mode
/// dampen test --release
///
/// # Run only ignored tests
/// dampen test --only-ignored
/// ```
pub fn execute(args: &TestArgs) -> Result<(), String> {
    // Check if Cargo.toml exists
    if !Path::new("Cargo.toml").exists() {
        return Err("Cargo.toml not found. Are you in a Rust project directory?".to_string());
    }

    if args.verbose {
        eprintln!("Running tests...");
    }

    // Build the cargo command
    let mut cmd = Command::new("cargo");
    cmd.arg("test");

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

    // Add quiet flag if requested
    if args.quiet {
        cmd.arg("--quiet");
    }

    // Add features if provided
    if !args.features.is_empty() {
        cmd.arg("--features").arg(args.features.join(","));
    }

    // Add test filter if provided
    if let Some(ref filter) = args.test_filter {
        cmd.arg(filter);
    }

    // Add test arguments separator and args
    if !args.test_args.is_empty() || args.ignored || args.only_ignored {
        cmd.arg("--");

        if args.only_ignored {
            cmd.arg("--ignored");
        } else if args.ignored {
            cmd.arg("--include-ignored");
        }

        cmd.args(&args.test_args);
    }

    // Execute cargo test
    if args.verbose {
        let mut command_str = String::from("cargo test");
        if let Some(ref package) = args.package {
            command_str.push_str(&format!(" -p {}", package));
        }
        if args.release {
            command_str.push_str(" --release");
        }
        if !args.features.is_empty() {
            command_str.push_str(&format!(" --features {}", args.features.join(",")));
        }
        if let Some(ref filter) = args.test_filter {
            command_str.push_str(&format!(" {}", filter));
        }
        eprintln!("Executing: {}", command_str);
    }

    let status = cmd
        .status()
        .map_err(|e| format!("Failed to execute cargo: {}", e))?;

    if !status.success() {
        return Err("Tests failed".to_string());
    }

    Ok(())
}
