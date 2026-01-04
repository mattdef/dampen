# Data Model: Advanced Widgets for Modern Todo App

**Feature**: 004-advanced-widgets-todo  
**Date**: 2026-01-04

## Overview

This document defines the data structures and state management for the advanced widgets feature and the todo-app example. It covers both the widget IR extensions in gravity-core and the application model for the todo app.

---

## Core Widget IR Extensions

### 1. WidgetKind Enum Extensions

**Location**: `gravity-core/src/ir/node.rs`

```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum WidgetKind {
    // ... existing variants ...
    Column,
    Row,
    Container,
    Scrollable,
    Stack,
    Text,
    Image,        // Already exists
    Svg,
    Button,
    TextInput,
    Checkbox,
    Slider,
    PickList,     // Already exists
    Toggler,
    Space,
    Rule,
    
    // NEW VARIANTS
    ComboBox,      // Searchable dropdown
    ProgressBar,   // Progress indicator
    Tooltip,       // Hover help text
    Grid,          // Multi-column layout
    Canvas,        // Custom drawing surface
    Float,         // Positioned overlay (or Pin)
    
    Custom(String),
}
```

**Validation Rules**:
- All variants must have corresponding parser patterns
- All variants must have rendering logic in GravityWidgetBuilder
- Variant names must match XML element names (snake_case to lowercase)

---

### 2. Widget Attributes by Type

#### ComboBox Attributes

```rust
// Expected attributes in WidgetNode.attributes HashMap
pub struct ComboBoxAttributes {
    pub options: Vec<String>,           // Required: Comma-separated in XML
    pub selected: Option<BindingExpr>,  // Optional: Binding to selected value
    pub placeholder: Option<String>,    // Optional: Default ""
    pub on_select: Option<String>,      // Optional: Handler name
}
```

**Parsing Logic**:
```xml
<combobox 
    options="Work,Personal,Shopping,Other"
    selected="{current_category}"
    placeholder="Select category..."
    on_select="update_category"
/>
```

**Attribute Validation**:
- `options` is required, must not be empty
- `selected` must be valid BindingExpr if present
- `on_select` must reference a registered handler if present

---

#### PickList Attributes

```rust
pub struct PickListAttributes {
    pub options: Vec<String>,           // Required: Comma-separated in XML
    pub selected: Option<BindingExpr>,  // Optional: Binding to selected value
    pub placeholder: Option<String>,    // Optional: Default ""
    pub on_select: Option<String>,      // Optional: Handler name
}
```

**Note**: Identical structure to ComboBox, different rendering widget

---

#### Canvas Attributes

```rust
pub struct CanvasAttributes {
    pub width: f32,                     // Required: Canvas width in pixels
    pub height: f32,                    // Required: Canvas height in pixels
    pub program: Option<BindingExpr>,   // Required: Binding to Program impl
    pub on_click: Option<String>,       // Optional: Click handler
}
```

**Parsing Logic**:
```xml
<canvas 
    width="400" 
    height="200" 
    program="{statistics_chart}"
    on_click="canvas_clicked"
/>
```

**Special Handling**:
- `program` binding must resolve to type implementing `canvas::Program<Message>`
- Click handler receives coordinates as `(f32, f32)` via custom event value

---

#### ProgressBar Attributes

```rust
pub struct ProgressBarAttributes {
    pub min: f32,                       // Optional: Default 0.0
    pub max: f32,                       // Optional: Default 1.0
    pub value: BindingExpr,             // Required: Current progress value
    pub style: Option<ProgressBarStyle>, // Optional: Visual style
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ProgressBarStyle {
    Primary,    // Default blue
    Success,    // Green
    Warning,    // Yellow/Orange
    Danger,     // Red
    Secondary,  // Gray
}
```

**Value Clamping**: Value is automatically clamped to [min, max] range during rendering

---

#### Tooltip Attributes

```rust
pub struct TooltipAttributes {
    pub message: String,                // Required: Tooltip text
    pub position: Option<TooltipPosition>, // Optional: Default FollowCursor
    pub delay: Option<u64>,             // Optional: Default 2000ms
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum TooltipPosition {
    FollowCursor,   // Default: follows mouse
    Top,            // Above the widget
    Bottom,         // Below the widget
    Left,           // Left of the widget
    Right,          // Right of the widget
}
```

**Parsing Logic**:
```xml
<tooltip message="Delete all completed tasks" position="top" delay="1000">
    <button label="Clear" on_click="clear_completed" />
</tooltip>
```

