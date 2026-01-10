//! Todo App example using the auto-loading pattern.

mod ui;

use dampen_core::AppState;
use dampen_iced::{DampenWidgetBuilder, HandlerMessage};
use iced::{Element, Task};

#[derive(Clone, Debug, PartialEq)]
enum CurrentView {
    Window,
}

struct TodoApp {
    current_view: CurrentView,
    window_state: AppState<ui::window::Model>,
}

fn dispatch_handler(app: &mut TodoApp, handler_name: &str, value: Option<String>) {
    let (model, registry) = match app.current_view {
        CurrentView::Window => (
            &mut app.window_state.model as &mut dyn std::any::Any,
            &app.window_state.handler_registry,
        ),
    };
    registry.dispatch(handler_name, model, value);
}

fn update(app: &mut TodoApp, message: HandlerMessage) -> Task<HandlerMessage> {
    match message {
        HandlerMessage::Handler(handler_name, value) => {
            dispatch_handler(app, &handler_name, value);
        }
    }
    Task::none()
}

fn view(app: &TodoApp) -> Element<'_, HandlerMessage> {
    match app.current_view {
        CurrentView::Window => DampenWidgetBuilder::from_app_state(&app.window_state),
    }
    .build()
}

fn init() -> (TodoApp, Task<HandlerMessage>) {
    (
        TodoApp {
            current_view: CurrentView::Window,
            window_state: ui::window::create_app_state(),
        },
        Task::none(),
    )
}

pub fn main() -> iced::Result {
    iced::application(init, update, view).run()
}
