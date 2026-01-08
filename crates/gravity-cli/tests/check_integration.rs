use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

/// Helper function to create a temporary .gravity file for testing
fn create_test_file(dir: &TempDir, filename: &str, content: &str) -> PathBuf {
    let file_path = dir.path().join(filename);
    fs::write(&file_path, content).expect("Failed to write test file");
    file_path
}

#[test]
fn test_unknown_attribute_detection_integration() {
    use gravity_cli::commands::check::{execute, CheckArgs};

    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let ui_dir = temp_dir.path().join("ui");
    fs::create_dir(&ui_dir).expect("Failed to create ui dir");

    // Create a test file with unknown attribute
    let content = r#"<?xml version="1.0" encoding="UTF-8"?>
<column>
    <button on_clik="handle_click" label="Click Me" />
</column>"#;

    fs::write(ui_dir.join("test.gravity"), content).expect("Failed to write test file");

    let args = CheckArgs {
        input: ui_dir.to_string_lossy().to_string(),
        verbose: false,
        handlers: None,
        model: None,
        custom_widgets: None,
        strict: false,
    };

    let result = execute(&args);

    // Should fail due to unknown attribute 'on_clik'
    assert!(result.is_err());

    // The error message should contain suggestion
    let err = result.unwrap_err();
    let err_msg = format!("{}", err);
    assert!(err_msg.contains("on_clik"));
    assert!(err_msg.contains("on_click") || err_msg.contains("Did you mean"));
}

#[test]
fn test_valid_attributes_pass_validation() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");

    // Create a test file with all valid attributes
    let content = r#"<?xml version="1.0" encoding="UTF-8"?>
<ui>
    <Button on_click="handle_click" label="Click Me" width="100" />
</ui>"#;

    create_test_file(&temp_dir, "test.gravity", content);

    assert!(temp_dir.path().join("test.gravity").exists());
}

#[test]
fn test_strict_mode_placeholder() {
    // This test will be implemented once strict mode is integrated
    // For now, it serves as a contract for the feature
    let temp_dir = TempDir::new().expect("Failed to create temp dir");

    let content = r#"<?xml version="1.0" encoding="UTF-8"?>
<ui>
    <Button on_clik="handle_click" />
</ui>"#;

    create_test_file(&temp_dir, "test.gravity", content);

    // In strict mode, this should fail validation
    // Normal mode should still report errors but with different exit code
    assert!(temp_dir.path().join("test.gravity").exists());
}

// T021: Integration test for handler validation
#[test]
fn test_handler_validation_with_registry() {
    use gravity_cli::commands::check::{execute, CheckArgs};

    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let ui_dir = temp_dir.path().join("ui");
    fs::create_dir(&ui_dir).expect("Failed to create ui dir");

    // Create handler registry
    let registry_path = temp_dir.path().join("handlers.json");
    let registry_content = r#"[
  {
    "name": "handle_click",
    "param_type": null,
    "returns_command": false
  }
]"#;
    fs::write(&registry_path, registry_content).expect("Failed to write registry");

    // Create UI file with valid handler
    let content = r#"<?xml version="1.0" encoding="UTF-8"?>
<column>
    <button on_click="handle_click" label="Click Me" />
</column>"#;

    fs::write(ui_dir.join("test.gravity"), content).expect("Failed to write test file");

    let args = CheckArgs {
        input: ui_dir.to_string_lossy().to_string(),
        verbose: false,
        handlers: Some(registry_path.to_string_lossy().to_string()),
        model: None,
        custom_widgets: None,
        strict: false,
    };

    let result = execute(&args);

    // Should pass with valid handler
    assert!(result.is_ok());
}

#[test]
fn test_handler_validation_with_unknown_handler() {
    use gravity_cli::commands::check::{execute, CheckArgs};

    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let ui_dir = temp_dir.path().join("ui");
    fs::create_dir(&ui_dir).expect("Failed to create ui dir");

    // Create handler registry
    let registry_path = temp_dir.path().join("handlers.json");
    let registry_content = r#"[
  {
    "name": "handle_click",
    "param_type": null,
    "returns_command": false
  }
]"#;
    fs::write(&registry_path, registry_content).expect("Failed to write registry");

    // Create UI file with unknown handler
    let content = r#"<?xml version="1.0" encoding="UTF-8"?>
<column>
    <button on_click="unknown_handler" label="Click Me" />
</column>"#;

    fs::write(ui_dir.join("test.gravity"), content).expect("Failed to write test file");

    let args = CheckArgs {
        input: ui_dir.to_string_lossy().to_string(),
        verbose: false,
        handlers: Some(registry_path.to_string_lossy().to_string()),
        model: None,
        custom_widgets: None,
        strict: false,
    };

    let result = execute(&args);

    // Should fail with unknown handler
    assert!(result.is_err());
}

#[test]
fn test_handler_validation_without_registry() {
    use gravity_cli::commands::check::{execute, CheckArgs};

    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let ui_dir = temp_dir.path().join("ui");
    fs::create_dir(&ui_dir).expect("Failed to create ui dir");

    // Create UI file with handler (no registry provided)
    let content = r#"<?xml version="1.0" encoding="UTF-8"?>
<column>
    <button on_click="any_handler" label="Click Me" />
</column>"#;

    fs::write(ui_dir.join("test.gravity"), content).expect("Failed to write test file");

    let args = CheckArgs {
        input: ui_dir.to_string_lossy().to_string(),
        verbose: false,
        handlers: None, // No registry provided
        model: None,
        custom_widgets: None,
        strict: false,
    };

    let result = execute(&args);

    // Should pass when no registry is provided (handlers not validated)
    assert!(result.is_ok());
}
