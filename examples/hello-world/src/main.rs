//! Hello World example using the auto-loading pattern.
//!
//! This example demonstrates how to create a Gravity application
//! with minimal boilerplate using the new AppState pattern.
mod ui;

use gravity_core::AppState;
use gravity_iced::{GravityWidgetBuilder, HandlerMessage};
use iced::{Element, Task};

/// Main application state wrapper
struct GravityApp {
    state: AppState<ui::window::Model>,
}

/// Update function
fn update(app: &mut GravityApp, message: HandlerMessage) -> Task<HandlerMessage> {
    match message {
        HandlerMessage::Handler(handler_name, _value) => {
            if let Some(gravity_core::HandlerEntry::Simple(h)) =
                app.state.handler_registry.get(&handler_name)
            {
                h(&mut app.state.model);
            }
        }
    }
    Task::none()
}

/// View function using GravityWidgetBuilder
fn view(app: &GravityApp) -> Element<'_, HandlerMessage> {
    GravityWidgetBuilder::from_app_state(&app.state).build()
}

/// Initialize the application
fn init() -> (GravityApp, Task<HandlerMessage>) {
    (
        GravityApp {
            state: ui::window::create_app_state(),
        },
        Task::none(),
    )
}

pub fn main() -> iced::Result {
    iced::application(init, update, view).run()
}
