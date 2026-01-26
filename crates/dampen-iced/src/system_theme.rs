//! System theme detection for production builds
//!
//! This module provides system theme change detection using Iced's native
//! theme subscription, which works in both debug and release builds.

use iced::Subscription;

/// Watch for system theme changes.
///
/// Returns a subscription that emits "light" or "dark" strings when the
/// operating system theme preference changes.
///
/// This function uses Iced's native `system::theme_changes()` subscription
/// and is safe to use in production (release) builds.
///
/// # Returns
///
/// A subscription that emits:
/// - `"light"` when the system theme changes to light mode
/// - `"dark"` when the system theme changes to dark mode
///
/// # Example
///
/// ```ignore
/// use dampen_iced::watch_system_theme;
///
/// fn subscription(&self) -> iced::Subscription<Message> {
///     watch_system_theme().map(Message::SystemThemeChanged)
/// }
/// ```
pub fn watch_system_theme() -> Subscription<String> {
    iced::system::theme_changes().map(|mode| match mode {
        iced::theme::Mode::Light => "light".to_string(),
        iced::theme::Mode::Dark => "dark".to_string(),
        iced::theme::Mode::None => "light".to_string(),
    })
}
