# Feature Specification: Advanced Widgets for Modern Todo App

**Feature Branch**: `004-advanced-widgets-todo`  
**Created**: 2026-01-04  
**Status**: Draft  
**Input**: User description: "Ajouter la gestion des widgets Iced necessaires pour fournir un exemple "todo-app" totalement fonctionnel. Widgets essentiels et obligatoires: Canvas, Combobox, Pick_list. Widgets facultatifs: Tooltip, Grid, Float, Progress_bar, Image. Faire un check sur les crates de Gravity pour mettre à jour les fonctionnalités qui utilisent ces nouveaux widgets. Ensuite refaire l'exemple todo_app pleinement fonctionnel et avec un visuel moderne en utilisant toutes les possibilités offerte par Gravity."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Basic Widget Support (Priority: P1)

A developer needs to use essential interactive widgets (Combobox, Pick_list) to create dropdown selections for todo categories, tags, and filters in a todo application.

**Why this priority**: These widgets are explicitly required ("essentiels et obligatoires") and are fundamental for user input in any modern todo app. Without them, users cannot categorize or filter their tasks effectively.

**Independent Test**: Create a minimal example with a Combobox for category selection and a Pick_list for filtering todos. Verify that selections trigger handlers and update the model correctly.

**Acceptance Scenarios**:

1. **Given** a todo app with a `<combobox>` for category selection, **When** user types and selects a category, **Then** the selected value is captured and can be used to categorize new todos
2. **Given** a todo app with a `<pick_list>` for filter options (All, Active, Completed), **When** user selects a filter, **Then** the todo list updates to show only matching items
3. **Given** both widgets with bindings like `selected="{current_category}"`, **When** the model updates, **Then** the widget selection reflects the current state
4. **Given** event handlers `on_select="update_category"`, **When** selection changes, **Then** the handler is invoked with the selected value
5. **Given** XML attributes for options (`options="Work,Personal,Shopping"`), **When** parsed, **Then** the widget displays all available options

---

### User Story 2 - Visual Enhancement Widgets (Priority: P2)

A developer wants to add visual polish to the todo app using Progress_bar (to show completion percentage), Tooltip (for help text), and Image (for icons or decorations).

**Why this priority**: These widgets improve user experience and make the app feel modern and professional. They are marked as optional but significantly enhance usability and aesthetics.

**Independent Test**: Add a progress bar showing "3 of 10 tasks completed", tooltips on action buttons explaining their purpose, and icons for task priorities. Verify visual rendering and tooltip behavior.

**Acceptance Scenarios**:

1. **Given** a `<progress_bar>` with `value="{completed_count}"` and `max="{items.len()}"`, **When** tasks are completed, **Then** the progress bar updates to reflect the percentage
2. **Given** a `<tooltip>` wrapping a button with `message="Delete all completed tasks"`, **When** user hovers, **Then** the tooltip appears with the help text
3. **Given** an `<image>` with `path="assets/icon-high-priority.png"` next to a task, **When** rendered, **Then** the image displays correctly
4. **Given** a `<progress_bar>` with no value attribute, **When** rendered, **Then** it shows indeterminate progress animation
5. **Given** multiple tooltips on the same screen, **When** user hovers different elements, **Then** only the relevant tooltip displays

---

### User Story 3 - Advanced Layout with Grid (Priority: P2)

A developer wants to organize todos in a grid layout showing multiple columns (task name, category, priority, due date, actions) for a spreadsheet-like view.

**Why this priority**: Grid layout is optional but enables more sophisticated UIs. It's particularly useful for todo apps that need to display multiple properties per task in a structured way.

**Independent Test**: Create a grid with 5 columns displaying todo properties. Verify that items align correctly and the grid is responsive.

**Acceptance Scenarios**:

1. **Given** a `<grid>` with `columns="5"` containing todo items, **When** rendered, **Then** items are displayed in a 5-column grid layout
2. **Given** grid items with varying content lengths, **When** rendered, **Then** columns maintain consistent alignment
3. **Given** a grid with `spacing="10"`, **When** rendered, **Then** there is 10px spacing between grid items
4. **Given** a grid with more items than fit in one row, **When** rendered, **Then** items wrap to new rows automatically
5. **Given** dynamic grid content from `{items}` binding, **When** items are added or removed, **Then** the grid updates automatically

---

### User Story 4 - Custom Visualizations with Canvas (Priority: P3)

A developer wants to use Canvas to create custom visualizations like a calendar view of tasks, a habit tracker heatmap, or a statistics chart showing task completion over time.

**Why this priority**: Canvas enables custom graphics and visualizations that go beyond standard widgets. It's marked as required but has lower priority because it's more advanced and not needed for basic todo functionality.

**Independent Test**: Create a simple canvas-based visualization showing a weekly calendar with colored dots indicating tasks per day. Verify that the canvas renders and responds to model updates.

**Acceptance Scenarios**:

