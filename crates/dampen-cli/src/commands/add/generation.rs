//! File generation logic for creating window files from templates.

use crate::commands::add::errors::GenerationError;
use crate::commands::add::integration::add_module_to_mod_rs;
use crate::commands::add::templates::{TemplateKind, WindowTemplate};
use crate::commands::add::validation::{TargetPath, WindowName};
use crate::commands::add::view_switching::{activate_view_switching, detect_second_window};
use std::fs;
use std::path::PathBuf;

/// Result of file generation
#[derive(Debug, Clone)]
pub struct GeneratedFiles {
    /// Path to generated .rs file
    pub rust_file: PathBuf,

    /// Path to generated .dampen file
    pub dampen_file: PathBuf,

    /// Validated window name
    pub window_name: WindowName,

    /// Target directory where files were created
    pub target_dir: PathBuf,

    /// Path to mod.rs that was updated (if integration was performed)
    pub updated_mod_file: Option<PathBuf>,

    /// Whether main.rs was modified to enable view switching
    pub view_switching_activated: bool,
}

impl GeneratedFiles {
    /// Generate a success message showing what was created
    pub fn success_message(&self) -> String {
        let mut message = format!(
            "✓ Created UI window '{}'\n  → {}\n  → {}",
            self.window_name.snake,
            self.rust_file.display(),
            self.dampen_file.display()
        );

        // Show mod.rs update if integration was performed
        if let Some(mod_file) = &self.updated_mod_file {
            message.push_str(&format!("\n  → Updated {}", mod_file.display()));
        }

        // Show view switching activation
        if self.view_switching_activated {
            message.push_str("\n  → Activated multi-view in src/main.rs");
        }

        message.push_str("\n\nNext steps:");

        // Only show manual mod.rs step if integration was NOT performed
        if self.updated_mod_file.is_none() {
            message.push_str(&format!(
                "\n  1. Add `pub mod {};` to src/ui/mod.rs",
                self.window_name.snake
            ));
            message.push_str("\n  2. Run `dampen check` to validate");
            message.push_str("\n  3. Run your application to see the new window");
        } else {
            message.push_str("\n  1. Run `dampen check` to validate");
            message.push_str("\n  2. Run your application to see the new window");
        }

        message
    }
}

