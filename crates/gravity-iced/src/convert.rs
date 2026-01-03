//! Type conversions from Gravity IR to Iced types
//!
//! This module re-exports the existing mapping functions from style_mapping.rs
//! for use in the GravityWidgetBuilder.

pub use crate::style_mapping::{
    map_background,
    map_border_radius,
    map_color,
    map_length,
    map_padding,
    map_style_properties,
    map_layout_constraints,
    map_alignment,
    map_justification,
    map_gradient,
    has_positioning,
    get_z_index,
    IcedLayout,
};

// Re-export IR types for convenience
pub use gravity_core::ir::layout::{Alignment, Justification, LayoutConstraints, Length, Padding};
pub use gravity_core::ir::style::{Background, Border, BorderRadius, Color, Shadow, StyleProperties, Transform};
