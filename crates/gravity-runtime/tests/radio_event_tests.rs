//! Integration tests for radio button selection change events

use gravity_core::binding::{BindingValue, UiBindable};
use gravity_core::{handler::HandlerRegistry, parse, EventBinding, EventKind, Span};
use gravity_runtime::Interpreter;
use serde::{Deserialize, Serialize};
use std::any::Any;

/// Test model for radio event handling
#[derive(Clone, Debug, Serialize, Deserialize)]
struct RadioModel {
    pub selected_option: Option<String>,
    pub event_count: i32,
    pub last_value: Option<String>,
}

impl UiBindable for RadioModel {
    fn get_field(&self, path: &[&str]) -> Option<BindingValue> {
        match path {
            ["selected_option"] => Some(BindingValue::String(
                self.selected_option.clone().unwrap_or_default(),
            )),
            ["event_count"] => Some(BindingValue::Integer(self.event_count as i64)),
            ["last_value"] => Some(BindingValue::String(
                self.last_value.clone().unwrap_or_default(),
            )),
            _ => None,
        }
    }

    fn available_fields() -> Vec<String> {
        vec![
            "selected_option".to_string(),
            "event_count".to_string(),
            "last_value".to_string(),
        ]
    }
}

#[test]
fn test_radio_event_dispatch() {
    // Create a model
    let mut model = RadioModel {
        selected_option: None,
        event_count: 0,
        last_value: None,
    };

    // Create handler registry
    let registry = HandlerRegistry::new();

    registry.register_with_value(
        "selectOption",
        |model: &mut dyn Any, value: Box<dyn Any>| {
            let value_str = value.downcast_ref::<String>().unwrap();
            let m = model.downcast_mut::<RadioModel>().unwrap();
            m.selected_option = Some(value_str.clone());
            m.event_count += 1;
            m.last_value = Some(value_str.clone());
        },
    );

    // Create interpreter
    let interpreter = Interpreter::new(registry);

    // Simulate a radio selection event
    let event = EventBinding {
        event: EventKind::Select,
        handler: "selectOption".to_string(),
        param: None,
        span: Span::new(0, 0, 1, 1),
    };

    // Dispatch the event with a value
    let value = Box::new("option_a".to_string());
    let result = interpreter.dispatch(&event, &mut model, Some(value));

    // Verify the event was handled
    assert!(result.is_ok());
    assert_eq!(model.selected_option, Some("option_a".to_string()));
    assert_eq!(model.event_count, 1);
    assert_eq!(model.last_value, Some("option_a".to_string()));
}

#[test]
fn test_radio_multiple_selections() {
    // Test that multiple radio selections update the model correctly
    let mut model = RadioModel {
        selected_option: None,
        event_count: 0,
        last_value: None,
    };

    let registry = HandlerRegistry::new();

    registry.register_with_value("setSize", |model: &mut dyn Any, value: Box<dyn Any>| {
        let value_str = value.downcast_ref::<String>().unwrap();
        let m = model.downcast_mut::<RadioModel>().unwrap();
        m.selected_option = Some(value_str.clone());
        m.event_count += 1;
        m.last_value = Some(value_str.clone());
    });

    let interpreter = Interpreter::new(registry);

    // First selection: "small"
    let event = EventBinding {
        event: EventKind::Select,
        handler: "setSize".to_string(),
        param: None,
        span: Span::new(0, 0, 1, 1),
    };

    let _ = interpreter.dispatch(&event, &mut model, Some(Box::new("small".to_string())));
    assert_eq!(model.selected_option, Some("small".to_string()));
    assert_eq!(model.event_count, 1);

    // Second selection: "large"
    let _ = interpreter.dispatch(&event, &mut model, Some(Box::new("large".to_string())));
    assert_eq!(model.selected_option, Some("large".to_string()));
    assert_eq!(model.event_count, 2);
    assert_eq!(model.last_value, Some("large".to_string()));
}

