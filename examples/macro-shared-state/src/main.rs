//! Macro-based Shared State Example
//!
//! Demonstrates using the `shared_model` attribute with #[dampen_app]
//! to automatically manage shared state across multiple views.
//!
//! Compare with manual setup to see how much boilerplate is eliminated!

// Allow cfg warnings from dampen_ui macro (internal feature checks)
#![allow(unexpected_cfgs)]

mod shared;
mod ui;

use dampen_iced::HandlerMessage;
use dampen_macros::dampen_app;

#[cfg(debug_assertions)]
use dampen_dev::FileEvent;

#[derive(Clone, Debug)]
enum Message {
    SwitchToView(CurrentView),
    Handler(HandlerMessage),
    #[cfg(debug_assertions)]
    HotReload(FileEvent),
    #[cfg(debug_assertions)]
    DismissError,
}

/// Automatic shared state management with the macro!
#[dampen_app(
    ui_dir = "src/ui",
    message_type = "Message",
    handler_variant = "Handler",
    hot_reload_variant = "HotReload",
    dismiss_error_variant = "DismissError",
    switch_view_variant = "SwitchToView",
    default_view = "window",
    shared_model = "SharedState"  // ← The magic happens here!
)]
struct MacroSharedStateApp;

pub fn main() -> iced::Result {
    #[cfg(debug_assertions)]
    println!("🔥 Hot-reload enabled with persistent shared state!");

    println!("✨ Using #[dampen_app] macro with shared_model attribute");
    println!("📝 Try modifying the theme or username in the Settings view");
    println!("🔄 Changes persist across view switching and hot-reload!");

    iced::application(
        MacroSharedStateApp::init,
        MacroSharedStateApp::update,
        MacroSharedStateApp::view,
    )
    .window_size(iced::Size::new(500.0, 600.0))
    .centered()
    .title("Shared State with Macro")
    .subscription(MacroSharedStateApp::subscription)
    .run()
}
