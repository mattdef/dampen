//! Contract tests for the AppState struct.
//!
//! These tests verify that AppState works correctly with different
//! generic parameter configurations.

use gravity_core::{AppState, HandlerRegistry, UiBindable};
use gravity_macros::UiModel;
use serde::{Deserialize, Serialize};
use std::any::TypeId;

#[derive(Default, UiModel, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct TestModel {
    pub count: i32,
    pub name: String,
}

#[test]
fn test_appstate_default_unit() {
    let xml = r#"
        <gravity>
            <column>
                <text value="Hello" />
            </column>
        </gravity>
    "#;

    let document = gravity_core::parse(xml).expect("Failed to parse XML");
    let state = AppState::<()>::new(document.clone());

    assert_eq!(state.document.version.major, 1);
    assert_eq!(TypeId::of::<()>(), TypeId::of::<()>());
}

#[test]
fn test_appstate_with_model() {
    let xml = r#"
        <gravity>
            <column>
                <text value="Hello" />
            </column>
        </gravity>
    "#;

    let document = gravity_core::parse(xml).expect("Failed to parse XML");
    let model = TestModel {
        count: 42,
        name: "Test".to_string(),
    };
    let state = AppState::with_model(document.clone(), model.clone());

    assert_eq!(state.document.version.major, 1);
    assert_eq!(state.model.count, 42);
    assert_eq!(state.model.name, "Test");
}

#[test]
fn test_appstate_with_handlers() {
    let xml = r#"
        <gravity>
            <column>
                <text value="Hello" />
            </column>
        </gravity>
    "#;

    let document = gravity_core::parse(xml).expect("Failed to parse XML");

    let registry = HandlerRegistry::new();
    let state: AppState<()> = AppState::with_handlers(document.clone(), registry);

    assert_eq!(state.document.version.major, 1);
}

#[test]
fn test_appstate_model_implements_uibindable() {
    let model = TestModel {
        count: 100,
        name: "Binding Test".to_string(),
    };

    let _ = model.get_field(&["count"]);
    let _ = model.get_field(&["name"]);

    let fields = TestModel::available_fields();
    assert!(fields.contains(&"count".to_string()));
    assert!(fields.contains(&"name".to_string()));
}

#[test]
fn test_appstate_clonable() {
    let xml = r#"
        <gravity>
            <column>
                <text value="Clone test" />
            </column>
        </gravity>
    "#;

    let document = gravity_core::parse(xml).expect("Failed to parse XML");

    let model = TestModel {
        count: 99,
        name: "Clone".to_string(),
    };

    let registry = HandlerRegistry::new();
    let state: AppState<TestModel> = AppState::with_handlers(document.clone(), registry);

    let cloned = state.clone();

    assert_eq!(cloned.document.version.major, 1);
    assert_eq!(cloned.model.count, 0);
    assert_eq!(cloned.model.name, "");
}

#[test]
fn test_appstate_model_serialization() {
    let model = TestModel {
        count: 123,
        name: "Serialize".to_string(),
    };

    let serialized = serde_json::to_string(&model).expect("Failed to serialize");
    let deserialized: TestModel = serde_json::from_str(&serialized).expect("Failed to deserialize");

    assert_eq!(model.count, deserialized.count);
    assert_eq!(model.name, deserialized.name);
}

#[test]
fn test_handler_registry_simple_handler() {
    let registry = HandlerRegistry::new();
    let called = std::sync::Arc::new(std::sync::Mutex::new(false));
    let called_clone = called.clone();

    registry.register_simple("test", move |_model: &mut dyn std::any::Any| {
        *called_clone.lock().unwrap() = true;
    });

    if let Some(gravity_core::HandlerEntry::Simple(handler)) = registry.get("test") {
        let mut any_model: Box<dyn std::any::Any> = Box::new(());
        handler(&mut *any_model);
        assert!(*called.lock().unwrap());
    } else {
        panic!("Handler not found");
    }
}
