# Quickstart: DatePicker & TimePicker Widgets

**Feature**: 001-datetime-picker
**Version**: Dampen 1.1+

## Basic Usage

### DatePicker

Select a date from a calendar overlay:

```xml
<dampen version="1.1">
    <column padding="20" spacing="10">
        <date_picker
            value="{selected_date}"
            show="{show_date_picker}"
            on_submit="handle_date_selected"
            on_cancel="close_date_picker">
            <button label="Pick Date" on_click="toggle_date_picker" />
        </date_picker>

        <text value="Selected: {selected_date}" />
    </column>
</dampen>
```

### TimePicker

Select a time with optional 24-hour format:

```xml
<dampen version="1.1">
    <column padding="20" spacing="10">
        <time_picker
            value="{selected_time}"
            show="{show_time_picker}"
            use_24h="true"
            show_seconds="true"
            on_submit="handle_time_selected"
            on_cancel="close_time_picker">
            <button label="Pick Time" on_click="toggle_time_picker" />
        </time_picker>

        <text value="Selected: {selected_time}" />
    </column>
</dampen>
```

## Handler Implementation

```rust
use chrono::{NaiveDate, NaiveTime};

#[derive(Default)]
pub struct Model {
    pub selected_date: Option<NaiveDate>,
    pub selected_time: Option<NaiveTime>,
    pub show_date_picker: bool,
    pub show_time_picker: bool,
}

// Toggle picker visibility
pub fn toggle_date_picker(model: &mut Model) {
    model.show_date_picker = !model.show_date_picker;
}

pub fn toggle_time_picker(model: &mut Model) {
    model.show_time_picker = !model.show_time_picker;
}

// Handle selections (payload is ISO format string)
pub fn handle_date_selected(model: &mut Model, date: &str) {
    model.selected_date = NaiveDate::parse_from_str(date, "%Y-%m-%d").ok();
    model.show_date_picker = false;
}

pub fn handle_time_selected(model: &mut Model, time: &str) {
    model.selected_time = NaiveTime::parse_from_str(time, "%H:%M:%S").ok();
    model.show_time_picker = false;
}

// Cancel handlers
pub fn close_date_picker(model: &mut Model) {
    model.show_date_picker = false;
}

pub fn close_time_picker(model: &mut Model) {
    model.show_time_picker = false;
}
```

## Custom Date Formats

Parse non-ISO date formats using the `format` attribute:

```xml
<!-- European format: DD/MM/YYYY -->
<date_picker
    value="25/01/2026"
    format="%d/%m/%Y"
    on_submit="handle_date">
    <button label="Pick Date" />
</date_picker>

<!-- US format: MM/DD/YYYY -->
<date_picker
    value="01/25/2026"
    format="%m/%d/%Y"
    on_submit="handle_date">
    <button label="Pick Date" />
</date_picker>

<!-- 12-hour time format -->
<time_picker
    value="2:30 PM"
    format="%I:%M %p"
    on_submit="handle_time">
    <button label="Pick Time" />
</time_picker>
```

## Date Range Constraints

Limit selectable dates:

```xml
<!-- Only allow dates in 2026 -->
<date_picker
    value="{booking_date}"
    min_date="2026-01-01"
    max_date="2026-12-31"
    on_submit="handle_booking">
    <button label="Select Booking Date" />
</date_picker>
```

## Combined Date-Time Picker

Build a datetime selection flow:

```xml
<dampen version="1.1">
    <column padding="20" spacing="15">
        <text value="Schedule Appointment" size="24" weight="bold" />

        <row spacing="10" align="center">
            <date_picker
                value="{appointment_date}"
                show="{show_date}"
                min_date="2026-01-25"
                on_submit="date_selected">
                <button label="Date: {appointment_date}" on_click="open_date" />
            </date_picker>

            <time_picker
                value="{appointment_time}"
                show="{show_time}"
                use_24h="false"
                on_submit="time_selected">
                <button label="Time: {appointment_time}" on_click="open_time" />
            </time_picker>
        </row>

        <button
            label="Confirm Appointment"
            on_click="confirm"
            enabled="{appointment_date != '' && appointment_time != ''}" />
    </column>
</dampen>
```

## Attributes Reference

### DatePicker

| Attribute | Type | Default | Description |
|-----------|------|---------|-------------|
| `value` | binding/string | - | Current date |
| `format` | string | `%Y-%m-%d` | Parse format for static values |
| `show` | binding | - | Controls overlay visibility |
| `min_date` | string | - | Minimum selectable date |
| `max_date` | string | - | Maximum selectable date |
| `on_submit` | handler | - | Called with ISO date on selection |
| `on_cancel` | handler | - | Called when cancelled |

### TimePicker

| Attribute | Type | Default | Description |
|-----------|------|---------|-------------|
| `value` | binding/string | - | Current time |
| `format` | string | `%H:%M:%S` | Parse format for static values |
| `show` | binding | - | Controls overlay visibility |
| `use_24h` | bool | `false` | Use 24-hour format |
| `show_seconds` | bool | `false` | Show seconds selector |
| `on_submit` | handler | - | Called with HH:MM:SS on selection |
| `on_cancel` | handler | - | Called when cancelled |

## Common Patterns

### Initial Value from Today

```rust
use chrono::Local;

impl Model {
    pub fn new() -> Self {
        Self {
            selected_date: Some(Local::now().naive_local().date()),
            selected_time: Some(Local::now().naive_local().time()),
            ..Default::default()
        }
    }
}
```

### Validation Before Submit

```rust
pub fn validate_and_submit(model: &mut Model) {
    if let (Some(date), Some(time)) = (&model.selected_date, &model.selected_time) {
        let datetime = date.and_time(*time);
        if datetime > Local::now().naive_local() {
            // Valid future datetime - proceed
            model.submit_appointment(datetime);
        } else {
            model.error = Some("Please select a future date and time".into());
        }
    }
}
```

### Formatting for Display

```rust
impl Model {
    pub fn formatted_date(&self) -> String {
        self.selected_date
            .map(|d| d.format("%B %d, %Y").to_string())
            .unwrap_or_else(|| "Not selected".into())
    }

    pub fn formatted_time(&self) -> String {
        self.selected_time
            .map(|t| t.format("%I:%M %p").to_string())
            .unwrap_or_else(|| "Not selected".into())
    }
}
```
