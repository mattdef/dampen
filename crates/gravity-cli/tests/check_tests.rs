//! Tests for the check command

use gravity_cli::commands::check::{execute, CheckArgs};
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
    }
}

#[test]
fn test_valid_ui_file() {
    let temp_dir = TempDir::new().unwrap();
    let ui_dir = temp_dir.path().join("ui");
    fs::create_dir(&ui_dir).unwrap();

    let valid_ui = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<column padding=\"10\">
    <text value=\"Hello World\" />
    <button label=\"Click me\" on_click=\"handle_click\" />
</column>";

    fs::write(ui_dir.join("main.gravity"), valid_ui).unwrap();

    let args = create_check_args(Some(ui_dir.to_string_lossy().to_string()), false);

    let result = execute(&args);
    assert!(result.is_ok());
}

#[test]
fn test_invalid_widget_detection() {
    let temp_dir = TempDir::new().unwrap();
    let ui_dir = temp_dir.path().join("ui");
    fs::create_dir(&ui_dir).unwrap();

    let invalid_ui = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<column padding=\"10\">
    <text value=\"Hello World\" />
    <invalid_widget label=\"This should fail\" />
</column>";

    fs::write(ui_dir.join("main.gravity"), invalid_ui).unwrap();

    let args = create_check_args(Some(ui_dir.to_string_lossy().to_string()), false);

    let result = execute(&args);
    assert!(result.is_err());
    let error_msg = result.unwrap_err().to_string();
    assert!(error_msg.contains("invalid_widget"));
}

#[test]
fn test_valid_style_attributes() {
    let temp_dir = TempDir::new().unwrap();
    let ui_dir = temp_dir.path().join("ui");
    fs::create_dir(&ui_dir).unwrap();

    let valid_ui = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<column padding=\"20\" spacing=\"10\">
    <text value=\"Styled text\" color=\"#3498db\" />
    <button label=\"Styled button\" background=\"#e74c3c\" border_width=\"2\"
            border_color=\"#c0392b\" border_radius=\"4\" shadow=\"2 2 4 #00000040\" opacity=\"0.9\" />
</column>";

    fs::write(ui_dir.join("main.gravity"), valid_ui).unwrap();

    let args = create_check_args(Some(ui_dir.to_string_lossy().to_string()), false);

    let result = execute(&args);
    assert!(result.is_ok());
}

#[test]
fn test_invalid_color_value() {
    let temp_dir = TempDir::new().unwrap();
    let ui_dir = temp_dir.path().join("ui");
    fs::create_dir(&ui_dir).unwrap();

    let invalid_ui = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<column>
    <text value=\"Invalid color\" color=\"not-a-color\" />
</column>";

    fs::write(ui_dir.join("main.gravity"), invalid_ui).unwrap();

    let args = create_check_args(Some(ui_dir.to_string_lossy().to_string()), false);

    let result = execute(&args);
    assert!(result.is_err());
    let error_msg = result.unwrap_err().to_string();
    assert!(error_msg.contains("color"));
}

#[test]
fn test_valid_layout_attributes() {
    let temp_dir = TempDir::new().unwrap();
    let ui_dir = temp_dir.path().join("ui");
    fs::create_dir(&ui_dir).unwrap();

    let valid_ui = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<column padding=\"10 20\" spacing=\"5\" align_items=\"center\" justify_content=\"space_between\">
    <text value=\"Text\" width=\"200\" height=\"50\" />
    <button label=\"Button\" width=\"fill\" />
    <container width=\"80%\" height=\"shrink\" padding=\"10\" />
</column>";

    fs::write(ui_dir.join("main.gravity"), valid_ui).unwrap();

    let args = create_check_args(Some(ui_dir.to_string_lossy().to_string()), false);

    let result = execute(&args);
    assert!(result.is_ok());
}

