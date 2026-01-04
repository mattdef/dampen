# Tasks: Advanced Widgets for Modern Todo App

**Input**: Design documents from `/specs/004-advanced-widgets-todo/`  
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/xml-schema.md

**Tests**: Tests will be written following TDD methodology as specified in the Constitution (Principle V)

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

---

## Phase 1: Setup (Shared Infrastructure) ✅ COMPLETE

**Purpose**: Verify project structure and prepare for widget implementation

- [X] T001 Verify workspace structure matches plan.md requirements in /home/matt/Documents/Dev/gravity/Cargo.toml
- [X] T002 [P] Verify Iced 0.14+ features enabled (canvas, image) in /home/matt/Documents/Dev/gravity/Cargo.toml
- [X] T003 [P] Create widget showcase example directory structure at /home/matt/Documents/Dev/gravity/examples/widget-showcase/
- [X] T004 [P] Create assets directory for todo-app at /home/matt/Documents/Dev/gravity/examples/todo-app/assets/

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core IR and state management infrastructure that MUST be complete before ANY user story can be implemented

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

### Core IR Extensions

- [X] T005 Add 6 new WidgetKind variants (ComboBox, ProgressBar, Tooltip, Grid, Canvas, Float) to /home/matt/Documents/Dev/gravity/crates/gravity-core/src/ir/node.rs
- [X] T006 [P] Add attribute structures for ComboBox in /home/matt/Documents/Dev/gravity/crates/gravity-core/src/ir/node.rs
- [X] T007 [P] Add attribute structures for PickList in /home/matt/Documents/Dev/gravity/crates/gravity-core/src/ir/node.rs
- [X] T008 [P] Add attribute structures for Canvas in /home/matt/Documents/Dev/gravity/crates/gravity-core/src/ir/node.rs
- [X] T009 [P] Add attribute structures for ProgressBar in /home/matt/Documents/Dev/gravity/crates/gravity-core/src/ir/node.rs
- [X] T010 [P] Add attribute structures for Tooltip in /home/matt/Documents/Dev/gravity/crates/gravity-core/src/ir/node.rs
- [X] T011 [P] Add attribute structures for Grid in /home/matt/Documents/Dev/gravity/crates/gravity-core/src/ir/node.rs
- [X] T012 [P] Add attribute structures for Float in /home/matt/Documents/Dev/gravity/crates/gravity-core/src/ir/node.rs

### Parser Extensions

- [X] T013 Implement XML element name to WidgetKind mapping for new widgets in /home/matt/Documents/Dev/gravity/crates/gravity-core/src/parser/mod.rs
- [X] T014 [P] Implement comma-separated list parsing utility in /home/matt/Documents/Dev/gravity/crates/gravity-core/src/parser/mod.rs
- [X] T015 [P] Implement enum parsing for position/style attributes in /home/matt/Documents/Dev/gravity/crates/gravity-core/src/parser/mod.rs

### Runtime State Management

- [X] T016 Add widget_states HashMap to GravityRuntimeState in /home/matt/Documents/Dev/gravity/crates/gravity-runtime/src/state.rs
- [X] T017 Implement get_or_create_state<T>() method in /home/matt/Documents/Dev/gravity/crates/gravity-runtime/src/state.rs
- [X] T018 Implement widget ID auto-generation logic in /home/matt/Documents/Dev/gravity/crates/gravity-runtime/src/state.rs
- [X] T019 Add state serialization/deserialization for hot-reload in /home/matt/Documents/Dev/gravity/crates/gravity-runtime/src/state.rs

### Builder Infrastructure

- [X] T020 Add widget state access method to GravityWidgetBuilder in /home/matt/Documents/Dev/gravity/crates/gravity-iced/src/builder.rs
- [X] T021 [P] Add type conversion utilities for ProgressBarStyle in /home/matt/Documents/Dev/gravity/crates/gravity-iced/src/convert.rs
- [X] T022 [P] Add type conversion utilities for TooltipPosition in /home/matt/Documents/Dev/gravity/crates/gravity-iced/src/convert.rs
- [X] T023 [P] Add type conversion utilities for FloatPosition in /home/matt/Documents/Dev/gravity/crates/gravity-iced/src/convert.rs

