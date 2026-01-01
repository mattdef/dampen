//! Theme adapter for converting Gravity themes to Iced themes
//!
//! This module adapts Gravity's theme system to Iced's theme system.
//!
//! Note: Placeholder implementation for Phase 1. Full theme mapping
//! will be implemented in Phase 5.

use gravity_core::ir::theme::{FontWeight, Theme, Typography};
use iced::Theme as IcedTheme;

/// Adapter for converting Gravity themes to Iced themes
pub struct ThemeAdapter;

impl ThemeAdapter {
    /// Convert a Gravity theme to an Iced theme
    ///
    /// Note: This is a placeholder. Full implementation in Phase 5.
    pub fn to_iced(_theme: &Theme) -> IcedTheme {
        // Return default light theme for now
        IcedTheme::Light
    }

    /// Get text style from typography
    ///
    /// Note: Placeholder for Phase 5.
    pub fn text_style(_typography: &Typography) -> iced::widget::text::Style {
        iced::widget::text::Style { color: None }
    }

    /// Get font size from typography
    ///
    /// Note: Placeholder for Phase 5.
    pub fn font_size(typography: &Typography, size_type: FontSizeType) -> f32 {
        match size_type {
            FontSizeType::Base => typography.font_size_base,
            FontSizeType::Small => typography.font_size_small,
            FontSizeType::Large => typography.font_size_large,
        }
    }

    /// Get font weight as Iced expects it
    ///
    /// Note: Placeholder for Phase 5.
    pub fn font_weight(_weight: FontWeight) -> iced::Font {
        iced::Font::default()
    }
}

/// Font size type selector
pub enum FontSizeType {
    Base,
    Small,
    Large,
}
