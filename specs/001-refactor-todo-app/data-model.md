# Data Model: Refactor Todo-App to Match Iced Example

**Feature**: 001-refactor-todo-app
**Date**: 2026-01-22
**Phase**: 1 - Design & Contracts

## Overview

The todo-app uses an in-memory data model with three core entities: Task, Filter, and the application Model state. All data is transient (not persisted) which is acceptable for a demo/example application.

## Core Entities

### 1. Task

Represents a single to-do item with a unique identifier, description, completion status, and edit state.

**Fields**:
- `id: Uuid` - Unique identifier for the task (used for edit/delete operations)
- `description: String` - Human-readable task description (max 1000 characters)
- `completed: bool` - Whether the task is completed (checked) or active (unchecked)
- `state: TaskState` - Current edit state (Idle or Editing)

**Constraints**:
- `id`: Must be a valid UUID v4
- `description`: Empty or whitespace-only values are invalid for new tasks
- `description`: Maximum 1000 characters
- `completed`: Default `false` for new tasks
- `state`: Default `Idle` for new tasks

**State Transitions**:

```
        edit_task()
    ┌─────────────────┐
    │                 │
    ▼                 │
Idle  ──────────►  Editing
    ◄──────────        │
    │  save_edit()     │
    │  cancel_edit()   │
    └─────────────────┘
```

**Type Definition**:
```rust
use uuid::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TaskState {
    #[default]
    Idle,
    Editing,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: Uuid,
    pub description: String,
    pub completed: bool,
    #[serde(skip)]  // State not persisted (not needed for this demo)
    pub state: TaskState,
}
```

**Validation Rules**:
- `Task::new(description: String)`: Creates task with `completed = false`, `state = Idle`
- Validates that description is not empty/whitespace before creation
- Generates UUID v4 automatically

---

### 2. Filter

Represents the current view filter which determines which tasks are displayed in the UI.

**Fields**: N/A (enum with variants)

**Variants**:
- `All` - Show all tasks (both completed and incomplete)
- `Active` - Show only incomplete tasks
- `Completed` - Show only completed tasks

**Constraints**:
- Default: `All`
- Can only be one of the three variants

**Type Definition**:
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum Filter {
    #[default]
    All,
    Active,
    Completed,
}

impl Filter {
    pub fn as_str(&self) -> &str {
        match self {
            Filter::All => "All",
            Filter::Active => "Active",
            Filter::Completed => "Completed",
        }
    }
}
```

**Usage**:
- Used to filter the `tasks` list into `filtered_tasks` for display
- Selected via filter buttons in UI
- Affects which tasks are visible and the empty state message

---

### 3. Model

The application state containing all data needed for UI rendering and user interactions.

**Fields**:

| Field | Type | Purpose | Bound to UI |
|-------|------|---------|-------------|
| `input_value` | `String` | Current text in the task input field | ✅ Yes |
| `filter` | `Filter` | Current active filter | ✅ Yes |
| `tasks` | `Vec<Task>` | List of all tasks (master list) | ✅ Yes |
| `editing_id` | `Option<Uuid>` | ID of task currently being edited | ❌ No (internal) |
| `edit_text` | `String` | Text in the active edit input | ✅ Yes |
| `filtered_tasks` | `Vec<Task>` | Tasks matching current filter (computed) | ✅ Yes |
| `tasks_left` | `i64` | Count of incomplete tasks (computed) | ✅ Yes |
| `tasks_left_text` | `String` | Display text "X task(s) left" (computed) | ✅ Yes |
| `empty_message` | `String` | Empty state message (computed) | ✅ Yes |
| `filtered_tasks_len` | `i64` | Count of filtered tasks (computed) | ✅ Yes |

**Constraints**:
- `input_value`: Can be empty or contain whitespace (validated on submit)
- `editing_id`: Only `Some` when a task is in `Editing` state
- `edit_text`: Only meaningful when `editing_id` is `Some`
- Computed fields must be kept in sync with `tasks` and `filter`

**Type Definition**:
```rust
#[derive(Default, UiModel, Serialize, Deserialize, Clone, Debug)]
pub struct Model {
    // User input
    pub input_value: String,

    // Filter selection
    pub filter: Filter,

    // Master task list
    pub tasks: Vec<Task>,

    // Internal edit state
    #[ui_skip]
    pub editing_id: Option<Uuid>,

    // Edit input text
    pub edit_text: String,

