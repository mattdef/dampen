//! Runtime theme context for managing active themes.
//!
//! This module provides the [`ThemeContext`] struct that holds the current
//! active theme and manages theme switching at runtime.

use crate::ir::theme::{Theme, ThemeDocument, ThemeError, ThemeErrorKind};
use std::collections::HashMap;

/// Runtime theme context shared across all windows.
///
/// This struct manages theme state including:
/// - The currently active theme
/// - All loaded themes from theme.dampen
/// - System preference detection
/// - User preference persistence
///
/// # Examples
///
/// ```rust,ignore
/// use dampen_core::{parse_theme_document, ThemeContext};
///
/// let xml = r#"<dampen><themes><theme name="light">...</theme></themes></dampen>"#;
/// let doc = parse_theme_document(xml).unwrap();
/// let ctx = ThemeContext::from_document(doc, Some("dark"));
///
/// assert_eq!(ctx.active().name, "dark");
/// ```
#[derive(Debug, Clone)]
pub struct ThemeContext {
    active_theme: String,
    themes: HashMap<String, Theme>,
    system_preference: Option<String>,
    follow_system: bool,
    user_preference: Option<String>,
}

impl ThemeContext {
    /// Create a ThemeContext from a parsed ThemeDocument.
    ///
    /// The active theme is determined by:
    /// 1. User preference (if set)
    /// 2. Document's default_theme (if set)
    /// 3. System preference (if follow_system is true)
    /// 4. "light" fallback
    ///
    /// # Arguments
    ///
    /// * `document` - The parsed theme document
    /// * `system_preference` - Optional detected system theme preference
    ///
    /// # Errors
    ///
    /// Returns `ThemeError` if the document is invalid or has no themes.
    pub fn from_document(
        document: ThemeDocument,
        system_preference: Option<&str>,
    ) -> Result<Self, ThemeError> {
        if document.themes.is_empty() {
            return Err(ThemeError {
                kind: ThemeErrorKind::NoThemesDefined,
                message: "THEME_001: Cannot create ThemeContext with no themes".to_string(),
            });
        }

        let active_theme = if let Some(user_pref) = document.themes.get("user_preference") {
            user_pref.name.clone()
        } else {
            document.effective_default(system_preference).to_string()
        };

        if !document.themes.contains_key(&active_theme) {
            return Err(ThemeError {
                kind: ThemeErrorKind::ThemeNotFound,
                message: format!(
                    "THEME_006: Active theme '{}' not found in document",
                    active_theme
                ),
            });
        }

        Ok(ThemeContext {
            active_theme,
            themes: document.resolve_inheritance(),
            system_preference: system_preference.map(|s| s.to_string()),
            follow_system: document.follow_system,
            user_preference: None,
        })
    }

    /// Get the currently active theme.
    ///
    /// # Returns
    ///
    /// Reference to the active [`Theme`].
    #[allow(clippy::unwrap_used)]
    pub fn active(&self) -> &Theme {
        self.themes.get(&self.active_theme).unwrap()
    }

    /// Get the name of the currently active theme.
    pub fn active_name(&self) -> &str {
        &self.active_theme
    }

    /// Switch to a different theme by name.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the theme to switch to
    ///
    /// # Errors
    ///
    /// Returns `ThemeError::ThemeNotFound` if the theme doesn't exist.
    pub fn set_theme(&mut self, name: &str) -> Result<(), ThemeError> {
        if !self.themes.contains_key(name) {
            return Err(ThemeError {
                kind: ThemeErrorKind::ThemeNotFound,
                message: format!("THEME_006: Theme '{}' not found", name),
            });
        }

        self.active_theme = name.to_string();
        self.user_preference = Some(name.to_string());
        Ok(())
    }

    /// Update the system preference and potentially switch theme.
    ///
    /// If the document is configured to follow system preference,
    /// this will switch to the system theme if it exists.
    ///
    /// # Arguments
    ///
    /// * `preference` - The new system preference ("light" or "dark")
    pub fn update_system_preference(&mut self, preference: &str) {
        self.system_preference = Some(preference.to_string());

        if self.follow_system
            && self.user_preference.is_none()
            && self.themes.contains_key(preference)
        {
            self.active_theme = preference.to_string();
        }
    }

    /// Set whether to follow system preference.
    pub fn set_follow_system(&mut self, follow: bool) {
        self.follow_system = follow;

        // If enabling, immediately apply system preference if available
        if follow
            && let Some(ref pref) = self.system_preference
            && self.themes.contains_key(pref)
        {
            self.active_theme = pref.clone();
        }
    }

    /// Check if currently following system preference.
    pub fn follow_system(&self) -> bool {
        self.follow_system
    }

