# Widget Extension Pattern

**Feature**: 003-widget-builder  
**Date**: 2026-01-03  
**Task**: T086 - Document extension pattern for future widgets

---

## Overview

This document describes the canonical pattern for adding new widget types to Gravity. The centralized builder architecture ensures that adding a widget requires changes to **only 3 files** in the codebase, with **zero changes** to examples or user code.

---

## Step-by-Step Guide

### Step 1: Add Widget Type to IR (gravity-core)

**File**: `crates/gravity-core/src/ir/node.rs`

**Location**: `WidgetKind` enum (lines 26-45)

```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize, Default)]
pub enum WidgetKind {
    // ... existing widgets ...
    ProgressBar,  // ← Add your widget here
    Custom(String),
}
```

**Why**: This defines the widget type in the intermediate representation, making it available to the parser and builder.

**Example**: Adding `ProgressBar` widget
```rust
ProgressBar,
```

---

### Step 2: Add Parser Mapping (gravity-core)

**File**: `crates/gravity-core/src/parser/mod.rs`

**Location**: `parse_widget_kind()` function (lines 95-115)

```rust
fn parse_widget_kind(tag: &str) -> Result<WidgetKind, ParseError> {
    Ok(match tag {
        // ... existing mappings ...
        "progress_bar" => WidgetKind::ProgressBar,  // ← Add XML tag mapping
        _ => WidgetKind::Custom(tag.to_string()),
    })
}
```

**Why**: Maps the XML tag name to the IR widget type.

**Example**: `<progress_bar />` → `WidgetKind::ProgressBar`

---

### Step 3: Add Builder Implementation (gravity-iced)

**File**: `crates/gravity-iced/src/builder.rs`

**Location 1**: `build_widget()` match statement (lines 119-138)

```rust
fn build_widget(&self, node: &WidgetNode) -> Element<'a, Message, Theme, Renderer>
where
    Message: Clone + 'static,
{
    match node.kind {
        // ... existing widgets ...
        WidgetKind::ProgressBar => self.build_progress_bar(node),  // ← Add match arm
        WidgetKind::Custom(_) => self.build_custom(node),
    }
}
```

**Location 2**: Add widget-specific builder method (end of impl block)

```rust
/// Build a progress bar widget
fn build_progress_bar(&self, node: &WidgetNode) -> Element<'a, Message, Theme, Renderer>
where
    Message: Clone + 'static,
{
    use iced::widget::progress_bar;
    
    // 1. Extract attributes
    let value = node
        .attributes
        .get("value")
        .map(|v| self.evaluate_attribute(v))
        .and_then(|s| s.parse::<f32>().ok())
        .unwrap_or(0.0);
    
    let min = node
        .attributes
        .get("min")
        .map(|v| self.evaluate_attribute(v))
        .and_then(|s| s.parse::<f32>().ok())
        .unwrap_or(0.0);
    
    let max = node
        .attributes
        .get("max")
        .map(|v| self.evaluate_attribute(v))
        .and_then(|s| s.parse::<f32>().ok())
        .unwrap_or(100.0);
    
    // 2. Create widget
    let widget = progress_bar(min..=max, value);
    
    // 3. Apply style and layout
    self.apply_style_layout(widget.into(), node)
}
```

**Why**: Implements the actual rendering logic using Iced widgets.

---

## Pattern Components

### 1. Attribute Extraction

**Pattern**:
```rust
let value = node
    .attributes
    .get("attribute_name")
    .map(|v| self.evaluate_attribute(v))  // Handles bindings
    .and_then(|s| s.parse::<TargetType>().ok())
    .unwrap_or(default_value);
```

**Handles**:
- Static values: `value="50"`
- Bindings: `value="{progress}"`
- Interpolation: `value="Progress: {count}/100"`

### 2. Widget Creation

**Pattern**:
```rust
use iced::widget::widget_name;

let widget = widget_name(/* constructor args */);
```

**Examples**:
- `text(content)` - Simple text
- `button(label).on_press(message)` - Interactive widget
- `progress_bar(range, value)` - Stateful widget

### 3. Style & Layout Application

**Pattern**:
```rust
self.apply_style_layout(widget.into(), node)
```

**Handles**:
- Width/height from `node.layout`
- Padding/spacing from `node.layout`
- Background/border from `node.style`
- Class-based styling from `node.classes`
- Breakpoint-aware attributes

### 4. Event Handling

**Pattern** (for interactive widgets):
```rust
let on_event = node
    .attributes
    .get("on_event")
    .map(|v| self.evaluate_attribute(v))
    .and_then(|handler_name| self.get_handler_message(&handler_name));

let widget = if let Some(msg) = on_event {
    widget_constructor(...).on_event(msg)
} else {
    widget_constructor(...)
};
```

**Example**:
```rust
let on_click = node
    .attributes
    .get("on_click")
    .map(|v| self.evaluate_attribute(v))
    .and_then(|handler| self.get_handler_message(&handler));

let widget = if let Some(msg) = on_click {
    button(label).on_press(msg)
} else {
    button(label)
};
```

---

## Complete Example: Adding ProgressBar

### 1. IR Definition

**File**: `gravity-core/src/ir/node.rs`

```rust
pub enum WidgetKind {
    // ... existing ...
    ProgressBar,
    // ...
}
```

### 2. Parser Mapping

**File**: `gravity-core/src/parser/mod.rs`

