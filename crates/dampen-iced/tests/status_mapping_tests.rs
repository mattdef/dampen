//! Unit tests for widget status mapping and state style resolution
//!
//! Tests for:
//! - resolve_state_style: Looking up state-specific styles
//! - merge_style_properties: Merging state overrides with base styles
//! - map_button_status: Mapping Iced button status to WidgetState
//! - map_text_input_status: Mapping Iced text_input status to WidgetState

use dampen_core::ir::style::{Background, Color, StyleProperties};
use dampen_core::ir::theme::{StyleClass, WidgetState};
use dampen_iced::style_mapping::{
    map_button_status, map_checkbox_status, map_radio_status, map_text_input_status,
    map_toggler_status, merge_style_properties, resolve_state_style,
};
use std::collections::HashMap;

// Helper function to create a test color
fn test_color(r: f32, g: f32, b: f32, a: f32) -> Color {
    Color { r, g, b, a }
}

// Helper function to create a simple background color
fn test_background(r: f32, g: f32, b: f32) -> Background {
    Background::Color(test_color(r, g, b, 1.0))
}

#[test]
fn test_resolve_state_style_with_hover() {
    // Create a style class with hover state
    let mut state_variants = HashMap::new();
    state_variants.insert(
        WidgetState::Hover,
        StyleProperties {
            background: Some(test_background(0.5, 0.7, 1.0)), // Light blue
            ..Default::default()
        },
    );

    let style_class = StyleClass {
        name: "button".to_string(),
        style: StyleProperties {
            background: Some(test_background(0.2, 0.4, 0.8)), // Dark blue
            ..Default::default()
        },
        layout: None,
        extends: vec![],
        state_variants,
        combined_state_variants: HashMap::new(),
    };

    // Resolve hover state - should return the hover style
    let hover_style = resolve_state_style(&style_class, WidgetState::Hover);
    assert!(hover_style.is_some(), "Hover style should exist");

    let hover_style = hover_style.unwrap();
    if let Some(Background::Color(color)) = &hover_style.background {
        assert_eq!(color.r, 0.5);
        assert_eq!(color.g, 0.7);
        assert_eq!(color.b, 1.0);
    } else {
        panic!("Expected color background");
    }
}

#[test]
fn test_resolve_state_style_missing_state() {
    // Create a style class with only hover state
    let mut state_variants = HashMap::new();
    state_variants.insert(
        WidgetState::Hover,
        StyleProperties {
            background: Some(test_background(0.5, 0.7, 1.0)),
            ..Default::default()
        },
    );

    let style_class = StyleClass {
        name: "button".to_string(),
        style: StyleProperties {
            background: Some(test_background(0.2, 0.4, 0.8)),
            ..Default::default()
        },
        layout: None,
        extends: vec![],
        state_variants,
        combined_state_variants: HashMap::new(),
    };

    // Try to resolve disabled state - should return None (fallback to base)
    let disabled_style = resolve_state_style(&style_class, WidgetState::Disabled);
    assert!(
        disabled_style.is_none(),
        "Disabled style should not exist, fallback to base"
    );
}

#[test]
fn test_resolve_state_style_all_states() {
    // Create a style class with all 4 states
    let mut state_variants = HashMap::new();
    state_variants.insert(
        WidgetState::Hover,
        StyleProperties {
            background: Some(test_background(0.5, 0.7, 1.0)),
            ..Default::default()
        },
    );
    state_variants.insert(
        WidgetState::Active,
        StyleProperties {
            background: Some(test_background(0.3, 0.5, 0.9)),
            ..Default::default()
        },
    );
    state_variants.insert(
        WidgetState::Focus,
        StyleProperties {
            background: Some(test_background(0.4, 0.6, 0.95)),
            ..Default::default()
        },
    );
    state_variants.insert(
        WidgetState::Disabled,
        StyleProperties {
            background: Some(test_background(0.6, 0.6, 0.6)),
            ..Default::default()
        },
    );

    let style_class = StyleClass {
        name: "button".to_string(),
        style: StyleProperties {
            background: Some(test_background(0.2, 0.4, 0.8)),
            ..Default::default()
        },
        layout: None,
        extends: vec![],
        state_variants,
        combined_state_variants: HashMap::new(),
    };

    // Verify each state resolves correctly
    assert!(resolve_state_style(&style_class, WidgetState::Hover).is_some());
    assert!(resolve_state_style(&style_class, WidgetState::Active).is_some());
    assert!(resolve_state_style(&style_class, WidgetState::Focus).is_some());
    assert!(resolve_state_style(&style_class, WidgetState::Disabled).is_some());
}

