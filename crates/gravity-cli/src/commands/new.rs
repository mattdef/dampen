//! Create a new Gravity project
//!
//! This module provides the `gravity new` command which scaffolds a new
//! Gravity UI project with a simple Hello World example using the
//! auto-loading pattern.
//!
//! # Example
//!
//! ```bash
//! gravity new my-app
//! cd my-app
//! cargo run
//! ```

#![allow(clippy::print_stdout)]

use std::fs;
use std::path::{Path, PathBuf};

/// Arguments for the new command
///
/// # Fields
///
/// * `name` - The name of the project to create. Must be a valid Rust package name.
#[derive(Debug, clap::Args)]
pub struct NewArgs {
    /// Name of the project to create
    pub name: String,
}

/// Execute the new command
///
/// Creates a new Gravity project directory with:
/// - `Cargo.toml` with Gravity dependencies
/// - `src/main.rs` with a complete Hello World application using auto-loading
/// - `src/ui/mod.rs` - UI module
/// - `src/ui/window.rs` - UI model and handlers with `#[gravity_ui]` macro
/// - `src/ui/window.gravity` - Declarative UI definition (XML)
/// - `tests/integration.rs` - Integration tests
/// - `README.md` with comprehensive getting started instructions
///
/// # Arguments
///
/// * `args` - Command arguments containing the project name
///
/// # Returns
///
/// * `Ok(())` - If project was created successfully
/// * `Err(String)` - If creation failed with error message
///
/// # Errors
///
/// This function will return an error if:
/// - The project name is invalid
/// - A directory with the same name already exists
/// - File system operations fail (e.g., permission denied)
pub fn execute(args: &NewArgs) -> Result<(), String> {
    let project_name = &args.name;

    // Validate project name
    validate_project_name(project_name)?;

    // Get the project path
    let project_path = PathBuf::from(project_name);

    // Check if directory already exists
    if project_path.exists() {
        return Err(format!("Directory '{}' already exists", project_name));
    }

    // Create project structure
    match create_project(project_name, &project_path) {
        Ok(()) => {
            println!("Created new Gravity project: {}", project_name);
            println!();
            println!("Next steps:");
            println!("  cd {}", project_name);
            println!("  cargo run");
            Ok(())
        }
        Err(e) => {
            // Cleanup on error
            cleanup_on_error(&project_path);
            Err(e)
        }
    }
}

/// Validate the project name
///
/// A valid project name must:
/// - Not be empty
/// - Start with a letter or underscore
/// - Contain only alphanumeric characters, hyphens, and underscores
/// - Not be a reserved name
fn validate_project_name(name: &str) -> Result<(), String> {
    // Check if empty
    if name.is_empty() {
        return Err("Project name cannot be empty".to_string());
    }

    // Check first character
    if let Some(first) = name.chars().next() {
        if !first.is_alphabetic() && first != '_' {
            return Err("Project name must start with a letter or underscore".to_string());
        }
    }

    // Check all characters
    if !name
        .chars()
        .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
    {
        return Err(
            "Project name can only contain letters, numbers, hyphens, and underscores".to_string(),
        );
    }

    // Check reserved names
    const RESERVED: &[&str] = &["test", "doc", "build", "target", "src"];
    if RESERVED.contains(&name) {
        return Err(format!("'{}' is a reserved name", name));
    }

    Ok(())
}

/// Create the complete project structure
fn create_project(project_name: &str, project_path: &Path) -> Result<(), String> {
    // Create directories
    create_project_structure(project_path)?;

    // Generate files
    generate_cargo_toml(project_path, project_name)?;
    generate_main_rs(project_path, project_name)?;
    generate_ui_mod_rs(project_path, project_name)?;
    generate_ui_window_rs(project_path, project_name)?;
    generate_window_gravity(project_path, project_name)?;
    generate_integration_tests(project_path, project_name)?;
    generate_readme(project_path, project_name)?;

    Ok(())
}

/// Create the directory structure
fn create_project_structure(project_path: &Path) -> Result<(), String> {
    // Create main project directory
    fs::create_dir(project_path).map_err(|e| {
        format!(
            "Failed to create directory '{}': {}",
            project_path.display(),
            e
        )
    })?;

    // Create src/ directory
    let src_dir = project_path.join("src");
    fs::create_dir(&src_dir)
        .map_err(|e| format!("Failed to create directory '{}': {}", src_dir.display(), e))?;

    // Create src/ui/ directory
    let ui_dir = src_dir.join("ui");
    fs::create_dir(&ui_dir)
        .map_err(|e| format!("Failed to create directory '{}': {}", ui_dir.display(), e))?;

    // Create tests/ directory
    let tests_dir = project_path.join("tests");
    fs::create_dir(&tests_dir).map_err(|e| {
        format!(
            "Failed to create directory '{}': {}",
            tests_dir.display(),
            e
        )
    })?;

    Ok(())
}