1. **Given** a `<canvas>` with `width="400"` and `height="200"`, **When** rendered, **Then** a canvas element of the specified size is created
2. **Given** a canvas with custom rendering logic in Rust, **When** model data changes, **Then** the canvas redraws to reflect new data
3. **Given** a canvas showing task statistics, **When** tasks are completed, **Then** the visualization updates in real-time
4. **Given** a canvas with `on_click` handler, **When** user clicks, **Then** the handler receives the click coordinates
5. **Given** a canvas displaying a heatmap, **When** user hovers over cells, **Then** tooltip shows the date and task count

---

### User Story 5 - Float for Overlay UI Elements (Priority: P3)

A developer wants to use Float widget to create overlay UI elements like a modal dialog for task details, a floating action button, or a notification banner.

**Why this priority**: Float is optional and less common in todo apps, but useful for advanced UI patterns like modals and overlays.

**Independent Test**: Create a floating "Add Task" button that stays in the bottom-right corner, and a modal dialog that appears when editing a task. Verify positioning and z-index behavior.

**Acceptance Scenarios**:

1. **Given** a `<float>` with `position="bottom-right"` containing an "Add" button, **When** rendered, **Then** the button appears in the bottom-right corner regardless of scrolling
2. **Given** a float with `z_index="100"` containing a modal, **When** displayed, **Then** it appears above all other content
3. **Given** a float with visibility controlled by `visible="{show_modal}"`, **When** the binding changes, **Then** the float appears or disappears
4. **Given** a float containing form elements, **When** user interacts, **Then** events are handled normally
5. **Given** multiple floats with different z-index values, **When** rendered, **Then** they stack in the correct order

---

### User Story 6 - Comprehensive Modern Todo App Example (Priority: P1)

A developer wants to see a complete, modern todo app example that demonstrates all new widgets working together with attractive styling and full functionality.

**Why this priority**: This is the ultimate goal ("refaire l'exemple todo_app pleinement fonctionnel et avec un visuel moderne"). It serves as both a demonstration and a template for developers building real applications.

**Independent Test**: Run the todo-app example and verify all features work: adding tasks, categorizing with combobox, filtering with pick_list, viewing progress bar, seeing tooltips, displaying task icons, and custom visualizations.

**Acceptance Scenarios**:

1. **Given** the todo-app example, **When** user adds a task with category and priority, **Then** the task appears in the list with appropriate visual indicators
2. **Given** the todo-app example, **When** user completes tasks, **Then** the progress bar updates and statistics reflect the change
3. **Given** the todo-app example, **When** user hovers over action buttons, **Then** tooltips explain each action
4. **Given** the todo-app example, **When** user filters by category using pick_list, **Then** only matching tasks are displayed
5. **Given** the todo-app example, **When** user views the statistics section, **Then** a canvas-based visualization shows completion trends
6. **Given** the todo-app example, **When** user clicks "Edit" on a task, **Then** a floating modal appears with edit options
7. **Given** the todo-app example with dark mode enabled, **When** rendered, **Then** all new widgets respect the theme
8. **Given** the todo-app example, **When** inspected, **Then** the UI demonstrates modern design patterns with clean layout and professional styling

---

### Edge Cases

- What happens when a Combobox has no options provided? Display an empty dropdown with placeholder text
- How does Pick_list handle a very long list of options (100+ items)? Render with scrolling support
- What happens when Progress_bar receives a value greater than max? Clamp to 100%
- How does Tooltip behave on touch devices without hover? Show on tap/long-press
- What happens when Canvas rendering logic is missing? Display a placeholder or error message
- How does Grid handle items that don't evenly divide into columns? Last row contains remaining items aligned to the left
- What happens when Image path is invalid or file is missing? Display a broken image icon or placeholder
- How does Float handle positioning when viewport is too small? Adjust position to remain visible
- What happens when multiple Tooltips overlap? Only show the topmost element's tooltip
- How does Combobox handle user typing text that doesn't match any option? Allow custom entry or show "no matches" message

---

## Requirements *(mandatory)*

### Functional Requirements

#### Widget Support

- **FR-001**: System MUST support Combobox widget for searchable dropdown selections with user text input
- **FR-002**: System MUST support Pick_list widget for simple dropdown selections from predefined options
- **FR-003**: System MUST support Canvas widget for custom drawing and visualizations
- **FR-004**: System MUST support Tooltip widget for displaying contextual help text on hover
- **FR-005**: System MUST support Grid widget for multi-column layouts with automatic wrapping
- **FR-006**: System MUST support Float widget for positioned overlay elements
- **FR-007**: System MUST support Progress_bar widget for displaying completion progress
- **FR-008**: System MUST support Image widget for displaying image files (already supported, needs verification)

#### Widget Attributes and Events