**Checkpoint**: Foundation ready - user story implementation can now begin in parallel

---

## Phase 3: User Story 1 - Basic Widget Support (Priority: P1) 🎯 MVP

**Goal**: Implement ComboBox and PickList widgets to enable dropdown selections for todo categories, tags, and filters

**Independent Test**: Create a minimal example with a Combobox for category selection and a Pick_list for filtering todos. Verify that selections trigger handlers and update the model correctly.

### Tests for User Story 1

> **NOTE: Write these tests FIRST, ensure they FAIL before implementation**

- [ ] T024 [P] [US1] Contract test for ComboBox XML parsing in /home/matt/Documents/Dev/gravity/crates/gravity-core/tests/widget_parsing_tests.rs
- [ ] T025 [P] [US1] Contract test for PickList XML parsing in /home/matt/Documents/Dev/gravity/crates/gravity-core/tests/widget_parsing_tests.rs
- [ ] T026 [P] [US1] Contract test for ComboBox with all attributes in /home/matt/Documents/Dev/gravity/crates/gravity-core/tests/widget_parsing_tests.rs
- [ ] T027 [P] [US1] Contract test for PickList with all attributes in /home/matt/Documents/Dev/gravity/crates/gravity-core/tests/widget_parsing_tests.rs
- [ ] T028 [P] [US1] Contract test for ComboBox missing required attributes in /home/matt/Documents/Dev/gravity/crates/gravity-core/tests/widget_parsing_tests.rs
- [ ] T029 [P] [US1] Contract test for PickList missing required attributes in /home/matt/Documents/Dev/gravity/crates/gravity-core/tests/widget_parsing_tests.rs
- [ ] T030 [P] [US1] Integration test for ComboBox rendering in /home/matt/Documents/Dev/gravity/crates/gravity-iced/tests/widget_rendering_tests.rs
- [ ] T031 [P] [US1] Integration test for PickList rendering in /home/matt/Documents/Dev/gravity/crates/gravity-iced/tests/widget_rendering_tests.rs
- [ ] T032 [P] [US1] Integration test for ComboBox event handling in /home/matt/Documents/Dev/gravity/crates/gravity-iced/tests/widget_rendering_tests.rs
- [ ] T033 [P] [US1] Integration test for PickList event handling in /home/matt/Documents/Dev/gravity/crates/gravity-iced/tests/widget_rendering_tests.rs

### Implementation for User Story 1

- [ ] T034 [P] [US1] Implement ComboBox attribute parsing (options, selected, placeholder, on_select) in /home/matt/Documents/Dev/gravity/crates/gravity-core/src/parser/mod.rs
- [ ] T035 [P] [US1] Implement PickList attribute parsing (options, selected, placeholder, on_select) in /home/matt/Documents/Dev/gravity/crates/gravity-core/src/parser/mod.rs
- [ ] T036 [US1] Implement ComboBox rendering with state management in /home/matt/Documents/Dev/gravity/crates/gravity-iced/src/builder.rs
- [ ] T037 [US1] Implement PickList rendering in /home/matt/Documents/Dev/gravity/crates/gravity-iced/src/builder.rs
- [ ] T038 [US1] Add ComboBox event handler connection (on_select) in /home/matt/Documents/Dev/gravity/crates/gravity-iced/src/builder.rs
- [ ] T039 [US1] Add PickList event handler connection (on_select) in /home/matt/Documents/Dev/gravity/crates/gravity-iced/src/builder.rs
- [ ] T040 [P] [US1] Create minimal ComboBox example in /home/matt/Documents/Dev/gravity/examples/widget-showcase/ui/combobox.gravity
- [ ] T041 [P] [US1] Create minimal PickList example in /home/matt/Documents/Dev/gravity/examples/widget-showcase/ui/picklist.gravity
- [ ] T042 [US1] Verify hot-reload works with ComboBox state persistence
- [ ] T043 [US1] Verify hot-reload works with PickList

