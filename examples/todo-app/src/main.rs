//! Todo App example using the auto-loading pattern.

mod ui;

use gravity_core::AppState;
use gravity_iced::{GravityWidgetBuilder, HandlerMessage};
use iced::{widget::container, Element, Task, Theme};
use std::path::PathBuf;

type Message = HandlerMessage;

struct TodoApp {
    state: AppState<ui::app::Model>,
}

fn update(app: &mut TodoApp, message: Message) -> Task<Message> {
    match message {
        HandlerMessage::Handler(handler_name, ref value) => {
            let (base_handler, param_value) = parse_handler_name(&handler_name);

            if let Some(handler) = app.state.handler_registry.get(&base_handler) {
                match handler {
                    gravity_core::HandlerEntry::Simple(h) => {
                        h(&mut app.state.model);
                    }
                    gravity_core::HandlerEntry::WithValue(h) => {
                        let handler_value = param_value.as_ref().or(value.as_ref());
                        if let Some(val) = handler_value {
                            h(&mut app.state.model, Box::new(val.clone()));
                        } else {
                            h(&mut app.state.model, Box::new(String::new()));
                        }
                    }
                    gravity_core::HandlerEntry::WithCommand(h) => {
                        let _cmd = h(&mut app.state.model);
                    }
                }
            }

            // Auto-save after any handler modification
            let save_path = PathBuf::from("todo-app-data.json");
            ui::app::save_to_path(&app.state.model, &save_path);
        }
    }
    Task::none()
}

fn parse_handler_name(name: &str) -> (String, Option<String>) {
    if let Some((base, value)) = name.split_once(':') {
        (base.to_string(), Some(value.to_string()))
    } else {
        (name.to_string(), None)
    }
}

fn view(app: &TodoApp) -> Element<'_, Message> {
    let is_dark = app.state.model.dark_mode;
    let bg_color = if is_dark {
        iced::Color::from_rgb(0.17, 0.24, 0.31)
    } else {
        iced::Color::from_rgb(0.93, 0.94, 0.95)
    };
    let text_color = if is_dark {
        iced::Color::from_rgb(0.93, 0.94, 0.95)
    } else {
        iced::Color::from_rgb(0.18, 0.24, 0.31)
    };

    let content = GravityWidgetBuilder::new(
        &app.state.document,
        &app.state.model,
        Some(&app.state.handler_registry),
    )
    .with_verbose(false)
    .build();

    let style_fn = move |_theme: &Theme| iced::widget::container::Style {
        background: Some(iced::Background::Color(bg_color)),
        text_color: Some(text_color),
        ..iced::widget::container::Style::default()
    };

    container(content).style(style_fn).into()
}

fn init() -> (TodoApp, Task<Message>) {
    let save_path = PathBuf::from("todo-app-data.json");
    let model = if save_path.exists() {
        ui::app::load_from_path(&save_path)
    } else {
        ui::app::Model::default()
    };
    let state = ui::app::create_app_state_with_model(model);
    (TodoApp { state }, Task::none())
}

pub fn main() -> iced::Result {
    iced::application(init, update, view).run()
}
