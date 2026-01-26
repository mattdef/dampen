//! Widget-specific tests for DampenWidgetBuilder

use dampen_core::binding::{BindingValue, UiBindable};
use dampen_core::{HandlerRegistry, parse};
use dampen_iced::{DampenWidgetBuilder, HandlerMessage};
use iced::{Element, Renderer, Theme};

/// Simple test model
#[derive(Clone)]
struct TestModel {
    name: String,
    age: i32,
    active: bool,
    priority: String,
    volume: f32,
}

impl UiBindable for TestModel {
    fn get_field(&self, path: &[&str]) -> Option<BindingValue> {
        match path {
            ["name"] => Some(BindingValue::String(self.name.clone())),
            ["age"] => Some(BindingValue::Integer(self.age as i64)),
            ["active"] => Some(BindingValue::Bool(self.active)),
            ["priority"] => Some(BindingValue::String(self.priority.clone())),
            ["volume"] => Some(BindingValue::Float(self.volume as f64)),
            _ => None,
        }
    }

    fn available_fields() -> Vec<String> {
        vec![
            "name".to_string(),
            "age".to_string(),
            "active".to_string(),
            "priority".to_string(),
            "volume".to_string(),
        ]
    }
}

fn create_model() -> TestModel {
    TestModel {
        name: "Alice".to_string(),
        age: 25,
        active: true,
        priority: "High".to_string(),
        volume: 75.0,
    }
}

fn create_registry() -> HandlerRegistry {
    let mut registry = HandlerRegistry::new();

    // Register simple handlers
    registry.register_simple("update_name", |model: &mut dyn std::any::Any| {
        // Placeholder - would update model
    });

    registry.register_simple("toggle_active", |model: &mut dyn std::any::Any| {
        // Placeholder
    });

    registry.register_simple("set_priority", |model: &mut dyn std::any::Any| {
        // Placeholder
    });

    registry.register_simple("set_volume", |model: &mut dyn std::any::Any| {
        // Placeholder
    });

    registry
}

#[test]
fn test_text_input_static_value() {
    let xml = r#"<text_input value="Hello World" />"#;
    let doc = parse(xml).unwrap();
    let model = create_model();
    let registry = create_registry();

    let builder = DampenWidgetBuilder::new(&doc, &model, Some(&registry));
    let _element: Element<'_, HandlerMessage, Theme, Renderer> = builder.build();
}

#[test]
fn test_text_input_with_binding() {
    let xml = r#"<text_input value="{name}" />"#;
    let doc = parse(xml).unwrap();
    let model = create_model();
    let registry = create_registry();

    let builder = DampenWidgetBuilder::new(&doc, &model, Some(&registry));
    let _element: Element<'_, HandlerMessage, Theme, Renderer> = builder.build();
}

#[test]
fn test_text_input_on_input_event() {
    let xml = r#"<text_input value="test" on_input="update_name" />"#;
    let doc = parse(xml).unwrap();
    let model = create_model();
    let registry = create_registry();

    let builder = DampenWidgetBuilder::new(&doc, &model, Some(&registry));
    let _element: Element<'_, HandlerMessage, Theme, Renderer> = builder.build();
}

#[test]
fn test_checkbox_static_checked() {
    let xml = r#"<checkbox checked="true" />"#;
    let doc = parse(xml).unwrap();
    let model = create_model();
    let registry = create_registry();

    let builder = DampenWidgetBuilder::new(&doc, &model, Some(&registry));
    let _element: Element<'_, HandlerMessage, Theme, Renderer> = builder.build();
}

#[test]
fn test_checkbox_with_binding() {
    let xml = r#"<checkbox checked="{active}" />"#;
    let doc = parse(xml).unwrap();
    let model = create_model();
    let registry = create_registry();

    let builder = DampenWidgetBuilder::new(&doc, &model, Some(&registry));
    let _element: Element<'_, HandlerMessage, Theme, Renderer> = builder.build();
}

#[test]
fn test_checkbox_on_toggle_event() {
    let xml = r#"<checkbox checked="false" on_toggle="toggle_active" />"#;
    let doc = parse(xml).unwrap();
    let model = create_model();
    let registry = create_registry();

    let builder = DampenWidgetBuilder::new(&doc, &model, Some(&registry));
    let _element: Element<'_, HandlerMessage, Theme, Renderer> = builder.build();
}

#[test]
fn test_toggler_static_active() {
    let xml = r#"<toggler active="true" />"#;
    let doc = parse(xml).unwrap();
    let model = create_model();
    let registry = create_registry();

    let builder = DampenWidgetBuilder::new(&doc, &model, Some(&registry));
    let _element: Element<'_, HandlerMessage, Theme, Renderer> = builder.build();
}

