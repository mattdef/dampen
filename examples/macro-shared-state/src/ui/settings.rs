//! Settings view - modifies shared state

use crate::shared::SharedState;
use dampen_core::{AppState, HandlerRegistry, SharedContext};
use dampen_macros::{UiModel, dampen_ui};
use serde::{Deserialize, Serialize};

#[dampen_ui("settings.dampen")]
mod _settings {}

#[derive(Default, UiModel, Serialize, Deserialize, Clone, Debug)]
pub struct Model {
    /// Dummy field to trigger re-renders when shared state changes
    #[ui_skip]
    _refresh_trigger: usize,
}

pub fn create_app_state_with_shared(
    shared: SharedContext<SharedState>,
) -> AppState<Model, SharedState> {
    let document = _settings::document();
    let handlers = create_handler_registry();
    let model = Model::default();
    AppState::with_shared(document, model, handlers, shared)
}

#[allow(dead_code)] // Backward compatibility - not used when shared_model is enabled
pub fn create_app_state() -> AppState<Model> {
    let document = _settings::document();
    let handlers = create_handler_registry();
    AppState::with_handlers(document, handlers)
}

pub fn create_handler_registry() -> HandlerRegistry {
    use crate::{CurrentView, Message};
    use std::any::Any;

    let registry = HandlerRegistry::new();

    // Handlers that modify shared state
    registry.register_with_value_and_shared(
        "change_theme",
        |model: &mut dyn Any, value: Box<dyn Any>, shared: &dyn Any| {
            if let (Some(m), Some(s), Ok(theme)) = (
                model.downcast_mut::<Model>(),
                shared.downcast_ref::<SharedContext<SharedState>>(),
                value.downcast::<String>(),
            ) {
                let mut guard = s.write();
                guard.theme = *theme;
                m._refresh_trigger += 1;
            }
        },
    );

    registry.register_with_value_and_shared(
        "change_username",
        |model: &mut dyn Any, value: Box<dyn Any>, shared: &dyn Any| {
            if let (Some(m), Some(s), Ok(username)) = (
                model.downcast_mut::<Model>(),
                shared.downcast_ref::<SharedContext<SharedState>>(),
                value.downcast::<String>(),
            ) {
                let mut guard = s.write();
                guard.username = *username;
                m._refresh_trigger += 1;
            }
        },
    );

    registry.register_with_shared(
        "increment_notifications",
        |model: &mut dyn Any, shared: &dyn Any| {
            if let (Some(m), Some(s)) = (
                model.downcast_mut::<Model>(),
                shared.downcast_ref::<SharedContext<SharedState>>(),
            ) {
                let mut guard = s.write();
                guard.notification_count += 1;
                m._refresh_trigger += 1;
            }
        },
    );

    // Switch back to main window
    registry.register_with_command("goto_main", |_model: &mut dyn Any| {
        Box::new(iced::Task::done(Message::SwitchToView(CurrentView::Window)))
    });

    registry
}
