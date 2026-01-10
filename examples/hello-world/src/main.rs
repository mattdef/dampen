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
#[derive(Clone, Debug)]
struct HelloApp {
    current_view: CurrentView,
    window_state: AppState<ui::window::Model>,
}

impl HelloApp {
    fn new() -> (Self, Task<HandlerMessage>) {
        (
            HelloApp {
                current_view: CurrentView::Window,
                window_state: ui::window::create_app_state(),
            },
            Task::none(),
        )
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

/// Update function
fn update(app: &mut HelloApp, message: HandlerMessage) -> Task<HandlerMessage> {
    match message {
        HandlerMessage::Handler(handler_name, value) => match handler_name.as_str() {
            _ => dispatch_handler(app, &handler_name, value),
        },
    }
    Task::none()
}

/// View function using DampenWidgetBuilder
fn view(app: &HelloApp) -> Element<'_, HandlerMessage> {
    DampenWidgetBuilder::from_app_state(&app.window_state).build()
}

/// Initialize the application
fn init() -> (HelloApp, Task<HandlerMessage>) {
    HelloApp::new()
}

pub fn main() -> iced::Result {
    iced::application(init, update, view).run()
}
