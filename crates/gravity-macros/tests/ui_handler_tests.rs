//! Tests for the #[ui_handler] macro

use gravity_core::HandlerRegistry;
use gravity_macros::ui_handler;
use std::any::Any;

/// Test model for handler tests
#[derive(Default, Debug, Clone)]
struct TestModel {
    count: i32,
    name: String,
}

/// Test simple handler
#[ui_handler]
fn increment(model: &mut TestModel) {
    model.count += 1;
}

/// Test handler with value parameter
#[ui_handler]
fn set_name(model: &mut TestModel, name: String) {
    model.name = name;
}

/// Test handler with command (commented out until Command type is available)
// #[ui_handler]
// fn fetch_data(model: &mut TestModel) -> Command<Message> {
//     Command::none()
// }

#[test]
fn test_simple_handler_registration() {
    let registry = HandlerRegistry::new();

    // Register the increment handler
    // Note: In a real scenario, the macro would generate registration code
    // For now, we manually register to test the concept
    registry.register_simple("increment", |model: &mut dyn Any| {
        let model = model.downcast_mut::<TestModel>().unwrap();
        increment(model);
    });

    // Verify handler exists
    assert!(registry.contains("increment"));

    // Test execution
    let mut model = TestModel::default();
    if let Some(handler) = registry.get("increment") {
        if let gravity_core::HandlerEntry::Simple(h) = handler {
            h(&mut model);
            assert_eq!(model.count, 1);
        }
    }
}

#[test]
fn test_handler_with_value_registration() {
    let registry = HandlerRegistry::new();

    // Register the set_name handler
    registry.register_with_value("set_name", |model: &mut dyn Any, value: Box<dyn Any>| {
        let model = model.downcast_mut::<TestModel>().unwrap();
        let name = *value.downcast::<String>().unwrap();
        set_name(model, name);
    });

    // Verify handler exists
    assert!(registry.contains("set_name"));

    // Test execution
    let mut model = TestModel::default();
    if let Some(handler) = registry.get("set_name") {
        if let gravity_core::HandlerEntry::WithValue(h) = handler {
            h(&mut model, Box::new("TestName".to_string()));
            assert_eq!(model.name, "TestName");
        }
    }
}

#[test]
fn test_handler_lookup() {
    let registry = HandlerRegistry::new();

    registry.register_simple("test", |model: &mut dyn Any| {
        let _ = model.downcast_mut::<TestModel>().unwrap();
    });

    assert!(registry.get("test").is_some());
    assert!(registry.get("nonexistent").is_none());
}

#[test]
fn test_multiple_handlers() {
    let registry = HandlerRegistry::new();

    registry.register_simple("handler1", |model: &mut dyn Any| {
        let model = model.downcast_mut::<TestModel>().unwrap();
        model.count += 1;
    });

    registry.register_simple("handler2", |model: &mut dyn Any| {
        let model = model.downcast_mut::<TestModel>().unwrap();
        model.count += 2;
    });

    let mut model = TestModel::default();

    if let Some(gravity_core::HandlerEntry::Simple(h)) = registry.get("handler1") {
        h(&mut model);
    }

    if let Some(gravity_core::HandlerEntry::Simple(h)) = registry.get("handler2") {
        h(&mut model);
    }

    assert_eq!(model.count, 3);
}
