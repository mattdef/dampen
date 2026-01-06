//! Hello World example using the auto-loading pattern.
//!
//! This example demonstrates how to create a Gravity application
//! with minimal boilerplate using the new AppState pattern.

use gravity_core::AppState;
use gravity_iced::{GravityWidgetBuilder, HandlerMessage};
use iced::{Element, Task};

pub mod ui {
    //! UI module with auto-loaded XML.
    //!
    //! This module demonstrates the new pattern where:
    //! - `app.rs` uses `#[gravity_ui]` macro
    //! - The XML file is loaded automatically
    //! - AppState is created with handlers

    include!("ui/mod.rs");
}

/// Messages for the application.
type Message = HandlerMessage;

/// Main application state wrapper
struct GravityApp {
    state: AppState<ui::window::Model>,
}

/// Update function
fn update(app: &mut GravityApp, message: Message) -> Task<Message> {
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
fn view(app: &GravityApp) -> Element<'_, Message> {
    GravityWidgetBuilder::new(
        &app.state.document,
        &app.state.model,
        Some(&app.state.handler_registry),
    )
    .build()
}

/// Initialize the application
fn init() -> (GravityApp, Task<Message>) {
    let state = ui::window::create_app_state();
    (GravityApp { state }, Task::none())
}

pub fn main() -> iced::Result {
    iced::application(init, update, view).run()
}
