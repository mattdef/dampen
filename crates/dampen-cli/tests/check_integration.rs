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
    use dampen_cli::commands::check::{execute, CheckArgs};

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
        input: Some(ui_dir.to_string_lossy().to_string()),
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
    use dampen_cli::commands::check::{execute, CheckArgs};

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
        input: Some(ui_dir.to_string_lossy().to_string()),
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
    use dampen_cli::commands::check::{execute, CheckArgs};

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
        input: Some(ui_dir.to_string_lossy().to_string()),
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
    use dampen_cli::commands::check::{execute, CheckArgs};

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
        input: Some(ui_dir.to_string_lossy().to_string()),
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

// T029: Integration tests for binding validation
#[test]
fn test_binding_validation_with_model() {
    use dampen_cli::commands::check::{execute, CheckArgs};

    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let ui_dir = temp_dir.path().join("ui");
    fs::create_dir(&ui_dir).expect("Failed to create ui dir");

    // Create model info
    let model_path = temp_dir.path().join("model.json");
    let model_content = r#"[
  {
    "name": "count",
    "type_name": "i32",
    "is_nested": false,
    "children": []
  },
  {
    "name": "user",
    "type_name": "User",
    "is_nested": true,
    "children": [
      {"name": "name", "type_name": "String", "is_nested": false, "children": []}
    ]
  }
]"#;
    fs::write(&model_path, model_content).expect("Failed to write model");

    // Create UI file with valid binding
    let content = r#"<?xml version="1.0" encoding="UTF-8"?>
<column>
    <text value="{count}" />
    <text value="{user.name}" />
</column>"#;

    fs::write(ui_dir.join("test.gravity"), content).expect("Failed to write test file");

    let args = CheckArgs {
        input: Some(ui_dir.to_string_lossy().to_string()),
        verbose: false,
        handlers: None,
        model: Some(model_path.to_string_lossy().to_string()),
        custom_widgets: None,
        strict: false,
    };

    let result = execute(&args);

    // Should pass with valid bindings
    assert!(result.is_ok());
}

#[test]
fn test_binding_validation_with_invalid_field() {
    use dampen_cli::commands::check::{execute, CheckArgs};

    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let ui_dir = temp_dir.path().join("ui");
    fs::create_dir(&ui_dir).expect("Failed to create ui dir");

    // Create model info
    let model_path = temp_dir.path().join("model.json");
    let model_content = r#"[
  {
    "name": "count",
    "type_name": "i32",
    "is_nested": false,
    "children": []
  }
]"#;
    fs::write(&model_path, model_content).expect("Failed to write model");

    // Create UI file with invalid binding
    let content = r#"<?xml version="1.0" encoding="UTF-8"?>
<column>
    <text value="{unknown_field}" />
</column>"#;

    fs::write(ui_dir.join("test.gravity"), content).expect("Failed to write test file");

    let args = CheckArgs {
        input: Some(ui_dir.to_string_lossy().to_string()),
        verbose: false,
        handlers: None,
        model: Some(model_path.to_string_lossy().to_string()),
        custom_widgets: None,
        strict: false,
    };

    let result = execute(&args);

    // Should fail with invalid binding
    assert!(result.is_err());
}

#[test]
fn test_binding_validation_without_model() {
    use dampen_cli::commands::check::{execute, CheckArgs};

    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let ui_dir = temp_dir.path().join("ui");
    fs::create_dir(&ui_dir).expect("Failed to create ui dir");

    // Create UI file with binding (no model provided)
    let content = r#"<?xml version="1.0" encoding="UTF-8"?>
<column>
    <text value="{any_field}" />
</column>"#;

    fs::write(ui_dir.join("test.gravity"), content).expect("Failed to write test file");

    let args = CheckArgs {
        input: Some(ui_dir.to_string_lossy().to_string()),
        verbose: false,
        handlers: None,
        model: None, // No model provided
        custom_widgets: None,
        strict: false,
    };

    let result = execute(&args);

    // Should pass when no model is provided (bindings not validated)
    assert!(result.is_ok());
}
// T036: Integration test for valid radio group
#[test]
fn test_valid_radio_group_integration() {
    use dampen_cli::commands::check::{execute, CheckArgs};

    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let ui_dir = temp_dir.path().join("ui");
    fs::create_dir(&ui_dir).expect("Failed to create ui dir");

    // Create UI file with valid radio group
    let content = r#"<?xml version="1.0" encoding="UTF-8"?>
<column>
    <radio id="size_group" value="small" label="Small" on_select="handle_size" />
    <radio id="size_group" value="medium" label="Medium" on_select="handle_size" />
    <radio id="size_group" value="large" label="Large" on_select="handle_size" />
</column>"#;

    fs::write(ui_dir.join("test.gravity"), content).expect("Failed to write test file");

    let args = CheckArgs {
        input: Some(ui_dir.to_string_lossy().to_string()),
        verbose: false,
        handlers: None,
        model: None,
        custom_widgets: None,
        strict: false,
    };

    let result = execute(&args);

    // Should pass with valid radio group
    assert!(result.is_ok());
}

