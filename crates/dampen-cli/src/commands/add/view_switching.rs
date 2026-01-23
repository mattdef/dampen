//! View switching activation logic for enabling multi-view in main.rs
//!
//! When a second UI window is added to a project, this module handles:
//! 1. Detecting if there are 2+ UI modules in src/ui/
//! 2. Uncommenting or adding `SwitchToView(CurrentView)` to Message enum
//! 3. Adding `switch_view_variant = "SwitchToView"` to `#[dampen_app]` macro

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// Error type for view switching operations
#[derive(Debug, thiserror::Error)]
pub enum ViewSwitchError {
    #[error("Failed to read main.rs: {source}")]
    MainFileRead {
        path: PathBuf,
        #[source]
        source: io::Error,
    },

    #[error("Failed to write main.rs: {source}")]
    MainFileWrite {
        path: PathBuf,
        #[source]
        source: io::Error,
    },

    #[error("Failed to read directory {path}: {source}")]
    DirectoryRead {
        path: PathBuf,
        #[source]
        source: io::Error,
    },

    #[error("Could not find Message enum in main.rs")]
    MessageEnumNotFound,

    #[error("Could not find #[dampen_app] macro in main.rs")]
    DampenAppMacroNotFound,
}

/// Detect if there are 2+ UI modules in src/ui/ directory
///
/// # Arguments
///
/// * `project_root` - Root directory of the project
///
/// # Returns
///
/// `true` if there are 2 or more .rs files in src/ui/ (excluding mod.rs)
pub fn detect_second_window(project_root: &Path) -> Result<bool, ViewSwitchError> {
    let ui_dir = project_root.join("src/ui");

    if !ui_dir.exists() {
        return Ok(false);
    }

    let entries = fs::read_dir(&ui_dir).map_err(|source| ViewSwitchError::DirectoryRead {
        path: ui_dir.clone(),
        source,
    })?;

    let mut rs_file_count = 0;
    for entry in entries {
        let entry = entry.map_err(|source| ViewSwitchError::DirectoryRead {
            path: ui_dir.clone(),
            source,
        })?;

        let path = entry.path();
        if path.is_file() && path.extension().is_some_and(|ext| ext == "rs") {
            // Ignore mod.rs
            if path.file_name().is_some_and(|name| name != "mod.rs") {
                rs_file_count += 1;

                // Early exit if we found 2
                if rs_file_count >= 2 {
                    return Ok(true);
                }
            }
        }
    }

    Ok(false)
}

/// Activate view switching in main.rs
///
/// This function modifies main.rs to enable multi-view support by:
/// 1. Uncommenting or adding `SwitchToView(CurrentView)` in Message enum
/// 2. Adding `switch_view_variant = "SwitchToView"` to `#[dampen_app]` macro
///
/// # Arguments
///
/// * `project_root` - Root directory of the project
///
/// # Returns
///
/// `Ok(())` if successful, or an error if modifications failed
pub fn activate_view_switching(project_root: &Path) -> Result<(), ViewSwitchError> {
    let main_path = project_root.join("src/main.rs");

    let content =
        fs::read_to_string(&main_path).map_err(|source| ViewSwitchError::MainFileRead {
            path: main_path.clone(),
            source,
        })?;

    // Step 1: Uncomment or add SwitchToView to Message enum
    let content = uncomment_or_add_switch_message(&content)?;

    // Step 2: Add switch_view_variant to #[dampen_app] macro
    let content = add_switch_variant_to_macro(&content)?;

    // Write back
    fs::write(&main_path, content).map_err(|source| ViewSwitchError::MainFileWrite {
        path: main_path.clone(),
        source,
    })?;

    Ok(())
}

