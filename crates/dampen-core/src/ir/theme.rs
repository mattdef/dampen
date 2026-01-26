//! Theming system types for Dampen UI framework
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
    /// Parent theme for inheritance
    #[serde(default)]
    pub extends: Option<String>,
}

impl Theme {
    /// Validate theme with detailed error messages
    ///
    /// Returns an error if:
    /// - Palette colors are invalid or missing
    /// - Typography values are invalid
    /// - Spacing unit is non-positive
    ///
    /// For themes with inheritance, validates only set colors
    /// For themes without inheritance, validates all required colors
    pub fn validate(&self, _allow_partial: bool) -> Result<(), String> {
        let parent_name = self.extends.as_deref();

        // Validate palette with detailed messages
        // Always use validate_with_inheritance to get proper error messages with theme name
        self.palette
            .validate_with_inheritance(&self.name, parent_name)?;

        // Validate typography with detailed messages
        self.typography
            .validate_with_inheritance(&self.name, parent_name)?;

        // Validate spacing
        if let Some(ref unit) = self.spacing.unit
            && *unit <= 0.0
        {
            return Err(format!(
                "Theme '{}': spacing unit must be positive, got {}\n\
                 Hint: Use a positive value like unit=\"8\" or unit=\"4\"",
                self.name, unit
            ));
        }

        // Validate base styles
        for (widget_type, style) in &self.base_styles {
            style.validate().map_err(|e| {
                format!(
                    "Theme '{}': Invalid base style for '{}': {}",
                    self.name, widget_type, e
                )
            })?;
        }

        Ok(())
    }

    /// Validate theme inheritance
    ///
    /// Returns an error if:
    /// - Parent theme doesn't exist
    /// - Circular inheritance detected
    /// - Inheritance depth exceeds 5 levels
    pub fn validate_inheritance(
        &self,
        all_themes: &HashMap<String, Theme>,
        visited: &mut Vec<String>,
    ) -> Result<(), ThemeError> {
        if let Some(ref parent_name) = self.extends {
            if visited.contains(parent_name) {
                let chain = visited.join(" → ");
                return Err(ThemeError {
                    kind: ThemeErrorKind::CircularInheritance,
                    message: format!(
                        "THEME_007: Circular theme inheritance detected: {} → {}",
                        chain, parent_name
                    ),
                });
            }

            if !all_themes.contains_key(parent_name) {
                return Err(ThemeError {
                    kind: ThemeErrorKind::ThemeNotFound,
                    message: format!(
                        "THEME_006: Parent theme '{}' not found for theme '{}'",
                        parent_name, self.name
                    ),
                });
            }

            visited.push(self.name.clone());
            if visited.len() > 5 {
                return Err(ThemeError {
                    kind: ThemeErrorKind::ExceedsMaxDepth,
                    message: format!(
                        "THEME_008: Theme inheritance depth exceeds 5 levels for '{}'",
                        self.name
                    ),
                });
            }

            if let Some(parent) = all_themes.get(parent_name) {
                parent.validate_inheritance(all_themes, visited)?;
            }
            visited.pop();
        }

        Ok(())
    }

    /// Inherit properties from a parent theme
    ///
    /// The child theme's values take precedence, but missing values
    /// are inherited from the parent theme.
    pub fn inherit_from(&self, parent: &Theme) -> Self {
        Theme {
            name: self.name.clone(),
            palette: self.palette.inherit_from(&parent.palette),
            typography: self.typography.inherit_from(&parent.typography),
            spacing: self.spacing.inherit_from(&parent.spacing),
            base_styles: self.base_styles.clone(),
            extends: self.extends.clone(),
        }
    }
}

/// Theme color palette
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ThemePalette {
    pub primary: Option<Color>,
    pub secondary: Option<Color>,
    pub success: Option<Color>,
    pub warning: Option<Color>,
    pub danger: Option<Color>,
    pub background: Option<Color>,
    pub surface: Option<Color>,
    pub text: Option<Color>,
    pub text_secondary: Option<Color>,
}

