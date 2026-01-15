//! Style mapping from Dampen IR to Iced types
//!
//! This module maps Dampen's backend-agnostic style types to Iced-specific
//! style types, including widget state-specific styling.

use dampen_core::ir::layout::{Alignment, Justification, LayoutConstraints, Length, Position};
use dampen_core::ir::style::{Background, Color, StyleProperties};
use dampen_core::ir::theme::{StyleClass, WidgetState};
use iced::border::Radius;

/// Resolve state-specific style properties from a style class
///
/// Returns the style override for the given widget state if defined,
/// otherwise returns None to indicate the base style should be used.
///
/// # Arguments
/// * `style_class` - The style class containing base and state-specific styles
/// * `state` - The current widget state (Hover, Focus, Active, Disabled)
///
/// # Example
/// ```ignore
/// let style_class = /* ... StyleClass with hover styles ... */;
/// if let Some(hover_style) = resolve_state_style(&style_class, WidgetState::Hover) {
///     // Use hover-specific styles
/// } else {
///     // Use base styles
/// }
/// ```
pub fn resolve_state_style(
    style_class: &StyleClass,
    state: WidgetState,
) -> Option<&StyleProperties> {
    style_class.state_variants.get(&state)
}

/// Merge state-specific style properties with base style properties
///
/// Returns a new StyleProperties where state-specific overrides take precedence
/// over base styles. Any property not overridden in the state style retains
/// the base value.
///
/// # Arguments
/// * `base` - The base style properties
/// * `state_override` - State-specific style properties to merge
///
/// # Example
/// ```ignore
/// let base = StyleProperties { background: Some(blue), color: Some(white), .. };
/// let hover = StyleProperties { background: Some(light_blue), .. };
/// let merged = merge_style_properties(&base, &hover);
/// // Result: background=light_blue (from hover), color=white (from base)
/// ```
pub fn merge_style_properties(
    base: &StyleProperties,
    state_override: &StyleProperties,
) -> StyleProperties {
    StyleProperties {
        // State override takes precedence if set
        background: state_override
            .background
            .clone()
            .or_else(|| base.background.clone()),
        color: state_override.color.or(base.color),
        border: state_override
            .border
            .clone()
            .or_else(|| base.border.clone()),
        shadow: state_override.shadow.or(base.shadow),
        opacity: state_override.opacity.or(base.opacity),
        transform: state_override
            .transform
            .clone()
            .or_else(|| base.transform.clone()),
    }
}

//
// Widget Status Mapping Functions
//
// These functions map Iced widget-specific status enums to Dampen's unified WidgetState enum.
// This abstraction layer allows backend-agnostic state styling in the IR.
//

/// Map button status to unified widget state
///
/// Maps Iced's `button::Status` enum to Dampen's `WidgetState`.
/// Returns `None` for the default/active state (which should use base styles).
///
/// # Mapping
/// - `button::Status::Active` → `None` (default/resting state, use base style)
/// - `button::Status::Hovered` → `Some(WidgetState::Hover)` (mouse over)
/// - `button::Status::Pressed` → `Some(WidgetState::Active)` (mouse button down)
/// - `button::Status::Disabled` → `Some(WidgetState::Disabled)` (not interactive)
///
/// # Example
/// ```ignore
/// use iced::widget::button;
/// use dampen_iced::style_mapping::map_button_status;
///
/// let state = map_button_status(button::Status::Hovered);
/// assert_eq!(state, Some(WidgetState::Hover));
///
/// let base_state = map_button_status(button::Status::Active);
/// assert_eq!(base_state, None); // Use base style
/// ```
pub fn map_button_status(status: iced::widget::button::Status) -> Option<WidgetState> {
    use iced::widget::button::Status;
    match status {
        Status::Active => None,                          // Base/default state
        Status::Hovered => Some(WidgetState::Hover),     // Mouse over button
        Status::Pressed => Some(WidgetState::Active),    // Mouse button pressed
        Status::Disabled => Some(WidgetState::Disabled), // Button disabled
    }
}

