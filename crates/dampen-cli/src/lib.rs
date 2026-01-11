//! Dampen CLI - Developer Command-Line Interface
//!
//! This crate provides the CLI for the Dampen UI framework.

#![allow(clippy::print_stderr, clippy::print_stdout)]

pub mod commands;

use clap::{Parser, Subcommand};

/// Dampen UI Framework CLI
#[derive(Parser)]
#[command(name = "dampen")]
#[command(about = "Developer CLI for Dampen UI framework", long_about = None)]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Build production code with codegen mode
    Build(commands::BuildArgs),

    /// Validate .dampen files without building
    Check(commands::CheckArgs),

    /// Inspect IR or generated code
    Inspect(commands::InspectArgs),

    /// Create a new Dampen project
    New(commands::NewArgs),

    /// Build optimized production binary (release mode with codegen)
    Release(commands::ReleaseArgs),

    /// Run application in development mode with interpreted execution
    Run(commands::RunArgs),

    /// Run tests for the Dampen project
    Test(commands::TestArgs),
}

/// CLI entry point
pub fn run() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Build(args) => commands::build_execute(&args).map_err(|e| e.to_string()),
        Commands::Check(args) => commands::check_execute(&args).map_err(|e| e.to_string()),
        Commands::Inspect(args) => commands::inspect_execute(&args),
        Commands::New(args) => commands::new_execute(&args),
        Commands::Release(args) => commands::release_execute(&args),
        Commands::Run(args) => commands::run_execute(&args).map_err(|e| e.to_string()),
        Commands::Test(args) => commands::test_execute(&args),
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
