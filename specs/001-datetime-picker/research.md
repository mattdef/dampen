# Research: DatePicker & TimePicker Widgets

**Feature**: 001-datetime-picker
**Date**: 2026-01-25
**Status**: Complete

## Research Questions

### 1. iced_aw DatePicker/TimePicker API

**Decision**: Use iced_aw 0.13+ widgets with underlay pattern

**Rationale**: iced_aw is the official extension library for Iced, providing production-ready date and time picker components. The API uses an underlay pattern where a child widget (typically a button) triggers the picker overlay.

**API Summary**:

```rust
// DatePicker constructor
DatePicker::new(
    show_picker: bool,           // Controls overlay visibility
    date: impl Into<Date>,       // Current date value
    underlay: U,                 // Child widget (trigger element)
    on_cancel: Message,          // Cancel callback
    on_submit: F,                // F: Fn(Date) -> Message
) -> Self

// TimePicker constructor
TimePicker::new(
    show_picker: bool,
    time: impl Into<Time>,
    underlay: U,
    on_cancel: Message,
    on_submit: F,                // F: Fn(Time) -> Message
) -> Self

// TimePicker configuration
.use_24h()                       // 24-hour format (default: 12-hour)
.show_seconds()                  // Show seconds selector (default: hidden)
```

**Alternatives Considered**:
- Custom implementation: Rejected due to complexity of calendar/clock UI
- egui date pickers: Rejected as Dampen uses Iced backend

### 2. chrono Integration

**Decision**: Use chrono::NaiveDate and chrono::NaiveTime for date/time representation

**Rationale**: chrono is the de-facto Rust library for date/time handling, provides ISO 8601 parsing, and integrates seamlessly with iced_aw's Date/Time types. The `serde` feature enables serialization for event payloads.

**Conversion Patterns**:
```rust
// iced_aw Date ↔ chrono NaiveDate
let iced_date = iced_aw::core::date::Date::from(naive_date);
let naive = chrono::NaiveDate::from_ymd_opt(date.year, date.month, date.day);

// iced_aw Time ↔ chrono NaiveTime
let iced_time = Time::new(hour, minute, second);
let naive = chrono::NaiveTime::from_hms_opt(time.hour, time.minute, time.second);
```

**Alternatives Considered**:
- time crate: Less widely adopted, no serde by default
- Custom date types: Unnecessary abstraction

### 3. Underlay Pattern Implementation

**Decision**: Require exactly one child widget as underlay

**Rationale**: iced_aw's DatePicker/TimePicker widgets wrap a child element that serves as the trigger. This matches common UI patterns (click button → show picker). The parser validates child count.

**XML Mapping**:
```xml
<date_picker value="{date}" show="{show_picker}" on_submit="handle_date">
    <button label="Pick Date" on_click="toggle_picker" />
</date_picker>
```

**Builder Pattern**:
```rust
fn build_date_picker(&self, node: &WidgetNode) -> Element<HandlerMessage> {
    let child = node.children.first().expect("validated by parser");
    let underlay = self.build_widget(child);

    DatePicker::new(
        self.resolve_show_binding(node),
        self.resolve_date_value(node),
        underlay,
        HandlerMessage::Handler(cancel_handler, None),
        |date| HandlerMessage::Handler(submit_handler, Some(date.to_string())),
    )
}
```

### 4. Event Payload Serialization

**Decision**: Serialize dates as ISO 8601 strings, times as HH:MM:SS strings

**Rationale**: String serialization provides universal handler compatibility without requiring handlers to understand Date/Time types. ISO 8601 is unambiguous and parseable.

**Format Examples**:
- Date: `"2026-01-25"` (YYYY-MM-DD)
- Time: `"14:30:00"` (HH:MM:SS, 24-hour)

**Handler Signature**:
```rust
fn handle_date_selection(model: &mut Model, date: &str) {
    model.selected_date = NaiveDate::parse_from_str(date, "%Y-%m-%d").ok();
}
```

### 5. Static Value Parsing

**Decision**: Support custom format attribute for non-ISO static values

**Rationale**: International applications need regional date formats. The `format` attribute uses chrono strftime syntax.