#[test]
fn test_merge_style_properties_background_override() {
    let base = StyleProperties {
        background: Some(test_background(0.2, 0.4, 0.8)), // Dark blue
        color: Some(test_color(1.0, 1.0, 1.0, 1.0)),      // White text
        ..Default::default()
    };

    let hover_override = StyleProperties {
        background: Some(test_background(0.5, 0.7, 1.0)), // Light blue
        ..Default::default()
    };

    let merged = merge_style_properties(&base, &hover_override);

    // Background should be overridden
    if let Some(Background::Color(color)) = &merged.background {
        assert_eq!(color.r, 0.5, "Background should be hover color");
        assert_eq!(color.g, 0.7);
        assert_eq!(color.b, 1.0);
    } else {
        panic!("Expected color background");
    }

    // Color should remain from base (not overridden)
    if let Some(color) = merged.color {
        assert_eq!(color.r, 1.0, "Text color should remain white from base");
        assert_eq!(color.g, 1.0);
        assert_eq!(color.b, 1.0);
    } else {
        panic!("Expected text color from base");
    }
}

#[test]
fn test_merge_style_properties_partial_override() {
    let base = StyleProperties {
        background: Some(test_background(0.2, 0.4, 0.8)),
        color: Some(test_color(1.0, 1.0, 1.0, 1.0)),
        opacity: Some(1.0),
        ..Default::default()
    };

    let state_override = StyleProperties {
        background: Some(test_background(0.5, 0.7, 1.0)), // Override background
        opacity: Some(0.8),                               // Override opacity
        ..Default::default()                              // Leave color unchanged
    };

    let merged = merge_style_properties(&base, &state_override);

    // Background overridden
    if let Some(Background::Color(color)) = &merged.background {
        assert_eq!(color.r, 0.5);
    } else {
        panic!("Expected background");
    }

    // Color from base
    if let Some(color) = merged.color {
        assert_eq!(color.r, 1.0);
    } else {
        panic!("Expected color from base");
    }

    // Opacity overridden
    assert_eq!(merged.opacity, Some(0.8), "Opacity should be overridden");
}

#[test]
fn test_merge_style_properties_empty_override() {
    let base = StyleProperties {
        background: Some(test_background(0.2, 0.4, 0.8)),
        color: Some(test_color(1.0, 1.0, 1.0, 1.0)),
        opacity: Some(1.0),
        ..Default::default()
    };

    let empty_override = StyleProperties::default();

    let merged = merge_style_properties(&base, &empty_override);

    // All properties should come from base
    assert!(
        merged.background.is_some(),
        "Background should be from base"
    );
    assert!(merged.color.is_some(), "Color should be from base");
    assert_eq!(merged.opacity, Some(1.0), "Opacity should be from base");
}

#[test]
fn test_merge_style_properties_full_override() {
    let base = StyleProperties {
        background: Some(test_background(0.2, 0.4, 0.8)),
        color: Some(test_color(1.0, 1.0, 1.0, 1.0)),
        opacity: Some(1.0),
        ..Default::default()
    };

    let full_override = StyleProperties {
        background: Some(test_background(0.5, 0.7, 1.0)),
        color: Some(test_color(0.0, 0.0, 0.0, 1.0)), // Black
        opacity: Some(0.5),
        ..Default::default()
    };

    let merged = merge_style_properties(&base, &full_override);

    // All properties should come from override
    if let Some(Background::Color(color)) = &merged.background {
        assert_eq!(color.r, 0.5, "Background should be from override");
    }

    if let Some(color) = merged.color {
        assert_eq!(color.r, 0.0, "Color should be black from override");
    }

    assert_eq!(merged.opacity, Some(0.5), "Opacity should be from override");
}