**Checkpoint**: ComboBox and PickList should be fully functional and testable independently. Run all US1 tests.

---

## Phase 4: User Story 2 - Visual Enhancement Widgets (Priority: P2)

**Goal**: Implement ProgressBar, Tooltip, and verify Image widget for visual polish and professional UI

**Independent Test**: Add a progress bar showing "3 of 10 tasks completed", tooltips on action buttons explaining their purpose, and icons for task priorities. Verify visual rendering and tooltip behavior.

### Tests for User Story 2

- [ ] T044 [P] [US2] Contract test for ProgressBar XML parsing in /home/matt/Documents/Dev/gravity/crates/gravity-core/tests/widget_parsing_tests.rs
- [ ] T045 [P] [US2] Contract test for Tooltip XML parsing in /home/matt/Documents/Dev/gravity/crates/gravity-core/tests/widget_parsing_tests.rs
- [ ] T046 [P] [US2] Contract test for ProgressBar with style variants in /home/matt/Documents/Dev/gravity/crates/gravity-core/tests/widget_parsing_tests.rs
- [ ] T047 [P] [US2] Contract test for Tooltip with position variants in /home/matt/Documents/Dev/gravity/crates/gravity-core/tests/widget_parsing_tests.rs
- [ ] T048 [P] [US2] Contract test for Tooltip child count validation (must be 1) in /home/matt/Documents/Dev/gravity/crates/gravity-core/tests/widget_parsing_tests.rs
- [ ] T049 [P] [US2] Integration test for ProgressBar rendering with value clamping in /home/matt/Documents/Dev/gravity/crates/gravity-iced/tests/widget_rendering_tests.rs
- [ ] T050 [P] [US2] Integration test for Tooltip rendering and positioning in /home/matt/Documents/Dev/gravity/crates/gravity-iced/tests/widget_rendering_tests.rs
- [ ] T051 [P] [US2] Integration test for Image widget verification in /home/matt/Documents/Dev/gravity/crates/gravity-iced/tests/widget_rendering_tests.rs

### Implementation for User Story 2

- [ ] T052 [P] [US2] Implement ProgressBar attribute parsing (min, max, value, style) in /home/matt/Documents/Dev/gravity/crates/gravity-core/src/parser/mod.rs
- [ ] T053 [P] [US2] Implement Tooltip attribute parsing (message, position, delay) in /home/matt/Documents/Dev/gravity/crates/gravity-core/src/parser/mod.rs
- [ ] T054 [US2] Implement ProgressBar rendering with value clamping in /home/matt/Documents/Dev/gravity/crates/gravity-iced/src/builder.rs
- [ ] T055 [US2] Implement Tooltip rendering as wrapper widget in /home/matt/Documents/Dev/gravity/crates/gravity-iced/src/builder.rs
- [ ] T056 [US2] Verify Image widget implementation and add tests if needed in /home/matt/Documents/Dev/gravity/crates/gravity-iced/src/builder.rs
- [ ] T057 [P] [US2] Create ProgressBar example with all styles in /home/matt/Documents/Dev/gravity/examples/widget-showcase/ui/progressbar.gravity
- [ ] T058 [P] [US2] Create Tooltip example with all positions in /home/matt/Documents/Dev/gravity/examples/widget-showcase/ui/tooltip.gravity
- [ ] T059 [P] [US2] Create placeholder priority icons (low, medium, high) in /home/matt/Documents/Dev/gravity/examples/todo-app/assets/
- [ ] T060 [US2] Verify hot-reload works with ProgressBar and Tooltip

**Checkpoint**: ProgressBar, Tooltip, and Image should be fully functional. Run all US2 tests.

---

## Phase 5: User Story 3 - Advanced Layout with Grid (Priority: P2)

**Goal**: Implement Grid widget for multi-column layouts to display task properties in spreadsheet-like view

**Independent Test**: Create a grid with 5 columns displaying todo properties. Verify that items align correctly and the grid is responsive.

### Tests for User Story 3