/// Map text input status to unified widget state
///
/// Maps Iced's `text_input::Status` enum to Dampen's `WidgetState`.
/// Returns `None` for the default/active state (which should use base styles).
///
/// # Mapping
/// - `text_input::Status::Active` → `None` (default/resting state, use base style)
/// - `text_input::Status::Hovered` → `Some(WidgetState::Hover)` (mouse over)
/// - `text_input::Status::Focused` → `Some(WidgetState::Focus)` (has keyboard focus)
/// - `text_input::Status::Disabled` → `Some(WidgetState::Disabled)` (not interactive)
///
/// # Example
/// ```ignore
/// use iced::widget::text_input;
/// use dampen_iced::style_mapping::map_text_input_status;
///
/// let state = map_text_input_status(text_input::Status::Focused);
/// assert_eq!(state, Some(WidgetState::Focus));
///
/// let base_state = map_text_input_status(text_input::Status::Active);
/// assert_eq!(base_state, None); // Use base style
/// ```
pub fn map_text_input_status(status: iced::widget::text_input::Status) -> Option<WidgetState> {
    use iced::widget::text_input::Status;
    match status {
        Status::Active => None,                             // Base/default state
        Status::Hovered => Some(WidgetState::Hover),        // Mouse over input
        Status::Focused { .. } => Some(WidgetState::Focus), // Input has keyboard focus (ignore fields)
        Status::Disabled => Some(WidgetState::Disabled),    // Input disabled
    }
}

/// Map checkbox status to unified widget state
///
/// Maps Iced's `checkbox::Status` enum to Dampen's `WidgetState`.
/// Returns `None` for the default/active state (which should use base styles).
///
/// # Mapping
/// - `checkbox::Status::Active { .. }` → `None` (default/resting state, use base style)
/// - `checkbox::Status::Hovered { .. }` → `Some(WidgetState::Hover)` (mouse over)
/// - `checkbox::Status::Disabled { .. }` → `Some(WidgetState::Disabled)` (not interactive)
///
/// # Note
/// Each status variant includes `is_checked: bool` context, which is ignored for state mapping.
/// The `is_checked` state should be handled separately in the style closure if needed.
///
/// # Example
/// ```ignore
/// use iced::widget::checkbox;
/// use dampen_iced::style_mapping::map_checkbox_status;
///
/// let state = map_checkbox_status(checkbox::Status::Hovered { is_checked: true });
/// assert_eq!(state, Some(WidgetState::Hover));
///
/// let base_state = map_checkbox_status(checkbox::Status::Active { is_checked: false });
/// assert_eq!(base_state, None); // Use base style
/// ```
pub fn map_checkbox_status(status: iced::widget::checkbox::Status) -> Option<WidgetState> {
    use iced::widget::checkbox::Status;
    match status {
        Status::Active { .. } => None, // Base/default state
        Status::Hovered { .. } => Some(WidgetState::Hover), // Mouse over checkbox
        Status::Disabled { .. } => Some(WidgetState::Disabled), // Checkbox disabled
    }
}

/// Map radio button status to unified widget state
///
/// Maps Iced's `radio::Status` enum to Dampen's `WidgetState`.
/// Returns `None` for the default/active state (which should use base styles).
///
/// # Mapping
/// - `radio::Status::Active { .. }` → `None` (default/resting state, use base style)
/// - `radio::Status::Hovered { .. }` → `Some(WidgetState::Hover)` (mouse over)
///
/// # Note
/// - Each status variant includes `is_selected: bool` context, which is ignored for state mapping.
/// - Radio buttons don't have a built-in `Disabled` status in Iced 0.14.
/// - For disabled styling, check the `disabled` attribute in the builder and manually apply `WidgetState::Disabled`.
///
/// # Example
/// ```ignore
/// use iced::widget::radio;
/// use dampen_iced::style_mapping::map_radio_status;
///
/// let state = map_radio_status(radio::Status::Hovered { is_selected: false });
/// assert_eq!(state, Some(WidgetState::Hover));
///
/// let base_state = map_radio_status(radio::Status::Active { is_selected: true });
/// assert_eq!(base_state, None); // Use base style
/// ```
pub fn map_radio_status(status: iced::widget::radio::Status) -> Option<WidgetState> {
    use iced::widget::radio::Status;
    match status {
        Status::Active { .. } => None, // Base/default state
        Status::Hovered { .. } => Some(WidgetState::Hover), // Mouse over radio
    }
}

