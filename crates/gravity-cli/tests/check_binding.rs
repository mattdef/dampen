use gravity_cli::commands::check::model::{ModelField, ModelInfo};
use std::fs;
use tempfile::TempDir;

#[test]
fn test_load_model_info_from_json() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let model_path = temp_dir.path().join("model.json");

    let json_content = r#"[
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

    fs::write(&model_path, json_content).expect("Failed to write model file");

    let model = ModelInfo::load_from_json(&model_path).expect("Failed to load model");

    assert!(model.contains_field(&["count"]));
    assert!(model.contains_field(&["user"]));
    assert!(model.contains_field(&["user", "name"]));
    assert!(model.contains_field(&["user", "email"]));
    assert!(!model.contains_field(&["unknown"]));
}

#[test]
fn test_load_empty_model_info() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let model_path = temp_dir.path().join("model.json");

    let json_content = "[]";

    fs::write(&model_path, json_content).expect("Failed to write model file");

    let model = ModelInfo::load_from_json(&model_path).expect("Failed to load model");

    assert!(!model.contains_field(&["count"]));
}

#[test]
fn test_load_invalid_model_json() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let model_path = temp_dir.path().join("model.json");

    let json_content = "{ invalid json }";

    fs::write(&model_path, json_content).expect("Failed to write model file");

    let result = ModelInfo::load_from_json(&model_path);

    assert!(result.is_err());
}

#[test]
fn test_model_info_top_level_fields() {
    let mut model = ModelInfo::new();

    let field1 = ModelField {
        name: "count".to_string(),
        type_name: "i32".to_string(),
        is_nested: false,
        children: vec![],
    };

    let field2 = ModelField {
        name: "enabled".to_string(),
        type_name: "bool".to_string(),
        is_nested: false,
        children: vec![],
    };

    model.add_field(field1);
    model.add_field(field2);

    let fields = model.top_level_fields();
    assert_eq!(fields.len(), 2);
    assert!(fields.contains(&"count"));
    assert!(fields.contains(&"enabled"));
}

// T027: Tests for simple field binding validation
#[test]
fn test_simple_field_binding_validation() {
    let mut model = ModelInfo::new();

    model.add_field(ModelField {
        name: "count".to_string(),
        type_name: "i32".to_string(),
        is_nested: false,
        children: vec![],
    });

    model.add_field(ModelField {
        name: "enabled".to_string(),
        type_name: "bool".to_string(),
        is_nested: false,
        children: vec![],
    });

    // Valid simple fields
    assert!(model.contains_field(&["count"]));
    assert!(model.contains_field(&["enabled"]));

    // Invalid fields
    assert!(!model.contains_field(&["unknown"]));
    assert!(!model.contains_field(&["cnt"])); // typo
}

#[test]
fn test_validate_binding_expression() {
    let mut model = ModelInfo::new();

    model.add_field(ModelField {
        name: "count".to_string(),
        type_name: "i32".to_string(),
        is_nested: false,
        children: vec![],
    });

    // Test different binding expression formats
    assert!(model.contains_field(&["count"]));

    // Empty path should be invalid
    assert!(!model.contains_field(&[]));
}

// T028: Tests for nested field binding validation
#[test]
fn test_nested_field_binding_validation() {
    let mut model = ModelInfo::new();

    let user_field = ModelField {
        name: "user".to_string(),
        type_name: "User".to_string(),
        is_nested: true,
        children: vec![
            ModelField {
                name: "name".to_string(),
                type_name: "String".to_string(),
                is_nested: false,
                children: vec![],
            },
            ModelField {
                name: "email".to_string(),
                type_name: "String".to_string(),
                is_nested: false,
                children: vec![],
            },
        ],
    };

    model.add_field(user_field);

    // Valid nested fields
    assert!(model.contains_field(&["user"]));
    assert!(model.contains_field(&["user", "name"]));
    assert!(model.contains_field(&["user", "email"]));

    // Invalid nested fields
    assert!(!model.contains_field(&["user", "unknown"]));
    assert!(!model.contains_field(&["user", "nme"])); // typo
}

#[test]
fn test_deeply_nested_fields() {
    let mut model = ModelInfo::new();

    let profile_field = ModelField {
        name: "profile".to_string(),
        type_name: "Profile".to_string(),
        is_nested: true,
        children: vec![ModelField {
            name: "address".to_string(),
            type_name: "Address".to_string(),
            is_nested: true,
            children: vec![ModelField {
                name: "street".to_string(),
                type_name: "String".to_string(),
                is_nested: false,
                children: vec![],
            }],
        }],
    };

    model.add_field(profile_field);

    // Valid deeply nested fields
    assert!(model.contains_field(&["profile"]));
    assert!(model.contains_field(&["profile", "address"]));
    assert!(model.contains_field(&["profile", "address", "street"]));

    // Invalid deeply nested fields
    assert!(!model.contains_field(&["profile", "address", "city"]));
}

#[test]
fn test_non_nested_field_cannot_have_children() {
    let mut model = ModelInfo::new();

    model.add_field(ModelField {
        name: "count".to_string(),
        type_name: "i32".to_string(),
        is_nested: false,
        children: vec![],
    });

    // Can access the field itself
    assert!(model.contains_field(&["count"]));

    // Cannot access children of non-nested field
    assert!(!model.contains_field(&["count", "something"]));
}