impl ThemePalette {
    /// Validate palette for themes with inheritance support
    ///
    /// For themes with inheritance, missing colors can be inherited from parent.
    /// This method provides detailed error messages with suggestions.
    pub fn validate_with_inheritance(
        &self,
        theme_name: &str,
        parent_name: Option<&str>,
    ) -> Result<(), String> {
        let required = [
            ("primary", "primary color for main UI elements"),
            ("secondary", "secondary/accent color"),
            ("success", "success state color"),
            ("warning", "warning state color"),
            ("danger", "danger/error state color"),
            ("background", "background color for containers"),
            ("surface", "surface color for cards, buttons, etc."),
            ("text", "primary text color"),
            ("text_secondary", "secondary/disabled text color"),
        ];

        let mut missing = Vec::new();
        let mut invalid = Vec::new();

        // Check missing colors
        for (color, description) in &required {
            let value = match *color {
                "primary" => &self.primary,
                "secondary" => &self.secondary,
                "success" => &self.success,
                "warning" => &self.warning,
                "danger" => &self.danger,
                "background" => &self.background,
                "surface" => &self.surface,
                "text" => &self.text,
                "text_secondary" => &self.text_secondary,
                _ => unreachable!(),
            };

            if value.is_none() {
                missing.push((*color, *description));
            }
        }

        // If we have a parent and some colors are missing, they're okay (will be inherited)
        if parent_name.is_some() && !missing.is_empty() {
            // Colors will be inherited from parent, no error needed for missing
        } else if !missing.is_empty() {
            let missing_list: Vec<_> = missing.iter().map(|(c, _)| *c).collect();
            let mut message = format!(
                "Theme '{}' is missing {} required color(s): {}",
                theme_name,
                missing.len(),
                missing_list.join(", ")
            );

            if parent_name.is_none() {
                message.push_str("\n\nTip: If you want to inherit colors from another theme, add 'extends=\"parent_theme\"' attribute to this theme.");
                message.push_str("\nExample: <theme name=\"dark\" extends=\"base\">");
            }

            return Err(message);
        }

        // Validate color values (only for colors that are set)
        for (color, _description) in &required {
            let value = match *color {
                "primary" => self.primary.as_ref(),
                "secondary" => self.secondary.as_ref(),
                "success" => self.success.as_ref(),
                "warning" => self.warning.as_ref(),
                "danger" => self.danger.as_ref(),
                "background" => self.background.as_ref(),
                "surface" => self.surface.as_ref(),
                "text" => self.text.as_ref(),
                "text_secondary" => self.text_secondary.as_ref(),
                _ => unreachable!(),
            };

            if let Some(color_val) = value
                && let Err(e) = color_val.validate()
            {
                invalid.push((*color, e));
            }
        }

        if !invalid.is_empty() {
            let mut message = format!("Theme '{}' has invalid color values:\n", theme_name);
            for (color, error) in &invalid {
                message.push_str(&format!("  - {}: {}\n", color, error));
            }
            message.push_str("\nValid color formats:\n");
            message.push_str("  - Hex: #RRGGBB or #RRGGBBAA\n");
            message.push_str("  - RGB: rgb(r, g, b) or rgba(r, g, b, a)\n");
            message.push_str("  - HSL: hsl(h, s%, l%) or hsla(h, s%, l%, a)\n");
            message.push_str("  - Named: red, blue, transparent, etc.");
            return Err(message);
        }

        Ok(())
    }

    /// Validate palette (legacy method, calls validate_with_inheritance with no parent)
    pub fn validate(&self) -> Result<(), String> {
        self.validate_with_inheritance("theme", None)
    }

    /// Merge with a parent palette, inheriting missing values
    pub fn inherit_from(&self, parent: &ThemePalette) -> Self {
        Self {
            primary: self.primary.or(parent.primary),
            secondary: self.secondary.or(parent.secondary),
            success: self.success.or(parent.success),
            warning: self.warning.or(parent.warning),
            danger: self.danger.or(parent.danger),
            background: self.background.or(parent.background),
            surface: self.surface.or(parent.surface),
            text: self.text.or(parent.text),
            text_secondary: self.text_secondary.or(parent.text_secondary),
        }
    }

