//! File discovery and ViewInfo management for #[dampen_app] macro
//!
//! This module handles:
//! - Scanning directories for `.dampen` files
//! - Building `ViewInfo` metadata structures
//! - Validating view names and file structure
//! - Applying exclusion patterns

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Represents a discovered `.dampen` view file with all metadata needed for code generation
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ViewInfo {
    /// Snake_case identifier derived from filename (e.g., "text_input")
    pub view_name: String,

    /// PascalCase enum variant name (e.g., "TextInput")
    pub variant_name: String,

    /// Struct field name for AppState instance (e.g., "text_input_state")
    pub field_name: String,

    /// Rust module path from ui_dir root (e.g., "ui::widgets::text_input")
    pub module_path: String,

    /// Absolute path to the .dampen file
    pub dampen_file: PathBuf,

    /// Absolute path to the corresponding .rs file
    pub rs_file: PathBuf,
}

/// Convert snake_case to PascalCase
///
/// Examples:
/// - "text_input" → "TextInput"
/// - "main_window" → "MainWindow"
/// - "button" → "Button"
pub fn to_pascal_case(snake: &str) -> String {
    snake
        .split('_')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().chain(chars).collect(),
                None => String::new(),
            }
        })
        .collect()
}

impl ViewInfo {
    /// Create ViewInfo from a .dampen file path and UI directory
    ///
    /// # Arguments
    /// * `dampen_file` - Absolute path to .dampen file
    /// * `ui_dir` - Root UI directory (absolute path)
    ///
    /// # Returns
    /// ViewInfo with all fields derived from the file path
    pub fn from_path(dampen_file: &Path, ui_dir: &Path) -> Result<Self, String> {
        // Extract filename without extension
        let file_stem = dampen_file
            .file_stem()
            .and_then(|s| s.to_str())
            .ok_or_else(|| format!("Invalid file name: {:?}", dampen_file))?;

        let view_name = file_stem.to_string();

        // Validate view name is a valid Rust identifier (VR-001)
        validate_rust_identifier(&view_name)?;

        let variant_name = to_pascal_case(&view_name);
        let field_name = format!("{}_state", view_name);

        // Build module path from relative path
        let relative_path = dampen_file.strip_prefix(ui_dir).map_err(|_| {
            format!(
                "File {:?} is not under UI directory {:?}",
                dampen_file, ui_dir
            )
        })?;

        let module_path = if let Some(parent) = relative_path.parent() {
            if parent.as_os_str().is_empty() {
                format!("ui::{}", view_name)
            } else {
                let parent_parts: Vec<_> = parent
                    .components()
                    .filter_map(|c| c.as_os_str().to_str())
                    .collect();
                format!("ui::{}::{}", parent_parts.join("::"), view_name)
            }
        } else {
            format!("ui::{}", view_name)
        };

        // Derive .rs file path
        let rs_file = dampen_file.with_extension("rs");

        Ok(ViewInfo {
            view_name,
            variant_name,
            field_name,
            module_path,
            dampen_file: dampen_file.to_path_buf(),
            rs_file,
        })
    }
}

/// Discover all .dampen files in a directory
///
/// # Arguments
/// * `ui_dir` - Directory to scan for .dampen files
/// * `exclude_patterns` - Glob patterns to exclude (not yet implemented)
///
/// # Returns
/// Sorted vector of ViewInfo structures
pub fn discover_dampen_files(
    ui_dir: &Path,
    _exclude_patterns: &[String],
) -> Result<Vec<ViewInfo>, String> {
    if !ui_dir.exists() {
        return Err(format!("UI directory not found: {:?}", ui_dir));
    }

    let mut views = Vec::new();

    // Walk directory tree
    for entry in WalkDir::new(ui_dir)
        .follow_links(false)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();

        // Filter .dampen files
        if path.extension().and_then(|s| s.to_str()) == Some("dampen") {
            let view_info = ViewInfo::from_path(path, ui_dir)?;

            // Validate .rs file exists (VR-003)
            if !view_info.rs_file.exists() {
                return Err(format!(
                    "No matching Rust module found for '{}'\nhelp: Create a file at '{}'",
                    view_info.dampen_file.display(),
                    view_info.rs_file.display()
                ));
            }

            views.push(view_info);
        }
    }

    // Sort alphabetically for deterministic behavior (FR-016)
    views.sort();

    // Validate unique variant names (VR-002)
    validate_unique_variants(&views)?;

    Ok(views)
}

/// Validate that a string is a valid Rust identifier (VR-001)
fn validate_rust_identifier(name: &str) -> Result<(), String> {
    if name.is_empty() {
        return Err("Invalid view name: empty string".to_string());
    }

    let first_char = name.chars().next().unwrap();
    if !first_char.is_alphabetic() && first_char != '_' {
        return Err(format!(
            "Invalid view name '{}'\nhelp: View names must start with a letter or underscore",
            name
        ));
    }

    if !name.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return Err(format!(
            "Invalid view name '{}'\nhelp: View names must contain only letters, numbers, and underscores",
            name
        ));
    }

    Ok(())
}

/// Validate that all variant names are unique (VR-002)
fn validate_unique_variants(views: &[ViewInfo]) -> Result<(), String> {
    let mut seen = HashMap::new();

    for view in views {
        if let Some(existing) = seen.insert(&view.variant_name, &view.dampen_file) {
            return Err(format!(
                "View naming conflict: '{}' found in multiple locations:\n  - {}\n  - {}\nhelp: Rename one of the files or exclude one via the 'exclude' attribute",
                view.variant_name,
                existing.display(),
                view.dampen_file.display()
            ));
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_pascal_case_single_word() {
        assert_eq!(to_pascal_case("button"), "Button");
    }

    #[test]
    fn test_to_pascal_case_multiple_words() {
        assert_eq!(to_pascal_case("text_input"), "TextInput");
        assert_eq!(to_pascal_case("main_window"), "MainWindow");
    }

    #[test]
    fn test_to_pascal_case_already_capitalized() {
        assert_eq!(to_pascal_case("Button"), "Button");
    }
}