**Child Handling**: Tooltip wraps exactly one child widget (validated by parser)

---

#### Grid Attributes

```rust
pub struct GridAttributes {
    pub columns: u32,                   // Required: Number of columns
    pub spacing: Option<f32>,           // Optional: Gap between items, default 0
    pub padding: Option<f32>,           // Optional: Outer padding, default 0
}
```

**Layout Behavior**:
- Children flow left-to-right, wrapping to new rows
- Last row may have fewer items than `columns`
- All cells in a row have equal width

---

#### Float Attributes

```rust
pub struct FloatAttributes {
    pub position: Option<FloatPosition>, // Optional: Default custom
    pub offset_x: Option<f32>,          // Optional: Default 0.0
    pub offset_y: Option<f32>,          // Optional: Default 0.0
    pub z_index: Option<u32>,           // Optional: Default 0
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum FloatPosition {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    Custom { x: f32, y: f32 },  // Absolute positioning
}
```

**Note**: API subject to change based on Float widget verification (may use Pin instead)

---

## Runtime State Management

### Widget State Container

**Location**: `gravity-runtime/src/state.rs`

```rust
use std::any::Any;
use std::collections::HashMap;

/// Extended runtime state with widget-specific state management
pub struct GravityRuntimeState {
    /// User-defined application model
    pub user_model: Box<dyn UiBindable>,
    
    /// Widget-specific state (e.g., ComboBox::State)
    /// Key: widget ID from XML, Value: widget state
    pub widget_states: HashMap<String, Box<dyn Any>>,
}

impl GravityRuntimeState {
    /// Get or create widget state for a specific widget
    pub fn get_or_create_state<T: Default + 'static>(&mut self, widget_id: &str) -> &mut T {
        self.widget_states
            .entry(widget_id.to_string())
            .or_insert_with(|| Box::new(T::default()))
            .downcast_mut::<T>()
            .expect("Widget state type mismatch")
    }
}
```

**Rationale**: 
- ComboBox requires `combo_box::State<T>` to manage search and options
- Canvas may require custom state via `Program::State`
- State is keyed by widget `id` attribute (auto-generated if not provided)

**Widget ID Generation**:
- If `id` attribute present: use as-is
- If no `id`: generate from widget type + parent path (e.g., "combobox_category_0")

---

## Todo App Data Model

### Application State

**Location**: `examples/todo-app/src/main.rs`

```rust
use gravity_macros::UiModel;
use serde::{Deserialize, Serialize};

#[derive(UiModel, Debug, Clone, Serialize, Deserialize)]
pub struct TodoAppModel {
    // Todo items
    pub items: Vec<TodoItem>,
    
    // Current filter
    pub current_filter: TodoFilter,
    
    // UI state
    pub new_item_text: String,
    pub selected_category: String,
    pub selected_priority: Priority,
    pub show_add_dialog: bool,
    pub dark_mode: bool,
    
    // Computed properties
    pub completed_count: usize,
    pub pending_count: usize,
    pub completion_percentage: f32,
    
    // Canvas data (for statistics chart)
    #[ui_skip]  // Canvas program doesn't need binding exposure
    pub statistics_chart: StatisticsChart,
}

impl Default for TodoAppModel {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            current_filter: TodoFilter::All,
            new_item_text: String::new(),
            selected_category: "Personal".to_string(),
            selected_priority: Priority::Medium,
            show_add_dialog: false,
            dark_mode: false,
            completed_count: 0,
            pending_count: 0,
            completion_percentage: 0.0,
            statistics_chart: StatisticsChart::default(),
        }
    }
}
```

---

### TodoItem Entity

```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TodoItem {
    pub id: String,              // Unique identifier (UUID or timestamp)
    pub text: String,            // Task description
    pub category: String,        // Work, Personal, Shopping, etc.
    pub priority: Priority,      // Low, Medium, High
    pub completed: bool,         // Completion status
    pub created_at: String,      // ISO 8601 timestamp
    pub due_date: Option<String>, // Optional due date
}

impl TodoItem {
    pub fn new(text: String, category: String, priority: Priority) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(), // or use timestamp
            text,
            category,
            priority,
            completed: false,
            created_at: chrono::Utc::now().to_rfc3339(),
            due_date: None,
        }
    }
}
```

**Validation Rules**:
- `id` must be unique across all items
- `text` must not be empty
- `category` should be one of predefined categories (or "Other")
- `created_at` must be valid ISO 8601 timestamp

