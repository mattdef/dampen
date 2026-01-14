use dampen_core::{BindingValue, HandlerRegistry, ToBindingValue};
use dampen_macros::{UiModel, dampen_ui};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum Priority {
    Low,
    #[default]
    Medium,
    High,
}

impl Priority {
    pub fn as_str(&self) -> &str {
        match self {
            Priority::Low => "Low",
            Priority::Medium => "Medium",
            Priority::High => "High",
        }
    }
}

impl ToBindingValue for Priority {
    fn to_binding_value(&self) -> BindingValue {
        match self {
            Priority::Low => BindingValue::String("Low".to_string()),
            Priority::Medium => BindingValue::String("Medium".to_string()),
            Priority::High => BindingValue::String("High".to_string()),
        }
    }
}

#[derive(Default, Clone, UiModel, Serialize, Deserialize, Debug)]
pub struct Model {
    pub new_item_text: String,
    pub description: String,
    pub selected_category: String,
    #[ui_skip]
    pub selected_priority: Priority,
    pub selected_priority_display: String,
    pub current_theme: String,
    pub dark_mode: bool,
}

#[dampen_ui("add_task.dampen")]
mod _add_task {}

/// Create AppState for add_task window (with SharedContext)
pub fn create_app_state_with_shared(
    shared: dampen_core::SharedContext<crate::shared::SharedState>,
) -> dampen_core::AppState<Model, crate::shared::SharedState> {
    let document = _add_task::document();
    let handler_registry = create_handler_registry();
    let model = Model {
        selected_category: "Personal".to_string(),
        selected_priority: Priority::Medium,
        selected_priority_display: "Medium".to_string(),
        current_theme: "light".to_string(),
        ..Default::default()
    };
    dampen_core::AppState::with_shared(document, model, handler_registry, shared)
}

pub fn create_handler_registry() -> HandlerRegistry {
    use std::any::Any;
    let registry = HandlerRegistry::new();

    // Update task name
    registry.register_with_value(
        "update_new_item",
        |model: &mut dyn Any, value: Box<dyn Any>| {
            if let Some(m) = model.downcast_mut::<Model>() {
                if let Ok(text) = value.downcast::<String>() {
                    m.new_item_text = (*text).clone();
                }
            }
        },
    );

    // Update description
    registry.register_with_value(
        "update_description",
        |model: &mut dyn Any, value: Box<dyn Any>| {
            if let Some(m) = model.downcast_mut::<Model>() {
                if let Ok(text) = value.downcast::<String>() {
                    m.description = (*text).clone();
                }
            }
        },
    );

    // Update category
    registry.register_with_value(
        "update_category",
        |model: &mut dyn Any, value: Box<dyn Any>| {
            if let Some(m) = model.downcast_mut::<Model>() {
                if let Ok(category) = value.downcast::<String>() {
                    m.selected_category = (*category).clone();
                }
            }
        },
    );

    // Select priority
    registry.register_with_value(
        "select_priority",
        |model: &mut dyn Any, value: Box<dyn Any>| {
            if let Some(m) = model.downcast_mut::<Model>() {
                if let Ok(priority_str) = value.downcast::<String>() {
                    let priority_str = (*priority_str).clone();
                    m.selected_priority = match priority_str.as_str() {
                        "High" => Priority::High,
                        "Medium" => Priority::Medium,
                        "Low" => Priority::Low,
                        _ => Priority::Medium,
                    };
                    m.selected_priority_display = priority_str;
                }
            }
        },
    );

    // Select date (placeholder)
    registry.register_simple("select_date", |model: &mut dyn Any| {
        if let Some(_m) = model.downcast_mut::<Model>() {
            println!("📅 Select date - Not implemented yet");
        }
    });

    // Submit task and send to main window via SharedContext
    registry.register_with_shared("submit_task", |model: &mut dyn Any, shared: &dyn Any| {
        if let (Some(m), Some(shared_ctx)) = (
            model.downcast_mut::<Model>(),
            shared.downcast_ref::<dampen_core::SharedContext<crate::shared::SharedState>>(),
        ) {
            if m.new_item_text.trim().is_empty() {
                println!("❌ Cannot submit empty task");
                return;
            }

            // Create pending task
            let pending_task = crate::shared::PendingTask {
                text: m.new_item_text.clone(),
                description: m.description.clone(),
                category: m.selected_category.clone(),
                priority: m.selected_priority.as_str().to_string(),
            };

            // Add to shared state queue
            let mut guard = shared_ctx.write();
            guard.add_pending_task(pending_task);
            drop(guard); // Release lock

            println!("✅ Task submitted and queued for main window");
            println!("   Name: {}", m.new_item_text);
            println!("   Category: {}", m.selected_category);
            println!("   Priority: {:?}", m.selected_priority);

            // Clear the form
            m.new_item_text.clear();
            m.description.clear();
            m.selected_priority = Priority::Medium;
            m.selected_priority_display = "Medium".to_string();
        }
    });

    // Close window and return to main view
    registry.register_with_command("close_window", |_model: &mut dyn Any| {
        use crate::{CurrentView, Message};
        Box::new(iced::Task::done(Message::SwitchToView(CurrentView::Window)))
    });

    // Toggle dark mode
    registry.register_simple("toggle_dark_mode", |model: &mut dyn Any| {
        if let Some(m) = model.downcast_mut::<Model>() {
            m.dark_mode = !m.dark_mode;
            m.current_theme = if m.dark_mode { "dark" } else { "light" }.to_string();
        }
    });

    registry
}