#[test]
fn test_radio_handler_not_found() {
    // Test that missing handlers return an error
    let mut model = RadioModel {
        selected_option: None,
        event_count: 0,
        last_value: None,
    };

    let registry = HandlerRegistry::new();
    // Don't register any handlers

    let interpreter = Interpreter::new(registry);

    let event = EventBinding {
        event: EventKind::Select,
        handler: "nonexistent".to_string(),
        param: None,
        span: Span::new(0, 0, 1, 1),
    };

    let result = interpreter.dispatch(&event, &mut model, Some(Box::new("value".to_string())));

    // Should return an error
    assert!(result.is_err());
}

#[test]
fn test_radio_event_from_xml() {
    // Test that radio events parsed from XML can be dispatched
    let xml = r#"<radio label="Option A" value="a" on_select="handleSelect" />"#;
    let document = parse(xml).unwrap();

    // Verify the event was parsed correctly
    assert_eq!(document.root.events.len(), 1);
    let event = &document.root.events[0];
    assert_eq!(event.event, EventKind::Select);
    assert_eq!(event.handler, "handleSelect");

    // Create a handler that captures the value
    let registry = HandlerRegistry::new();
    registry.register_with_value(
        "handleSelect",
        |model: &mut dyn Any, value: Box<dyn Any>| {
            let value_str = value.downcast_ref::<String>().unwrap();
            let m = model.downcast_mut::<RadioModel>().unwrap();
            m.last_value = Some(value_str.clone());
        },
    );

    let interpreter = Interpreter::new(registry);

    // Simulate the radio being clicked with value "a"
    let mut model = RadioModel {
        selected_option: None,
        event_count: 0,
        last_value: None,
    };

    let result = interpreter.dispatch(event, &mut model, Some(Box::new("a".to_string())));

    assert!(result.is_ok());
    assert_eq!(model.last_value, Some("a".to_string()));
}

#[test]
fn test_radio_simple_handler() {
    // Test radio with a simple handler (no value parameter)
    let mut model = RadioModel {
        selected_option: None,
        event_count: 0,
        last_value: None,
    };

    let registry = HandlerRegistry::new();

    registry.register_simple("increment", |model: &mut dyn Any| {
        let m = model.downcast_mut::<RadioModel>().unwrap();
        m.event_count += 1;
    });

    let interpreter = Interpreter::new(registry);

    let event = EventBinding {
        event: EventKind::Select,
        handler: "increment".to_string(),
        param: None,
        span: Span::new(0, 0, 1, 1),
    };

    let result = interpreter.dispatch(&event, &mut model, None);

    assert!(result.is_ok());
    assert_eq!(model.event_count, 1);
}

#[test]
fn test_radio_group_event_flow() {
    // Test complete event flow for a radio group
    let xml = r#"
        <column>
            <radio label="Small" value="small" selected="{size}" on_select="setSize" />
            <radio label="Medium" value="medium" selected="{size}" on_select="setSize" />
            <radio label="Large" value="large" selected="{size}" on_select="setSize" />
        </column>
    "#;

    let document = parse(xml).unwrap();

    // Verify all radios have the same handler
    assert_eq!(document.root.children.len(), 3);
    for child in &document.root.children {
        assert_eq!(child.events.len(), 1);
        assert_eq!(child.events[0].handler, "setSize");
        assert_eq!(child.events[0].event, EventKind::Select);
    }

    // Create handler that updates the selection
    let mut model = RadioModel {
        selected_option: None,
        event_count: 0,
        last_value: None,
    };

    let registry = HandlerRegistry::new();

    registry.register_with_value("setSize", |model: &mut dyn Any, value: Box<dyn Any>| {
        let value_str = value.downcast_ref::<String>().unwrap();
        let m = model.downcast_mut::<RadioModel>().unwrap();
        m.selected_option = Some(value_str.clone());
        m.event_count += 1;
    });

    let interpreter = Interpreter::new(registry);

    // Simulate clicking each radio button
    let radios = &document.root.children;
    let values = vec!["small", "medium", "large"];

    for (radio, expected_value) in radios.iter().zip(values.iter()) {
        let event = &radio.events[0];
        let _ = interpreter.dispatch(
            event,
            &mut model,
            Some(Box::new(expected_value.to_string())),
        );
    }

    // After clicking all three, the last one should be selected
    assert_eq!(model.selected_option, Some("large".to_string()));
    assert_eq!(model.event_count, 3);
}
