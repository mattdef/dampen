//! Build script for gravity-macros.
//!
//! This build script discovers `.gravity` files in the project and
//! generates file tracking for Cargo's incremental compilation.

use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    // Get the manifest directory (crate root)
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

    // Find all .gravity files in the ui/ directory (if it exists)
    let ui_dir = manifest_dir.join("ui");

    if ui_dir.exists() {
        println!("cargo:rerun-if-changed={}", ui_dir.display());

        // Walk the ui directory and find all .gravity files
        if let Ok(entries) = fs::read_dir(&ui_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().map_or(false, |ext| ext == "gravity") {
                    println!("cargo:rerun-if-changed={}", path.display());
                }
            }
        }
    }

    // Also watch for changes in the examples directory
    let examples_dir = manifest_dir.join("examples");
    if examples_dir.exists() {
        println!("cargo:rerun-if-changed={}", examples_dir.display());
    }
}
