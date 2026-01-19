//! Theming example showcasing all theme features.
//! - Runtime theme switching
//! - Theme inheritance
//! - Custom themes
//! - Widget-level overrides
//! - System theme detection

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
    /// Set theme explicitly
    SetTheme(String),
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
    exclude = ["theme/*"]
)]
struct ThemingApp;

#[cfg(feature = "interpreted")]
pub fn main() -> iced::Result {
    #[cfg(debug_assertions)]
    println!("Theming example - Edit src/ui/theme/theme.dampen to see hot-reload!");

    iced::application(ThemingApp::init, ThemingApp::update, ThemingApp::view)
        .theme(ThemingApp::theme)
        .subscription(ThemingApp::subscription)
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
    println!("🚀 Running theming in codegen mode (production)");

    iced::application(window::new_model, window::update_model, window::view_model)
        .theme(window::theme)
        .subscription(|_model| window::subscription_model())
        .run()
}
