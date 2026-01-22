# Contracts: Refactor Todo-App to Match Iced Example

**Feature**: 001-refactor-todo-app
**Date**: 2026-01-22

## Overview

This directory contains contract definitions for the todo-app example application. Since this is a UI demo application without external APIs or database, traditional API contracts (REST, GraphQL) are not applicable.

Instead, this directory documents:

1. **Handler Contracts** - Expected handler functions for UI interactions
2. **Binding Contracts** - Expected data bindings in UI XML
3. **Message Contracts** - Expected message types for Iced integration

---

## Handler Contracts

### Overview

Handlers are Rust functions that respond to UI events. They are decorated with `#[ui_handler]` and registered with the HandlerRegistry.

### Handler Signature Contract

All handlers must follow this pattern:
```rust
#[ui_handler]
pub fn handler_name(model: &mut Model, [arg: ArgType, ...]) {
    // implementation
}
```

**Requirements**:
- First parameter: `&mut Model` (mutable reference to app state)
- Additional parameters: Values from UI (e.g., `String`, `i64`)
- Return type: `()` (void)
- Must use `#[ui_handler]` macro for registration

### Handler List

| Handler Name | Parameters | Purpose |
|--------------|------------|---------|
| `input_changed` | `value: String` | Update input field text as user types |
| `create_task` | (none) | Create new task from input_value |
| `toggle_task` | `id: String` (UUID string) | Toggle task completion status |
| `edit_task` | `id: String` (UUID string) | Enter edit mode for task |
| `save_edit` | (none) | Save edited task description |
| `cancel_edit` | (none) | Cancel active edit |
| `update_edit_text` | `value: String` | Update edit input text |
| `delete_task` | `id: String` (UUID string) | Delete task from list |
| `filter_changed` | `value: String` ("All"/"Active"/"Completed") | Change current filter |

### Handler Contracts

#### input_changed

**Signature**:
```rust
#[ui_handler]
pub fn input_changed(model: &mut Model, value: String);
```

**Input**:
- `value: String` - Current text in input field

**Output**: Updates `model.input_value`

**Preconditions**: None

**Postconditions**: `model.input_value == value`

**Side Effects**: None (no computed fields updated)

---

#### create_task

**Signature**:
```rust
#[ui_handler]
pub fn create_task(model: &mut Model);
```

**Input**: None (uses `model.input_value`)

**Output**: Creates new task if `input_value` is not empty/whitespace

**Preconditions**: `model.input_value` contains text (may be whitespace)

**Postconditions**:
- If `model.input_value.trim().is_empty()`:
  - No task created
  - `model.input_value` unchanged
- If `model.input_value.trim()` is valid:
  - New task added to `model.tasks`
  - `task.id` is valid UUID v4
  - `task.description` == `model.input_value.trim()`
  - `task.completed == false`
  - `task.state == TaskState::Idle`
  - `model.input_value` cleared to empty string
  - `update_computed_fields(model)` called

**Side Effects**: `model.tasks` modified, computed fields updated

---

#### toggle_task

**Signature**:
```rust
#[ui_handler]
pub fn toggle_task(model: &mut Model, id: String);
```

**Input**:
- `id: String` - UUID string of task to toggle

**Output**: Toggles task completion status

**Preconditions**:
- `id` must be valid UUID string
- Task with that ID exists in `model.tasks`
- Task must be in `Idle` state (not `Editing`)

**Postconditions**:
- If preconditions not met: No changes
- If preconditions met:
  - `task.completed` flipped (`true → false`, `false → true`)
  - `update_computed_fields(model)` called

**Side Effects**: `model.tasks` modified, computed fields updated

---

#### edit_task

**Signature**:
```rust
#[ui_handler]
pub fn edit_task(model: &mut Model, id: String);
```

**Input**:
- `id: String` - UUID string of task to edit

**Output**: Enters edit mode for task

**Preconditions**:
- `id` must be valid UUID string
- Task with that ID exists in `model.tasks`
- Task must be in `Idle` state
- No other task is currently being edited (`model.editing_id.is_none()`)

**Postconditions**:
- If preconditions not met: No changes
- If preconditions met:
  - `model.editing_id == Some(uuid)`
  - `task.state == TaskState::Editing`
  - `model.edit_text == task.description`
  - `update_computed_fields(model)` called

**Side Effects**: `model.tasks` modified, `model.editing_id` set, `model.edit_text` set, computed fields updated

---

#### save_edit