- **FR-009**: Combobox MUST support attributes: `options`, `selected`, `placeholder`, `on_select`
- **FR-010**: Pick_list MUST support attributes: `options`, `selected`, `placeholder`, `on_select`
- **FR-011**: Canvas MUST support attributes: `width`, `height`, `on_click`, and custom rendering callbacks
- **FR-012**: Tooltip MUST support attributes: `message`, `position` (top/bottom/left/right)
- **FR-013**: Grid MUST support attributes: `columns`, `spacing`, `padding`
- **FR-014**: Float MUST support attributes: `position` (top-left/top-right/bottom-left/bottom-right), `z_index`, `offset_x`, `offset_y`
- **FR-015**: Progress_bar MUST support attributes: `value`, `max`, `show_percentage`

#### Widget Bindings

- **FR-016**: All new widgets MUST support data bindings in attributes (e.g., `value="{count}"`)
- **FR-017**: All new widgets MUST evaluate bindings using existing `evaluate_binding_expr` functionality
- **FR-018**: All new widgets MUST update automatically when bound model values change

#### Integration Requirements

- **FR-019**: System MUST add new widget types to `WidgetKind` enum in gravity-core
- **FR-020**: System MUST add widget rendering logic to `GravityWidgetBuilder` in gravity-iced
- **FR-021**: System MUST add event kinds for new widgets (e.g., `EventKind::Select`) to gravity-core
- **FR-022**: System MUST implement XML parsing for new widget attributes in gravity-core parser
- **FR-023**: System MUST support all new widgets in `gravity dev` hot-reload mode
- **FR-024**: System MUST support all new widgets in `gravity check` validation

#### Example Requirements

- **FR-025**: Todo-app example MUST demonstrate all 8 new widgets in a cohesive, functional application
- **FR-026**: Todo-app example MUST include modern styling with consistent colors, spacing, and typography
- **FR-027**: Todo-app example MUST support full CRUD operations on todo items
- **FR-028**: Todo-app example MUST include task categorization using Combobox
- **FR-029**: Todo-app example MUST include task filtering using Pick_list
- **FR-030**: Todo-app example MUST display completion progress using Progress_bar
- **FR-031**: Todo-app example MUST show contextual help using Tooltips
- **FR-032**: Todo-app example MUST use Canvas for a statistics or calendar visualization
- **FR-033**: Todo-app example MUST demonstrate Grid layout for multi-column task display
- **FR-034**: Todo-app example MUST use Float for modal dialogs or floating action buttons
- **FR-035**: Todo-app example MUST display task priority icons using Image widget

### Key Entities

- **Combobox**: Searchable dropdown widget allowing both selection from options and custom text input
- **Pick_list**: Simple dropdown widget for selecting from a predefined list of options
- **Canvas**: Custom drawing surface for visualizations and graphics
- **Tooltip**: Contextual help text that appears on hover or tap
- **Grid**: Multi-column layout container with automatic wrapping
- **Float**: Positioned overlay container for modals, floating buttons, notifications
- **Progress_bar**: Visual indicator of task completion or progress
- **TodoItem**: Data entity representing a task with properties: text, category, priority, completed status, due date
- **TodoCategory**: Predefined or custom categories for organizing tasks (Work, Personal, Shopping, etc.)
- **TodoFilter**: Filter criteria for displaying subsets of todos (All, Active, Completed, By Category)

---

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: All 8 widgets (Combobox, Pick_list, Canvas, Tooltip, Grid, Float, Progress_bar, Image) can be used in Gravity XML files
- **SC-002**: Each widget renders correctly in Iced with proper styling and layout
- **SC-003**: All widget event handlers (on_select, on_click, etc.) successfully connect to Rust handler functions
- **SC-004**: Data bindings in all widget attributes update correctly when model changes
- **SC-005**: Todo-app example demonstrates all 8 widgets working together in a single application
- **SC-006**: Todo-app example provides a modern, professional UI comparable to commercial todo applications
- **SC-007**: Users can complete all common todo app tasks (add, edit, complete, delete, categorize, filter) using the new example
- **SC-008**: Hot-reload mode (`gravity dev`) works with all new widgets without requiring app restart
- **SC-009**: Validation command (`gravity check`) successfully validates XML files using all new widgets
- **SC-010**: All existing tests continue to pass without modification
- **SC-011**: New widgets add less than 15% to compile time for gravity-iced crate
- **SC-012**: Widget rendering performance remains under 50ms for typical todo app UI (50-100 widgets)

---

## Assumptions

- Iced 0.14+ provides native support for all required widgets (Combobox, Pick_list, Canvas, etc.)
- Existing `GravityWidgetBuilder` architecture can accommodate new widgets without major refactoring
- Existing binding evaluation and handler registry systems work for new widget event types
- Canvas widget will require custom rendering callbacks in Rust code (cannot be fully declarative in XML)
- Float widget positioning can be achieved using Iced's existing layout primitives
- Grid widget can be implemented using Iced's flexbox or custom layout logic
- Todo-app example can fit within 300-400 lines of Rust code while demonstrating all features
- Modern styling can be achieved using existing Gravity theming and style system
- Developers accept that Canvas requires Rust code for custom drawing (not pure XML declarative)

---

## Clarifications

None at this time. The requirements are clear and the scope is well-defined.
