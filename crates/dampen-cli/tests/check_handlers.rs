use dampen_cli::commands::check::handlers::{HandlerDefinition, HandlerRegistry};
use std::fs;
use tempfile::TempDir;

#[test]
fn test_load_handler_registry_from_json() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let registry_path = temp_dir.path().join("handlers.json");

    let json_content = r#"[
  {
    "name": "increment",
    "param_type": null,
    "returns_command": false
  },
  {
    "name": "setValue",
    "param_type": "i32",
    "returns_command": true
  }
]"#;

    fs::write(&registry_path, json_content).expect("Failed to write registry file");

    let registry =
        HandlerRegistry::load_from_json(&registry_path).expect("Failed to load registry");

    assert!(registry.contains("increment"));
    assert!(registry.contains("setValue"));
    assert!(!registry.contains("unknown_handler"));
}

#[test]
fn test_load_empty_handler_registry() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let registry_path = temp_dir.path().join("handlers.json");

    let json_content = "[]";

    fs::write(&registry_path, json_content).expect("Failed to write registry file");

    let registry =
        HandlerRegistry::load_from_json(&registry_path).expect("Failed to load registry");

    assert!(!registry.contains("increment"));
}

#[test]
fn test_load_invalid_json() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let registry_path = temp_dir.path().join("handlers.json");

    let json_content = "{ invalid json }";

    fs::write(&registry_path, json_content).expect("Failed to write registry file");

    let result = HandlerRegistry::load_from_json(&registry_path);

    assert!(result.is_err());
}

#[test]
fn test_handler_registry_all_names() {
    let mut registry = HandlerRegistry::new();

    let handler1 = HandlerDefinition {
        name: "increment".to_string(),
        param_type: None,
        returns_command: false,
    };

    let handler2 = HandlerDefinition {
        name: "decrement".to_string(),
        param_type: None,
        returns_command: false,
    };

    registry.add_handler(handler1);
    registry.add_handler(handler2);

    let names = registry.all_names();
    assert_eq!(names.len(), 2);
    assert!(names.contains(&"increment".to_string()));
    assert!(names.contains(&"decrement".to_string()));
}

// T019: Tests for unknown handler detection
#[test]
fn test_detect_unknown_handler() {
    let mut registry = HandlerRegistry::new();

    let handler = HandlerDefinition {
        name: "increment".to_string(),
        param_type: None,
        returns_command: false,
    };

    registry.add_handler(handler);

    assert!(registry.contains("increment"));
    assert!(!registry.contains("incremnt")); // typo
    assert!(!registry.contains("unknown"));
}

#[test]
fn test_validate_handlers() {
    let mut registry = HandlerRegistry::new();

    registry.add_handler(HandlerDefinition {
        name: "increment".to_string(),
        param_type: None,
        returns_command: false,
    });

    registry.add_handler(HandlerDefinition {
        name: "decrement".to_string(),
        param_type: None,
        returns_command: false,
    });

    let valid_handlers = vec!["increment", "decrement"];
    let invalid_handlers = vec!["incremnt", "unknown"];

    for handler in valid_handlers {
        assert!(registry.contains(handler));
    }

    for handler in invalid_handlers {
        assert!(!registry.contains(handler));
    }
}

// T020: Tests for handler suggestions with Levenshtein distance
#[test]
fn test_handler_suggestion_with_levenshtein() {
    use dampen_cli::commands::check::suggestions;

    let mut registry = HandlerRegistry::new();

    registry.add_handler(HandlerDefinition {
        name: "increment".to_string(),
        param_type: None,
        returns_command: false,
    });

    registry.add_handler(HandlerDefinition {
        name: "decrement".to_string(),
        param_type: None,
        returns_command: false,
    });

    let names = registry.all_names();
    let names_refs: Vec<&str> = names.iter().map(|s| s.as_str()).collect();

    // Test suggestion for typo
    let suggestion = suggestions::suggest("incremnt", &names_refs, 3);
    assert!(suggestion.contains("increment"));

    // Test suggestion for another typo
    let suggestion = suggestions::suggest("decremnt", &names_refs, 3);
    assert!(suggestion.contains("decrement"));
}

#[test]
fn test_handler_no_suggestion_for_very_different_name() {
    use dampen_cli::commands::check::suggestions;

    let mut registry = HandlerRegistry::new();

    registry.add_handler(HandlerDefinition {
        name: "increment".to_string(),
        param_type: None,
        returns_command: false,
    });

    let names = registry.all_names();
    let names_refs: Vec<&str> = names.iter().map(|s| s.as_str()).collect();

    // Test no suggestion for completely different name
    let suggestion = suggestions::suggest("completely_different", &names_refs, 3);
    assert_eq!(suggestion, "");
}
