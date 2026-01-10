//! Counter example using the auto-loading pattern.

mod ui;

use dampen_core::AppState;
use dampen_iced::{DampenWidgetBuilder, HandlerMessage};
use iced::{Element, Task};

#[derive(Clone, Debug, PartialEq)]
enum CurrentView {
    Window,
}

#[derive(Clone, Debug)]
/// Main application state wrapper
struct CounterApp {
    current_view: CurrentView,
    window_state: AppState<ui::window::Model>,
}

impl CounterApp {
    fn new() -> (Self, Task<HandlerMessage>) {
        (
            CounterApp {
                current_view: CurrentView::Window,
                window_state: ui::window::create_app_state(),
            },
            Task::none(),
        )
    }
}

/// Dispatch a handler to the current view
fn dispatch_handler(app: &mut CounterApp, handler_name: &str, value: Option<String>) {
    let (model, registry) = match app.current_view {
        CurrentView::Window => (
            &mut app.window_state.model as &mut dyn std::any::Any,
            &app.window_state.handler_registry,
        ),
    };
    registry.dispatch(handler_name, model, value);
}

/// Update function
fn update(app: &mut CounterApp, message: HandlerMessage) -> Task<HandlerMessage> {
    match message {
        HandlerMessage::Handler(handler_name, value) => match handler_name.as_str() {
            _ => dispatch_handler(app, &handler_name, value),
        },
    }
    Task::none()
}

/// View function using DampenWidgetBuilder
fn view(app: &CounterApp) -> Element<'_, HandlerMessage> {
    DampenWidgetBuilder::from_app_state(&app.window_state).build()
}

/// Initialize the application
fn init() -> (CounterApp, Task<HandlerMessage>) {
    CounterApp::new()
}

pub fn main() -> iced::Result {
    iced::application(init, update, view).run()
}
