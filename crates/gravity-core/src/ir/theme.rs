//! Theming system types for Gravity UI framework
//!
//! This module defines the IR types for theme definitions, color palettes,
//! typography, spacing scales, and style classes with inheritance.
//! All types are backend-agnostic and serializable.

use super::layout::LayoutConstraints;
use super::style::{Color, StyleProperties};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Theme definition containing all visual properties
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Theme {
    pub name: String,
    pub palette: ThemePalette,
    pub typography: Typography,
    pub spacing: SpacingScale,
    /// Default styles per widget type
    pub base_styles: HashMap<String, StyleProperties>,
}

impl Theme {
    /// Validate theme
    ///
    /// Returns an error if:
    /// - Palette colors are invalid
    /// - Typography values are invalid
    /// - Spacing unit is non-positive
    pub fn validate(&self) -> Result<(), String> {
        self.palette.validate()?;
        self.typography.validate()?;
        self.spacing.validate()?;

        for (widget_type, style) in &self.base_styles {
            style
                .validate()
                .map_err(|e| format!("Invalid base style for '{}': {}", widget_type, e))?;
        }

        Ok(())
    }
}

/// Theme color palette
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ThemePalette {
    pub primary: Color,
    pub secondary: Color,
    pub success: Color,
    pub warning: Color,
    pub danger: Color,
    pub background: Color,
    pub surface: Color,
    pub text: Color,
    pub text_secondary: Color,
}

impl ThemePalette {
    pub fn validate(&self) -> Result<(), String> {
        self.primary.validate()?;
        self.secondary.validate()?;
        self.success.validate()?;
        self.warning.validate()?;
        self.danger.validate()?;
        self.background.validate()?;
        self.surface.validate()?;
        self.text.validate()?;
        self.text_secondary.validate()?;
        Ok(())
    }
}

/// Typography configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Typography {
    pub font_family: String,
    pub font_size_base: f32,
    pub font_size_small: f32,
    pub font_size_large: f32,
    pub font_weight: FontWeight,
    pub line_height: f32,
}

impl Typography {
    pub fn validate(&self) -> Result<(), String> {
        if self.font_size_base <= 0.0 {
            return Err(format!(
                "font_size_base must be positive, got {}",
                self.font_size_base
            ));
        }
        if self.font_size_small <= 0.0 {
            return Err(format!(
                "font_size_small must be positive, got {}",
                self.font_size_small
            ));
        }
        if self.font_size_large <= 0.0 {
            return Err(format!(
                "font_size_large must be positive, got {}",
                self.font_size_large
            ));
        }
        if self.line_height <= 0.0 {
            return Err(format!(
                "line_height must be positive, got {}",
                self.line_height
            ));
        }
        Ok(())
    }
}

/// Font weight
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FontWeight {
    Thin,
    Light,
    Normal,
    Medium,
    Bold,
    Black,
}

impl FontWeight {
    /// Parse from string
    pub fn parse(s: &str) -> Result<Self, String> {
        match s.trim().to_lowercase().as_str() {
            "thin" => Ok(FontWeight::Thin),
            "light" => Ok(FontWeight::Light),
            "normal" => Ok(FontWeight::Normal),
            "medium" => Ok(FontWeight::Medium),
            "bold" => Ok(FontWeight::Bold),
            "black" => Ok(FontWeight::Black),
            _ => Err(format!(
                "Invalid font weight: '{}'. Expected thin, light, normal, medium, bold, or black",
                s
            )),
        }
    }

    /// Convert to CSS numeric value
    pub fn to_css(&self) -> u16 {
        match self {
            FontWeight::Thin => 100,
            FontWeight::Light => 300,
            FontWeight::Normal => 400,
            FontWeight::Medium => 500,
            FontWeight::Bold => 700,
            FontWeight::Black => 900,
        }
    }
}

/// Spacing scale configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpacingScale {
    /// Base spacing unit (e.g., 4px)
    pub unit: f32,
}

impl SpacingScale {
    pub fn validate(&self) -> Result<(), String> {
        if self.unit <= 0.0 {
            return Err(format!("spacing unit must be positive, got {}", self.unit));
        }
        Ok(())
    }

    /// Get spacing for a multiplier
    pub fn get(&self, multiplier: u8) -> f32 {
        self.unit * multiplier as f32
    }
}

/// Style class definition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StyleClass {
    pub name: String,
    pub style: StyleProperties,
    pub layout: Option<LayoutConstraints>,
    /// Inherit from other classes
    pub extends: Vec<String>,
    /// State-specific overrides
    pub state_variants: HashMap<WidgetState, StyleProperties>,
}

impl StyleClass {
    /// Validate class definition
    ///
    /// Returns an error if:
    /// - Style properties are invalid
    /// - Layout constraints are invalid
    /// - Circular dependency detected
    /// - Inheritance depth exceeds 5 levels
    /// - Referenced parent classes don't exist
    pub fn validate(&self, all_classes: &HashMap<String, StyleClass>) -> Result<(), String> {
        // Validate own properties
        self.style
            .validate()
            .map_err(|e| format!("Invalid style: {}", e))?;

        if let Some(layout) = &self.layout {
            layout
                .validate()
                .map_err(|e| format!("Invalid layout: {}", e))?;
        }

        // Check inheritance depth
        self.check_inheritance_depth(all_classes, 0)?;

        // Check for circular dependencies
        self.check_circular_dependency(all_classes, &mut Vec::new())?;

        // Validate state variants
        for (state, style) in &self.state_variants {
            style
                .validate()
                .map_err(|e| format!("Invalid style for state {:?}: {}", state, e))?;
        }

        // Verify all extended classes exist
        for parent in &self.extends {
            if !all_classes.contains_key(parent) {
                return Err(format!("Parent class '{}' not found", parent));
            }
        }

        Ok(())
    }

    fn check_inheritance_depth(
        &self,
        all_classes: &HashMap<String, StyleClass>,
        depth: u8,
    ) -> Result<(), String> {
        if depth > 5 {
            return Err(format!(
                "Style class inheritance depth exceeds 5 levels (class: {})",
                self.name
            ));
        }

        for parent_name in &self.extends {
            if let Some(parent) = all_classes.get(parent_name) {
                parent.check_inheritance_depth(all_classes, depth + 1)?;
            }
        }

        Ok(())
    }

    fn check_circular_dependency(
        &self,
        all_classes: &HashMap<String, StyleClass>,
        path: &mut Vec<String>,
    ) -> Result<(), String> {
        if path.contains(&self.name) {
            let chain = path.join(" → ");
            return Err(format!(
                "Circular style class dependency detected: {} → {}",
                chain, self.name
            ));
        }

        path.push(self.name.clone());

        for parent_name in &self.extends {
            if let Some(parent) = all_classes.get(parent_name) {
                parent.check_circular_dependency(all_classes, path)?;
            }
        }

        path.pop();
        Ok(())
    }
}

/// Widget interaction state
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WidgetState {
    Hover,
    Focus,
    Active,
    Disabled,
}

impl WidgetState {
    /// Parse from string prefix
    pub fn from_prefix(s: &str) -> Option<Self> {
        match s.trim().to_lowercase().as_str() {
            "hover" => Some(WidgetState::Hover),
            "focus" => Some(WidgetState::Focus),
            "active" => Some(WidgetState::Active),
            "disabled" => Some(WidgetState::Disabled),
            _ => None,
        }
    }
}