/// Map toggler status to unified widget state
///
/// Maps Iced's `toggler::Status` enum to Dampen's `WidgetState`.
/// Returns `None` for the default/active state (which should use base styles).
///
/// # Mapping
/// - `toggler::Status::Active { .. }` → `None` (default/resting state, use base style)
/// - `toggler::Status::Hovered { .. }` → `Some(WidgetState::Hover)` (mouse over)
/// - `toggler::Status::Disabled { .. }` → `Some(WidgetState::Disabled)` (not interactive)
///
/// # Note
/// Each status variant includes `is_toggled: bool` context, which is ignored for state mapping.
/// The `is_toggled` state should be handled separately in the style closure if needed.
///
/// # Example
/// ```ignore
/// use iced::widget::toggler;
/// use dampen_iced::style_mapping::map_toggler_status;
///
/// let state = map_toggler_status(toggler::Status::Hovered { is_toggled: true });
/// assert_eq!(state, Some(WidgetState::Hover));
///
/// let base_state = map_toggler_status(toggler::Status::Active { is_toggled: false });
/// assert_eq!(base_state, None); // Use base style
/// ```
pub fn map_toggler_status(status: iced::widget::toggler::Status) -> Option<WidgetState> {
    use iced::widget::toggler::Status;
    match status {
        Status::Active { .. } => None, // Base/default state
        Status::Hovered { .. } => Some(WidgetState::Hover), // Mouse over toggler
        Status::Disabled { .. } => Some(WidgetState::Disabled), // Toggler disabled
    }
}

/// Map Iced slider status to Dampen WidgetState
///
/// Maps the Iced slider::Status enum to the appropriate Dampen WidgetState.
/// Returns None for the base/default state (Active).
///
/// **Note on Disabled State**: Iced 0.14's slider::Status does NOT have a Disabled variant.
/// Disabled state must be handled manually by checking the `disabled` attribute in the builder.
///
/// # Arguments
/// * `status` - The Iced slider status
/// * `is_disabled` - Manual disabled flag from attribute evaluation
///
/// # Returns
/// * `Some(WidgetState::Hover)` - Mouse over slider
/// * `Some(WidgetState::Active)` - User dragging slider thumb
/// * `Some(WidgetState::Disabled)` - Slider is disabled (from manual check)
/// * `None` - Base/default state (slider active but not hovered/dragged)
///
/// # Example
/// ```
/// use dampen_iced::style_mapping::map_slider_status;
/// use iced::widget::slider;
///
/// let state = map_slider_status(slider::Status::Hovered, false);
/// assert_eq!(state, Some(dampen_core::ir::WidgetState::Hover));
///
/// let drag_state = map_slider_status(slider::Status::Dragged, false);
/// assert_eq!(drag_state, Some(dampen_core::ir::WidgetState::Active));
///
/// let disabled_state = map_slider_status(slider::Status::Active, true);
/// assert_eq!(disabled_state, Some(dampen_core::ir::WidgetState::Disabled));
/// ```
pub fn map_slider_status(
    status: iced::widget::slider::Status,
    is_disabled: bool,
) -> Option<WidgetState> {
    // Check disabled first (manual handling since Iced doesn't have Status::Disabled)
    if is_disabled {
        return Some(WidgetState::Disabled);
    }

    use iced::widget::slider::Status;
    match status {
        Status::Active => None,                       // Base/default state
        Status::Hovered => Some(WidgetState::Hover),  // Mouse over slider
        Status::Dragged => Some(WidgetState::Active), // Dragging slider thumb
    }
}

