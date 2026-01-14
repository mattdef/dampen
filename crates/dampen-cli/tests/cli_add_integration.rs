//! Deep integration tests for the `dampen add` command.
//!
//! These tests verify that generated code actually compiles and integrates correctly.

use assert_cmd::Command;
use serial_test::serial;
use std::fs;
use tempfile::TempDir;

/// Helper to get the dampen binary command
fn dampen_cmd() -> Command {
    Command::cargo_bin("dampen").unwrap()
}

/// Helper to create a minimal Dampen project for testing
fn create_test_project(temp_dir: &TempDir) -> std::path::PathBuf {
    let project_path = temp_dir.path().join("test-project");
    fs::create_dir_all(&project_path).unwrap();
    fs::create_dir_all(project_path.join("src/ui")).unwrap();

    // Get absolute path to dampen crates
    let workspace_root = std::env::current_dir()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf();

    // Create minimal Cargo.toml with dampen-core dependency
    let cargo_toml = format!(
        r#"
[package]
name = "test-project"
version = "0.1.0"
edition = "2024"

[dependencies]
dampen-core = {{ path = "{}/crates/dampen-core" }}
dampen-macros = {{ path = "{}/crates/dampen-macros" }}
dampen-iced = {{ path = "{}/crates/dampen-iced" }}
iced = "0.14"
serde = {{ version = "1.0", features = ["derive"] }}
"#,
        workspace_root.display(),
        workspace_root.display(),
        workspace_root.display()
    );
    fs::write(project_path.join("Cargo.toml"), cargo_toml).unwrap();

    // Create src/main.rs
    let main_rs = r#"
fn main() {
    println!("Test project");
}
"#;
    fs::write(project_path.join("src/main.rs"), main_rs).unwrap();

    // Create src/ui/mod.rs
    fs::write(project_path.join("src/ui/mod.rs"), "// UI modules\n").unwrap();

    project_path
}

#[test]
#[serial]
fn test_generated_model_has_ui_model_derive() {
    // T115: Verify generated .rs file has #[derive(UiModel)]
    let temp = TempDir::new().unwrap();
    let project_path = create_test_project(&temp);

    // Generate window
    dampen_cmd()
        .arg("add")
        .arg("--ui")
        .arg("profile")
        .current_dir(&project_path)
        .assert()
        .success();

    // Read generated .rs file
    let rs_content = fs::read_to_string(project_path.join("src/ui/profile.rs")).unwrap();

    // Verify Model struct with UiModel derive
    assert!(
        rs_content.contains("#[derive(Default, UiModel, Serialize, Deserialize, Clone, Debug)]"),
        "Model should have UiModel derive macro"
    );
    assert!(
        rs_content.contains("pub struct Model"),
        "Should define Model struct"
    );
    assert!(
        rs_content.contains("use dampen_macros::{UiModel, dampen_ui}"),
        "Should import UiModel macro"
    );
}

#[test]
#[serial]
fn test_generated_module_exports_complete() {
    // T116: Verify generated .rs file has all required exports
    let temp = TempDir::new().unwrap();
    let project_path = create_test_project(&temp);

    // Generate window
    dampen_cmd()
        .arg("add")
        .arg("--ui")
        .arg("dashboard")
        .current_dir(&project_path)
        .assert()
        .success();

    // Read generated .rs file
    let rs_content = fs::read_to_string(project_path.join("src/ui/dashboard.rs")).unwrap();

    // Verify required functions exist
    assert!(
        rs_content.contains("pub fn create_app_state() -> AppState<Model>"),
        "Should export create_app_state() function"
    );
    assert!(
        rs_content.contains("pub fn create_handler_registry() -> HandlerRegistry"),
        "Should export create_handler_registry() function"
    );

    // Verify dampen_ui attribute
    assert!(
        rs_content.contains("#[dampen_ui(\"dashboard.dampen\")]"),
        "Should have dampen_ui attribute with correct filename"
    );

    // Verify handler registration
    assert!(
        rs_content.contains("registry.register_simple"),
        "Should register handlers"
    );
}

#[test]
#[serial]
fn test_generated_xml_has_correct_structure() {
    // Verify generated .dampen file has valid structure
    let temp = TempDir::new().unwrap();
    let project_path = create_test_project(&temp);

    // Generate window
    dampen_cmd()
        .arg("add")
        .arg("--ui")
        .arg("settings")
        .current_dir(&project_path)
        .assert()
        .success();

    // Read generated .dampen file
    let dampen_content = fs::read_to_string(project_path.join("src/ui/settings.dampen")).unwrap();

    // Verify XML structure
    assert!(
        dampen_content.contains("<?xml version=\"1.0\" encoding=\"UTF-8\" ?>"),
        "Should have XML declaration"
    );
    assert!(
        dampen_content.contains("<dampen version=\"1.0\">"),
        "Should have dampen root with version"
    );
    assert!(
        dampen_content.contains("<column"),
        "Should have column layout"
    );
    assert!(
        dampen_content.contains("</dampen>"),
        "Should close dampen tag"
    );
    assert!(dampen_content.contains("<text"), "Should have text widget");
    assert!(
        dampen_content.contains("<button"),
        "Should have button widget"
    );
}

#[test]
#[serial]
fn test_generated_xml_passes_dampen_check() {
    // T114: Verify generated XML passes dampen check validation
    let temp = TempDir::new().unwrap();
    let project_path = create_test_project(&temp);

    // Generate window
    dampen_cmd()
        .arg("add")
        .arg("--ui")
        .arg("orders")
        .current_dir(&project_path)
        .assert()
        .success();

    // Run dampen check on the directory (it will find all .dampen files)
    let check_result = dampen_cmd()
        .arg("check")
        .arg("--input")
        .arg("src/ui")
        .current_dir(&project_path)
        .assert();

    // Should pass validation
    check_result.success();
}

#[test]
#[serial]
#[ignore] // This test takes longer - run with --ignored
fn test_generated_window_compiles() {
    // T113: Verify generated code actually compiles
    let temp = TempDir::new().unwrap();
    let project_path = create_test_project(&temp);

    // Generate window
    dampen_cmd()
        .arg("add")
        .arg("--ui")
        .arg("checkout")
        .current_dir(&project_path)
        .assert()
        .success();

    // Add window to mod.rs
    let mod_content = fs::read_to_string(project_path.join("src/ui/mod.rs")).unwrap();
    let updated_mod = format!("{}pub mod checkout;\n", mod_content);
    fs::write(project_path.join("src/ui/mod.rs"), updated_mod).unwrap();

    // Try to build the project
    let build_result = Command::new("cargo")
        .arg("check")
        .arg("--quiet")
        .current_dir(&project_path)
        .assert();

    // Should compile successfully
    build_result.success();
}
