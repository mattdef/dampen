//! Integration tests for the `dampen add` command.
//!
//! These tests verify the add command's behavior in various scenarios.

use assert_cmd::Command;
use predicates::prelude::*;
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

    // Create minimal Cargo.toml with dampen-core dependency
    let cargo_toml = r#"
[package]
name = "test-project"
version = "0.1.0"
edition = "2024"

[dependencies]
dampen-core = "0.2.2"
dampen-macros = "0.2.2"
"#;
    fs::write(project_path.join("Cargo.toml"), cargo_toml).unwrap();

    // Create src/main.rs
    fs::write(project_path.join("src/main.rs"), "fn main() {}").unwrap();

    // Create src/ui/mod.rs
    fs::write(project_path.join("src/ui/mod.rs"), "// UI modules\n").unwrap();

    project_path
}

#[test]
#[serial]
fn test_add_ui_creates_files() {
    let temp = TempDir::new().unwrap();
    let project_path = create_test_project(&temp);

    // Execute dampen add --ui settings
    dampen_cmd()
        .arg("add")
        .arg("--ui")
        .arg("settings")
        .current_dir(&project_path)
        .assert()
        .success()
        .stdout(predicate::str::contains("Created UI window 'settings'"))
        .stdout(predicate::str::contains("settings.rs"))
        .stdout(predicate::str::contains("settings.dampen"));

    // Verify files were created
    let settings_rs = project_path.join("src/ui/settings.rs");
    let settings_dampen = project_path.join("src/ui/settings.dampen");

    assert!(settings_rs.exists(), "settings.rs should be created");
    assert!(
        settings_dampen.exists(),
        "settings.dampen should be created"
    );

    // Verify content of .rs file
    let rs_content = fs::read_to_string(&settings_rs).unwrap();
    assert!(
        rs_content.contains("settings"),
        "Rust file should contain module name"
    );
    assert!(
        rs_content.contains("#[dampen_ui"),
        "Rust file should have dampen_ui attribute"
    );
    assert!(
        rs_content.contains("pub struct Model"),
        "Rust file should define Model struct"
    );

    // Verify content of .dampen file
    let dampen_content = fs::read_to_string(&settings_dampen).unwrap();
    assert!(
        dampen_content.contains("<?xml"),
        "XML file should have declaration"
    );
    assert!(
        dampen_content.contains("Settings"),
        "XML file should contain window title"
    );
}

#[test]
#[serial]
fn test_add_ui_prevents_overwrite() {
    let temp = TempDir::new().unwrap();
    let project_path = create_test_project(&temp);

    // Create initial file
    dampen_cmd()
        .arg("add")
        .arg("--ui")
        .arg("dashboard")
        .current_dir(&project_path)
        .assert()
        .success();

    // Try to create same file again
    dampen_cmd()
        .arg("add")
        .arg("--ui")
        .arg("dashboard")
        .current_dir(&project_path)
        .assert()
        .failure()
        .stderr(predicate::str::contains("already exists"))
        .stderr(predicate::str::contains("dashboard"));
}

#[test]
#[serial]
fn test_add_ui_fails_outside_dampen_project() {
    let temp = TempDir::new().unwrap();

    // Execute in directory without Cargo.toml
    dampen_cmd()
        .arg("add")
        .arg("--ui")
        .arg("test")
        .current_dir(temp.path())
        .assert()
        .failure()
        .stderr(predicate::str::contains("Not a Dampen project"));
}

#[test]
#[serial]
fn test_add_ui_validates_window_name() {
    let temp = TempDir::new().unwrap();
    let project_path = create_test_project(&temp);

    // Test empty name
    dampen_cmd()
        .arg("add")
        .arg("--ui")
        .arg("")
        .current_dir(&project_path)
        .assert()
        .failure();

    // Test invalid first character
    dampen_cmd()
        .arg("add")
        .arg("--ui")
        .arg("9window")
        .current_dir(&project_path)
        .assert()
        .failure()
        .stderr(predicate::str::contains("must start with a letter"));

    // Test reserved name
    dampen_cmd()
        .arg("add")
        .arg("--ui")
        .arg("mod")
        .current_dir(&project_path)
        .assert()
        .failure()
        .stderr(predicate::str::contains("reserved"));
}

#[test]
#[serial]
fn test_add_ui_case_conversion() {
    let temp = TempDir::new().unwrap();
    let project_path = create_test_project(&temp);

    // Test with PascalCase input
    dampen_cmd()
        .arg("add")
        .arg("--ui")
        .arg("UserProfile")
        .current_dir(&project_path)
        .assert()
        .success();

    // Should create snake_case files
    assert!(project_path.join("src/ui/user_profile.rs").exists());
    assert!(project_path.join("src/ui/user_profile.dampen").exists());

    // Content should have proper case variants
    let rs_content = fs::read_to_string(project_path.join("src/ui/user_profile.rs")).unwrap();
    assert!(rs_content.contains("user_profile"));
}

