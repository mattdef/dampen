//! File discovery and ViewInfo management for #[dampen_app] macro
//!
//! This module handles:
//! - Scanning directories for `.dampen` files
//! - Building `ViewInfo` metadata structures
//! - Validating view names and file structure
//! - Applying exclusion patterns

use globset::{Glob, GlobSetBuilder};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Check if a file path matches any exclusion pattern
///
/// # Arguments
/// * `relative_path` - Path relative to ui_dir (e.g., "debug.dampen", "experimental/feature.dampen")
/// * `exclude_patterns` - List of glob patterns (e.g., ["debug", "experimental/*"])
///
/// # Returns
/// * `Ok(true)` if path matches any exclusion pattern
/// * `Ok(false)` if path does not match any exclusion pattern
/// * `Err` if glob pattern compilation fails
///
/// # Notes
/// Patterns are matched both with and without .dampen extension:
/// - Pattern "debug" matches both "debug.dampen" and "debug"
/// - Pattern "experimental/*" matches "experimental/feature.dampen"
fn is_excluded(relative_path: &str, exclude_patterns: &[String]) -> Result<bool, String> {
    if exclude_patterns.is_empty() {
        return Ok(false);
    }

    let mut builder = GlobSetBuilder::new();
    for pattern in exclude_patterns {
        // Add the pattern as-is
        let glob =
            Glob::new(pattern).map_err(|e| format!("Invalid glob pattern '{}': {}", pattern, e))?;
        builder.add(glob);

        // Also add pattern with .dampen extension if not already present
        if !pattern.ends_with(".dampen") && !pattern.ends_with("/*") {
            let pattern_with_ext = format!("{}.dampen", pattern);
            let glob_with_ext = Glob::new(&pattern_with_ext)
                .map_err(|e| format!("Invalid glob pattern '{}': {}", pattern_with_ext, e))?;
            builder.add(glob_with_ext);
        }
    }

    let glob_set = builder
        .build()
        .map_err(|e| format!("Failed to build glob set: {}", e))?;

    Ok(glob_set.is_match(relative_path))
}

/// Represents a discovered `.dampen` view file with all metadata needed for code generation
/// Metadata about a discovered view file.
///
/// Contains all information needed to generate code for a single view in a multi-view application.
/// Each `.dampen` file in the UI directory becomes one ViewInfo instance.
///
/// # Fields
///
/// * `view_name` - Snake_case identifier derived from filename (e.g., "text_input", "main_window")
/// * `variant_name` - PascalCase enum variant name for `CurrentView` (e.g., "TextInput", "MainWindow")
/// * `field_name` - Struct field name for the AppState instance (e.g., "text_input_state")
/// * `module_path` - Rust module path from ui_dir root (e.g., "ui::widgets::text_input")
/// * `dampen_file` - Absolute path to the `.dampen` XML file
/// * `rs_file` - Absolute path to the corresponding `.rs` module file (must exist)
///
/// # Examples
///
/// For a file at `src/ui/widgets/text_input.dampen`:
///
/// ```ignore
/// ViewInfo {
///     view_name: "text_input".to_string(),
///     variant_name: "TextInput".to_string(),
///     field_name: "text_input_state".to_string(),
///     module_path: "ui::widgets::text_input".to_string(),
///     dampen_file: PathBuf::from("/absolute/path/to/src/ui/widgets/text_input.dampen"),
///     rs_file: PathBuf::from("/absolute/path/to/src/ui/widgets/text_input.rs"),
/// }
/// ```
///
/// # Validation
///
/// ViewInfo instances are validated to ensure:
/// - `view_name` is a valid Rust identifier (VR-001)
/// - All variant names are unique within a set of views (VR-002)
/// - The corresponding `.rs` file exists (VR-003)
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

/// Discovers all `.dampen` files in a directory tree and creates ViewInfo for each.
///
/// Recursively walks the specified directory, finding all `.dampen` files and converting
/// them to ViewInfo metadata. Files matching exclusion patterns are filtered out.
///
/// # Arguments
///
/// * `ui_dir` - Root directory to scan for `.dampen` files (absolute path)
/// * `exclude_patterns` - Glob patterns to exclude (e.g., `["debug", "experimental/*"]`)
///
/// # Returns
///
/// * `Ok(Vec<ViewInfo>)` - Sorted vector of discovered views (alphabetically by view_name)
/// * `Err(String)` - Error message if:
///   - Directory traversal fails
///   - ViewInfo creation fails (invalid filename, missing .rs file, etc.)
///   - Glob pattern compilation fails
///   - Duplicate view names are found
///
/// # Examples
///
/// ```ignore
/// let views = discover_dampen_files(
///     Path::new("/path/to/src/ui"),
///     &["debug".to_string(), "experimental/*".to_string()]
/// )?;
/// // Returns ViewInfo for all .dampen files except those matching patterns
/// ```
///
/// # Notes
///
/// - Files are sorted alphabetically by `view_name` for deterministic code generation
/// - Exclusion patterns match both with and without `.dampen` extension
/// - Empty directories are silently skipped
/// - Validation is performed on all discovered ViewInfo instances
pub fn discover_dampen_files(
    ui_dir: &Path,
    exclude_patterns: &[String],
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
            // Check if this path should be excluded
            let relative_path = path.strip_prefix(ui_dir).unwrap_or(path);
            let relative_str = relative_path.to_string_lossy();

            match is_excluded(&relative_str, exclude_patterns) {
                Ok(true) => {
                    // Path is excluded, skip it
                    continue;
                }
                Ok(false) => {
                    // Not excluded, process it
                }
                Err(_e) => {
                    // Invalid pattern - this should have been caught in MacroAttributes::parse()
                    // Continue processing to avoid silent failures
                }
            }

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

    // Safe: we checked above that name is not empty
    if let Some(first_char) = name.chars().next()
        && !first_char.is_alphabetic()
        && first_char != '_'
    {
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