/// Uncomment or add `SwitchToView(CurrentView)` to Message enum
///
/// Patterns handled:
/// 1. Commented line: `// SwitchToView(CurrentView),` → uncommented
/// 2. Missing entirely → added after enum declaration
fn uncomment_or_add_switch_message(content: &str) -> Result<String, ViewSwitchError> {
    // Check if it's already uncommented and present
    if content.contains("SwitchToView(CurrentView)")
        && !content.contains("// SwitchToView(CurrentView)")
    {
        // Already present and uncommented
        return Ok(content.to_string());
    }

    // Try to uncomment if it exists as a comment
    if content.contains("// SwitchToView(CurrentView)") {
        // Uncomment the line
        let result = content.replace(
            "// SwitchToView(CurrentView),",
            "SwitchToView(CurrentView),",
        );
        return Ok(result);
    }

    // Not found, need to add it
    // Find the Message enum and add it as the first variant
    let enum_pattern = "enum Message {";
    if let Some(enum_pos) = content.find(enum_pattern) {
        let insert_pos = enum_pos + enum_pattern.len();
        let mut result = String::with_capacity(content.len() + 100);
        result.push_str(&content[..insert_pos]);
        result.push_str("\n    /// View switching\n    SwitchToView(CurrentView),");
        result.push_str(&content[insert_pos..]);
        return Ok(result);
    }

    Err(ViewSwitchError::MessageEnumNotFound)
}

