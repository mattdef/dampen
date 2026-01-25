# Data Model: DatePicker & TimePicker Widgets

**Feature**: 001-datetime-picker
**Date**: 2026-01-25

## Entity Definitions

### WidgetKind (Extended)

**Location**: `crates/dampen-core/src/ir/node.rs`

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WidgetKind {
    // ... existing variants ...

    /// Date selection widget with calendar overlay
    DatePicker,

    /// Time selection widget with hour/minute/second picker
    TimePicker,
}
```

**Minimum Version**: Both require `Version::new(1, 1, 0)`

### DatePickerAttributes

**Logical representation of parsed date_picker attributes**

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| value | `Option<AttributeValue>` | No | None | Current date (binding or static) |
| format | `Option<String>` | No | `"%Y-%m-%d"` | Parse format for static values |
| show | `Option<AttributeValue>` | No | None | Overlay visibility binding |
| min_date | `Option<String>` | No | None | Minimum selectable date |
| max_date | `Option<String>` | No | None | Maximum selectable date |

**Events**:
| Event | Type | Payload |
|-------|------|---------|
| on_submit | `EventBinding` | Selected date as ISO 8601 string |
| on_cancel | `EventBinding` | None |

### TimePickerAttributes

**Logical representation of parsed time_picker attributes**

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| value | `Option<AttributeValue>` | No | None | Current time (binding or static) |
| format | `Option<String>` | No | `"%H:%M:%S"` | Parse format for static values |
| show | `Option<AttributeValue>` | No | None | Overlay visibility binding |
| use_24h | `bool` | No | `false` | Use 24-hour format |
| show_seconds | `bool` | No | `false` | Show seconds selector |

**Events**:
| Event | Type | Payload |
|-------|------|---------|
| on_submit | `EventBinding` | Selected time as HH:MM:SS string |
| on_cancel | `EventBinding` | None |

### WidgetSchema Definitions

**Location**: `crates/dampen-core/src/schema/mod.rs`

```rust
WidgetKind::DatePicker => WidgetSchema {
    required: &[],
    optional: &["value", "format", "show", "min_date", "max_date"],
    events: &["on_submit", "on_cancel"],
    style_attributes: COMMON_STYLE_ATTRIBUTES,
    layout_attributes: COMMON_LAYOUT_ATTRIBUTES,
},

WidgetKind::TimePicker => WidgetSchema {
    required: &[],
    optional: &["value", "format", "show", "use_24h", "show_seconds"],
    events: &["on_submit", "on_cancel"],
    style_attributes: COMMON_STYLE_ATTRIBUTES,
    layout_attributes: COMMON_LAYOUT_ATTRIBUTES,
},
```

## State Transitions

### DatePicker State Machine

```
┌─────────────┐    click underlay    ┌─────────────┐
│   CLOSED    │ ──────────────────► │    OPEN     │
│ show=false  │                      │  show=true  │
└─────────────┘                      └─────────────┘
      ▲                                    │
      │                                    │
      │         select date                │
      │    ◄────────────────────────       │
      │    on_submit(date)                 │
      │                                    │
      │         cancel                     │
      └────────────────────────────────────┘
             on_cancel()
```

### TimePicker State Machine

```
┌─────────────┐    click underlay    ┌─────────────┐
│   CLOSED    │ ──────────────────► │    OPEN     │
│ show=false  │                      │  show=true  │
└─────────────┘                      └─────────────┘
      ▲                                    │
      │                                    │
      │         confirm time               │
      │    ◄────────────────────────       │
      │    on_submit(time)                 │
      │                                    │
      │         cancel                     │
      └────────────────────────────────────┘
             on_cancel()
```

## Validation Rules

### DatePicker Validation

1. **Child count**: Must have exactly 1 child widget
   - Error: `ParseError::InvalidChildCount { expected: 1, found: N }`

2. **Value format**: If static string, must parse with format
   - Default format: `%Y-%m-%d`
   - Error: `ParseError::InvalidDateFormat { value, format }`

3. **Date range**: If min_date and max_date both present, min <= max
   - Error: `ParseError::InvalidDateRange { min, max }`

4. **Show binding**: Must resolve to boolean type
   - Error: `ParseError::TypeMismatch { expected: "bool", found }`

### TimePicker Validation

1. **Child count**: Must have exactly 1 child widget

2. **Value format**: If static string, must parse with format
   - Default format: `%H:%M:%S`
   - Error: `ParseError::InvalidTimeFormat { value, format }`

3. **Boolean attributes**: `use_24h` and `show_seconds` must be valid booleans
   - Accepted: `"true"`, `"false"`, `"1"`, `"0"`, `"yes"`, `"no"`

## Relationships

```
┌──────────────┐         ┌──────────────────┐
│  WidgetNode  │────────►│   WidgetKind     │
│              │         │  (DatePicker/    │
│ - kind       │         │   TimePicker)    │
│ - attributes │         └──────────────────┘
│ - children   │
│ - events     │
└──────────────┘
       │
       │ exactly 1 child
       ▼
┌──────────────┐
│  WidgetNode  │  (underlay widget - typically Button)
│  (child)     │
└──────────────┘
```

## Type Conversions

### XML → IR

```
XML Attribute              →  AttributeValue
───────────────────────────────────────────────
value="2026-01-25"         →  Static("2026-01-25")
value="{model.date}"       →  Binding(BindingExpr)
format="%d/%m/%Y"          →  Static("%d/%m/%Y")
show="{show_picker}"       →  Binding(BindingExpr)
use_24h="true"             →  Static("true")
min_date="2026-01-01"      →  Static("2026-01-01")
```

### IR → iced_aw (Runtime)

```
AttributeValue             →  iced_aw Type
───────────────────────────────────────────────
Static("2026-01-25")       →  Date { year: 2026, month: 1, day: 25 }
Binding evaluated          →  Date from BindingValue::String
Static("14:30:00")         →  Time { hour: 14, minute: 30, second: 0 }
```

### iced_aw → Event Payload

```
iced_aw Type               →  Handler Payload
───────────────────────────────────────────────
Date { 2026, 1, 25 }       →  "2026-01-25" (ISO 8601)
Time { 14, 30, 0 }         →  "14:30:00" (HH:MM:SS)
```
