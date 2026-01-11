//! Settings example demonstrating multiple UI views.
//!
//! This example demonstrates how to switch between multiple UI views
//! (main and settings) at runtime.

mod ui;

use dampen_core::AppState;
use dampen_iced::{DampenWidgetBuilder, HandlerMessage};
use iced::{Element, Subscription, Task};
use std::path::PathBuf;

#[cfg(debug_assertions)]
use dampen_dev::{ErrorOverlay, FileEvent, watch_files};

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

#[derive(Clone, Debug, PartialEq)]
enum CurrentView {
    Window,
    Settings,
}

#[derive(Clone, Debug)]
struct SettingsApp {
    current_view: CurrentView,
    window_state: AppState<ui::window::Model>,
    settings_state: AppState<ui::settings::Model>,
    #[cfg(debug_assertions)]
    error_overlay: ErrorOverlay,
}

impl SettingsApp {
    fn new() -> (Self, Task<Message>) {
        (
            SettingsApp {
                current_view: CurrentView::Window,
                window_state: ui::window::create_app_state(),
                settings_state: ui::settings::create_app_state(),
                #[cfg(debug_assertions)]
                error_overlay: ErrorOverlay::new(),
            },
            Task::none(),
        )
    }
}

fn update(app: &mut SettingsApp, message: Message) -> Task<Message> {
    match message {
        Message::Handler(HandlerMessage::Handler(handler_name, value)) => {
            match handler_name.as_str() {
                "switch_to_main" => {
                    app.current_view = CurrentView::Window;
                    Task::none()
                }
                "switch_to_settings" => {
                    app.current_view = CurrentView::Settings;
                    Task::none()
                }
                _ => {
                    dispatch_handler(app, &handler_name, value);
                    Task::none()
                }
            }
        }
        #[cfg(debug_assertions)]
        Message::HotReload(event) => {
            match event {
                FileEvent::Success { document, path } => {
                    println!("🔄 Hot-reloading {:?}...", path);

                    // Determine which view to reload based on the file path
                    if path.to_string_lossy().contains("window.dampen") {
                        app.window_state.hot_reload(*document);
                    } else if path.to_string_lossy().contains("settings.dampen") {
                        app.settings_state.hot_reload(*document);
                    } else {
                        println!("⚠️  Unknown file: {:?}", path);
                    }

                    app.error_overlay.hide();
                    println!("✅ Hot-reload successful!");
                }
                FileEvent::ParseError { error, path, .. } => {
                    println!("❌ Parse error in {:?}: {}", path, error);
                    app.error_overlay.show(error);
                }
                FileEvent::WatcherError { path, error } => {
                    println!("⚠️  File watcher error for {:?}: {}", path, error);
                }
            }
            Task::none()
        }
        #[cfg(debug_assertions)]
        Message::DismissError => {
            app.error_overlay.hide();
            Task::none()
        }
    }
}

fn view(app: &SettingsApp) -> Element<'_, Message> {
    #[cfg(debug_assertions)]
    if app.error_overlay.is_visible() {
        // Show error overlay on top of normal UI
        return app.error_overlay.render(Message::DismissError);
    }

    let element = match app.current_view {
        CurrentView::Window => DampenWidgetBuilder::from_app_state(&app.window_state).build(),
        CurrentView::Settings => DampenWidgetBuilder::from_app_state(&app.settings_state).build(),
    };

    // Map HandlerMessage to Message
    element.map(Message::Handler)
}

fn init() -> (SettingsApp, Task<Message>) {
    SettingsApp::new()
}

pub fn main() -> iced::Result {
    println!("🔥 Hot-reload enabled! Edit src/ui/*.dampen files to see live updates.");

    iced::application(init, update, view)
        .window_size(iced::Size::new(400.0, 250.0))
        .centered()
        .subscription(subscription)
        .run()
}

fn subscription(_app: &SettingsApp) -> Subscription<Message> {
    #[cfg(debug_assertions)]
    {
        // Resolve UI file path relative to the manifest directory
        // This works whether running from workspace root or example directory
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let window_file = PathBuf::from(manifest_dir).join("src/ui/window.dampen");
        let settings_file = PathBuf::from(manifest_dir).join("src/ui/settings.dampen");

        println!("👀 Watching for changes:");
        println!("   - {}", window_file.display());
        println!("   - {}", settings_file.display());

        // Watch the UI files for changes in development mode (100ms debounce)
        watch_files(vec![window_file, settings_file], 100).map(Message::HotReload)
    }

    #[cfg(not(debug_assertions))]
    {
        Subscription::none()
    }
}

fn dispatch_handler(app: &mut SettingsApp, handler_name: &str, value: Option<String>) {
    let (model, registry) = match app.current_view {
        CurrentView::Window => (
            &mut app.window_state.model as &mut dyn std::any::Any,
            &app.window_state.handler_registry,
        ),
        CurrentView::Settings => (
            &mut app.settings_state.model as &mut dyn std::any::Any,
            &app.settings_state.handler_registry,
        ),
    };
    registry.dispatch(handler_name, model, value);
}
