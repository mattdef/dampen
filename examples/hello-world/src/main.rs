//! Hello World example using the auto-loading pattern.
//!
//! This example demonstrates how to create a Dampen application
//! with minimal boilerplate using the AppState pattern.
//!
//! - In **interpreted mode** (default): Uses `#[dampen_app]` macro with hot-reload
//! - In **codegen mode**: Uses pre-generated code for production builds

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

/// Main application structure with auto-generated view management (interpreted mode)
#[cfg(feature = "interpreted")]
#[dampen_app(
    ui_dir = "src/ui",
    message_type = "Message",
    handler_variant = "Handler",
    hot_reload_variant = "HotReload",
    dismiss_error_variant = "DismissError",
    system_theme_variant = "SystemThemeChanged",
    default_view = "window",
    exclude = ["theme/*"],
)]
struct DampenApp;

#[cfg(feature = "interpreted")]
pub fn main() -> iced::Result {
    use iced::{Size, window};

    #[cfg(debug_assertions)]
    println!("🔥 Hot-reload enabled! Edit src/ui/*.dampen files to see live updates.");

    #[cfg(not(debug_assertions))]
    println!("🚀 Running in interpreted release mode.");

    iced::application(DampenApp::init, DampenApp::update, DampenApp::view)
        .window(window::Settings {
            size: Size::new(400.0, 300.0),
            min_size: Some(Size::new(400.0, 300.0)),
            resizable: true,
            ..Default::default()
        })
        .centered()
        .theme(DampenApp::theme)
        .title("Dampen Hello World!")
        .subscription(DampenApp::subscription)
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
        .window_size((400.0, 300.0))
        .centered()
        .theme(window::theme)
        .title("Dampen Hello World!")
        .subscription(|_model| window::subscription_model())
        .run()
}
