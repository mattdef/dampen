//! Widget showcase demonstrating all Dampen UI widgets.
//!
//! This example shows all currently supported widgets in Dampen.

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
}

/// Main application structure with auto-generated view management
#[dampen_app(
    ui_dir = "src/ui",
    message_type = "Message",
    handler_variant = "Handler",
    hot_reload_variant = "HotReload",
    dismiss_error_variant = "DismissError",
    default_view = "window"
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