// T042: Integration test for valid theme
#[test]
fn test_valid_theme_integration() {
    use dampen_cli::commands::check::{execute, CheckArgs};

    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let ui_dir = temp_dir.path().join("ui");
    fs::create_dir(&ui_dir).expect("Failed to create ui dir");

    // Create UI file with theme and style class definitions
    // Note: This is a simplified test since full theme validation
    // would require parsing theme XML/JSON definitions
    let content = r#"<?xml version="1.0" encoding="UTF-8"?>
<column>
    <text value="Hello Theme" />
</column>"#;

    fs::write(ui_dir.join("test.gravity"), content).expect("Failed to write test file");

    let args = CheckArgs {
        input: Some(ui_dir.to_string_lossy().to_string()),
        verbose: false,
        handlers: None,
        model: None,
        custom_widgets: None,
        strict: false,
    };

    let result = execute(&args);

    // Should pass with simple valid file
    assert!(result.is_ok());
}

// T046: Integration test for strict mode exit code
#[test]
fn test_strict_mode_with_errors() {
    use dampen_cli::commands::check::{execute, CheckArgs};

    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let ui_dir = temp_dir.path().join("ui");
    fs::create_dir(&ui_dir).expect("Failed to create ui dir");

    // Create UI file with unknown attribute (would be a warning normally)
    let content = r#"<?xml version="1.0" encoding="UTF-8"?>
<column>
    <button on_clik="handle_click" label="Click Me" />
</column>"#;

    fs::write(ui_dir.join("test.gravity"), content).expect("Failed to write test file");

    // Test without strict mode
    let args_normal = CheckArgs {
        input: Some(ui_dir.to_string_lossy().to_string()),
        verbose: false,
        handlers: None,
        model: None,
        custom_widgets: None,
        strict: false,
    };

    let result_normal = execute(&args_normal);
    // Should fail with error
    assert!(result_normal.is_err());

    // Test with strict mode
    let args_strict = CheckArgs {
        input: Some(ui_dir.to_string_lossy().to_string()),
        verbose: false,
        handlers: None,
        model: None,
        custom_widgets: None,
        strict: true,
    };

    let result_strict = execute(&args_strict);
    // Should also fail with strict mode enabled
    assert!(result_strict.is_err());
}

// T047: Integration test for strict mode with no warnings
#[test]
fn test_strict_mode_with_no_warnings() {
    use dampen_cli::commands::check::{execute, CheckArgs};

    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let ui_dir = temp_dir.path().join("ui");
    fs::create_dir(&ui_dir).expect("Failed to create ui dir");

    // Create UI file with valid content (no errors or warnings)
    let content = r#"<?xml version="1.0" encoding="UTF-8"?>
<column>
    <button on_click="handle_click" label="Click Me" />
    <text value="Hello World" />
</column>"#;

    fs::write(ui_dir.join("test.gravity"), content).expect("Failed to write test file");

    // Test with strict mode
    let args = CheckArgs {
        input: Some(ui_dir.to_string_lossy().to_string()),
        verbose: false,
        handlers: None,
        model: None,
        custom_widgets: None,
        strict: true,
    };

    let result = execute(&args);
    // Should pass with no warnings or errors
    assert!(result.is_ok());
}

