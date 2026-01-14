# Data Model: Showcase Todo Application

**Feature**: 001-showcase-todo-app  
**Date**: 2026-01-14  
**Purpose**: Define entities, relationships, validation rules, and state management

## Overview

This document specifies the data model for the showcase todo application. The model supports task management, theme preferences, multi-window statistics, and shared state synchronization. All entities follow Dampen's UiBindable pattern for seamless XML binding integration.

## Entity Definitions

### 1. Task

**Purpose**: Represents a single todo item with metadata for categorization and priority

**Fields**:

| Field | Type | Description | Validation | Immutable |
|-------|------|-------------|------------|-----------|
| `id` | `usize` | Unique identifier | > 0, unique across all tasks | ✓ (after creation) |
| `text` | `String` | Task description | Non-empty, trimmed, ≤500 chars | |
| `completed` | `bool` | Completion status | true/false | |
| `category` | `String` | Category label | One of: Work, Personal, Shopping, Health, Finance, Other | |
| `priority` | `Priority` (enum) | Priority level | Low \| Medium \| High | |
| `created_at` | `u64` (timestamp) | Creation time (Unix epoch seconds) | > 0 | ✓ |

**Rust Definition**:
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: usize,
    pub text: String,
    pub completed: bool,
    pub category: String,
    pub priority: Priority,
    pub created_at: u64,  // Unix timestamp
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum Priority {
    Low,
    #[default]
    Medium,
    High,
}

impl ToBindingValue for Task {
    fn to_binding_value(&self) -> BindingValue {
        let mut map = HashMap::new();
        map.insert("id".to_string(), BindingValue::Integer(self.id as i64));
        map.insert("text".to_string(), BindingValue::String(self.text.clone()));
        map.insert("completed".to_string(), BindingValue::Bool(self.completed));
        map.insert("category".to_string(), BindingValue::String(self.category.clone()));
        map.insert("priority".to_string(), self.priority.to_binding_value());
        BindingValue::Object(map)
    }
}

