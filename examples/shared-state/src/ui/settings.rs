use dampen_core::{AppState, HandlerRegistry, SharedContext};
use dampen_macros::{UiModel, dampen_ui};
use serde::{Deserialize, Serialize};

use crate::shared::SharedState;

/// Model for the settings window (local state)
#[derive(Default, Clone, Serialize, Deserialize, UiModel)]
pub struct SettingsModel {
    pub status: String,
}

#[dampen_ui("settings.dampen")]
mod _settings {}

pub fn create_app_state(
    shared: SharedContext<SharedState>,
) -> AppState<SettingsModel, SharedState> {
    let document = _settings::document();
    let model = SettingsModel::default();
    let handlers = create_handler_registry(shared.clone());
    AppState::with_shared(document, model, handlers, shared)
}

pub fn create_handler_registry(shared: SharedContext<SharedState>) -> HandlerRegistry {
    let mut registry = HandlerRegistry::new();

    // Toggle theme handler
    {
        let shared = shared.clone();
        registry.register_with_shared(
            "toggle_theme",
            move |_model: &mut dyn std::any::Any, shared_any: &dyn std::any::Any| {
                let shared_ctx = shared_any
                    .downcast_ref::<SharedContext<SharedState>>()
                    .unwrap_or(&shared);
                shared_ctx.write().toggle_theme();
                println!("Theme toggled to: {}", shared_ctx.read().theme);
            },
        );
    }

    // Set language handlers
    {
        let shared = shared.clone();
        registry.register_with_shared(
            "set_language_en",
            move |_model: &mut dyn std::any::Any, shared_any: &dyn std::any::Any| {
                let shared_ctx = shared_any
                    .downcast_ref::<SharedContext<SharedState>>()
                    .unwrap_or(&shared);
                shared_ctx.write().language = "en".to_string();
                println!("Language set to: English");
            },
        );
    }

    {
        let shared = shared.clone();
        registry.register_with_shared(
            "set_language_es",
            move |_model: &mut dyn std::any::Any, shared_any: &dyn std::any::Any| {
                let shared_ctx = shared_any
                    .downcast_ref::<SharedContext<SharedState>>()
                    .unwrap_or(&shared);
                shared_ctx.write().language = "es".to_string();
                println!("Language set to: Spanish");
            },
        );
    }

    {
        let shared = shared.clone();
        registry.register_with_shared(
            "set_language_fr",
            move |_model: &mut dyn std::any::Any, shared_any: &dyn std::any::Any| {
                let shared_ctx = shared_any
                    .downcast_ref::<SharedContext<SharedState>>()
                    .unwrap_or(&shared);
                shared_ctx.write().language = "fr".to_string();
                println!("Language set to: French");
            },
        );
    }

    // Toggle notifications handler
    {
        let shared = shared.clone();
        registry.register_with_shared(
            "toggle_notifications",
            move |_model: &mut dyn std::any::Any, shared_any: &dyn std::any::Any| {
                let shared_ctx = shared_any
                    .downcast_ref::<SharedContext<SharedState>>()
                    .unwrap_or(&shared);
                shared_ctx.write().toggle_notifications();
                println!(
                    "Notifications toggled to: {}",
                    shared_ctx.read().notifications_enabled
                );
            },
        );
    }

    registry
}