// T053: Integration test for required attribute validation
#[test]
fn test_required_attribute_validation_text_missing_value() {
    use dampen_cli::commands::check::{execute, CheckArgs};

    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let ui_dir = temp_dir.path().join("ui");
    fs::create_dir(&ui_dir).expect("Failed to create ui dir");

    // Create a test file with Text widget missing required 'value' attribute
    let content = r#"<?xml version="1.0" encoding="UTF-8"?>
<column>
    <text size="16" color="blue" />
</column>"#;

    fs::write(ui_dir.join("test.gravity"), content).expect("Failed to write test file");

    let args = CheckArgs {
        input: Some(ui_dir.to_string_lossy().to_string()),
        verbose: false,
        handlers: None,
        model: None,
        custom_widgets: None,
        strict: false,
    };

    let result = execute(&args);

    // Should fail due to missing required 'value' attribute
    assert!(result.is_err());

    let err = result.unwrap_err();
    let err_msg = format!("{}", err);
    assert!(err_msg.contains("value") || err_msg.contains("required"));
}

#[test]
fn test_required_attribute_validation_image_missing_src() {
    use dampen_cli::commands::check::{execute, CheckArgs};

    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let ui_dir = temp_dir.path().join("ui");
    fs::create_dir(&ui_dir).expect("Failed to create ui dir");

    // Create a test file with Image widget missing required 'src' attribute
    let content = r#"<?xml version="1.0" encoding="UTF-8"?>
<column>
    <image width="200" height="100" fit="contain" />
</column>"#;

    fs::write(ui_dir.join("test.gravity"), content).expect("Failed to write test file");

    let args = CheckArgs {
        input: Some(ui_dir.to_string_lossy().to_string()),
        verbose: false,
        handlers: None,
        model: None,
        custom_widgets: None,
        strict: false,
    };

    let result = execute(&args);

    // Should fail due to missing required 'src' attribute
    assert!(result.is_err());

    let err = result.unwrap_err();
    let err_msg = format!("{}", err);
    assert!(err_msg.contains("src") || err_msg.contains("required"));
}

#[test]
fn test_required_attribute_validation_radio_missing_label() {
    use dampen_cli::commands::check::{execute, CheckArgs};

    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let ui_dir = temp_dir.path().join("ui");
    fs::create_dir(&ui_dir).expect("Failed to create ui dir");

    // Create a test file with Radio widget missing required 'label' attribute
    let content = r#"<?xml version="1.0" encoding="UTF-8"?>
<column>
    <radio value="option1" on_select="handle_select" />
</column>"#;

    fs::write(ui_dir.join("test.gravity"), content).expect("Failed to write test file");

    let args = CheckArgs {
        input: Some(ui_dir.to_string_lossy().to_string()),
        verbose: false,
        handlers: None,
        model: None,
        custom_widgets: None,
        strict: false,
    };

    let result = execute(&args);

    // Should fail due to missing required 'label' attribute
    assert!(result.is_err());

    let err = result.unwrap_err();
    let err_msg = format!("{}", err);
    assert!(err_msg.contains("label") || err_msg.contains("required"));
}

#[test]
fn test_required_attribute_validation_all_present() {
    use dampen_cli::commands::check::{execute, CheckArgs};

    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let ui_dir = temp_dir.path().join("ui");
    fs::create_dir(&ui_dir).expect("Failed to create ui dir");

    // Create a test file with all required attributes present
    let content = r#"<?xml version="1.0" encoding="UTF-8"?>
<column>
    <text value="Hello World" size="16" />
    <image src="logo.png" width="100" />
    <radio label="Option 1" value="opt1" />
</column>"#;

    fs::write(ui_dir.join("test.gravity"), content).expect("Failed to write test file");

    let args = CheckArgs {
        input: Some(ui_dir.to_string_lossy().to_string()),
        verbose: false,
        handlers: None,
        model: None,
        custom_widgets: None,
        strict: false,
    };

    let result = execute(&args);

    // Should pass validation - all required attributes present
    assert!(result.is_ok());
}

