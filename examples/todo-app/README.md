# Todo App Example

A comprehensive modern todo application demonstrating advanced Gravity widgets including ProgressBar, Canvas, Tooltip, PickList, and Image widgets.

## Features

### Widget Demonstration

This example showcases the following Gravity widgets:

- **ProgressBar**: Visual completion tracking showing overall task completion percentage
- **Canvas**: Custom 7-day completion trend visualization using the `canvas::Program` trait
- **Tooltip**: Contextual help text on action buttons
- **PickList**: Dropdown selections for category, priority, and filtering
- **Image**: Priority icons (Low, Medium, High)
- **Toggler**: Dark mode toggle
- **Standard widgets**: Text, Button, TextInput, Row, Column, Scrollable, Rule, Space

### Functionality

- **CRUD Operations**: Add, toggle completion, and delete todo items
- **Category Management**: Organize tasks by category (Work, Personal, Shopping, Health, Finance, Other)
- **Priority Levels**: Assign priority levels (Low, Medium, High) with visual indicators
- **Filtering**: View All, Active, or Completed tasks
- **Statistics**: Real-time completion tracking with progress bar
- **Data Persistence**: State persists across hot-reloads via `.gravity-state.json`
- **Dark Mode**: Toggle between light and dark themes

## Data Model

### TodoItem
```rust
pub struct TodoItem {
    pub id: usize,
    pub text: String,
    pub category: String,
    pub priority: Priority,
    pub completed: bool,
}
```

### Enums
- **Priority**: Low | Medium | High
- **TodoFilter**: All | Active | Completed

## Running the Example

```bash
cd examples/todo-app
cargo run
```

The application will open in a window. You can:
1. Add new tasks using the text input and "Add Task" button
2. Select a category and priority for each task
3. Filter tasks by status using the dropdown
4. Toggle task completion (implementation pending)
5. View overall progress in the progress bar
6. See completion trends in the canvas chart
7. Toggle dark mode using the toggler

## Hot Reload

The todo app supports hot-reload. Try editing `ui/main.gravity` while the app is running:

```bash
# In another terminal
gravity dev --ui ui --file main.gravity
```

Changes to the UI will update automatically without losing your todos!

## Implementation Details

### Event Handlers

All event handlers are registered manually with HandlerRegistry:

- `add_item`: Add new todo with category and priority
- `toggle_item`: Toggle completion status
- `delete_item`: Remove a todo
- `clear_all`: Remove all todos
- `clear_completed`: Remove only completed todos
- `update_category`: Change selected category
- `update_priority`: Change selected priority
- `apply_filter`: Apply filter (All/Active/Completed)
- `toggle_dark_mode`: Toggle dark/light mode
- `update_new_item`: Update new item text input

### Canvas Visualization

The statistics chart implements the `canvas::Program<Message>` trait:

```rust
impl canvas::Program<Message> for StatisticsChart {
    type State = ();
    
    fn draw(&self, ...) -> Vec<canvas::Geometry> {
        // Draw 7-day completion trend
        // - Axes with labels
        // - Data points as circles
        // - Connected lines
    }
}
```

### State Management

The application uses `#[derive(UiModel)]` for automatic binding support:

```rust
#[derive(UiModel, Debug, Clone, Serialize, Deserialize)]
pub struct TodoAppModel {
    pub items: Vec<TodoItem>,
    pub current_filter: TodoFilter,
    pub new_item_text: String,
    pub selected_category: String,
    pub selected_priority: Priority,
    pub dark_mode: bool,
    pub completed_count: i64,
    pub pending_count: i64,
    pub completion_percentage: f32,
    // ...
}
```

## Limitations

- **ComboBox**: Not yet implemented in the widget builder, using PickList instead
- **Grid**: Not yet implemented, using Row layout for headers
- **Float**: Not yet implemented, no floating action button
- **Dynamic Lists**: Current implementation uses placeholder text instead of rendering individual todo items (awaiting list rendering support)

## Next Steps

Once ComboBox, Grid, and dynamic list rendering are implemented, this example will be updated to:

1. Replace PickList with ComboBox for searchable category selection
2. Use Grid layout for proper task table display
3. Render individual todo items with checkboxes
4. Add floating action button for quick task addition

## Screenshots

*Coming soon: Screenshots showing the todo app with various tasks, categories, and the canvas visualization*

## License

This example is part of the Gravity framework and follows the same licensing terms.
