//! Settings example demonstrating multiple UI views.
//!
//! This example demonstrates how to switch between multiple UI views
//! (main and settings) at runtime.

mod ui;

use dampen_core::AppState;
use dampen_iced::{DampenWidgetBuilder, HandlerMessage};
use iced::{Element, Task};

#[derive(Clone, Debug)]
enum CurrentView {
    Window,
    Settings,
}

#[derive(Clone, Debug)]
struct SettingsApp {
    current_view: CurrentView,
    window_state: AppState<ui::window::Model>,
    settings_state: AppState<ui::settings::Model>,
}

impl SettingsApp {
    fn new() -> (Self, Task<HandlerMessage>) {
        (
            SettingsApp {
                current_view: CurrentView::Window,
                window_state: ui::window::create_app_state(),
                settings_state: ui::settings::create_app_state(),
            },
            Task::none(),
        )
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

fn update(app: &mut SettingsApp, message: HandlerMessage) -> Task<HandlerMessage> {
    match message {
        HandlerMessage::Handler(handler_name, value) => match handler_name.as_str() {
            "switch_to_main" => app.current_view = CurrentView::Window,
            "switch_to_settings" => app.current_view = CurrentView::Settings,
            _ => dispatch_handler(app, &handler_name, value),
        },
    }
    Task::none()
}

fn view(app: &SettingsApp) -> Element<'_, HandlerMessage> {
    match app.current_view {
        CurrentView::Window => DampenWidgetBuilder::from_app_state(&app.window_state).build(),
        CurrentView::Settings => DampenWidgetBuilder::from_app_state(&app.settings_state).build(),
    }
}

fn init() -> (SettingsApp, Task<HandlerMessage>) {
    SettingsApp::new()
}

pub fn main() -> iced::Result {
    iced::application(init, update, view).run()
}
