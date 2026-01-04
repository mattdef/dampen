//! Type conversions from Gravity IR to Iced types
//!
//! This module re-exports the existing mapping functions from style_mapping.rs
//! for use in the GravityWidgetBuilder.

pub use crate::style_mapping::{
    get_z_index, has_positioning, map_alignment, map_background, map_border_radius, map_color,
    map_gradient, map_justification, map_layout_constraints, map_length, map_padding,
    map_style_properties, IcedLayout,
};

// Re-export IR types for convenience
pub use gravity_core::ir::layout::{Alignment, Justification, LayoutConstraints, Length, Padding};
pub use gravity_core::ir::node::{FloatPosition, ProgressBarStyle, TooltipPosition};
pub use gravity_core::ir::style::{
    Background, Border, BorderRadius, Color, Shadow, StyleProperties, Transform,
};

pub fn map_progress_bar_style(style: ProgressBarStyle) -> &'static str {
    match style {
        ProgressBarStyle::Primary => "primary",
        ProgressBarStyle::Success => "success",
        ProgressBarStyle::Warning => "warning",
        ProgressBarStyle::Danger => "danger",
        ProgressBarStyle::Secondary => "secondary",
    }
}

pub fn map_tooltip_position(position: TooltipPosition) -> &'static str {
    match position {
        TooltipPosition::FollowCursor => "follow_cursor",
        TooltipPosition::Top => "top",
        TooltipPosition::Bottom => "bottom",
        TooltipPosition::Left => "left",
        TooltipPosition::Right => "right",
    }
}

pub fn map_float_position(position: FloatPosition) -> &'static str {
    match position {
        FloatPosition::TopLeft => "top_left",
        FloatPosition::TopRight => "top_right",
        FloatPosition::BottomLeft => "bottom_left",
        FloatPosition::BottomRight => "bottom_right",
    }
}