- [ ] T061 [P] [US3] Contract test for Grid XML parsing in /home/matt/Documents/Dev/gravity/crates/gravity-core/tests/widget_parsing_tests.rs
- [ ] T062 [P] [US3] Contract test for Grid with varying child counts in /home/matt/Documents/Dev/gravity/crates/gravity-core/tests/widget_parsing_tests.rs
- [ ] T063 [P] [US3] Contract test for Grid column validation (min 1, max 20) in /home/matt/Documents/Dev/gravity/crates/gravity-core/tests/widget_parsing_tests.rs
- [ ] T064 [P] [US3] Integration test for Grid rendering and layout in /home/matt/Documents/Dev/gravity/crates/gravity-iced/tests/widget_rendering_tests.rs
- [ ] T065 [P] [US3] Integration test for Grid wrapping behavior in /home/matt/Documents/Dev/gravity/crates/gravity-iced/tests/widget_rendering_tests.rs

### Implementation for User Story 3

- [ ] T066 [US3] Implement Grid attribute parsing (columns, spacing, padding) in /home/matt/Documents/Dev/gravity/crates/gravity-core/src/parser/mod.rs
- [ ] T067 [US3] Implement Grid rendering with multi-column layout in /home/matt/Documents/Dev/gravity/crates/gravity-iced/src/builder.rs
- [ ] T068 [US3] Implement Grid child widget collection and recursion in /home/matt/Documents/Dev/gravity/crates/gravity-iced/src/builder.rs
- [ ] T069 [P] [US3] Create Grid example with 5-column task table in /home/matt/Documents/Dev/gravity/examples/widget-showcase/ui/grid.gravity
- [ ] T070 [US3] Verify hot-reload works with Grid layout changes

**Checkpoint**: Grid should render multi-column layouts correctly. Run all US3 tests.

---

## Phase 6: User Story 4 - Custom Visualizations with Canvas (Priority: P3)

**Goal**: Implement Canvas widget for custom graphics like statistics charts and calendar views

**Independent Test**: Create a simple canvas-based visualization showing a weekly calendar with colored dots indicating tasks per day. Verify that the canvas renders and responds to model updates.

### Tests for User Story 4

- [ ] T071 [P] [US4] Contract test for Canvas XML parsing in /home/matt/Documents/Dev/gravity/crates/gravity-core/tests/widget_parsing_tests.rs
- [ ] T072 [P] [US4] Contract test for Canvas size validation (min 50px, max 4000px) in /home/matt/Documents/Dev/gravity/crates/gravity-core/tests/widget_parsing_tests.rs
- [ ] T073 [P] [US4] Integration test for Canvas rendering with Program trait in /home/matt/Documents/Dev/gravity/crates/gravity-iced/tests/widget_rendering_tests.rs
- [ ] T074 [P] [US4] Integration test for Canvas click event handling in /home/matt/Documents/Dev/gravity/crates/gravity-iced/tests/widget_rendering_tests.rs

### Implementation for User Story 4

- [ ] T075 [US4] Implement Canvas attribute parsing (width, height, program, on_click) in /home/matt/Documents/Dev/gravity/crates/gravity-core/src/parser/mod.rs
- [ ] T076 [US4] Implement Canvas rendering with Program binding evaluation in /home/matt/Documents/Dev/gravity/crates/gravity-iced/src/builder.rs
- [ ] T077 [US4] Implement Canvas click event handling with coordinate passing in /home/matt/Documents/Dev/gravity/crates/gravity-iced/src/builder.rs
- [ ] T078 [P] [US4] Create simple Canvas example with drawing in /home/matt/Documents/Dev/gravity/examples/widget-showcase/ui/canvas.gravity
- [ ] T079 [P] [US4] Create Canvas Program implementation for simple chart in /home/matt/Documents/Dev/gravity/examples/widget-showcase/src/main.rs
- [ ] T080 [US4] Verify hot-reload works with Canvas program changes

**Checkpoint**: Canvas should render custom graphics correctly. Run all US4 tests.

---

## Phase 7: User Story 5 - Float for Overlay UI Elements (Priority: P3)