**Signature**:
```rust
#[ui_handler]
pub fn save_edit(model: &mut Model);
```

**Input**: None (uses `model.editing_id` and `model.edit_text`)

**Output**: Saves edited task description and exits edit mode

**Preconditions**: `model.editing_id.is_some()`

**Postconditions**:
- If precondition not met: No changes
- If precondition met:
  - `task.description == model.edit_text.trim()`
  - `task.state == TaskState::Idle`
  - `model.editing_id == None`
  - `model.edit_text` cleared
  - `update_computed_fields(model)` called

**Side Effects**: `model.tasks` modified, `model.editing_id` cleared, `model.edit_text` cleared, computed fields updated

---

#### cancel_edit

**Signature**:
```rust
#[ui_handler]
pub fn cancel_edit(model: &mut Model);
```

**Input**: None (uses `model.editing_id`)

**Output**: Cancels active edit without saving

**Preconditions**: `model.editing_id.is_some()`

**Postconditions**:
- If precondition not met: No changes
- If precondition met:
  - `task.state == TaskState::Idle`
  - `model.editing_id == None`
  - `model.edit_text` cleared
  - `update_computed_fields(model)` called

**Side Effects**: `model.tasks` modified, `model.editing_id` cleared, `model.edit_text` cleared, computed fields updated

---

#### update_edit_text

**Signature**:
```rust
#[ui_handler]
pub fn update_edit_text(model: &mut Model, value: String);
```

**Input**:
- `value: String` - Current text in edit input field

**Output**: Updates edit text

**Preconditions**: `model.editing_id.is_some()`

**Postconditions**:
- `model.edit_text == value`

**Side Effects**: None (no computed fields updated)

---

#### delete_task

**Signature**:
```rust
#[ui_handler]
pub fn delete_task(model: &mut Model, id: String);
```

**Input**:
- `id: String` - UUID string of task to delete

**Output**: Removes task from list

**Preconditions**:
- `id` must be valid UUID string
- Task with that ID exists in `model.tasks`
- Task must be in `Idle` state (cannot delete while editing)

**Postconditions**:
- If preconditions not met: No changes
- If preconditions met:
  - Task removed from `model.tasks`
  - `update_computed_fields(model)` called

**Side Effects**: `model.tasks` modified, computed fields updated

---

#### filter_changed

**Signature**:
```rust
#[ui_handler]
pub fn filter_changed(model: &mut Model, value: String);
```

**Input**:
- `value: String` - "All", "Active", or "Completed"

**Output**: Changes current filter

**Preconditions**: None

**Postconditions**:
- `model.filter` matches `value`:
  - "Active" → `Filter::Active`
  - "Completed" → `Filter::Completed`
  - Anything else → `Filter::All`
- If `model.editing_id.is_some()`:
  - `cancel_edit(model)` called first (to preserve state consistency)
- `update_computed_fields(model)` called

**Side Effects**: `model.filter` changed, active edit cancelled, computed fields updated

---

## Binding Contracts

### Overview

Dampen uses data binding syntax `{variable}` in XML to bind UI elements to model fields. This section documents expected bindings in `window.dampen`.

### Required Bindings

| Binding Name | Type | Source | Purpose |
|--------------|------|--------|---------|
| `input_value` | `String` | `model.input_value` | Text in task input field |
| `tasks_left_text` | `String` | `model.tasks_left_text` | Display text "X task(s) left" |
| `empty_message` | `String` | `model.empty_message` | Empty state message |
| `filtered_tasks` | `Vec<Task>` | `model.filtered_tasks` | Tasks to display in list |
| `filtered_tasks_len` | `i64` | `model.filtered_tasks_len` | Count of filtered tasks (for conditional rendering) |
| `task` | `Task` | Iteration variable in `<for>` | Current task in loop |
| `task.id` | `String` | `task.id.to_string()` | Task ID (passed to handlers) |
| `task.description` | `String` | `task.description` | Task description (display) |
| `task.completed` | `bool` | `task.completed` | Task completion status |
| `task.state` | `TaskState` | `task.state` | Task edit state (for conditional rendering) |
| `edit_text` | `String` | `model.edit_text` | Text in edit input field |

### Binding Usage Examples

#### Text Input Binding

```xml
<text_input
    id="new-task"
    value="{input_value}"
    on_input="input_changed"
    on_submit="create_task"
    placeholder="What needs to be done?"
/>
```

