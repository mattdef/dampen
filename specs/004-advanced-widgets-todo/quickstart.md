# Quick Start: Using Advanced Widgets in Gravity

**Feature**: 004-advanced-widgets-todo  
**Date**: 2026-01-04

## Overview

This guide shows you how to use the 8 new advanced widgets in your Gravity applications: ComboBox, PickList, Canvas, ProgressBar, Tooltip, Grid, Float, and Image.

---

## Installation

### Prerequisites

- Gravity framework installed (`cargo install gravity-cli`)
- Rust 1.75+ with Edition 2024
- Iced 0.14+ (handled by Gravity dependencies)

### Project Setup

```bash
# Create new Gravity project
gravity new my-app
cd my-app

# Verify advanced widgets are available
gravity check ui/main.gravity
```

---

## Quick Reference

### Widget Cheat Sheet

| Widget | Use Case | Key Attributes |
|--------|----------|----------------|
| ComboBox | Searchable dropdown | `options`, `selected`, `on_select` |
| PickList | Simple dropdown | `options`, `selected`, `on_select` |
| Canvas | Custom graphics | `width`, `height`, `program` |
| ProgressBar | Progress display | `min`, `max`, `value` |
| Tooltip | Help text on hover | `message`, `position` |
| Grid | Multi-column layout | `columns`, `spacing` |
| Float | Positioned overlays | `position`, `z_index` |
| Image | Display images | `path`, `width`, `height` |

---

## ComboBox: Searchable Dropdown

### Basic Usage

**XML** (`ui/main.gravity`):
```xml
<combobox 
    options="Apple,Orange,Banana,Grape,Strawberry"
    selected="{favorite_fruit}"
    placeholder="Select a fruit..."
    on_select="update_fruit"
/>
```

**Rust** (`src/main.rs`):
```rust
use gravity_macros::{UiModel, ui_handler};

#[derive(UiModel, Clone, Serialize, Deserialize)]
struct Model {
    favorite_fruit: String,
}

#[ui_handler]
fn update_fruit(model: &mut Model, value: String) {
    model.favorite_fruit = value;
    println!("Selected: {}", value);
}
```

### When to Use

- ✅ Option lists with 10+ items
- ✅ Users need to search/filter options
- ✅ Autocomplete behavior desired

---

## PickList: Simple Dropdown

### Basic Usage

**XML**:
```xml
<pick_list 
    options="All,Active,Completed"
    selected="{filter}"
    on_select="apply_filter"
/>
```

**Rust**:
```rust
#[derive(UiModel, Clone, Serialize, Deserialize)]
struct Model {
    filter: String,
}

#[ui_handler]
fn apply_filter(model: &mut Model, value: String) {
    model.filter = value;
}
```

### When to Use

- ✅ Short option lists (< 10 items)
- ✅ All options fit comfortably in dropdown
- ✅ Simpler, more performant than ComboBox

---

## Canvas: Custom Graphics

### Basic Usage

**XML**:
```xml
<canvas 
    width="400" 
    height="200" 
    program="{chart}"
/>
```

**Rust**:
```rust
use iced::widget::canvas;
use iced::{Color, Point, Rectangle, Renderer, Theme};

#[derive(Debug, Clone, Default)]
struct SimpleChart {
    values: Vec<f32>,
}

impl canvas::Program<Message> for SimpleChart {
    type State = ();
    
    fn draw(&self, _: &(), renderer: &Renderer, _: &Theme,
            bounds: Rectangle, _: mouse::Cursor) -> Vec<canvas::Geometry> {
        let mut frame = canvas::Frame::new(renderer, bounds.size());
        
        // Draw a simple bar chart
        let bar_width = bounds.width / self.values.len() as f32;
        
        for (i, &value) in self.values.iter().enumerate() {
            let x = i as f32 * bar_width;
            let height = value * bounds.height;
            let y = bounds.height - height;
            
            let rect = canvas::Path::rectangle(
                Point::new(x, y),
                iced::Size::new(bar_width - 2.0, height),
            );
            frame.fill(&rect, Color::from_rgb(0.2, 0.6, 1.0));
        }
        
        vec![frame.into_geometry()]
    }
}

#[derive(UiModel, Clone, Serialize, Deserialize)]
struct Model {
    #[ui_skip]  // Canvas Program doesn't need binding exposure
    chart: SimpleChart,
}
```

### When to Use

- ✅ Custom visualizations (charts, graphs, diagrams)
- ✅ Interactive drawing surfaces
- ✅ Game rendering
- ❌ Simple shapes (use SVG instead)

---

## ProgressBar: Progress Indicator

### Basic Usage