    /// Get colors for Iced Palette (6 colors)
    ///
    /// Returns RGB tuples (r, g, b) in 0.0-1.0 range for Iced compatibility.
    /// Panics if any required color is missing.
    #[allow(clippy::expect_used)]
    pub fn iced_colors(&self) -> IcedPaletteColors {
        IcedPaletteColors {
            primary: (
                self.primary.expect("primary color must be set").r,
                self.primary.expect("primary color must be set").g,
                self.primary.expect("primary color must be set").b,
            ),
            background: (
                self.background.expect("background color must be set").r,
                self.background.expect("background color must be set").g,
                self.background.expect("background color must be set").b,
            ),
            text: (
                self.text.expect("text color must be set").r,
                self.text.expect("text color must be set").g,
                self.text.expect("text color must be set").b,
            ),
            success: (
                self.success.expect("success color must be set").r,
                self.success.expect("success color must be set").g,
                self.success.expect("success color must be set").b,
            ),
            warning: (
                self.warning.expect("warning color must be set").r,
                self.warning.expect("warning color must be set").g,
                self.warning.expect("warning color must be set").b,
            ),
            danger: (
                self.danger.expect("danger color must be set").r,
                self.danger.expect("danger color must be set").g,
                self.danger.expect("danger color must be set").b,
            ),
        }
    }

    /// Create a light theme palette with standard colors
    ///
    /// # Example
    ///
    /// ```rust
    /// use dampen_core::ir::theme::ThemePalette;
    ///
    /// let palette = ThemePalette::light();
    /// ```
    pub fn light() -> Self {
        use crate::ir::style::Color;
        Self {
            primary: Some(Color::from_rgb8(0x34, 0x98, 0xDB)),
            secondary: Some(Color::from_rgb8(0x2E, 0xCC, 0x71)),
            success: Some(Color::from_rgb8(0x27, 0xAE, 0x60)),
            warning: Some(Color::from_rgb8(0xF3, 0x9C, 0x12)),
            danger: Some(Color::from_rgb8(0xE7, 0x4C, 0x3C)),
            background: Some(Color::from_rgb8(0xEC, 0xF0, 0xF1)),
            surface: Some(Color::from_rgb8(0xFF, 0xFF, 0xFF)),
            text: Some(Color::from_rgb8(0x2C, 0x3E, 0x50)),
            text_secondary: Some(Color::from_rgb8(0x7F, 0x8C, 0x8D)),
        }
    }

    /// Create a dark theme palette with standard colors
    ///
    /// # Example
    ///
    /// ```rust
    /// use dampen_core::ir::theme::ThemePalette;
    ///
    /// let palette = ThemePalette::dark();
    /// ```
    pub fn dark() -> Self {
        use crate::ir::style::Color;
        Self {
            primary: Some(Color::from_rgb8(0x5D, 0xAD, 0xE2)),
            secondary: Some(Color::from_rgb8(0x52, 0xBE, 0x80)),
            success: Some(Color::from_rgb8(0x27, 0xAE, 0x60)),
            warning: Some(Color::from_rgb8(0xF3, 0x9C, 0x12)),
            danger: Some(Color::from_rgb8(0xEC, 0x70, 0x63)),
            background: Some(Color::from_rgb8(0x2C, 0x3E, 0x50)),
            surface: Some(Color::from_rgb8(0x34, 0x49, 0x5E)),
            text: Some(Color::from_rgb8(0xEC, 0xF0, 0xF1)),
            text_secondary: Some(Color::from_rgb8(0x95, 0xA5, 0xA6)),
        }
    }
}

/// Colors suitable for Iced Palette (0.0-1.0 RGB range)
#[derive(Debug, Clone, Copy)]
pub struct IcedPaletteColors {
    pub primary: (f32, f32, f32),
    pub background: (f32, f32, f32),
    pub text: (f32, f32, f32),
    pub success: (f32, f32, f32),
    pub warning: (f32, f32, f32),
    pub danger: (f32, f32, f32),
}

/// Typography configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Typography {
    pub font_family: Option<String>,
    pub font_size_base: Option<f32>,
    pub font_size_small: Option<f32>,
    pub font_size_large: Option<f32>,
    pub font_weight: FontWeight,
    pub line_height: Option<f32>,
}