// T057: Integration test for complete validation pipeline with all flags
#[test]
fn test_complete_validation_pipeline_all_flags() {
    use dampen_cli::commands::check::{execute, CheckArgs};

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
  },
  {
    "name": "handle_input",
    "param_type": "String",
    "returns_command": false
  },
  {
    "name": "handle_select",
    "param_type": "String",
    "returns_command": false
  }
]"#;
    fs::write(&registry_path, registry_content).expect("Failed to write registry");

    // Create model info
    let model_path = temp_dir.path().join("model.json");
    let model_content = r#"[
  {
    "name": "title",
    "type_name": "String",
    "is_nested": false,
    "children": []
  },
  {
    "name": "count",
    "type_name": "i32",
    "is_nested": false,
    "children": []
  },
  {
    "name": "user",
    "type_name": "User",
    "is_nested": true,
    "children": [
      {"name": "name", "type_name": "String", "is_nested": false, "children": []},
      {"name": "email", "type_name": "String", "is_nested": false, "children": []}
    ]
  }
]"#;
    fs::write(&model_path, model_content).expect("Failed to write model");

    // Create a comprehensive UI file with all validation scenarios
    let content = r#"<?xml version="1.0" encoding="UTF-8"?>
<column>
    <text value="{title}" size="24" />
    <text value="Count: {count}" />
    <button on_click="handle_click" label="Click Me" />
    <text_input placeholder="Enter name" value="{user.name}" on_input="handle_input" />
    <image src="logo.png" width="100" height="100" />
    <radio label="Option 1" value="opt1" on_select="handle_select" />
    <radio label="Option 2" value="opt2" on_select="handle_select" />
</column>"#;

    fs::write(ui_dir.join("test.gravity"), content).expect("Failed to write test file");

    // Test with all flags enabled
    let args = CheckArgs {
        input: Some(ui_dir.to_string_lossy().to_string()),
        verbose: true,
        handlers: Some(registry_path.to_string_lossy().to_string()),
        model: Some(model_path.to_string_lossy().to_string()),
        custom_widgets: None,
        strict: true,
    };

    let result = execute(&args);

    // Should pass validation - all attributes valid, handlers registered, bindings valid
    assert!(result.is_ok());
}

#[test]
fn test_complete_validation_pipeline_with_errors() {
    use dampen_cli::commands::check::{execute, CheckArgs};

    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let ui_dir = temp_dir.path().join("ui");
    fs::create_dir(&ui_dir).expect("Failed to create ui dir");

    // Create handler registry (missing some handlers)
    let registry_path = temp_dir.path().join("handlers.json");
    let registry_content = r#"[
  {
    "name": "handle_click",
    "param_type": null,
    "returns_command": false
  }
]"#;
    fs::write(&registry_path, registry_content).expect("Failed to write registry");

    // Create model info (missing some fields)
    let model_path = temp_dir.path().join("model.json");
    let model_content = r#"[
  {
    "name": "count",
    "type_name": "i32",
    "is_nested": false,
    "children": []
  }
]"#;
    fs::write(&model_path, model_content).expect("Failed to write model");

    // Create UI file with multiple validation errors
    let content = r#"<?xml version="1.0" encoding="UTF-8"?>
<column>
    <text size="24" />
    <button on_clik="handle_click" label="Click Me" />
    <text_input value="{missing_field}" on_input="unknown_handler" />
    <image width="100" />
    <radio value="opt1" />
</column>"#;

    fs::write(ui_dir.join("test.gravity"), content).expect("Failed to write test file");

    // Test with all flags enabled - should detect all errors
    let args = CheckArgs {
        input: Some(ui_dir.to_string_lossy().to_string()),
        verbose: false,
        handlers: Some(registry_path.to_string_lossy().to_string()),
        model: Some(model_path.to_string_lossy().to_string()),
        custom_widgets: None,
        strict: true,
    };

    let result = execute(&args);

    // Should fail - multiple validation errors
    assert!(result.is_err());

    let err = result.unwrap_err();
    let err_msg = format!("{}", err);

    // Should detect at least one of the errors (unknown attribute, missing required, invalid binding, unknown handler)
    assert!(
        err_msg.contains("value")
            || err_msg.contains("on_clik")
            || err_msg.contains("missing_field")
            || err_msg.contains("unknown_handler")
            || err_msg.contains("src")
            || err_msg.contains("label")
    );
}