    // Computed fields for display
    #[ui_skip]  // Not directly bound, but computed for display
    pub filtered_tasks: Vec<Task>,

    pub tasks_left: i64,
    pub tasks_left_text: String,
    pub empty_message: String,
    pub filtered_tasks_len: i64,
}
```

**Computed Field Logic**:

```rust
fn update_computed_fields(model: &mut Model) {
    // Count incomplete tasks
    let tasks_left = model.tasks.iter().filter(|t| !t.completed).count();
    model.tasks_left = tasks_left as i64;

    // Generate display text
    model.tasks_left_text = format!(
        "{} {} left",
        tasks_left,
        if tasks_left == 1 { "task" } else { "tasks" }
    );

    // Filter tasks based on current filter
    model.filtered_tasks = model.tasks
        .iter()
        .filter(|task| {
            match model.filter {
                Filter::All => true,
                Filter::Active => !task.completed,
                Filter::Completed => task.completed,
            }
        })
        .cloned()
        .collect();

    model.filtered_tasks_len = model.filtered_tasks.len() as i64;

    // Generate empty state message
    model.empty_message = match model.filter {
        Filter::All => "You have not created a task yet...",
        Filter::Active => "All your tasks are done! :D",
        Filter::Completed => "You have not completed a task yet...",
    }
    .to_string();
}
```

**Invariants**:
- `filtered_tasks` is always derived from `tasks` and `filter`
- `tasks_left` is always the count of `!task.completed` in `tasks`
- `editing_id.is_some()` ⇔ There exists a task with `state == Editing`
- When `editing_id.is_some()`, `filtered_tasks` may or may not contain that task (depends on filter)

---

## Entity Relationships

```
┌─────────────────────────────────────────────────────────┐
│                       Model                              │
├─────────────────────────────────────────────────────────┤
│  input_value: String                                    │
│  filter: Filter ◄─────────────┐                         │
│  tasks: Vec<Task> ◄───────────┼───┐                    │
│  editing_id: Option<Uuid> ────┼───┼───┐               │
│  edit_text: String            │   │   │               │
│  ─────────────────────────────┼───┼───┘               │
│  filtered_tasks: Vec<Task> ────┘   │                   │
│  tasks_left: i64                  │                   │
│  tasks_left_text: String          │                   │
│  empty_message: String            │                   │
│  filtered_tasks_len: i64          │                   │
└──────────────────────────────────┼───────────────────┘
                                   │
                                   │ contains
                                   ▼
                              ┌────────────────┐
                              │     Task       │
                              ├────────────────┤
                              │ id: Uuid       │
                              │ description:   │
                              │   String       │
                              │ completed:     │
                              │   bool         │
                              │ state:         │
                              │   TaskState    │
                              └────────────────┘

┌────────────────┐
│    Filter      │
├────────────────┤
│ All            │
│ Active         │
│ Completed      │
└────────────────┘
```

**Relationship Types**:
- **Model → Task**: Composition (Model owns Vec<Task>)
- **Model → Filter**: Composition (Model owns Filter)
- **Task**: Standalone entity (no relationships to other Task objects)
- **Filter**: Enum with no dependencies

---

## State Transitions

### Task Lifecycle

```
Creation → Idle ──► Editing ──► Idle
               │            │
               └──────► Deleted
                    (only in Idle state)
```

**Valid State Changes**:
1. `new → Idle`: When task is created (initial state)
2. `Idle → Editing`: When user clicks edit button
3. `Editing → Idle`: When user saves (Enter) or cancels (Escape)
4. `Idle → Deleted`: When user clicks delete button (task removed from Vec)

**Invalid State Changes**:
- Cannot delete while `Editing` (must save/cancel first)
- Cannot transition from `Editing` to `Deleted` (must save/cancel first)

### Filter Changes

```
All ←──── Active
  │         │
  └─────► Completed
      (any transition allowed)
```

**Valid State Changes**:
- Any filter can transition to any other filter
- Filter change triggers `update_computed_fields()`
- Filter change cancels any active edit (per research decision)

### Edit State (Model-level)

```
editing_id: None  ←────  editing_id: Some(uuid)
    │                         │
    │                         │
    └─────► edit_task() ──────┘
            (uuid)
        cancel_edit()
        save_edit()
        filter_changed()
