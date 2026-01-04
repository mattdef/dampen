# XML Schema: Advanced Widgets

**Feature**: 004-advanced-widgets-todo  
**Date**: 2026-01-04  
**Version**: 1.0

## Overview

This document defines the XML schema contract for all new widgets added in this feature. Each widget specification includes element name, attributes, children constraints, and usage examples.

---

## ComboBox Widget

### Element

```xml
<combobox />
```

### Attributes

| Attribute | Type | Required | Default | Description |
|-----------|------|----------|---------|-------------|
| `options` | String | ✅ Yes | - | Comma-separated list of selectable options |
| `selected` | Binding | No | `None` | Currently selected value (binding to model field) |
| `placeholder` | String | No | `""` | Text shown when no selection made |
| `on_select` | Handler | No | - | Event handler called when option selected |
| `id` | String | No | Auto-generated | Widget identifier for state management |

### Children

**Not allowed**. ComboBox is a leaf widget.

### Examples

**Basic usage:**
```xml
<combobox 
    options="Apple,Orange,Banana"
    placeholder="Select a fruit..."
/>
```

**With binding and handler:**
```xml
<combobox 
    options="Work,Personal,Shopping,Other"
    selected="{current_category}"
    placeholder="Select category..."
    on_select="update_category"
/>
```

**Dynamic options from model (future enhancement):**
```xml
<!-- Not yet supported - options must be static string -->
<combobox 
    options="{available_categories}"  
    selected="{current_category}"
/>
```

### Validation Rules

1. `options` attribute must not be empty
2. `options` must contain at least one item
3. Each option max length: 100 characters
4. Max total options: 100
5. `on_select` handler must accept `(model: &mut Model, value: String)`
6. `selected` binding must evaluate to `String` or `Option<String>`

### Events

- **on_select**: Fired when user selects an option
  - Handler signature: `fn(model: &mut Model, value: String)`
  - Value: The selected option text

---

## PickList Widget

### Element

```xml
<pick_list />
```

### Attributes

| Attribute | Type | Required | Default | Description |
|-----------|------|----------|---------|-------------|
| `options` | String | ✅ Yes | - | Comma-separated list of selectable options |
| `selected` | Binding | No | `None` | Currently selected value (binding to model field) |
| `placeholder` | String | No | `""` | Text shown when no selection made |
| `on_select` | Handler | No | - | Event handler called when option selected |

### Children

**Not allowed**. PickList is a leaf widget.

### Examples

**Filter selector:**
```xml
<pick_list 
    options="All,Active,Completed"
    selected="{current_filter}"
    placeholder="Filter by status..."
    on_select="apply_filter"
/>
```

**Priority selector:**
```xml
<pick_list 
    options="Low,Medium,High"
    selected="{selected_priority}"
    on_select="update_priority"
/>
```

### Validation Rules

1. Same as ComboBox validation rules
2. Difference: PickList doesn't support search/typing (simpler UX)

### Events

- **on_select**: Fired when user selects an option
  - Handler signature: `fn(model: &mut Model, value: String)`
  - Value: The selected option text

---

## Canvas Widget

### Element

```xml
<canvas />
```

### Attributes

| Attribute | Type | Required | Default | Description |
|-----------|------|----------|---------|-------------|
| `width` | Number | ✅ Yes | - | Canvas width in pixels |
| `height` | Number | ✅ Yes | - | Canvas height in pixels |
| `program` | Binding | ✅ Yes | - | Binding to `canvas::Program` implementation |
| `on_click` | Handler | No | - | Handler for canvas click events |

### Children

**Not allowed**. Canvas is a leaf widget.

### Examples

**Statistics chart:**
```xml
<canvas 
    width="400" 
    height="200" 
    program="{statistics_chart}"
/>
```

**Interactive drawing:**
```xml
<canvas 
    width="600" 
    height="400" 
    program="{drawing_canvas}"
    on_click="canvas_clicked"
/>
```

### Validation Rules

1. `width` must be positive number, min 50, max 4000
2. `height` must be positive number, min 50, max 4000
3. `program` binding must resolve to type implementing `canvas::Program<Message>`
4. Canvas requires Rust code - cannot be fully declarative

