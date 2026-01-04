# Feature Specification: Implement Real Iced Widgets

**Feature Branch**: `005-implement-real-widgets`  
**Created**: 2026-01-04  
**Status**: Draft  
**Input**: User description: "Les widgets text-input, checkbox, slider, pick_list, toggle et image sont actuellement de simple placeholder dans le crates/gravity-iced/src/builder.rs. Il faut implémenter les vrais widget de Iced pour qu'ils soit pleinement fonctionnels avec leurs évènements et leurs styles."

## Overview

The Gravity framework's Iced backend currently has six widget builders that return placeholder text elements instead of functional widgets. This feature implements the actual Iced widgets for `text_input`, `checkbox`, `slider`, `pick_list`, `toggler`, and `image`, with full support for:

- **Attribute binding**: Reading values from the model via `{binding}` syntax
- **Event handling**: Connecting handlers from the registry for user interactions
- **Style application**: Applying visual styles from classes and attributes

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Text Input Widget (Priority: P1)

As a developer, I want to use a text input widget in my Gravity UI so that users can enter and modify text values with real-time updates to my application model.

**Why this priority**: Text input is the most fundamental interactive widget. Without it, users cannot provide any text-based input to the application. It's used in forms, search bars, and data entry - essential for virtually any interactive application.

**Independent Test**: Create a simple `.gravity` file with a text input bound to a model field. Typing in the input should update the model field, and changing the model field programmatically should update the displayed text.

**Acceptance Scenarios**:

1. **Given** a text input with `value="{field}"`, **When** the widget renders, **Then** it displays the current value of `field` from the model
2. **Given** a text input with `on_input="handler"`, **When** the user types text, **Then** the handler receives the new text value
3. **Given** a text input with `placeholder="Enter text..."`, **When** the field is empty, **Then** the placeholder text is displayed
4. **Given** a text input with a style class, **When** rendered, **Then** visual styling is applied

---

### User Story 2 - Checkbox Widget (Priority: P1)

As a developer, I want to use a checkbox widget in my Gravity UI so that users can toggle boolean options with a label displayed next to the checkbox.

**Why this priority**: Checkboxes are essential for boolean input (yes/no, enable/disable). They're used in settings, forms, and task lists. Critical for the todo-app example which requires checkboxes for task completion.

**Independent Test**: Create a `.gravity` file with a checkbox bound to a boolean model field. Clicking the checkbox should toggle the model field value.

**Acceptance Scenarios**:

1. **Given** a checkbox with `checked="{is_enabled}"`, **When** the widget renders, **Then** it shows checked state matching the model value
2. **Given** a checkbox with `on_toggle="handler"`, **When** the user clicks the checkbox, **Then** the handler is called with the new boolean value
3. **Given** a checkbox with `label="Accept Terms"`, **When** rendered, **Then** the label text appears next to the checkbox

---

### User Story 3 - Toggler Widget (Priority: P1)

As a developer, I want to use a toggler switch widget in my Gravity UI so that users can toggle settings with a modern switch-style UI control.

**Why this priority**: Togglers are commonly used for on/off settings (dark mode, notifications). They provide a more modern alternative to checkboxes for binary choices. Already used in the todo-app example for dark mode toggle.

**Independent Test**: Create a `.gravity` file with a toggler bound to a boolean model field. Sliding the toggle should update the model field.

**Acceptance Scenarios**:

1. **Given** a toggler with `active="{dark_mode}"`, **When** the widget renders, **Then** it shows active/inactive state matching the model value
2. **Given** a toggler with `on_toggle="handler"`, **When** the user clicks/slides the toggler, **Then** the handler is called with the new boolean value
3. **Given** a toggler with `label="Enable Feature"`, **When** rendered, **Then** the label text appears next to the toggle switch

---

### User Story 4 - Pick List Widget (Priority: P2)

As a developer, I want to use a dropdown selection widget in my Gravity UI so that users can choose from a predefined list of options.

**Why this priority**: Pick lists enable selection from multiple options (categories, priorities, filters). Essential for forms and settings. Used in todo-app for category and priority selection.

**Independent Test**: Create a `.gravity` file with a pick list showing options. Selecting an option should update the model field.

**Acceptance Scenarios**:

1. **Given** a pick_list with `options="A,B,C"`, **When** the widget renders, **Then** it displays a dropdown with options A, B, and C
2. **Given** a pick_list with `selected="{choice}"`, **When** rendered, **Then** the currently selected option matches the model value
3. **Given** a pick_list with `on_select="handler"`, **When** the user selects an option, **Then** the handler receives the selected option value
4. **Given** a pick_list with `placeholder="Select..."`, **When** no option is selected, **Then** the placeholder is displayed

