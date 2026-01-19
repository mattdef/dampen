//! Settings example demonstrating multiple UI views.
//!
//! This example demonstrates how to switch between multiple UI views
//! (main and settings) at runtime using the #[dampen_app] macro.
//!
//! **Note:** This example currently only supports interpreted mode (default).
//! Codegen mode is not yet supported for multi-view applications.

// Ensure codegen and interpreted are mutually exclusive
#[cfg(all(feature = "codegen", feature = "interpreted"))]
compile_error!(
    "Features 'codegen' and 'interpreted' are mutually exclusive. Use --no-default-features with --features codegen for production builds."
);

// ============================================================================
// INTERPRETED MODE (development with hot-reload)
// ============================================================================
#[cfg(feature = "interpreted")]
mod ui;

#[cfg(feature = "interpreted")]
use dampen_iced::HandlerMessage;
#[cfg(feature = "interpreted")]
use dampen_macros::dampen_app;

#[cfg(all(feature = "interpreted", debug_assertions))]
use dampen_dev::FileEvent;

/// Application messages (interpreted mode)
#[cfg(feature = "interpreted")]
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

/// Main application structure with auto-generated view management (interpreted mode)
#[cfg(feature = "interpreted")]
#[dampen_app(
    ui_dir = "src/ui",
    message_type = "Message",
    handler_variant = "Handler",
    hot_reload_variant = "HotReload",
    dismiss_error_variant = "DismissError",
    switch_view_variant = "SwitchToView",
    default_view = "window",
    exclude = ["theme/*"],
)]
struct SettingsApp;

#[cfg(feature = "interpreted")]
pub fn main() -> iced::Result {
    #[cfg(debug_assertions)]
    println!("🔥 Hot-reload enabled! Edit src/ui/*.dampen files to see live updates.");

    #[cfg(not(debug_assertions))]
    println!("🚀 Running in interpreted release mode.");

    iced::application(SettingsApp::init, SettingsApp::update, SettingsApp::view)
        .window_size(iced::Size::new(400.0, 250.0))
        .centered()
        .subscription(SettingsApp::subscription)
        .run()
}

// ============================================================================
// CODEGEN MODE (production builds with pre-generated code)
// ============================================================================
#[cfg(all(feature = "codegen", not(feature = "interpreted")))]
mod ui;

// Include the generated code (contains Message enum, update/view functions, etc.)
#[cfg(all(feature = "codegen", not(feature = "interpreted")))]
include!(concat!(env!("OUT_DIR"), "/ui_generated.rs"));

#[cfg(all(feature = "codegen", not(feature = "interpreted")))]
pub fn main() -> iced::Result {
    println!("🚀 Running in codegen mode (production)");

    iced::application(window::new_model, window::update_model, window::view_model)
        .window_size(iced::Size::new(400.0, 250.0))
        .centered()
        .subscription(|_model| window::subscription_model())
        .run()
}