/// Map Iced pick_list status to Dampen WidgetState
///
/// Maps the Iced pick_list::Status enum to the appropriate Dampen WidgetState.
/// Returns None for the base/default state (Active).
///
/// The `Opened` status (dropdown menu open) is mapped to `WidgetState::Focus`
/// since it represents active user interaction with the widget.
///
/// # Arguments
/// * `status` - The Iced pick_list status
///
/// # Returns
/// * `Some(WidgetState::Hover)` - Mouse over picklist
/// * `Some(WidgetState::Focus)` - Dropdown menu is open
/// * `None` - Base/default state (dropdown closed, not hovered)
///
/// # Example
/// ```
/// use dampen_iced::style_mapping::map_picklist_status;
/// use iced::widget::pick_list;
///
/// let state = map_picklist_status(pick_list::Status::Hovered);
/// assert_eq!(state, Some(dampen_core::ir::WidgetState::Hover));
///
/// let opened_state = map_picklist_status(pick_list::Status::Opened { is_hovered: false });
/// assert_eq!(opened_state, Some(dampen_core::ir::WidgetState::Focus));
/// ```
pub fn map_picklist_status(status: iced::widget::pick_list::Status) -> Option<WidgetState> {
    use iced::widget::pick_list::Status;
    match status {
        Status::Active => None,                      // Base/default state
        Status::Hovered => Some(WidgetState::Hover), // Mouse over picklist
        Status::Opened { is_hovered: _ } => Some(WidgetState::Focus), // Dropdown menu open (treat as focused)
    }
}

// ============================================================================
// ComboBox Widget Status Mapping - Uses TextInput Status
// ============================================================================
//
// ℹ️ ICED 0.14 API NOTE: ComboBox does not have its own Status enum
//
// ## Implementation Detail
//
// In Iced 0.14, the `combo_box` widget does not define its own `Status` enum.
// Instead, it reuses `text_input::Status` since a combo box is essentially
// a text input with a dropdown menu.
//
// The combo_box style closure signature is:
// ```text
// pub fn style<F>(self, f: F) -> Self
// where
//     F: 'a + Fn(&Theme, text_input::Status) -> text_input::Style
// ```
//
// ## Dampen Mapping
//
// - ComboBox state styling uses `map_text_input_status()`
// - States available: Active, Hovered, Focused, Disabled
// - No separate mapping function needed for ComboBox
//
// ## Future Builder Integration
//
// When implementing state-aware styling for ComboBox in the builder,
// use `map_text_input_status()` to convert the status to WidgetState:
//
// ```rust
// let style = move |_theme: &Theme, status: text_input::Status| {
//     let widget_state = map_text_input_status(status);
//     // ... resolve and apply state-specific styling
// };
// combo_box.style(style)
// ```
//
// See `build_combo_box()` in `builder.rs` for integration location.
//
// ============================================================================

// ============================================================================
// Container Widget Status Mapping - API Limitation
// ============================================================================
//
// ⚠️ ICED 0.14 API LIMITATION: Container hover styling not currently available
//
// ## Problem
//
// In Iced 0.14, the `container` widget's style closure signature is:
// ```text
// pub fn style<F>(self, f: F) -> Self
// where
//     F: 'a + Fn(&Theme) -> container::Style
// ```
//
// Unlike `button`, `text_input`, `checkbox`, `radio`, and `toggler` widgets,
// the container style closure does NOT receive a `Status` parameter.
// This means we cannot detect hover state in the style function.
//
// ## Impact on Dampen
//
// - Container widgets can receive base styles (background, border, etc.)
// - Container widgets CANNOT receive hover-specific styles via the standard approach
// - The `<hover>` state variant for containers in XML will be ignored
//
// ## Current Behavior
//
// The `DampenWidgetBuilder::build_container` method (via `apply_style_layout`)
// applies only the base style:
// ```text
// container = container.style(move |_theme| iced_style);
// //                                  ^^^ No status parameter!
// ```
//
// ## Workaround (Future Enhancement)
//
// To support container hover styling, we would need to:
//
// 1. Create a custom `HoverContainer` widget wrapper around Iced's container
// 2. Track mouse state manually using Iced's event system
// 3. Apply state-aware styling based on tracked mouse position
// 4. Integrate this wrapper into `DampenWidgetBuilder::build_container`
//
// This is tracked as a future enhancement (Phase 10+ or separate feature).
//
// ## Testing
//
// Integration tests for container hover styling are included but may be
// marked as `#[ignore]` or conditional until the API limitation is resolved
// or a custom wrapper is implemented.
//
// ## Related Issues
//
// - Spec: specs/003-widget-state-styling/spec.md (User Story 4)
// - Tasks: specs/003-widget-state-styling/tasks.md (Phase 6, T044-T052)
// - Research: specs/003-widget-state-styling/research.md (Container Widget section)
//
// ## Status Mapping Function (Placeholder)
//
// ```text
// // This function signature would be used IF Iced added Status parameter:
// pub fn map_container_status(status: container::Status) -> Option<WidgetState> {
//     use iced::widget::container::Status;
//     match status {
//         Status::Active => None,  // Base/default state
//         Status::Hovered => Some(WidgetState::Hover),  // Mouse over container
//     }
// }
// ```
//
// Note: The above code is HYPOTHETICAL and will NOT compile because
// `container::Status` is not publicly exported in Iced 0.14 and the style
// closure does not receive it.
// ============================================================================
// Color and Layout Mapping Functions
// ============================================================================