```rust
fn parse_widget_kind(tag: &str) -> Result<WidgetKind, ParseError> {
    Ok(match tag {
        // ...
        "progress_bar" => WidgetKind::ProgressBar,
        // ...
    })
}
```

### 3. Builder Implementation

**File**: `gravity-iced/src/builder.rs`

```rust
// In build_widget()
match node.kind {
    // ...
    WidgetKind::ProgressBar => self.build_progress_bar(node),
    // ...
}

// New method
fn build_progress_bar(&self, node: &WidgetNode) -> Element<'a, Message, Theme, Renderer>
where
    Message: Clone + 'static,
{
    use iced::widget::progress_bar;
    
    let value = node
        .attributes
        .get("value")
        .map(|v| self.evaluate_attribute(v))
        .and_then(|s| s.parse::<f32>().ok())
        .unwrap_or(0.0);
    
    let min = node
        .attributes
        .get("min")
        .map(|v| self.evaluate_attribute(v))
        .and_then(|s| s.parse::<f32>().ok())
        .unwrap_or(0.0);
    
    let max = node
        .attributes
        .get("max")
        .map(|v| self.evaluate_attribute(v))
        .and_then(|s| s.parse::<f32>().ok())
        .unwrap_or(100.0);
    
    let widget = progress_bar(min..=max, value);
    
    if self.verbose {
        eprintln!(
            "[GravityWidgetBuilder] Created progress_bar: value={}, min={}, max={}",
            value, min, max
        );
    }
    
    self.apply_style_layout(widget.into(), node)
}
```

### 4. Usage in XML

```xml
<!-- Static value -->
<progress_bar value="50" min="0" max="100" />

<!-- Dynamic binding -->
<progress_bar value="{download_progress}" max="100" />

<!-- With styling -->
<progress_bar 
    value="{progress}" 
    max="100"
    width="fill"
    height="20"
/>
```

### 5. Example Application

```rust
use gravity_core::parse;
use gravity_iced::GravityWidgetBuilder;
use gravity_macros::UiModel;

#[derive(UiModel, Serialize, Deserialize, Clone)]
struct Model {
    progress: f32,
}

fn view(state: &AppState) -> Element<'_, Message> {
    GravityWidgetBuilder::new(
        &state.document.root,
        &state.model,
        None  // No handlers needed for progress bar
    ).build()
}
```

**Result**: The progress bar automatically updates when `model.progress` changes, with **zero manual rendering code**.

---

## Files That DON'T Change

✅ **Examples**: No changes needed  
✅ **User Code**: Existing apps continue to work  
✅ **Tests**: Existing tests remain valid  
✅ **Documentation**: Only add new widget docs  

---

## Verification Checklist

When adding a new widget, verify:

- [ ] Added variant to `WidgetKind` enum
- [ ] Added mapping in `parse_widget_kind()`
- [ ] Added match arm in `build_widget()`
- [ ] Implemented `build_<widget>()` method
- [ ] Handles all relevant attributes
- [ ] Applies style/layout with `apply_style_layout()`
- [ ] Includes verbose logging (if enabled)
- [ ] Provides sensible defaults for missing attributes
- [ ] Compiles without errors: `cargo check -p gravity-iced`
- [ ] No changes needed in examples

---

## Best Practices

### 1. Attribute Naming

Follow Iced naming conventions:
- `value` for primary input
- `on_click`, `on_change`, etc. for events
- `min`, `max` for ranges
- `placeholder` for hints

### 2. Error Handling

Use `.unwrap_or(default)` for graceful degradation:
```rust
.unwrap_or(0.0)          // Numbers
.unwrap_or_default()     // Strings, vecs
.unwrap_or(false)        // Booleans
```

### 3. Verbose Logging

Always add logging for debugging:
```rust
if self.verbose {
    eprintln!(
        "[GravityWidgetBuilder] Created {}: attr1={}, attr2={}",
        "widget_name", val1, val2
    );
}
```

### 4. Documentation

Document each builder method:
```rust
/// Build a progress bar widget
///
/// # Attributes
/// - `value`: Current progress value (binding supported)
/// - `min`: Minimum value (default: 0)
/// - `max`: Maximum value (default: 100)
///
/// # Styling
/// Supports standard width/height/padding attributes.
fn build_progress_bar(...) { ... }
```

---

## Alternative Backend Pattern

If creating a **non-Iced backend**, follow the same pattern:

1. Create `my-backend/src/builder.rs`
2. Implement same trait/interface as `GravityWidgetBuilder`
3. Use backend-specific widgets instead of Iced
4. Share `gravity-core` IR without changes

**Example** (hypothetical GTK backend):
```rust
// gravity-gtk/src/builder.rs
fn build_progress_bar(&self, node: &WidgetNode) -> gtk::Widget {
    let value = /* same extraction logic */;
    let widget = gtk::ProgressBar::new();
    widget.set_fraction(value / 100.0);
    widget.into()
}
```

**Result**: Same XML works across all backends.

---

## Summary

**Files to Modify**: 3 (gravity-core/node.rs, gravity-core/parser.rs, gravity-iced/builder.rs)  
**Files NOT Modified**: Examples, user code, tests  
**Lines of Code**: ~30 lines total  
**Compilation Impact**: ~2-5 seconds  
**User Impact**: Zero (automatic support in all examples)

**Conclusion**: The centralized builder architecture successfully isolates widget implementation details, making extension straightforward and predictable.
