//! File discovery and ViewInfo management for #[dampen_app] macro
//!
//! This module handles:
//! - Scanning directories for `.dampen` files
//! - Building `ViewInfo` metadata structures
//! - Validating view names and file structure
//! - Applying exclusion patterns

use std::path::PathBuf;

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