**Contract**:
- `value="{input_value}"` - Binds to `model.input_value`
- `on_input="input_changed"` - Calls `input_changed(value)` on each keystroke
- `on_submit="create_task"` - Calls `create_task()` on Enter

#### Checkbox Binding

```xml
<checkbox
    checked="{task.completed}"
    on_change="toggle_task:{task.id}"
    label="{task.description}"
/>
```

**Contract**:
- `checked="{task.completed}"` - Binds to `task.completed`
- `on_change="toggle_task:{task.id}"` - Calls `toggle_task(id)` when clicked
- `label="{task.description}"` - Displays task description

#### Conditional Rendering

```xml
<if test="{task.state == 'Editing'}">
    <text_input value="{edit_text}" on_input="update_edit_text" />
</if>
```

**Contract**:
- `test="{task.state == 'Editing'}"` - Compares `task.state` to string "Editing"
- Rendered only when condition is true

---

## Message Contracts

### Overview

The `Message` enum defines all messages that can be sent to the Iced application. This section documents expected message types.

### Message Enum

```rust
#[derive(Clone, Debug)]
pub enum Message {
    Handler(HandlerMessage),              // UI handler messages
    #[cfg(debug_assertions)]
    HotReload(FileEvent),                  // Hot-reload event (dev mode)
    #[cfg(debug_assertions)]
    DismissError,                         // Dismiss error message (dev mode)
}
```

### Message Variants

| Variant | Purpose | When Sent |
|---------|---------|-----------|
| `Handler(HandlerMessage)` | UI interaction | User clicks, types, etc. |
| `HotReload(FileEvent)` | File changed | XML file modified in dev mode |
| `DismissError` | Close error | User dismisses error dialog |

### HandlerMessage

Generated by `dampen-macros` based on handler registrations. Each handler becomes a variant:

```rust
pub enum HandlerMessage {
    InputChanged(String),    // from input_changed handler
    CreateTask,              // from create_task handler
    ToggleTask(String),       // from toggle_task handler
    EditTask(String),         // from edit_task handler
    SaveEdit,                // from save_edit handler
    CancelEdit,              // from cancel_edit handler
    UpdateEditText(String),   // from update_edit_text handler
    DeleteTask(String),      // from delete_task handler
    FilterChanged(String),    // from filter_changed handler
}
```

### Message Flow

```
User Action (click, type, etc.)
    ↓
UI Widget (button, text_input, checkbox)
    ↓
Handler invocation (e.g., toggle_task("uuid"))
    ↓
HandlerMessage::ToggleTask("uuid")
    ↓
Message::Handler(HandlerMessage::ToggleTask("uuid"))
    ↓
Application::update(message)
    ↓
Model mutation + UI re-render
```

---

## Integration Contracts

### #[dampen_app] Macro Contract

The `#[dampen_app]` macro generates the Iced application boilerplate. Expected attributes:

```rust
#[dampen_app(
    ui_dir = "src/ui",
    message_type = "Message",
    handler_variant = "Handler",
    hot_reload_variant = "HotReload",
    dismiss_error_variant = "DismissError",
    default_view = "window"
)]
struct TodosApp;
```

**Contract**:
- `ui_dir`: Directory containing `.dampen` XML files
- `message_type`: Name of Message enum type
- `handler_variant`: Variant name for handler messages
- `hot_reload_variant`: Variant name for hot-reload messages
- `dismiss_error_variant`: Variant name for dismiss error messages
- `default_view`: Default view to load

### AppState Contract

The `AppState<T>` struct (from `dampen-core`) holds the application state:

```rust
pub struct AppState<M> {
    pub document: Document,
    pub handler_registry: HandlerRegistry,
    pub model: M,
}
```

**Contract**:
- `document`: Parsed XML document from `window.dampen`
- `handler_registry`: Registered handlers (input_changed, create_task, etc.)
- `model`: Application model (TaskModel with all fields)

---

## No API Contracts

This application does not expose any external APIs (REST, GraphQL, etc.) because:
- It is a demo/example application
- All state is in-memory
- No persistence layer
- No client-server communication

If the application were to be extended with persistence or API endpoints, appropriate contracts would be added to this directory.

---

## Summary

This contracts directory documents:
- **Handler Contracts**: 9 handlers with preconditions, postconditions, and side effects
- **Binding Contracts**: Expected data bindings in UI XML
- **Message Contracts**: Message types for Iced integration
- **Integration Contracts**: Macro and AppState contracts

No traditional API contracts are needed for this UI demo application.
