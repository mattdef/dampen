//! Shared State example demonstrating inter-view communication.
//!
//! This example shows how multiple views can share state via SharedContext.
//! The application has two views (tabs):
//! 1. Main View - Displays current shared preferences
//! 2. Settings View - Allows modifying shared preferences
//!
//! Changes made in the Settings view are immediately reflected in the Main view.

mod shared;
mod ui;

use dampen_core::SharedContext;
use dampen_iced::{DampenWidgetBuilder, HandlerMessage};
use iced::{Element, Task};
use shared::SharedState;

/// Application messages
#[derive(Clone, Debug)]
enum Message {
    /// Handler invocation from UI widgets
    Handler(String, Option<String>),
    /// Switch between views
    SwitchView(View),
}

/// Available views
#[derive(Clone, Debug, PartialEq)]
enum View {
    Main,
    Settings,
}

/// Application state
struct App {
    /// Shared state accessible by all views
    shared: SharedContext<SharedState>,
    /// Cached clone of shared state for rendering
    shared_cache: SharedState,
    /// Current view
    current_view: View,
    /// Main window state
    window_state: ui::window::WindowModel,
    /// Main window app state (includes document and handlers)
    window_app_state: dampen_core::AppState<ui::window::WindowModel, ()>,
    /// Settings state
    settings_state: ui::settings::SettingsModel,
    /// Settings app state (includes document and handlers)
    settings_app_state: dampen_core::AppState<ui::settings::SettingsModel, SharedState>,
}

impl App {
    fn init() -> (Self, Task<Message>) {
        let shared = SharedContext::new(SharedState::new());
        let shared_cache = shared.read().clone();

        let window_state = ui::window::WindowModel::default();
        let window_app_state = ui::window::create_app_state();

        let settings_state = ui::settings::SettingsModel::default();
        let settings_app_state = ui::settings::create_app_state(shared.clone());

        (
            Self {
                shared,
                shared_cache,
                current_view: View::Main,
                window_state,
                window_app_state,
                settings_state,
                settings_app_state,
            },
            Task::none(),
        )
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Handler(handler_name, value) => {
                match self.current_view {
                    View::Main => {
                        self.window_app_state.handler_registry.dispatch(
                            &handler_name,
                            &mut self.window_state as &mut dyn std::any::Any,
                            value,
                        );
                    }
                    View::Settings => {
                        self.settings_app_state
                            .handler_registry
                            .dispatch_with_shared(
                                &handler_name,
                                &mut self.settings_state as &mut dyn std::any::Any,
                                &self.shared as &dyn std::any::Any,
                                value,
                            );
                        // Update cache after handler modifies shared state
                        self.shared_cache = self.shared.read().clone();
                    }
                }
                Task::none()
            }
            Message::SwitchView(view) => {
                // Update cache when switching views to show latest state
                self.shared_cache = self.shared.read().clone();
                self.current_view = view;
                Task::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        use iced::Length;
        use iced::widget::{button, column, container, row, text};

        // Tab buttons
        let tabs = row![
            button(text("Main View"))
                .on_press(Message::SwitchView(View::Main))
                .style(if self.current_view == View::Main {
                    iced::widget::button::primary
                } else {
                    iced::widget::button::secondary
                }),
            button(text("Settings"))
                .on_press(Message::SwitchView(View::Settings))
                .style(if self.current_view == View::Settings {
                    iced::widget::button::primary
                } else {
                    iced::widget::button::secondary
                }),
        ]
        .spacing(10);

        // View content
        let content: Element<_> = match self.current_view {
            View::Main => {
                let builder = DampenWidgetBuilder::new(
                    &self.window_app_state.document,
                    &self.window_state,
                    Some(&self.window_app_state.handler_registry),
                )
                .with_shared(&self.shared_cache);

                builder
                    .build()
                    .map(|HandlerMessage::Handler(name, value)| Message::Handler(name, value))
            }
            View::Settings => {
                let builder = DampenWidgetBuilder::new(
                    &self.settings_app_state.document,
                    &self.settings_state,
                    Some(&self.settings_app_state.handler_registry),
                )
                .with_shared(&self.shared_cache);

                builder
                    .build()
                    .map(|HandlerMessage::Handler(name, value)| Message::Handler(name, value))
            }
        };

        let layout = column![tabs, content].spacing(20).padding(20);

        container(layout)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

pub fn main() -> iced::Result {
    iced::application(App::init, App::update, App::view)
        .window_size(iced::Size::new(600.0, 500.0))
        .centered()
        .run()
}
