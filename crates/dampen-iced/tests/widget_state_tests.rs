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

// ============================================================================
// PHASE 8: INTEGRATION & EDGE CASES TESTS
// ============================================================================

/// Test inline style precedence over class-based state styles
///
/// Given: A widget with both class-based state styles and inline styles
/// When: Widget is in a state with class-based styles defined
/// Then: Inline styles should take precedence over class-based state styles
///
/// This verifies that the style precedence hierarchy is maintained:
/// Inline styles > State styles > Base class styles
#[test]
fn test_inline_style_precedence() {
    use dampen_iced::style_mapping::merge_style_properties;

    // Arrange: Create a style class with state variants
    let mut state_variants = HashMap::new();
    state_variants.insert(
        WidgetState::Hover,
        StyleProperties {
            background: Some(bg_color(0.5, 0.7, 1.0)), // Light blue (hover)
            color: Some(color(1.0, 1.0, 1.0)),         // White text
            border: Some(border(2.0, 0.3, 0.6, 1.0)),  // Blue border
            ..Default::default()
        },
    );

    let style_class = StyleClass {
        name: "button".to_string(),
        style: StyleProperties {
            background: Some(bg_color(0.2, 0.4, 0.8)), // Dark blue (base)
            color: Some(color(1.0, 1.0, 1.0)),         // White text
            border: Some(border(1.0, 0.5, 0.5, 0.5)),  // Gray border
            ..Default::default()
        },
        layout: None,
        extends: vec![],
        state_variants,
        combined_state_variants: HashMap::new(),
    };

    // Inline styles (e.g., from XML attributes: background="#ff0000" color="#000000")
    let inline_styles = StyleProperties {
        background: Some(bg_color(1.0, 0.0, 0.0)), // Red background (inline)
        color: Some(color(0.0, 0.0, 0.0)),         // Black text (inline)
        // Note: No border in inline styles
        ..Default::default()
    };

    // Act: Resolve hover state and merge with inline styles
    let hover_style =
        dampen_iced::style_mapping::resolve_state_style(&style_class, WidgetState::Hover);
    assert!(hover_style.is_some(), "Hover state should resolve");

    // First merge base with hover state
    let state_merged = merge_style_properties(&style_class.style, hover_style.as_ref().unwrap());

    // Then merge with inline styles (inline takes precedence)
    let final_style = merge_style_properties(&state_merged, &inline_styles);

    // Assert: Inline styles should override state styles
    if let Some(Background::Color(final_bg)) = &final_style.background {
        assert_eq!(
            final_bg.r, 1.0,
            "Inline background (red) should override hover background (light blue)"
        );
        assert_eq!(final_bg.g, 0.0);
        assert_eq!(final_bg.b, 0.0);
    } else {
        panic!("Final style should have background color");
    }

    if let Some(final_color) = final_style.color {
        assert_eq!(
            final_color.r, 0.0,
            "Inline color (black) should override hover color (white)"
        );
        assert_eq!(final_color.g, 0.0);
        assert_eq!(final_color.b, 0.0);
    } else {
        panic!("Final style should have text color");
    }

    // Border should come from hover state (not overridden by inline)
    if let Some(final_border) = &final_style.border {
        assert_eq!(
            final_border.width, 2.0,
            "Border should come from hover state (not overridden)"
        );
        assert_eq!(
            final_border.color.r, 0.3,
            "Border should be blue from hover"
        );
        assert_eq!(final_border.color.g, 0.6);
        assert_eq!(final_border.color.b, 1.0);
    } else {
        panic!("Final style should have border from hover state");
    }
}

