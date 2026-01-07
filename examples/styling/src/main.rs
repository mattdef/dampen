//! Counter example using the auto-loading pattern.

mod ui;

use gravity_core::AppState;
use gravity_iced::{GravityWidgetBuilder, HandlerMessage};
use iced::{Element, Task};

type Message = HandlerMessage;

struct StylingApp {
    state: AppState<ui::window::Model>,
}

fn update(app: &mut StylingApp, message: Message) -> Task<Message> {
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

fn view(app: &StylingApp) -> Element<'_, Message> {
    GravityWidgetBuilder::new(
        &app.state.document,
        &app.state.model,
        Some(&app.state.handler_registry),
    )
    .build()
}

fn init() -> (StylingApp, Task<Message>) {
    let state = ui::window::create_app_state();
    (StylingApp { state }, Task::none())
}

pub fn main() -> iced::Result {
    iced::application(init, update, view).run()
}