//
// Button Status Mapping Tests
//

#[test]
fn test_map_button_status_active() {
    use iced::widget::button::Status;

    // Active status (not hovered, not pressed) should map to None (use base style)
    let result = map_button_status(Status::Active);
    assert_eq!(
        result, None,
        "Active (default) status should return None for base style"
    );
}

#[test]
fn test_map_button_status_hovered() {
    use iced::widget::button::Status;

    // Hovered status should map to WidgetState::Hover
    let result = map_button_status(Status::Hovered);
    assert_eq!(
        result,
        Some(WidgetState::Hover),
        "Hovered status should map to Hover state"
    );
}

#[test]
fn test_map_button_status_pressed() {
    use iced::widget::button::Status;

    // Pressed status should map to WidgetState::Active
    let result = map_button_status(Status::Pressed);
    assert_eq!(
        result,
        Some(WidgetState::Active),
        "Pressed status should map to Active state"
    );
}

#[test]
fn test_map_button_status_disabled() {
    use iced::widget::button::Status;

    // Disabled status should map to WidgetState::Disabled
    let result = map_button_status(Status::Disabled);
    assert_eq!(
        result,
        Some(WidgetState::Disabled),
        "Disabled status should map to Disabled state"
    );
}

// ============================================================================
// TEXT INPUT STATUS MAPPING TESTS
// ============================================================================

#[test]
fn test_map_text_input_status_active() {
    use iced::widget::text_input::Status;

    // Active status should map to None (use base style)
    let result = map_text_input_status(Status::Active);
    assert_eq!(
        result, None,
        "Active status should map to None (base style)"
    );
}

#[test]
fn test_map_text_input_status_hovered() {
    use iced::widget::text_input::Status;

    // Hovered status should map to WidgetState::Hover
    let result = map_text_input_status(Status::Hovered);
    assert_eq!(
        result,
        Some(WidgetState::Hover),
        "Hovered status should map to Hover state"
    );
}

#[test]
fn test_map_text_input_status_focused() {
    use iced::widget::text_input::Status;

    // Focused status should map to WidgetState::Focus
    // Status::Focused is a struct variant with is_hovered field
    let result = map_text_input_status(Status::Focused { is_hovered: false });
    assert_eq!(
        result,
        Some(WidgetState::Focus),
        "Focused status should map to Focus state"
    );

    // Test with is_hovered=true as well
    let result_hovered = map_text_input_status(Status::Focused { is_hovered: true });
    assert_eq!(
        result_hovered,
        Some(WidgetState::Focus),
        "Focused (hovered) status should map to Focus state"
    );
}

#[test]
fn test_map_text_input_status_disabled() {
    use iced::widget::text_input::Status;

    // Disabled status should map to WidgetState::Disabled
    let result = map_text_input_status(Status::Disabled);
    assert_eq!(
        result,
        Some(WidgetState::Disabled),
        "Disabled status should map to Disabled state"
    );
}

// ============================================================================
// CHECKBOX STATUS MAPPING TESTS
// ============================================================================

#[test]
fn test_map_checkbox_status_active() {
    use iced::widget::checkbox::Status;

    // Active status should map to None (use base style) - checked
    let result = map_checkbox_status(Status::Active { is_checked: true });
    assert_eq!(
        result, None,
        "Active (checked) status should map to None (base style)"
    );

    // Active status should map to None - unchecked
    let result_unchecked = map_checkbox_status(Status::Active { is_checked: false });
    assert_eq!(
        result_unchecked, None,
        "Active (unchecked) status should map to None (base style)"
    );
}

#[test]
fn test_map_checkbox_status_hovered() {
    use iced::widget::checkbox::Status;

    // Hovered status should map to WidgetState::Hover - checked
    let result = map_checkbox_status(Status::Hovered { is_checked: true });
    assert_eq!(
        result,
        Some(WidgetState::Hover),
        "Hovered (checked) status should map to Hover state"
    );

    // Hovered status should map to WidgetState::Hover - unchecked
    let result_unchecked = map_checkbox_status(Status::Hovered { is_checked: false });
    assert_eq!(
        result_unchecked,
        Some(WidgetState::Hover),
        "Hovered (unchecked) status should map to Hover state"
    );
}

