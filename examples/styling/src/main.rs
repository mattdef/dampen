//! Styling example using the #[dampen_app] macro for automatic view management.

#[cfg(feature = "interpreted")]
mod ui;

#[cfg(feature = "interpreted")]
use dampen_iced::HandlerMessage;
#[cfg(feature = "interpreted")]
use dampen_macros::dampen_app;

#[cfg(all(feature = "interpreted", debug_assertions))]
use dampen_dev::FileEvent;

/// Application messages
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

/// Main application structure with auto-generated view management
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
struct StylingApp;

#[cfg(feature = "interpreted")]
pub fn main() -> iced::Result {
    #[cfg(debug_assertions)]
    println!("🔥 Hot-reload enabled! Edit src/ui/window.dampen to see live updates.");

    iced::application(StylingApp::init, StylingApp::update, StylingApp::view)
        .theme(StylingApp::theme)
        .subscription(StylingApp::subscription)
        .run()
}

// ============================================================================
// CODEGEN MODE (production builds with pre-generated code)
// Note: This mode requires fixing bugs in dampen-core/src/codegen/
// For now, use interpreted mode with release builds instead.
// ============================================================================
#[cfg(all(feature = "codegen", not(feature = "interpreted")))]
mod ui;

// Include the generated code
#[cfg(all(feature = "codegen", not(feature = "interpreted")))]
include!(concat!(env!("OUT_DIR"), "/ui_generated.rs"));

#[cfg(all(feature = "codegen", not(feature = "interpreted")))]
use ui::window::Model;

#[cfg(all(feature = "codegen", not(feature = "interpreted")))]
fn codegen_init() -> (Model, iced::Task<Message>) {
    (Model::default(), iced::Task::none())
}

#[cfg(all(feature = "codegen", not(feature = "interpreted")))]
fn codegen_update(model: &mut Model, message: Message) -> iced::Task<Message> {
    update_model(model, message)
}

#[cfg(all(feature = "codegen", not(feature = "interpreted")))]
fn codegen_view(model: &Model) -> iced::Element<'_, Message> {
    view_model(model)
}

#[cfg(all(feature = "codegen", not(feature = "interpreted")))]
pub fn main() -> iced::Result {
    println!("🚀 Running in codegen mode (production)");

    iced::application(codegen_init, codegen_update, codegen_view)
        .theme(codegen_theme)
        .subscription(codegen_subscription)
        .run()
}

#[cfg(all(feature = "codegen", not(feature = "interpreted")))]
fn codegen_theme(_model: &Model) -> iced::Theme {
    app_theme()
}

#[cfg(all(feature = "codegen", not(feature = "interpreted")))]
fn codegen_subscription(_model: &Model) -> iced::Subscription<Message> {
    subscription_model()
}