**Goal**: Implement Float widget for positioned overlays like floating action buttons and modal dialogs

**Independent Test**: Create a floating "Add Task" button that stays in the bottom-right corner, and a modal dialog that appears when editing a task. Verify positioning and z-index behavior.

### Tests for User Story 5

- [ ] T081 [P] [US5] Contract test for Float XML parsing in /home/matt/Documents/Dev/gravity/crates/gravity-core/tests/widget_parsing_tests.rs
- [ ] T082 [P] [US5] Contract test for Float position variants in /home/matt/Documents/Dev/gravity/crates/gravity-core/tests/widget_parsing_tests.rs
- [ ] T083 [P] [US5] Contract test for Float child count validation (must be 1) in /home/matt/Documents/Dev/gravity/crates/gravity-core/tests/widget_parsing_tests.rs
- [ ] T084 [P] [US5] Integration test for Float positioning in /home/matt/Documents/Dev/gravity/crates/gravity-iced/tests/widget_rendering_tests.rs
- [ ] T085 [P] [US5] Integration test for Float z-index layering in /home/matt/Documents/Dev/gravity/crates/gravity-iced/tests/widget_rendering_tests.rs

### Implementation for User Story 5

- [ ] T086 [US5] Verify Float widget API in Iced (may pivot to pin widget) by testing in /home/matt/Documents/Dev/gravity/crates/gravity-iced/src/builder.rs
- [ ] T087 [US5] Implement Float attribute parsing (position, offset_x, offset_y, z_index, visible) in /home/matt/Documents/Dev/gravity/crates/gravity-core/src/parser/mod.rs
- [ ] T088 [US5] Implement Float rendering with positioning logic in /home/matt/Documents/Dev/gravity/crates/gravity-iced/src/builder.rs
- [ ] T089 [US5] Implement Float visibility binding support in /home/matt/Documents/Dev/gravity/crates/gravity-iced/src/builder.rs
- [ ] T090 [P] [US5] Create Float example with floating action button in /home/matt/Documents/Dev/gravity/examples/widget-showcase/ui/float.gravity
- [ ] T091 [US5] Verify hot-reload works with Float visibility changes

**Checkpoint**: Float should position overlay elements correctly. Run all US5 tests.

---

## Phase 8: User Story 6 - Comprehensive Modern Todo App (Priority: P1) 🎯 INTEGRATION

**Goal**: Create a complete, modern todo app example demonstrating all 8 widgets working together with attractive styling and full functionality

**Independent Test**: Run the todo-app example and verify all features work: adding tasks, categorizing with combobox, filtering with pick_list, viewing progress bar, seeing tooltips, displaying task icons, and custom visualizations.

### Todo App Data Model

- [ ] T092 [P] [US6] Define TodoItem struct in /home/matt/Documents/Dev/gravity/examples/todo-app/src/main.rs
- [ ] T093 [P] [US6] Define Priority enum in /home/matt/Documents/Dev/gravity/examples/todo-app/src/main.rs
- [ ] T094 [P] [US6] Define TodoFilter enum in /home/matt/Documents/Dev/gravity/examples/todo-app/src/main.rs
- [ ] T095 [US6] Define TodoAppModel with #[derive(UiModel)] in /home/matt/Documents/Dev/gravity/examples/todo-app/src/main.rs

### Todo App Event Handlers

- [ ] T096 [P] [US6] Implement add_item handler with #[ui_handler] in /home/matt/Documents/Dev/gravity/examples/todo-app/src/main.rs
- [ ] T097 [P] [US6] Implement toggle_item handler in /home/matt/Documents/Dev/gravity/examples/todo-app/src/main.rs
- [ ] T098 [P] [US6] Implement delete_item handler in /home/matt/Documents/Dev/gravity/examples/todo-app/src/main.rs
- [ ] T099 [P] [US6] Implement clear_all handler in /home/matt/Documents/Dev/gravity/examples/todo-app/src/main.rs
- [ ] T100 [P] [US6] Implement clear_completed handler in /home/matt/Documents/Dev/gravity/examples/todo-app/src/main.rs
- [ ] T101 [P] [US6] Implement update_category handler in /home/matt/Documents/Dev/gravity/examples/todo-app/src/main.rs
- [ ] T102 [P] [US6] Implement update_priority handler in /home/matt/Documents/Dev/gravity/examples/todo-app/src/main.rs
- [ ] T103 [P] [US6] Implement apply_filter handler in /home/matt/Documents/Dev/gravity/examples/todo-app/src/main.rs
- [ ] T104 [P] [US6] Implement toggle_dark_mode handler in /home/matt/Documents/Dev/gravity/examples/todo-app/src/main.rs
- [ ] T105 [US6] Implement update_counts helper method in /home/matt/Documents/Dev/gravity/examples/todo-app/src/main.rs

