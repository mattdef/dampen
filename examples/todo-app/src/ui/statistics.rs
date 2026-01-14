// Statistics window module for todo-app example.
// Displays real-time task statistics using SharedContext.

use dampen_core::HandlerRegistry;
use dampen_macros::{UiModel, dampen_ui};
use serde::{Deserialize, Serialize};

/// Statistics window has no local state - all data comes from SharedContext
#[derive(Default, Clone, UiModel, Serialize, Deserialize, Debug)]
pub struct Model {
    // Empty model - statistics window is stateless and reads from shared context
}

#[dampen_ui("statistics.dampen")]
mod _stats {}

/// Create AppState with shared context (called by dampen_app macro)
pub fn create_app_state_with_shared(
    shared: dampen_core::SharedContext<crate::shared::SharedState>,
) -> dampen_core::AppState<Model, crate::shared::SharedState> {
    let document = _stats::document();
    let handler_registry = create_handler_registry();
    let model = Model::default();
    dampen_core::AppState::with_shared(document, model, handler_registry, shared)
}

fn create_handler_registry() -> HandlerRegistry {
    let registry = HandlerRegistry::new();

    // T068-T069: Command handler to return to main window
    registry.register_with_command("close_window", |_model: &mut dyn std::any::Any| {
        use crate::{CurrentView, Message};
        Box::new(iced::Task::done(Message::SwitchToView(CurrentView::Window)))
    });

    registry
}
