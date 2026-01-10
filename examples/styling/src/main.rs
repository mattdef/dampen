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
struct StylingApp {
    current_view: CurrentView,
    window_state: AppState<ui::window::Model>,
}

impl StylingApp {
    fn new() -> (Self, Task<HandlerMessage>) {
        (
            StylingApp {
                current_view: CurrentView::Window,
                window_state: ui::window::create_app_state(),
            },
            Task::none(),
        )
    }
}

fn dispatch_handler(app: &mut StylingApp, handler_name: &str, value: Option<String>) {
    let (model, registry) = match app.current_view {
        CurrentView::Window => (
            &mut app.window_state.model as &mut dyn std::any::Any,
            &app.window_state.handler_registry,
        ),
    };
    registry.dispatch(handler_name, model, value);
}

fn update(app: &mut StylingApp, message: HandlerMessage) -> Task<HandlerMessage> {
    match message {
        HandlerMessage::Handler(handler_name, value) => match handler_name.as_str() {
            _ => dispatch_handler(app, &handler_name, value),
        },
    }
    Task::none()
}

fn view(app: &StylingApp) -> Element<'_, HandlerMessage> {
    DampenWidgetBuilder::from_app_state(&app.window_state).build()
}

fn init() -> (StylingApp, Task<HandlerMessage>) {
    StylingApp::new()
}

pub fn main() -> iced::Result {
    iced::application(init, update, view).run()
}
