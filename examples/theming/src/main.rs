//! Theming example showcasing all theme features.
//! - Runtime theme switching
//! - Theme inheritance
//! - Custom themes
//! - Widget-level overrides
//! - System theme detection

mod ui;

use dampen_iced::HandlerMessage;
use dampen_macros::dampen_app;

#[cfg(debug_assertions)]
use dampen_dev::FileEvent;

/// Application messages
#[derive(Clone, Debug)]
enum Message {
    /// Handler invocation from UI widgets
    Handler(HandlerMessage),
    /// Hot-reload event (development mode only)
    #[cfg(debug_assertions)]
    HotReload(FileEvent),
    /// Dismiss error overlay
    #[cfg(debug_assertions)]
    DismissError,
    /// System theme change
    SystemThemeChanged(String),
}

/// Main application structure with auto-generated view management
#[dampen_app(
    ui_dir = "src/ui",
    message_type = "Message",
    handler_variant = "Handler",
    hot_reload_variant = "HotReload",
    dismiss_error_variant = "DismissError",
    system_theme_variant = "SystemThemeChanged",
    exclude = ["theme/*"]
)]
struct ThemingApp;

pub fn main() -> iced::Result {
    #[cfg(debug_assertions)]
    println!("Theming example - Edit src/ui/theme/theme.dampen to see hot-reload!");

    iced::application(ThemingApp::init, ThemingApp::update, ThemingApp::view)
        .theme(ThemingApp::theme)
        .subscription(ThemingApp::subscription)
        .run()
}
