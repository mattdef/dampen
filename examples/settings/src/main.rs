//! Settings example demonstrating multiple UI views.
//!
//! This example demonstrates how to switch between multiple UI views
//! (main and settings) at runtime using the #[dampen_app] macro.

mod ui;

use dampen_iced::HandlerMessage;
use dampen_macros::dampen_app;

#[cfg(debug_assertions)]
use dampen_dev::FileEvent;

/// Application messages
#[derive(Clone, Debug)]
enum Message {
    /// View switching
    SwitchToView(CurrentView),
    /// Handler invocation from UI widgets
    Handler(HandlerMessage),
    /// Hot-reload event (development mode only)
    #[cfg(debug_assertions)]
    HotReload(FileEvent),
    /// Dismiss error overlay
    #[cfg(debug_assertions)]
    DismissError,
}

/// Main application structure with auto-generated view management
#[dampen_app(
    ui_dir = "src/ui",
    message_type = "Message",
    handler_variant = "Handler",
    hot_reload_variant = "HotReload",
    dismiss_error_variant = "DismissError",
    switch_view_variant = "SwitchToView",
    default_view = "window"
)]
struct SettingsApp;

pub fn main() -> iced::Result {
    #[cfg(debug_assertions)]
    println!("🔥 Hot-reload enabled! Edit src/ui/*.dampen files to see live updates.");

    iced::application(SettingsApp::init, SettingsApp::update, SettingsApp::view)
        .window_size(iced::Size::new(400.0, 250.0))
        .centered()
        .subscription(SettingsApp::subscription)
        .run()
}