### Todo App Canvas Visualization

- [ ] T106 [US6] Implement StatisticsChart struct with canvas::Program trait in /home/matt/Documents/Dev/gravity/examples/todo-app/src/main.rs
- [ ] T107 [US6] Implement StatisticsChart::draw() for 7-day completion trend in /home/matt/Documents/Dev/gravity/examples/todo-app/src/main.rs
- [ ] T108 [US6] Add completion_history data to TodoAppModel in /home/matt/Documents/Dev/gravity/examples/todo-app/src/main.rs

### Todo App UI Layout

- [ ] T109 [US6] Create header section with title and dark mode toggle in /home/matt/Documents/Dev/gravity/examples/todo-app/ui/main.gravity
- [ ] T110 [US6] Create statistics section with ProgressBar and Canvas in /home/matt/Documents/Dev/gravity/examples/todo-app/ui/main.gravity
- [ ] T111 [US6] Create add task form with ComboBox and PickList in /home/matt/Documents/Dev/gravity/examples/todo-app/ui/main.gravity
- [ ] T112 [US6] Create task list with Grid layout in /home/matt/Documents/Dev/gravity/examples/todo-app/ui/main.gravity
- [ ] T113 [US6] Add Tooltips to all action buttons in /home/matt/Documents/Dev/gravity/examples/todo-app/ui/main.gravity
- [ ] T114 [US6] Add Image widgets for priority icons in /home/matt/Documents/Dev/gravity/examples/todo-app/ui/main.gravity
- [ ] T115 [US6] Add Float widget for floating action button in /home/matt/Documents/Dev/gravity/examples/todo-app/ui/main.gravity
- [ ] T116 [US6] Apply modern styling (spacing, padding, colors) in /home/matt/Documents/Dev/gravity/examples/todo-app/ui/main.gravity

### Todo App Integration & Testing

- [ ] T117 [US6] Register all handlers in HandlerRegistry in /home/matt/Documents/Dev/gravity/examples/todo-app/src/main.rs
- [ ] T118 [US6] Implement update() function with handler dispatch in /home/matt/Documents/Dev/gravity/examples/todo-app/src/main.rs
- [ ] T119 [US6] Implement view() function using GravityWidgetBuilder in /home/matt/Documents/Dev/gravity/examples/todo-app/src/main.rs
- [ ] T120 [US6] Test all CRUD operations (add, toggle, delete, clear) manually
- [ ] T121 [US6] Test category filtering with PickList manually
- [ ] T122 [US6] Test priority assignment with ComboBox manually
- [ ] T123 [US6] Test progress bar updates with task completion manually
- [ ] T124 [US6] Test statistics chart visualization manually
- [ ] T125 [US6] Test tooltips on all action buttons manually
- [ ] T126 [US6] Test hot-reload with all widgets in todo-app
- [ ] T127 [US6] Test dark mode toggle affecting all widgets

**Checkpoint**: Todo-app should be fully functional with all 8 widgets. Complete end-to-end testing.

---

## Phase 9: Polish & Cross-Cutting Concerns

**Purpose**: Final improvements affecting multiple user stories and documentation

### CLI Verification

- [ ] T128 [P] Verify gravity check validates all new widgets in /home/matt/Documents/Dev/gravity/crates/gravity-cli/src/commands/check.rs
- [ ] T129 [P] Verify gravity dev hot-reload works with all new widgets in /home/matt/Documents/Dev/gravity/crates/gravity-cli/src/commands/dev.rs

