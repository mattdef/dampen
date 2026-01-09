//! Basic tests for DampenWidgetBuilder

use dampen_core::binding::{BindingValue, UiBindable};
use dampen_core::{parse, HandlerRegistry};
use dampen_iced::{DampenWidgetBuilder, HandlerMessage};
use iced::{Element, Renderer, Theme};

/// Simple test model
#[derive(Clone)]
struct TestModel {
    count: i32,
    message: String,
}

impl UiBindable for TestModel {
    fn get_field(&self, path: &[&str]) -> Option<BindingValue> {
        match path {
            ["count"] => Some(BindingValue::Integer(self.count as i64)),
            ["message"] => Some(BindingValue::String(self.message.clone())),
            _ => None,
        }
    }

    fn available_fields() -> Vec<String> {
        vec!["count".to_string(), "message".to_string()]
    }
}

fn create_model() -> TestModel {
    TestModel {
        count: 42,
        message: "Hello".to_string(),
    }
}

fn create_registry() -> HandlerRegistry {
    HandlerRegistry::new()
}

#[test]
fn test_text_static() {
    let xml = r#"<text value="Hello World" />"#;
    let doc = parse(xml).unwrap();
    let model = create_model();
    let registry = create_registry();

    let builder = DampenWidgetBuilder::new(&doc, &model, Some(&registry));
    let _element: Element<'_, HandlerMessage, Theme, Renderer> = builder.build();
}

#[test]
fn test_text_with_binding() {
    let xml = r#"<text value="{message}" />"#;
    let doc = parse(xml).unwrap();
    let model = create_model();
    let registry = create_registry();

    let builder = DampenWidgetBuilder::new(&doc, &model, Some(&registry));
    let _element: Element<'_, HandlerMessage, Theme, Renderer> = builder.build();
}

#[test]
fn test_column_with_children() {
    let xml = r#"
        <column spacing="10">
            <text value="Item 1" />
            <text value="Item 2" />
        </column>
    "#;
    let doc = parse(xml).unwrap();
    let model = create_model();
    let registry = create_registry();

    let builder = DampenWidgetBuilder::new(&doc, &model, Some(&registry));
    let _element: Element<'_, HandlerMessage, Theme, Renderer> = builder.build();
}

#[test]
fn test_button_with_event() {
    let xml = r#"<button label="Click me" on_click="handler1" />"#;
    let doc = parse(xml).unwrap();
    let model = create_model();
    let registry = create_registry();

    let builder = DampenWidgetBuilder::new(&doc, &model, Some(&registry));
    let _element: Element<'_, HandlerMessage, Theme, Renderer> = builder.build();
}

#[test]
fn test_verbose_logging() {
    let xml = r#"<text value="Test" />"#;
    let doc = parse(xml).unwrap();
    let model = create_model();
    let registry = create_registry();

    let builder = DampenWidgetBuilder::new(&doc, &model, Some(&registry)).with_verbose(true);

    let _element: Element<'_, HandlerMessage, Theme, Renderer> = builder.build();
}

#[test]
fn test_interpolated_string() {
    let xml = r#"<text value="Count: {count}, Message: {message}" />"#;
    let doc = parse(xml).unwrap();
    let model = create_model();
    let registry = create_registry();

    let builder = DampenWidgetBuilder::new(&doc, &model, Some(&registry));
    let _element: Element<'_, HandlerMessage, Theme, Renderer> = builder.build();
}

#[test]
fn test_with_style() {
    let xml = r##"<text value="Styled" background="#FF0000" padding="10" />"##;
    let doc = parse(xml).unwrap();
    let model = create_model();
    let registry = create_registry();

    let builder = DampenWidgetBuilder::new(&doc, &model, Some(&registry));
    let _element: Element<'_, HandlerMessage, Theme, Renderer> = builder.build();
}

#[test]
fn test_without_registry() {
    let xml = r#"<button label="No Handler" />"#;
    let doc = parse(xml).unwrap();
    let model = create_model();

    let builder = DampenWidgetBuilder::new(&doc, &model, None);
    let _element: Element<'_, HandlerMessage, Theme, Renderer> = builder.build();
}

#[test]
fn test_complex_nested() {
    let xml = r#"
        <container padding="20">
            <column spacing="10">
                <row spacing="5">
                    <text value="Left" />
                    <text value="Right" />
                </row>
                <button label="Submit" on_click="submit" />
            </column>
        </container>
    "#;
    let doc = parse(xml).unwrap();
    let model = create_model();
    let registry = create_registry();

    let builder = DampenWidgetBuilder::new(&doc, &model, Some(&registry));
    let _element: Element<'_, HandlerMessage, Theme, Renderer> = builder.build();
}

#[test]
fn test_button_enabled_static_true() {
    let xml = r#"<button label="Click" on_click="handler1" enabled="true" />"#;
    let doc = parse(xml).unwrap();
    let model = create_model();
    let registry = create_registry();

    let builder = DampenWidgetBuilder::new(&doc, &model, Some(&registry));
    let _element: Element<'_, HandlerMessage, Theme, Renderer> = builder.build();
}

#[test]
fn test_button_enabled_static_false() {
    let xml = r#"<button label="Click" on_click="handler1" enabled="false" />"#;
    let doc = parse(xml).unwrap();
    let model = create_model();
    let registry = create_registry();

    let builder = DampenWidgetBuilder::new(&doc, &model, Some(&registry));
    let _element: Element<'_, HandlerMessage, Theme, Renderer> = builder.build();
}

#[test]
fn test_button_enabled_binding() {
    let xml = r#"<button label="Click" on_click="handler1" enabled="{count > 0}" />"#;
    let doc = parse(xml).unwrap();
    let model = create_model();
    let registry = create_registry();

    let builder = DampenWidgetBuilder::new(&doc, &model, Some(&registry));
    let _element: Element<'_, HandlerMessage, Theme, Renderer> = builder.build();
}

#[test]
fn test_button_enabled_binding_false() {
    let xml = r#"<button label="Click" on_click="handler1" enabled="{count &lt; 0}" />"#;
    let doc = parse(xml).unwrap();
    let model = create_model();
    let registry = create_registry();

    let builder = DampenWidgetBuilder::new(&doc, &model, Some(&registry));
    let _element: Element<'_, HandlerMessage, Theme, Renderer> = builder.build();
}

#[test]
fn test_button_enabled_with_verbose() {
    let xml = r#"<button label="Decrement" on_click="handler1" enabled="{count > 0}" />"#;
    let doc = parse(xml).unwrap();
    let model = create_model();
    let registry = create_registry();

    let builder = DampenWidgetBuilder::new(&doc, &model, Some(&registry)).with_verbose(true);
    let _element: Element<'_, HandlerMessage, Theme, Renderer> = builder.build();
}