#[test]
fn test_invalid_layout_constraints() {
    let temp_dir = TempDir::new().unwrap();
    let ui_dir = temp_dir.path().join("ui");
    fs::create_dir(&ui_dir).unwrap();

    let invalid_ui = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<column>
    <text value=\"Invalid\" width=\"invalid_length\" />
</column>";

    fs::write(ui_dir.join("main.gravity"), invalid_ui).unwrap();

    let args = create_check_args(Some(ui_dir.to_string_lossy().to_string()), false);

    let result = execute(&args);
    assert!(result.is_err());
    let error_msg = result.unwrap_err().to_string();
    // The error message might say "Invalid length value" or mention "width"
    assert!(error_msg.contains("Invalid length") || error_msg.contains("width"));
}

#[test]
fn test_valid_theme_and_class_references() {
    let temp_dir = TempDir::new().unwrap();
    let ui_dir = temp_dir.path().join("ui");
    fs::create_dir(&ui_dir).unwrap();

    let valid_ui = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<gravity>
    <themes>
        <theme name=\"custom\">
            <palette primary=\"#3498db\" secondary=\"#2ecc71\" success=\"#27ae60\"
                     warning=\"#f39c12\" danger=\"#e74c3c\" background=\"#ecf0f1\"
                     surface=\"#ffffff\" text=\"#2c3e50\" text_secondary=\"#7f8c8d\" />
            <typography font_family=\"Inter\" font_size_base=\"16\" font_size_small=\"12\"
                        font_size_large=\"20\" font_weight=\"normal\" line_height=\"1.5\" />
            <spacing unit=\"8\" />
        </theme>
    </themes>

    <style_classes>
        <class name=\"btn_primary\" background=\"#3498db\" color=\"#ffffff\" border_radius=\"4\" />
        <class name=\"card\" background=\"#ffffff\" padding=\"20\" border_radius=\"8\" />
    </style_classes>

    <global_theme name=\"custom\" />

    <column theme_ref=\"custom\" class=\"card\">
        <text value=\"Themed text\" />
        <button label=\"Primary button\" class=\"btn_primary\" />
    </column>
</gravity>";

    fs::write(ui_dir.join("main.gravity"), valid_ui).unwrap();

    let args = create_check_args(Some(ui_dir.to_string_lossy().to_string()), false);

    let result = execute(&args);
    assert!(result.is_ok());
}

// NOTE: theme_ref attribute is not yet parsed by the parser
// This test is disabled until the parser supports theme_ref
// #[test]
// fn test_unknown_theme_reference() {
//     let temp_dir = TempDir::new().unwrap();
//     let ui_dir = temp_dir.path().join("ui");
//     fs::create_dir(&ui_dir).unwrap();
//
//     let invalid_ui = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>
// <column theme_ref=\"nonexistent\">
//     <text value=\"Test\" />
// </column>";
//
//     fs::write(ui_dir.join("main.gravity"), invalid_ui).unwrap();
//
//     let args = CheckArgs {
//         input: ui_dir.to_string_lossy().to_string(),
//         verbose: false,
//     };
//
//     let result = execute(&args);
//     assert!(result.is_err());
//     let error_msg = result.unwrap_err().to_string();
//     assert!(error_msg.contains("theme") && error_msg.contains("nonexistent"));
// }

#[test]
fn test_unknown_class_reference() {
    let temp_dir = TempDir::new().unwrap();
    let ui_dir = temp_dir.path().join("ui");
    fs::create_dir(&ui_dir).unwrap();

    let invalid_ui = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<column>
    <text value=\"Test\" class=\"nonexistent_class\" />
</column>";

    fs::write(ui_dir.join("main.gravity"), invalid_ui).unwrap();

    let args = create_check_args(Some(ui_dir.to_string_lossy().to_string()), false);

    let result = execute(&args);
    assert!(result.is_err());
    let error_msg = result.unwrap_err().to_string();
    assert!(error_msg.contains("class") && error_msg.contains("nonexistent_class"));
}