impl ToBindingValue for Priority {
    fn to_binding_value(&self) -> BindingValue {
        BindingValue::String(self.as_str().to_string())
    }
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
```

**Relationships**:
- Belongs to: `Model` (main window state)
- Referenced by: `filtered_tasks` (computed field in Model)
- Contributes to: `SharedState` metrics (via computed aggregation)

**Validation Rules**:
- ID must be unique (enforced by sequential `next_id` counter)
- Text must not be empty after trimming whitespace
- Text length ≤500 characters (prevent UI overflow)
- Category must match predefined list (enforced in UI via pick_list)
- Created_at set once on creation, never modified

**State Transitions**:
```
[New] --add_task()--> [Active: completed=false]
       <--toggle_task()--
[Active] --toggle_task()--> [Completed: completed=true]
[Active|Completed] --delete_task()--> [Deleted (removed from Vec)]
```

**Persistence**: Serialized to JSON via serde, saved to `.dampen-state.json`

---

### 2. Theme

**Purpose**: Defines visual appearance settings for the application

**Fields**:

| Field | Type | Description | Validation |
|-------|------|-------------|------------|
| `variant` | `ThemeVariant` (enum) | Light or Dark theme | Light \| Dark |
| `palette` | `ColorPalette` (struct) | Color definitions | All colors valid hex codes |
| `typography` | `Typography` (struct) | Font settings | Font sizes > 0, weights valid |
| `spacing` | `u32` | Spacing grid unit (px) | ≥4, ≤16 (reasonable grid) |

**Rust Definition**:
```rust
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ThemeVariant {
    Light,
    Dark,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorPalette {
    pub primary: String,       // e.g., "#3498db"
    pub secondary: String,
    pub success: String,
    pub warning: String,
    pub danger: String,
    pub background: String,
    pub surface: String,
    pub text: String,
    pub text_secondary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Typography {
    pub font_family: String,
    pub font_size_base: u32,    // in px
    pub font_size_small: u32,
    pub font_size_large: u32,
    pub font_weight: String,    // "normal", "medium", "bold"
    pub line_height: f32,       // e.g., 1.5
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    pub variant: ThemeVariant,
    pub palette: ColorPalette,
    pub typography: Typography,
    pub spacing: u32,  // Base unit for 8px grid
}
```

**Relationships**:
- Stored in: `Model.theme_name` (string identifier: "light" or "dark")
- Defined in: XML `<theme>` elements (parsed by Dampen core)
- Applied to: All windows via `<global_theme>` binding

**Validation Rules**:
- Colors must be valid hex format: `#RRGGBB` (6 hex digits)
- Contrast ratios: ≥4.5:1 for text/background, ≥3:1 for UI components (WCAG AA)
- Font sizes: base ≥12px, small ≥10px, large ≥18px
- Spacing unit: 4px, 8px, or 16px (common grid systems)

**Theme Switching**:
- User toggles `is_dark_mode` boolean in Model
- Handler updates `Model.theme_name` to "light" or "dark"
- Dampen core applies new theme from `<themes>` definition
- Widget tree rebuilds with new palette/typography

**Persistence**: Theme preference saved in `.dampen-state.json` as `theme_name: String`

---

### 3. Statistics (Computed Entity)

**Purpose**: Aggregate metrics derived from task collection, displayed in statistics window

**Fields**:

| Field | Type | Description | Computation |
|-------|------|-------------|-------------|
| `total_count` | `i64` | Total number of tasks | `tasks.len()` |
| `completed_count` | `i64` | Number of completed tasks | `tasks.iter().filter(t => t.completed).count()` |
| `pending_count` | `i64` | Number of pending tasks | `total_count - completed_count` |
| `completion_percentage` | `i64` | Completion rate (0-100) | `(completed_count * 100 / total_count).clamp(0, 100)` or 0 if empty |

**Rust Definition**:
```rust
// Note: Statistics is NOT a separate struct, but computed fields in SharedState
#[derive(Default, Clone, UiModel, Serialize, Deserialize, Debug)]
pub struct SharedState {
    pub total_count: i64,
    pub completed_count: i64,
    pub pending_count: i64,
    pub completion_percentage: i64,
}

// Updated whenever tasks change
fn update_shared_statistics(shared: &SharedContext<SharedState>, tasks: &[Task]) {
    let mut state = shared.write();
    state.total_count = tasks.len() as i64;
    state.completed_count = tasks.iter().filter(|t| t.completed).count() as i64;
    state.pending_count = state.total_count - state.completed_count;
    state.completion_percentage = if state.total_count > 0 {
        (state.completed_count * 100 / state.total_count)
    } else {
        0
    };
}
```

**Relationships**:
- Derived from: `Model.tasks` (Vec<Task>)
- Stored in: `SharedState` (shared across windows via SharedContext)
- Displayed in: Statistics window via `{shared.*}` bindings

**Validation Rules**:
- All counts ≥0 (enforced by unsigned/non-negative types)
- Percentage 0-100 (enforced by computation logic)
- Consistency: `total_count = completed_count + pending_count`

**Update Triggers**:
- `add_task()` → recompute all metrics
- `toggle_task()` → recompute completed/pending/percentage
- `delete_task()` → recompute all metrics
- `clear_all()` → reset all to 0
- `clear_completed()` → recompute all metrics

**Persistence**: Not persisted (recomputed on application start from tasks)

---

### 4. Model (Main Window State)

**Purpose**: Root state for main window, contains task collection and UI state

**Fields**:

| Field | Type | Description | UI Bindable |
|-------|------|-------------|-------------|
| `tasks` | `Vec<Task>` | All tasks | `#[ui_skip]` (use filtered_tasks) |
| `filtered_tasks` | `Vec<Task>` | Filtered/searched tasks | ✓ (for `<for>` loop) |
| `next_id` | `usize` | ID counter for new tasks | `#[ui_skip]` |
| `new_task_text` | `String` | Input field value | ✓ |
| `new_task_category` | `String` | Selected category for new task | ✓ |
| `new_task_priority` | `Priority` | Selected priority for new task | `#[ui_skip]` (use display string) |
| `current_filter` | `TodoFilter` | Active filter (All/Active/Completed) | `#[ui_skip]` |
| `search_text` | `String` | Search query | ✓ |
| `theme_name` | `String` | "light" or "dark" | ✓ (for `<global_theme>` binding) |
| `is_dark_mode` | `bool` | Theme toggle state | ✓ |

**Rust Definition**:
```rust
#[derive(Default, UiModel, Serialize, Deserialize, Clone, Debug)]
pub struct Model {
    #[ui_skip]
    pub tasks: Vec<Task>,
    pub filtered_tasks: Vec<Task>,  // Cached for performance
    #[ui_skip]
    pub next_id: usize,
    pub new_task_text: String,
    pub new_task_category: String,
    #[ui_skip]
    pub new_task_priority: Priority,
    #[ui_skip]
    pub current_filter: TodoFilter,
    pub search_text: String,
    pub theme_name: String,
    pub is_dark_mode: bool,
}

#[derive(Debug, Clone, Copy, Default)]
pub enum TodoFilter {
    #[default]
    All,
    Active,
    Completed,
}
```

**Relationships**:
- Contains: `Vec<Task>` (one-to-many)
- Updates: `SharedState` via handlers
- Persisted: Full model saved to `.dampen-state.json`

**Computed Fields**:
- `filtered_tasks`: Recomputed whenever `tasks`, `current_filter`, or `search_text` changes
- Filtering logic:
  ```rust
  fn update_filtered_tasks(model: &mut Model) {
      let search_lower = model.search_text.to_lowercase();
      model.filtered_tasks = model.tasks
          .iter()
          .filter(|task| {
              // Filter by completion status
              let matches_filter = match model.current_filter {
                  TodoFilter::All => true,
                  TodoFilter::Active => !task.completed,
                  TodoFilter::Completed => task.completed,
              };
              // Filter by search text
              let matches_search = search_lower.is_empty()
                  || task.text.to_lowercase().contains(&search_lower)
                  || task.category.to_lowercase().contains(&search_lower);
              matches_filter && matches_search
          })
          .cloned()
          .collect();
  }
  ```

**Validation Rules**:
- `next_id` always increments (never decrements or reuses)
- `tasks` IDs are unique (enforced by sequential assignment)
- `theme_name` must be "light" or "dark" (enforced by handlers)
- `new_task_text` validated on submit (non-empty, ≤500 chars)

**Persistence**:
- Serialized to JSON: `.dampen-state.json` in application directory
- Saved on: task modifications, theme changes, filter changes
- Loaded on: application startup (or defaults if file missing)

---

### 5. SharedState (Cross-Window State)

**Purpose**: State shared between main and statistics windows, synchronized in real-time

**Fields**:

| Field | Type | Description | Updated By |
|-------|------|-------------|------------|
| `total_count` | `i64` | Total tasks | Main window handlers |
| `completed_count` | `i64` | Completed tasks | Main window handlers |
| `pending_count` | `i64` | Pending tasks | Main window handlers |
| `completion_percentage` | `i64` | Completion rate (0-100) | Main window handlers |

**Rust Definition**:
```rust
#[derive(Default, Clone, UiModel, Serialize, Deserialize, Debug)]
pub struct SharedState {
    pub total_count: i64,
    pub completed_count: i64,
    pub pending_count: i64,
    pub completion_percentage: i64,
}
```

**Relationships**:
- Wrapped in: `SharedContext<SharedState>` (Arc<RwLock<SharedState>>)
- Updated by: Main window handlers (add_task, toggle_task, delete_task, etc.)
- Consumed by: Statistics window via `{shared.*}` bindings

**Threading Model**:
- All access on main thread (Iced requirement)
- RwLock provides safe multi-reader, single-writer semantics
- Arc enables zero-cost cloning (pointer copy, not data copy)

**Synchronization**:
- Updates are synchronous (write lock acquired, modified, released)
- UI updates on next Iced redraw cycle (~16ms at 60 FPS)
- Total latency: <50ms from handler execution to statistics window update

**Validation Rules**:
- All counts ≥0
- Percentage 0-100
- Consistency: `total_count = completed_count + pending_count`

**Persistence**: Optionally saved to `.dampen-state.json` (currently not persisted, recomputed on load)

---

## Relationships Diagram

```
┌─────────────────────────────────────────────────────────────┐
│ Application                                                  │
│ ┌─────────────────────┐         ┌────────────────────────┐  │
│ │ Main Window         │         │ Statistics Window      │  │
│ │                     │         │                        │  │
│ │ Model               │         │ (no local model)       │  │
│ │ ├─ tasks: Vec<Task> │         │                        │  │
│ │ ├─ filtered_tasks   │         │ Bindings:              │  │
│ │ ├─ new_task_text    │         │ ├─ {shared.total}      │  │
│ │ ├─ theme_name       │         │ ├─ {shared.completed}  │  │
│ │ └─ is_dark_mode     │         │ ├─ {shared.pending}    │  │
│ │                     │         │ └─ {shared.percentage} │  │
│ │ Handlers:           │         │                        │  │
│ │ ├─ add_task()       │━━━━━━━┓ │                        │  │
│ │ ├─ toggle_task()    │       ║ │                        │  │
│ │ ├─ delete_task()    │       ║ │                        │  │
│ │ └─ toggle_theme()   │       ║ │                        │  │
│ └─────────────────────┘       ║ └────────────────────────┘  │
│                               ║                             │
│       ┌───────────────────────▼──────────────────────┐      │
│       │ SharedContext<SharedState>                   │      │
│       │ (Arc<RwLock<SharedState>>)                   │      │
│       │ ├─ total_count: i64                          │      │
│       │ ├─ completed_count: i64                      │      │
│       │ ├─ pending_count: i64                        │      │
│       │ └─ completion_percentage: i64                │      │
│       └──────────────────────────────────────────────┘      │
│                                                              │
│       ┌──────────────────────────────────────────────┐      │
│       │ Persistence (.dampen-state.json)              │      │
│       │ {                                             │      │
│       │   "tasks": [...],                             │      │
│       │   "theme_name": "dark",                       │      │
│       │   "is_dark_mode": true,                       │      │
│       │   // SharedState not persisted (recomputed)   │      │
│       │ }                                             │      │
│       └──────────────────────────────────────────────┘      │
└─────────────────────────────────────────────────────────────┘
```

## State Management Patterns

### Initialization

```rust
// In main.rs
#[dampen_app(
    shared_model = "SharedState",  // One-line setup
    ui_dir = "src/ui",
    // ...
)]
struct TodoApp;

// Generated init() creates SharedContext<SharedState>::new(SharedState::default())
// and passes to all views via create_app_state_with_shared()
```

### State Updates (Main Window)

```rust
// Handler updates both local and shared state
fn add_task(model: &mut Model, shared: &SharedContext<SharedState>) {
    if !model.new_task_text.trim().is_empty() {
        // Update local state
        model.tasks.push(Task {
            id: model.next_id,
            text: model.new_task_text.clone(),
            completed: false,
            category: model.new_task_category.clone(),
            priority: model.new_task_priority,
            created_at: unix_timestamp(),
        });
        model.next_id += 1;
        model.new_task_text.clear();
        update_filtered_tasks(model);
        
        // Update shared state
        update_shared_statistics(shared, &model.tasks);
    }
}
```

### State Reading (Statistics Window)

```xml
<!-- statistics.dampen -->
<text value="{shared.total_count}" />
<text value="{shared.completed_count}" />
<text value="{shared.completion_percentage}%" />
```

Bindings resolve via:
```rust
// In Iced backend
let shared_value = app_state.shared_context.read().get_field(&["total_count"]);
// Returns BindingValue::Integer(5) if total_count=5
```

### Hot-Reload Behavior

```rust
// On XML hot-reload:
AppState::hot_reload(&mut self) {
    // 1. Re-parse XML
    let new_document = parse_xml("window.dampen")?;
    self.document = new_document;
    
    // 2. Reset local model to default
    self.model = Model::default();
    
    // 3. Preserve shared context (Arc pointer unchanged)
    // self.shared_context NOT modified
    
    // 4. Re-register handlers
    self.handlers = create_handler_registry();
    
    Ok(())
}
```

**Result**: Tasks lost on hot-reload (local state), but shared metrics preserved (if not recomputed)

**Note**: For better dev experience, consider persisting tasks to file on every change and reloading on hot-reload.

---

## Validation Summary

| Entity | Key Validations | Enforcement |
|--------|-----------------|-------------|
| **Task** | ID unique, text non-empty, ≤500 chars, category valid | Runtime checks in handlers |
| **Theme** | Colors valid hex, contrast ≥4.5:1 text, ≥3:1 UI | Design-time verification (manual testing) |
| **Statistics** | Counts ≥0, percentage 0-100, consistency | Computation logic guarantees |
| **Model** | next_id increments only, theme_name in ["light", "dark"] | Sequential assignment, enum validation |
| **SharedState** | Counts ≥0, percentage 0-100 | Computation logic guarantees |

---

## Performance Considerations

### Memory

- **Task**: ~120 bytes/instance (String allocations dominate)
- **Model**: ~8KB base + (120 bytes × task count)
- **SharedState**: ~32 bytes (4 × i64)
- **Total**: ~10KB for 500 tasks (acceptable)

### Computation

- **Filtering**: O(n) where n = task count, runs on every filter/search change
  - Optimization: Cache filtered_tasks, recompute only on tasks/filter/search change
- **Statistics Update**: O(n) where n = task count, runs on every task modification
  - Acceptable: 500 tasks = ~500 comparisons = <0.1ms
- **Theme Switch**: O(w) where w = widget count, rebuilds entire widget tree
  - Target: <300ms for typical UIs (verified in testing)

### Network/I/O

- **Persistence**: JSON serialization on every state change
  - Debounce: Write to file max once per second
  - Async I/O: Non-blocking save operation
- **Hot-Reload**: File system watcher polling at 100ms intervals
  - Acceptable overhead: <1% CPU usage

---

## Next Steps

With data model complete:
1. Create XML contracts (`contracts/*.md`) specifying widget structure and bindings
2. Document handler signatures and behavior in contracts
3. Create quickstart guide referencing this data model
4. Begin implementation with test-driven approach (entity creation first, then handlers)
