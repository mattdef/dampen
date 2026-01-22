//! Todo application UI module demonstrating Dampen's declarative UI capabilities.
//!
//! This module contains the data model, handlers, and business logic for a todo
//! application that supports task creation, editing, completion, and filtering.

use dampen_core::{AppState, BindingValue, HandlerRegistry, ToBindingValue};
use dampen_macros::{UiModel, dampen_ui, inventory_handlers, ui_handler};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[dampen_ui("window.dampen")]
mod _app {}

/// Filter for displaying tasks based on their completion status.
///
/// Used to control which tasks are visible in the UI.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum Filter {
    /// Show all tasks regardless of completion status
    #[default]
    All,
    /// Show only incomplete tasks
    Active,
    /// Show only completed tasks
    Completed,
}

impl Filter {
    /// Returns the string representation of the filter for UI binding.
    pub fn as_str(&self) -> &str {
        match self {
            Filter::All => "All",
            Filter::Active => "Active",
            Filter::Completed => "Completed",
        }
    }
}

impl ToBindingValue for Filter {
    fn to_binding_value(&self) -> BindingValue {
        BindingValue::String(self.as_str().to_string())
    }
}

/// State of a task indicating whether it's being edited or not.
///
/// Tasks can only be toggled or deleted when in `Idle` state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum TaskState {
    /// Task is not being edited (normal state)
    #[default]
    Idle,
    /// Task is currently being edited
    Editing,
}

impl TaskState {
    /// Returns the string representation of the task state for UI binding.
    pub fn as_str(&self) -> &str {
        match self {
            TaskState::Idle => "Idle",
            TaskState::Editing => "Editing",
        }
    }
}

impl ToBindingValue for TaskState {
    fn to_binding_value(&self) -> BindingValue {
        BindingValue::String(self.as_str().to_string())
    }
}

/// A single todo task with unique identifier, description, and state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    /// Unique identifier for the task
    pub id: Uuid,
    /// Human-readable task description
    pub description: String,
    /// Whether the task is marked as complete
    pub completed: bool,
    /// Current edit state (Idle or Editing)
    pub state: TaskState,
}

impl Task {
    /// Creates a new task with the given description.
    ///
    /// The task is initialized with:
    /// - A random UUID v4
    /// - `completed = false`
    /// - `state = TaskState::Idle`
    pub fn new(description: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            description,
            completed: false,
            state: TaskState::Idle,
        }
    }
}

impl ToBindingValue for Task {
    fn to_binding_value(&self) -> BindingValue {
        let mut map = HashMap::new();
        map.insert("id".to_string(), BindingValue::String(self.id.to_string()));
        map.insert(
            "description".to_string(),
            BindingValue::String(self.description.clone()),
        );
        map.insert("completed".to_string(), BindingValue::Bool(self.completed));
        map.insert("state".to_string(), self.state.to_binding_value());
        BindingValue::Object(map)
    }
}

/// Application state model containing all data for the todo app.
///
/// This struct uses the `#[derive(UiModel)]` macro to enable data binding
/// with the declarative UI defined in `window.dampen`.
///
/// Fields marked with `#[ui_skip]` are not directly bound to the UI but are
/// used internally for state management.
#[derive(Default, UiModel, Serialize, Deserialize, Clone, Debug)]
pub struct Model {
    /// Current text in the task input field
    pub input_value: String,
    /// Active filter (All, Active, or Completed)
    pub filter: Filter,
    /// Master list of all tasks
    #[ui_skip]
    pub tasks: Vec<Task>,
    /// ID of the task currently being edited (if any)
    #[ui_skip]
    pub editing_id: Option<Uuid>,
    /// Text in the edit input field
    pub edit_text: String,

    // Computed fields (derived from tasks and filter)
    /// Tasks matching the current filter
    pub filtered_tasks: Vec<Task>,
    /// Count of incomplete tasks
    pub tasks_left: i64,
    /// Display text "X item(s) left"
    pub tasks_left_text: String,
    /// Empty state message based on current filter
    pub empty_message: String,
    /// Count of filtered tasks (for conditional rendering)
    pub filtered_tasks_len: i64,
}

