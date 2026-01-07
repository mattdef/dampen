//! Counter example using the auto-loading pattern.

mod ui;

use gravity_core::AppState;
use gravity_iced::{GravityWidgetBuilder, HandlerMessage};
use iced::{Element, Task};

struct CounterApp {
    state: AppState<ui::window::Model>,
}

fn update(app: &mut CounterApp, message: HandlerMessage) -> Task<HandlerMessage> {
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

fn view(app: &CounterApp) -> Element<'_, HandlerMessage> {
    GravityWidgetBuilder::from_app_state(&app.state).build()
}

fn init() -> (CounterApp, Task<HandlerMessage>) {
    (
        CounterApp {
            state: ui::window::create_app_state(),
        },
        Task::none(),
    )
}

pub fn main() -> iced::Result {
    iced::application(init, update, view).run()
}