### Events

- **on_click**: Fired when user clicks the canvas
  - Handler signature: `fn(model: &mut Model, coords: (f32, f32))`
  - Value: Click coordinates relative to canvas origin

### Rust Integration

**Required Rust code:**
```rust
use iced::widget::canvas;

#[derive(Debug, Clone)]
struct MyChart {
    data: Vec<f32>,
}

impl canvas::Program<Message> for MyChart {
    type State = ();
    
    fn draw(&self, state: &(), renderer: &Renderer, theme: &Theme,
            bounds: Rectangle, cursor: mouse::Cursor) -> Vec<canvas::Geometry> {
        let mut frame = canvas::Frame::new(renderer, bounds.size());
        
        // Custom drawing logic here
        // frame.fill(&path, color);
        // frame.stroke(&path, stroke);
        
        vec![frame.into_geometry()]
    }
}

// In model:
#[derive(UiModel)]
struct Model {
    #[ui_skip]  // Don't expose Program to bindings
    pub statistics_chart: MyChart,
}
```

---

## ProgressBar Widget

### Element

```xml
<progress_bar />
```

### Attributes

| Attribute | Type | Required | Default | Description |
|-----------|------|----------|---------|-------------|
| `min` | Number | No | `0.0` | Minimum value of progress range |
| `max` | Number | No | `1.0` | Maximum value of progress range |
| `value` | Binding | ✅ Yes | - | Current progress value |
| `style` | Enum | No | `"primary"` | Visual style (primary/success/warning/danger/secondary) |

### Children

**Not allowed**. ProgressBar is a leaf widget.

### Examples

**Percentage-based (0-100):**
```xml
<progress_bar 
    min="0" 
    max="100" 
    value="{completion_percentage}"
    style="success"
/>
```

**Fraction-based (0.0-1.0):**
```xml
<progress_bar 
    value="{progress_fraction}"
/>
```

**With computed value:**
```xml
<progress_bar 
    min="0"
    max="{total_tasks}"
    value="{completed_tasks}"
/>
```

### Validation Rules

1. `min` must be less than `max`
2. `value` is automatically clamped to [min, max] range (no error on overflow)
3. `value` binding must evaluate to numeric type (`f32`, `i32`, `u32`, etc.)
4. `style` must be one of: `primary`, `success`, `warning`, `danger`, `secondary`

### Styling

**Available styles:**
- `primary` - Default blue theme color
- `success` - Green (for completed items)
- `warning` - Yellow/Orange (for items nearing deadline)
- `danger` - Red (for overdue or critical items)
- `secondary` - Gray/muted color

---

## Tooltip Widget

### Element

```xml
<tooltip>...</tooltip>
```

### Attributes

| Attribute | Type | Required | Default | Description |
|-----------|------|----------|---------|-------------|
| `message` | String | ✅ Yes | - | Tooltip text to display on hover |
| `position` | Enum | No | `"follow_cursor"` | Where tooltip appears relative to widget |
| `delay` | Number | No | `2000` | Delay in milliseconds before showing tooltip |

### Children

**Required**. Tooltip must wrap exactly **one** child widget.

### Examples

**Button with help text:**
```xml
<tooltip message="Delete all completed tasks" position="top">
    <button label="Clear Completed" on_click="clear_completed" />
</tooltip>
```

**Icon with explanation:**
```xml
<tooltip message="High priority task" delay="500">
    <image path="assets/priority-high.png" width="24" height="24" />
</tooltip>
```

**Following cursor (default):**
```xml
<tooltip message="Click to edit task details">
    <text value="{task.name}" />
</tooltip>
```

### Validation Rules

1. `message` must not be empty
2. `message` max length: 200 characters
3. Must have exactly 1 child element (parser error if 0 or >1)
4. `position` must be one of: `follow_cursor`, `top`, `bottom`, `left`, `right`
5. `delay` must be non-negative integer (in milliseconds)

### Position Values

