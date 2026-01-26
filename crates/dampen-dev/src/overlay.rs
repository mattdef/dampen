//! Error overlay UI components for displaying parse errors
//!
//! This module provides UI widgets for displaying error overlays during
//! hot-reload when XML parsing or validation fails.

use dampen_core::parser::error::ParseError;
use iced::{
    Alignment, Color, Element, Length,
    widget::{button, column, container, text},
};
use std::time::Instant;

/// UI state for displaying parse errors during hot-reload
#[derive(Debug, Clone)]
pub struct ErrorOverlay {
    /// Parse error details
    pub error: Option<ParseError>,

    /// Whether overlay is visible
    pub visible: bool,

    /// Timestamp when error occurred
    pub timestamp: Instant,
}

impl ErrorOverlay {
    /// Create a new error overlay (initially hidden)
    pub fn new() -> Self {
        Self {
            error: None,
            visible: false,
            timestamp: Instant::now(),
        }
    }

    /// Show the overlay with an error
    ///
    /// # Arguments
    /// * `error` - The parse error to display
    pub fn show(&mut self, error: ParseError) {
        self.error = Some(error);
        self.visible = true;
        self.timestamp = Instant::now();
    }

    /// Hide the overlay
    pub fn hide(&mut self) {
        self.visible = false;
    }

    /// Check if the overlay is currently visible
    pub fn is_visible(&self) -> bool {
        self.visible
    }

    /// Render the error overlay as an Iced widget
    ///
    /// Returns a full-screen overlay with error details and a dismiss button.
    /// If the overlay is not visible, returns an empty container.
    ///
    /// # Type Parameters
    /// * `Message` - Application message type that must have a variant to dismiss the overlay
    ///
    /// # Arguments
    /// * `on_dismiss` - Message to send when the dismiss button is clicked
    ///
    /// # Example
    /// ```ignore
    /// let overlay = ErrorOverlay::new();
    /// let widget = overlay.render(Message::DismissError);
    /// ```
    pub fn render<'a, Message: Clone + 'a>(&'a self, on_dismiss: Message) -> Element<'a, Message> {
        if !self.visible {
            return container(text("")).into();
        }

        let error = match &self.error {
            Some(e) => e,
            None => return container(text("")).into(),
        };

        // Title
        let title = text("Hot-Reload Error")
            .size(24)
            .style(|_theme| text::Style {
                color: Some(Color::WHITE),
            });

        // Error message
        let message = text(&error.message).size(16).style(|_theme| text::Style {
            color: Some(Color::WHITE),
        });

        // Location info
        let location = text(format!(
            "at line {}, column {}",
            error.span.line, error.span.column
        ))
        .size(14)
        .style(|_theme| text::Style {
            color: Some(Color::from_rgb(0.9, 0.9, 0.9)),
        });

        // Suggestion (if available)
        let suggestion_widget = if let Some(ref suggestion) = error.suggestion {
            let label = text(format!("💡 {}", suggestion))
                .size(14)
                .style(|_theme| text::Style {
                    color: Some(Color::from_rgb(1.0, 1.0, 0.6)),
                });
            Some(label)
        } else {
            None
        };

        // Dismiss button
        let dismiss_btn = button(text("Dismiss (Esc)").size(14).style(|_theme| text::Style {
            color: Some(Color::BLACK),
        }))
        .on_press(on_dismiss)
        .padding(10);

        // Build content column
        let mut content = column![title, message, location]
            .spacing(12)
            .align_x(Alignment::Start);

        if let Some(suggestion) = suggestion_widget {
            content = content.push(suggestion);
        }

        content = content.push(dismiss_btn);

        // Wrap in red container with padding
        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(40)
            .style(|_theme| container::Style {
                background: Some(Color::from_rgb(0.8, 0.2, 0.2).into()),
                text_color: Some(Color::WHITE),
                ..Default::default()
            })
            .into()
    }
}

impl Default for ErrorOverlay {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dampen_core::ir::span::Span;
    use dampen_core::parser::error::ParseErrorKind;

    #[derive(Debug, Clone)]
    enum TestMessage {
        Dismiss,
    }

    #[test]
    fn test_new_overlay_is_hidden() {
        let overlay = ErrorOverlay::new();
        assert!(!overlay.is_visible());
        assert!(overlay.error.is_none());
    }

    #[test]
    fn test_show_makes_overlay_visible() {
        let mut overlay = ErrorOverlay::new();
        let error = ParseError {
            kind: ParseErrorKind::XmlSyntax,
            message: "Test error".to_string(),
            span: Span {
                start: 0,
                end: 5,
                line: 1,
                column: 5,
            },
            suggestion: None,
        };

        overlay.show(error.clone());
        assert!(overlay.is_visible());
        assert_eq!(overlay.error, Some(error));
    }

    #[test]
    fn test_hide_makes_overlay_invisible() {
        let mut overlay = ErrorOverlay::new();
        let error = ParseError {
            kind: ParseErrorKind::XmlSyntax,
            message: "Test error".to_string(),
            span: Span {
                start: 0,
                end: 5,
                line: 1,
                column: 5,
            },
            suggestion: None,
        };

        overlay.show(error);
        overlay.hide();
        assert!(!overlay.is_visible());
        // Error is preserved even when hidden
        assert!(overlay.error.is_some());
    }

    #[test]
    fn test_render_returns_empty_when_hidden() {
        let overlay = ErrorOverlay::new();
        let element = overlay.render(TestMessage::Dismiss);
        // We can't easily inspect the Element structure, but we can verify it compiles
        // and returns successfully
        drop(element);
    }

    #[test]
    fn test_render_with_visible_error() {
        let mut overlay = ErrorOverlay::new();
        let error = ParseError {
            kind: ParseErrorKind::UnknownWidget,
            message: "Unknown widget 'foo'".to_string(),
            span: Span {
                start: 50,
                end: 53,
                line: 10,
                column: 15,
            },
            suggestion: Some("Did you mean 'button'?".to_string()),
        };

        overlay.show(error);
        let element = overlay.render(TestMessage::Dismiss);
        // Verify the element is created successfully
        drop(element);
    }

    #[test]
    fn test_render_without_suggestion() {
        let mut overlay = ErrorOverlay::new();
        let error = ParseError {
            kind: ParseErrorKind::InvalidValue,
            message: "Invalid value".to_string(),
            span: Span {
                start: 100,
                end: 105,
                line: 5,
                column: 20,
            },
            suggestion: None,
        };

        overlay.show(error);
        let element = overlay.render(TestMessage::Dismiss);
        // Verify the element is created successfully even without suggestion
        drop(element);
    }

    #[test]
    fn test_timestamp_updated_on_show() {
        let mut overlay = ErrorOverlay::new();
        let initial_timestamp = overlay.timestamp;

        // Wait a tiny bit to ensure timestamp changes
        std::thread::sleep(std::time::Duration::from_millis(1));

        let error = ParseError {
            kind: ParseErrorKind::XmlSyntax,
            message: "Test".to_string(),
            span: Span {
                start: 0,
                end: 1,
                line: 1,
                column: 1,
            },
            suggestion: None,
        };

        overlay.show(error);
        assert!(overlay.timestamp > initial_timestamp);
    }
}