#[test]
#[serial]
fn test_add_ui_custom_path() {
    let temp = TempDir::new().unwrap();
    let project_path = create_test_project(&temp);

    // Create with custom path
    dampen_cmd()
        .arg("add")
        .arg("--ui")
        .arg("admin_panel")
        .arg("--path")
        .arg("src/ui/admin")
        .current_dir(&project_path)
        .assert()
        .success();

    // Verify files in custom location
    assert!(project_path.join("src/ui/admin/admin_panel.rs").exists());
    assert!(
        project_path
            .join("src/ui/admin/admin_panel.dampen")
            .exists()
    );
}

#[test]
#[serial]
fn test_generated_files_compile() {
    let temp = TempDir::new().unwrap();
    let project_path = create_test_project(&temp);

    // Generate files
    dampen_cmd()
        .arg("add")
        .arg("--ui")
        .arg("test_window")
        .current_dir(&project_path)
        .assert()
        .success();

    // Test that files have valid syntax by reading and basic checks
    let rs_content = fs::read_to_string(project_path.join("src/ui/test_window.rs")).unwrap();
    let dampen_content =
        fs::read_to_string(project_path.join("src/ui/test_window.dampen")).unwrap();

    // Rust file should be valid Rust syntax (basic check)
    assert!(
        rs_content.contains("use") && rs_content.contains("pub fn"),
        "Rust file should have proper structure"
    );

    // XML should be well-formed (basic check)
    assert!(
        dampen_content.starts_with("<?xml") && dampen_content.contains("</dampen>"),
        "XML should be well-formed"
    );
}

#[test]
#[serial]
fn test_add_ui_creates_missing_directories() {
    let temp = TempDir::new().unwrap();
    let project_path = create_test_project(&temp);

    // Create with path that doesn't exist yet
    dampen_cmd()
        .arg("add")
        .arg("--ui")
        .arg("order_form")
        .arg("--path")
        .arg("ui/orders/forms")
        .current_dir(&project_path)
        .assert()
        .success();

    // Verify directory was created and files exist
    let target_dir = project_path.join("ui/orders/forms");
    assert!(target_dir.exists(), "Directory should be created");
    assert!(target_dir.join("order_form.rs").exists());
    assert!(target_dir.join("order_form.dampen").exists());
}

#[test]
#[serial]
fn test_add_ui_rejects_absolute_path() {
    let temp = TempDir::new().unwrap();
    let project_path = create_test_project(&temp);

    // Try to use an absolute path
    dampen_cmd()
        .arg("add")
        .arg("--ui")
        .arg("window")
        .arg("--path")
        .arg("/absolute/path")
        .current_dir(&project_path)
        .assert()
        .failure()
        .stderr(predicate::str::contains("Absolute paths are not allowed"))
        .stderr(predicate::str::contains("help:"));
}

#[test]
#[serial]
fn test_add_ui_rejects_outside_project() {
    let temp = TempDir::new().unwrap();
    let project_path = create_test_project(&temp);

    // Try to escape project directory using ..
    dampen_cmd()
        .arg("add")
        .arg("--ui")
        .arg("window")
        .arg("--path")
        .arg("../outside")
        .current_dir(&project_path)
        .assert()
        .failure()
        .stderr(predicate::str::contains("outside the project directory"))
        .stderr(predicate::str::contains("help:"));
}

#[test]
#[serial]
fn test_add_ui_error_message_helpful() {
    // T111: Test that duplicate file error message is helpful
    let temp = TempDir::new().unwrap();
    let project_path = create_test_project(&temp);

    // Create initial window
    dampen_cmd()
        .arg("add")
        .arg("--ui")
        .arg("orders")
        .current_dir(&project_path)
        .assert()
        .success();

    // Try to create same window again
    let output = dampen_cmd()
        .arg("add")
        .arg("--ui")
        .arg("orders")
        .current_dir(&project_path)
        .assert()
        .failure();

    // Verify error message contains:
    output
        .stderr(predicate::str::contains("already exists")) // Clear error
        .stderr(predicate::str::contains("orders")) // Window name
        .stderr(predicate::str::contains("orders.rs").or(predicate::str::contains("orders.dampen"))) // File path
        .stderr(predicate::str::contains("help:")) // Help prefix
        .stderr(
            predicate::str::contains("Choose a different name")
                .or(predicate::str::contains("remove the existing file")),
        ); // Actionable suggestion
}