impl Typography {
    /// Validate typography with detailed error messages
    pub fn validate_with_inheritance(
        &self,
        theme_name: &str,
        parent_name: Option<&str>,
    ) -> Result<(), String> {
        let mut errors = Vec::new();

        if let Some(size) = self.font_size_base {
            if size <= 0.0 {
                errors.push(format!("font_size_base must be positive, got {}", size));
            } else if size < 8.0 {
                errors.push(format!(
                    "font_size_base {} is very small (recommended: 14-18px)",
                    size
                ));
            } else if size > 32.0 {
                errors.push(format!(
                    "font_size_base {} is very large (recommended: 14-18px)",
                    size
                ));
            }
        }

        if let Some(size) = self.font_size_small {
            if size <= 0.0 {
                errors.push(format!("font_size_small must be positive, got {}", size));
            } else if size >= self.font_size_base.unwrap_or(16.0) {
                errors.push("font_size_small should be smaller than font_size_base".to_string());
            }
        }

        if let Some(size) = self.font_size_large {
            if size <= 0.0 {
                errors.push(format!("font_size_large must be positive, got {}", size));
            } else if size <= self.font_size_base.unwrap_or(16.0) {
                errors.push("font_size_large should be larger than font_size_base".to_string());
            }
        }

        if let Some(height) = self.line_height {
            if height <= 0.0 {
                errors.push(format!("line_height must be positive, got {}", height));
            } else if height < 1.0 {
                errors.push(format!(
                    "line_height {} is too tight (recommended: 1.4-1.6)",
                    height
                ));
            } else if height > 2.5 {
                errors.push(format!(
                    "line_height {} is too loose (recommended: 1.4-1.6)",
                    height
                ));
            }
        }

        if !errors.is_empty() {
            let mut message = format!("Typography validation failed for theme '{}':\n", theme_name);
            for error in &errors {
                message.push_str(&format!("  - {}\n", error));
            }

            if parent_name.is_none() {
                message.push_str("\nTip: Missing typography values will inherit from parent theme if 'extends' is used.");
            }

            message.push_str("\nExample typography configuration:");
            message.push_str("\n  <typography");
            message.push_str("\n      font_family=\"Inter, sans-serif\"");
            message.push_str("\n      font_size_base=\"16\"");
            message.push_str("\n      font_size_small=\"12\"");
            message.push_str("\n      font_size_large=\"20\"");
            message.push_str("\n      font_weight=\"normal\"");
            message.push_str("\n      line_height=\"1.5\" />");

            return Err(message);
        }

        Ok(())
    }

    /// Validate typography (legacy method)
    pub fn validate(&self) -> Result<(), String> {
        self.validate_with_inheritance("theme", None)
    }

    /// Merge with a parent typography, inheriting missing values
    pub fn inherit_from(&self, parent: &Typography) -> Self {
        Self {
            font_family: self
                .font_family
                .clone()
                .or_else(|| parent.font_family.clone()),
            font_size_base: self.font_size_base.or(parent.font_size_base),
            font_size_small: self.font_size_small.or(parent.font_size_small),
            font_size_large: self.font_size_large.or(parent.font_size_large),
            font_weight: self.font_weight,
            line_height: self.line_height.or(parent.line_height),
        }
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
    pub unit: Option<f32>,
}

impl SpacingScale {
    /// Validate spacing scale with detailed error messages
    pub fn validate_with_inheritance(
        &self,
        theme_name: &str,
        parent_name: Option<&str>,
    ) -> Result<(), String> {
        if let Some(unit) = self.unit {
            if unit <= 0.0 {
                let mut message = format!(
                    "Theme '{}': spacing unit must be positive, got {}\n",
                    theme_name, unit
                );
                message.push_str("Valid spacing examples:\n");
                message.push_str("  - <spacing unit=\"4\" />   (4px base)\n");
                message.push_str("  - <spacing unit=\"8\" />   (8px base, recommended)\n");
                message.push_str("  - <spacing unit=\"16\" />  (16px base)\n");

                if parent_name.is_none() {
                    message.push_str("\nTip: Missing spacing will inherit from parent theme if 'extends' is used.");
                }

                return Err(message);
            }

            if unit > 32.0 {
                // Consider using 4-8px for better visual consistency
            }
        }
        Ok(())
    }

    /// Validate spacing scale (legacy method)
    pub fn validate(&self) -> Result<(), String> {
        self.validate_with_inheritance("theme", None)
    }

    /// Get spacing for a multiplier
    pub fn get(&self, multiplier: u8) -> f32 {
        (self.unit.unwrap_or(8.0)) * multiplier as f32
    }

