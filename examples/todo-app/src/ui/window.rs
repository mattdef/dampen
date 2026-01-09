// Auto-loaded UI module for todo-app example.

use dampen_core::{BindingValue, HandlerRegistry, ToBindingValue};
use dampen_macros::{dampen_ui, UiModel};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

pub fn save_to_path(model: &Model, path: &PathBuf) {
    if let Ok(data) = serde_json::to_string_pretty(model) {
        let _ = std::fs::write(path, data);
    }
}

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Priority {
    Low,
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

impl Default for Priority {
    fn default() -> Self {
        Priority::Medium
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TodoFilter {
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

    pub fn matches(&self, completed: bool) -> bool {
        match self {
            TodoFilter::All => true,
            TodoFilter::Active => !completed,
            TodoFilter::Completed => completed,
        }
    }
}

impl Default for TodoFilter {
    fn default() -> Self {
        TodoFilter::All
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
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StatisticsChart {
    pub completion_history: Vec<f32>,
}

#[dampen_ui("window.dampen")]
mod _app {}

fn add_item(model: &mut Model) {
    if !model.new_item_text.trim().is_empty() {
        model.items.push(TodoItem {
            id: model.next_id,
            text: model.new_item_text.clone(),
            completed: false,
            category: model.new_item_category.clone(),
            priority: model.new_item_priority,
        });
        model.next_id += 1;
        model.new_item_text.clear();
        update_computed_fields(model);
    }
}

fn toggle_dark_mode(model: &mut Model) {
    model.dark_mode = !model.dark_mode;
}

fn toggle_item(model: &mut Model, id: usize) {
    if let Some(item) = model.items.iter_mut().find(|i| i.id == id) {
        item.completed = !item.completed;
    }
    update_computed_fields(model);
}

fn delete_item(model: &mut Model, id: usize) {
    model.items.retain(|i| i.id != id);
    update_computed_fields(model);
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

fn clear_completed(model: &mut Model) {
    model.items.retain(|i| !i.completed);
    update_computed_fields(model);
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
}

pub fn create_app_state() -> dampen_core::AppState<Model> {
    let document = _app::document();
    let handler_registry = create_handler_registry();
    let mut state = dampen_core::AppState::with_handlers(document, handler_registry);
    update_computed_fields(&mut state.model);
    state
}

pub fn create_app_state_with_model(model: Model) -> dampen_core::AppState<Model> {
    let document = _app::document();
    let handler_registry = create_handler_registry();
    let mut state = dampen_core::AppState::with_handlers(document, handler_registry);
    state.model = model;
    state
}

pub fn create_handler_registry() -> HandlerRegistry {
    let registry = HandlerRegistry::new();

    // Simple handlers (no parameters)
    registry.register_simple("add_item", |model: &mut dyn std::any::Any| {
        if let Some(m) = model.downcast_mut::<Model>() {
            add_item(m);
        }
    });

    registry.register_simple("clear_all", |model: &mut dyn std::any::Any| {
        if let Some(m) = model.downcast_mut::<Model>() {
            clear_all(m);
        }
    });

    registry.register_simple("clear_completed", |model: &mut dyn std::any::Any| {
        if let Some(m) = model.downcast_mut::<Model>() {
            clear_completed(m);
        }
    });

    registry.register_simple("toggle_dark_mode", |model: &mut dyn std::any::Any| {
        if let Some(m) = model.downcast_mut::<Model>() {
            toggle_dark_mode(m);
        }
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

    // Handlers with numeric ID (from "handler_name:id" pattern)
    registry.register_with_value(
        "toggle_item",
        |model: &mut dyn std::any::Any, value: Box<dyn std::any::Any>| {
            if let Some(m) = model.downcast_mut::<Model>() {
                if value.is::<usize>() {
                    if let Ok(id) = value.downcast::<usize>() {
                        toggle_item(m, *id);
                    }
                } else if let Ok(s) = value.downcast::<String>() {
                    if let Ok(id) = s.parse() {
                        toggle_item(m, id);
                    }
                }
            }
        },
    );

    registry.register_with_value(
        "delete_item",
        |model: &mut dyn std::any::Any, value: Box<dyn std::any::Any>| {
            if let Some(m) = model.downcast_mut::<Model>() {
                if value.is::<usize>() {
                    if let Ok(id) = value.downcast::<usize>() {
                        delete_item(m, *id);
                    }
                } else if let Ok(s) = value.downcast::<String>() {
                    if let Ok(id) = s.parse() {
                        delete_item(m, id);
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

    registry
}
