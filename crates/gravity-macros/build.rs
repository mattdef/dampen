//! Build script for gravity-macros.
//!
//! This build script discovers `.gravity` files in the project and
//! generates file tracking for Cargo's incremental compilation.

use std::env;
use std::path::PathBuf;
use walkdir::WalkDir;

fn main() {
    // Get the manifest directory (crate root)
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

    // Find all .gravity files recursively in the project
    discover_gravity_files(&manifest_dir);

    // Also watch for changes in the examples directory
    let examples_dir = manifest_dir.join("examples");
    if examples_dir.exists() {
        println!("cargo:rerun-if-changed={}", examples_dir.display());
    }
}

/// Recursively discover all .gravity files and emit rerun-if-changed directives.
fn discover_gravity_files(manifest_dir: &PathBuf) {
    // Watch the ui directory
    let ui_dir = manifest_dir.join("ui");
    if ui_dir.exists() {
        println!("cargo:rerun-if-changed={}", ui_dir.display());
    }

    // Recursively find all .gravity files
    for entry in WalkDir::new(manifest_dir)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path: &PathBuf = &entry.path().to_path_buf();

        // Check if it's a .gravity file (not .gravity.rs)
        if let Some(ext) = path.extension() {
            if ext == "gravity" {
                println!("cargo:rerun-if-changed={}", path.display());
            }
        }
    }
}