/// Updates all computed fields based on current tasks and filter.
///
/// This function must be called after any operation that modifies the task list
/// or filter to keep the UI in sync with the model state.
fn update_computed_fields(model: &mut Model) {
    let tasks = &model.tasks;
    let filter = model.filter;

    // Update tasks_left count (active tasks)
    let active_count = tasks.iter().filter(|t| !t.completed).count();
    model.tasks_left = active_count as i64;
    model.tasks_left_text = format!(
        "{} item{} left",
        active_count,
        if active_count == 1 { "" } else { "s" }
    );

    // Update filtered tasks
    model.filtered_tasks = tasks
        .iter()
        .filter(|t| match filter {
            Filter::All => true,
            Filter::Active => !t.completed,
            Filter::Completed => t.completed,
        })
        .cloned()
        .collect();

    model.filtered_tasks_len = model.filtered_tasks.len() as i64;

    // Update empty message based on filter and count
    model.empty_message = if tasks.is_empty() {
        "You have no tasks yet. Add some above!".to_string()
    } else {
        match filter {
            Filter::All => "No tasks found.".to_string(),
            Filter::Active => "No active tasks.".to_string(),
            Filter::Completed => "No completed tasks yet.".to_string(),
        }
    };
}

/// Handler called when the user types in the task input field.
///
/// Updates the `input_value` field in real-time as the user types.
#[ui_handler]
pub fn input_changed(model: &mut Model, value: String) {
    model.input_value = value;
}

/// Handler called when the user submits the task input (presses Enter).
///
/// Creates a new task if the input is not empty or whitespace-only.
/// Clears the input field and updates computed fields after creation.
#[ui_handler]
pub fn create_task(model: &mut Model) {
    let description = model.input_value.trim();
    if !description.is_empty() {
        model.tasks.push(Task::new(description.to_string()));
        model.input_value.clear();
        update_computed_fields(model);
    }
}

/// Handler called when the user clicks the delete button on a task.
///
/// Deletes the task only if it's in `Idle` state (not being edited).
/// Updates computed fields after deletion.
#[ui_handler]
pub fn delete_task(model: &mut Model, id_str: String) {
    if let Ok(id) = Uuid::parse_str(&id_str)
        && let Some(pos) = model.tasks.iter().position(|t| t.id == id)
    {
        // Prevent deleting while not editing
        if model.tasks[pos].state == TaskState::Editing {
            model.tasks.remove(pos);
            update_computed_fields(model);
        }
    }
}

/// Handler called when the user toggles the completion checkbox on a task.
///
/// Toggles the `completed` status only if the task is in `Idle` state.
/// Updates computed fields to reflect the change in active task count.
#[ui_handler]
pub fn toggle_task(model: &mut Model, id_str: String) {
    if let Ok(id) = Uuid::parse_str(&id_str)
        && let Some(task) = model.tasks.iter_mut().find(|t| t.id == id)
        && task.state == TaskState::Idle
    {
        task.completed = !task.completed;
        update_computed_fields(model);
    }
}

/// Handler called when the user clicks a filter button (All/Active/Completed).
///
/// Cancels any active edit before changing the filter to prevent editing
/// a task that may become hidden by the new filter.
#[ui_handler]
pub fn filter_changed(model: &mut Model, value: String) {
    cancel_edit(model);
    model.filter = match value.as_str() {
        "Active" => Filter::Active,
        "Completed" => Filter::Completed,
        _ => Filter::All,
    };
    update_computed_fields(model);
}

/// Handler called when the user clicks the edit button on a task.
///
/// Enters edit mode for the specified task, canceling any other active edit.
/// Copies the task description to `edit_text` for modification.
#[ui_handler]
pub fn edit_task(model: &mut Model, id_str: String) {
    if let Ok(id) = Uuid::parse_str(&id_str) {
        if model.editing_id.is_some() {
            cancel_edit(model);
        }

        if let Some(task) = model.tasks.iter_mut().find(|t| t.id == id) {
            model.editing_id = Some(id);
            model.edit_text = task.description.clone();
            task.state = TaskState::Editing;
            update_computed_fields(model);
        }
    }
}

#[ui_handler]
pub fn save_edit(model: &mut Model) {
    if let Some(id) = model.editing_id {
        if let Some(task) = model.tasks.iter_mut().find(|t| t.id == id) {
            let trimmed = model.edit_text.trim();
            if !trimmed.is_empty() {
                task.description = trimmed.to_string();
            }
            task.state = TaskState::Idle;
        }
        model.editing_id = None;
        model.edit_text.clear();
        update_computed_fields(model);
    }
}

