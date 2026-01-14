// Auto-loaded UI module for todo-app example.

use dampen_core::{BindingValue, HandlerRegistry, ToBindingValue};
use dampen_macros::{UiModel, dampen_ui};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[allow(dead_code)]
pub fn save_to_path(model: &Model, path: &PathBuf) {
    if let Ok(data) = serde_json::to_string_pretty(model) {
        let _ = std::fs::write(path, data);
    }
}

#[allow(dead_code)]
pub fn load_from_path(path: &PathBuf) -> Model {
    if let Ok(data) = std::fs::read_to_string(path) {
        if let Ok(model) = serde_json::from_str::<Model>(&data) {
            let mut model = model;
            update_computed_fields(&mut model);
            model
        } else {
            Model::default()
        }
    } else {
        Model::default()
    }
}

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

impl std::fmt::Display for Priority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl ToBindingValue for Priority {
    fn to_binding_value(&self) -> BindingValue {
        BindingValue::String(self.to_string())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum TodoFilter {
    #[default]
    All,
    Active,
    Completed,
}

impl TodoFilter {
    pub fn as_str(&self) -> &str {
        match self {
            TodoFilter::All => "All",
            TodoFilter::Active => "Active",
            TodoFilter::Completed => "Completed",
        }
    }

    #[allow(dead_code)]
    pub fn matches(&self, completed: bool) -> bool {
        match self {
            TodoFilter::All => true,
            TodoFilter::Active => !completed,
            TodoFilter::Completed => completed,
        }
    }
}

impl std::fmt::Display for TodoFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl ToBindingValue for TodoFilter {
    fn to_binding_value(&self) -> BindingValue {
        BindingValue::String(self.to_string())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TodoItem {
    pub id: usize,
    pub text: String,
    pub completed: bool,
    pub category: String,
    pub priority: Priority,
}

impl ToBindingValue for TodoItem {
    fn to_binding_value(&self) -> BindingValue {
        let mut map = std::collections::HashMap::new();
        map.insert("id".to_string(), BindingValue::Integer(self.id as i64));
        map.insert("text".to_string(), BindingValue::String(self.text.clone()));
        map.insert("completed".to_string(), BindingValue::Bool(self.completed));
        map.insert(
            "category".to_string(),
            BindingValue::String(self.category.clone()),
        );
        map.insert("priority".to_string(), self.priority.to_binding_value());
        BindingValue::Object(map)
    }
}

#[derive(Default, UiModel, Serialize, Deserialize, Clone, Debug)]
pub struct Model {
    #[ui_skip]
    pub items: Vec<TodoItem>,
    pub filtered_items_cache: Vec<TodoItem>,
    #[ui_skip]
    pub next_id: usize,
    pub new_item_text: String,
    pub new_item_category: String,
    pub new_item_priority: Priority,
    pub current_filter: TodoFilter,
    pub search_text: String,
    #[ui_skip]
    pub editing_id: Option<usize>,
    pub edit_text: String,
    pub dark_mode: bool,
    pub selected_category: String,
    #[ui_skip]
    pub selected_priority: Priority,
    pub selected_priority_display: String,
    pub current_filter_display: String,
    pub completed_count: i64,
    pub pending_count: i64,
    pub completion_percentage: i64,
    pub items_len: i64,
    #[ui_skip]
    pub statistics_chart: StatisticsChart,

    // Theme-related fields (for US1 & US2)
    pub current_theme: String,          // "light" or "dark"
    pub theme_transition_progress: f32, // 0.0 to 1.0 for animations
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StatisticsChart {
    pub completion_history: Vec<f32>,
}

#[dampen_ui("window.dampen")]
mod _app {}

fn add_item(model: &mut Model) {
    let trimmed = model.new_item_text.trim();

    // Validate: non-empty after trim and max 500 characters
    if !trimmed.is_empty() && trimmed.len() <= 500 {
        model.items.push(TodoItem {
            id: model.next_id,
            text: trimmed.to_string(),
            completed: false,
            category: model.new_item_category.clone(),
            priority: model.new_item_priority,
        });
        model.next_id += 1;
        model.new_item_text.clear();
        update_computed_fields(model);
    }
}

// T076: Version with SharedContext support
fn add_item_with_shared(
    model: &mut Model,
    shared: &dampen_core::SharedContext<crate::shared::SharedState>,
) {
    add_item(model);
    update_shared_statistics(&model.items, shared);
}

fn toggle_dark_mode(model: &mut Model) {
    model.dark_mode = !model.dark_mode;
    model.current_theme = if model.dark_mode {
        "dark".to_string()
    } else {
        "light".to_string()
    };
    model.theme_transition_progress = 0.0; // Reset animation
}

fn toggle_item(model: &mut Model, id: usize) {
    if let Some(item) = model.items.iter_mut().find(|i| i.id == id) {
        item.completed = !item.completed;
    }
    update_computed_fields(model);
}

// T077: Version with SharedContext support
fn toggle_item_with_shared(
    model: &mut Model,
    id: usize,
    shared: &dampen_core::SharedContext<crate::shared::SharedState>,
) {
    toggle_item(model, id);
    update_shared_statistics(&model.items, shared);
}

fn delete_item(model: &mut Model, id: usize) {
    model.items.retain(|i| i.id != id);
    update_computed_fields(model);
}

// T078: Version with SharedContext support
fn delete_item_with_shared(
    model: &mut Model,
    id: usize,
    shared: &dampen_core::SharedContext<crate::shared::SharedState>,
) {
    delete_item(model, id);
    update_shared_statistics(&model.items, shared);
}

fn update_new_item(model: &mut Model, value: String) {
    model.new_item_text = value;
}

fn update_category(model: &mut Model, value: String) {
    model.selected_category = value.clone();
    model.new_item_category = value;
}

fn update_priority(model: &mut Model, value: String) {
    model.selected_priority_display = value.clone();
    model.selected_priority = match value.as_str() {
        "Low" => Priority::Low,
        "High" => Priority::High,
        _ => Priority::Medium,
    };
    model.new_item_priority = model.selected_priority;
    update_computed_fields(model);
}

fn apply_filter(model: &mut Model, value: String) {
    model.current_filter = match value.as_str() {
        "Active" => TodoFilter::Active,
        "Completed" => TodoFilter::Completed,
        _ => TodoFilter::All,
    };
    update_computed_fields(model);
}

fn update_search(model: &mut Model, value: String) {
    model.search_text = value;
    update_computed_fields(model);
}

fn start_edit(model: &mut Model, id: usize) {
    model.editing_id = Some(id);
    if let Some(item) = model.items.iter().find(|i| i.id == id) {
        model.edit_text = item.text.clone();
    }
}

fn save_edit(model: &mut Model) {
    if let Some(id) = model.editing_id {
        if let Some(item) = model.items.iter_mut().find(|i| i.id == id) {
            item.text = model.edit_text.clone();
        }
        model.editing_id = None;
        model.edit_text.clear();
    }
}

fn cancel_edit(model: &mut Model) {
    model.editing_id = None;
    model.edit_text.clear();
}

fn update_edit_text(model: &mut Model, value: String) {
    model.edit_text = value;
}

fn clear_all(model: &mut Model) {
    model.items.clear();
    model.next_id = 0;
    update_computed_fields(model);
}

// T079: Version with SharedContext support
fn clear_all_with_shared(
    model: &mut Model,
    shared: &dampen_core::SharedContext<crate::shared::SharedState>,
) {
    clear_all(model);
    update_shared_statistics(&model.items, shared);
}

fn clear_completed(model: &mut Model) {
    model.items.retain(|i| !i.completed);
    update_computed_fields(model);
}

// T080: Version with SharedContext support
fn clear_completed_with_shared(
    model: &mut Model,
    shared: &dampen_core::SharedContext<crate::shared::SharedState>,
) {
    clear_completed(model);
    update_shared_statistics(&model.items, shared);
}

#[allow(dead_code)]
fn open_statistics(_model: &mut Model) {
    // Switch to statistics view - handled by register_with_command below
    println!("📊 Switching to statistics view...");
}

/// Update SharedContext with current task statistics
///
/// Call this after any operation that changes task counts (add, delete, toggle)
/// to sync statistics to shared state for multi-window views (e.g., statistics window)
pub fn update_shared_statistics(
    items: &[TodoItem],
    shared: &dampen_core::SharedContext<crate::shared::SharedState>,
) {
    let total = items.len() as i64;
    let completed = items.iter().filter(|i| i.completed).count() as i64;
    let pending = total - completed;
    let completion_percentage = if total > 0 {
        (completed * 100) / total
    } else {
        0
    };

    // Build category breakdown
    let mut categories = std::collections::HashMap::new();
    for item in items {
        *categories.entry(item.category.clone()).or_insert(0) += 1;
    }
    let breakdown: Vec<String> = categories
        .iter()
        .map(|(cat, count)| format!("{}: {}", cat, count))
        .collect();
    let category_breakdown = breakdown.join(", ");

    // Update shared state
    let mut guard = shared.write();
    guard.total_tasks = total;
    guard.completed_tasks = completed;
    guard.pending_tasks = pending;
    guard.completion_percentage = completion_percentage;
    guard.category_breakdown = category_breakdown;
}

fn update_computed_fields(model: &mut Model) {
    let total = model.items.len();
    let completed = model.items.iter().filter(|i| i.completed).count();
    let pending = total - completed;

    model.items_len = total as i64;
    model.completed_count = completed as i64;
    model.pending_count = pending as i64;
    model.completion_percentage = if total > 0 {
        (completed * 100 / total) as i64
    } else {
        0
    };

    let search_lower = model.search_text.to_lowercase();
    model.filtered_items_cache = model
        .items
        .iter()
        .filter(|item| {
            let matches_filter = match model.current_filter {
                TodoFilter::All => true,
                TodoFilter::Active => !item.completed,
                TodoFilter::Completed => item.completed,
            };
            let matches_search = search_lower.is_empty()
                || item.text.to_lowercase().contains(&search_lower)
                || item.category.to_lowercase().contains(&search_lower);
            matches_filter && matches_search
        })
        .cloned()
        .collect();

    model.selected_priority_display = model.selected_priority.to_string();
    model.current_filter_display = model.current_filter.to_string();

    // Initialize theme fields if not set
    if model.current_theme.is_empty() {
        model.current_theme = if model.dark_mode {
            "dark".to_string()
        } else {
            "light".to_string()
        };
    }
}

#[allow(dead_code)]
pub fn create_app_state() -> dampen_core::AppState<Model> {
    let document = _app::document();
    let handler_registry = create_handler_registry();
    let mut state = dampen_core::AppState::with_handlers(document, handler_registry);
    update_computed_fields(&mut state.model);
    state
}

#[allow(dead_code)]
pub fn create_app_state_with_model(model: Model) -> dampen_core::AppState<Model> {
    let document = _app::document();
    let handler_registry = create_handler_registry();
    let mut state = dampen_core::AppState::with_handlers(document, handler_registry);
    state.model = model;
    state
}

/// Create AppState WITH shared context (called by macro)
#[allow(dead_code)]
pub fn create_app_state_with_shared(
    shared: dampen_core::SharedContext<crate::shared::SharedState>,
) -> dampen_core::AppState<Model, crate::shared::SharedState> {
    let document = _app::document();
    let handler_registry = create_handler_registry();
    let mut model = Model::default();
    update_computed_fields(&mut model);
    dampen_core::AppState::with_shared(document, model, handler_registry, shared)
}

pub fn create_handler_registry() -> HandlerRegistry {
    use std::any::Any;
    let registry = HandlerRegistry::new();

    // T076: Simple handlers with SharedContext support
    registry.register_with_shared("add_item", |model: &mut dyn Any, shared: &dyn Any| {
        if let (Some(m), Some(s)) = (
            model.downcast_mut::<Model>(),
            shared.downcast_ref::<dampen_core::SharedContext<crate::shared::SharedState>>(),
        ) {
            add_item_with_shared(m, s);
        }
    });

    // T079: clear_all with SharedContext
    registry.register_with_shared("clear_all", |model: &mut dyn Any, shared: &dyn Any| {
        if let (Some(m), Some(s)) = (
            model.downcast_mut::<Model>(),
            shared.downcast_ref::<dampen_core::SharedContext<crate::shared::SharedState>>(),
        ) {
            clear_all_with_shared(m, s);
        }
    });

    // T080: clear_completed with SharedContext
    registry.register_with_shared(
        "clear_completed",
        |model: &mut dyn Any, shared: &dyn Any| {
            if let (Some(m), Some(s)) = (
                model.downcast_mut::<Model>(),
                shared.downcast_ref::<dampen_core::SharedContext<crate::shared::SharedState>>(),
            ) {
                clear_completed_with_shared(m, s);
            }
        },
    );

    registry.register_simple("toggle_dark_mode", |model: &mut dyn std::any::Any| {
        if let Some(m) = model.downcast_mut::<Model>() {
            toggle_dark_mode(m);
        }
    });

    // T071-T072: Command handler to switch to statistics view
    registry.register_with_command("open_statistics", |_model: &mut dyn std::any::Any| {
        use crate::{CurrentView, Message};
        Box::new(iced::Task::done(Message::SwitchToView(
            CurrentView::Statistics,
        )))
    });

    registry.register_simple("save_edit", |model: &mut dyn std::any::Any| {
        if let Some(m) = model.downcast_mut::<Model>() {
            save_edit(m);
        }
    });

    registry.register_simple("cancel_edit", |model: &mut dyn std::any::Any| {
        if let Some(m) = model.downcast_mut::<Model>() {
            cancel_edit(m);
        }
    });

    // Handlers with string value
    registry.register_with_value(
        "update_new_item",
        |model: &mut dyn std::any::Any, value: Box<dyn std::any::Any>| {
            if let Some(m) = model.downcast_mut::<Model>() {
                if let Ok(s) = value.downcast::<String>() {
                    update_new_item(m, *s);
                }
            }
        },
    );

    registry.register_with_value(
        "update_category",
        |model: &mut dyn std::any::Any, value: Box<dyn std::any::Any>| {
            if let Some(m) = model.downcast_mut::<Model>() {
                if let Ok(s) = value.downcast::<String>() {
                    update_category(m, *s);
                }
            }
        },
    );

    registry.register_with_value(
        "update_priority",
        |model: &mut dyn std::any::Any, value: Box<dyn std::any::Any>| {
            if let Some(m) = model.downcast_mut::<Model>() {
                if let Ok(s) = value.downcast::<String>() {
                    update_priority(m, *s);
                }
            }
        },
    );

    registry.register_with_value(
        "apply_filter",
        |model: &mut dyn std::any::Any, value: Box<dyn std::any::Any>| {
            if let Some(m) = model.downcast_mut::<Model>() {
                if let Ok(s) = value.downcast::<String>() {
                    apply_filter(m, *s);
                }
            }
        },
    );

    registry.register_with_value(
        "update_search",
        |model: &mut dyn std::any::Any, value: Box<dyn std::any::Any>| {
            if let Some(m) = model.downcast_mut::<Model>() {
                if let Ok(s) = value.downcast::<String>() {
                    update_search(m, *s);
                }
            }
        },
    );

    registry.register_with_value(
        "update_edit_text",
        |model: &mut dyn std::any::Any, value: Box<dyn std::any::Any>| {
            if let Some(m) = model.downcast_mut::<Model>() {
                if let Ok(s) = value.downcast::<String>() {
                    update_edit_text(m, *s);
                }
            }
        },
    );

    // T077: Handlers with numeric ID and SharedContext (from "handler_name:id" pattern)
    registry.register_with_value_and_shared(
        "toggle_item",
        |model: &mut dyn Any, value: Box<dyn Any>, shared: &dyn Any| {
            if let (Some(m), Some(s)) = (
                model.downcast_mut::<Model>(),
                shared.downcast_ref::<dampen_core::SharedContext<crate::shared::SharedState>>(),
            ) {
                if value.is::<usize>() {
                    if let Ok(id) = value.downcast::<usize>() {
                        toggle_item_with_shared(m, *id, s);
                    }
                } else if let Ok(str_val) = value.downcast::<String>() {
                    if let Ok(id) = str_val.parse() {
                        toggle_item_with_shared(m, id, s);
                    }
                }
            }
        },
    );

    // T078: delete_item with SharedContext
    registry.register_with_value_and_shared(
        "delete_item",
        |model: &mut dyn Any, value: Box<dyn Any>, shared: &dyn Any| {
            if let (Some(m), Some(s)) = (
                model.downcast_mut::<Model>(),
                shared.downcast_ref::<dampen_core::SharedContext<crate::shared::SharedState>>(),
            ) {
                if value.is::<usize>() {
                    if let Ok(id) = value.downcast::<usize>() {
                        delete_item_with_shared(m, *id, s);
                    }
                } else if let Ok(str_val) = value.downcast::<String>() {
                    if let Ok(id) = str_val.parse() {
                        delete_item_with_shared(m, id, s);
                    }
                }
            }
        },
    );

    registry.register_with_value(
        "start_edit",
        |model: &mut dyn std::any::Any, value: Box<dyn std::any::Any>| {
            if let Some(m) = model.downcast_mut::<Model>() {
                if value.is::<usize>() {
                    if let Ok(id) = value.downcast::<usize>() {
                        start_edit(m, *id);
                    }
                } else if let Ok(s) = value.downcast::<String>() {
                    if let Ok(id) = s.parse() {
                        start_edit(m, id);
                    }
                }
            }
        },
    );

    // New handlers for simplified UI
    registry.register_with_shared(
        "go_home",
        |model: &mut dyn std::any::Any, shared: &dyn Any| {
            if let (Some(m), Some(shared_ctx)) = (
                model.downcast_mut::<Model>(),
                shared.downcast_ref::<dampen_core::SharedContext<crate::shared::SharedState>>(),
            ) {
                println!("🏠 Go home clicked");

                // Process any pending tasks from add_task window
                let mut guard = shared_ctx.write();
                let pending_tasks = guard.take_pending_tasks();
                let task_count = pending_tasks.len();
                drop(guard); // Release lock early

                for task in pending_tasks {
                    let priority = match task.priority.as_str() {
                        "High" => Priority::High,
                        "Low" => Priority::Low,
                        _ => Priority::Medium,
                    };

                    m.items.push(TodoItem {
                        id: m.next_id,
                        text: task.text.clone(),
                        completed: false,
                        priority,
                        category: task.category.clone(),
                    });
                    m.next_id += 1;

                    println!("✅ Added task from add_task window: {}", task.text);
                }

                if task_count > 0 {
                    update_computed_fields(m);
                    update_shared_statistics(&m.items, shared_ctx);
                }
            }
        },
    );

    registry.register_with_command("open_add_task", |_model: &mut dyn std::any::Any| {
        use crate::{CurrentView, Message};
        Box::new(iced::Task::done(Message::SwitchToView(
            CurrentView::AddTask,
        )))
    });

    // Process pending tasks from add_task window via SharedContext
    registry.register_with_shared(
        "process_pending_tasks",
        |model: &mut dyn Any, shared: &dyn Any| {
            if let (Some(m), Some(shared_ctx)) = (
                model.downcast_mut::<Model>(),
                shared.downcast_ref::<dampen_core::SharedContext<crate::shared::SharedState>>(),
            ) {
                let mut guard = shared_ctx.write();
                let pending_tasks = guard.take_pending_tasks();
                let task_count = pending_tasks.len();
                drop(guard); // Release lock early

                for task in pending_tasks {
                    let priority = match task.priority.as_str() {
                        "High" => Priority::High,
                        "Low" => Priority::Low,
                        _ => Priority::Medium,
                    };

                    m.items.push(TodoItem {
                        id: m.next_id,
                        text: task.text.clone(),
                        completed: false,
                        priority,
                        category: task.category.clone(),
                    });
                    m.next_id += 1;

                    println!("✅ Added task from add_task window: {}", task.text);
                }

                if task_count > 0 {
                    update_computed_fields(m);
                    update_shared_statistics(&m.items, shared_ctx);
                }
            }
        },
    );

    registry.register_simple("toggle_sort", |model: &mut dyn std::any::Any| {
        if let Some(_m) = model.downcast_mut::<Model>() {
            println!("↕️ Toggle sort - Not implemented yet");
        }
    });

    registry.register_simple("toggle_menu", |model: &mut dyn std::any::Any| {
        if let Some(_m) = model.downcast_mut::<Model>() {
            println!("☰ Toggle menu - Not implemented yet");
        }
    });

    registry.register_with_value(
        "filter",
        |model: &mut dyn std::any::Any, value: Box<dyn std::any::Any>| {
            if let Some(m) = model.downcast_mut::<Model>() {
                if let Ok(filter) = value.downcast::<String>() {
                    m.current_filter = match filter.as_str() {
                        "all" => TodoFilter::All,
                        "today" => TodoFilter::Active,
                        "upcoming" => TodoFilter::Active,
                        _ => TodoFilter::All,
                    };
                    m.current_filter_display = filter.to_string();
                    update_computed_fields(m);
                    println!("🔍 Filter changed to: {}", filter);
                }
            }
        },
    );

    registry.register_with_value(
        "filter_by_category",
        |model: &mut dyn std::any::Any, value: Box<dyn std::any::Any>| {
            if let Some(m) = model.downcast_mut::<Model>() {
                if let Ok(category) = value.downcast::<String>() {
                    m.selected_category = (*category).clone();
                    // Filter to show only tasks from this category
                    update_computed_fields(m);
                    println!("📁 Filtering by category: {}", category);
                }
            }
        },
    );

    registry
}
