// Ensure codegen and interpreted are mutually exclusive
#[cfg(all(feature = "codegen", feature = "interpreted"))]
compile_error!(
    "Features 'codegen' and 'interpreted' are mutually exclusive. Use --no-default-features with --features codegen for production builds."
);

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
    exclude = ["theme/*"],
)]
struct App;

#[cfg(feature = "interpreted")]
pub fn main() -> iced::Result {
    iced::application(App::init, App::update, App::view).run()
}

// Codegen mode placeholder (requires build.rs generation which I set up but didn't verify details)
#[cfg(all(feature = "codegen", not(feature = "interpreted")))]
mod ui;

#[cfg(all(feature = "codegen", not(feature = "interpreted")))]
include!(concat!(env!("OUT_DIR"), "/ui_generated.rs"));

#[cfg(all(feature = "codegen", not(feature = "interpreted")))]
pub fn main() -> iced::Result {
    iced::application(window::new_model, window::update_model, window::view_model).run()
}
