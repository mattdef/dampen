//! Todo App example using the #[dampen_app] macro for automatic view management.

mod shared;
mod ui;

use dampen_iced::HandlerMessage;
use dampen_macros::dampen_app;

#[cfg(debug_assertions)]
use dampen_dev::FileEvent;

/// Application messages
#[derive(Clone, Debug)]
enum Message {
    /// Switch between views (window ↔ statistics)
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
    shared_model = "SharedState",
    default_view = "window"
)]
struct TodoApp;

pub fn main() -> iced::Result {
    #[cfg(debug_assertions)]
    println!("🔥 Hot-reload enabled! Edit src/ui/window.dampen to see live updates.");

    println!("📊 Multi-view enabled: Main window ↔ Statistics");
    println!("   Click '📊 Statistics' to view real-time task analytics!");

    iced::application(TodoApp::init, TodoApp::update, TodoApp::view)
        .window_size(iced::Size::new(1280.0, 800.0))
        .centered()
        .subscription(TodoApp::subscription)
        .run()
}