#[test]
fn test_map_checkbox_status_disabled() {
    use iced::widget::checkbox::Status;

    // Disabled status should map to WidgetState::Disabled - checked
    let result = map_checkbox_status(Status::Disabled { is_checked: true });
    assert_eq!(
        result,
        Some(WidgetState::Disabled),
        "Disabled (checked) status should map to Disabled state"
    );

    // Disabled status should map to WidgetState::Disabled - unchecked
    let result_unchecked = map_checkbox_status(Status::Disabled { is_checked: false });
    assert_eq!(
        result_unchecked,
        Some(WidgetState::Disabled),
        "Disabled (unchecked) status should map to Disabled state"
    );
}

// ============================================================================
// RADIO STATUS MAPPING TESTS
// ============================================================================

#[test]
fn test_map_radio_status_active() {
    use iced::widget::radio::Status;

    // Active status should map to None (use base style) - selected
    let result = map_radio_status(Status::Active { is_selected: true });
    assert_eq!(
        result, None,
        "Active (selected) status should map to None (base style)"
    );

    // Active status should map to None - unselected
    let result_unselected = map_radio_status(Status::Active { is_selected: false });
    assert_eq!(
        result_unselected, None,
        "Active (unselected) status should map to None (base style)"
    );
}

#[test]
fn test_map_radio_status_hovered() {
    use iced::widget::radio::Status;

    // Hovered status should map to WidgetState::Hover - selected
    let result = map_radio_status(Status::Hovered { is_selected: true });
    assert_eq!(
        result,
        Some(WidgetState::Hover),
        "Hovered (selected) status should map to Hover state"
    );

    // Hovered status should map to WidgetState::Hover - unselected
    let result_unselected = map_radio_status(Status::Hovered { is_selected: false });
    assert_eq!(
        result_unselected,
        Some(WidgetState::Hover),
        "Hovered (unselected) status should map to Hover state"
    );
}

// ============================================================================
// TOGGLER STATUS MAPPING TESTS
// ============================================================================

#[test]
fn test_map_toggler_status_active() {
    use iced::widget::toggler::Status;

    // Active status should map to None (use base style) - toggled on
    let result = map_toggler_status(Status::Active { is_toggled: true });
    assert_eq!(
        result, None,
        "Active (on) status should map to None (base style)"
    );

    // Active status should map to None - toggled off
    let result_off = map_toggler_status(Status::Active { is_toggled: false });
    assert_eq!(
        result_off, None,
        "Active (off) status should map to None (base style)"
    );
}

#[test]
fn test_map_toggler_status_hovered() {
    use iced::widget::toggler::Status;

    // Hovered status should map to WidgetState::Hover - toggled on
    let result = map_toggler_status(Status::Hovered { is_toggled: true });
    assert_eq!(
        result,
        Some(WidgetState::Hover),
        "Hovered (on) status should map to Hover state"
    );

    // Hovered status should map to WidgetState::Hover - toggled off
    let result_off = map_toggler_status(Status::Hovered { is_toggled: false });
    assert_eq!(
        result_off,
        Some(WidgetState::Hover),
        "Hovered (off) status should map to Hover state"
    );
}

#[test]
fn test_map_toggler_status_disabled() {
    use iced::widget::toggler::Status;

    // Disabled status should map to WidgetState::Disabled - toggled on
    let result = map_toggler_status(Status::Disabled { is_toggled: true });
    assert_eq!(
        result,
        Some(WidgetState::Disabled),
        "Disabled (on) status should map to Disabled state"
    );

    // Disabled status should map to WidgetState::Disabled - toggled off
    let result_off = map_toggler_status(Status::Disabled { is_toggled: false });
    assert_eq!(
        result_off,
        Some(WidgetState::Disabled),
        "Disabled (off) status should map to Disabled state"
    );
}