/// Add `switch_view_variant = "SwitchToView"` to #[dampen_app] macro
///
/// Patterns handled:
/// 1. Commented line: `// switch_view_variant = "SwitchToView",` → uncommented
/// 2. Missing entirely → added before the closing `)]`
fn add_switch_variant_to_macro(content: &str) -> Result<String, ViewSwitchError> {
    // Check if it's already uncommented and present
    if content.contains(r#"switch_view_variant = "SwitchToView""#)
        && !content.contains(r#"// switch_view_variant = "SwitchToView""#)
    {
        // Already present and uncommented
        return Ok(content.to_string());
    }

    // Try to uncomment if it exists as a comment
    if content.contains(r#"// switch_view_variant = "SwitchToView""#) {
        // Uncomment the line (handle with or without comma)
        let result = content
            .replace(
                r#"// switch_view_variant = "SwitchToView","#,
                r#"switch_view_variant = "SwitchToView","#,
            )
            .replace(
                r#"// switch_view_variant = "SwitchToView""#,
                r#"switch_view_variant = "SwitchToView","#,
            );
        return Ok(result);
    }

    // Not found, need to add it
    // Find the #[dampen_app(...)] macro and add it before the closing )]
    if let Some(macro_start) = content.find("#[dampen_app(")
        && let Some(macro_end) = content[macro_start..].find(")]")
    {
        let absolute_macro_end = macro_start + macro_end;

        // Get the content before )]
        let before_close = &content[..absolute_macro_end];

        // Find the last line before )] that's not just whitespace
        let lines: Vec<&str> = before_close.lines().collect();
        let last_non_empty_idx = lines.iter().rposition(|line| !line.trim().is_empty());

        if let Some(idx) = last_non_empty_idx {
            // Rebuild with comma added if needed
            let mut result = String::with_capacity(content.len() + 100);

            // Add all lines up to the last non-empty line
            for (i, line) in lines.iter().enumerate() {
                result.push_str(line);
                result.push('\n');

                if i == idx {
                    // This is the last non-empty line
                    // Add comma if it doesn't have one
                    if !line.trim_end().ends_with(',') {
                        // Go back and add comma before the newline
                        result.pop(); // Remove the \n we just added
                        result.push(',');
                        result.push('\n');
                    }

                    // Add the switch_view_variant line
                    result.push_str(r#"    switch_view_variant = "SwitchToView","#);
                    result.push('\n');
                }
            }

            // Add the rest of the content (from )] onwards)
            result.push_str(&content[absolute_macro_end..]);
            return Ok(result);
        }

        // Fallback: just insert before )]
        let mut result = String::with_capacity(content.len() + 100);
        result.push_str(&content[..absolute_macro_end]);
        result.push_str(r#"    switch_view_variant = "SwitchToView","#);
        result.push('\n');
        result.push_str(&content[absolute_macro_end..]);
        return Ok(result);
    }

    Err(ViewSwitchError::DampenAppMacroNotFound)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_detect_second_window_no_ui_dir() {
        let temp = TempDir::new().unwrap();
        let result = detect_second_window(temp.path());
        assert!(result.is_ok());
        assert!(!result.unwrap());
    }

    #[test]
    fn test_detect_second_window_only_one_file() {
        let temp = TempDir::new().unwrap();
        let ui_dir = temp.path().join("src/ui");
        fs::create_dir_all(&ui_dir).unwrap();
        fs::write(ui_dir.join("window.rs"), "").unwrap();
        fs::write(ui_dir.join("mod.rs"), "").unwrap();

        let result = detect_second_window(temp.path());
        assert!(result.is_ok());
        assert!(!result.unwrap());
    }

    #[test]
    fn test_detect_second_window_two_files() {
        let temp = TempDir::new().unwrap();
        let ui_dir = temp.path().join("src/ui");
        fs::create_dir_all(&ui_dir).unwrap();
        fs::write(ui_dir.join("window.rs"), "").unwrap();
        fs::write(ui_dir.join("settings.rs"), "").unwrap();
        fs::write(ui_dir.join("mod.rs"), "").unwrap();

        let result = detect_second_window(temp.path());
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[test]
    fn test_uncomment_switch_message() {
        let input = r#"
enum Message {
    // SwitchToView(CurrentView),
    Handler(HandlerMessage),
}
"#;

        let result = uncomment_or_add_switch_message(input).unwrap();
        assert!(result.contains("SwitchToView(CurrentView),"));
        assert!(!result.contains("// SwitchToView(CurrentView),"));
    }

    #[test]
    fn test_add_switch_message_when_missing() {
        let input = r#"
enum Message {
    Handler(HandlerMessage),
}
"#;

        let result = uncomment_or_add_switch_message(input).unwrap();
        assert!(result.contains("SwitchToView(CurrentView),"));
        assert!(result.contains("/// View switching"));
    }

    #[test]
    fn test_uncomment_switch_variant_in_macro() {
        let input = r#"
#[dampen_app(
    ui_dir = "src/ui",
    message_type = "Message",
    // switch_view_variant = "SwitchToView",
)]
"#;

        let result = add_switch_variant_to_macro(input).unwrap();
        assert!(result.contains(r#"switch_view_variant = "SwitchToView","#));
        assert!(!result.contains(r#"// switch_view_variant"#));
    }

    #[test]
    fn test_add_switch_variant_when_missing() {
        let input = r#"
#[dampen_app(
    ui_dir = "src/ui",
    message_type = "Message"
)]
"#;

        let result = add_switch_variant_to_macro(input).unwrap();
        assert!(result.contains(r#"switch_view_variant = "SwitchToView","#));
        // Should add comma to previous line
        assert!(result.contains(r#"message_type = "Message","#));
    }

    #[test]
    fn test_add_switch_variant_when_missing_with_comma() {
        let input = r#"
#[dampen_app(
    ui_dir = "src/ui",
    message_type = "Message",
)]
"#;

        let result = add_switch_variant_to_macro(input).unwrap();
        assert!(result.contains(r#"switch_view_variant = "SwitchToView","#));
        // Should keep existing comma
        assert!(result.contains(r#"message_type = "Message","#));
    }

    #[test]
    fn test_already_activated_no_changes() {
        let input = r#"
enum Message {
    SwitchToView(CurrentView),
    Handler(HandlerMessage),
}

#[dampen_app(
    ui_dir = "src/ui",
    switch_view_variant = "SwitchToView",
)]
"#;

        let result1 = uncomment_or_add_switch_message(input).unwrap();
        let result2 = add_switch_variant_to_macro(&result1).unwrap();

        // Should be unchanged
        assert_eq!(result1, input);
        assert_eq!(result2, result1);
    }
}