**Parsing Logic**:
```rust
fn parse_static_date(value: &str, format: Option<&str>) -> Result<NaiveDate, ParseError> {
    let fmt = format.unwrap_or("%Y-%m-%d");
    NaiveDate::parse_from_str(value, fmt)
        .map_err(|e| ParseError::invalid_date(value, fmt, e))
}
```

**Supported Formats**:
- `%Y-%m-%d` - ISO 8601 (default)
- `%d/%m/%Y` - European
- `%m/%d/%Y` - US
- `%d-%m-%Y` - Alternative European

### 6. Schema Definition

**Decision**: Define comprehensive schemas matching iced_aw capabilities

**DatePicker Schema**:
| Attribute | Type | Required | Description |
|-----------|------|----------|-------------|
| value | Binding\|String | No | Current date |
| format | String | No | Parse format for static value |
| show | Binding | No | Overlay visibility control |
| min_date | String | No | Minimum selectable date |
| max_date | String | No | Maximum selectable date |

**DatePicker Events**:
| Event | Payload | Description |
|-------|---------|-------------|
| on_submit | ISO 8601 date string | Date selected |
| on_cancel | None | Picker cancelled |

**TimePicker Schema**:
| Attribute | Type | Required | Description |
|-----------|------|----------|-------------|
| value | Binding\|String | No | Current time |
| format | String | No | Parse format for static value |
| show | Binding | No | Overlay visibility control |
| use_24h | Bool | No | 24-hour format (default: false) |
| show_seconds | Bool | No | Show seconds (default: false) |

**TimePicker Events**:
| Event | Payload | Description |
|-------|---------|-------------|
| on_submit | HH:MM:SS time string | Time selected |
| on_cancel | None | Picker cancelled |

### 7. Codegen Strategy

**Decision**: Generate iced_aw widget construction code identical to interpreted mode

**Rationale**: Maintains visual parity between modes (Constitution Principle III).

**Generated Code Pattern**:
```rust
// Generated by dampen build
let date_picker = DatePicker::new(
    model.show_date_picker,
    model.selected_date.unwrap_or_else(|| Date::from(Local::now().naive_local().date())),
    button(text("Pick Date")).on_press(Message::ToggleDatePicker),
    Message::CancelDatePicker,
    Message::SubmitDate,
);
```

### 8. Error Messages

**Decision**: Provide actionable parse errors with suggestions

**Error Patterns**:

```text
error: DatePicker requires exactly one child widget
  --> src/ui/window.dampen:15:5
   |
15 | <date_picker value="{date}">
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = help: Add a child widget to serve as the picker trigger:

   <date_picker value="{date}">
       <button label="Pick Date" />
   </date_picker>
```

```text
error: Invalid date format in value attribute
  --> src/ui/window.dampen:15:23
   |
15 | <date_picker value="23-01-2026" ...>
   |                     ^^^^^^^^^^^
   |
   = note: Could not parse "23-01-2026" with format "%Y-%m-%d"
   = help: Use ISO 8601 format (2026-01-23) or add format="%d-%m-%Y"
```

## Dependencies

### New Dependencies (dampen-iced/Cargo.toml)

```toml
[dependencies]
iced_aw = { version = "0.13", default-features = false, features = ["date_picker", "time_picker"] }
chrono = { version = "0.4", features = ["serde"] }
```

### Feature Dependencies
- `date_picker` requires `chrono`
- `time_picker` requires `chrono` + `iced_widget/canvas`

## Test Strategy

### Unit Tests (dampen-core)
1. Parse valid date_picker XML → WidgetKind::DatePicker
2. Parse valid time_picker XML → WidgetKind::TimePicker
3. Schema validation accepts valid attributes
4. Schema rejects unknown attributes
5. Parser error on zero children
6. Parser error on multiple children
7. Static date parsing with default format
8. Static date parsing with custom format
9. Invalid date format produces actionable error

### Integration Tests (dampen-iced)
1. DatePicker renders without crash
2. TimePicker renders with use_24h=true
3. TimePicker renders with show_seconds=true
4. on_submit fires with correct date payload
5. on_cancel fires when cancelled

### Snapshot Tests (codegen)
1. DatePicker codegen matches expected Rust output
2. TimePicker codegen matches expected Rust output
3. Complex date_picker with all attributes
