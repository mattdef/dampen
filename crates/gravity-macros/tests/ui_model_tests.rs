//! Tests for the #[derive(UiModel)] macro

use gravity_core::binding::{BindingValue, UiBindable};
use gravity_macros::UiModel;

/// Test model with primitive types
#[derive(UiModel, Debug, Clone, Default)]
struct SimpleModel {
    name: String,
    count: i32,
    ratio: f64,
    active: bool,
}

/// Test model with Option and Vec
#[derive(UiModel, Debug, Clone, Default)]
struct ComplexModel {
    items: Vec<String>,
    maybe_value: Option<i32>,
}

/// Test model with #[ui_skip]
#[derive(UiModel, Debug, Clone, Default)]
struct SkippedModel {
    visible: String,
    #[ui_skip]
    internal_cache: String,
}

#[test]
fn test_simple_field_accessors() {
    let model = SimpleModel {
        name: "Test".to_string(),
        count: 42,
        ratio: 3.14,
        active: true,
    };

    assert_eq!(
        model.get_field(&["name"]),
        Some(BindingValue::String("Test".to_string()))
    );
    assert_eq!(model.get_field(&["count"]), Some(BindingValue::Integer(42)));
    assert_eq!(model.get_field(&["ratio"]), Some(BindingValue::Float(3.14)));
    assert_eq!(model.get_field(&["active"]), Some(BindingValue::Bool(true)));
}

#[test]
fn test_primitive_field_types() {
    // Check that all fields are accessible
    let fields = SimpleModel::available_fields();
    assert!(fields.contains(&"name".to_string()));
    assert!(fields.contains(&"count".to_string()));
    assert!(fields.contains(&"ratio".to_string()));
    assert!(fields.contains(&"active".to_string()));
    assert_eq!(fields.len(), 4);
}

#[test]
fn test_option_and_vec_support() {
    let model = ComplexModel {
        items: vec!["item1".to_string()],
        maybe_value: Some(42),
    };

    // Vec should convert to List
    let items_value = model.get_field(&["items"]).unwrap();
    match items_value {
        BindingValue::List(list) => assert_eq!(list.len(), 1),
        _ => panic!("Expected List"),
    }

    // Some should convert to value
    let maybe_value = model.get_field(&["maybe_value"]).unwrap();
    assert_eq!(maybe_value, BindingValue::Integer(42));
}

// Note: Nested struct access (e.g., {nested.inner}) will be handled by the expression evaluator
// in a future phase. For now, we focus on simple field bindings.

#[test]
fn test_ui_skip_attribute() {
    let model = SkippedModel {
        visible: "visible".to_string(),
        internal_cache: "hidden".to_string(),
    };

    // Visible field should be accessible
    assert_eq!(
        model.get_field(&["visible"]),
        Some(BindingValue::String("visible".to_string()))
    );

    // Skipped field should not be accessible
    assert_eq!(model.get_field(&["internal_cache"]), None);

    // available_fields should not include skipped field
    let fields = SkippedModel::available_fields();
    assert!(fields.contains(&"visible".to_string()));
    assert!(!fields.contains(&"internal_cache".to_string()));
    assert_eq!(fields.len(), 1);
}

#[test]
fn test_nonexistent_field() {
    let model = SimpleModel::default();

    // Non-existent field returns None
    assert_eq!(model.get_field(&["nonexistent"]), None);
    assert_eq!(model.get_field(&["name", "too"]), None);
}

#[test]
fn test_default_implementations() {
    // Test that models can derive Default
    #[derive(UiModel, Debug, Clone, Default)]
    struct DefaultModel {
        value: i32,
    }

    let model = DefaultModel::default();
    assert_eq!(model.value, 0);
    assert_eq!(model.get_field(&["value"]), Some(BindingValue::Integer(0)));
}

/// Test model with #[ui_bind] attribute
#[derive(UiModel, Debug, Clone)]
struct BindModel {
    #[ui_bind]
    #[ui_skip]
    special_field: String,
    normal_field: String,
}

#[test]
fn test_ui_bind_attribute() {
    let model = BindModel {
        special_field: "special".to_string(),
        normal_field: "normal".to_string(),
    };

    // Both fields should be accessible
    assert_eq!(
        model.get_field(&["special_field"]),
        Some(BindingValue::String("special".to_string()))
    );
    assert_eq!(
        model.get_field(&["normal_field"]),
        Some(BindingValue::String("normal".to_string()))
    );

    // Both should be in available_fields
    let fields = BindModel::available_fields();
    assert!(fields.contains(&"special_field".to_string()));
    assert!(fields.contains(&"normal_field".to_string()));
}

#[test]
fn test_binding_value_conversions() {
    // Test that BindingValue conversions work correctly
    let string_val = BindingValue::from_value(&"hello".to_string());
    assert_eq!(string_val, BindingValue::String("hello".to_string()));

    let i32_val = BindingValue::from_value(&42i32);
    assert_eq!(i32_val, BindingValue::Integer(42));

    let bool_val = BindingValue::from_value(&true);
    assert_eq!(bool_val, BindingValue::Bool(true));

    let vec_val = BindingValue::from_value(&vec![1i32, 2i32]);
    assert_eq!(
        vec_val,
        BindingValue::List(vec![BindingValue::Integer(1), BindingValue::Integer(2),])
    );

    let opt_some = BindingValue::from_value(&Some(10i32));
    assert_eq!(opt_some, BindingValue::Integer(10));

    let opt_none: Option<i32> = None;
    let opt_none_val = BindingValue::from_value(&opt_none);
    assert_eq!(opt_none_val, BindingValue::None);
}

// Helper for tests
fn default_complex_model() -> ComplexModel {
    ComplexModel {
        items: Vec::new(),
        maybe_value: None,
    }
}

#[test]
fn test_to_display_string() {
    assert_eq!(
        BindingValue::String("hello".to_string()).to_display_string(),
        "hello"
    );
    assert_eq!(BindingValue::Integer(42).to_display_string(), "42");
    assert_eq!(BindingValue::Float(3.14).to_display_string(), "3.14");
    assert_eq!(BindingValue::Bool(true).to_display_string(), "true");
    assert_eq!(BindingValue::List(vec![]).to_display_string(), "[0 items]");
    assert_eq!(BindingValue::None.to_display_string(), "");
}

#[test]
fn test_to_bool() {
    assert_eq!(BindingValue::String("hello".to_string()).to_bool(), true);
    assert_eq!(BindingValue::String("".to_string()).to_bool(), false);
    assert_eq!(BindingValue::Integer(42).to_bool(), true);
    assert_eq!(BindingValue::Integer(0).to_bool(), false);
    assert_eq!(BindingValue::Float(1.0).to_bool(), true);
    assert_eq!(BindingValue::Float(0.0).to_bool(), false);
    assert_eq!(BindingValue::Bool(true).to_bool(), true);
    assert_eq!(BindingValue::Bool(false).to_bool(), false);
    assert_eq!(
        BindingValue::List(vec![BindingValue::Integer(1)]).to_bool(),
        true
    );
    assert_eq!(BindingValue::List(vec![]).to_bool(), false);
    assert_eq!(BindingValue::None.to_bool(), false);
}