    /// Merge with a parent spacing scale, inheriting missing values
    pub fn inherit_from(&self, parent: &SpacingScale) -> Self {
        Self {
            unit: self.unit.or(parent.unit),
        }
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
    /// State-specific overrides (single states)
    pub state_variants: HashMap<WidgetState, StyleProperties>,
    /// Combined state overrides (e.g., hover:active)
    #[serde(default)]
    pub combined_state_variants: HashMap<StateSelector, StyleProperties>,
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

        // Validate combined state variants
        for (selector, style) in &self.combined_state_variants {
            style
                .validate()
                .map_err(|e| format!("Invalid style for state selector {:?}: {}", selector, e))?;
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, PartialOrd, Ord)]
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

/// State selector for style matching - can be single or combined states
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StateSelector {
    /// Single state (e.g., "hover")
    Single(WidgetState),
    /// Combined states that must all be active (e.g., "hover:active")
    /// Sorted for consistent comparison
    Combined(Vec<WidgetState>),
}

impl StateSelector {
    /// Create a single state selector
    pub fn single(state: WidgetState) -> Self {
        StateSelector::Single(state)
    }

    /// Create a combined state selector from multiple states
    pub fn combined(mut states: Vec<WidgetState>) -> Self {
        if states.len() == 1 {
            StateSelector::Single(states[0])
        } else {
            // Sort for consistent comparison
            states.sort();
            states.dedup(); // Remove duplicates
            StateSelector::Combined(states)
        }
    }

    /// Check if this selector matches the given active states
    pub fn matches(&self, active_states: &[WidgetState]) -> bool {
        match self {
            StateSelector::Single(state) => active_states.contains(state),
            StateSelector::Combined(required_states) => {
                required_states.iter().all(|s| active_states.contains(s))
            }
        }
    }

    /// Get specificity for cascade resolution (more specific = higher number)
    pub fn specificity(&self) -> usize {
        match self {
            StateSelector::Single(_) => 1,
            StateSelector::Combined(states) => states.len(),
        }
    }
}

/// Error codes for theme-related errors
#[derive(Debug, Clone, PartialEq)]
pub enum ThemeErrorKind {
    NoThemesDefined,
    InvalidDefaultTheme,
    MissingPaletteColor,
    InvalidColorValue,
    DuplicateThemeName,
    ThemeNotFound,
    CircularInheritance,
    ExceedsMaxDepth,
}

/// Theme-related errors
#[derive(Debug, Clone, PartialEq)]
pub struct ThemeError {
    pub kind: ThemeErrorKind,
    pub message: String,
}

impl std::fmt::Display for ThemeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ThemeError {}

impl std::fmt::Display for ThemeErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ThemeErrorKind::NoThemesDefined => write!(f, "THEME_001"),
            ThemeErrorKind::InvalidDefaultTheme => write!(f, "THEME_002"),
            ThemeErrorKind::MissingPaletteColor => write!(f, "THEME_003"),
            ThemeErrorKind::InvalidColorValue => write!(f, "THEME_004"),
            ThemeErrorKind::DuplicateThemeName => write!(f, "THEME_005"),
            ThemeErrorKind::ThemeNotFound => write!(f, "THEME_006"),
            ThemeErrorKind::CircularInheritance => write!(f, "THEME_007"),
            ThemeErrorKind::ExceedsMaxDepth => write!(f, "THEME_008"),
        }
    }
}

/// Root document for theme.dampen file
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ThemeDocument {
    /// All defined themes (light, dark, custom, etc.)
    pub themes: HashMap<String, Theme>,

    /// Default theme name to use on startup
    /// If None, follows system preference
    pub default_theme: Option<String>,

    /// Whether to auto-detect system dark/light mode
    pub follow_system: bool,
}

impl ThemeDocument {
    /// Validate the document
    ///
    /// Returns an error if:
    /// - No themes are defined (THEME_001)
    /// - Default theme is specified but doesn't exist (THEME_002)
    /// - Any theme fails validation
    pub fn validate(&self) -> Result<(), ThemeError> {
        if self.themes.is_empty() {
            return Err(ThemeError {
                kind: ThemeErrorKind::NoThemesDefined,
                message: "THEME_001: At least one theme must be defined in theme.dampen"
                    .to_string(),
            });
        }

        if let Some(ref default) = self.default_theme
            && !self.themes.contains_key(default)
        {
            let available: Vec<_> = self.themes.keys().cloned().collect();
            return Err(ThemeError {
                kind: ThemeErrorKind::InvalidDefaultTheme,
                message: format!(
                    "THEME_002: Default theme '{}' not found. Available: {}",
                    default,
                    available.join(", ")
                ),
            });
        }

        for (name, theme) in &self.themes {
            let allow_partial = theme.extends.is_some();
            theme.validate(allow_partial).map_err(|e| ThemeError {
                kind: ThemeErrorKind::MissingPaletteColor,
                message: format!("THEME_003: Invalid theme '{}': {}", name, e),
            })?;
        }

        Ok(())
    }

