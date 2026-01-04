# Data Model: Implement Real Iced Widgets

**Feature Branch**: `005-implement-real-widgets`  
**Date**: 2026-01-04

## Overview

This feature modifies existing builder methods in `gravity-iced`. No new data structures are required. The existing IR types (`WidgetNode`, `AttributeValue`, `EventBinding`) already support all necessary attributes and events.

## Existing Types (No Changes Required)

### HandlerMessage (gravity-iced/src/lib.rs)

```rust
/// Standard message type for handler-based applications
#[derive(Clone, Debug, PartialEq)]
pub enum HandlerMessage {
    /// Simple handler with optional payload
    Handler(String, Option<String>),
    //        ^-- handler name
    //                     ^-- optional value (text, "true"/"false", selection, number string)
}
```

**Usage for each widget type**:
- `TextInput`: `Handler("on_input_handler", Some("typed text"))`
- `Checkbox`: `Handler("on_toggle_handler", Some("true"))` or `Some("false")`
- `Toggler`: `Handler("on_toggle_handler", Some("true"))` or `Some("false")`
- `PickList`: `Handler("on_select_handler", Some("selected option"))`
- `Slider`: `Handler("on_change_handler", Some("42.5"))`
- `Image`: No events

### EventKind (gravity-core/src/ir/node.rs)

```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EventKind {
    Click,    // Button
    Input,    // TextInput
    Toggle,   // Checkbox, Toggler
    Select,   // PickList
    Change,   // Slider
    // ... others
}
```

### WidgetKind (gravity-core/src/ir/node.rs)

```rust
pub enum WidgetKind {
    // Already defined:
    TextInput,
    Checkbox,
    Slider,
    PickList,
    Toggler,
    Image,
    // ...
}
```

## Attribute Mapping

### TextInput Attributes

| XML Attribute | Type | Required | Default | Description |
|---------------|------|----------|---------|-------------|
| `placeholder` | String | No | `""` | Placeholder text when empty |
| `value` | Binding/String | No | `""` | Current text value |
| `on_input` | Handler | No | - | Handler called on input |

### Checkbox Attributes

| XML Attribute | Type | Required | Default | Description |
|---------------|------|----------|---------|-------------|
| `label` | String | No | `""` | Label text |
| `checked` | Binding/Bool | No | `false` | Checked state |
| `on_toggle` | Handler | No | - | Handler called on toggle |

### Toggler Attributes

| XML Attribute | Type | Required | Default | Description |
|---------------|------|----------|---------|-------------|
| `label` | String | No | `""` | Label text |
| `active` | Binding/Bool | No | `false` | Active state |
| `on_toggle` | Handler | No | - | Handler called on toggle |

### PickList Attributes

| XML Attribute | Type | Required | Default | Description |
|---------------|------|----------|---------|-------------|
| `options` | String | Yes | - | Comma-separated options |
| `selected` | Binding/String | No | `None` | Currently selected |
| `placeholder` | String | No | `"Select..."` | Placeholder when none selected |
| `on_select` | Handler | No | - | Handler called on selection |

### Slider Attributes

| XML Attribute | Type | Required | Default | Description |
|---------------|------|----------|---------|-------------|
| `min` | Float | No | `0.0` | Minimum value |
| `max` | Float | No | `100.0` | Maximum value |
| `value` | Binding/Float | No | `50.0` | Current value |
| `on_change` | Handler | No | - | Handler called on change |

### Image Attributes

| XML Attribute | Type | Required | Default | Description |
|---------------|------|----------|---------|-------------|
| `src` | String | Yes | - | Path to image file |
| `width` | Float | No | - | Display width |
| `height` | Float | No | - | Display height |

## Event Flow

```
┌─────────────────────┐
│   XML Definition    │
│ <text_input         │
│   value="{name}"    │
│   on_input="update" │
│ />                  │
└─────────┬───────────┘
          │
          ▼
┌─────────────────────┐
│   GravityWidgetBuilder   │
│ - evaluate_attribute()   │
│ - build_text_input()     │
└─────────┬───────────┘
          │
          ▼
┌─────────────────────┐
│   Iced TextInput    │
│ .on_input(|s| {     │
│   HandlerMessage::  │
│   Handler("update", │
│     Some(s))        │
│ })                  │
└─────────┬───────────┘
          │
          ▼ (user types)
┌─────────────────────┐
│ Application::update │
│ - Dispatch to       │
│   registry handler  │
│ - Update model      │
└─────────────────────┘
```

## Validation Rules

1. **PickList options**: Must have at least one option or render as empty dropdown
2. **Slider range**: `min` must be less than `max`
3. **Slider value**: Clamped to `[min, max]` range
4. **Image src**: Path validation at runtime (warning logged if not found)
5. **Boolean parsing**: Accept `"true"`, `"false"`, `"1"`, `"0"` for checked/active attributes
