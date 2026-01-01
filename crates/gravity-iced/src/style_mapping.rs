//! Style mapping from Gravity IR to Iced types
//!
//! This module maps Gravity's backend-agnostic style types to Iced-specific
//! style types.

use gravity_core::ir::layout::{Alignment, Justification, LayoutConstraints, Length};
use gravity_core::ir::style::{Background, Color, StyleProperties};
use iced::border::Radius;

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
        align_items: layout.align_items.map(|a| map_alignment(a)),
        justify_content: layout.justify_content.map(|j| map_justification(j)),
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

/// Map StyleProperties to Iced container Style
pub fn map_style_properties(style: &StyleProperties) -> iced::widget::container::Style {
    let mut container_style = iced::widget::container::Style::default();

    // Map background
    if let Some(bg) = &style.background {
        container_style.background = Some(map_background(bg));
    }

    // Map text color (this would be applied to text widgets, not container)
    // For now, we'll store it in a custom field if needed

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

    // Opacity would need to be applied at the widget level
    // Transform would need special handling

    container_style
}

/// Map Background to Iced Background
pub fn map_background(background: &Background) -> iced::Background {
    match background {
        Background::Color(color) => iced::Background::Color(map_color(color)),
        Background::Gradient(_gradient) => {
            // Iced supports gradients, but we need to map them
            // For now, return a default color
            iced::Background::Color(iced::Color::WHITE)
        }
        Background::Image { path, fit } => {
            // Iced doesn't have built-in image backgrounds
            // This would need special handling
            iced::Background::Color(iced::Color::WHITE)
        }
    }
}

/// Map BorderRadius to Iced Radius
pub fn map_border_radius(radius: &gravity_core::ir::style::BorderRadius) -> Radius {
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