### Performance & Optimization

- [ ] T130 Test rendering performance for 50-100 widget UI (target < 50ms)
- [ ] T131 Test hot-reload latency (target < 500ms)
- [ ] T132 Optimize Canvas caching for static visualizations
- [ ] T133 Profile and optimize ComboBox state management

### Documentation

- [ ] T134 [P] Update AGENTS.md with new widget types and technologies in /home/matt/Documents/Dev/gravity/AGENTS.md
- [ ] T135 [P] Update examples/README.md with todo-app features in /home/matt/Documents/Dev/gravity/examples/README.md
- [ ] T136 [P] Create todo-app README with feature documentation in /home/matt/Documents/Dev/gravity/examples/todo-app/README.md
- [ ] T137 [P] Create widget-showcase README with usage examples in /home/matt/Documents/Dev/gravity/examples/widget-showcase/README.md

### Code Quality

- [ ] T138 Run cargo clippy --workspace -- -D warnings and fix all warnings
- [ ] T139 Run cargo fmt --all -- --check and format all code
- [ ] T140 Run cargo test --workspace and ensure all tests pass
- [ ] T141 Verify no breaking changes to existing widgets

### Property-Based Testing

- [ ] T142 [P] Add proptest for ComboBox search filtering in /home/matt/Documents/Dev/gravity/crates/gravity-iced/tests/widget_rendering_tests.rs
- [ ] T143 [P] Add proptest for Grid layout with varying children in /home/matt/Documents/Dev/gravity/crates/gravity-iced/tests/widget_rendering_tests.rs
- [ ] T144 [P] Add proptest for ProgressBar value clamping in /home/matt/Documents/Dev/gravity/crates/gravity-iced/tests/widget_rendering_tests.rs

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories
- **User Stories (Phase 3-8)**: All depend on Foundational phase completion
  - User stories CAN proceed in parallel (if staffed)
  - OR sequentially in priority order: US1 (P1) → US6 (P1) → US2 (P2) → US3 (P2) → US4 (P3) → US5 (P3)
- **Polish (Phase 9)**: Depends on all user stories being complete

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Foundational - No dependencies on other stories
- **User Story 2 (P2)**: Can start after Foundational - No dependencies on other stories
- **User Story 3 (P2)**: Can start after Foundational - No dependencies on other stories
- **User Story 4 (P3)**: Can start after Foundational - No dependencies on other stories
- **User Story 5 (P3)**: Can start after Foundational - No dependencies on other stories
- **User Story 6 (P1 Integration)**: Depends on US1-US5 completion for full functionality, but can start with subset

### Within Each User Story

- Tests MUST be written and FAIL before implementation (TDD)
- Parsing before rendering
- State management (if needed) before rendering
- Core implementation before examples
- Examples before verification
- Story complete before moving to next priority

### Parallel Opportunities

**Setup Phase**:
```bash
# Can run simultaneously:
T002 [P] Verify Iced features
T003 [P] Create widget-showcase directory
T004 [P] Create assets directory
```

**Foundational Phase**:
```bash
# All attribute structures (T006-T012) can run in parallel
# Parser utilities (T014-T015) can run in parallel
# Type conversions (T021-T023) can run in parallel
```

**User Story 1**:
```bash
# All contract tests (T024-T029) can run in parallel
# All integration tests (T030-T033) can run in parallel
# Attribute parsing (T034-T035) can run in parallel
# Examples (T040-T041) can run in parallel
```

**User Story 2**:
```bash
# All contract tests (T044-T048) can run in parallel
# All integration tests (T049-T051) can run in parallel
# Attribute parsing (T052-T053) can run in parallel
# Examples (T057-T059) can run in parallel
```

**User Story 6** (Todo App):
```bash
# Data models (T092-T094) can run in parallel
# Event handlers (T096-T104) can run in parallel after T095
```

