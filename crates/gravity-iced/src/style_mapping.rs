//! Style mapping from Gravity IR to Iced types
//!
//! This module maps Gravity's backend-agnostic style types to Iced-specific
//! style types.
//!
//! Note: This is a placeholder implementation for Phase 1. Full style mapping
//! will be implemented in later phases.

use gravity_core::ir::style::Color;

/// Map Color to Iced Color
pub fn map_color(color: &Color) -> iced::Color {
    iced::Color {
        r: color.r,
        g: color.g,
        b: color.b,
        a: color.a,
    }
}

/// Placeholder for style properties mapping
/// Full implementation will be added in Phase 6
pub fn map_style_properties(
    _style: &gravity_core::ir::style::StyleProperties,
) -> iced::widget::container::Style {
    iced::widget::container::Style::default()
}

/// Placeholder for layout constraints mapping
/// Full implementation will be added in Phase 3
pub fn map_layout_constraints(_layout: &gravity_core::ir::layout::LayoutConstraints) -> IcedLayout {
    IcedLayout::default()
}

/// Placeholder struct for Iced layout
pub struct IcedLayout {
    pub width: iced::Length,
    pub height: iced::Length,
    pub padding: iced::Padding,
    pub align_items: Option<iced::Alignment>,
    pub justify_content: Option<iced::Alignment>,
}

impl Default for IcedLayout {
    fn default() -> Self {
        Self {
            width: iced::Length::Shrink,
            height: iced::Length::Shrink,
            padding: iced::Padding::new(0.0),
            align_items: None,
            justify_content: None,
        }
    }
}
