//! Build script to extract dependency versions from workspace Cargo.toml
//!
//! This script parses the workspace Cargo.toml and exports dependency versions
//! as environment variables for use in the `dampen new` template generation.

// Allow clippy lints for build scripts - these run at compile-time only
#![allow(clippy::expect_used, clippy::unwrap_used, clippy::panic)]

use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    // Get the workspace root (two levels up from dampen-cli/build.rs)
    let manifest_dir = PathBuf::from(
        env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR must be set during build"),
    );
    let workspace_root = manifest_dir
        .parent()
        .and_then(|p| p.parent())
        .expect("Failed to find workspace root (expected two levels up)");
    let workspace_toml = workspace_root.join("Cargo.toml");

    // Re-run if workspace Cargo.toml changes (if it exists)
    if workspace_toml.exists() {
        println!("cargo:rerun-if-changed={}", workspace_toml.display());
    }

    // Read and parse workspace Cargo.toml
    // If it doesn't exist (e.g., during crates.io packaging), use default versions
    let content = match fs::read_to_string(&workspace_toml) {
        Ok(content) => content,
        Err(_) => {
            // Use default versions when workspace Cargo.toml is not available
            println!("cargo:rustc-env=ICED_VERSION=0.14");
            println!("cargo:rustc-env=SERDE_VERSION=1.0");
            println!("cargo:rustc-env=SERDE_JSON_VERSION=1.0");
            return;
        }
    };

    // Extract versions using simple string parsing
    // Note: This is a simple approach. For more complex scenarios, consider using toml crate.
    for line in content.lines() {
        let line = line.trim();

        // Look for dependency version definitions like: iced = { version = "0.14", ... }
        if let Some(version) = extract_version(line, "iced") {
            println!("cargo:rustc-env=ICED_VERSION={}", version);
        }
        if let Some(version) = extract_version(line, "serde") {
            println!("cargo:rustc-env=SERDE_VERSION={}", version);
        }
        if let Some(version) = extract_version(line, "serde_json") {
            println!("cargo:rustc-env=SERDE_JSON_VERSION={}", version);
        }
    }
}

/// Extract version from a TOML line like: name = { version = "1.0", ... }
fn extract_version(line: &str, dep_name: &str) -> Option<String> {
    // Check if line starts with the dependency name
    if !line.starts_with(dep_name) {
        return None;
    }

    // Find version = "..." pattern
    if let Some(version_start) = line.find("version = \"") {
        let after_version = &line[version_start + 11..]; // Skip 'version = "'
        if let Some(quote_pos) = after_version.find('"') {
            return Some(after_version[..quote_pos].to_string());
        }
    }

    // Handle simple version format: name = "1.0"
    if let Some(eq_pos) = line.find('=') {
        let after_eq = line[eq_pos + 1..].trim();
        if after_eq.starts_with('"') && after_eq.ends_with('"') {
            return Some(after_eq[1..after_eq.len() - 1].to_string());
        }
    }

    None
}