**Relationships**:
- TodoItem belongs to TodoAppModel via `items` Vec
- TodoItem has one Priority (enum)
- TodoItem has one Category (String)

---

### Priority Enum

```rust
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
    
    pub fn icon_path(&self) -> &str {
        match self {
            Priority::Low => "assets/priority-low.png",
            Priority::Medium => "assets/priority-medium.png",
            Priority::High => "assets/priority-high.png",
        }
    }
}

impl std::fmt::Display for Priority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
```

---

### TodoFilter Enum

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TodoFilter {
    All,         // Show all items
    Active,      // Show only incomplete items
    Completed,   // Show only completed items
    Category(String), // Filter by specific category (future enhancement)
}

impl TodoFilter {
    pub fn as_str(&self) -> &str {
        match self {
            TodoFilter::All => "All",
            TodoFilter::Active => "Active",
            TodoFilter::Completed => "Completed",
            TodoFilter::Category(cat) => cat.as_str(),
        }
    }
    
    pub fn matches(&self, item: &TodoItem) -> bool {
        match self {
            TodoFilter::All => true,
            TodoFilter::Active => !item.completed,
            TodoFilter::Completed => item.completed,
            TodoFilter::Category(cat) => &item.category == cat,
        }
    }
}

impl std::fmt::Display for TodoFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
```

---

### StatisticsChart (Canvas Program)

```rust
use iced::widget::canvas;

#[derive(Debug, Clone, Default)]
pub struct StatisticsChart {
    // Data for visualization
    pub completion_history: Vec<(String, f32)>, // (date, percentage)
}

impl canvas::Program<Message> for StatisticsChart {
    type State = ();
    
    fn draw(
        &self,
        _state: &(),
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let mut frame = canvas::Frame::new(renderer, bounds.size());
        
        // Draw axes
        let x_axis = canvas::Path::line(
            Point::new(30.0, bounds.height - 30.0),
            Point::new(bounds.width - 10.0, bounds.height - 30.0),
        );
        frame.stroke(&x_axis, canvas::Stroke::default().with_width(2.0));
        
        let y_axis = canvas::Path::line(
            Point::new(30.0, 10.0),
            Point::new(30.0, bounds.height - 30.0),
        );
        frame.stroke(&y_axis, canvas::Stroke::default().with_width(2.0));
        
        // Draw data points (simplified)
        if !self.completion_history.is_empty() {
            let data_width = bounds.width - 40.0;
            let data_height = bounds.height - 40.0;
            let point_spacing = data_width / (self.completion_history.len() as f32);
            
            for (i, (_date, percentage)) in self.completion_history.iter().enumerate() {
                let x = 30.0 + (i as f32 * point_spacing);
                let y = (bounds.height - 30.0) - (percentage * data_height);
                
                let circle = canvas::Path::circle(Point::new(x, y), 4.0);
                frame.fill(&circle, Color::from_rgb(0.2, 0.6, 1.0));
            }
        }
        
        vec![frame.into_geometry()]
    }
    
    fn update(
        &self,
        _state: &mut (),
        event: canvas::Event,
        _bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> (canvas::event::Status, Option<Message>) {
        // Handle canvas interactions if needed
        match event {
            canvas::Event::Mouse(mouse_event) => {
                // Could handle clicks, hovers, etc.
                (canvas::event::Status::Ignored, None)
            }
            _ => (canvas::event::Status::Ignored, None),
        }
    }
}
```

**Note**: This is a simplified example. Real implementation would include:
- Line charts connecting points
- Labels for axes
- Hover tooltips showing exact values
- Responsive scaling

---

## State Transitions

### TodoItem Lifecycle

```
[New] --add_item--> [Active] --toggle--> [Completed] --toggle--> [Active]
                       |                      |
                       +-------delete--------+
                                |
                              [Deleted]
```

**State Transition Rules**:
1. **Add Item**: 
   - Precondition: `new_item_text` is not empty
   - Action: Create new TodoItem with `completed = false`
   - Postcondition: Item added to `items`, `new_item_text` cleared, `pending_count` incremented

2. **Toggle Complete**:
   - Precondition: Item exists in `items`
   - Action: Flip `item.completed` boolean
   - Postcondition: Update `completed_count` and `pending_count`

3. **Delete Item**:
   - Precondition: Item exists in `items`
   - Action: Remove item from `items` Vec
   - Postcondition: Update counts, filtered list refreshes

4. **Edit Item**:
   - Precondition: Item exists, edit dialog is shown
   - Action: Update item fields (text, category, priority, due_date)
   - Postcondition: Item reflects new values, dialog closes

---

## Computed Properties

### Derived Values

```rust
impl TodoAppModel {
    /// Recompute counts when items change
    pub fn update_counts(&mut self) {
        self.completed_count = self.items.iter().filter(|i| i.completed).count();
        self.pending_count = self.items.len() - self.completed_count;
        
        self.completion_percentage = if self.items.is_empty() {
            0.0
        } else {
            (self.completed_count as f32 / self.items.len() as f32) * 100.0
        };
    }
    
