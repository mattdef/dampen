//! Widget showcase demonstrating all Dampen UI widgets.
//!
//! This example shows all currently supported widgets in Dampen.
//!
//! **Note:** This example currently only supports interpreted mode (default).
//! Codegen mode is not yet supported for multi-view applications.

mod ui;

use dampen_iced::HandlerMessage;
use dampen_macros::dampen_app;

#[cfg(all(debug_assertions, feature = "interpreted"))]
use dampen_dev::FileEvent;

/// Application messages
#[derive(Clone, Debug)]
enum Message {
    /// Handler invocation from UI widgets
    Handler(HandlerMessage),
    /// Switch to a different view
    SwitchToView(CurrentView),
    /// Hot-reload event (development mode only)
    #[cfg(all(debug_assertions, feature = "interpreted"))]
    HotReload(FileEvent),
    /// Dismiss error overlay
    #[cfg(all(debug_assertions, feature = "interpreted"))]
    DismissError,
    /// System theme change
    SystemThemeChanged(String),
}

/// Main application structure with auto-generated view management
#[dampen_app(
    ui_dir = "src/ui",
    message_type = "Message",
    handler_variant = "Handler",
    switch_view_variant = "SwitchToView",
    hot_reload_variant = "HotReload",
    dismiss_error_variant = "DismissError",
    system_theme_variant = "SystemThemeChanged",
    default_view = "window",
    exclude = ["theme/*"],
)]
struct ShowcaseApp;

pub fn main() -> iced::Result {
    println!("🔥 Hot-reload enabled! Edit src/ui/*.dampen files to see live updates.");

    iced::application(ShowcaseApp::init, ShowcaseApp::update, ShowcaseApp::view)
        .window_size(iced::Size::new(1024.0, 800.0))
        .centered()
        .subscription(ShowcaseApp::subscription)
        .run()
}