/// Map Color to Iced Color
pub fn map_color(color: &Color) -> iced::Color {
    iced::Color {
        r: color.r,
        g: color.g,
        b: color.b,
        a: color.a,
    }
}

/// Map LayoutConstraints to Iced layout properties
pub fn map_layout_constraints(layout: &LayoutConstraints) -> IcedLayout {
    IcedLayout {
        width: map_length(&layout.width),
        height: map_length(&layout.height),
        padding: map_padding(layout),
        align_items: layout.align_items.map(map_alignment),
        justify_content: layout.justify_content.map(map_justification),
        position: layout.position,
        top: layout.top,
        right: layout.right,
        bottom: layout.bottom,
        left: layout.left,
        z_index: layout.z_index,
    }
}

/// Map Length to Iced Length
pub fn map_length(length: &Option<Length>) -> iced::Length {
    match length {
        Some(Length::Fixed(pixels)) => iced::Length::Fixed(*pixels),
        Some(Length::Fill) => iced::Length::Fill,
        Some(Length::Shrink) => iced::Length::Shrink,
        Some(Length::FillPortion(n)) => iced::Length::FillPortion(*n as u16),
        Some(Length::Percentage(pct)) => {
            // Iced doesn't have percentage, use FillPortion as approximation
            // or Fixed if we know the parent size
            iced::Length::FillPortion((pct / 10.0) as u16)
        }
        None => iced::Length::Shrink,
    }
}

/// Map Padding to Iced Padding
pub fn map_padding(layout: &LayoutConstraints) -> iced::Padding {
    if let Some(padding) = &layout.padding {
        iced::Padding::new(padding.top)
            .left(padding.left)
            .right(padding.right)
            .bottom(padding.bottom)
    } else {
        iced::Padding::new(0.0)
    }
}

/// Map Alignment to Iced Alignment
pub fn map_alignment(alignment: Alignment) -> iced::Alignment {
    match alignment {
        Alignment::Start => iced::Alignment::Start,
        Alignment::Center => iced::Alignment::Center,
        Alignment::End => iced::Alignment::End,
        Alignment::Stretch => iced::Alignment::Center,
    }
}

/// Map Justification to Iced Alignment (for justify_content)
pub fn map_justification(justification: Justification) -> iced::Alignment {
    match justification {
        Justification::Start => iced::Alignment::Start,
        Justification::Center => iced::Alignment::Center,
        Justification::End => iced::Alignment::End,
        Justification::SpaceBetween => iced::Alignment::Center, // Iced doesn't have SpaceBetween
        Justification::SpaceAround => iced::Alignment::Center,  // Iced doesn't have SpaceAround
        Justification::SpaceEvenly => iced::Alignment::Center,  // Iced doesn't have SpaceEvenly
    }
}

/// Check if layout requires special positioning handling
///
/// Returns true if the layout has position offsets that need special handling
pub fn has_positioning(layout: &IcedLayout) -> bool {
    layout.position.is_some()
        || layout.top.is_some()
        || layout.right.is_some()
        || layout.bottom.is_some()
        || layout.left.is_some()
        || layout.z_index.is_some()
}

/// Get the z-index for layering widgets
///
/// Higher z-index values should be rendered on top
pub fn get_z_index(layout: &IcedLayout) -> i32 {
    layout.z_index.unwrap_or(0)
}

