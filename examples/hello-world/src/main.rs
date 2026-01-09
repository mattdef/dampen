//! Hello World example using the auto-loading pattern.
//!
//! This example demonstrates how to create a Gravity application
//! with minimal boilerplate using the new AppState pattern.
mod ui;

use dampen_core::AppState;
use dampen_iced::{DampenWidgetBuilder, HandlerMessage};
use iced::{Element, Task};

#[derive(Clone, Debug, PartialEq)]
enum CurrentView {
    Window,
}

/// Main application state wrapper
struct GravityApp {
    current_view: CurrentView,
    window_state: AppState<ui::window::Model>,
}

/// Dispatch a handler to the current view
fn dispatch_handler(app: &mut GravityApp, handler_name: &str, value: Option<String>) {
    let (model, registry) = match app.current_view {
        CurrentView::Window => (
            &mut app.window_state.model as &mut dyn std::any::Any,
            &app.window_state.handler_registry,
        ),
    };
    registry.dispatch(handler_name, model, value);
}

/// Update function
fn update(app: &mut GravityApp, message: HandlerMessage) -> Task<HandlerMessage> {
    match message {
        HandlerMessage::Handler(handler_name, value) => match handler_name.as_str() {
            _ => dispatch_handler(app, &handler_name, value),
        },
    }
    Task::none()
}

/// View function using DampenWidgetBuilder
fn view(app: &GravityApp) -> Element<'_, HandlerMessage> {
    DampenWidgetBuilder::from_app_state(&app.window_state).build()
}

/// Initialize the application
fn init() -> (GravityApp, Task<HandlerMessage>) {
    (
        GravityApp {
            current_view: CurrentView::Window,
            window_state: ui::window::create_app_state(),
        },
        Task::none(),
    )
}

pub fn main() -> iced::Result {
    iced::application(init, update, view).run()
}