/// Generate Cargo.toml from template
fn generate_cargo_toml(project_path: &Path, project_name: &str) -> Result<(), String> {
    let template = include_str!("../../templates/new/Cargo.toml.template");
    let content = template.replace("{{PROJECT_NAME}}", project_name);

    let file_path = project_path.join("Cargo.toml");
    fs::write(&file_path, content)
        .map_err(|e| format!("Failed to write '{}': {}", file_path.display(), e))?;

    Ok(())
}

/// Generate src/main.rs from template
fn generate_main_rs(project_path: &Path, project_name: &str) -> Result<(), String> {
    let template = include_str!("../../templates/new/main.rs.template");
    let content = template.replace("{{PROJECT_NAME}}", project_name);

    let file_path = project_path.join("src/main.rs");
    fs::write(&file_path, content)
        .map_err(|e| format!("Failed to write '{}': {}", file_path.display(), e))?;

    Ok(())
}

/// Generate src/ui/mod.rs from template
fn generate_ui_mod_rs(project_path: &Path, project_name: &str) -> Result<(), String> {
    let template = include_str!("../../templates/new/src/ui/mod.rs.template");
    let content = template.replace("{{PROJECT_NAME}}", project_name);

    let file_path = project_path.join("src/ui/mod.rs");
    fs::write(&file_path, content)
        .map_err(|e| format!("Failed to write '{}': {}", file_path.display(), e))?;

    Ok(())
}

/// Generate src/ui/window.rs from template
fn generate_ui_window_rs(project_path: &Path, project_name: &str) -> Result<(), String> {
    let template = include_str!("../../templates/new/src/ui/window.rs.template");
    let content = template.replace("{{PROJECT_NAME}}", project_name);

    let file_path = project_path.join("src/ui/window.rs");
    fs::write(&file_path, content)
        .map_err(|e| format!("Failed to write '{}': {}", file_path.display(), e))?;

    Ok(())
}

/// Generate src/ui/window.gravity from template
fn generate_window_gravity(project_path: &Path, project_name: &str) -> Result<(), String> {
    let template = include_str!("../../templates/new/window.gravity.template");
    let content = template.replace("{{PROJECT_NAME}}", project_name);

    let file_path = project_path.join("src/ui/window.gravity");
    fs::write(&file_path, content)
        .map_err(|e| format!("Failed to write '{}': {}", file_path.display(), e))?;

    Ok(())
}

/// Generate tests/integration.rs from template
fn generate_integration_tests(project_path: &Path, project_name: &str) -> Result<(), String> {
    let template = include_str!("../../templates/new/tests/integration.rs.template");
    let content = template.replace("{{PROJECT_NAME}}", project_name);

    let file_path = project_path.join("tests/integration.rs");
    fs::write(&file_path, content)
        .map_err(|e| format!("Failed to write '{}': {}", file_path.display(), e))?;

    Ok(())
}

/// Generate README.md from template
fn generate_readme(project_path: &Path, project_name: &str) -> Result<(), String> {
    let template = include_str!("../../templates/new/README.md.template");
    let content = template.replace("{{PROJECT_NAME}}", project_name);

    let file_path = project_path.join("README.md");
    fs::write(&file_path, content)
        .map_err(|e| format!("Failed to write '{}': {}", file_path.display(), e))?;

    Ok(())
}

/// Cleanup project directory on error
fn cleanup_on_error(project_path: &Path) {
    if project_path.exists() {
        let _ = fs::remove_dir_all(project_path);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_project_name_valid() {
        assert!(validate_project_name("my-app").is_ok());
        assert!(validate_project_name("my_app").is_ok());
        assert!(validate_project_name("myapp").is_ok());
        assert!(validate_project_name("MyApp").is_ok());
        assert!(validate_project_name("my-app-123").is_ok());
        assert!(validate_project_name("_private").is_ok());
    }

    #[test]
    fn test_validate_project_name_invalid() {
        assert!(validate_project_name("").is_err());
        assert!(validate_project_name("123").is_err());
        assert!(validate_project_name("-invalid").is_err());
        assert!(validate_project_name("my app").is_err());
        assert!(validate_project_name("my/app").is_err());
        assert!(validate_project_name("test").is_err());
        assert!(validate_project_name("build").is_err());
    }
}
