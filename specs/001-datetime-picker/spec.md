# Feature Specification: DatePicker & TimePicker Widgets

**Feature Branch**: `001-datetime-picker`
**Created**: 2026-01-25
**Status**: Draft
**Input**: User description: "Add DatePicker and TimePicker widgets to Dampen v1.1, leveraging chrono for date/time handling and iced_aw for UI backend"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Select a Date from Calendar (Priority: P1)

As a user building a form in Dampen, I want to add a date picker widget that displays a calendar overlay, so users can easily select dates without typing.

**Why this priority**: Date selection is the core functionality of the DatePicker widget. Without this, the widget has no purpose.

**Independent Test**: Can be fully tested by defining a `<date_picker>` element in XML with a button underlay, clicking the button to show the calendar, selecting a date, and verifying the selected date is captured.

**Acceptance Scenarios**:

1. **Given** a DatePicker widget with a button underlay, **When** the user clicks the button, **Then** a calendar overlay appears showing the current month
2. **Given** an open calendar overlay, **When** the user clicks on a date, **Then** the overlay closes and the `on_submit` event fires with the selected date
3. **Given** an open calendar overlay, **When** the user clicks cancel or outside the overlay, **Then** the overlay closes and the `on_cancel` event fires
4. **Given** a DatePicker with `value` attribute set, **When** the calendar opens, **Then** the specified date is highlighted as the current selection

---

### User Story 2 - Select a Time (Priority: P1)

As a user building a form in Dampen, I want to add a time picker widget that displays hour/minute selectors, so users can easily select times without typing.

**Why this priority**: Time selection is the core functionality of the TimePicker widget. This is equally important as date selection.

**Independent Test**: Can be fully tested by defining a `<time_picker>` element in XML with a button underlay, clicking the button to show the time picker, selecting a time, and verifying the selected time is captured.

**Acceptance Scenarios**:

1. **Given** a TimePicker widget with a button underlay, **When** the user clicks the button, **Then** a time selection overlay appears
2. **Given** an open time picker overlay with `use_24h="true"`, **When** viewing the hour selector, **Then** hours are displayed in 24-hour format (0-23)
3. **Given** an open time picker overlay with `use_24h="false"` or unset, **When** viewing the hour selector, **Then** hours are displayed in 12-hour format with AM/PM toggle
4. **Given** an open time picker overlay with `show_seconds="true"`, **When** viewing the picker, **Then** a seconds selector is visible
5. **Given** an open time picker, **When** the user confirms selection, **Then** the overlay closes and `on_submit` event fires with the selected time

---

### User Story 3 - Custom Date/Time Formats (Priority: P2)

As a developer, I want to specify custom date/time formats for parsing static values, so I can support regional date formats and specific time notations.

**Why this priority**: Format flexibility is important for international applications but the widgets work without it using ISO defaults.

**Independent Test**: Can be fully tested by defining a DatePicker with `value="23/01/2026" format="%d/%m/%Y"`, rendering the widget, and verifying the date is correctly parsed and displayed.

**Acceptance Scenarios**:

1. **Given** a DatePicker with `value="23/01/2026" format="%d/%m/%Y"`, **When** the widget renders, **Then** the date January 23, 2026 is recognized
2. **Given** a DatePicker with `value="2026-01-23"` (no format specified), **When** the widget renders, **Then** the date is parsed using ISO 8601 default format
3. **Given** a TimePicker with `value="2:30 PM" format="%I:%M %p"`, **When** the widget renders, **Then** the time 14:30 is recognized
4. **Given** a TimePicker with `value="14:30:00"` (no format specified), **When** the widget renders, **Then** the time is parsed using default 24-hour format

---

### User Story 4 - Data Binding for Dates and Times (Priority: P2)

As a developer, I want to bind date/time picker values to my application state, so selected dates/times automatically sync with my data model.

**Why this priority**: Data binding enables reactive UIs but widgets can function with static values and event handlers alone.

**Independent Test**: Can be fully tested by creating a DatePicker with `value="{selected_date}"` binding, selecting a date, and verifying the bound state variable updates.

**Acceptance Scenarios**:

1. **Given** a DatePicker with `value="{model.date}"`, **When** the date in `model.date` changes, **Then** the picker reflects the new date
2. **Given** a TimePicker with `value="{model.time}"`, **When** a time is selected, **Then** `model.time` updates with the new value
3. **Given** a bound DatePicker, **When** `on_submit` fires, **Then** the event payload contains the selected date in a serializable format

---

### User Story 5 - Date Range Constraints (Priority: P3)

As a developer, I want to specify minimum and maximum allowed dates, so users cannot select dates outside a valid range.

**Why this priority**: Range constraints are valuable for booking systems but not essential for basic date selection.

**Independent Test**: Can be fully tested by defining a DatePicker with `min_date` and `max_date`, opening the calendar, and verifying out-of-range dates are disabled.

**Acceptance Scenarios**:

