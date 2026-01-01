//! Theme management for runtime interpretation
//!
//! This module provides theme resolution and switching capabilities for
//! hot-reload mode.

use gravity_core::ir::theme::{Theme, ThemePalette};
use gravity_core::ir::GravityDocument;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Manages theme definitions and active theme
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeManager {
    /// All available themes
    themes: HashMap<String, Theme>,
    /// Currently active theme name
    current_theme: Option<String>,
    /// Built-in themes
    built_in: HashMap<String, Theme>,
}

impl ThemeManager {
    /// Create a new theme manager
    pub fn new() -> Self {
        let mut built_in = HashMap::new();

        // Define built-in themes
        built_in.insert("light".to_string(), Self::light_theme());
        built_in.insert("dark".to_string(), Self::dark_theme());
        built_in.insert("default".to_string(), Self::default_theme());

        Self {
            themes: HashMap::new(),
            current_theme: None,
            built_in,
        }
    }

    /// Load themes from a document
    pub fn load_from_document(&mut self, doc: &GravityDocument) {
        // Load custom themes
        for (name, theme) in &doc.themes {
            self.themes.insert(name.clone(), theme.clone());
        }

        // Set global theme if specified
        if let Some(global_theme) = &doc.global_theme {
            self.current_theme = Some(global_theme.clone());
        }
    }

    /// Get current theme
    pub fn get_current_theme(&self) -> Option<&Theme> {
        let name = self.current_theme.as_ref()?;

        // Check custom themes first, then built-in
        if let Some(theme) = self.themes.get(name) {
            return Some(theme);
        }

        self.built_in.get(name)
    }

    /// Get theme by name (including built-in)
    pub fn get_theme(&self, name: &str) -> Option<&Theme> {
        if let Some(theme) = self.themes.get(name) {
            return Some(theme);
        }
        self.built_in.get(name)
    }

    /// Set current theme
    pub fn set_theme(&mut self, name: String) -> Result<(), String> {
        if self.themes.contains_key(&name) || self.built_in.contains_key(&name) {
            self.current_theme = Some(name);
            Ok(())
        } else {
            Err(format!("Theme '{}' not found", name))
        }
    }

    /// Get the active palette
    pub fn get_palette(&self) -> Option<&ThemePalette> {
        self.get_current_theme().map(|t| &t.palette)
    }

    /// Get theme for a specific widget (handles theme_ref override)
    pub fn get_widget_theme(&self, widget_theme_ref: Option<&str>) -> Option<&Theme> {
        if let Some(name) = widget_theme_ref {
            self.get_theme(name)
        } else {
            self.get_current_theme()
        }
    }

    // Built-in theme definitions

    #[allow(clippy::unwrap_used)]
    fn light_theme() -> Theme {
        Theme {
            name: "light".to_string(),
            palette: ThemePalette {
                primary: gravity_core::ir::style::Color::parse("#3498db").unwrap(),
                secondary: gravity_core::ir::style::Color::parse("#2ecc71").unwrap(),
                success: gravity_core::ir::style::Color::parse("#27ae60").unwrap(),
                warning: gravity_core::ir::style::Color::parse("#f39c12").unwrap(),
                danger: gravity_core::ir::style::Color::parse("#e74c3c").unwrap(),
                background: gravity_core::ir::style::Color::parse("#ecf0f1").unwrap(),
                surface: gravity_core::ir::style::Color::parse("#ffffff").unwrap(),
                text: gravity_core::ir::style::Color::parse("#2c3e50").unwrap(),
                text_secondary: gravity_core::ir::style::Color::parse("#7f8c8d").unwrap(),
            },
            typography: gravity_core::ir::theme::Typography {
                font_family: "sans-serif".to_string(),
                font_size_base: 16.0,
                font_size_small: 12.0,
                font_size_large: 20.0,
                font_weight: gravity_core::ir::theme::FontWeight::Normal,
                line_height: 1.5,
            },
            spacing: gravity_core::ir::theme::SpacingScale { unit: 4.0 },
            base_styles: HashMap::new(),
        }
    }

    #[allow(clippy::unwrap_used)]
    fn dark_theme() -> Theme {
        Theme {
            name: "dark".to_string(),
            palette: ThemePalette {
                primary: gravity_core::ir::style::Color::parse("#3498db").unwrap(),
                secondary: gravity_core::ir::style::Color::parse("#2ecc71").unwrap(),
                success: gravity_core::ir::style::Color::parse("#27ae60").unwrap(),
                warning: gravity_core::ir::style::Color::parse("#f39c12").unwrap(),
                danger: gravity_core::ir::style::Color::parse("#e74c3c").unwrap(),
                background: gravity_core::ir::style::Color::parse("#2c3e50").unwrap(),
                surface: gravity_core::ir::style::Color::parse("#34495e").unwrap(),
                text: gravity_core::ir::style::Color::parse("#ecf0f1").unwrap(),
                text_secondary: gravity_core::ir::style::Color::parse("#bdc3c7").unwrap(),
            },
            typography: gravity_core::ir::theme::Typography {
                font_family: "sans-serif".to_string(),
                font_size_base: 16.0,
                font_size_small: 12.0,
                font_size_large: 20.0,
                font_weight: gravity_core::ir::theme::FontWeight::Normal,
                line_height: 1.5,
            },
            spacing: gravity_core::ir::theme::SpacingScale { unit: 4.0 },
            base_styles: HashMap::new(),
        }
    }

    fn default_theme() -> Theme {
        Self::light_theme()
    }
}

impl Default for ThemeManager {
    fn default() -> Self {
        Self::new()
    }
}
