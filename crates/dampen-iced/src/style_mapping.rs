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
