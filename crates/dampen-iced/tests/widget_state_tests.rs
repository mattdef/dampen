//! Integration tests for widget state styling
//!
//! Tests verify that widgets respond to interaction states (hover, active, focus, disabled)
//! by applying the correct style variants from StyleClass definitions.
//!
//! TDD Approach: These tests are written FIRST and should FAIL before implementation.

use dampen_core::ir::style::{
    Background, Border, BorderRadius, BorderStyle, Color, StyleProperties,
};
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

// Helper to create border
fn border(width: f32, r: f32, g: f32, b: f32) -> Border {
    Border {
        width,
        color: color(r, g, b),
        radius: BorderRadius {
            top_left: 0.0,
            top_right: 0.0,
            bottom_right: 0.0,
            bottom_left: 0.0,
        },
        style: BorderStyle::Solid,
    }
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

// ============================================================================
// TEXT INPUT STATE STYLING TESTS
// ============================================================================

/// Test text input focus styling
///
/// Given: A text input with focus state styles defined
/// When: Text input is focused
/// Then: Focus styles should be applied (e.g., blue border)
#[test]
fn test_text_input_focus_styling() {
    // Arrange: Create a style class with base and focus styles
    let mut state_variants = HashMap::new();
    state_variants.insert(
        WidgetState::Focus,
        StyleProperties {
            border: Some(border(2.0, 0.0, 0.5, 1.0)), // Blue border, 2px width (focus)
            ..Default::default()
        },
    );

    let style_class = StyleClass {
        name: "text_input".to_string(),
        style: StyleProperties {
            border: Some(border(1.0, 0.5, 0.5, 0.5)), // Gray border, 1px width (base)
            background: Some(bg_color(1.0, 1.0, 1.0)), // White background
            ..Default::default()
        },
        layout: None,
        extends: vec![],
        state_variants,
        combined_state_variants: HashMap::new(),
    };

    // Act: Resolve focus state
    let focus_style =
        dampen_iced::style_mapping::resolve_state_style(&style_class, WidgetState::Focus);

    // Assert: Focus style should exist
    assert!(focus_style.is_some(), "Focus state should resolve to Some");

    let focus_style = focus_style.unwrap();

    // Check border is blue and thicker
    if let Some(focus_border) = &focus_style.border {
        assert_eq!(focus_border.width, 2.0, "Focus border should be 2px wide");
        assert_eq!(focus_border.color.r, 0.0, "Focus border should be blue");
        assert_eq!(focus_border.color.g, 0.5);
        assert_eq!(focus_border.color.b, 1.0);
    } else {
        panic!("Focus style should have border");
    }

    // Verify base style is different
    if let Some(base_border) = &style_class.style.border {
        assert_eq!(base_border.width, 1.0, "Base border should be 1px wide");
        assert_eq!(base_border.color.r, 0.5, "Base border should be gray");
    }
}

/// Test text input hover styling
///
/// Given: A text input with hover state styles defined
/// When: Text input is hovered
/// Then: Hover styles should be applied (e.g., lighter background)
#[test]
fn test_text_input_hover_styling() {
    // Arrange: Create a style class with base and hover styles
    let mut state_variants = HashMap::new();
    state_variants.insert(
        WidgetState::Hover,
        StyleProperties {
            background: Some(bg_color(0.95, 0.95, 1.0)), // Light blue tint (hover)
            ..Default::default()
        },
    );

    let style_class = StyleClass {
        name: "text_input".to_string(),
        style: StyleProperties {
            background: Some(bg_color(1.0, 1.0, 1.0)), // White (base)
            border: Some(border(1.0, 0.5, 0.5, 0.5)),  // Gray border
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

    // Assert: Hover style should exist
    assert!(hover_style.is_some(), "Hover state should resolve to Some");

    let hover_style = hover_style.unwrap();
    if let Some(Background::Color(hover_color)) = &hover_style.background {
        assert_eq!(hover_color.r, 0.95, "Hover background should be light blue");
        assert_eq!(hover_color.g, 0.95);
        assert_eq!(hover_color.b, 1.0);
    } else {
        panic!("Hover style should have background color");
    }

    // Verify base style is different
    if let Some(Background::Color(base_color)) = &style_class.style.background {
        assert_eq!(base_color.r, 1.0, "Base background should be white");
        assert_eq!(base_color.b, 1.0);
    }
}

/// Test text input disabled styling
///
/// Given: A text input with disabled state styles defined
/// When: Text input is disabled
/// Then: Disabled styles should be applied (grayed out, not editable)
#[test]
fn test_text_input_disabled_styling() {
    // Arrange: Create a style class with disabled styles
    let mut state_variants = HashMap::new();
    state_variants.insert(
        WidgetState::Disabled,
        StyleProperties {
            background: Some(bg_color(0.9, 0.9, 0.9)), // Light gray background (disabled)
            color: Some(color(0.6, 0.6, 0.6)),         // Gray text
            border: Some(border(1.0, 0.7, 0.7, 0.7)),  // Light gray border
            ..Default::default()
        },
    );

    let style_class = StyleClass {
        name: "text_input".to_string(),
        style: StyleProperties {
            background: Some(bg_color(1.0, 1.0, 1.0)), // White (base)
            color: Some(color(0.0, 0.0, 0.0)),         // Black text
            border: Some(border(1.0, 0.5, 0.5, 0.5)),  // Gray border
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

    // Check background is light gray
    if let Some(Background::Color(disabled_bg)) = &disabled_style.background {
        assert_eq!(
            disabled_bg.r, 0.9,
            "Disabled background should be light gray"
        );
        assert_eq!(disabled_bg.g, 0.9);
        assert_eq!(disabled_bg.b, 0.9);
    } else {
        panic!("Disabled style should have background color");
    }

    // Check text color is gray
    if let Some(disabled_color) = disabled_style.color {
        assert_eq!(disabled_color.r, 0.6, "Disabled text should be gray");
        assert_eq!(disabled_color.g, 0.6);
        assert_eq!(disabled_color.b, 0.6);
    } else {
        panic!("Disabled style should have text color");
    }

    // Check border is light gray
    if let Some(disabled_border) = &disabled_style.border {
        assert_eq!(
            disabled_border.color.r, 0.7,
            "Disabled border should be light gray"
        );
        assert_eq!(disabled_border.color.g, 0.7);
        assert_eq!(disabled_border.color.b, 0.7);
    } else {
        panic!("Disabled style should have border");
    }
}