#[test]
fn test_negative_spacing() {
    let temp_dir = TempDir::new().unwrap();
    let ui_dir = temp_dir.path().join("ui");
    fs::create_dir(&ui_dir).unwrap();

    let invalid_ui = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<column spacing=\"-5\">
    <text value=\"Test\" />
</column>";

    fs::write(ui_dir.join("main.gravity"), invalid_ui).unwrap();

    let args = create_check_args(Some(ui_dir.to_string_lossy().to_string()), false);

    let result = execute(&args);
    assert!(result.is_err());
    let error_msg = result.unwrap_err().to_string();
    assert!(error_msg.contains("spacing") || error_msg.contains("negative"));
}

#[test]
fn test_min_greater_than_max() {
    let temp_dir = TempDir::new().unwrap();
    let ui_dir = temp_dir.path().join("ui");
    fs::create_dir(&ui_dir).unwrap();

    let invalid_ui = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<column>
    <text value=\"Test\" min_width=\"200\" max_width=\"100\" />
</column>";

    fs::write(ui_dir.join("main.gravity"), invalid_ui).unwrap();

    let args = create_check_args(Some(ui_dir.to_string_lossy().to_string()), false);

    let result = execute(&args);
    assert!(result.is_err());
    let error_msg = result.unwrap_err().to_string();
    assert!(error_msg.contains("min_width") || error_msg.contains("max_width"));
}

#[test]
fn test_invalid_opacity() {
    let temp_dir = TempDir::new().unwrap();
    let ui_dir = temp_dir.path().join("ui");
    fs::create_dir(&ui_dir).unwrap();

    let invalid_ui = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<column>
    <text value=\"Test\" opacity=\"1.5\" />
</column>";

    fs::write(ui_dir.join("main.gravity"), invalid_ui).unwrap();

    let args = create_check_args(Some(ui_dir.to_string_lossy().to_string()), false);

    let result = execute(&args);
    assert!(result.is_err());
    let error_msg = result.unwrap_err().to_string();
    assert!(error_msg.contains("Opacity") || error_msg.contains("opacity"));
}

#[test]
fn test_invalid_shadow() {
    let temp_dir = TempDir::new().unwrap();
    let ui_dir = temp_dir.path().join("ui");
    fs::create_dir(&ui_dir).unwrap();

    let invalid_ui = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<column>
    <text value=\"Test\" shadow=\"invalid\" />
</column>";

    fs::write(ui_dir.join("main.gravity"), invalid_ui).unwrap();

    let args = create_check_args(Some(ui_dir.to_string_lossy().to_string()), false);

    let result = execute(&args);
    assert!(result.is_err());
    let error_msg = result.unwrap_err().to_string();
    assert!(error_msg.contains("shadow"));
}

#[test]
fn test_invalid_transform() {
    let temp_dir = TempDir::new().unwrap();
    let ui_dir = temp_dir.path().join("ui");
    fs::create_dir(&ui_dir).unwrap();

    let invalid_ui = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<column>
    <text value=\"Test\" transform=\"invalid_transform\" />
</column>";

    fs::write(ui_dir.join("main.gravity"), invalid_ui).unwrap();

    let args = create_check_args(Some(ui_dir.to_string_lossy().to_string()), false);

    let result = execute(&args);
    assert!(result.is_err());
    let error_msg = result.unwrap_err().to_string();
    assert!(error_msg.contains("transform"));
}

#[test]
fn test_invalid_border_style() {
    let temp_dir = TempDir::new().unwrap();
    let ui_dir = temp_dir.path().join("ui");
    fs::create_dir(&ui_dir).unwrap();

    let invalid_ui = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<column>
    <text value=\"Test\" border_style=\"dotted_solid\" />
</column>";

    fs::write(ui_dir.join("main.gravity"), invalid_ui).unwrap();

    let args = create_check_args(Some(ui_dir.to_string_lossy().to_string()), false);

    let result = execute(&args);
    assert!(result.is_err());
    let error_msg = result.unwrap_err().to_string();
    assert!(error_msg.contains("Invalid border style") || error_msg.contains("border_style"));
}