/// Generate window files (.rs and .dampen) from templates
///
/// # Arguments
///
/// * `target_path` - Validated target path with project context
/// * `window_name` - Validated window name with case variants
/// * `enable_integration` - Whether to automatically add module to mod.rs
///
/// # Returns
///
/// `GeneratedFiles` struct with paths to created files
///
/// # Errors
///
/// Returns `GenerationError` if:
/// - Files already exist (prevents overwriting)
/// - Directory creation fails
/// - File write operations fail
///
/// # Note
///
/// Integration errors (mod.rs updates) are non-fatal and emitted as warnings
pub fn generate_window_files(
    target_path: &TargetPath,
    window_name: &WindowName,
    enable_integration: bool,
) -> Result<GeneratedFiles, GenerationError> {
    // 1. Check if files already exist (prevent overwriting)
    let rust_file = target_path.file_path(&window_name.snake, "rs");
    let dampen_file = target_path.file_path(&window_name.snake, "dampen");

    if rust_file.exists() {
        return Err(GenerationError::FileExists {
            window_name: window_name.snake.clone(),
            path: rust_file,
        });
    }

    if dampen_file.exists() {
        return Err(GenerationError::FileExists {
            window_name: window_name.snake.clone(),
            path: dampen_file,
        });
    }

    // 2. Create target directory if it doesn't exist
    fs::create_dir_all(&target_path.absolute).map_err(|e| GenerationError::DirectoryCreation {
        path: target_path.absolute.clone(),
        source: e,
    })?;

    // 3. Load and render templates
    let rust_template = WindowTemplate::load(TemplateKind::RustModule);
    let dampen_template = WindowTemplate::load(TemplateKind::DampenXml);

    let variants = window_name.to_variants();
    let rust_content = rust_template.render(&variants);
    let dampen_content = dampen_template.render(&variants);

    // 4. Write .rs file
    fs::write(&rust_file, rust_content).map_err(|e| GenerationError::FileWrite {
        path: rust_file.clone(),
        source: e,
    })?;

    // 5. Write .dampen file (with cleanup on error)
    if let Err(e) = fs::write(&dampen_file, dampen_content) {
        // Cleanup: remove .rs file if .dampen write fails
        let _ = fs::remove_file(&rust_file);
        return Err(GenerationError::FileWrite {
            path: dampen_file,
            source: e,
        });
    }

    // 6. Attempt mod.rs integration if enabled
    let updated_mod_file = if enable_integration {
        match add_module_to_mod_rs(
            &target_path.project_root,
            &target_path.absolute,
            &window_name.snake,
        ) {
            Ok(mod_path) => {
                eprintln!("✓ Updated {}", mod_path.display());
                Some(mod_path)
            }
            Err(e) => {
                eprintln!("⚠ Warning: Failed to update mod.rs: {}", e);
                eprintln!(
                    "  Please manually add `pub mod {};` to the appropriate mod.rs file",
                    window_name.snake
                );
                None
            }
        }
    } else {
        None
    };

    // 7. Activate view switching if this is the second window
    let view_switching_activated = if enable_integration {
        match detect_second_window(&target_path.project_root) {
            Ok(true) => {
                // This is the second window, activate view switching
                match activate_view_switching(&target_path.project_root) {
                    Ok(()) => {
                        eprintln!("✓ Activated multi-view in src/main.rs");
                        true
                    }
                    Err(e) => {
                        eprintln!("⚠ Warning: Failed to activate view switching: {}", e);
                        eprintln!(
                            "  You may need to manually enable multi-view support in src/main.rs"
                        );
                        false
                    }
                }
            }
            Ok(false) => false,
            Err(e) => {
                eprintln!("⚠ Warning: Could not detect second window: {}", e);
                false
            }
        }
    } else {
        false
    };

    Ok(GeneratedFiles {
        rust_file,
        dampen_file,
        window_name: window_name.clone(),
        target_dir: target_path.absolute.clone(),
        updated_mod_file,
        view_switching_activated,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::add::validation::{TargetPath, WindowName};
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_generate_files_default_path() {
        let temp = TempDir::new().unwrap();
        let project_root = temp.path();
        let target_path = TargetPath::resolve(project_root, Some("src/ui")).unwrap();
        let window_name = WindowName::new("settings").unwrap();

        let result = generate_window_files(&target_path, &window_name, false);

        assert!(result.is_ok());
        let generated = result.unwrap();
        assert_eq!(generated.rust_file, project_root.join("src/ui/settings.rs"));
        assert_eq!(
            generated.dampen_file,
            project_root.join("src/ui/settings.dampen")
        );
        assert!(generated.rust_file.exists());
        assert!(generated.dampen_file.exists());
    }

    #[test]
    fn test_generate_files_creates_directory() {
        let temp = TempDir::new().unwrap();
        let project_root = temp.path();
        let target_path = TargetPath::resolve(project_root, Some("src/ui/admin")).unwrap();
        let window_name = WindowName::new("dashboard").unwrap();

        // Directory doesn't exist yet
        assert!(!target_path.absolute.exists());

        let result = generate_window_files(&target_path, &window_name, false);

        assert!(result.is_ok());
        assert!(target_path.absolute.exists());
        assert!(target_path.absolute.join("dashboard.rs").exists());
        assert!(target_path.absolute.join("dashboard.dampen").exists());
    }

    #[test]
    fn test_generate_files_rs_content() {
        let temp = TempDir::new().unwrap();
        let project_root = temp.path();
        let target_path = TargetPath::resolve(project_root, Some("src/ui")).unwrap();
        let window_name = WindowName::new("user_profile").unwrap();

        let result = generate_window_files(&target_path, &window_name, false);

        assert!(result.is_ok());
        let content = fs::read_to_string(target_path.absolute.join("user_profile.rs")).unwrap();

        // Check for key patterns in generated Rust file
        assert!(
            content.contains("user_profile"),
            "Should contain module name"
        );
        assert!(
            content.contains("pub struct Model"),
            "Should contain Model struct"
        );
        assert!(
            content.contains("#[dampen_ui"),
            "Should contain dampen_ui attribute"
        );
        assert!(
            content.contains("create_app_state"),
            "Should contain create_app_state function"
        );
    }

    #[test]
    fn test_generate_files_dampen_content() {
        let temp = TempDir::new().unwrap();
        let project_root = temp.path();
        let target_path = TargetPath::resolve(project_root, Some("src/ui")).unwrap();
        let window_name = WindowName::new("settings").unwrap();

        let result = generate_window_files(&target_path, &window_name, false);

        assert!(result.is_ok());
        let content = fs::read_to_string(target_path.absolute.join("settings.dampen")).unwrap();

        // Check for key patterns in generated XML file
        assert!(content.contains("<?xml"), "Should have XML declaration");
        assert!(content.contains("<column"), "Should have column layout");
        assert!(content.contains("Settings"), "Should contain window title");
    }

    #[test]
    fn test_generate_files_rejects_existing_rs() {
        // T101: Test that existing .rs file prevents generation
        let temp = TempDir::new().unwrap();
        let project_root = temp.path();
        let target_dir = project_root.join("src/ui");
        fs::create_dir_all(&target_dir).unwrap();

        // Create existing .rs file
        let existing_file = target_dir.join("settings.rs");
        fs::write(&existing_file, "existing content").unwrap();

        let target_path = TargetPath::resolve(project_root, Some("src/ui")).unwrap();
        let window_name = WindowName::new("settings").unwrap();
        let result = generate_window_files(&target_path, &window_name, false);

        assert!(result.is_err());
        match result {
            Err(GenerationError::FileExists {
                window_name: name,
                path,
            }) => {
                assert_eq!(name, "settings");
                assert_eq!(path, existing_file);
            }
            _ => panic!("Expected FileExists error"),
        }
    }

    #[test]
    fn test_generate_files_rejects_existing_dampen() {
        // T102: Test that existing .dampen file prevents generation
        let temp = TempDir::new().unwrap();
        let project_root = temp.path();
        let target_dir = project_root.join("src/ui");
        fs::create_dir_all(&target_dir).unwrap();

        // Create existing .dampen file
        let existing_file = target_dir.join("dashboard.dampen");
        fs::write(&existing_file, "existing xml content").unwrap();

        let target_path = TargetPath::resolve(project_root, Some("src/ui")).unwrap();
        let window_name = WindowName::new("dashboard").unwrap();
        let result = generate_window_files(&target_path, &window_name, false);

        assert!(result.is_err());
        match result {
            Err(GenerationError::FileExists {
                window_name: name,
                path,
            }) => {
                assert_eq!(name, "dashboard");
                assert_eq!(path, existing_file);
            }
            _ => panic!("Expected FileExists error"),
        }
    }

    #[test]
    fn test_generate_files_rejects_partial_conflict() {
        // T103: Test that partial conflict (only .rs exists) still prevents generation
        let temp = TempDir::new().unwrap();
        let project_root = temp.path();
        let target_dir = project_root.join("src/ui");
        fs::create_dir_all(&target_dir).unwrap();

        // Create only .rs file (no .dampen file)
        let existing_rs = target_dir.join("profile.rs");
        fs::write(&existing_rs, "existing rust content").unwrap();

        let target_path = TargetPath::resolve(project_root, Some("src/ui")).unwrap();
        let window_name = WindowName::new("profile").unwrap();
        let result = generate_window_files(&target_path, &window_name, false);

        // Should still reject because .rs exists
        assert!(result.is_err());
        match result {
            Err(GenerationError::FileExists {
                window_name: name,
                path,
            }) => {
                assert_eq!(name, "profile");
                assert_eq!(path, existing_rs);
            }
            _ => panic!("Expected FileExists error for partial conflict"),
        }

        // Clean up and try with only .dampen file
        fs::remove_file(&existing_rs).unwrap();
        let existing_dampen = target_dir.join("profile.dampen");
        fs::write(&existing_dampen, "existing xml").unwrap();

        let result2 = generate_window_files(&target_path, &window_name, false);

        // Should also reject because .dampen exists
        assert!(result2.is_err());
        match result2 {
            Err(GenerationError::FileExists {
                window_name: name,
                path,
            }) => {
                assert_eq!(name, "profile");
                assert_eq!(path, existing_dampen);
            }
            _ => panic!("Expected FileExists error for .dampen conflict"),
        }
    }

    #[test]
    fn test_success_message_format() {
        let temp = TempDir::new().unwrap();
        let project_root = temp.path();
        let target_path = TargetPath::resolve(project_root, Some("src/ui")).unwrap();
        let window_name = WindowName::new("dashboard").unwrap();

        let generated = GeneratedFiles {
            rust_file: target_path.absolute.join("dashboard.rs"),
            dampen_file: target_path.absolute.join("dashboard.dampen"),
            window_name: window_name.clone(),
            target_dir: target_path.absolute.clone(),
            updated_mod_file: None,
            view_switching_activated: false,
        };

        let message = generated.success_message();

        assert!(message.contains("Created UI window 'dashboard'"));
        assert!(message.contains("dashboard.rs"));
        assert!(message.contains("dashboard.dampen"));
        assert!(message.contains("pub mod dashboard"));
        assert!(message.contains("Next steps"));
    }

    #[test]
    fn test_success_message_with_integration() {
        let temp = TempDir::new().unwrap();
        let project_root = temp.path();
        let target_path = TargetPath::resolve(project_root, Some("src/ui")).unwrap();
        let window_name = WindowName::new("settings").unwrap();

        let generated = GeneratedFiles {
            rust_file: target_path.absolute.join("settings.rs"),
            dampen_file: target_path.absolute.join("settings.dampen"),
            window_name: window_name.clone(),
            target_dir: target_path.absolute.clone(),
            updated_mod_file: Some(project_root.join("src/ui/mod.rs")),
            view_switching_activated: false,
        };

        let message = generated.success_message();

        assert!(message.contains("Created UI window 'settings'"));
        assert!(message.contains("settings.rs"));
        assert!(message.contains("settings.dampen"));
        assert!(message.contains("Updated"), "Should mention mod.rs update");
        assert!(message.contains("src/ui/mod.rs"), "Should show mod.rs path");
        assert!(
            !message.contains("pub mod settings"),
            "Should NOT show manual mod instruction"
        );
        assert!(message.contains("Next steps"));
    }

    #[test]
    fn test_generate_files_with_integration() {
        let temp = TempDir::new().unwrap();
        let project_root = temp.path();

        // Create src/ui directory and empty mod.rs
        let ui_dir = project_root.join("src/ui");
        fs::create_dir_all(&ui_dir).unwrap();
        fs::write(ui_dir.join("mod.rs"), "").unwrap();

        let target_path = TargetPath::resolve(project_root, Some("src/ui")).unwrap();
        let window_name = WindowName::new("settings").unwrap();

        let result = generate_window_files(&target_path, &window_name, true);

        assert!(result.is_ok());
        let generated = result.unwrap();

        // Files should be created
        assert!(generated.rust_file.exists());
        assert!(generated.dampen_file.exists());

        // mod.rs should be updated
        assert!(generated.updated_mod_file.is_some());
        let mod_file = generated.updated_mod_file.unwrap();
        assert_eq!(mod_file, ui_dir.join("mod.rs"));

        // Check mod.rs content
        let mod_content = fs::read_to_string(&mod_file).unwrap();
        assert!(mod_content.contains("pub mod settings;"));
    }

    #[test]
    fn test_generate_files_integration_disabled() {
        let temp = TempDir::new().unwrap();
        let project_root = temp.path();

        // Create src/ui directory and empty mod.rs
        let ui_dir = project_root.join("src/ui");
        fs::create_dir_all(&ui_dir).unwrap();
        fs::write(ui_dir.join("mod.rs"), "").unwrap();

        let target_path = TargetPath::resolve(project_root, Some("src/ui")).unwrap();
        let window_name = WindowName::new("settings").unwrap();

        let result = generate_window_files(&target_path, &window_name, false);

        assert!(result.is_ok());
        let generated = result.unwrap();

        // Files should be created
        assert!(generated.rust_file.exists());
        assert!(generated.dampen_file.exists());

        // mod.rs should NOT be updated
        assert!(generated.updated_mod_file.is_none());

        // Check mod.rs is still empty
        let mod_content = fs::read_to_string(ui_dir.join("mod.rs")).unwrap();
        assert!(mod_content.is_empty());
    }

    #[test]
    fn test_view_switching_activated_on_second_window() {
        let temp = TempDir::new().unwrap();
        let project_root = temp.path();

        // Create src/ui directory with mod.rs and first window
        let ui_dir = project_root.join("src/ui");
        fs::create_dir_all(&ui_dir).unwrap();
        fs::write(ui_dir.join("mod.rs"), "pub mod window;\n").unwrap();
        fs::write(ui_dir.join("window.rs"), "// First window").unwrap();

        // Create a minimal main.rs with commented view switching
        let src_dir = project_root.join("src");
        fs::create_dir_all(&src_dir).unwrap();
        let main_content = r#"
enum Message {
    // SwitchToView(CurrentView),
    Handler(HandlerMessage),
}

#[dampen_app(
    ui_dir = "src/ui",
    message_type = "Message",
    // switch_view_variant = "SwitchToView",
)]
struct App;
"#;
        fs::write(src_dir.join("main.rs"), main_content).unwrap();

        // Add second window
        let target_path = TargetPath::resolve(project_root, Some("src/ui")).unwrap();
        let window_name = WindowName::new("settings").unwrap();

        let result = generate_window_files(&target_path, &window_name, true);

        assert!(result.is_ok());
        let generated = result.unwrap();

        // View switching should be activated
        assert!(generated.view_switching_activated);

        // Check main.rs was modified
        let main_content = fs::read_to_string(src_dir.join("main.rs")).unwrap();
        assert!(main_content.contains("SwitchToView(CurrentView),"));
        assert!(!main_content.contains("// SwitchToView(CurrentView),"));
        assert!(main_content.contains(r#"switch_view_variant = "SwitchToView","#));
        assert!(!main_content.contains(r#"// switch_view_variant"#));
    }

    #[test]
    fn test_view_switching_not_activated_on_first_window() {
        let temp = TempDir::new().unwrap();
        let project_root = temp.path();

        // Create src/ui directory with only mod.rs (no existing windows)
        let ui_dir = project_root.join("src/ui");
        fs::create_dir_all(&ui_dir).unwrap();
        fs::write(ui_dir.join("mod.rs"), "").unwrap();

        // Add first window
        let target_path = TargetPath::resolve(project_root, Some("src/ui")).unwrap();
        let window_name = WindowName::new("window").unwrap();

        let result = generate_window_files(&target_path, &window_name, true);

        assert!(result.is_ok());
        let generated = result.unwrap();

        // View switching should NOT be activated (only one window)
        assert!(!generated.view_switching_activated);
    }
}
