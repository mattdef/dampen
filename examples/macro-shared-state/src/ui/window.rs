//! Main window view - displays shared state

use crate::shared::SharedState;
use dampen_core::{AppState, HandlerRegistry, SharedContext};
use dampen_macros::{UiModel, dampen_ui};
use serde::{Deserialize, Serialize};

#[dampen_ui("window.dampen")]
mod _window {}

#[derive(Default, UiModel, Serialize, Deserialize, Clone, Debug)]
pub struct Model {
    /// Local message (view-specific)
    pub local_message: String,
}

/// Create AppState WITH shared context (called by macro)
pub fn create_app_state_with_shared(
    shared: SharedContext<SharedState>,
) -> AppState<Model, SharedState> {
    let document = _window::document();
    let handlers = create_handler_registry();
    let model = Model::default();
    AppState::with_shared(document, model, handlers, shared)
}

/// Create AppState WITHOUT shared context (backward compatibility)
#[allow(dead_code)] // Not used when shared_model is enabled
pub fn create_app_state() -> AppState<Model> {
    let document = _window::document();
    let handlers = create_handler_registry();
    AppState::with_handlers(document, handlers)
}

pub fn create_handler_registry() -> HandlerRegistry {
    use crate::{CurrentView, Message};
    use std::any::Any;

    let registry = HandlerRegistry::new();

    // Simple handler using local model
    registry.register_simple("update_local", |model: &mut dyn Any| {
        if let Some(m) = model.downcast_mut::<Model>() {
            m.local_message = "Local state updated!".to_string();
        }
    });

    // Handler that reads shared state
    registry.register_with_shared("read_theme", |model: &mut dyn Any, shared: &dyn Any| {
        if let (Some(m), Some(s)) = (
            model.downcast_mut::<Model>(),
            shared.downcast_ref::<SharedContext<SharedState>>(),
        ) {
            let guard = s.read();
            m.local_message = format!("Current theme: {} (user: {})", guard.theme, guard.username);
        }
    });

    // Switch to settings view
    registry.register_with_command("goto_settings", |_model: &mut dyn Any| {
        Box::new(iced::Task::done(Message::SwitchToView(
            CurrentView::Settings,
        )))
    });

    registry
}
