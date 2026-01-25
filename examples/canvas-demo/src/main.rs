//! Canvas Demo example

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

#[cfg(feature = "interpreted")]
#[derive(Clone, Debug)]
enum Message {
    Handler(HandlerMessage),
    #[cfg(debug_assertions)]
    HotReload(FileEvent),
    #[cfg(debug_assertions)]
    DismissError,
    SystemThemeChanged(String),
    Window(iced::window::Id, iced::window::Event),
}

#[cfg(feature = "interpreted")]
#[dampen_app(
    ui_dir = "src/ui",
    message_type = "Message",
    handler_variant = "Handler",
    hot_reload_variant = "HotReload",
    dismiss_error_variant = "DismissError",
    system_theme_variant = "SystemThemeChanged",
    default_view = "window",
    app_name = "canvas-demo",
    persistence = true
)]
struct DampenApp;

#[cfg(feature = "interpreted")]
pub fn main() -> iced::Result {
    #[cfg(debug_assertions)]
    println!("🔥 Hot-reload enabled! Edit src/ui/*.dampen files to see live updates.");

    iced::application(DampenApp::init, DampenApp::update, DampenApp::view)
        .window(
            DampenApp::window_settings()
                .default_size(500, 400)
                .min_size(350, 300)
                .build(),
        )
        .theme(DampenApp::theme)
        .title("Dampen Canvas Demo")
        .subscription(DampenApp::subscription)
        .exit_on_close_request(false)
        .run()
}

// ============================================================================
// CODEGEN MODE (production builds with pre-generated code)
// ============================================================================
#[cfg(all(feature = "codegen", not(feature = "interpreted")))]
mod ui;

#[cfg(all(feature = "codegen", not(feature = "interpreted")))]
include!(concat!(env!("OUT_DIR"), "/ui_generated.rs"));

#[cfg(all(feature = "codegen", not(feature = "interpreted")))]
pub fn main() -> iced::Result {
    println!("🚀 Running in codegen mode (production)");

    iced::application(window::new_model, window::update_model, window::view_model)
        .window(
            window::window_settings()
                .default_size(500, 400)
                .min_size(400, 300)
                .build(),
        )
        .title("Dampen Canvas Demo")
        .subscription(window::subscription_model)
        .exit_on_close_request(false)
        .run()
}