- `follow_cursor` - Tooltip follows mouse cursor (default)
- `top` - Above the widget
- `bottom` - Below the widget
- `left` - Left of the widget
- `right` - Right of the widget

---

## Grid Widget

### Element

```xml
<grid>...</grid>
```

### Attributes

| Attribute | Type | Required | Default | Description |
|-----------|------|----------|---------|-------------|
| `columns` | Number | ✅ Yes | - | Number of columns in the grid |
| `spacing` | Number | No | `0` | Gap between grid cells (pixels) |
| `padding` | Number | No | `0` | Outer padding around grid (pixels) |

### Children

**Required**. Grid can have **zero or more** children. Children flow left-to-right, wrapping to new rows.

### Examples

**Task table (5 columns):**
```xml
<grid columns="5" spacing="10" padding="20">
    <!-- Row 1 -->
    <text value="Task Name" weight="bold" />
    <text value="Category" weight="bold" />
    <text value="Priority" weight="bold" />
    <text value="Due Date" weight="bold" />
    <text value="Actions" weight="bold" />
    
    <!-- Row 2 (first task) -->
    <text value="{task1.name}" />
    <text value="{task1.category}" />
    <text value="{task1.priority}" />
    <text value="{task1.due_date}" />
    <button label="Edit" on_click="edit_task1" />
    
    <!-- More rows... -->
</grid>
```

**Icon grid (3 columns):**
```xml
<grid columns="3" spacing="15">
    <image path="icon1.png" width="48" height="48" />
    <image path="icon2.png" width="48" height="48" />
    <image path="icon3.png" width="48" height="48" />
    <image path="icon4.png" width="48" height="48" />
</grid>
```

### Validation Rules

1. `columns` must be positive integer, min 1, max 20
2. `spacing` must be non-negative number
3. `padding` must be non-negative number
4. Last row may have fewer items than `columns` (automatically left-aligned)

### Layout Behavior

- Children distributed left-to-right, top-to-bottom
- Each row has `columns` items (except possibly last row)
- All cells in a row have equal width
- Cell height determined by tallest item in row
- Grid adapts to container width

---

## Float Widget

### Element

```xml
<float>...</float>
```

**⚠️ Note**: Float widget API needs verification. May use `pin` widget instead based on Iced capabilities.

### Attributes (Tentative)

| Attribute | Type | Required | Default | Description |
|-----------|------|----------|---------|-------------|
| `position` | Enum | No | Custom | Positioning strategy |
| `offset_x` | Number | No | `0` | Horizontal offset from position (pixels) |
| `offset_y` | Number | No | `0` | Vertical offset from position (pixels) |
| `z_index` | Number | No | `0` | Stacking order (higher = on top) |
| `visible` | Binding | No | `true` | Whether float is visible |

### Children

**Required**. Float must wrap exactly **one** child widget.

### Examples (Tentative)

**Floating action button:**
```xml
<float position="bottom_right" offset_x="-20" offset_y="-20" z_index="100">
    <button label="+" on_click="show_add_dialog" />
</float>
```

**Modal overlay:**
```xml
<float position="center" z_index="200" visible="{show_modal}">
    <container padding="40" background="#ffffff">
        <column spacing="20">
            <text value="Edit Task" size="24" weight="bold" />
            <text_input value="{edit_text}" on_input="update_edit_text" />
            <row spacing="10">
                <button label="Save" on_click="save_task" />
                <button label="Cancel" on_click="cancel_edit" />
            </row>
        </column>
    </container>
</float>
```

### Validation Rules (Tentative)

1. `position` must be one of: `top_left`, `top_right`, `bottom_left`, `bottom_right`, `center`
2. `z_index` must be non-negative integer
3. `visible` binding must evaluate to boolean
4. Must have exactly 1 child element

### Position Values (Tentative)

- `top_left` - Top-left corner of container
- `top_right` - Top-right corner
- `bottom_left` - Bottom-left corner
- `bottom_right` - Bottom-right corner
- `center` - Center of container

**Action Required**: Verify Float widget API and update this section with confirmed details.

---

## Widget Comparison

### ComboBox vs PickList

