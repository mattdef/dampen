//! Integration tests for the `gravity new` command

use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

/// Helper to get the gravity binary command
fn gravity_cmd() -> Command {
    Command::cargo_bin("gravity").unwrap()
}

#[test]
fn test_new_creates_project_structure() {
    let temp = TempDir::new().unwrap();
    let project_name = "test-app";

    // Execute gravity new
    gravity_cmd()
        .arg("new")
        .arg(project_name)
        .current_dir(temp.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("Created new Gravity project"));

    // Verify directory structure
    let project_path = temp.path().join(project_name);
    assert!(project_path.exists(), "Project directory should exist");
    assert!(project_path.is_dir(), "Project path should be a directory");

    // Verify files exist
    assert!(
        project_path.join("Cargo.toml").exists(),
        "Cargo.toml should exist"
    );
    assert!(
        project_path.join("README.md").exists(),
        "README.md should exist"
    );
    assert!(
        project_path.join("src").is_dir(),
        "src/ directory should exist"
    );
    assert!(
        project_path.join("src/main.rs").exists(),
        "src/main.rs should exist"
    );
    assert!(
        project_path.join("ui").is_dir(),
        "ui/ directory should exist"
    );
    assert!(
        project_path.join("ui/main.gravity").exists(),
        "ui/main.gravity should exist"
    );
}

#[test]
fn test_new_substitutes_project_name_in_cargo_toml() {
    let temp = TempDir::new().unwrap();
    let project_name = "my-cool-app";

    gravity_cmd()
        .arg("new")
        .arg(project_name)
        .current_dir(temp.path())
        .assert()
        .success();

    // Read and verify Cargo.toml
    let cargo_toml_path = temp.path().join(project_name).join("Cargo.toml");
    let cargo_toml = fs::read_to_string(cargo_toml_path).unwrap();

    assert!(cargo_toml.contains(&format!("name = \"{}\"", project_name)));
    assert!(cargo_toml.contains("gravity-core = \"0.1.0\""));
    assert!(cargo_toml.contains("gravity-macros = \"0.1.0\""));
    assert!(cargo_toml.contains("gravity-iced = \"0.1.0\""));
}

#[test]
fn test_new_substitutes_project_name_in_readme() {
    let temp = TempDir::new().unwrap();
    let project_name = "readme-test";

    gravity_cmd()
        .arg("new")
        .arg(project_name)
        .current_dir(temp.path())
        .assert()
        .success();

    // Read and verify README.md
    let readme_path = temp.path().join(project_name).join("README.md");
    let readme = fs::read_to_string(readme_path).unwrap();

    assert!(readme.contains(&format!("# {}", project_name)));
    assert!(readme.contains("Quick Start"));
    assert!(readme.contains("cargo run"));
}

#[test]
fn test_new_substitutes_project_name_in_ui() {
    let temp = TempDir::new().unwrap();
    let project_name = "ui-test";

    gravity_cmd()
        .arg("new")
        .arg(project_name)
        .current_dir(temp.path())
        .assert()
        .success();

    // Read and verify main.gravity
    let ui_path = temp.path().join(project_name).join("ui/main.gravity");
    let ui_content = fs::read_to_string(ui_path).unwrap();

    assert!(ui_content.contains(&format!("Welcome to {}!", project_name)));
}

#[test]
fn test_new_rejects_empty_name() {
    let temp = TempDir::new().unwrap();

    gravity_cmd()
        .arg("new")
        .arg("")
        .current_dir(temp.path())
        .assert()
        .failure()
        .stderr(predicate::str::contains("Project name cannot be empty"));
}

#[test]
fn test_new_rejects_name_starting_with_number() {
    let temp = TempDir::new().unwrap();

    gravity_cmd()
        .arg("new")
        .arg("123invalid")
        .current_dir(temp.path())
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "must start with a letter or underscore",
        ));
}

#[test]
fn test_new_rejects_name_with_special_chars() {
    let temp = TempDir::new().unwrap();

    gravity_cmd()
        .arg("new")
        .arg("my@app")
        .current_dir(temp.path())
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "can only contain letters, numbers, hyphens, and underscores",
        ));
}

#[test]
fn test_new_rejects_name_with_spaces() {
    let temp = TempDir::new().unwrap();

    gravity_cmd()
        .arg("new")
        .arg("my app")
        .current_dir(temp.path())
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "can only contain letters, numbers, hyphens, and underscores",
        ));
}

#[test]
fn test_new_rejects_reserved_names() {
    let temp = TempDir::new().unwrap();

    for reserved in &["test", "build", "target", "src"] {
        gravity_cmd()
            .arg("new")
            .arg(reserved)
            .current_dir(temp.path())
            .assert()
            .failure()
            .stderr(predicate::str::contains("is a reserved name"));
    }
}

#[test]
fn test_new_accepts_valid_names() {
    let temp = TempDir::new().unwrap();

    let valid_names = vec![
        "my-app", "my_app", "MyApp", "app123", "_private", "a", "a-b-c",
    ];

    for (i, name) in valid_names.iter().enumerate() {
        // Use different names to avoid conflicts
        let unique_name = format!("{}-{}", name, i);

        gravity_cmd()
            .arg("new")
            .arg(&unique_name)
            .current_dir(temp.path())
            .assert()
            .success();
    }
}

#[test]
fn test_new_detects_existing_directory() {
    let temp = TempDir::new().unwrap();
    let project_name = "existing";

    // Create directory first
    fs::create_dir(temp.path().join(project_name)).unwrap();

    // Try to create project with same name
    gravity_cmd()
        .arg("new")
        .arg(project_name)
        .current_dir(temp.path())
        .assert()
        .failure()
        .stderr(predicate::str::contains("already exists"));
}

#[test]
fn test_new_creates_valid_rust_code() {
    let temp = TempDir::new().unwrap();
    let project_name = "valid-code-test";

    gravity_cmd()
        .arg("new")
        .arg(project_name)
        .current_dir(temp.path())
        .assert()
        .success();

    let project_path = temp.path().join(project_name);

    // Read main.rs and verify it's valid Rust syntax
    let main_rs = fs::read_to_string(project_path.join("src/main.rs")).unwrap();

    // Check for key elements
    assert!(main_rs.contains("struct Model"));
    assert!(main_rs.contains("#[derive(Default, UiModel, Serialize, Deserialize, Clone, Debug)]"));
    assert!(main_rs.contains("register_simple"));
    assert!(main_rs.contains("fn greet(model: &mut Model)"));
    assert!(main_rs.contains("fn main() -> iced::Result"));
}

#[test]
fn test_new_creates_valid_xml() {
    let temp = TempDir::new().unwrap();
    let project_name = "valid-xml-test";

    gravity_cmd()
        .arg("new")
        .arg(project_name)
        .current_dir(temp.path())
        .assert()
        .success();

    let ui_file = temp.path().join(project_name).join("ui/main.gravity");
    let xml_content = fs::read_to_string(ui_file).unwrap();

    // Verify it's valid XML (at least has XML declaration and root element)
    assert!(xml_content.contains("<?xml"));
    assert!(xml_content.contains("<column"));
    assert!(xml_content.contains("</column>"));
}

#[test]
fn test_new_output_messages() {
    let temp = TempDir::new().unwrap();
    let project_name = "output-test";

    gravity_cmd()
        .arg("new")
        .arg(project_name)
        .current_dir(temp.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("Created new Gravity project"))
        .stdout(predicate::str::contains("Next steps:"))
        .stdout(predicate::str::contains("cd output-test"))
        .stdout(predicate::str::contains("cargo run"));
}