#[test]
fn test_toggler_with_binding() {
    let xml = r#"<toggler active="{active}" />"#;
    let doc = parse(xml).unwrap();
    let model = create_model();
    let registry = create_registry();

    let builder = DampenWidgetBuilder::new(&doc, &model, Some(&registry));
    let _element: Element<'_, HandlerMessage, Theme, Renderer> = builder.build();
}

#[test]
fn test_toggler_on_toggle_event() {
    let xml = r#"<toggler active="false" on_toggle="toggle_active" />"#;
    let doc = parse(xml).unwrap();
    let model = create_model();
    let registry = create_registry();

    let builder = DampenWidgetBuilder::new(&doc, &model, Some(&registry));
    let _element: Element<'_, HandlerMessage, Theme, Renderer> = builder.build();
}

#[test]
fn test_pick_list_with_options() {
    let xml = r#"<pick_list options="Low,Medium,High" />"#;
    let doc = parse(xml).unwrap();
    let model = create_model();
    let registry = create_registry();

    let builder = DampenWidgetBuilder::new(&doc, &model, Some(&registry));
    let _element: Element<'_, HandlerMessage, Theme, Renderer> = builder.build();
}

#[test]
fn test_pick_list_with_selected() {
    let xml = r#"<pick_list options="Low,Medium,High" selected="{priority}" />"#;
    let doc = parse(xml).unwrap();
    let model = create_model();
    let registry = create_registry();

    let builder = DampenWidgetBuilder::new(&doc, &model, Some(&registry));
    let _element: Element<'_, HandlerMessage, Theme, Renderer> = builder.build();
}

#[test]
fn test_pick_list_on_select_event() {
    let xml = r#"<pick_list options="Low,Medium,High" on_select="set_priority" />"#;
    let doc = parse(xml).unwrap();
    let model = create_model();
    let registry = create_registry();

    let builder = DampenWidgetBuilder::new(&doc, &model, Some(&registry));
    let _element: Element<'_, HandlerMessage, Theme, Renderer> = builder.build();
}

#[test]
fn test_slider_with_values() {
    let xml = r#"<slider min="0" max="100" value="50" />"#;
    let doc = parse(xml).unwrap();
    let model = create_model();
    let registry = create_registry();

    let builder = DampenWidgetBuilder::new(&doc, &model, Some(&registry));
    let _element: Element<'_, HandlerMessage, Theme, Renderer> = builder.build();
}

#[test]
fn test_slider_with_binding() {
    let xml = r#"<slider min="0" max="100" value="{volume}" />"#;
    let doc = parse(xml).unwrap();
    let model = create_model();
    let registry = create_registry();

    let builder = DampenWidgetBuilder::new(&doc, &model, Some(&registry));
    let _element: Element<'_, HandlerMessage, Theme, Renderer> = builder.build();
}

#[test]
fn test_slider_on_change_event() {
    let xml = r#"<slider min="0" max="100" value="50" on_change="set_volume" />"#;
    let doc = parse(xml).unwrap();
    let model = create_model();
    let registry = create_registry();

    let builder = DampenWidgetBuilder::new(&doc, &model, Some(&registry));
    let _element: Element<'_, HandlerMessage, Theme, Renderer> = builder.build();
}

#[test]
fn test_image_with_src() {
    let xml = r#"<image src="test.png" />"#;
    let doc = parse(xml).unwrap();
    let model = create_model();
    let registry = create_registry();

    let builder = DampenWidgetBuilder::new(&doc, &model, Some(&registry));
    let _element: Element<'_, HandlerMessage, Theme, Renderer> = builder.build();
}

#[test]
fn test_image_with_dimensions() {
    let xml = r#"<image src="test.png" width="100" height="50" />"#;
    let doc = parse(xml).unwrap();
    let model = create_model();
    let registry = create_registry();

    let builder = DampenWidgetBuilder::new(&doc, &model, Some(&registry));
    let _element: Element<'_, HandlerMessage, Theme, Renderer> = builder.build();
}

#[test]
fn test_image_with_invalid_src() {
    let xml = r#"<image src="" />"#;
    let doc = parse(xml).unwrap();
    let model = create_model();
    let registry = create_registry();

    let builder = DampenWidgetBuilder::new(&doc, &model, Some(&registry));
    let _element: Element<'_, HandlerMessage, Theme, Renderer> = builder.build();
}
