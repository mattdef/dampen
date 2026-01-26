use assert_cmd::Command;
use dampen_cli::commands::check::{CheckArgs, execute};
use predicates::prelude::*;
use serial_test::serial;
use std::fs;
use tempfile::TempDir;

// Helper function to create CheckArgs with default values for new fields
fn create_check_args(input: Option<String>, verbose: bool) -> CheckArgs {
    CheckArgs {
        input,
        verbose,
        handlers: None,
        model: None,
        custom_widgets: None,
        strict: false,
        show_widget_versions: false,
    }
}

#[test]
#[serial]
fn test_auto_discover_handlers_json_in_src() {
    let temp_dir = TempDir::new().unwrap();

    // Create src/ui/ directory
    let src_ui_dir = temp_dir.path().join("src/ui");
    let src_dir = temp_dir.path().join("src");
    fs::create_dir_all(&src_ui_dir).unwrap();

    let ui_content = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<column>
    <button label=\"Click\" on_click=\"handle_click\" />
</column>";
    fs::write(src_ui_dir.join("main.dampen"), ui_content).unwrap();

    // Create handlers.json in src/ directory
    let handlers_json = r#"[
        {
            "name": "handle_click",
            "param_type": null,
            "returns_command": false
        }
    ]"#;
    fs::write(src_dir.join("handlers.json"), handlers_json).unwrap();

    // Change to temp directory
    let original_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(temp_dir.path()).unwrap();

    // Run check without --handlers flag (should auto-discover)
    let args = create_check_args(None, false);
    let result = execute(&args);

    // Restore original directory
    std::env::set_current_dir(original_dir).unwrap();

    assert!(result.is_ok(), "Should auto-discover handlers.json in src/");
}

#[test]
#[serial]
fn test_explicit_handlers_overrides_auto_discovery() {
    let temp_dir = TempDir::new().unwrap();

    // Create src/ui/ directory
    let src_ui_dir = temp_dir.path().join("src/ui");
    fs::create_dir_all(&src_ui_dir).unwrap();

    let ui_content = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<column>
    <button label=\"Click\" on_click=\"custom_handler\" />
</column>";
    fs::write(src_ui_dir.join("main.dampen"), ui_content).unwrap();

    // Create handlers.json in root (would be auto-discovered)
    let root_handlers = r#"[
        {
            "name": "handle_click",
            "param_type": null,
            "returns_command": false
        }
    ]"#;
    fs::write(temp_dir.path().join("handlers.json"), root_handlers).unwrap();

    // Create custom handlers.json
    let custom_handlers = r#"[
        {
            "name": "custom_handler",
            "param_type": null,
            "returns_command": false
        }
    ]"#;
    let custom_path = temp_dir.path().join("custom_handlers.json");
    fs::write(&custom_path, custom_handlers).unwrap();

    // Run with explicit --handlers flag
    let args = CheckArgs {
        input: None,
        verbose: false,
        handlers: Some(custom_path.to_string_lossy().to_string()),
        model: None,
        custom_widgets: None,
        strict: false,
        show_widget_versions: false,
    };

    // Change to temp directory
    let original_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(temp_dir.path()).unwrap();

    let result = execute(&args);

    // Restore original directory
    std::env::set_current_dir(original_dir).unwrap();

    assert!(
        result.is_ok(),
        "Explicit --handlers should override auto-discovery"
    );
}

#[test]
fn test_backward_compatibility_with_explicit_paths() {
    let temp_dir = TempDir::new().unwrap();

    // Create custom directory structure
    let custom_dir = temp_dir.path().join("custom");
    fs::create_dir(&custom_dir).unwrap();

    let ui_content = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<column>
    <text value=\"Test\" />
</column>";
    fs::write(custom_dir.join("main.dampen"), ui_content).unwrap();

    // Old-style explicit path specification
    let args = CheckArgs {
        input: Some(custom_dir.to_string_lossy().to_string()),
        verbose: false,
        handlers: None,
        model: None,
        custom_widgets: None,
        strict: false,
        show_widget_versions: false,
    };

    let result = execute(&args);
    assert!(
        result.is_ok(),
        "Backward compatibility: explicit paths should still work"
    );
}

#[test]
fn test_error_message_format_unchanged() {
    let temp = TempDir::new().unwrap();
    let ui_dir = temp.path().join("ui");
    fs::create_dir_all(&ui_dir).unwrap();

    fs::write(
        ui_dir.join("main.dampen"),
        r#"
<dampen>
    <column>
        <button on_clik="msg" />
    </column>
</dampen>
    "#,
    )
    .unwrap();

    let mut cmd = Command::cargo_bin("dampen").unwrap();
    let result = cmd.arg("check").arg("--input").arg(ui_dir).assert();

    result
        .failure()
        .stderr(predicate::str::contains(
            "[ERROR] Unknown attribute 'on_clik' for widget 'button'",
        ))
        .stderr(predicate::str::contains("Did you mean 'on_click'?"));
}

#[test]
fn test_suggestion_logic_unchanged() {
    let temp = TempDir::new().unwrap();
    let ui_dir = temp.path().join("ui");
    fs::create_dir_all(&ui_dir).unwrap();

    fs::write(
        ui_dir.join("main.dampen"),
        r#"
<dampen>
    <column>
        <text valeu="Hello" />
    </column>
</dampen>
    "#,
    )
    .unwrap();

    let mut cmd = Command::cargo_bin("dampen").unwrap();
    let result = cmd.arg("check").arg("--input").arg(ui_dir).assert();

    result
        .failure()
        .stderr(predicate::str::contains("Did you mean 'value'?"));
}
