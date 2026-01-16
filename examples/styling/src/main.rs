//! Styling example using the #[dampen_app] macro for automatic view management.

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
struct StylingApp;

pub fn main() -> iced::Result {
    #[cfg(debug_assertions)]
    println!("🔥 Hot-reload enabled! Edit src/ui/window.dampen to see live updates.");

    iced::application(StylingApp::init, StylingApp::update, StylingApp::view)
        .theme(StylingApp::theme)
        .subscription(StylingApp::subscription)
        .run()
}
