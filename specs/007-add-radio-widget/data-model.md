# Data Model: Add Radio Widget

**Feature Branch**: `007-add-radio-widget`
**Date**: 2026-01-08

## Overview

This feature adds a Radio widget to the Gravity framework. Radio buttons allow users to select a single option from a mutually exclusive group. The implementation follows the existing widget patterns and reuses core IR types without requiring new data structures.

## Entity Definitions

### RadioWidget (Conceptual)

The Radio widget is an atomic widget (like Button, Checkbox) that displays a circular selection control with a label. It does not require a container or special attributes struct.

**Attributes** (standard WidgetNode fields):

| Field | Type | Description |
|-------|------|-------------|
| `kind` | WidgetKind::Radio | Widget type discriminator |
| `label` | String | Text displayed next to radio button |
| `value` | String | Internal value for this option |
| `selected` | Option<String> | Currently selected value in the group |
| `on_select` | Handler | Called when selection changes |

## Existing Types (Extended)

### WidgetKind (gravity-core/src/ir/node.rs)

```rust
pub enum WidgetKind {
    // ... existing variants
    Radio,  // NEW: Radio button widget
    // ...
}
```

### EventKind (gravity-core/src/ir/node.rs)

```rust
pub enum EventKind {
    // ... existing variants
    Select,  // Used for PickList, will also be used for Radio
    // ...
}
```

## Attribute Mapping

### Radio Attributes

| XML Attribute | Type | Required | Default | Description |
|---------------|------|----------|---------|-------------|
| `label` | String | Yes | - | Text displayed next to radio button |
| `value` | String | Yes | - | Internal value for this option |
| `selected` | Binding/Option<String> | No | `None` | Currently selected value |
| `on_select` | Handler | No | - | Handler called on selection change |
| `disabled` | Binding/Bool | No | `false` | Disable user interaction |

### Handler Message Format

```rust
Handler("handler_name", Some("selected_value"))
//                     ^-- the value of the selected radio option
```

## Validation Rules

1. **Value uniqueness**: Within a logical radio group, each radio's `value` should be unique
2. **Selected value match**: If `selected` is set, it should match one of the radio `value`s in the group
3. **Disabled state**: Disabled radio buttons should not respond to clicks
4. **Label text**: Empty labels are allowed but not recommended (poor UX)

## Event Flow

```
┌─────────────────────────────────┐
│   XML Definition                │
│ <radio label="Option A"         │
│   value="a"                     │
│   selected={current_selection}  │
│   on_select="handleSelect"      │
│ />                              │
└─────────────┬───────────────────┘
              │
              ▼
┌─────────────────────────────────┐
│   GravityWidgetBuilder          │
│ - evaluate_attribute()          │
│ - build_radio()                 │
└─────────────┬───────────────────┘
              │
              ▼
┌─────────────────────────────────┐
│   Iced Radio                    │
│ radio("Option A",              │
│   Value::A,                     │
│   current_selection,            │
│   |v| HandlerMessage::          │
│     Handler("handleSelect",     │
│       Some(format!("{:?}", v))) │
│ )                               │
└─────────────┬───────────────────┘
              │
              ▼ (user clicks)
┌─────────────────────────────────┐
│   Application::update           │
│ - Dispatch to handler registry  │
│ - Update model                  │
│ - Re-render with new selection  │
└─────────────────────────────────┘
```

## XML Examples

### Basic Radio Group

```xml
<column>
  <radio label="Small" value="small" selected={size} on_select="setSize"/>
  <radio label="Medium" value="medium" selected={size} on_select="setSize"/>
  <radio label="Large" value="large" selected={size} on_select="setSize"/>
</column>
```

### Radio with Disabled Option

```xml
<radio label="Premium Feature" 
       value="premium" 
       selected={plan} 
       on_select="setPlan"
       disabled={!is_premium_user}/>
```

### Read-Only Radio Display

```xml
<radio label="Current Selection" 
       value={current_value} 
       selected={current_value}
       on_select=""/>
```

## Relationships

- **RadioGroup** (conceptual): Multiple Radio widgets sharing the same bound `selected` value
- **Selection Model**: The bound `selected` value determines which radio appears checked
- **Event Dispatch**: Selection changes generate `Handler("handler_name", Some(value))` messages
