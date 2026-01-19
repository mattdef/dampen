use dampen_core::{AppState, HandlerRegistry};
use dampen_macros::{UiModel, dampen_ui, inventory_handlers, ui_handler};
use serde::{Deserialize, Serialize};

#[dampen_ui("window.dampen")]
mod _window {}

#[derive(Default, UiModel, Serialize, Deserialize, Clone, Debug)]
pub struct Model {
    pub message: String,
}

#[ui_handler]
pub fn set_theme(model: &mut Model, theme: String) {
    model.message = format!("Switched to theme: {}", theme);

    // In codegen mode, call the generated app_set_current_theme function
    // to actually change the active theme
    #[cfg(feature = "codegen")]
    {
        crate::window::app_set_current_theme(&theme);
    }
}

inventory_handlers! {
    set_theme
}

pub fn create_app_state() -> AppState<Model> {
    let document = _window::document();
    let handler_registry = create_handler_registry();
    AppState::with_handlers(document, handler_registry)
}

pub fn create_handler_registry() -> HandlerRegistry {
    HandlerRegistry::new()
}