    /// Get filtered items based on current filter
    pub fn filtered_items(&self) -> Vec<&TodoItem> {
        self.items
            .iter()
            .filter(|item| self.current_filter.matches(item))
            .collect()
    }
    
    /// Get items by category for category selector
    pub fn categories(&self) -> Vec<String> {
        let mut cats: Vec<String> = self.items
            .iter()
            .map(|item| item.category.clone())
            .collect();
        cats.sort();
        cats.dedup();
        
        // Always include standard categories
        for standard in &["Work", "Personal", "Shopping", "Other"] {
            if !cats.contains(&standard.to_string()) {
                cats.push(standard.to_string());
            }
        }
        cats.sort();
        cats
    }
}
```

**Update Strategy**:
- Call `update_counts()` after any item mutation
- Recompute `filtered_items()` on every render (cheap due to iterator)
- Cache `categories()` result if performance becomes issue

---

## Persistence

### State Serialization

**File Format**: JSON via serde_json

**Location**: `.gravity-state.json` in project root

**Schema**:
```json
{
  "items": [
    {
      "id": "550e8400-e29b-41d4-a716-446655440000",
      "text": "Buy groceries",
      "category": "Shopping",
      "priority": "Medium",
      "completed": false,
      "created_at": "2026-01-04T10:30:00Z",
      "due_date": null
    }
  ],
  "current_filter": "All",
  "new_item_text": "",
  "selected_category": "Personal",
  "selected_priority": "Medium",
  "show_add_dialog": false,
  "dark_mode": false,
  "completed_count": 0,
  "pending_count": 1,
  "completion_percentage": 0.0
}
```

**Persistence Triggers**:
- Auto-save after each model mutation
- Load on app startup
- Clear on explicit user action

---

## Validation & Constraints

### Model-Level Constraints

1. **TodoItem.text**: 
   - Must not be empty string
   - Max length: 500 characters
   - Trim whitespace before validation

2. **TodoItem.category**:
   - Max length: 50 characters
   - Allowed characters: alphanumeric + spaces + hyphens

3. **TodoItem.id**:
   - Must be unique across all items
   - Generated via UUID v4 or timestamp

4. **TodoAppModel.items**:
   - Max items: 1000 (performance limit)
   - No duplicate IDs

5. **TodoAppModel.new_item_text**:
   - Max length: 500 characters
   - Validation occurs before add_item handler

### Widget-Level Constraints

1. **ComboBox.options**:
   - Min 1 option, max 100 options
   - Each option max 100 characters

2. **Grid.columns**:
   - Min 1, max 20 columns
   - Must be positive integer

3. **ProgressBar.value**:
   - Auto-clamped to [min, max] range
   - No explicit validation needed

4. **Canvas.width/height**:
   - Min 50px, max 4000px
   - Must be positive floats

---

## Summary

### New IR Types

- 6 new `WidgetKind` variants
- 8 new attribute structures
- 2 new enum types (ProgressBarStyle, TooltipPosition, FloatPosition)
- Widget state management container

### Todo App Entities

- `TodoAppModel` (main application state)
- `TodoItem` (task entity)
- `Priority` enum
- `TodoFilter` enum
- `StatisticsChart` (Canvas program)

### Relationships

```
TodoAppModel
  ├── items: Vec<TodoItem>
  │     └── priority: Priority
  ├── current_filter: TodoFilter
  ├── selected_priority: Priority
  └── statistics_chart: StatisticsChart
```

### Computed Properties

- `completed_count`, `pending_count`, `completion_percentage`
- `filtered_items()`, `categories()`

### Persistence

- JSON serialization via serde
- Auto-save on mutations
- Load on startup

All data models support the 35 functional requirements and 12 success criteria defined in the feature specification.