---

### User Story 5 - Slider Widget (Priority: P2)

As a developer, I want to use a slider widget in my Gravity UI so that users can select numeric values within a range by dragging a slider handle.

**Why this priority**: Sliders are essential for selecting values in a range (volume, brightness, quantities). Important for applications requiring numeric input with visual feedback.

**Independent Test**: Create a `.gravity` file with a slider bound to a numeric model field. Dragging the slider should update the model field value.

**Acceptance Scenarios**:

1. **Given** a slider with `min="0" max="100" value="{volume}"`, **When** the widget renders, **Then** the slider handle position reflects the model value
2. **Given** a slider with `on_change="handler"`, **When** the user drags the slider, **Then** the handler receives the new numeric value continuously
3. **Given** a slider with step attribute, **When** dragged, **Then** values snap to step increments

---

### User Story 6 - Image Widget (Priority: P3)

As a developer, I want to display images in my Gravity UI so that I can show visual content loaded from files.

**Why this priority**: Images enhance visual presentation. While not strictly required for functionality, they're important for complete UI experiences. Used in todo-app for priority icons.

**Independent Test**: Create a `.gravity` file with an image element pointing to a valid image file. The image should display at the specified dimensions.

**Acceptance Scenarios**:

1. **Given** an image with `src="path/to/image.png"`, **When** the widget renders, **Then** the image from the file is displayed
2. **Given** an image with `width="100" height="100"`, **When** rendered, **Then** the image is sized to the specified dimensions
3. **Given** an image with an invalid path, **When** rendered, **Then** a fallback or error indicator is shown gracefully

---

### Edge Cases

- What happens when a binding expression cannot be evaluated? → Display a sensible default value (empty string, false, 0)
- How does the slider handle values outside min/max range? → Clamp to valid range
- What happens when pick_list options are empty? → Render empty dropdown
- How does text_input handle very long text? → Scrollable input following Iced defaults
- What happens when image file doesn't exist? → Show placeholder or error message

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST render `text_input` as a functional Iced TextInput widget with placeholder, value binding, and on_input event support
- **FR-002**: System MUST render `checkbox` as a functional Iced Checkbox widget with label, checked binding, and on_toggle event support
- **FR-003**: System MUST render `toggler` as a functional Iced Toggler widget with label, active binding, and on_toggle event support
- **FR-004**: System MUST render `pick_list` as a functional Iced PickList widget with options, selected binding, and on_select event support
- **FR-005**: System MUST render `slider` as a functional Iced Slider widget with min, max, value binding, and on_change event support
- **FR-006**: System MUST render `image` as a functional Iced Image widget loading from the specified src path
- **FR-007**: All widgets MUST evaluate attribute bindings via the existing `evaluate_attribute` method
- **FR-008**: All event-based widgets MUST use the HandlerMessage pattern for events that pass values (text, boolean, selection, numeric)
- **FR-009**: All widgets MUST apply style properties from class references and inline styles
- **FR-010**: System MUST handle binding evaluation failures gracefully with sensible defaults
- **FR-011**: System MUST log widget creation and binding evaluation when verbose mode is enabled

### Key Entities

- **HandlerMessage**: Message enum with `Handler(String, Option<String>)` variant for passing handler name and optional value
- **WidgetNode**: IR node containing attributes, events, style, and layout information
- **EventBinding**: Links event types (Input, Toggle, Select, Change) to handler names
- **AttributeValue**: Static, Binding, or Interpolated attribute values

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: All six placeholder widgets are replaced with functional Iced widgets
- **SC-002**: Existing tests pass after widget implementation
- **SC-003**: The todo-app example renders correctly with functional text_input, pick_list, and toggler widgets
- **SC-004**: Event handlers receive correct values from all interactive widgets
- **SC-005**: Binding expressions correctly populate widget initial values from model state
- **SC-006**: No regression in build performance (< 1 second for builder builds)

## Assumptions

- Iced 0.14+ provides all required widget types (TextInput, Checkbox, Toggler, PickList, Slider, Image)
- The existing message factory pattern (`Box<dyn Fn(&str) -> Message + 'a>`) can be extended to pass values
- Images will be loaded from file paths relative to the application working directory
- Options for pick_list are provided as comma-separated strings in the options attribute
- Slider step defaults to continuous if not specified