    /// Validate inheritance for all themes
    ///
    /// Returns an error if:
    /// - Parent theme doesn't exist (THEME_006)
    /// - Circular inheritance detected (THEME_007)
    /// - Inheritance depth exceeds 5 levels (THEME_008)
    pub fn validate_inheritance(&self) -> Result<(), ThemeError> {
        for theme in self.themes.values() {
            let mut visited = Vec::new();
            theme.validate_inheritance(&self.themes, &mut visited)?;
        }
        Ok(())
    }

    /// Resolve inheritance for all themes
    ///
    /// Creates a new HashMap where each theme inherits from its parent.
    /// Themes without inheritance are copied as-is.
    pub fn resolve_inheritance(&self) -> HashMap<String, Theme> {
        let mut resolved = HashMap::new();

        // Helper function to resolve a single theme recursively
        fn resolve(
            name: &str,
            themes: &HashMap<String, Theme>,
            resolved: &mut HashMap<String, Theme>,
        ) {
            if resolved.contains_key(name) {
                return;
            }

            if let Some(theme) = themes.get(name) {
                if let Some(ref parent_name) = theme.extends {
                    resolve(parent_name, themes, resolved);
                    if let Some(parent) = resolved.get(parent_name) {
                        resolved.insert(name.to_string(), theme.inherit_from(parent));
                    } else {
                        // Parent not found (should be caught by validation)
                        resolved.insert(name.to_string(), theme.clone());
                    }
                } else {
                    resolved.insert(name.to_string(), theme.clone());
                }
            }
        }

        for name in self.themes.keys() {
            resolve(name, &self.themes, &mut resolved);
        }

        resolved
    }

    /// Get the effective default theme name
    ///
    /// Priority: user_preference > default_theme > system_preference > "light"
    pub fn effective_default<'a>(&'a self, system_preference: Option<&'a str>) -> &'a str {
        if self.follow_system
            && let Some(sys) = system_preference
            && self.themes.contains_key(sys)
        {
            return sys;
        }
        if let Some(ref default) = self.default_theme {
            return default;
        }
        "light"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_widget_state_from_prefix_hover() {
        assert_eq!(WidgetState::from_prefix("hover"), Some(WidgetState::Hover));
    }

    #[test]
    fn test_widget_state_from_prefix_focus() {
        assert_eq!(WidgetState::from_prefix("focus"), Some(WidgetState::Focus));
    }

    #[test]
    fn test_widget_state_from_prefix_active() {
        assert_eq!(
            WidgetState::from_prefix("active"),
            Some(WidgetState::Active)
        );
    }

    #[test]
    fn test_widget_state_from_prefix_disabled() {
        assert_eq!(
            WidgetState::from_prefix("disabled"),
            Some(WidgetState::Disabled)
        );
    }

    #[test]
    fn test_widget_state_from_prefix_case_insensitive() {
        assert_eq!(WidgetState::from_prefix("HOVER"), Some(WidgetState::Hover));
        assert_eq!(WidgetState::from_prefix("Focus"), Some(WidgetState::Focus));
        assert_eq!(
            WidgetState::from_prefix("AcTiVe"),
            Some(WidgetState::Active)
        );
    }

    #[test]
    fn test_widget_state_from_prefix_invalid() {
        assert_eq!(WidgetState::from_prefix("unknown"), None);
        assert_eq!(WidgetState::from_prefix("pressed"), None);
        assert_eq!(WidgetState::from_prefix(""), None);
    }

    #[test]
    fn test_widget_state_from_prefix_with_whitespace() {
        // from_prefix uses trim() so whitespace should be handled
        assert_eq!(
            WidgetState::from_prefix("  hover  "),
            Some(WidgetState::Hover)
        );
    }
}
