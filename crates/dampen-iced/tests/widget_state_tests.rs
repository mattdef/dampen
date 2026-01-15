//! Integration tests for widget state styling
//!
//! Tests verify that widgets respond to interaction states (hover, active, focus, disabled)
//! by applying the correct style variants from StyleClass definitions.
//!
//! TDD Approach: These tests are written FIRST and should FAIL before implementation.

use dampen_core::ir::style::{Background, Color, StyleProperties};
use dampen_core::ir::theme::{StyleClass, WidgetState};
use std::collections::HashMap;

// Helper to create test colors
fn color(r: f32, g: f32, b: f32) -> Color {
    Color { r, g, b, a: 1.0 }
}

// Helper to create background color
fn bg_color(r: f32, g: f32, b: f32) -> Background {
    Background::Color(color(r, g, b))
}

/// Test button hover styling
///
/// Given: A button with hover state styles defined
/// When: Button is in hover state
/// Then: Hover styles should be applied (not base styles)
#[test]
fn test_button_hover_styling() {
    // Arrange: Create a style class with base and hover styles
    let mut state_variants = HashMap::new();
    state_variants.insert(
        WidgetState::Hover,
        StyleProperties {
            background: Some(bg_color(0.5, 0.7, 1.0)), // Light blue (hover)
            ..Default::default()
        },
    );

    let style_class = StyleClass {
        name: "button".to_string(),
        style: StyleProperties {
            background: Some(bg_color(0.2, 0.4, 0.8)), // Dark blue (base)
            ..Default::default()
        },
        layout: None,
        extends: vec![],
        state_variants,
        combined_state_variants: HashMap::new(),
    };

    // Act: Resolve hover state
    let hover_style =
        dampen_iced::style_mapping::resolve_state_style(&style_class, WidgetState::Hover);

    // Assert: Hover style should exist and be different from base
    assert!(hover_style.is_some(), "Hover state should resolve to Some");

    let hover_style = hover_style.unwrap();
    if let Some(Background::Color(hover_color)) = &hover_style.background {
        assert_eq!(hover_color.r, 0.5, "Hover background should be light blue");
        assert_eq!(hover_color.g, 0.7);
        assert_eq!(hover_color.b, 1.0);
    } else {
        panic!("Hover style should have background color");
    }

    // Verify base style is different
    if let Some(Background::Color(base_color)) = &style_class.style.background {
        assert_eq!(base_color.r, 0.2, "Base background should be dark blue");
        assert_ne!(base_color.r, 0.5, "Base and hover colors should differ");
    }
}

/// Test button active (pressed) styling
///
/// Given: A button with active state styles defined
/// When: Button is pressed (active state)
/// Then: Active styles should be applied
#[test]
fn test_button_active_styling() {
    // Arrange: Create a style class with active styles
    let mut state_variants = HashMap::new();
    state_variants.insert(
        WidgetState::Active,
        StyleProperties {
            background: Some(bg_color(0.1, 0.2, 0.5)), // Very dark blue (active/pressed)
            ..Default::default()
        },
    );

    let style_class = StyleClass {
        name: "button".to_string(),
        style: StyleProperties {
            background: Some(bg_color(0.2, 0.4, 0.8)), // Dark blue (base)
            ..Default::default()
        },
        layout: None,
        extends: vec![],
        state_variants,
        combined_state_variants: HashMap::new(),
    };

    // Act: Resolve active state
    let active_style =
        dampen_iced::style_mapping::resolve_state_style(&style_class, WidgetState::Active);

    // Assert: Active style should exist
    assert!(
        active_style.is_some(),
        "Active state should resolve to Some"
    );

    let active_style = active_style.unwrap();
    if let Some(Background::Color(active_color)) = &active_style.background {
        assert_eq!(
            active_color.r, 0.1,
            "Active background should be very dark blue"
        );
        assert_eq!(active_color.g, 0.2);
        assert_eq!(active_color.b, 0.5);
    } else {
        panic!("Active style should have background color");
    }
}

/// Test button disabled styling
///
/// Given: A button with disabled state styles defined
/// When: Button is disabled
/// Then: Disabled styles should be applied (typically grayed out)
#[test]
fn test_button_disabled_styling() {
    // Arrange: Create a style class with disabled styles
    let mut state_variants = HashMap::new();
    state_variants.insert(
        WidgetState::Disabled,
        StyleProperties {
            background: Some(bg_color(0.5, 0.5, 0.5)), // Gray (disabled)
            color: Some(color(0.7, 0.7, 0.7)),         // Light gray text
            ..Default::default()
        },
    );

    let style_class = StyleClass {
        name: "button".to_string(),
        style: StyleProperties {
            background: Some(bg_color(0.2, 0.4, 0.8)), // Blue (base)
            color: Some(color(1.0, 1.0, 1.0)),         // White text
            ..Default::default()
        },
        layout: None,
        extends: vec![],
        state_variants,
        combined_state_variants: HashMap::new(),
    };

    // Act: Resolve disabled state
    let disabled_style =
        dampen_iced::style_mapping::resolve_state_style(&style_class, WidgetState::Disabled);

    // Assert: Disabled style should exist
    assert!(
        disabled_style.is_some(),
        "Disabled state should resolve to Some"
    );

    let disabled_style = disabled_style.unwrap();

    // Check background is gray
    if let Some(Background::Color(disabled_bg)) = &disabled_style.background {
        assert_eq!(disabled_bg.r, 0.5, "Disabled background should be gray");
        assert_eq!(disabled_bg.g, 0.5);
        assert_eq!(disabled_bg.b, 0.5);
    } else {
        panic!("Disabled style should have background color");
    }

    // Check text color is light gray
    if let Some(disabled_color) = disabled_style.color {
        assert_eq!(disabled_color.r, 0.7, "Disabled text should be light gray");
        assert_eq!(disabled_color.g, 0.7);
        assert_eq!(disabled_color.b, 0.7);
    } else {
        panic!("Disabled style should have text color");
    }
}

/// Test fallback to base style when state variant is not defined
///
/// Given: A button with NO hover state styles defined
/// When: Button is in hover state
/// Then: Should fall back to base styles (resolve_state_style returns None)
#[test]
fn test_fallback_to_base_style() {
    // Arrange: Create a style class with NO state variants
    let style_class = StyleClass {
        name: "button".to_string(),
        style: StyleProperties {
            background: Some(bg_color(0.2, 0.4, 0.8)), // Blue (base)
            ..Default::default()
        },
        layout: None,
        extends: vec![],
        state_variants: HashMap::new(), // No state variants!
        combined_state_variants: HashMap::new(),
    };

    // Act: Try to resolve hover state (should return None)
    let hover_style =
        dampen_iced::style_mapping::resolve_state_style(&style_class, WidgetState::Hover);

    // Assert: Should return None, indicating fallback to base
    assert!(
        hover_style.is_none(),
        "Hover state should return None when not defined, triggering fallback to base"
    );

    // Verify base style exists
    assert!(
        style_class.style.background.is_some(),
        "Base style should have background"
    );

    if let Some(Background::Color(base_color)) = &style_class.style.background {
        assert_eq!(base_color.r, 0.2, "Base background should be used");
    }
}
