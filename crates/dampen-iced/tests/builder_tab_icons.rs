//! Builder tests for Tab icons

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
fn test_icon_resolution_all_supported_icons() {
    // Test all 10 supported icons
    let icons = vec![
        ("home", '\u{F015}'),
        ("settings", '\u{F013}'),
        ("user", '\u{F007}'),
        ("search", '\u{F002}'),
        ("add", '\u{F067}'),
        ("delete", '\u{F1F8}'),
        ("edit", '\u{F044}'),
        ("save", '\u{F0C7}'),
        ("close", '\u{F00D}'),
        ("back", '\u{F060}'),
        ("forward", '\u{F061}'),
    ];

    for (icon_name, expected_unicode) in icons {
        let xml = format!(
            r#"<dampen version="1.1">
                <tab_bar selected="0" on_select="on_tab_selected">
                    <tab label="Tab" icon="{}" />
                </tab_bar>
            </dampen>
            "#,
            icon_name
        );

        let doc = parse(&xml).unwrap();
        let model = create_model();
        let registry = create_registry();

        let builder = DampenWidgetBuilder::new(&doc, &model, Some(&registry));
        let _element: Element<'_, HandlerMessage, Theme, Renderer> = builder.build();

        // The icon should be resolved correctly during build
        // We verify the build succeeds without errors
    }
}

#[test]
fn test_tab_label_icon_text_construction() {
    let xml = r#"
        <dampen version="1.1">
            <tab_bar selected="0" on_select="on_tab_selected">
                <tab label="Home" icon="home" />
                <tab label="Settings" icon="settings" />
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
fn test_tab_with_only_icon_no_label() {
    let xml = r#"
        <dampen version="1.1">
            <tab_bar selected="0" on_select="on_tab_selected">
                <tab icon="home" />
                <tab icon="settings" />
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
fn test_unknown_icon_fallback() {
    // Unknown icons should fallback to circle (\u{F111})
    let xml = r#"
        <dampen version="1.1">
            <tab_bar selected="0" on_select="on_tab_selected">
                <tab label="Unknown" icon="unknown_icon" />
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
fn test_icon_size_attribute() {
    let xml = r#"
        <dampen version="1.1">
            <tab_bar selected="0" on_select="on_tab_selected" icon_size="24">
                <tab label="Home" icon="home" />
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
fn test_text_size_attribute() {
    let xml = r#"
        <dampen version="1.1">
            <tab_bar selected="0" on_select="on_tab_selected" text_size="16">
                <tab label="Home" icon="home" />
            </tab_bar>
        </dampen>
    "#;

    let doc = parse(xml).unwrap();
    let model = create_model();
    let registry = create_registry();

    let builder = DampenWidgetBuilder::new(&doc, &model, Some(&registry));
    let _element: Element<'_, HandlerMessage, Theme, Renderer> = builder.build();
}