```

**Valid State Changes**:
- `None → Some(uuid)`: When `edit_task(uuid)` called
- `Some(uuid) → None`: When `save_edit()`, `cancel_edit()`, or `filter_changed()` called

---

## Validation Rules

### Task Creation

```rust
pub fn create_task(model: &mut Model) {
    let trimmed = model.input_value.trim();

    // Validation: Empty or whitespace-only
    if trimmed.is_empty() {
        return;  // Do not create task
    }

    // Validation: Max 1000 characters
    if trimmed.len() > 1000 {
        return;  // Or truncate? Decision: reject to prevent UI issues
    }

    // Create task
    model.tasks.push(Task::new(trimmed.to_string()));
    model.input_value.clear();
    update_computed_fields(model);
}
```

### Task Edit Save

```rust
pub fn save_edit(model: &mut Model) {
    if let Some(id) = model.editing_id {
        if let Some(task) = model.tasks.iter_mut().find(|t| t.id == id) {
            let trimmed = model.edit_text.trim();

            // Validation: Empty or whitespace-only allowed
            // Decision: Allow empty (user can clear description)
            task.description = trimmed.to_string();

            // Validation: Max 1000 characters
            if task.description.len() > 1000 {
                task.description.truncate(1000);
            }

            task.state = TaskState::Idle;
        }

        model.editing_id = None;
        model.edit_text.clear();
        update_computed_fields(model);
    }
}
```

### Task Update (Completion Toggle)

```rust
pub fn toggle_task(model: &mut Model, id: String) {
    if let Ok(uuid) = Uuid::parse_str(&id) {
        if let Some(task) = model.tasks.iter_mut().find(|t| t.id == uuid) {
            // Validation: Task must be in Idle state
            if task.state == TaskState::Idle {
                task.completed = !task.completed;
                update_computed_fields(model);
            }
            // If task is Editing, ignore toggle (user must save/cancel first)
        }
    }
}
```

---

## Serialization

### What is Serialized

The Model is serializable using `serde` to support:
- Future persistence (even though not used currently)
- Testing (snapshot tests of model state)
- Debugging (inspecting model state)

### What is NOT Serialized

- `Task::state`: Marked with `#[serde(skip)]` because edit state is transient
- `Model::editing_id`: Marked with `#[ui_skip]` and not serialized
- `Model::filtered_tasks`: Computed field, derived from `tasks` and `filter`
- `Model::tasks_left`: Computed field
- `Model::tasks_left_text`: Computed field
- `Model::empty_message`: Computed field
- `Model::filtered_tasks_len`: Computed field

**Rationale**: Edit state and computed fields are derived state. If deserializing, they would be recomputed from the serialized `tasks` and `filter`.

---

## Performance Considerations

### Task List Size

**Scale**: Up to 1000 tasks (per SC-002)

**Memory Impact**:
- One Task: ~200 bytes (UUID + String + bool + enum)
- 1000 tasks: ~200 KB
- Model total: ~300-400 KB (including Vec overhead, strings)

**Performance Impact**:
- `update_computed_fields()`: O(n) where n = number of tasks
- Filter operation: O(n) but only runs on state changes
- UI rendering: Iced uses virtualization for scrollable lists

**Optimizations**:
- No additional optimizations needed for 1000 tasks
- Iced handles large lists efficiently with virtualization
- If scaling beyond 10,000 tasks, consider pagination or incremental filtering

---

## Testing Strategy

### Unit Tests

```rust
#[test]
fn test_task_creation() {
    let task = Task::new("Buy milk".to_string());
    assert!(!task.completed);
    assert_eq!(task.state, TaskState::Idle);
}

#[test]
fn test_task_filter_active() {
    let mut model = Model::default();
    model.tasks = vec![
        Task::new("Task 1".to_string()),
        Task::new("Task 2".to_string()),
    ];
    model.tasks[0].completed = true;
    model.filter = Filter::Active;

    update_computed_fields(&mut model);

    assert_eq!(model.filtered_tasks.len(), 1);
    assert_eq!(model.filtered_tasks[0].description, "Task 2");
}
```

### Integration Tests

Manual testing in both interpreted and codegen modes (documented in quickstart.md).

---

## Summary

The data model consists of three core entities:
- **Task**: Individual to-do items with unique IDs, descriptions, completion status, and edit state
- **Filter**: View filter (All/Active/Completed) for filtering tasks
- **Model**: Application state containing input value, filter, task list, edit state, and computed fields

All entities use Rust's type system for compile-time safety. The model is in-memory and not persisted, which is acceptable for a demo application. Computed fields are kept in sync via `update_computed_fields()` function.
