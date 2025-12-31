//! Gravity CLI - Developer Command-Line Interface
//!
//! This crate provides the CLI for the Gravity UI framework.

pub mod commands;
pub mod config;

use clap::{Parser, Subcommand};

/// Gravity UI Framework CLI
#[derive(Parser)]
#[command(name = "gravity")]
#[command(about = "Developer CLI for Gravity UI framework", long_about = None)]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Build production code from .gravity files
    Build(commands::BuildArgs),
    
    /// Validate .gravity files without building
    Check(commands::CheckArgs),
    
    /// Run in development mode with hot-reload
    Dev(commands::DevArgs),
    
    /// Inspect IR or generated code
    Inspect(commands::InspectArgs),
}

/// CLI entry point
pub fn run() {
    let cli = Cli::parse();
    
    let result = match cli.command {
        Commands::Build(args) => commands::build_execute(&args).map_err(|e| e.to_string()),
        Commands::Check(args) => commands::check_execute(&args).map_err(|e| e.to_string()),
        Commands::Dev(args) => commands::dev_execute(&args),
        Commands::Inspect(args) => commands::inspect_execute(&args),
    };
    
    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
