//! Complex binding tests for GravityWidgetBuilder

use gravity_core::binding::{BindingValue, UiBindable};
use gravity_core::{parse, HandlerRegistry};
use gravity_iced::{GravityWidgetBuilder, HandlerMessage};
use iced::{Element, Renderer, Theme};

/// Complex test model with nested data
#[derive(Clone)]
struct ComplexModel {
    count: i32,
    price: f64,
    user: User,
    items: Vec<String>,
}

#[derive(Clone)]
struct User {
    name: String,
    email: String,
}

impl UiBindable for ComplexModel {
    fn get_field(&self, path: &[&str]) -> Option<BindingValue> {
        match path {
            ["count"] => Some(BindingValue::Integer(self.count as i64)),
            ["price"] => Some(BindingValue::Float(self.price)),
            ["user", "name"] => Some(BindingValue::String(self.user.name.clone())),
            ["user", "email"] => Some(BindingValue::String(self.user.email.clone())),
            ["items"] => Some(BindingValue::List(
                self.items
                    .iter()
                    .map(|s| BindingValue::String(s.clone()))
                    .collect(),
            )),
            _ => None,
        }
    }

    fn available_fields() -> Vec<String> {
        vec![
            "count".to_string(),
            "price".to_string(),
            "user.name".to_string(),
            "user.email".to_string(),
            "items".to_string(),
        ]
    }
}

fn create_complex_model() -> ComplexModel {
    ComplexModel {
        count: 42,
        price: 19.99,
        user: User {
            name: "Alice".to_string(),
            email: "alice@example.com".to_string(),
        },
        items: vec!["item1".to_string(), "item2".to_string()],
    }
}

fn create_registry() -> HandlerRegistry {
    HandlerRegistry::new()
}

// T057: Test nested field access
#[test]
fn test_nested_field_access() {
    let xml = r#"<text value="{user.name}" />"#;
    let doc = parse(xml).unwrap();
    let model = create_complex_model();
    let registry = create_registry();

    let builder = GravityWidgetBuilder::new(&doc.root, &model, Some(&registry));
    let _element: Element<'_, HandlerMessage, Theme, Renderer> = builder.build();
}

// T058: Test method calls in bindings
#[test]
fn test_method_calls() {
    let xml = r#"<text value="{items.len()}" />"#;
    let doc = parse(xml).unwrap();
    let model = create_complex_model();
    let registry = create_registry();

    let builder = GravityWidgetBuilder::new(&doc.root, &model, Some(&registry));
    let _element: Element<'_, HandlerMessage, Theme, Renderer> = builder.build();
}

// T059: Test binary operations
#[test]
fn test_binary_operations() {
    let xml = r#"<text value="{count * 2}" />"#;
    let doc = parse(xml).unwrap();
    let model = create_complex_model();
    let registry = create_registry();

    let builder = GravityWidgetBuilder::new(&doc.root, &model, Some(&registry));
    let _element: Element<'_, HandlerMessage, Theme, Renderer> = builder.build();
}

// T060: Test conditionals
#[test]
fn test_conditionals() {
    let xml = r#"<text value="{if count > 10 then 'High' else 'Low'}" />"#;
    let doc = parse(xml).unwrap();
    let model = create_complex_model();
    let registry = create_registry();

    let builder = GravityWidgetBuilder::new(&doc.root, &model, Some(&registry));
    let _element: Element<'_, HandlerMessage, Theme, Renderer> = builder.build();
}

// T061: Test formatted strings
#[test]
fn test_formatted_strings() {
    let xml = r#"<text value="Price: ${price}, User: {user.name}" />"#;
    let doc = parse(xml).unwrap();
    let model = create_complex_model();
    let registry = create_registry();

    let builder = GravityWidgetBuilder::new(&doc.root, &model, Some(&registry));
    let _element: Element<'_, HandlerMessage, Theme, Renderer> = builder.build();
}

// T066: Test missing attributes (should use defaults)
#[test]
fn test_missing_attributes() {
    let xml = r#"<text value="No size or weight" />"#;
    let doc = parse(xml).unwrap();
    let model = create_complex_model();
    let registry = create_registry();

    let builder = GravityWidgetBuilder::new(&doc.root, &model, Some(&registry));
    let _element: Element<'_, HandlerMessage, Theme, Renderer> = builder.build();
}

// T067: Test invalid attribute values (should use defaults)
#[test]
fn test_invalid_attribute_values() {
    let xml = r#"<text value="Test" size="not_a_number" />"#;
    let doc = parse(xml).unwrap();
    let model = create_complex_model();
    let registry = create_registry();

    let builder = GravityWidgetBuilder::new(&doc.root, &model, Some(&registry));
    let _element: Element<'_, HandlerMessage, Theme, Renderer> = builder.build();
}

// T068: Test deeply nested widgets
#[test]
fn test_deeply_nested_widgets() {
    let xml = r#"
        <container padding="10">
            <column spacing="5">
                <row spacing="3">
                    <container padding="2">
                        <text value="Deep 1" />
                    </container>
                    <container padding="2">
                        <text value="Deep 2" />
                    </container>
                </row>
                <container padding="2">
                    <column spacing="1">
                        <text value="Deep 3" />
                        <text value="Deep 4" />
                    </column>
                </container>
            </column>
        </container>
    "#;
    let doc = parse(xml).unwrap();
    let model = create_complex_model();
    let registry = create_registry();

    let builder = GravityWidgetBuilder::new(&doc.root, &model, Some(&registry));
    let _element: Element<'_, HandlerMessage, Theme, Renderer> = builder.build();
}

// T069: Test empty containers
#[test]
fn test_empty_containers() {
    let xml = r#"
        <column spacing="10">
            <container></container>
            <text value="After empty" />
        </column>
    "#;
    let doc = parse(xml).unwrap();
    let model = create_complex_model();
    let registry = create_registry();

    let builder = GravityWidgetBuilder::new(&doc.root, &model, Some(&registry));
    let _element: Element<'_, HandlerMessage, Theme, Renderer> = builder.build();
}

// T070: Test mixed widget types
#[test]
fn test_mixed_widget_types() {
    let xml = r#"
        <column spacing="10">
            <text value="Text" />
            <button label="Button" on_click="click" />
            <container padding="5">
                <text value="In container" />
            </container>
            <row spacing="5">
                <text value="Row 1" />
                <text value="Row 2" />
            </row>
            <scrollable>
                <text value="Scrollable" />
            </scrollable>
        </column>
    "#;
    let doc = parse(xml).unwrap();
    let model = create_complex_model();
    let registry = create_registry();

    let builder = GravityWidgetBuilder::new(&doc.root, &model, Some(&registry));
    let _element: Element<'_, HandlerMessage, Theme, Renderer> = builder.build();
}
