//! Template loading and rendering logic.

/// Type of window template.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TemplateKind {
    /// Rust module template (.rs file)
    RustModule,
    /// Dampen XML template (.dampen file)
    DampenXml,
}

/// A window file template with placeholder replacement.
#[derive(Debug, Clone)]
pub struct WindowTemplate {
    /// Template content with placeholders
    pub content: String,
    /// Template type
    pub kind: TemplateKind,
}

/// Window name variants for template rendering.
///
/// This is a simplified version for template rendering.
/// The full WindowName with validation will be in validation.rs.
#[derive(Debug, Clone)]
pub struct WindowNameVariants {
    /// snake_case representation (e.g., "user_profile")
    pub snake: String,
    /// PascalCase representation (e.g., "UserProfile")
    pub pascal: String,
    /// Title Case representation (e.g., "User Profile")
    pub title: String,
}

impl WindowTemplate {
    /// Load a template from embedded resources.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use dampen_cli::commands::add::templates::{WindowTemplate, TemplateKind};
    ///
    /// let template = WindowTemplate::load(TemplateKind::RustModule);
    /// assert!(!template.content.is_empty());
    /// ```
    pub fn load(kind: TemplateKind) -> Self {
        let content = match kind {
            TemplateKind::RustModule => {
                include_str!("../../../templates/add/window.rs.template")
            }
            TemplateKind::DampenXml => {
                include_str!("../../../templates/add/window.dampen.template")
            }
        };

        Self {
            content: content.to_string(),
            kind,
        }
    }

    /// Render the template by replacing placeholders with actual values.
    ///
    /// # Placeholders
    ///
    /// - `{{WINDOW_NAME}}` - replaced with snake_case name
    /// - `{{WINDOW_NAME_PASCAL}}` - replaced with PascalCase name
    /// - `{{WINDOW_NAME_TITLE}}` - replaced with Title Case name
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use dampen_cli::commands::add::templates::{WindowTemplate, TemplateKind, WindowNameVariants};
    ///
    /// let template = WindowTemplate::load(TemplateKind::RustModule);
    /// let names = WindowNameVariants {
    ///     snake: "user_profile".to_string(),
    ///     pascal: "UserProfile".to_string(),
    ///     title: "User Profile".to_string(),
    /// };
    /// let rendered = template.render(&names);
    /// assert!(rendered.contains("user_profile"));
    /// ```
    pub fn render(&self, window_name: &WindowNameVariants) -> String {
        self.content
            .replace("{{WINDOW_NAME}}", &window_name.snake)
            .replace("{{WINDOW_NAME_PASCAL}}", &window_name.pascal)
            .replace("{{WINDOW_NAME_TITLE}}", &window_name.title)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_template_load_rust_module() {
        let template = WindowTemplate::load(TemplateKind::RustModule);
        assert!(!template.content.is_empty());
        assert_eq!(template.kind, TemplateKind::RustModule);
        // Verify it contains expected placeholders
        assert!(template.content.contains("{{WINDOW_NAME}}"));
        // Note: Current template only uses {{WINDOW_NAME}}, not separate pascal/title variants
        assert!(template.content.contains("use dampen_core"));
        assert!(template.content.contains("Model"));
    }

    #[test]
    fn test_template_load_dampen_xml() {
        let template = WindowTemplate::load(TemplateKind::DampenXml);
        assert!(!template.content.is_empty());
        assert_eq!(template.kind, TemplateKind::DampenXml);
        // Verify it contains expected placeholders
        assert!(template.content.contains("{{WINDOW_NAME_TITLE}}"));
    }

    #[test]
    fn test_template_render_snake_case() {
        let template = WindowTemplate {
            content: "File: {{WINDOW_NAME}}.rs".to_string(),
            kind: TemplateKind::RustModule,
        };
        let names = WindowNameVariants {
            snake: "user_profile".to_string(),
            pascal: "UserProfile".to_string(),
            title: "User Profile".to_string(),
        };
        let rendered = template.render(&names);
        assert_eq!(rendered, "File: user_profile.rs");
    }

    #[test]
    fn test_template_render_pascal_case() {
        let template = WindowTemplate {
            content: "struct {{WINDOW_NAME_PASCAL}} {}".to_string(),
            kind: TemplateKind::RustModule,
        };
        let names = WindowNameVariants {
            snake: "user_profile".to_string(),
            pascal: "UserProfile".to_string(),
            title: "User Profile".to_string(),
        };
        let rendered = template.render(&names);
        assert_eq!(rendered, "struct UserProfile {}");
    }

    #[test]
    fn test_template_render_title_case() {
        let template = WindowTemplate {
            content: "<text>{{WINDOW_NAME_TITLE}}</text>".to_string(),
            kind: TemplateKind::DampenXml,
        };
        let names = WindowNameVariants {
            snake: "user_profile".to_string(),
            pascal: "UserProfile".to_string(),
            title: "User Profile".to_string(),
        };
        let rendered = template.render(&names);
        assert_eq!(rendered, "<text>User Profile</text>");
    }

    #[test]
    fn test_template_render_all_variants() {
        let template = WindowTemplate {
            content: "{{WINDOW_NAME}} {{WINDOW_NAME_PASCAL}} {{WINDOW_NAME_TITLE}}".to_string(),
            kind: TemplateKind::RustModule,
        };
        let names = WindowNameVariants {
            snake: "test_window".to_string(),
            pascal: "TestWindow".to_string(),
            title: "Test Window".to_string(),
        };
        let rendered = template.render(&names);
        assert_eq!(rendered, "test_window TestWindow Test Window");
    }

    #[test]
    fn test_template_render_multiple_occurrences() {
        let template = WindowTemplate {
            content: "{{WINDOW_NAME}} and {{WINDOW_NAME}} again".to_string(),
            kind: TemplateKind::RustModule,
        };
        let names = WindowNameVariants {
            snake: "settings".to_string(),
            pascal: "Settings".to_string(),
            title: "Settings".to_string(),
        };
        let rendered = template.render(&names);
        assert_eq!(rendered, "settings and settings again");
    }
}
