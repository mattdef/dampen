//! Theme file discovery and loading for dampen-dev.
//!
//! This module provides functions for discovering theme.dampen files
//! and loading them into ThemeContext for runtime use.
//!
//! Note: System theme detection is handled by Iced's built-in `system::theme_changes()`
//! subscription, which uses winit's native theme detection. The initial theme is
//! determined by the theme document's `default_theme` setting, and then updated
//! reactively when the system theme changes.

use dampen_core::ir::theme::ThemeDocument;
use dampen_core::parser::theme_parser::parse_theme_document;
use dampen_core::state::ThemeContext;
use std::fs;
use std::path::{Path, PathBuf};

/// Discover and load theme.dampen from a project directory.
///
/// This function looks for `src/ui/theme/theme.dampen` in the given
/// project directory and loads it if found.
///
/// Note: The initial theme is set to the document's `default_theme`. System theme
/// preference will be applied reactively via the `watch_system_theme()` subscription
/// once the application starts.
///
/// # Arguments
///
/// * `project_dir` - The root directory of the Dampen project
///
/// # Returns
///
/// * `Ok(Some(ThemeContext))` - If a valid theme file was found and loaded
/// * `Ok(None)` - If no theme file was found (use default Iced theme)
/// * `Err(_)` - If a theme file exists but couldn't be parsed
pub fn load_theme_context(project_dir: &Path) -> Result<Option<ThemeContext>, ThemeLoadError> {
    load_theme_context_with_preference(project_dir, None)
}

/// Discover and load theme.dampen with an optional system preference.
///
/// This variant allows passing a pre-detected system preference for cases
/// where it's already known (e.g., from a previous theme change event).
///
/// # Arguments
///
/// * `project_dir` - The root directory of the Dampen project
/// * `system_preference` - Optional system theme preference ("light" or "dark")
///
/// # Returns
///
/// * `Ok(Some(ThemeContext))` - If a valid theme file was found and loaded
/// * `Ok(None)` - If no theme file was found (use default Iced theme)
/// * `Err(_)` - If a theme file exists but couldn't be parsed
pub fn load_theme_context_with_preference(
    project_dir: &Path,
    system_preference: Option<&str>,
) -> Result<Option<ThemeContext>, ThemeLoadError> {
    match discover_theme_file(project_dir) {
        Some(Ok(doc)) => {
            let ctx = ThemeContext::from_document(doc, system_preference)
                .map_err(ThemeLoadError::InvalidDocument)?;
            Ok(Some(ctx))
        }
        Some(Err(e)) => Err(ThemeLoadError::ParseError(e)),
        None => Ok(None),
    }
}

/// Discover theme.dampen file in a project directory.
///
/// Looks for `src/ui/theme/theme.dampen` in the given project directory.
///
/// # Arguments
///
/// * `project_dir` - The root directory of the Dampen project
///
/// # Returns
///
/// * `Some(Ok(ThemeDocument))` - If a valid theme file was found
/// * `Some(Err(_))` - If the file exists but couldn't be parsed
/// * `None` - If no theme file was found
pub fn discover_theme_file(project_dir: &Path) -> Option<Result<ThemeDocument, String>> {
    let theme_path = find_theme_file_path(project_dir)?;

    if !theme_path.exists() {
        return None;
    }

    match fs::read_to_string(&theme_path) {
        Ok(content) => match parse_theme_document(&content) {
            Ok(doc) => Some(Ok(doc)),
            Err(e) => Some(Err(format!("Failed to parse theme: {}", e))),
        },
        Err(e) => Some(Err(format!("Failed to read theme file: {}", e))),
    }
}

/// Find the path to theme.dampen in a project directory.
///
/// Searches in order:
/// 1. `src/ui/theme/theme.dampen`
///
/// # Arguments
///
/// * `project_dir` - The root directory of the Dampen project
///
/// # Returns
///
/// The path to theme.dampen if found, None otherwise
pub fn find_theme_file_path(project_dir: &Path) -> Option<PathBuf> {
    let paths = vec![project_dir.join("src/ui/theme/theme.dampen")];

    paths.into_iter().find(|path| path.exists())
}

/// Find the project root by searching for theme.dampen in multiple locations.
///
/// This function implements a robust search strategy for finding the project root
/// when the application is run from various locations (e.g., target/release).
///
/// Search order:
/// 1. `CARGO_MANIFEST_DIR` environment variable (available during `cargo run`)
/// 2. Ancestors of the executable location (for binaries in target/release)
/// 3. Workspace examples directory (for examples in a workspace)
/// 4. Ancestors of the current working directory
///
/// # Returns
///
/// The project root path if found, None otherwise
pub fn find_project_root() -> Option<PathBuf> {
    // 1. Try CARGO_MANIFEST_DIR (available during cargo run)
    if let Ok(manifest_dir) = std::env::var("CARGO_MANIFEST_DIR") {
        let path = PathBuf::from(&manifest_dir);
        if path.join("src/ui/theme/theme.dampen").exists() {
            return Some(path);
        }
    }

    // 2. Try relative to the executable location
    // This handles the case where the binary is in target/release or target/debug
    if let Ok(exe_path) = std::env::current_exe() {
        // Get the executable name (e.g., "todo-app")
        let exe_name = exe_path
            .file_stem()
            .and_then(|s| s.to_str())
            .map(|s| s.to_string());

        if let Some(exe_dir) = exe_path.parent() {
            // Walk up the directory tree
            for ancestor in exe_dir.ancestors() {
                // Direct check: src/ui/theme/theme.dampen
                let theme_path = ancestor.join("src/ui/theme/theme.dampen");
                if theme_path.exists() {
                    return Some(ancestor.to_path_buf());
                }

                // Workspace check: Look for the project in examples/ directory
                // This handles cases where the exe is in workspace/target/release
                // but the project is in workspace/examples/project-name/
                if let Some(ref name) = exe_name {
                    let examples_path = ancestor.join("examples").join(name);
                    let theme_in_examples =
                        examples_path.join("src/ui/theme/theme.dampen");
                    if theme_in_examples.exists() {
                        return Some(examples_path);
                    }
                }
            }
        }
    }

    // 3. Try from current directory upwards
    if let Ok(cwd) = std::env::current_dir() {
        for ancestor in cwd.ancestors() {
            let theme_path = ancestor.join("src/ui/theme/theme.dampen");
            if theme_path.exists() {
                return Some(ancestor.to_path_buf());
            }
        }
    }

    None
}

/// Create a minimal Dampen application structure for testing.
///
/// This helper function creates the basic directory structure of a
/// Dampen application, optionally with or without a theme file.
///
/// # Arguments
///
/// * `dir` - The temporary directory to create the app structure in
pub fn create_minimal_dampen_app(dir: &Path) {
    let src_dir = dir.join("src");
    let ui_dir = src_dir.join("ui");
    let theme_dir = ui_dir.join("theme");

    let _ = fs::create_dir_all(&theme_dir);
}

/// Errors that can occur when loading themes
#[derive(Debug, thiserror::Error)]
pub enum ThemeLoadError {
    /// The theme file exists but couldn't be parsed
    #[error("Failed to parse theme file: {0}")]
    ParseError(String),

    /// The parsed theme document is invalid
    #[error("Invalid theme document: {0}")]
    InvalidDocument(#[from] dampen_core::ir::theme::ThemeError),
}
