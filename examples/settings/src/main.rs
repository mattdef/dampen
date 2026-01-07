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

type AppMessage = HandlerMessage;

fn update(app: &mut SettingsApp, message: AppMessage) -> Task<AppMessage> {
    match message {
        HandlerMessage::Handler(handler_name, _) => match handler_name.as_str() {
            "switch_to_settings" => {
                app.current_view = CurrentView::Settings;
            }
            "switch_to_main" => {
                app.current_view = CurrentView::Main;
            }
            _ => {}
        },
    }
    Task::none()
}

fn view(app: &SettingsApp) -> Element<'_, AppMessage> {
    match app.current_view {
        CurrentView::Main => GravityWidgetBuilder::new(
            &app.main_state.document,
            &app.main_state.model,
            Some(&app.main_state.handler_registry),
        )
        .build(),
        CurrentView::Settings => GravityWidgetBuilder::new(
            &app.settings_state.document,
            &app.settings_state.model,
            Some(&app.settings_state.handler_registry),
        )
        .build(),
    }
}

fn init() -> (SettingsApp, Task<AppMessage>) {
    let main_state = ui::window::create_app_state();
    let settings_state = ui::settings::create_app_state();

    (
        SettingsApp {
            current_view: CurrentView::Main,
            main_state,
            settings_state,
        },
        Task::none(),
    )
}

pub fn main() -> iced::Result {
    iced::application(init, update, view).run()
}
