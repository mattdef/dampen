//! Counter example using the auto-loading pattern.

mod ui;

use gravity_core::AppState;
use gravity_iced::{GravityWidgetBuilder, HandlerMessage};
use iced::{Element, Task};

type Message = HandlerMessage;

struct CounterApp {
    state: AppState<ui::app::Model>,
}

fn update(app: &mut CounterApp, message: Message) -> Task<Message> {
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

fn view(app: &CounterApp) -> Element<'_, Message> {
    GravityWidgetBuilder::new(
        &app.state.document,
        &app.state.model,
        Some(&app.state.handler_registry),
    )
    .build()
}

fn init() -> (CounterApp, Task<Message>) {
    let state = ui::app::create_app_state();
    (CounterApp { state }, Task::none())
}

pub fn main() -> iced::Result {
    iced::application(init, update, view).run()
}
