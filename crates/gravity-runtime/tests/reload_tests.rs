//! Hot-reload integration tests

use gravity_core::{evaluate_binding_expr, parse, tokenize_binding_expr, BindingValue, UiBindable};
use gravity_runtime::{overlay::ErrorOverlay, state::RuntimeState};
use serde::{Deserialize, Serialize};
use std::fs;
use std::time::Duration;

/// Test model for hot-reload
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct TestModel {
    count: i32,
    name: String,
}

impl UiBindable for TestModel {
    fn get_field(&self, path: &[&str]) -> Option<BindingValue> {
        match path.first()? {
            &"count" => Some(BindingValue::Integer(self.count as i64)),
            &"name" => Some(BindingValue::String(self.name.clone())),
            _ => None,
        }
    }

    fn available_fields() -> Vec<String> {
        vec!["count".to_string(), "name".to_string()]
    }
}

#[test]
fn test_file_change_detection() {
    use notify::{RecursiveMode, Watcher};
    use std::sync::mpsc::channel;
    use std::thread;

    // Create a temporary directory and file
    let temp_dir = tempfile::tempdir().unwrap();
    let test_file = temp_dir.path().join("test.gravity");

    // Write initial content
    fs::write(&test_file, r#"<column><text value="Initial" /></column>"#).unwrap();

    // Set up file watcher
    let (tx, rx) = channel();
    let mut watcher = notify::recommended_watcher(move |res| {
        let _ = tx.send(res);
    })
    .unwrap();

    watcher
        .watch(&test_file, RecursiveMode::NonRecursive)
        .unwrap();

    // Modify the file
    thread::sleep(Duration::from_millis(150)); // Debounce
    fs::write(&test_file, r#"<column><text value="Updated" /></column>"#).unwrap();

    // Wait for event
    let event = rx.recv_timeout(Duration::from_secs(2)).unwrap();

    assert!(event.is_ok());

    // Cleanup
    drop(watcher);
    drop(temp_dir);
}

#[test]
fn test_state_serialization() {
    let model = TestModel {
        count: 42,
        name: "Test".to_string(),
    };

    let state = RuntimeState::new(model.clone());

    // Serialize
    let json = serde_json::to_string(&state).unwrap();
    assert!(json.contains("count"));
    assert!(json.contains("42"));

    // Deserialize
    let restored: RuntimeState<TestModel> = serde_json::from_str(&json).unwrap();
    assert_eq!(restored.model.count, 42);
    assert_eq!(restored.model.name, "Test");
}

#[test]
fn test_state_restoration_partial() {
    // Test with missing fields (simulating schema evolution)
    #[derive(Debug, Clone, Serialize, Deserialize, Default)]
    struct OldModel {
        count: i32,
    }

    #[derive(Debug, Clone, Serialize, Deserialize, Default)]
    struct NewModel {
        count: i32,
        name: String, // New field
    }

    let old_state = RuntimeState {
        model: OldModel { count: 10 },
        version: 1,
        saved_at: 0,
    };

    // Serialize old model
    let json = serde_json::to_string(&old_state).unwrap();

    // Try to deserialize into new model (with lenient parsing)
    #[derive(Deserialize)]
    struct TempModel {
        count: i32,
        #[serde(default)]
        name: String,
    }

    let temp: RuntimeState<TempModel> = serde_json::from_str(&json).unwrap();

    // Should have count from old state, empty name for new field
    assert_eq!(temp.model.count, 10);
    assert_eq!(temp.model.name, "");
}

#[test]
fn test_error_overlay_creation() {
    use gravity_core::ParseError;

    // Create a parse error
    let error = ParseError {
        kind: gravity_core::ParseErrorKind::UnknownWidget,
        message: "Unknown widget: <buton>".to_string(),
        span: gravity_core::Span::new(10, 20, 5, 12),
        suggestion: Some("Did you mean: <button>?".to_string()),
    };

    // Create overlay
    let overlay = ErrorOverlay::from_parse_error(&error);

    // Verify it contains the error information
    assert!(overlay.title.contains("Parse Error"));
    assert!(overlay.message.contains("Unknown widget"));
    assert!(overlay.suggestion.is_some());
}

#[test]
fn test_parse_error_with_span() {
    let xml = r#"<column>
    <buton label="Click" />
</column>"#;

    let result = parse(xml);

    assert!(result.is_err());
    let err = result.unwrap_err();

    // Should have span information
    assert!(err.span.line > 0);
    assert!(err.span.column > 0);
    assert!(err.message.contains("buton"));
}

#[test]
fn test_binding_error_detection() {
    use gravity_core::evaluate_binding_expr;
    use gravity_core::expr::tokenize_binding_expr;

    // Parse a binding expression with unknown field
    let expr = tokenize_binding_expr("unknown_field", 0, 1, 1).unwrap();

    // Create a model without that field
    #[derive(Debug, Clone, Serialize, Deserialize, Default)]
    struct SimpleModel {
        count: i32,
    }

    impl UiBindable for SimpleModel {
        fn get_field(&self, path: &[&str]) -> Option<BindingValue> {
            match path.first()? {
                &"count" => Some(BindingValue::Integer(self.count as i64)),
                _ => None,
            }
        }

        fn available_fields() -> Vec<String> {
            vec!["count".to_string()]
        }
    }

    let model = SimpleModel::default();
    let result = evaluate_binding_expr(&expr, &model);

    // Should fail gracefully
    assert!(result.is_err());
}

#[test]
fn test_reload_latency_measurement() {
    use std::time::Instant;

    // Simulate a reload cycle
    let start = Instant::now();

    // Parse XML
    let xml = r#"<column><text value="Test" /></column>"#;
    let _doc = parse(xml).unwrap();

    // Evaluate bindings (if any)
    // In a real scenario, this would involve state serialization/deserialization

    let duration = start.elapsed();

    // Should be fast (< 500ms as per requirements)
    assert!(duration.as_millis() < 500);
}

#[test]
fn test_state_preservation_across_reload() {
    // Simulate state before reload
    let original_model = TestModel {
        count: 100,
        name: "Preserved".to_string(),
    };

    // Serialize
    let state = RuntimeState::new(original_model.clone());
    let json = serde_json::to_string(&state).unwrap();

    // Simulate reload - deserialize
    let restored_state: RuntimeState<TestModel> = serde_json::from_str(&json).unwrap();

    // Verify state is preserved
    assert_eq!(restored_state.model.count, original_model.count);
    assert_eq!(restored_state.model.name, original_model.name);
}

#[test]
fn test_graceful_handling_of_removed_fields() {
    #[derive(Debug, Clone, Serialize, Deserialize, Default)]
    struct OldModel {
        count: i32,
        removed: String, // This field will be removed
    }

    #[derive(Debug, Clone, Serialize, Deserialize, Default)]
    struct NewModel {
        count: i32,
    }

    let old_state = RuntimeState {
        model: OldModel {
            count: 42,
            removed: "old data".to_string(),
        },
        version: 1,
        saved_at: 0,
    };

    let json = serde_json::to_string(&old_state).unwrap();

    // Deserialize into model with fewer fields
    let new_state: RuntimeState<NewModel> = serde_json::from_str(&json).unwrap();

    // Should keep the fields that exist, ignore the rest
    assert_eq!(new_state.model.count, 42);
}

#[test]
fn test_error_overlay_dismissal() {
    use gravity_core::ParseError;

    let error = ParseError {
        kind: gravity_core::ParseErrorKind::InvalidExpression,
        message: "Syntax error in binding".to_string(),
        span: gravity_core::Span::new(0, 0, 1, 1),
        suggestion: None,
    };

    let mut overlay = ErrorOverlay::from_parse_error(&error);

    // Initially visible
    assert!(overlay.visible);

    // Dismiss
    overlay.dismiss();

    // Should be hidden
    assert!(!overlay.visible);
}
