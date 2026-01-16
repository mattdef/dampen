//! Hello World example using the auto-loading pattern.
//!
//! This example demonstrates how to create a Gravity application
//! with minimal boilerplate using the new AppState pattern.
mod ui;

use dampen_iced::HandlerMessage;
use dampen_macros::dampen_app;

#[cfg(debug_assertions)]
use dampen_dev::FileEvent;

/// Application messages
#[derive(Clone, Debug)]
enum Message {
    /// Uncoment to enable view switching
    // SwitchToView(CurrentView),
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
    default_view = "window",
    exclude = ["theme/*"],
    // Uncomment to enable view switching
    // switch_view_variant = "SwitchToView",
)]
struct DampenApp;

pub fn main() -> iced::Result {
    println!("🔥 Hot-reload enabled! Edit src/ui/*.dampen files to see live updates.");

    iced::application(DampenApp::init, DampenApp::update, DampenApp::view)
        .window_size(iced::Size::new(400.0, 300.0))
        .centered()
        .title("Dampen Hello World!")
        .subscription(DampenApp::subscription)
        .run()
}
