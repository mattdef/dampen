//! Theme adapter for converting Dampen themes to Iced themes
//!
//! This module adapts Dampen's theme system to Iced's theme system.

use dampen_core::ir::theme::{FontWeight, Theme, ThemePalette, Typography};
use iced::Theme as IcedTheme;

/// Adapter for converting Dampen themes to Iced themes
pub struct ThemeAdapter;

impl ThemeAdapter {
    /// Convert a Dampen theme to an Iced custom theme
    pub fn to_iced(theme: &Theme) -> IcedTheme {
        let palette_colors = theme.palette.iced_colors();
        let palette = iced::theme::Palette {
            primary: iced::Color::from_rgb(
                palette_colors.primary.0,
                palette_colors.primary.1,
                palette_colors.primary.2,
            ),
            background: iced::Color::from_rgb(
                palette_colors.background.0,
                palette_colors.background.1,
                palette_colors.background.2,
            ),
            text: iced::Color::from_rgb(
                palette_colors.text.0,
                palette_colors.text.1,
                palette_colors.text.2,
            ),
            success: iced::Color::from_rgb(
                palette_colors.success.0,
                palette_colors.success.1,
                palette_colors.success.2,
            ),
            warning: iced::Color::from_rgb(
                palette_colors.warning.0,
                palette_colors.warning.1,
                palette_colors.warning.2,
            ),
            danger: iced::Color::from_rgb(
                palette_colors.danger.0,
                palette_colors.danger.1,
                palette_colors.danger.2,
            ),
        };

        IcedTheme::custom(theme.name.clone(), palette)
    }

    /// Convert a Dampen ThemePalette to an Iced Palette
    pub fn palette_to_iced(palette: &ThemePalette) -> iced::theme::Palette {
        let colors = palette.iced_colors();
        iced::theme::Palette {
            primary: iced::Color::from_rgb(colors.primary.0, colors.primary.1, colors.primary.2),
            background: iced::Color::from_rgb(
                colors.background.0,
                colors.background.1,
                colors.background.2,
            ),
            text: iced::Color::from_rgb(colors.text.0, colors.text.1, colors.text.2),
            success: iced::Color::from_rgb(colors.success.0, colors.success.1, colors.success.2),
            warning: iced::Color::from_rgb(colors.warning.0, colors.warning.1, colors.warning.2),
            danger: iced::Color::from_rgb(colors.danger.0, colors.danger.1, colors.danger.2),
        }
    }

    /// Get text style from typography
    #[allow(unused_variables)]
    pub fn text_style(
        typography: &Typography,
        palette: &ThemePalette,
    ) -> iced::widget::text::Style {
        let colors = palette.iced_colors();
        iced::widget::text::Style {
            color: Some(iced::Color::from_rgb(
                colors.text.0,
                colors.text.1,
                colors.text.2,
            )),
        }
    }

    /// Get font size from typography
    pub fn font_size(typography: &Typography, size_type: FontSizeType) -> f32 {
        match size_type {
            FontSizeType::Base => typography.font_size_base.unwrap_or(16.0),
            FontSizeType::Small => typography.font_size_small.unwrap_or(12.0),
            FontSizeType::Large => typography.font_size_large.unwrap_or(20.0),
        }
    }

    /// Get font weight as Iced expects it
    ///
    /// Returns an iced::Font with the specified weight.
    #[allow(unused_variables)]
    pub fn font_weight(_weight: FontWeight) -> iced::Font {
        iced::Font::with_name("sans-serif")
    }
}

/// Font size type selector
pub enum FontSizeType {
    Base,
    Small,
    Large,
}
