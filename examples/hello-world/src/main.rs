//! Hello World example using the auto-loading pattern.
//!
//! This example demonstrates how to create a Gravity application
//! with minimal boilerplate using the new AppState pattern.
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
}

/// Main application state wrapper
#[derive(Clone, Debug)]
struct HelloApp {
    current_view: CurrentView,
    window_state: AppState<ui::window::Model>,
    #[cfg(debug_assertions)]
    error_overlay: ErrorOverlay,
}

impl HelloApp {
    fn new() -> (Self, Task<Message>) {
        (
            HelloApp {
                current_view: CurrentView::Window,
                window_state: ui::window::create_app_state(),
                #[cfg(debug_assertions)]
                error_overlay: ErrorOverlay::new(),
            },
            Task::none(),
        )
    }
}

/// Update function
fn update(app: &mut HelloApp, message: Message) -> Task<Message> {
    match message {
        Message::Handler(HandlerMessage::Handler(handler_name, value)) => {
            dispatch_handler(app, &handler_name, value);
            Task::none()
        }
        #[cfg(debug_assertions)]
        Message::HotReload(event) => {
            match event {
                FileEvent::Success { document, path } => {
                    println!("🔄 Hot-reloading {:?}...", path);

                    // Simple hot-reload: just update the document
                    // (Model state is already preserved in window_state)
                    // Unbox the document since FileEvent now uses Box<DampenDocument>
                    app.window_state.hot_reload(*document);
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

/// View function using DampenWidgetBuilder
fn view(app: &HelloApp) -> Element<'_, Message> {
    #[cfg(debug_assertions)]
    if app.error_overlay.is_visible() {
        // Show error overlay on top of normal UI
        return app.error_overlay.render(Message::DismissError);
    }

    // Normal UI view
    DampenWidgetBuilder::from_app_state(&app.window_state)
        .build()
        .map(Message::Handler)
}

/// Initialize the application
fn init() -> (HelloApp, Task<Message>) {
    HelloApp::new()
}

pub fn main() -> iced::Result {
    println!("🔥 Hot-reload enabled! Edit src/ui/window.dampen to see live updates.");

    iced::application(init, update, view)
        .subscription(subscription)
        .run()
}

/// Subscription function for hot-reload (development mode only)
fn subscription(_app: &HelloApp) -> Subscription<Message> {
    #[cfg(debug_assertions)]
    {
        // Resolve UI file path relative to the manifest directory
        // This works whether running from workspace root or example directory
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let ui_file = PathBuf::from(manifest_dir).join("src/ui/window.dampen");

        println!("👀 Watching for changes: {}", ui_file.display());

        // Watch the UI file for changes in development mode (100ms debounce)
        watch_files(vec![ui_file], 100).map(Message::HotReload)
    }

    #[cfg(not(debug_assertions))]
    {
        Subscription::none()
    }
}

/// Dispatch a handler to the current view
fn dispatch_handler(app: &mut HelloApp, handler_name: &str, value: Option<String>) {
    let (model, registry) = match app.current_view {
        CurrentView::Window => (
            &mut app.window_state.model as &mut dyn std::any::Any,
            &app.window_state.handler_registry,
        ),
    };
    registry.dispatch(handler_name, model, value);
}
