//! Integration tests for inline state styles (T014, T015)
//!
//! Tests verify that inline state-prefixed attributes (e.g., hover:background="#ff0000")
//! are correctly parsed and applied to widgets during rendering.
//!
//! TDD Approach: These tests are written FIRST and should FAIL before implementation.

use dampen_core::ir::style::{Background, Color};
use dampen_core::ir::theme::WidgetState;
use dampen_core::parser::parse;

// ============================================================================
// T014: Test button with hover state style
// ============================================================================

#[test]
fn test_button_with_inline_hover_state() {
    // Given: XML with button that has inline hover:background style
    let xml = r##"
        <dampen version="1.0" xmlns:hover="urn:dampen:state:hover">
            <button label="Click Me" hover:background="#ff0000" />
        </dampen>
    "##;

    // When: Parse the XML
    let result = parse(xml);
    assert!(result.is_ok(), "Should parse XML with inline hover state");
    let doc = result.unwrap();

    // Then: Button should have hover state in inline_state_variants
    let button = &doc.root;
    assert!(
        button
            .inline_state_variants
            .contains_key(&WidgetState::Hover),
        "Button should have hover state variant"
    );

    // And: Hover state should have red background
    let hover_style = button
        .inline_state_variants
        .get(&WidgetState::Hover)
        .unwrap();
    assert!(
        hover_style.background.is_some(),
        "Hover state should have background"
    );

    match &hover_style.background {
        Some(Background::Color(c)) => {
            assert_eq!(c.r, 1.0, "Hover background should be red (r=1.0)");
            assert_eq!(c.g, 0.0, "Hover background should be red (g=0.0)");
            assert_eq!(c.b, 0.0, "Hover background should be red (b=0.0)");
        }
        _ => panic!("Expected color background"),
    }
}

// ============================================================================
// T015: Test button with multiple state styles
// ============================================================================

#[test]
fn test_button_with_multiple_inline_states() {
    // Given: XML with button that has multiple inline state styles
    let xml = r##"
        <dampen version="1.0"
            xmlns:hover="urn:dampen:state:hover"
            xmlns:active="urn:dampen:state:active"
            xmlns:disabled="urn:dampen:state:disabled">
            <button
                label="Click Me"
                background="#0000ff"
                hover:background="#ff0000"
                active:background="#00ff00"
                disabled:opacity="0.5"
            />
        </dampen>
    "##;

    // When: Parse the XML
    let result = parse(xml);
    assert!(
        result.is_ok(),
        "Should parse XML with multiple inline states"
    );
    let doc = result.unwrap();

    // Then: Button should have all three state variants
    let button = &doc.root;
    assert!(
        button
            .inline_state_variants
            .contains_key(&WidgetState::Hover),
        "Button should have hover state"
    );
    assert!(
        button
            .inline_state_variants
            .contains_key(&WidgetState::Active),
        "Button should have active state"
    );
    assert!(
        button
            .inline_state_variants
            .contains_key(&WidgetState::Disabled),
        "Button should have disabled state"
    );

    // And: Hover state should have red background
    let hover_style = button
        .inline_state_variants
        .get(&WidgetState::Hover)
        .unwrap();
    match &hover_style.background {
        Some(Background::Color(c)) => {
            assert_eq!(c.r, 1.0, "Hover background should be red");
            assert_eq!(c.g, 0.0);
            assert_eq!(c.b, 0.0);
        }
        _ => panic!("Expected color background for hover"),
    }

    // And: Active state should have green background
    let active_style = button
        .inline_state_variants
        .get(&WidgetState::Active)
        .unwrap();
    match &active_style.background {
        Some(Background::Color(c)) => {
            assert_eq!(c.r, 0.0, "Active background should be green");
            assert_eq!(c.g, 1.0);
            assert_eq!(c.b, 0.0);
        }
        _ => panic!("Expected color background for active"),
    }

    // And: Disabled state should have opacity
    let disabled_style = button
        .inline_state_variants
        .get(&WidgetState::Disabled)
        .unwrap();
    assert!(
        disabled_style.opacity.is_some(),
        "Disabled state should have opacity"
    );
    assert_eq!(
        disabled_style.opacity.unwrap(),
        0.5,
        "Disabled opacity should be 0.5"
    );

    // And: Base style should have blue background
    assert!(button.style.is_some(), "Button should have base style");
    let base_style = button.style.as_ref().unwrap();
    match &base_style.background {
        Some(Background::Color(c)) => {
            assert_eq!(c.r, 0.0, "Base background should be blue");
            assert_eq!(c.g, 0.0);
            assert_eq!(c.b, 1.0);
        }
        _ => panic!("Expected color background for base"),
    }
}