| Feature | ComboBox | PickList |
|---------|----------|----------|
| Search/typing | ✅ Yes | ❌ No |
| State management | Requires `State<T>` | No state needed |
| Use case | Long option lists | Short option lists |
| Complexity | Higher | Lower |

**When to use ComboBox**: 
- Option list > 10 items
- Users need to search/filter
- Need autocomplete behavior

**When to use PickList**:
- Option list < 10 items
- All options fit in dropdown
- Simpler, more performant

---

## Complete Todo App Example

**File**: `examples/todo-app/ui/main.gravity`

```xml
<?xml version="1.0" encoding="UTF-8" ?>
<container padding="20">
    <scrollable>
        <column spacing="15">
            <!-- Header -->
            <row spacing="10" align="center">
                <text value="Modern Todo App" size="28" weight="bold" />
                <space />
                <toggler 
                    label="Dark Mode" 
                    active="{dark_mode}" 
                    on_toggle="toggle_dark_mode" 
                />
            </row>
            
            <rule />
            
            <!-- Statistics Section -->
            <column spacing="10">
                <text value="Overview" size="20" weight="bold" />
                
                <row spacing="20">
                    <!-- Progress Bar -->
                    <column spacing="5" width="fill">
                        <text value="Completion Progress" size="14" />
                        <progress_bar 
                            min="0" 
                            max="100" 
                            value="{completion_percentage}"
                            style="success"
                        />
                        <text value="{completion_percentage}% complete" size="12" />
                    </column>
                    
                    <!-- Statistics Canvas -->
                    <column spacing="5">
                        <text value="7-Day Trend" size="14" />
                        <canvas 
                            width="300" 
                            height="150" 
                            program="{statistics_chart}"
                        />
                    </column>
                </row>
                
                <!-- Counts -->
                <row spacing="15">
                    <text value="Total: {items.len()}" />
                    <text value="Active: {pending_count}" />
                    <text value="Completed: {completed_count}" />
                </row>
            </column>
            
            <rule />
            
            <!-- Add Task Section -->
            <column spacing="10">
                <text value="Add New Task" size="20" weight="bold" />
                
                <row spacing="10">
                    <text_input 
                        value="{new_item_text}"
                        on_input="update_new_item"
                        placeholder="What needs to be done?"
                        width="fill"
                    />
                    
                    <tooltip message="Add this task to your list" position="top">
                        <button 
                            label="Add"
                            on_click="add_item"
                            enabled="{new_item_text.len() > 0}"
                        />
                    </tooltip>
                </row>
                
                <row spacing="10">
                    <!-- Category Selector (ComboBox for searchability) -->
                    <column spacing="5" width="fill">
                        <text value="Category" size="12" />
                        <combobox 
                            options="Work,Personal,Shopping,Health,Finance,Other"
                            selected="{selected_category}"
                            placeholder="Select category..."
                            on_select="update_category"
                        />
                    </column>
                    
                    <!-- Priority Selector (PickList for simplicity) -->
                    <column spacing="5" width="fill">
                        <text value="Priority" size="12" />
                        <pick_list 
                            options="Low,Medium,High"
                            selected="{selected_priority}"
                            on_select="update_priority"
                        />
                    </column>
                </row>
            </column>
            
            <rule />
            
            <!-- Task List Section -->
            <column spacing="10">
                <row spacing="10" align="center">
                    <text value="Your Tasks" size="20" weight="bold" />
                    <space />
                    
                    <!-- Filter -->
                    <pick_list 
                        options="All,Active,Completed"
                        selected="{current_filter}"
                        placeholder="Filter..."
                        on_select="apply_filter"
                    />
                </row>
                
                <!-- Task Grid (5 columns: checkbox, name, category, priority icon, actions) -->
                <grid columns="5" spacing="10">
                    <!-- Header Row -->
                    <text value="" />  <!-- Checkbox column -->
                    <text value="Task" weight="bold" />
                    <text value="Category" weight="bold" />
                    <text value="Priority" weight="bold" />
                    <text value="Actions" weight="bold" />
                    
                    <!-- TODO: Dynamic rows would go here -->
                    <!-- For now, static example rows -->
                    
                    <!-- Example Task 1 -->
                    <checkbox checked="{task1_completed}" on_toggle="toggle_task1" />
                    <text value="{task1_text}" />
                    <text value="{task1_category}" />
                    <tooltip message="High priority" position="top">
                        <image path="assets/priority-high.png" width="24" height="24" />
                    </tooltip>
                    <row spacing="5">
                        <button label="Edit" on_click="edit_task1" />
                        <button label="Delete" on_click="delete_task1" />
                    </row>
                </grid>
                
                <!-- Empty State -->
                <text 
                    value="{if items.len() == 0 then 'No tasks yet! Add one above.' else ''}"
                    size="14"
                />
            </column>
            
            <rule />
            
            <!-- Actions Section -->
            <row spacing="10">
                <tooltip message="Remove all completed tasks from the list" position="top">
                    <button 
                        label="Clear Completed"
                        on_click="clear_completed"
                        enabled="{completed_count > 0}"
                    />
                </tooltip>
                
                <tooltip message="Remove all tasks from the list" position="top">
                    <button 
                        label="Clear All"
                        on_click="clear_all"
                        enabled="{items.len() > 0}"
                    />
                </tooltip>
            </row>
        </column>
    </scrollable>
    
    <!-- Floating Add Button (alternative to top form) -->
    <float position="bottom_right" offset_x="-20" offset_y="-20" z_index="100">
        <tooltip message="Quick add task" position="left">
            <button label="+" on_click="show_quick_add" />
        </tooltip>
    </float>
</container>
```

