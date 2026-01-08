//! Error overlay widget for displaying parse and binding errors

use gravity_core::{BindingError, BindingErrorKind, ParseError};
use std::fmt::Write;

/// Overlay widget displaying errors
#[derive(Debug, Clone)]
pub struct ErrorOverlay {
    /// Overlay title
    pub title: String,

    /// Main error message
    pub message: String,

    /// Source location information
    pub location: Option<String>,

    /// Suggestion for fixing the error
    pub suggestion: Option<String>,

    /// Whether the overlay is visible
    pub visible: bool,
}

impl ErrorOverlay {
    /// Create overlay from a parse error
    pub fn from_parse_error(error: &ParseError) -> Self {
        let mut message = String::new();
        let _ = write!(message, "{}", error.message);

        let location = Some(format!(
            "Line {}, Column {}",
            error.span.line, error.span.column
        ));

        Self {
            title: "Parse Error".to_string(),
            message,
            location,
            suggestion: error.suggestion.clone(),
            visible: true,
        }
    }

    /// Create overlay from a binding error
    pub fn from_binding_error(error: &BindingError) -> Self {
        let mut message = String::new();
        let _ = write!(message, "{}", error.message);

        let location = Some(format!(
            "Line {}, Column {}",
            error.span.line, error.span.column
        ));

        let title = match error.kind {
            BindingErrorKind::UnknownField => "Unknown Field".to_string(),
            BindingErrorKind::TypeMismatch => "Type Mismatch".to_string(),
            BindingErrorKind::UnknownMethod => "Unknown Method".to_string(),
            BindingErrorKind::InvalidOperation => "Invalid Operation".to_string(),
        };

        Self {
            title,
            message,
            location,
            suggestion: error.suggestion.clone(),
            visible: true,
        }
    }

    /// Create a generic error overlay
    pub fn new(title: &str, message: &str) -> Self {
        Self {
            title: title.to_string(),
            message: message.to_string(),
            location: None,
            suggestion: None,
            visible: true,
        }
    }

    /// Dismiss the overlay (hide it)
    pub fn dismiss(&mut self) {
        self.visible = false;
    }

    /// Show the overlay
    pub fn show(&mut self) {
        self.visible = true;
    }

    /// Check if overlay is currently visible
    pub fn is_visible(&self) -> bool {
        self.visible
    }

    /// Format the overlay content for display
    pub fn format_content(&self) -> String {
        let mut output = String::new();

        // Title
        let _ = writeln!(
            &mut output,
            "╔═══════════════════════════════════════════════════════════╗"
        );
        let _ = writeln!(&mut output, "║  {}  ", self.title);
        let _ = writeln!(
            &mut output,
            "╚═══════════════════════════════════════════════════════════╝"
        );
        let _ = writeln!(&mut output);

        // Location
        if let Some(loc) = &self.location {
            let _ = writeln!(&mut output, "Location: {}", loc);
            let _ = writeln!(&mut output);
        }

        // Message
        let _ = writeln!(&mut output, "Error: {}", self.message);
        let _ = writeln!(&mut output);

        // Suggestion
        if let Some(sugg) = &self.suggestion {
            let _ = writeln!(&mut output, "Suggestion: {}", sugg);
            let _ = writeln!(&mut output);
        }

        // Dismiss hint
        let _ = writeln!(&mut output, "[Press ESC or click to dismiss]");

        output
    }
}

/// Overlay manager for handling multiple errors
#[derive(Debug, Clone, Default)]
pub struct OverlayManager {
    overlays: Vec<ErrorOverlay>,
}

impl OverlayManager {
    /// Add a new overlay
    pub fn add(&mut self, overlay: ErrorOverlay) {
        self.overlays.push(overlay);
    }

    /// Add overlay from parse error
    pub fn add_parse_error(&mut self, error: &ParseError) {
        self.add(ErrorOverlay::from_parse_error(error));
    }

    /// Add overlay from binding error
    pub fn add_binding_error(&mut self, error: &BindingError) {
        self.add(ErrorOverlay::from_binding_error(error));
    }

    /// Get the most recent visible overlay
    pub fn current(&self) -> Option<&ErrorOverlay> {
        self.overlays.iter().rev().find(|o| o.visible)
    }

    /// Get mutable reference to current overlay
    pub fn current_mut(&mut self) -> Option<&mut ErrorOverlay> {
        self.overlays.iter_mut().rev().find(|o| o.visible)
    }

    /// Dismiss the current overlay
    pub fn dismiss_current(&mut self) {
        if let Some(overlay) = self.current_mut() {
            overlay.dismiss();
        }
    }

    /// Clear all overlays
    pub fn clear(&mut self) {
        self.overlays.clear();
    }

    /// Check if any overlay is visible
    pub fn has_visible(&self) -> bool {
        self.overlays.iter().any(|o| o.visible)
    }

    /// Get all visible overlays
    pub fn visible_overlays(&self) -> Vec<&ErrorOverlay> {
        self.overlays.iter().filter(|o| o.visible).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gravity_core::Span;

    #[test]
    fn test_parse_error_overlay() {
        let error = ParseError {
            kind: gravity_core::ParseErrorKind::UnknownWidget,
            message: "Unknown widget: <buton>".to_string(),
            span: Span::new(10, 20, 5, 12),
            suggestion: Some("Did you mean: <button>?".to_string()),
        };

        let overlay = ErrorOverlay::from_parse_error(&error);

        assert!(overlay.title.contains("Parse Error"));
        assert!(overlay.message.contains("Unknown widget"));
        assert!(overlay.location.is_some());
        assert!(overlay.suggestion.is_some());
        assert!(overlay.visible);
    }

    #[test]
    fn test_binding_error_overlay() {
        let error = BindingError {
            kind: BindingErrorKind::UnknownField,
            message: "Field 'counter' not found".to_string(),
            span: Span::new(0, 10, 1, 1),
            suggestion: Some("Available fields: count, name".to_string()),
        };

        let overlay = ErrorOverlay::from_binding_error(&error);

        assert!(overlay.title.contains("Unknown Field"));
        assert!(overlay.message.contains("counter"));
        assert!(overlay.visible);
    }

    #[test]
    fn test_overlay_dismissal() {
        let mut overlay = ErrorOverlay::new("Test", "Test error");
        assert!(overlay.is_visible());

        overlay.dismiss();
        assert!(!overlay.is_visible());

        overlay.show();
        assert!(overlay.is_visible());
    }

    #[test]
    fn test_overlay_manager() {
        let mut manager = OverlayManager::default();

        // Add multiple overlays
        manager.add(ErrorOverlay::new("Error 1", "First error"));
        manager.add(ErrorOverlay::new("Error 2", "Second error"));

        assert!(manager.has_visible());
        assert_eq!(manager.visible_overlays().len(), 2);

        // Dismiss current
        manager.dismiss_current();

        // Should still have one visible
        assert!(manager.has_visible());

        // Clear all
        manager.clear();
        assert!(!manager.has_visible());
    }

    #[test]
    fn test_format_content() {
        let overlay = ErrorOverlay {
            title: "Parse Error".to_string(),
            message: "Syntax error".to_string(),
            location: Some("Line 5, Column 12".to_string()),
            suggestion: Some("Add closing tag".to_string()),
            visible: true,
        };

        let content = overlay.format_content();
        assert!(content.contains("Parse Error"));
        assert!(content.contains("Syntax error"));
        assert!(content.contains("Line 5, Column 12"));
        assert!(content.contains("Add closing tag"));
    }
}