    /// Reload themes from a new document (for hot-reload).
    ///
    /// Preserves the current active theme if it exists in the new document,
    /// otherwise falls back to the new document's default.
    ///
    /// # Arguments
    ///
    /// * `document` - The new parsed theme document
    pub fn reload(&mut self, document: ThemeDocument) {
        let old_active = self.active_theme.clone();
        let resolved_themes = document.resolve_inheritance();
        let fallback_theme = document.effective_default(self.system_preference.as_deref());

        self.themes = resolved_themes;
        self.active_theme = if self.themes.contains_key(&old_active) {
            old_active
        } else {
            fallback_theme.to_string()
        };
    }

    /// Get all available theme names.
    pub fn available_themes(&self) -> Vec<&str> {
        self.themes.keys().map(|s| s.as_str()).collect()
    }

    /// Check if a theme with the given name exists.
    pub fn has_theme(&self, name: &str) -> bool {
        self.themes.contains_key(name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::style::Color;
    use crate::ir::theme::{SpacingScale, Typography};

    fn create_test_theme(name: &str) -> Theme {
        Theme {
            name: name.to_string(),
            palette: crate::ir::theme::ThemePalette {
                primary: Some(Color::from_hex("#3498db").unwrap()),
                secondary: Some(Color::from_hex("#2ecc71").unwrap()),
                success: Some(Color::from_hex("#27ae60").unwrap()),
                warning: Some(Color::from_hex("#f39c12").unwrap()),
                danger: Some(Color::from_hex("#e74c3c").unwrap()),
                background: Some(Color::from_hex("#ecf0f1").unwrap()),
                surface: Some(Color::from_hex("#ffffff").unwrap()),
                text: Some(Color::from_hex("#2c3e50").unwrap()),
                text_secondary: Some(Color::from_hex("#7f8c8d").unwrap()),
            },
            typography: Typography {
                font_family: Some("sans-serif".to_string()),
                font_size_base: Some(16.0),
                font_size_small: Some(12.0),
                font_size_large: Some(24.0),
                font_weight: crate::ir::theme::FontWeight::Normal,
                line_height: Some(1.5),
            },
            spacing: SpacingScale { unit: Some(8.0) },
            base_styles: HashMap::new(),
            extends: None,
        }
    }

    fn create_test_document() -> ThemeDocument {
        ThemeDocument {
            themes: HashMap::from([
                ("light".to_string(), create_test_theme("light")),
                ("dark".to_string(), create_test_theme("dark")),
            ]),
            default_theme: Some("light".to_string()),
            follow_system: true,
        }
    }

    #[test]
    fn test_from_document_with_system_preference() {
        let doc = create_test_document();
        let ctx = ThemeContext::from_document(doc, Some("dark")).unwrap();

        assert_eq!(ctx.active_name(), "dark");
    }

    #[test]
    fn test_from_document_without_system_preference() {
        let doc = create_test_document();
        let ctx = ThemeContext::from_document(doc, None).unwrap();

        assert_eq!(ctx.active_name(), "light");
    }

    #[test]
    fn test_set_theme() {
        let doc = create_test_document();
        let mut ctx = ThemeContext::from_document(doc, None).unwrap();

        assert_eq!(ctx.active_name(), "light");

        ctx.set_theme("dark").unwrap();
        assert_eq!(ctx.active_name(), "dark");

        assert!(ctx.set_theme("nonexistent").is_err());
    }

    #[test]
    fn test_update_system_preference() {
        let doc = create_test_document();
        let mut ctx = ThemeContext::from_document(doc.clone(), None).unwrap();

        assert_eq!(ctx.active_name(), "light");

        ctx.update_system_preference("dark");
        assert_eq!(ctx.active_name(), "dark");

        ctx.update_system_preference("light");
        assert_eq!(ctx.active_name(), "light");

        ctx.set_follow_system(false);
        ctx.update_system_preference("dark");
        assert_eq!(ctx.active_name(), "light");
    }

    #[test]
    fn test_reload() {
        let doc = create_test_document();
        let mut ctx = ThemeContext::from_document(doc, None).unwrap();

        assert_eq!(ctx.active_name(), "light");

        let mut new_doc = create_test_document();
        new_doc.default_theme = Some("dark".to_string());
        new_doc.themes.remove("light");

        ctx.reload(new_doc);
        assert_eq!(ctx.active_name(), "dark");
    }

    #[test]
    fn test_inheritance_resolution() {
        let mut themes = HashMap::new();

        // Base theme
        let base = create_test_theme("base");
        themes.insert("base".to_string(), base);

        // Derived theme
        let mut derived = create_test_theme("derived");
        derived.extends = Some("base".to_string());
        derived.palette.primary = None; // Should be inherited
        themes.insert("derived".to_string(), derived);

        let doc = ThemeDocument {
            themes,
            default_theme: Some("derived".to_string()),
            follow_system: false,
        };

        let ctx = ThemeContext::from_document(doc, None).unwrap();
        let active = ctx.active();

        assert_eq!(active.name, "derived");
        assert!(
            active.palette.primary.is_some(),
            "Primary color should be inherited from base"
        );
        assert_eq!(
            active.palette.primary,
            create_test_theme("base").palette.primary
        );
    }
}
