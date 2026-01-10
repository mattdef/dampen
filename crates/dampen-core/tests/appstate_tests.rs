//! Contract tests for the AppState struct.
//!
//! These tests verify that AppState works correctly with different
//! generic parameter configurations.

use dampen_core::{AppState, HandlerRegistry, UiBindable};
use dampen_macros::UiModel;
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
        <dampen>
            <column>
                <text value="Hello" />
            </column>
        </dampen>
    "#;

    let document = dampen_core::parse(xml).expect("Failed to parse XML");
    let state = AppState::<()>::new(document.clone());

    assert_eq!(state.document.version.major, 1);
    assert_eq!(TypeId::of::<()>(), TypeId::of::<()>());
}

#[test]
fn test_appstate_with_model() {
    let xml = r#"
        <dampen>
            <column>
                <text value="Hello" />
            </column>
        </dampen>
    "#;

    let document = dampen_core::parse(xml).expect("Failed to parse XML");
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
        <dampen>
            <column>
                <text value="Hello" />
            </column>
        </dampen>
    "#;

    let document = dampen_core::parse(xml).expect("Failed to parse XML");

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
        <dampen>
            <column>
                <text value="Clone test" />
            </column>
        </dampen>
    "#;

    let document = dampen_core::parse(xml).expect("Failed to parse XML");

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

    if let Some(dampen_core::HandlerEntry::Simple(handler)) = registry.get("test") {
        let mut any_model: Box<dyn std::any::Any> = Box::new(());
        handler(&mut *any_model);
        assert!(*called.lock().unwrap());
    } else {
        panic!("Handler not found");
    }
}

// ===== Hot-Reload Tests =====

#[test]
fn test_hot_reload_preserves_model() {
    // Create initial state with a model
    let xml_v1 = r#"
        <dampen>
            <column>
                <text value="Old UI" />
            </column>
        </dampen>
    "#;

    let document_v1 = dampen_core::parse(xml_v1).expect("Failed to parse XML v1");
    let model = TestModel {
        count: 42,
        name: "Preserved".to_string(),
    };
    let mut state = AppState::with_model(document_v1, model);

    // Verify initial state
    assert_eq!(state.model.count, 42);
    assert_eq!(state.model.name, "Preserved");

    // Parse new UI definition
    let xml_v2 = r#"
        <dampen>
            <column>
                <text value="New UI" />
                <button label="Click me" />
            </column>
        </dampen>
    "#;

    let document_v2 = dampen_core::parse(xml_v2).expect("Failed to parse XML v2");

    // Hot-reload with new document
    state.hot_reload(document_v2.clone());

    // Model should be preserved
    assert_eq!(state.model.count, 42);
    assert_eq!(state.model.name, "Preserved");

    // Document should be updated
    assert_eq!(
        state.document.root.children.len(),
        document_v2.root.children.len()
    );
}

#[test]
fn test_hot_reload_updates_document() {
    let xml_v1 = r#"
        <dampen>
            <column>
                <text value="Version 1" />
            </column>
        </dampen>
    "#;

    let document_v1 = dampen_core::parse(xml_v1).expect("Failed to parse XML v1");
    let mut state = AppState::<TestModel>::new(document_v1);

    // Count children in original document
    let original_widget_count = state.document.root.children.len();

    // Parse new UI with more widgets
    let xml_v2 = r#"
        <dampen>
            <column>
                <text value="Version 2" />
                <button label="Button 1" />
                <button label="Button 2" />
            </column>
        </dampen>
    "#;

    let document_v2 = dampen_core::parse(xml_v2).expect("Failed to parse XML v2");
    let new_widget_count = document_v2.root.children.len();

    // Hot-reload
    state.hot_reload(document_v2);

    // Document should be updated
    assert_eq!(state.document.root.children.len(), new_widget_count);
    assert_ne!(original_widget_count, new_widget_count);
}

#[test]
fn test_hot_reload_preserves_handlers() {
    let xml = r#"
        <dampen>
            <column>
                <text value="Test" />
            </column>
        </dampen>
    "#;

    let document_v1 = dampen_core::parse(xml).expect("Failed to parse XML");

    // Create handler registry
    let called = std::sync::Arc::new(std::sync::Mutex::new(false));
    let called_clone = called.clone();

    let mut registry = HandlerRegistry::new();
    registry.register_simple("test_handler", move |_model: &mut dyn std::any::Any| {
        *called_clone.lock().unwrap() = true;
    });

    let mut state = AppState::<TestModel>::with_handlers(document_v1, registry);

    // Verify handler works before hot-reload
    if let Some(dampen_core::HandlerEntry::Simple(handler)) =
        state.handler_registry.get("test_handler")
    {
        let mut any_model: Box<dyn std::any::Any> = Box::new(());
        handler(&mut *any_model);
        assert!(*called.lock().unwrap());
    } else {
        panic!("Handler not found before hot-reload");
    }

    // Reset the flag
    *called.lock().unwrap() = false;

    // Hot-reload with new document
    let xml_v2 = r#"
        <dampen>
            <column>
                <text value="Updated" />
            </column>
        </dampen>
    "#;

    let document_v2 = dampen_core::parse(xml_v2).expect("Failed to parse XML v2");
    state.hot_reload(document_v2);

    // Verify handler still works after hot-reload
    if let Some(dampen_core::HandlerEntry::Simple(handler)) =
        state.handler_registry.get("test_handler")
    {
        let mut any_model: Box<dyn std::any::Any> = Box::new(());
        handler(&mut *any_model);
        assert!(*called.lock().unwrap());
    } else {
        panic!("Handler not found after hot-reload");
    }
}

#[test]
fn test_hot_reload_multiple_times() {
    let xml_v1 = r#"<dampen><column><text value="V1" /></column></dampen>"#;
    let xml_v2 = r#"<dampen><column><text value="V2" /></column></dampen>"#;
    let xml_v3 = r#"<dampen><column><text value="V3" /></column></dampen>"#;

    let document_v1 = dampen_core::parse(xml_v1).unwrap();
    let model = TestModel {
        count: 100,
        name: "Stable".to_string(),
    };
    let mut state = AppState::with_model(document_v1, model);

    // First reload
    let document_v2 = dampen_core::parse(xml_v2).unwrap();
    state.hot_reload(document_v2);
    assert_eq!(state.model.count, 100);
    assert_eq!(state.model.name, "Stable");

    // Second reload
    let document_v3 = dampen_core::parse(xml_v3).unwrap();
    state.hot_reload(document_v3);
    assert_eq!(state.model.count, 100);
    assert_eq!(state.model.name, "Stable");

    // Model remains unchanged through multiple reloads
}