1. **Given** a DatePicker with `min_date="2026-01-01"`, **When** the calendar displays December 2025, **Then** all dates are disabled (not selectable)
2. **Given** a DatePicker with `max_date="2026-12-31"`, **When** the calendar displays January 2027, **Then** all dates are disabled
3. **Given** a DatePicker with both min and max constraints, **When** a user tries to select an out-of-range date, **Then** the selection is prevented

---

### Edge Cases

- What happens when an invalid date format is provided in the `value` attribute? The parser returns a clear error with the invalid value and expected format.
- What happens when `min_date` is after `max_date`? The parser returns a validation error.
- What happens when the DatePicker/TimePicker has zero children? A parse error indicates exactly one child widget is required.
- What happens when the DatePicker/TimePicker has more than one child? A parse error indicates only one child widget is allowed.
- What happens when `show` binding is not a boolean? A type error is raised during validation.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST support a `<date_picker>` XML element that renders a date selection widget
- **FR-002**: System MUST support a `<time_picker>` XML element that renders a time selection widget
- **FR-003**: DatePicker MUST accept exactly one child widget as the "underlay" (trigger element)
- **FR-004**: TimePicker MUST accept exactly one child widget as the "underlay" (trigger element)
- **FR-005**: DatePicker MUST support a `value` attribute accepting either a binding expression or static string
- **FR-006**: TimePicker MUST support a `value` attribute accepting either a binding expression or static string
- **FR-007**: DatePicker MUST support a `format` attribute for specifying the parsing format of static date strings
- **FR-008**: TimePicker MUST support a `format` attribute for specifying the parsing format of static time strings
- **FR-009**: DatePicker MUST support a `show` attribute bound to a boolean for controlling overlay visibility
- **FR-010**: TimePicker MUST support a `show` attribute bound to a boolean for controlling overlay visibility
- **FR-011**: DatePicker MUST support `on_submit` event triggered when a date is selected
- **FR-012**: DatePicker MUST support `on_cancel` event triggered when selection is cancelled
- **FR-013**: TimePicker MUST support `on_submit` event triggered when a time is selected
- **FR-014**: TimePicker MUST support `on_cancel` event triggered when selection is cancelled
- **FR-015**: TimePicker MUST support a `use_24h` boolean attribute to toggle between 12/24-hour display
- **FR-016**: TimePicker MUST support a `show_seconds` boolean attribute to show/hide seconds selector
- **FR-017**: DatePicker MUST support optional `min_date` attribute to constrain minimum selectable date
- **FR-018**: DatePicker MUST support optional `max_date` attribute to constrain maximum selectable date
- **FR-019**: System MUST use `%Y-%m-%d` as the default date parsing format when no `format` attribute is specified
- **FR-020**: System MUST use `%H:%M:%S` as the default time parsing format when no `format` attribute is specified
- **FR-021**: System MUST extend `BindingValue` enum to include `Date` and `Time` variants
- **FR-022**: System MUST register these widgets as requiring Dampen version 1.1 or higher
- **FR-023**: Parser MUST return a clear error if DatePicker/TimePicker has zero or more than one child
- **FR-024**: Both widgets MUST work in interpreted mode (dampen run) and codegen mode (dampen build)

### Key Entities

- **Date**: Represents a calendar date (year, month, day) without time zone information. Uses ISO 8601 format by default.
- **Time**: Represents a time of day (hour, minute, second) without date or time zone. Uses 24-hour notation by default.
- **BindingValue::Date**: New enum variant wrapping a date value for the reactive binding system.
- **BindingValue::Time**: New enum variant wrapping a time value for the reactive binding system.
- **WidgetKind::DatePicker**: New widget kind in the IR representing date picker elements.
- **WidgetKind::TimePicker**: New widget kind in the IR representing time picker elements.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Developers can define a DatePicker widget in XML and see a functional calendar overlay when triggered
- **SC-002**: Developers can define a TimePicker widget in XML and see a functional time selection overlay when triggered
- **SC-003**: Selected dates/times are correctly captured and passed to event handlers in a usable format
- **SC-004**: Custom format strings correctly parse non-ISO date/time values without errors
- **SC-005**: Both widgets function identically in interpreted mode and codegen mode (visual and behavioral parity)
- **SC-006**: Parser provides clear, actionable error messages for malformed widget definitions within 1 second of validation
- **SC-007**: Widgets render and respond to user interaction within 100ms on standard hardware
- **SC-008**: 100% of example XML configurations in documentation are valid and produce expected results

## Assumptions

- The `iced_aw` crate (v0.13+) provides the underlying DatePicker and TimePicker UI components
- The `chrono` crate provides date/time parsing and representation
- 12-hour format is the default for TimePicker (matching common user expectations) unless `use_24h="true"` is specified
- Seconds are hidden by default in TimePicker unless `show_seconds="true"` is specified
- The `show` attribute defaults to `false` (overlay hidden) when not explicitly bound
- Event payloads serialize dates as ISO 8601 strings and times as `HH:MM:SS` strings for handler consumption
