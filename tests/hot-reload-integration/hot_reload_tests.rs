//! Integration tests for hot-reload functionality
//!
//! These tests validate the complete hot-reload flow including:
//! - State preservation across reloads
//! - Parse error handling
//! - Rapid successive file changes
//!
//! Tests require dampen-dev crate to be available.

use dampen_core::binding::UiBindable;
use dampen_core::handler::HandlerRegistry;
use dampen_core::parser;
use dampen_core::state::AppState;
use dampen_dev::reload::{attempt_hot_reload, HotReloadContext, ReloadResult};
use serde::{Deserialize, Serialize};
use std::thread;
use std::time::{Duration, Instant};

/// Test model representing application state
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
struct CounterModel {
    count: i32,
    name: String,
}

impl UiBindable for CounterModel {
    fn get_field(&self, path: &[&str]) -> Option<dampen_core::binding::BindingValue> {
        match path {
            ["count"] => Some(dampen_core::binding::BindingValue::Integer(
                self.count as i64,
            )),
            ["name"] => Some(dampen_core::binding::BindingValue::String(
                self.name.clone(),
            )),
            _ => None,
        }
    }

    fn available_fields() -> Vec<String> {
        vec!["count".to_string(), "name".to_string()]
    }
}

/// Create a simple handler registry for testing
fn create_test_handlers() -> HandlerRegistry {
    HandlerRegistry::new()
}

/// T088: Test that application state is preserved across hot-reloads
#[test]
fn test_state_preservation_across_reload() {
    // Initial XML with a counter UI
    let initial_xml = r#"
<dampen version="1.1" encoding="utf-8">
    <column>
        <text value="Count: {count}" />
        <button label="Increment" />
    </column>
</dampen>"#;

    // Modified XML (changed button label, added text)
    let modified_xml = r#"
<dampen version="1.1" encoding="utf-8">
    <column>
        <text value="Count: {count}" />
        <text value="Name: {name}" />
        <button label="Add One" />
    </column>
</dampen>"#;

    // Create initial state with non-default values
    let initial_doc = parser::parse(initial_xml).expect("Initial parse should succeed");
    let mut model = CounterModel {
        count: 42,
        name: "Alice".to_string(),
    };

    let handlers = create_test_handlers();
    let app_state = AppState::with_all(initial_doc, model.clone(), handlers);

    // Simulate user interaction (change model state)
    model.count = 100;
    model.name = "Bob".to_string();

    // Create hot-reload context
    let mut context = HotReloadContext::<CounterModel>::new();

    // Attempt hot-reload with modified XML
    let result = attempt_hot_reload(
        modified_xml,
        &AppState::with_all(
            app_state.document.clone(),
            model.clone(),
            create_test_handlers(),
        ),
        &mut context,
        create_test_handlers,
    );

    // Verify reload succeeded
    match result {
        ReloadResult::Success(new_state) => {
            // Verify state was preserved
            assert_eq!(new_state.model.count, 100, "Count should be preserved");
            assert_eq!(new_state.model.name, "Bob", "Name should be preserved");

            // Verify document was updated (new widget count)
            assert_eq!(
                new_state.document.root.children.len(),
                3,
                "Should have 3 children (2 texts + 1 button)"
            );
        }
        ReloadResult::ParseError(e) => {
            panic!("Hot-reload should not fail with parse error: {}", e);
        }
        ReloadResult::ValidationError(errors) => {
            panic!(
                "Hot-reload should not fail with validation errors: {:?}",
                errors
            );
        }
        ReloadResult::StateRestoreWarning(_, e) => {
            panic!("State restoration should not fail: {}", e);
        }
    }
}