**XML**:
```xml
<!-- Percentage-based (0-100) -->
<progress_bar 
    min="0" 
    max="100" 
    value="{completion_percent}"
    style="success"
/>

<!-- Fraction-based (0.0-1.0, default) -->
<progress_bar value="{progress_fraction}" />
```

**Rust**:
```rust
#[derive(UiModel, Clone, Serialize, Deserialize)]
struct Model {
    completion_percent: f32,  // e.g., 75.5
    progress_fraction: f32,   // e.g., 0.755
}
```

### Styles

- `primary` - Blue (default)
- `success` - Green
- `warning` - Yellow/Orange
- `danger` - Red
- `secondary` - Gray

---

## Tooltip: Contextual Help

### Basic Usage

**XML**:
```xml
<tooltip message="Click to save your changes" position="top">
    <button label="Save" on_click="save" />
</tooltip>
```

### Positioning

- `follow_cursor` - Follows mouse (default)
- `top` - Above widget
- `bottom` - Below widget
- `left` - Left of widget
- `right` - Right of widget

### Custom Delay

```xml
<tooltip message="Quick tip!" delay="500">
    <text value="Hover me" />
</tooltip>
```

---

## Grid: Multi-Column Layout

### Basic Usage

**XML**:
```xml
<grid columns="3" spacing="10" padding="20">
    <image path="icon1.png" width="64" height="64" />
    <image path="icon2.png" width="64" height="64" />
    <image path="icon3.png" width="64" height="64" />
    <image path="icon4.png" width="64" height="64" />
    <image path="icon5.png" width="64" height="64" />
</grid>
```

### Table Layout

```xml
<grid columns="4" spacing="5">
    <!-- Header row -->
    <text value="Name" weight="bold" />
    <text value="Age" weight="bold" />
    <text value="City" weight="bold" />
    <text value="Actions" weight="bold" />
    
    <!-- Data rows -->
    <text value="Alice" />
    <text value="30" />
    <text value="NYC" />
    <button label="Edit" on_click="edit_alice" />
    
    <text value="Bob" />
    <text value="25" />
    <text value="LA" />
    <button label="Edit" on_click="edit_bob" />
</grid>
```

---

## Float: Positioned Overlays

### Floating Action Button

**XML**:
```xml
<float position="bottom_right" offset_x="-20" offset_y="-20" z_index="100">
    <button label="+" on_click="add_item" />
</float>
```

### Modal Dialog

**XML**:
```xml
<float position="center" z_index="200" visible="{show_modal}">
    <container padding="40" background="#ffffff">
        <column spacing="20">
            <text value="Confirm Delete" size="20" weight="bold" />
            <text value="Are you sure?" />
            <row spacing="10">
                <button label="Yes" on_click="confirm_delete" />
                <button label="No" on_click="cancel_delete" />
            </row>
        </column>
    </container>
</float>
```

**Rust**:
```rust
#[derive(UiModel, Clone, Serialize, Deserialize)]
struct Model {
    show_modal: bool,
}
```

---

## Image: Display Images

### Basic Usage

**XML**:
```xml
<image path="assets/logo.png" width="100" height="100" />
```

### With Tooltip

```xml
<tooltip message="Company Logo">
    <image path="assets/logo.png" width="64" height="64" />
</tooltip>
```

---

## Complete Example: Todo App

See `examples/todo-app/` for a fully functional modern todo application demonstrating all 8 widgets:

```bash
cd examples/todo-app
cargo run
```

**Features:**
- ComboBox for category selection
- PickList for filtering tasks
- Canvas for completion statistics chart
- ProgressBar showing overall completion
- Tooltips on all action buttons
- Grid layout for task table
- Float for "Add Task" button
- Images for priority indicators

---

## Development Workflow

### 1. Create UI File

```bash
mkdir ui
touch ui/main.gravity
```

### 2. Add Widgets

Edit `ui/main.gravity` with your favorite text editor:

```xml
<?xml version="1.0" encoding="UTF-8" ?>
<container padding="20">
    <column spacing="15">
        <text value="My App" size="24" weight="bold" />
        
        <pick_list 
            options="Option 1,Option 2,Option 3"
            selected="{choice}"
            on_select="update_choice"
        />
        
        <progress_bar 
            min="0" 
            max="100" 
            value="{progress}"
        />
    </column>
</container>
```

### 3. Create Model

In `src/main.rs`:

```rust
use gravity_macros::UiModel;
use serde::{Deserialize, Serialize};

#[derive(UiModel, Clone, Serialize, Deserialize, Default)]
struct Model {
    choice: String,
    progress: f32,
}
```