---

## Schema Validation

### Parser Validation Checklist

For each new widget, the parser must validate:

1. ✅ Element name matches `WidgetKind` variant
2. ✅ All required attributes are present
3. ✅ Attribute values match expected types
4. ✅ Enum values are valid (e.g., `position`, `style`)
5. ✅ Numeric values are within allowed ranges
6. ✅ Child element count matches constraints
7. ✅ Event handler names reference valid handlers
8. ✅ Binding expressions are syntactically valid

### Runtime Validation Checklist

At build/render time:

1. ✅ Bindings evaluate to expected types
2. ✅ Event handlers have correct signatures
3. ✅ Widget state can be created/retrieved
4. ✅ Canvas programs implement required trait
5. ✅ Image paths resolve to valid files (if used)

---

## Version History

### v1.0 (2026-01-04)

- Initial schema for 8 new widgets
- ComboBox, PickList, Canvas, ProgressBar, Tooltip, Grid, Float
- Complete todo-app example
- Validation rules defined

---

## Future Enhancements

### Planned for Later Versions

1. **Dynamic ComboBox/PickList options**:
   ```xml
   <combobox options="{available_categories}" />
   ```
   Currently only supports static comma-separated strings

2. **Grid with dynamic rows**:
   ```xml
   <grid columns="5">
       <for each="item" in="{items}">
           <text value="{item.name}" />
           <!-- ... -->
       </for>
   </grid>
   ```
   Requires iteration support in Gravity

3. **Tooltip with rich content**:
   ```xml
   <tooltip position="top">
       <button label="Info" />
       <tooltip_content>
           <column>
               <text value="Bold title" weight="bold" />
               <text value="Description here" />
           </column>
       </tooltip_content>
   </tooltip>
   ```
   Currently only supports plain text `message`

4. **Canvas declarative drawing**:
   Simple shape DSL for common visualizations without Rust code

---

## Summary

This XML schema defines 8 new widgets for Gravity:

- **ComboBox**: Searchable dropdown (requires state)
- **PickList**: Simple dropdown (stateless)
- **Canvas**: Custom graphics (requires Rust `Program` impl)
- **ProgressBar**: Progress indicator (simple)
- **Tooltip**: Hover help (wrapper widget)
- **Grid**: Multi-column layout (container widget)
- **Float**: Positioned overlay (wrapper widget, API TBD)
- **Image**: Already supported, verified

All widgets follow Gravity's declarative XML philosophy while accommodating necessary Rust integration for complex features like Canvas.