/// T088: Test state preservation with default fallback when deserialization fails
#[test]
fn test_state_preservation_with_schema_change() {
    let initial_xml = r#"
<dampen version="1.1" encoding="utf-8">
    <column>
        <text value="Count: {count}" />
    </column>
</dampen>"#;

    let modified_xml = r#"
<dampen version="1.1" encoding="utf-8">
    <column>
        <text value="Count: {count}" />
        <text value="Updated UI" />
    </column>
</dampen>"#;

    // Create initial state
    let initial_doc = parser::parse(initial_xml).expect("Initial parse should succeed");
    let model = CounterModel {
        count: 50,
        name: "Test".to_string(),
    };

    let app_state = AppState::with_all(initial_doc, model.clone(), create_test_handlers());

    // Create context and attempt reload
    let mut context = HotReloadContext::<CounterModel>::new();

    let result = attempt_hot_reload(modified_xml, &app_state, &mut context, create_test_handlers);

    // Should succeed (even if state restoration had issues, we have a fallback)
    match result {
        ReloadResult::Success(new_state) => {
            // State should be preserved
            assert_eq!(new_state.model.count, 50);
            assert_eq!(new_state.model.name, "Test");
        }
        ReloadResult::StateRestoreWarning(new_state, warning) => {
            // If deserialization failed, we should have default state
            println!(
                "State restore warning (expected in some cases): {}",
                warning
            );
            assert_eq!(new_state.model.count, 0, "Should use default count");
            assert_eq!(new_state.model.name, "", "Should use default name");
        }
        ReloadResult::ParseError(e) => {
            panic!("Should not fail with parse error: {}", e);
        }
        ReloadResult::ValidationError(e) => {
            panic!("Should not fail with validation error: {:?}", e);
        }
    }
}

/// T089: Test that parse errors are handled gracefully without crashing
#[test]
fn test_parse_error_handling() {
    let valid_xml = r#"
<dampen version="1.1" encoding="utf-8">
    <column>
        <text value="Hello" />
    </column>
</dampen>"#;

    // Invalid XML - missing closing tag
    let invalid_xml = r#"
<dampen version="1.1" encoding="utf-8">
    <column>
        <text value="Hello" />
    </column>
"#;

    // Create initial valid state
    let initial_doc = parser::parse(valid_xml).expect("Initial parse should succeed");
    let model = CounterModel::default();
    let app_state = AppState::with_all(initial_doc.clone(), model, create_test_handlers());

    // Create context
    let mut context = HotReloadContext::<CounterModel>::new();

    // Attempt reload with invalid XML
    let result = attempt_hot_reload(invalid_xml, &app_state, &mut context, create_test_handlers);

    // Should fail with parse error
    match result {
        ReloadResult::ParseError(err) => {
            // Verify error contains useful information
            assert!(!err.message.is_empty(), "Error message should not be empty");
            assert!(err.span.line > 0, "Error should have line number");
            println!("Parse error caught successfully: {}", err);
        }
        ReloadResult::Success(_) => {
            panic!("Hot-reload should fail with invalid XML");
        }
        ReloadResult::ValidationError(_) => {
            panic!("Should fail with parse error, not validation error");
        }
        ReloadResult::StateRestoreWarning(_, _) => {
            panic!("Should fail with parse error, not state restore warning");
        }
    }

    // Verify original state is unchanged (reload was rejected)
    assert_eq!(
        app_state.document.root.children.len(),
        initial_doc.root.children.len(),
        "Document should be unchanged after failed reload"
    );
}

/// T089: Test handling of unknown widget errors
#[test]
fn test_unknown_widget_error_handling() {
    let valid_xml = r#"
<dampen version="1.1" encoding="utf-8">
    <column>
        <text value="Hello" />
    </column>
</dampen>"#;

    // Invalid XML - unknown widget type
    let invalid_xml = r#"
<dampen version="1.1" encoding="utf-8">
    <column>
        <unknown_widget />
    </column>
</dampen>"#;

    // Create initial state
    let initial_doc = parser::parse(valid_xml).expect("Initial parse should succeed");
    let model = CounterModel::default();
    let app_state = AppState::with_all(initial_doc, model, create_test_handlers());

    // Create context
    let mut context = HotReloadContext::<CounterModel>::new();

    // Attempt reload with unknown widget
    let result = attempt_hot_reload(invalid_xml, &app_state, &mut context, create_test_handlers);

    // Should fail with parse error for unknown widget
    match result {
        ReloadResult::ParseError(err) => {
            println!("Unknown widget error caught: {}", err);
            // Error should indicate unknown widget
            assert!(
                err.message.to_lowercase().contains("unknown")
                    || err.message.to_lowercase().contains("widget"),
                "Error should mention unknown widget"
            );
        }
        _ => {
            panic!("Should fail with parse error for unknown widget");
        }
    }
}

