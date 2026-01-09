//! Tests for radio widget custom value type support

use dampen_core::binding::{BindingValue, UiBindable};
use dampen_core::parse;
use serde::{Deserialize, Serialize};

/// Example enum type for radio selection
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
enum Priority {
    Low,
    Medium,
    High,
}

impl ToString for Priority {
    fn to_string(&self) -> String {
        match self {
            Priority::Low => "low".to_string(),
            Priority::Medium => "medium".to_string(),
            Priority::High => "high".to_string(),
        }
    }
}

/// Test model with enum field
#[derive(Clone, Debug, Serialize, Deserialize)]
struct TaskModel {
    pub priority: Priority,
    pub status: String,
}

impl UiBindable for TaskModel {
    fn get_field(&self, path: &[&str]) -> Option<BindingValue> {
        match path {
            ["priority"] => Some(BindingValue::String(self.priority.to_string())),
            ["status"] => Some(BindingValue::String(self.status.clone())),
            _ => None,
        }
    }

    fn available_fields() -> Vec<String> {
        vec!["priority".to_string(), "status".to_string()]
    }
}

#[test]
fn test_radio_with_enum_value_binding() {
    // Test that radio works with enum types via string coercion
    let xml = r#"
        <column>
            <radio label="Low Priority" value="low" selected="{priority}" on_select="setPriority" />
            <radio label="Medium Priority" value="medium" selected="{priority}" on_select="setPriority" />
            <radio label="High Priority" value="high" selected="{priority}" on_select="setPriority" />
        </column>
    "#;

    let document = parse(xml).unwrap();

    // Verify parsing succeeded
    assert_eq!(document.root.children.len(), 3);

    // Model with enum value
    let model = TaskModel {
        priority: Priority::Medium,
        status: "active".to_string(),
    };

    // Verify the enum can be converted to BindingValue
    let binding = model.get_field(&["priority"]);
    assert!(binding.is_some());

    if let Some(BindingValue::String(value)) = binding {
        assert_eq!(value, "medium");
    } else {
        panic!("Expected String binding value");
    }
}

#[test]
fn test_radio_with_string_value_types() {
    // Test that radio works with plain String fields
    let xml = r#"
        <column>
            <radio label="Active" value="active" selected="{status}" on_select="setStatus" />
            <radio label="Paused" value="paused" selected="{status}" on_select="setStatus" />
            <radio label="Complete" value="complete" selected="{status}" on_select="setStatus" />
        </column>
    "#;

    let document = parse(xml).unwrap();

    assert_eq!(document.root.children.len(), 3);

    let model = TaskModel {
        priority: Priority::Low,
        status: "active".to_string(),
    };

    // Verify string binding works
    let binding = model.get_field(&["status"]);
    assert!(binding.is_some());

    if let Some(BindingValue::String(value)) = binding {
        assert_eq!(value, "active");
    }
}

#[test]
fn test_radio_value_attribute_parsing() {
    // Test that value attributes are correctly parsed
    let xml = r#"<radio label="Test" value="test_value" />"#;
    let document = parse(xml).unwrap();

    // Verify value attribute exists
    let value_attr = document.root.attributes.get("value");
    assert!(value_attr.is_some());

    // Verify it's a static attribute
    match value_attr {
        Some(dampen_core::AttributeValue::Static(s)) => {
            assert_eq!(s, "test_value");
        }
        _ => panic!("Expected static value attribute"),
    }
}

#[test]
fn test_radio_selected_attribute_binding() {
    // Test that selected attribute can be a binding
    let xml = r#"<radio label="Test" value="test" selected="{field}" />"#;
    let document = parse(xml).unwrap();

    let selected_attr = document.root.attributes.get("selected");
    assert!(selected_attr.is_some());

    // Verify it's a binding
    match selected_attr {
        Some(dampen_core::AttributeValue::Binding(_)) => {
            // Correct - binding expression
        }
        _ => panic!("Expected binding for selected attribute"),
    }
}

#[test]
fn test_radio_value_coercion() {
    // Test that different enum values map correctly
    let model1 = TaskModel {
        priority: Priority::Low,
        status: String::new(),
    };
    let model2 = TaskModel {
        priority: Priority::Medium,
        status: String::new(),
    };
    let model3 = TaskModel {
        priority: Priority::High,
        status: String::new(),
    };

    // Verify each enum value converts to correct string
    assert_eq!(
        model1.get_field(&["priority"]),
        Some(BindingValue::String("low".to_string()))
    );
    assert_eq!(
        model2.get_field(&["priority"]),
        Some(BindingValue::String("medium".to_string()))
    );
    assert_eq!(
        model3.get_field(&["priority"]),
        Some(BindingValue::String("high".to_string()))
    );
}

/// Example numeric enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
enum Level {
    One,
    Two,
    Three,
}

impl ToString for Level {
    fn to_string(&self) -> String {
        match self {
            Level::One => "1".to_string(),
            Level::Two => "2".to_string(),
            Level::Three => "3".to_string(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct LevelModel {
    pub level: Level,
}

impl UiBindable for LevelModel {
    fn get_field(&self, path: &[&str]) -> Option<BindingValue> {
        match path {
            ["level"] => Some(BindingValue::String(self.level.to_string())),
            _ => None,
        }
    }

    fn available_fields() -> Vec<String> {
        vec!["level".to_string()]
    }
}

#[test]
fn test_radio_with_numeric_enum_values() {
    // Test that numeric enum values work
    let xml = r#"
        <column>
            <radio label="Level 1" value="1" selected="{level}" on_select="setLevel" />
            <radio label="Level 2" value="2" selected="{level}" on_select="setLevel" />
            <radio label="Level 3" value="3" selected="{level}" on_select="setLevel" />
        </column>
    "#;

    let document = parse(xml).unwrap();
    assert_eq!(document.root.children.len(), 3);

    let model = LevelModel { level: Level::Two };

    // Verify numeric enum converts correctly
    let binding = model.get_field(&["level"]);
    assert_eq!(binding, Some(BindingValue::String("2".to_string())));
}

#[test]
fn test_radio_group_type_consistency() {
    // Test that all radios in a group have consistent value types
    let xml = r#"
        <column>
            <radio label="Option 1" value="opt1" selected="{choice}" />
            <radio label="Option 2" value="opt2" selected="{choice}" />
            <radio label="Option 3" value="opt3" selected="{choice}" />
        </column>
    "#;

    let document = parse(xml).unwrap();

    // Verify all radios have value attributes
    for child in &document.root.children {
        assert!(child.attributes.contains_key("value"));
        assert!(child.attributes.contains_key("label"));
        assert!(child.attributes.contains_key("selected"));
    }
}