/// Map StyleProperties to Iced container Style
pub fn map_style_properties(style: &StyleProperties) -> iced::widget::container::Style {
    // Start with a transparent container (no background)
    let mut container_style = iced::widget::container::Style {
        background: None,
        text_color: None,
        border: iced::Border::default(),
        shadow: iced::Shadow::default(),
        snap: false,
    };

    // Map background - only if explicitly set
    if let Some(bg) = &style.background {
        container_style.background = Some(map_background(bg));
    }

    // Map text color - this affects all text widgets inside the container
    if let Some(color) = &style.color {
        container_style.text_color = Some(map_color(color));
    }

    // Map border
    if let Some(border) = &style.border {
        container_style.border = iced::Border {
            width: border.width,
            color: map_color(&border.color),
            radius: map_border_radius(&border.radius),
        };
    }

    // Map shadow
    if let Some(shadow) = &style.shadow {
        container_style.shadow = iced::Shadow {
            color: map_color(&shadow.color),
            offset: iced::Vector::new(shadow.offset_x, shadow.offset_y),
            blur_radius: shadow.blur_radius,
        };
    }

    // Opacity: Handled via color alpha channels in Iced 0.14
    // (e.g., background color with alpha, text color with alpha)
    // No direct opacity field in container::Style

    // Transform: Not supported in Iced 0.14 container::Style
    // Would require custom widget wrapper (Phase 10)

    container_style
}

/// Map Background to Iced Background
pub fn map_background(background: &Background) -> iced::Background {
    match background {
        Background::Color(color) => iced::Background::Color(map_color(color)),
        Background::Gradient(gradient) => iced::Background::Gradient(map_gradient(gradient)),
        Background::Image { path: _, fit: _ } => {
            // Iced doesn't have built-in image backgrounds
            // This would need special handling (e.g., loading image as texture)
            // For now, return transparent
            iced::Background::Color(iced::Color::TRANSPARENT)
        }
    }
}

/// Map Dampen Gradient to Iced Gradient
pub fn map_gradient(gradient: &dampen_core::ir::style::Gradient) -> iced::Gradient {
    match gradient {
        dampen_core::ir::style::Gradient::Linear { angle, stops } => {
            // Convert angle from degrees to radians
            let radians = angle * (std::f32::consts::PI / 180.0);

            // Create Iced Linear gradient
            let mut iced_linear = iced::gradient::Linear::new(iced::Radians(radians));

            // Add color stops (Iced supports up to 8)
            for stop in stops.iter().take(8) {
                iced_linear = iced_linear.add_stop(stop.offset, map_color(&stop.color));
            }

            iced::Gradient::Linear(iced_linear)
        }
        dampen_core::ir::style::Gradient::Radial { shape: _, stops } => {
            // Iced 0.14 only supports linear gradients
            // Convert to linear as fallback
            let radians = 0.0;
            let mut iced_linear = iced::gradient::Linear::new(iced::Radians(radians));

            for stop in stops.iter().take(8) {
                iced_linear = iced_linear.add_stop(stop.offset, map_color(&stop.color));
            }

            iced::Gradient::Linear(iced_linear)
        }
    }
}

/// Map BorderRadius to Iced Radius
pub fn map_border_radius(radius: &dampen_core::ir::style::BorderRadius) -> Radius {
    Radius::from(radius.top_left)
        .top_right(radius.top_right)
        .bottom_right(radius.bottom_right)
        .bottom_left(radius.bottom_left)
}

/// Struct to hold resolved Iced layout properties
pub struct IcedLayout {
    pub width: iced::Length,
    pub height: iced::Length,
    pub padding: iced::Padding,
    pub align_items: Option<iced::Alignment>,
    pub justify_content: Option<iced::Alignment>,
    pub position: Option<Position>,
    pub top: Option<f32>,
    pub right: Option<f32>,
    pub bottom: Option<f32>,
    pub left: Option<f32>,
    pub z_index: Option<i32>,
}

impl Default for IcedLayout {
    fn default() -> Self {
        Self {
            width: iced::Length::Shrink,
            height: iced::Length::Shrink,
            padding: iced::Padding::new(0.0),
            align_items: None,
            justify_content: None,
            position: None,
            top: None,
            right: None,
            bottom: None,
            left: None,
            z_index: None,
        }
    }
}
