//! Settings example demonstrating multiple UI views.
//!
//! This example demonstrates how to switch between multiple UI views
//! (main and settings) at runtime.

mod ui;

use gravity_core::AppState;
use gravity_iced::{GravityWidgetBuilder, HandlerMessage};
use iced::{Element, Task};

#[derive(Clone, Debug)]
enum CurrentView {
    Main,
    Settings,
}

struct SettingsApp {
    current_view: CurrentView,
    main_state: AppState<ui::window::Model>,
    settings_state: AppState<ui::settings::Model>,
}

fn dispatch_handler(app: &mut SettingsApp, handler_name: &str, value: Option<String>) {
    let (model, registry) = match app.current_view {
        CurrentView::Main => (
            &mut app.main_state.model as &mut dyn std::any::Any,
            &app.main_state.handler_registry,
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
            "switch_to_main" => app.current_view = CurrentView::Main,
            "switch_to_settings" => app.current_view = CurrentView::Settings,
            _ => dispatch_handler(app, &handler_name, value),
        },
    }
    Task::none()
}

fn view(app: &SettingsApp) -> Element<'_, HandlerMessage> {
    match app.current_view {
        CurrentView::Main => GravityWidgetBuilder::from_app_state(&app.main_state).build(),
        CurrentView::Settings => GravityWidgetBuilder::from_app_state(&app.settings_state).build(),
    }
}

fn init() -> (SettingsApp, Task<HandlerMessage>) {
    (
        SettingsApp {
            current_view: CurrentView::Main,
            main_state: ui::window::create_app_state(),
            settings_state: ui::settings::create_app_state(),
        },
        Task::none(),
    )
}

pub fn main() -> iced::Result {
    iced::application(init, update, view).run()
}