**Polish Phase**:
```bash
# CLI verification (T128-T129) can run in parallel
# Documentation (T134-T137) can run in parallel
# Property tests (T142-T144) can run in parallel
```

---

## Parallel Example: User Story 1

```bash
# Launch all contract tests together (write first):
Task: "[US1] Contract test for ComboBox XML parsing"
Task: "[US1] Contract test for PickList XML parsing"
Task: "[US1] Contract test for ComboBox with all attributes"
Task: "[US1] Contract test for PickList with all attributes"
Task: "[US1] Contract test for ComboBox missing required attributes"
Task: "[US1] Contract test for PickList missing required attributes"

# After tests fail, launch implementation in parallel:
Task: "[US1] Implement ComboBox attribute parsing"
Task: "[US1] Implement PickList attribute parsing"

# Then sequential:
Task: "[US1] Implement ComboBox rendering" (needs T034, T020)
Task: "[US1] Implement PickList rendering" (needs T035)
```

---

## Implementation Strategy

### MVP First (User Stories 1 + 6 Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational (CRITICAL - blocks all stories)
3. Complete Phase 3: User Story 1 (ComboBox + PickList)
4. Complete Phase 8: User Story 6 (Basic Todo App with US1 widgets)
5. **STOP and VALIDATE**: Test independently
6. Deploy/demo if ready

**Why this MVP**: US1 + US6 gives a functional todo app with dropdowns. This is the minimum viable product that demonstrates the framework's value.

### Incremental Delivery

1. Complete Setup + Foundational → Foundation ready ✅
2. Add User Story 1 → Test independently → Basic dropdowns work ✅
3. Add User Story 6 (basic) → Test independently → Todo app works with dropdowns ✅ **MVP!**
4. Add User Story 2 → Test independently → Visual polish added ✅
5. Add User Story 3 → Test independently → Grid layout added ✅
6. Add User Story 4 → Test independently → Custom visualizations added ✅
7. Add User Story 5 → Test independently → Floating elements added ✅
8. Update User Story 6 → Test all widgets together → Complete todo app ✅

Each increment adds value without breaking previous stories.

### Parallel Team Strategy

With multiple developers:

1. **Week 1**: Team completes Setup + Foundational together
2. **Week 2**: Once Foundational is done:
   - Developer A: User Story 1 (ComboBox, PickList)
   - Developer B: User Story 2 (ProgressBar, Tooltip, Image)
   - Developer C: User Story 3 (Grid)
3. **Week 3**:
   - Developer A: User Story 4 (Canvas)
   - Developer B: User Story 5 (Float)
   - Developer C: User Story 6 (Todo App integration)
4. **Week 4**: Polish phase together

---

## Task Summary

**Total Tasks**: 144

**By Phase**:
- Phase 1 (Setup): 4 tasks
- Phase 2 (Foundational): 19 tasks
- Phase 3 (US1 - Basic Widgets): 20 tasks
- Phase 4 (US2 - Visual Enhancement): 17 tasks
- Phase 5 (US3 - Grid): 10 tasks
- Phase 6 (US4 - Canvas): 10 tasks
- Phase 7 (US5 - Float): 11 tasks
- Phase 8 (US6 - Todo App): 36 tasks
- Phase 9 (Polish): 17 tasks

**By Type**:
- Tests: 43 tasks (TDD approach)
- Implementation: 84 tasks
- Documentation: 4 tasks
- Verification: 13 tasks

**Parallel Opportunities**:
- 67 tasks marked [P] can run in parallel within their phase
- All 6 user stories can be developed in parallel after Foundational phase

**Estimated Timeline**:
- Sequential (1 developer): 3-4 weeks
- Parallel (3 developers): 2-3 weeks
- MVP only (US1 + US6): 1-2 weeks

---

## Notes

- [P] tasks = different files, no dependencies within phase
- [Story] label maps task to specific user story for traceability
- Each user story should be independently completable and testable
- Write tests FIRST, ensure they FAIL before implementing (TDD)
- Commit after each task or logical group
- Stop at any checkpoint to validate story independently
- Follow Constitution Principle V: Test-First Development
- All file paths are absolute from repository root
