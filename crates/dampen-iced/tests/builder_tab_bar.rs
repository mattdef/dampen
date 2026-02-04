//! Builder tests for TabBar and Tab widgets

use dampen_core::binding::{BindingValue, UiBindable};
use dampen_core::{HandlerRegistry, parse};
use dampen_iced::{DampenWidgetBuilder, HandlerMessage};
use iced::{Element, Renderer, Theme};

/// Simple test model
#[derive(Clone)]
struct TestModel {
    selected_tab: usize,
}

impl UiBindable for TestModel {
    fn get_field(&self, path: &[&str]) -> Option<BindingValue> {
        match path {
            ["selected_tab"] => Some(BindingValue::Integer(self.selected_tab as i64)),
            _ => None,
        }
    }

    fn available_fields() -> Vec<String> {
        vec!["selected_tab".to_string()]
    }
}

fn create_model() -> TestModel {
    TestModel { selected_tab: 0 }
}

fn create_registry() -> HandlerRegistry {
    HandlerRegistry::new()
}

#[test]
fn test_tab_bar_construction_with_selected_attribute() {
    let xml = r#"
        <dampen version="1.1">
            <tab_bar selected="0" on_select="on_tab_selected">
                <tab label="General" />
                <tab label="Appearance" />
                <tab label="Notifications" />
            </tab_bar>
        </dampen>
    "#;
    let doc = parse(xml).unwrap();
    let model = create_model();
    let registry = create_registry();

    let builder = DampenWidgetBuilder::new(&doc, &model, Some(&registry));
    let _element: Element<'_, HandlerMessage, Theme, Renderer> = builder.build();
}

#[test]
fn test_tab_bar_with_binding() {
    let xml = r#"
        <dampen version="1.1">
            <tab_bar selected="{selected_tab}" on_select="on_tab_selected">
                <tab label="Tab 1" />
                <tab label="Tab 2" />
            </tab_bar>
        </dampen>
    "#;
    let doc = parse(xml).unwrap();
    let model = create_model();
    let registry = create_registry();

    let builder = DampenWidgetBuilder::new(&doc, &model, Some(&registry));
    let _element: Element<'_, HandlerMessage, Theme, Renderer> = builder.build();
}

#[test]
fn test_tab_bar_on_select_event_handler() {
    let xml = r#"
        <dampen version="1.1">
            <tab_bar selected="0" on_select="on_tab_selected">
                <tab label="Tab 1" />
                <tab label="Tab 2" />
            </tab_bar>
        </dampen>
    "#;
    let doc = parse(xml).unwrap();
    let model = create_model();
    let registry = create_registry();

    let builder = DampenWidgetBuilder::new(&doc, &model, Some(&registry));
    let _element: Element<'_, HandlerMessage, Theme, Renderer> = builder.build();
}

#[test]
fn test_tab_bar_empty() {
    let xml = r#"
        <dampen version="1.1">
            <tab_bar selected="0" on_select="on_tab_selected">
            </tab_bar>
        </dampen>
    "#;
    let doc = parse(xml).unwrap();
    let model = create_model();
    let registry = create_registry();

    let builder = DampenWidgetBuilder::new(&doc, &model, Some(&registry));
    let _element: Element<'_, HandlerMessage, Theme, Renderer> = builder.build();
}

#[test]
fn test_tab_bar_without_handler() {
    let xml = r#"
        <dampen version="1.1">
            <tab_bar selected="0">
                <tab label="Tab 1" />
                <tab label="Tab 2" />
            </tab_bar>
        </dampen>
    "#;
    let doc = parse(xml).unwrap();
    let model = create_model();
    let registry = create_registry();

    let builder = DampenWidgetBuilder::new(&doc, &model, Some(&registry));
    let _element: Element<'_, HandlerMessage, Theme, Renderer> = builder.build();
}