#[test]
fn test_invalid_direction() {
    let temp_dir = TempDir::new().unwrap();
    let ui_dir = temp_dir.path().join("ui");
    fs::create_dir(&ui_dir).unwrap();

    let invalid_ui = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<column direction=\"diagonal\">
    <text value=\"Test\" />
</column>";

    fs::write(ui_dir.join("main.gravity"), invalid_ui).unwrap();

    let args = create_check_args(Some(ui_dir.to_string_lossy().to_string()), false);

    let result = execute(&args);
    assert!(result.is_err());
    let error_msg = result.unwrap_err().to_string();
    assert!(error_msg.contains("direction"));
}

#[test]
fn test_invalid_position() {
    let temp_dir = TempDir::new().unwrap();
    let ui_dir = temp_dir.path().join("ui");
    fs::create_dir(&ui_dir).unwrap();

    let invalid_ui = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<column position=\"floating\">
    <text value=\"Test\" />
</column>";

    fs::write(ui_dir.join("main.gravity"), invalid_ui).unwrap();

    let args = create_check_args(Some(ui_dir.to_string_lossy().to_string()), false);

    let result = execute(&args);
    assert!(result.is_err());
    let error_msg = result.unwrap_err().to_string();
    assert!(error_msg.contains("position"));
}

#[test]
fn test_circular_class_dependency() {
    let temp_dir = TempDir::new().unwrap();
    let ui_dir = temp_dir.path().join("ui");
    fs::create_dir(&ui_dir).unwrap();

    let invalid_ui = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<gravity>
    <style_classes>
        <class name=\"class_a\" extends=\"class_b\" background=\"#fff\" />
        <class name=\"class_b\" extends=\"class_a\" color=\"#000\" />
    </style_classes>
    <column>
        <text value=\"Test\" class=\"class_a\" />
    </column>
</gravity>";

    fs::write(ui_dir.join("main.gravity"), invalid_ui).unwrap();

    let args = create_check_args(Some(ui_dir.to_string_lossy().to_string()), false);

    let result = execute(&args);
    assert!(result.is_err());
    let error_msg = result.unwrap_err().to_string();
    assert!(error_msg.contains("circular") || error_msg.contains("class"));
}

// T058: Backward compatibility tests for enhanced validation features
#[test]
fn test_backward_compatibility_without_optional_flags() {
    let temp_dir = TempDir::new().unwrap();
    let ui_dir = temp_dir.path().join("ui");
    fs::create_dir(&ui_dir).unwrap();

    // Create a UI file that would have validation errors with new flags,
    // but should still work in basic mode without them
    let ui_content = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<column>
    <text value=\"Hello World\" />
    <button label=\"Click\" on_click=\"unregistered_handler\" />
    <text_input value=\"{unvalidated_binding}\" />
</column>";

    fs::write(ui_dir.join("main.gravity"), ui_content).unwrap();

    // Create args without new optional flags (backward compatible mode)
    let args = create_check_args(Some(ui_dir.to_string_lossy().to_string()), false);

    // Verify that handlers and model are None (backward compatible)
    assert!(args.handlers.is_none());
    assert!(args.model.is_none());
    assert!(args.custom_widgets.is_none());
    assert!(!args.strict);

    // Should pass basic validation (only widget names and attributes)
    let result = execute(&args);
    assert!(
        result.is_ok(),
        "Basic validation should pass without optional validation flags"
    );
}

#[test]
fn test_backward_compatibility_existing_validation_still_works() {
    let temp_dir = TempDir::new().unwrap();
    let ui_dir = temp_dir.path().join("ui");
    fs::create_dir(&ui_dir).unwrap();

    // Test that existing validations (unknown widget, parse errors) still work
    let invalid_ui = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<column>
    <unknown_widget_type value=\"test\" />
</column>";

    fs::write(ui_dir.join("main.gravity"), invalid_ui).unwrap();

    let args = create_check_args(Some(ui_dir.to_string_lossy().to_string()), false);

    // Should still fail on unknown widget type
    let result = execute(&args);
    assert!(
        result.is_err(),
        "Unknown widget validation should still work"
    );
}