### 4. Run with Hot Reload

```bash
gravity dev --ui ui --file main.gravity --verbose
```

### 5. Make Changes

Edit `ui/main.gravity` and save - UI updates automatically!

---

## Testing

### Validate XML

```bash
gravity check ui/main.gravity
```

### Run Tests

```bash
cargo test
```

---

## Best Practices

### ComboBox vs PickList

**Use ComboBox when:**
- More than 10 options
- Users might not know exact option name
- Search/autocomplete is valuable

**Use PickList when:**
- Fewer than 10 options
- Options are well-known
- Simpler UI is preferred

### Canvas Performance

**Do:**
- ✅ Use `Cache` for static geometry
- ✅ Keep drawing logic efficient
- ✅ Limit redraw frequency

**Don't:**
- ❌ Redraw on every frame if data unchanged
- ❌ Create new geometries unnecessarily
- ❌ Use for simple shapes (use SVG instead)

### Tooltip UX

**Do:**
- ✅ Keep messages under 200 characters
- ✅ Use for clarification, not critical info
- ✅ Test on both mouse and touch devices

**Don't:**
- ❌ Put essential information only in tooltips
- ❌ Use tooltips for obvious actions
- ❌ Stack tooltips (causes confusion)

### Grid Layout

**Do:**
- ✅ Use consistent column counts
- ✅ Align headers with data
- ✅ Add spacing for readability

**Don't:**
- ❌ Exceed 10-12 columns (gets cramped)
- ❌ Mix different column counts in same view
- ❌ Forget to handle empty grids

---

## Troubleshooting

### ComboBox Not Showing Options

**Problem**: Dropdown appears empty

**Solution**: Check that `options` attribute is not empty:
```xml
<!-- Wrong -->
<combobox options="" />

<!-- Right -->
<combobox options="One,Two,Three" />
```

### Canvas Not Rendering

**Problem**: Canvas shows blank

**Solution**: Ensure `program` binding points to valid `canvas::Program` impl:
```rust
// In Model
#[ui_skip]
pub chart: MyChart,  // Must implement canvas::Program<Message>
```

### ProgressBar Shows Wrong Value

**Problem**: Progress bar doesn't match expected value

**Solution**: Check value is within [min, max] range:
```rust
// Wrong
model.progress = 150.0;  // Exceeds max of 100

// Right - value is auto-clamped
model.progress = 100.0;
```

### Tooltip Not Appearing

**Problem**: Hover doesn't show tooltip

**Solution**: 
1. Check default delay (2000ms = 2 seconds)
2. Reduce delay if needed:
```xml
<tooltip message="Quick tip" delay="500">
    <button label="Click" />
</tooltip>
```

### Grid Items Not Aligning

**Problem**: Grid layout looks uneven

**Solution**: Ensure all rows have same structure:
```xml
<!-- Wrong - inconsistent columns -->
<grid columns="3">
    <text value="A" />
    <text value="B" />
    <!-- Missing 3rd item -->
    
    <text value="C" />
</grid>

<!-- Right - fill all columns or leave last row incomplete -->
<grid columns="3">
    <text value="A" />
    <text value="B" />
    <text value="C" />
    
    <text value="D" />
    <text value="E" />
    <!-- Last row can have fewer items -->
</grid>
```

---

## Performance Tips

### Widget State

ComboBox requires widget state management. Use unique `id` attributes:

```xml
<combobox 
    id="category_selector"
    options="Work,Personal"
    selected="{category}"
/>
```

Without `id`, Gravity auto-generates based on position (may cause issues if UI structure changes).

### Canvas Caching

For static visualizations, use Canvas `Cache`:

```rust
use iced::widget::canvas::Cache;

struct MyChart {
    cache: Cache,
}

impl canvas::Program<Message> for MyChart {
    fn draw(&self, ...) -> Vec<canvas::Geometry> {
        self.cache.draw(renderer, bounds.size(), |frame| {
            // Drawing code here - only runs when cache invalidated
        })
    }
}
```

---

## Next Steps

1. **Explore Examples**: Check `examples/todo-app/` for complete implementation
2. **Read XML Schema**: See `contracts/xml-schema.md` for detailed attribute reference
3. **Build Your App**: Start with simple widgets and gradually add complexity
4. **Join Community**: Share your creations and get help

---

## Additional Resources

- [Gravity Documentation](https://docs.gravity-ui.dev)
- [Iced Widget Gallery](https://docs.rs/iced/latest/iced/widget/)
- [Todo App Source](../../../examples/todo-app/)
- [XML Schema Reference](./contracts/xml-schema.md)

Happy building! 🚀