/// Test combined state priority (state resolution order)
///
/// Given: Multiple state variants defined for a widget
/// When: Determining which state style to apply
/// Then: Should follow priority: Disabled > Active > Hover > Focus > Base
///
/// This test verifies the state priority hierarchy. When multiple states
/// could apply, the highest priority state should be used.
#[test]
fn test_combined_state_priority() {
    // Arrange: Create a style class with ALL state variants
    let mut state_variants = HashMap::new();

    state_variants.insert(
        WidgetState::Hover,
        StyleProperties {
            background: Some(bg_color(0.5, 0.7, 1.0)), // Light blue (hover)
            ..Default::default()
        },
    );

    state_variants.insert(
        WidgetState::Active,
        StyleProperties {
            background: Some(bg_color(0.1, 0.2, 0.5)), // Very dark blue (active)
            ..Default::default()
        },
    );

    state_variants.insert(
        WidgetState::Focus,
        StyleProperties {
            background: Some(bg_color(0.3, 0.5, 0.9)), // Medium blue (focus)
            ..Default::default()
        },
    );

    state_variants.insert(
        WidgetState::Disabled,
        StyleProperties {
            background: Some(bg_color(0.5, 0.5, 0.5)), // Gray (disabled)
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

    // Act & Assert: Verify each state resolves to its own color

    // 1. Disabled state (highest priority)
    let disabled_style =
        dampen_iced::style_mapping::resolve_state_style(&style_class, WidgetState::Disabled);
    assert!(disabled_style.is_some(), "Disabled state should resolve");
    if let Some(Background::Color(color)) = &disabled_style.unwrap().background {
        assert_eq!(
            color.r, 0.5,
            "Disabled state should return gray (highest priority)"
        );
    }

    // 2. Active state (second priority)
    let active_style =
        dampen_iced::style_mapping::resolve_state_style(&style_class, WidgetState::Active);
    assert!(active_style.is_some(), "Active state should resolve");
    if let Some(Background::Color(color)) = &active_style.unwrap().background {
        assert_eq!(color.r, 0.1, "Active state should return very dark blue");
    }

    // 3. Hover state (third priority)
    let hover_style =
        dampen_iced::style_mapping::resolve_state_style(&style_class, WidgetState::Hover);
    assert!(hover_style.is_some(), "Hover state should resolve");
    if let Some(Background::Color(color)) = &hover_style.unwrap().background {
        assert_eq!(color.r, 0.5, "Hover state should return light blue");
    }

    // 4. Focus state (fourth priority)
    let focus_style =
        dampen_iced::style_mapping::resolve_state_style(&style_class, WidgetState::Focus);
    assert!(focus_style.is_some(), "Focus state should resolve");
    if let Some(Background::Color(color)) = &focus_style.unwrap().background {
        assert_eq!(color.r, 0.3, "Focus state should return medium blue");
    }

    // 5. Verify all states are different colors (no conflicts)
    assert!(
        disabled_style.unwrap().background != active_style.as_ref().unwrap().background,
        "Disabled and Active should have different colors"
    );
    assert!(
        active_style.unwrap().background != hover_style.as_ref().unwrap().background,
        "Active and Hover should have different colors"
    );
    assert!(
        hover_style.unwrap().background != focus_style.as_ref().unwrap().background,
        "Hover and Focus should have different colors"
    );

    // Note: The priority is enforced by the widget builder choosing which state
    // to query based on current interaction. This test verifies that each state
    // can be resolved independently with correct styles.
}

/// Test hot-reload preserves interaction state
///
/// Given: A widget in an interaction state (e.g., hover) with XML definition loaded
/// When: XML is reloaded (hot-reload scenario)
/// Then: Interaction state should be preserved (hover remains hover)
///
/// This test simulates hot-reload by verifying that:
/// 1. State styles are resolved before reload
/// 2. State styles are resolved after reload with same document structure
/// 3. The resolved styles remain consistent (state preserved)
///
/// Note: Actual hot-reload state preservation happens at the Iced application
/// level (mouse position, focus, etc.). This test verifies that the style
/// resolution layer doesn't interfere with state preservation.
#[test]
fn test_hot_reload_preserves_state() {
    // Arrange: Create initial style class (before hot-reload)
    let mut state_variants_before = HashMap::new();
    state_variants_before.insert(
        WidgetState::Hover,
        StyleProperties {
            background: Some(bg_color(0.5, 0.7, 1.0)), // Light blue (hover)
            border: Some(border(2.0, 0.3, 0.6, 1.0)),  // Blue border
            ..Default::default()
        },
    );

    let style_class_before = StyleClass {
        name: "button".to_string(),
        style: StyleProperties {
            background: Some(bg_color(0.2, 0.4, 0.8)), // Dark blue (base)
            border: Some(border(1.0, 0.5, 0.5, 0.5)),  // Gray border
            ..Default::default()
        },
        layout: None,
        extends: vec![],
        state_variants: state_variants_before.clone(),
        combined_state_variants: HashMap::new(),
    };

    // Act 1: Resolve hover state before hot-reload
    let hover_before =
        dampen_iced::style_mapping::resolve_state_style(&style_class_before, WidgetState::Hover);
    assert!(hover_before.is_some(), "Hover state should resolve before");

    // Simulate hot-reload: Create "new" style class with same structure
    // (In real hot-reload, this comes from re-parsing XML)
    let mut state_variants_after = HashMap::new();
    state_variants_after.insert(
        WidgetState::Hover,
        StyleProperties {
            background: Some(bg_color(0.5, 0.7, 1.0)), // Same light blue (hover)
            border: Some(border(2.0, 0.3, 0.6, 1.0)),  // Same blue border
            ..Default::default()
        },
    );

    let style_class_after = StyleClass {
        name: "button".to_string(),
        style: StyleProperties {
            background: Some(bg_color(0.2, 0.4, 0.8)), // Same dark blue (base)
            border: Some(border(1.0, 0.5, 0.5, 0.5)),  // Same gray border
            ..Default::default()
        },
        layout: None,
        extends: vec![],
        state_variants: state_variants_after,
        combined_state_variants: HashMap::new(),
    };

    // Act 2: Resolve hover state after hot-reload
    let hover_after =
        dampen_iced::style_mapping::resolve_state_style(&style_class_after, WidgetState::Hover);
    assert!(hover_after.is_some(), "Hover state should resolve after");

    // Assert: Hover styles should be identical before and after reload
    let hover_before = hover_before.unwrap();
    let hover_after = hover_after.unwrap();

    // Compare backgrounds
    assert_eq!(
        hover_before.background, hover_after.background,
        "Hover background should be preserved after hot-reload"
    );

    // Compare borders
    assert_eq!(
        hover_before.border, hover_after.border,
        "Hover border should be preserved after hot-reload"
    );

    // Verify the hover style is still correct (light blue)
    if let Some(Background::Color(hover_color)) = &hover_after.background {
        assert_eq!(
            hover_color.r, 0.5,
            "Hover background should still be light blue after reload"
        );
        assert_eq!(hover_color.g, 0.7);
        assert_eq!(hover_color.b, 1.0);
    } else {
        panic!("Hover style should have background color after reload");
    }

    // Verify border is still correct
    if let Some(hover_border) = &hover_after.border {
        assert_eq!(
            hover_border.width, 2.0,
            "Hover border should still be 2px after reload"
        );
        assert_eq!(
            hover_border.color.r, 0.3,
            "Hover border should still be blue after reload"
        );
    } else {
        panic!("Hover style should have border after reload");
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

// ============================================================================
// CHECKBOX/RADIO/TOGGLER STATE STYLING TESTS (Phase 5)
// ============================================================================

/// Test checkbox hover styling
///
/// Given: A checkbox with hover state styles defined
/// When: Checkbox is hovered
/// Then: Hover styles should be applied (e.g., background highlight)
#[test]
fn test_checkbox_hover_styling() {
    // Arrange: Create a style class with base and hover styles
    let mut state_variants = HashMap::new();
    state_variants.insert(
        WidgetState::Hover,
        StyleProperties {
            background: Some(bg_color(0.9, 0.95, 1.0)), // Light blue tint (hover)
            ..Default::default()
        },
    );

    let style_class = StyleClass {
        name: "checkbox".to_string(),
        style: StyleProperties {
            background: Some(bg_color(1.0, 1.0, 1.0)), // White (base)
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
        assert_eq!(hover_color.r, 0.9, "Hover background should be light blue");
        assert_eq!(hover_color.g, 0.95);
        assert_eq!(hover_color.b, 1.0);
    } else {
        panic!("Hover style should have background color");
    }
}

/// Test checkbox disabled styling
///
/// Given: A checkbox with disabled state styles defined
/// When: Checkbox is disabled
/// Then: Disabled styles should be applied (grayed out)
#[test]
fn test_checkbox_disabled_styling() {
    // Arrange: Create a style class with disabled styles
    let mut state_variants = HashMap::new();
    state_variants.insert(
        WidgetState::Disabled,
        StyleProperties {
            background: Some(bg_color(0.85, 0.85, 0.85)), // Light gray (disabled)
            color: Some(color(0.6, 0.6, 0.6)),            // Gray text
            ..Default::default()
        },
    );

    let style_class = StyleClass {
        name: "checkbox".to_string(),
        style: StyleProperties {
            background: Some(bg_color(1.0, 1.0, 1.0)), // White (base)
            color: Some(color(0.0, 0.0, 0.0)),         // Black text
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
        assert_eq!(disabled_bg.r, 0.85, "Disabled background should be gray");
        assert_eq!(disabled_bg.g, 0.85);
        assert_eq!(disabled_bg.b, 0.85);
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
}

/// Test radio button hover styling
///
/// Given: A radio button with hover state styles defined
/// When: Radio button is hovered
/// Then: Hover styles should be applied (e.g., subtle highlight)
#[test]
fn test_radio_hover_styling() {
    // Arrange: Create a style class with base and hover styles
    let mut state_variants = HashMap::new();
    state_variants.insert(
        WidgetState::Hover,
        StyleProperties {
            border: Some(border(2.0, 0.3, 0.6, 1.0)), // Blue border on hover
            ..Default::default()
        },
    );

    let style_class = StyleClass {
        name: "radio".to_string(),
        style: StyleProperties {
            border: Some(border(1.0, 0.5, 0.5, 0.5)), // Gray border (base)
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
    if let Some(hover_border) = &hover_style.border {
        assert_eq!(hover_border.width, 2.0, "Hover border should be 2px");
        assert_eq!(hover_border.color.r, 0.3, "Hover border should be blue");
        assert_eq!(hover_border.color.g, 0.6);
        assert_eq!(hover_border.color.b, 1.0);
    } else {
        panic!("Hover style should have border");
    }
}

/// Test toggler hover styling
///
/// Given: A toggler with hover state styles defined
/// When: Toggler is hovered
/// Then: Hover styles should be applied (e.g., track highlight)
#[test]
fn test_toggler_hover_styling() {
    // Arrange: Create a style class with base and hover styles
    let mut state_variants = HashMap::new();
    state_variants.insert(
        WidgetState::Hover,
        StyleProperties {
            background: Some(bg_color(0.85, 0.9, 1.0)), // Light blue tint (hover)
            ..Default::default()
        },
    );

    let style_class = StyleClass {
        name: "toggler".to_string(),
        style: StyleProperties {
            background: Some(bg_color(0.8, 0.8, 0.8)), // Gray (base)
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
        assert_eq!(hover_color.r, 0.85, "Hover background should be light blue");
        assert_eq!(hover_color.g, 0.9);
        assert_eq!(hover_color.b, 1.0);
    } else {
        panic!("Hover style should have background color");
    }
}

/// Test toggler disabled styling
///
/// Given: A toggler with disabled state styles defined
/// When: Toggler is disabled
/// Then: Disabled styles should be applied (grayed out, no interaction)
#[test]
fn test_toggler_disabled_styling() {
    // Arrange: Create a style class with disabled styles
    let mut state_variants = HashMap::new();
    state_variants.insert(
        WidgetState::Disabled,
        StyleProperties {
            background: Some(bg_color(0.7, 0.7, 0.7)), // Gray (disabled)
            ..Default::default()
        },
    );

    let style_class = StyleClass {
        name: "toggler".to_string(),
        style: StyleProperties {
            background: Some(bg_color(0.3, 0.7, 0.3)), // Green (base, active state)
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
        assert_eq!(disabled_bg.r, 0.7, "Disabled background should be gray");
        assert_eq!(disabled_bg.g, 0.7);
        assert_eq!(disabled_bg.b, 0.7);
    } else {
        panic!("Disabled style should have background color");
    }
}