#[test]
fn test_backward_compatibility_helper_function() {
    // Verify that the helper function creates backward-compatible args
    let args = create_check_args(Some("./ui".to_string()), false);

    assert_eq!(args.input, Some("./ui".to_string()));
    assert!(!args.verbose);
    assert!(args.handlers.is_none());
    assert!(args.model.is_none());
    assert!(args.custom_widgets.is_none());
    assert!(!args.strict);
}

#[test]
fn test_enhanced_validation_requires_opt_in() {
    let temp_dir = TempDir::new().unwrap();
    let ui_dir = temp_dir.path().join("ui");
    fs::create_dir(&ui_dir).unwrap();

    // UI file with handler that doesn't exist
    let ui_content = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<column>
    <button label=\"Click\" on_click=\"nonexistent_handler\" />
</column>";

    fs::write(ui_dir.join("main.gravity"), ui_content).unwrap();

    // Without handler registry, should pass (handler validation is opt-in)
    let args_without_registry =
        create_check_args(Some(ui_dir.to_string_lossy().to_string()), false);
    let result = execute(&args_without_registry);
    assert!(
        result.is_ok(),
        "Handler validation should be opt-in via --handlers flag"
    );
}

// New tests for auto-detection functionality

#[test]
fn test_auto_detect_src_ui_directory() {
    let temp_dir = TempDir::new().unwrap();

    // Create src/ui/ directory structure
    let src_ui_dir = temp_dir.path().join("src/ui");
    fs::create_dir_all(&src_ui_dir).unwrap();

    let valid_ui = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<column>
    <text value=\"Hello World\" />
</column>";

    fs::write(src_ui_dir.join("main.gravity"), valid_ui).unwrap();

    // Change to temp directory
    let original_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(temp_dir.path()).unwrap();

    // Run check without --input flag
    let args = create_check_args(None, false);
    let result = execute(&args);

    // Restore original directory
    std::env::set_current_dir(original_dir).unwrap();

    assert!(result.is_ok(), "Should auto-detect src/ui/ directory");
}

#[test]
fn test_auto_detect_ui_directory_fallback() {
    let temp_dir = TempDir::new().unwrap();

    // Create only ui/ directory (not src/ui/)
    let ui_dir = temp_dir.path().join("ui");
    fs::create_dir(&ui_dir).unwrap();

    let valid_ui = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<column>
    <text value=\"Hello World\" />
</column>";

    fs::write(ui_dir.join("main.gravity"), valid_ui).unwrap();

    // Change to temp directory
    let original_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(temp_dir.path()).unwrap();

    // Run check without --input flag
    let args = create_check_args(None, false);
    let result = execute(&args);

    // Restore original directory
    std::env::set_current_dir(original_dir).unwrap();

    assert!(
        result.is_ok(),
        "Should auto-detect ui/ directory as fallback"
    );
}

#[test]
fn test_prefer_src_ui_over_ui() {
    let temp_dir = TempDir::new().unwrap();

    // Create both src/ui/ and ui/ directories
    let src_ui_dir = temp_dir.path().join("src/ui");
    let ui_dir = temp_dir.path().join("ui");
    fs::create_dir_all(&src_ui_dir).unwrap();
    fs::create_dir(&ui_dir).unwrap();

    // Put valid content in src/ui/
    let valid_ui = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<column>
    <text value=\"From src/ui\" />
</column>";
    fs::write(src_ui_dir.join("main.gravity"), valid_ui).unwrap();

    // Put invalid content in ui/ to verify src/ui/ is used
    let invalid_ui = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<column>
    <invalid_widget value=\"Should not be used\" />
</column>";
    fs::write(ui_dir.join("main.gravity"), invalid_ui).unwrap();

    // Change to temp directory
    let original_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(temp_dir.path()).unwrap();

    // Run check without --input flag
    let args = create_check_args(None, false);
    let result = execute(&args);

    // Restore original directory
    std::env::set_current_dir(original_dir).unwrap();

    // Should succeed because src/ui/ is preferred
    assert!(result.is_ok(), "Should prefer src/ui/ over ui/");
}

