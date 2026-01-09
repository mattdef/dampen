//! Integration tests for the hello-world example.
//!
//! These tests verify that the auto-loading pattern works correctly
//! in the context of a complete application.

use dampen_core::{parse, AppState, HandlerRegistry};
use dampen_macros::UiModel;
use serde::{Deserialize, Serialize};

#[derive(Default, UiModel, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Model;

#[test]
fn test_hello_world_xml_parsing() {
    let xml = include_str!("../src/ui/window.dampen");
    let document = parse(xml).expect("Failed to parse window.dampen");
    assert_eq!(document.version.major, 1);
}

#[test]
fn test_hello_world_app_state_creation() {
    let xml = include_str!("../src/ui/window.dampen");
    let document = parse(xml).expect("Failed to parse XML");

    let registry = HandlerRegistry::new();
    let state: AppState<()> = AppState::with_handlers(document, registry);

    assert_eq!(state.document.version.major, 1);
}

#[test]
fn test_hello_world_macro_pattern() {
    let xml = r#"
        <gravity>
            <column>
                <text value="Test" />
            </column>
        </gravity>
    "#;

    let document = parse(xml).expect("Failed to parse");
    assert!(matches!(
        document.root.kind,
        dampen_core::ir::WidgetKind::Column
    ));
}

#[test]
fn test_model_serialization() {
    let model = Model;
    let serialized = serde_json::to_string(&model).expect("Failed to serialize");
    let deserialized: Model = serde_json::from_str(&serialized).expect("Failed to deserialize");
    assert_eq!(model, deserialized);
}