/// T090: Test that rapid successive file saves are handled correctly
#[test]
fn test_rapid_successive_reloads() {
    let base_xml = r#"
<dampen version="1.1" encoding="utf-8">
    <column>
        <text value="Version {count}" />
    </column>
</dampen>"#;

    // Create initial state
    let initial_doc = parser::parse(base_xml).expect("Initial parse should succeed");
    let initial_model = CounterModel {
        count: 0,
        name: "Initial".to_string(),
    };

    let mut app_state =
        AppState::with_all(initial_doc, initial_model.clone(), create_test_handlers());

    let mut context = HotReloadContext::<CounterModel>::new();

    // Perform 10 rapid reloads in succession
    let start = Instant::now();
    let reload_count = 10;

    for i in 1..=reload_count {
        // Generate slightly different XML each time
        let modified_xml = format!(
            r#"
<dampen version="1.1" encoding="utf-8">
    <column>
        <text value="Version {{count}}" />
        <text value="Reload {}" />
    </column>
</dampen>"#,
            i
        );

        // Update model state
        let mut model = app_state.model.clone();
        model.count = i;
        model.name = format!("Reload-{}", i);

        // Perform reload
        let result = attempt_hot_reload(
            &modified_xml,
            &AppState::with_all(
                app_state.document.clone(),
                model.clone(),
                create_test_handlers(),
            ),
            &mut context,
            create_test_handlers,
        );

        match result {
            ReloadResult::Success(new_state) => {
                // Verify state was preserved
                assert_eq!(new_state.model.count, i, "Count should match iteration");
                assert_eq!(
                    new_state.model.name,
                    format!("Reload-{}", i),
                    "Name should match iteration"
                );

                // Update app_state for next iteration
                app_state = new_state;
            }
            ReloadResult::ParseError(e) => {
                panic!("Reload {} failed with parse error: {}", i, e);
            }
            ReloadResult::ValidationError(e) => {
                panic!("Reload {} failed with validation error: {:?}", i, e);
            }
            ReloadResult::StateRestoreWarning(new_state, warning) => {
                println!("Reload {} had state restore warning: {}", i, warning);
                app_state = new_state;
            }
        }

        // Small delay to simulate realistic file save timing
        thread::sleep(Duration::from_millis(10));
    }

    let elapsed = start.elapsed();

    // Verify all reloads completed
    assert_eq!(
        app_state.model.count, reload_count,
        "All reloads should have been applied"
    );

    // Log timing information
    let avg_per_reload = elapsed.as_millis() / reload_count as u128;
    println!(
        "Completed {} reloads in {:?} (avg {}ms per reload)",
        reload_count, elapsed, avg_per_reload
    );

    // Each reload should be reasonably fast (not a strict requirement for this test)
    assert!(
        avg_per_reload < 100,
        "Average reload time should be reasonable (<100ms)"
    );
}

/// T090: Test debouncing behavior with very rapid changes
#[test]
fn test_debouncing_with_invalid_intermediate_states() {
    let valid_xml_v1 = r#"
<dampen version="1.1" encoding="utf-8">
    <column>
        <text value="Version 1" />
    </column>
</dampen>"#;

    let invalid_xml = r#"
<dampen version="1.1" encoding="utf-8">
    <column>
        <text value="Incomplete"#;

    let valid_xml_v2 = r#"
<dampen version="1.1" encoding="utf-8">
    <column>
        <text value="Version 2" />
        <text value="Complete" />
    </column>
</dampen>"#;

    // Create initial state
    let initial_doc = parser::parse(valid_xml_v1).expect("Initial parse should succeed");
    let model = CounterModel::default();
    let app_state = AppState::with_all(initial_doc, model, create_test_handlers());

    let mut context = HotReloadContext::<CounterModel>::new();

    // Simulate rapid editing: valid -> invalid -> valid
    // First reload: try invalid XML (should fail)
    let result1 = attempt_hot_reload(invalid_xml, &app_state, &mut context, create_test_handlers);

    assert!(
        matches!(result1, ReloadResult::ParseError(_)),
        "Invalid XML should fail"
    );

    // Second reload: try valid XML (should succeed)
    let result2 = attempt_hot_reload(valid_xml_v2, &app_state, &mut context, create_test_handlers);

    match result2 {
        ReloadResult::Success(new_state) => {
            // Should have successfully reloaded to the final valid state
            assert_eq!(
                new_state.document.root.children.len(),
                2,
                "Should have 2 text widgets"
            );
        }
        _ => {
            panic!("Final valid XML should succeed");
        }
    }
}