#[ui_handler]
pub fn cancel_edit(model: &mut Model) {
    if let Some(id) = model.editing_id {
        if let Some(task) = model.tasks.iter_mut().find(|t| t.id == id) {
            task.state = TaskState::Idle;
        }
        model.editing_id = None;
        model.edit_text.clear();
        update_computed_fields(model);
    }
}

#[ui_handler]
pub fn update_edit_text(model: &mut Model, value: String) {
    model.edit_text = value;
}

// Declare all handlers for codegen mode
inventory_handlers! {
    input_changed,
    create_task,
    delete_task,
    toggle_task,
    filter_changed,
    edit_task,
    save_edit,
    cancel_edit,
    update_edit_text
}

/// Creates the initial application state for the todo app.
///
/// This function initializes:
/// - The UI document from `window.dampen`
/// - The handler registry with all UI event handlers
/// - The model with default values and computed fields
///
/// # Returns
///
/// An `AppState` ready to be used with the Iced application.
pub fn create_app_state() -> AppState<Model> {
    let document = _app::document();
    let handler_registry = create_handler_registry();
    let mut model = Model::default();
    update_computed_fields(&mut model);
    let mut state = AppState::with_handlers(document, handler_registry);
    state.model = model;
    state
}

/// Creates and populates the handler registry with all UI event handlers.
///
/// Registers the following handlers:
/// - `input_changed`: Updates input field text
/// - `create_task`: Creates a new task
/// - `delete_task`: Deletes a task by ID
/// - `toggle_task`: Toggles task completion status
/// - `filter`: Changes the active filter
/// - `edit_task`: Enters edit mode for a task
/// - `save_edit`: Saves the edited task description
/// - `cancel_edit`: Cancels the active edit
/// - `update_edit_text`: Updates the edit input text
///
/// # Returns
///
/// A fully configured `HandlerRegistry`.
pub fn create_handler_registry() -> HandlerRegistry {
    let registry = HandlerRegistry::new();

    registry.register_with_value(
        "input_changed",
        |model: &mut dyn std::any::Any, value: Box<dyn std::any::Any>| {
            if let Some(m) = model.downcast_mut::<Model>()
                && let Ok(s) = value.downcast::<String>()
            {
                input_changed(m, *s);
            }
        },
    );

    registry.register_simple("create_task", |model: &mut dyn std::any::Any| {
        if let Some(m) = model.downcast_mut::<Model>() {
            create_task(m);
        }
    });

    registry.register_with_value(
        "delete_task",
        |model: &mut dyn std::any::Any, value: Box<dyn std::any::Any>| {
            if let Some(m) = model.downcast_mut::<Model>()
                && let Ok(s) = value.downcast::<String>()
            {
                delete_task(m, *s);
            }
        },
    );

    registry.register_with_value(
        "toggle_task",
        |model: &mut dyn std::any::Any, value: Box<dyn std::any::Any>| {
            if let Some(m) = model.downcast_mut::<Model>()
                && let Ok(s) = value.downcast::<String>()
            {
                toggle_task(m, *s);
            }
        },
    );

    registry.register_with_value(
        "filter",
        |model: &mut dyn std::any::Any, value: Box<dyn std::any::Any>| {
            if let Some(m) = model.downcast_mut::<Model>()
                && let Ok(s) = value.downcast::<String>()
            {
                filter_changed(m, *s);
            }
        },
    );

    registry.register_with_value(
        "edit_task",
        |model: &mut dyn std::any::Any, value: Box<dyn std::any::Any>| {
            if let Some(m) = model.downcast_mut::<Model>()
                && let Ok(s) = value.downcast::<String>()
            {
                edit_task(m, *s);
            }
        },
    );

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

    registry.register_with_value(
        "update_edit_text",
        |model: &mut dyn std::any::Any, value: Box<dyn std::any::Any>| {
            if let Some(m) = model.downcast_mut::<Model>()
                && let Ok(s) = value.downcast::<String>()
            {
                update_edit_text(m, *s);
            }
        },
    );

    registry
}