#[test]
fn test_explicit_input_overrides_auto_detection() {
    let temp_dir = TempDir::new().unwrap();

    // Create both src/ui/ and custom/ directories
    let src_ui_dir = temp_dir.path().join("src/ui");
    let custom_dir = temp_dir.path().join("custom");
    fs::create_dir_all(&src_ui_dir).unwrap();
    fs::create_dir(&custom_dir).unwrap();

    // Put content in both
    let ui1 = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<column>
    <text value=\"From src/ui\" />
</column>";
    fs::write(src_ui_dir.join("main.gravity"), ui1).unwrap();

    let ui2 = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<column>
    <text value=\"From custom\" />
    <button label=\"Custom button\" />
</column>";
    fs::write(custom_dir.join("main.gravity"), ui2).unwrap();

    // Explicitly specify custom directory
    let args = create_check_args(Some(custom_dir.to_string_lossy().to_string()), false);
    let result = execute(&args);

    assert!(
        result.is_ok(),
        "Explicit --input should override auto-detection"
    );
}

#[test]
fn test_error_when_no_ui_directory_found() {
    let temp_dir = TempDir::new().unwrap();

    // Create an empty directory (no ui/ or src/ui/)

    // Change to temp directory
    let original_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(temp_dir.path()).unwrap();

    // Run check without --input flag
    let args = create_check_args(None, false);
    let result = execute(&args);

    // Restore original directory
    std::env::set_current_dir(original_dir).unwrap();

    assert!(result.is_err(), "Should error when no UI directory found");
    let error_msg = result.unwrap_err().to_string();
    assert!(
        error_msg.contains("No UI directory found") || error_msg.contains("src/ui"),
        "Error message should be helpful: {}",
        error_msg
    );
}

#[test]
fn test_auto_discover_handlers_json_in_root() {
    let temp_dir = TempDir::new().unwrap();

    // Create src/ui/ directory
    let src_ui_dir = temp_dir.path().join("src/ui");
    fs::create_dir_all(&src_ui_dir).unwrap();

    let ui_content = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<column>
    <button label=\"Click\" on_click=\"handle_click\" />
</column>";
    fs::write(src_ui_dir.join("main.gravity"), ui_content).unwrap();

    // Create handlers.json in project root
    let handlers_json = r#"[
        {
            "name": "handle_click",
            "param_type": null,
            "returns_command": false
        }
    ]"#;
    fs::write(temp_dir.path().join("handlers.json"), handlers_json).unwrap();

    // Change to temp directory
    let original_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(temp_dir.path()).unwrap();

    // Run check without --handlers flag (should auto-discover)
    let args = create_check_args(None, true); // verbose to see discovery
    let result = execute(&args);

    // Restore original directory
    std::env::set_current_dir(original_dir).unwrap();

    assert!(result.is_ok(), "Should auto-discover handlers.json in root");
}

#[test]
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
    fs::write(src_ui_dir.join("main.gravity"), ui_content).unwrap();

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
fn test_explicit_handlers_overrides_auto_discovery() {
    let temp_dir = TempDir::new().unwrap();

    // Create src/ui/ directory
    let src_ui_dir = temp_dir.path().join("src/ui");
    fs::create_dir_all(&src_ui_dir).unwrap();

    let ui_content = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<column>
    <button label=\"Click\" on_click=\"custom_handler\" />
</column>";
    fs::write(src_ui_dir.join("main.gravity"), ui_content).unwrap();

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
    fs::write(custom_dir.join("main.gravity"), ui_content).unwrap();

    // Old-style explicit path specification
    let args = CheckArgs {
        input: Some(custom_dir.to_string_lossy().to_string()),
        verbose: false,
        handlers: None,
        model: None,
        custom_widgets: None,
        strict: false,
    };

    let result = execute(&args);
    assert!(
        result.is_ok(),
        "Backward compatibility: explicit paths should still work"
    );
}
