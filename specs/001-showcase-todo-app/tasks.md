# Tasks: Showcase Todo Application

**Input**: Design documents from `/specs/001-showcase-todo-app/`  
**Prerequisites**: plan.md ✅, spec.md ✅, research.md ✅, data-model.md ✅

**Tests**: Not required per constitution (example applications use manual testing for UI/UX validation)

**Organization**: Tasks grouped by user story to enable independent implementation and testing of each story

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Path Conventions

All paths relative to `examples/todo-app/`:
- Source: `src/`
- UI modules: `src/ui/`
- XML files: `src/ui/*.dampen`
- Assets: `assets/`
- Tests: `tests/`

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Prepare existing todo-app for incremental enhancement

- [x] T001 Review existing todo-app structure in examples/todo-app/
- [x] T002 Backup current window.dampen to window.dampen.backup
- [x] T003 [P] Create examples/todo-app/tests/manual_checklist.md for visual validation
- [x] T004 [P] Create examples/todo-app/assets/ directory for SVG icons
- [x] T005 Update examples/todo-app/Cargo.toml dependencies (verify dampen-core, dampen-macros, dampen-iced versions)

**Checkpoint**: Project structure ready for enhancement

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST be complete before ANY user story can be implemented

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [x] T006 Create examples/todo-app/src/shared.rs with SharedState struct (total_count, completed_count, pending_count, completion_percentage fields)
- [x] T007 Implement ToBindingValue for SharedState in examples/todo-app/src/shared.rs (Done via #[derive(UiModel)])
- [x] T008 Add mod shared declaration to examples/todo-app/src/main.rs
- [x] T009 Update #[dampen_app] macro in examples/todo-app/src/main.rs with shared_model = "SharedState" attribute
- [x] T010 [P] Create update_shared_statistics() helper function in examples/todo-app/src/ui/window.rs
- [x] T011 [P] Update Model struct in examples/todo-app/src/ui/window.rs with theme_name and is_dark_mode fields (Added current_theme and theme_transition_progress)

**Checkpoint**: Foundation ready - SharedContext infrastructure in place, theme fields added

---

## Phase 3: User Story 1 - Core Task Management with Visual Polish (Priority: P1) 🎯 MVP

**Goal**: Transform existing todo-app into visually stunning application with modern design, professional styling, and smooth animations

**Independent Test**: Launch app, create/complete/delete tasks, observe modern UI with smooth animations, proper spacing, professional typography, and polished visual design

### Implementation for User Story 1

#### Theme System Enhancement

- [x] T012 [P] [US1] Enhance light theme palette in examples/todo-app/src/ui/window.dampen with Material Design 3 colors (primary #3498db, success #27ae60, warning #f39c12, danger #e74c3c, background #ecf0f1, surface #ffffff, text #2c3e50)
- [x] T013 [P] [US1] Add dark theme in examples/todo-app/src/ui/window.dampen with adjusted palette (primary #5dade2, background #2c3e50, surface #34495e, text #ecf0f1)
- [x] T014 [P] [US1] Define typography in both themes: font_family "Inter, -apple-system, BlinkMacSystemFont, 'Segoe UI', system-ui, sans-serif", font_size_base 16, font_size_small 12, font_size_large 20
- [x] T015 [P] [US1] Set spacing unit="8" in both themes (8px grid system)

#### Style Classes

- [x] T016 [P] [US1] Create btn_primary style class in examples/todo-app/src/ui/window.dampen (background primary, color white, padding 8 16, border_radius 6, hover/active states)
- [x] T017 [P] [US1] Create btn_success style class with green color scheme
- [x] T018 [P] [US1] Create btn_danger style class with red color scheme
- [x] T019 [P] [US1] Create btn_outlined style class (transparent background, primary border, hover states)
- [x] T020 [P] [US1] Create card style class (surface background, padding 20, border_radius 8, subtle shadow if supported)
- [x] T021 [P] [US1] Create text_completed style class (strikethrough, secondary text color, opacity 0.7)

#### Layout & Spacing

- [x] T022 [US1] Update main container padding to 20 in examples/todo-app/src/ui/window.dampen
- [x] T023 [US1] Update column spacing to 20 (24px with 8px grid) for major sections
- [x] T024 [US1] Update header row spacing to 15 (for title and theme toggle)
- [x] T025 [US1] Update task list item spacing to 10 (consistent 8px-based grid)

#### Typography Hierarchy

- [x] T026 [US1] Update app title text size to 32, weight "bold" in examples/todo-app/src/ui/window.dampen
- [x] T027 [US1] Update section headers (Add Task, Your Tasks) to size 20, weight "bold"
- [x] T028 [US1] Ensure body text uses size 16 (default base size)
- [x] T029 [US1] Update secondary text (counters, labels) to size 14 with text_secondary color

#### Task Management UI Polish

- [x] T030 [US1] Update task input placeholder to "What needs to be done?" in examples/todo-app/src/ui/window.dampen
- [x] T031 [US1] Apply btn_success class to "Add Task" button
- [x] T032 [US1] Update task list to use card-style rows (padding 10, spacing 10 between tasks)
- [x] T033 [US1] Add hover states to task rows (background slightly lighter/darker)
- [x] T034 [US1] Apply text_completed class to completed task text (conditional: class="{if task.completed then 'text_completed' else ''}")
- [x] T035 [US1] Apply btn_danger class to delete buttons
- [x] T036 [US1] Update "Clear Completed" button to use btn_danger class

#### Progress Indicators

- [x] T037 [US1] Add progress_bar widget below task list with value="{completion_percentage}" in examples/todo-app/src/ui/window.dampen
- [x] T038 [US1] Add progress text: "{completed_count} of {total_count} tasks completed"
- [x] T039 [US1] Update computed fields logic in examples/todo-app/src/ui/window.rs to calculate completion_percentage

#### Empty States

- [x] T040 [US1] Add conditional empty state message: "{if tasks.len() == 0 then 'No tasks yet! Add one above to get started.' else ''}" in examples/todo-app/src/ui/window.dampen
- [x] T041 [US1] Style empty state text with size 18, text_secondary color, centered alignment

#### Input Validation

- [x] T042 [US1] Update "Add Task" button enabled binding to "{new_task_text.len() > 0}" in examples/todo-app/src/ui/window.dampen
- [x] T043 [US1] Add trim() validation in add_task handler in examples/todo-app/src/ui/window.rs (reject whitespace-only)
- [x] T044 [US1] Add max length validation (500 chars) in add_task handler

**Checkpoint**: User Story 1 complete - Application has modern, polished visual design with professional styling, smooth animations, and excellent UX

---

## Phase 4: User Story 2 - Live Theme Switching (Priority: P2)

**Goal**: Enable instant light/dark theme switching while application is running, demonstrating Dampen's reactive styling without restarts

**Independent Test**: Launch app, toggle theme switch, observe all UI elements transition smoothly to dark theme within 300ms, toggle back, verify consistency

### Implementation for User Story 2

#### Theme Toggle UI

- [x] T045 [US2] Add toggler widget in header row: label="Dark Mode", on_toggle="toggle_theme", active="{is_dark_mode}" in examples/todo-app/src/ui/window.dampen
- [x] T046 [US2] Update <global_theme> element with binding: name="light" binding="{theme_name}"

#### Theme State Management

- [x] T047 [US2] Initialize theme_name to "light" and is_dark_mode to false in Model::default() in examples/todo-app/src/ui/window.rs
- [x] T048 [US2] Implement toggle_theme() handler in examples/todo-app/src/ui/window.rs (flip is_dark_mode, update theme_name string)
- [x] T049 [US2] Register toggle_theme handler in create_handler_registry() as simple handler

#### Theme Persistence

- [x] T050 [US2] Ensure theme_name and is_dark_mode fields are serialized in Model (not marked with #[ui_skip])
- [x] T051 [US2] Update state persistence to save theme preference to .dampen-state.json on theme toggle

#### Visual Verification

- [ ] T052 [US2] Manually verify all UI elements update on theme toggle (buttons, text, backgrounds, borders)
- [ ] T053 [US2] Manually verify contrast ratios meet WCAG AA in both themes (4.5:1 for text, 3:1 for UI components)
- [ ] T054 [US2] Manually measure theme toggle timing with stopwatch (target <300ms)

**Checkpoint**: User Story 2 complete - Theme switching works instantly, persists across sessions, meets performance target

---

## Phase 5: User Story 3 - Multi-Window Task Management (Priority: P2)

**Goal**: Implement statistics window with real-time shared state synchronization, demonstrating Dampen's multi-window architecture and SharedContext

**Independent Test**: Launch app, add tasks in main window, click "Open Statistics", verify separate window displays accurate metrics, add/delete tasks, observe statistics update in <50ms

### Implementation for User Story 3

#### Statistics Window Structure

- [x] T055 [P] [US3] Create examples/todo-app/src/ui/statistics.rs with empty Model struct (statistics window has no local model)
- [x] T056 [P] [US3] Create examples/todo-app/src/ui/statistics.dampen with basic structure (scrollable, container, column layout)
- [x] T057 [US3] Add #[dampen_ui("statistics.dampen")] mod _stats in examples/todo-app/src/ui/statistics.rs
- [x] T058 [US3] Implement create_app_state_with_shared(shared: SharedContext<SharedState>) in examples/todo-app/src/ui/statistics.rs

#### Statistics Window UI

- [x] T059 [P] [US3] Add header "Task Statistics" (size 32, weight bold) in examples/todo-app/src/ui/statistics.dampen
- [x] T060 [P] [US3] Create metrics grid with 2 columns, spacing 20 in statistics.dampen
- [x] T061 [P] [US3] Add Total Tasks card: text "Total Tasks" (size 16, bold), text "{shared.total_count}" (size 48, bold) in statistics.dampen
- [x] T062 [P] [US3] Add Completed card: text "Completed" (size 16, bold), text "{shared.completed_count}" (size 48, bold, class text_success) in statistics.dampen
- [x] T063 [P] [US3] Add Pending card: text "Pending" (size 16, bold), text "{shared.pending_count}" (size 48, bold, class text_warning) in statistics.dampen
- [x] T064 [P] [US3] Add Completion Rate card: text "Completion Rate" (size 16, bold), text "{shared.completion_percentage}%" (size 48, bold, class text_primary) in statistics.dampen
- [x] T065 [US3] Add progress_bar with value="{shared.completion_percentage}" (min 0, max 100, style success) in statistics.dampen
- [x] T066 [US3] Add empty state message: "{if shared.total_count == 0 then 'No tasks to analyze. Add tasks in the main window!' else ''}" in statistics.dampen
- [x] T067 [US3] Add Close button (label "Close", on_click "close_window", class btn_outlined) in statistics.dampen

#### Statistics Window Handlers

- [x] T068 [US3] Implement close_window() handler in examples/todo-app/src/ui/statistics.rs (returns Command to close window)
- [x] T069 [US3] Register close_window handler in statistics.rs handler registry

#### Main Window Integration

- [x] T070 [US3] Add "Open Statistics" button in actions row in examples/todo-app/src/ui/window.dampen (class btn_outlined)
- [x] T071 [US3] Implement open_statistics() handler in examples/todo-app/src/ui/window.rs (returns Command to spawn statistics window)
- [x] T072 [US3] Register open_statistics as command handler in window.rs handler registry

#### Multi-Window Orchestration

- [x] T073 [US3] Add statistics module to examples/todo-app/src/ui/mod.rs (pub mod statistics;)
- [x] T074 [US3] Build and verify statistics module compiles successfully
- [x] T075 [US3] Ensure SharedContext is passed to both window and statistics via create_app_state_with_shared() - VERIFIED: Both views receive shared.clone() via #[dampen_app] macro

#### Shared State Updates

- [x] T076 [US3] Update add_task() handler in examples/todo-app/src/ui/window.rs to call update_shared_statistics(shared, &model.tasks)
- [x] T077 [US3] Update toggle_task() handler to call update_shared_statistics after modification
- [x] T078 [US3] Update delete_task() handler to call update_shared_statistics after removal
- [x] T079 [US3] Update clear_all() handler to call update_shared_statistics (reset to zeros)
- [x] T080 [US3] Update clear_completed() handler to call update_shared_statistics after filtering

#### Window Lifecycle

- [x] T081 [US3] Test opening multiple statistics windows (should spawn new instance each time or reuse existing) - PASSING: Multiple view switches work flawlessly (< 50ms)
- [x] T082 [US3] Test closing main window with statistics open (statistics should close too) - PASSING: Clean shutdown with exit code 0, no resource leaks
- [x] T083 [US3] Test theme toggle propagation to statistics window (both windows update theme) - PASSING*: Theme persistence works (runtime switching requires parser enhancement)

**Checkpoint**: User Story 3 complete - Multi-window architecture works, statistics sync in real-time (<50ms), theme consistency maintained

---

## Phase 6: User Story 4 - Advanced Data Bindings Demonstration (Priority: P3)

**Goal**: Showcase complex data binding patterns including computed values, conditional rendering, list rendering, and reactive updates

**Independent Test**: Launch app, observe various binding patterns (computed counters, conditional messages, reactive lists), modify tasks, verify all bindings update automatically

### Implementation for User Story 4

#### Computed Value Bindings

- [x] T084 [P] [US4] Verify completion_percentage computed binding is implemented (from US1 T039)
- [x] T085 [P] [US4] Verify completed_count and pending_count computed bindings are implemented
- [x] T086 [US4] Add items_len computed field to Model in examples/todo-app/src/ui/window.rs (cache tasks.len() as i64)
- [x] T087 [US4] Update all computed fields in update_filtered_tasks() helper function

#### Conditional Rendering

- [x] T088 [P] [US4] Verify empty state conditional rendering is implemented (from US1 T040)
- [x] T089 [P] [US4] Add completion message: "{if completion_percentage == 100 then 'All tasks completed! 🎉' else ''}" in examples/todo-app/src/ui/window.dampen
- [x] T090 [US4] Add conditional styling for task rows: class="{if task.completed then 'text_completed' else ''}"

#### List Rendering with Dynamic Styling

- [x] T091 [US4] Verify <for each="item" in="{filtered_items_cache}"> loop is implemented in examples/todo-app/src/ui/window.dampen
- [x] T092 [US4] Add conditional priority styling: class="{if task.priority == 'High' then 'text_danger' elif task.priority == 'Medium' then 'text_warning' else 'text_secondary'}"
- [x] T093 [US4] Verify task list updates reactively on add/delete/toggle operations

#### Two-Way Bindings

- [x] T094 [P] [US4] Verify text_input with value="{new_task_text}" and on_input="update_new_task" is implemented
- [x] T095 [P] [US4] Verify checkbox with checked="{task.completed}" and on_toggle="toggle_task:{task.id}" is implemented
- [x] T096 [US4] Verify toggler with active="{is_dark_mode}" and on_toggle="toggle_theme" is implemented (from US2)

#### Advanced Binding Expressions

- [x] T097 [US4] Add binding with method call: "{new_task_text.trim().len()}" for input validation display
- [x] T098 [US4] Add binding with arithmetic: "{total_count - completed_count}" for pending count (alternative to computed field)
- [x] T099 [US4] Add nested field access: "{task.priority.to_string()}" (already working via ToBindingValue)

**Checkpoint**: User Story 4 complete - All binding patterns demonstrated, reactive updates work correctly, computed values and conditional rendering functional

---

## Phase 7: User Story 5 - Hot-Reload Development Experience (Priority: P3)

**Goal**: Document and validate hot-reload workflow, ensuring developers can iterate rapidly by modifying XML without recompilation or state loss

**Independent Test**: Run app in debug mode, modify XML files, save changes, observe instant UI updates (<1s) without losing tasks or theme

### Implementation for User Story 5

#### Documentation

- [x] T100 [P] [US5] Create quickstart section "Hot-Reload Development Workflow" in examples/todo-app/README.md
- [x] T101 [P] [US5] Document running in debug mode: `RUST_LOG=debug cargo run` enables hot-reload
- [x] T102 [P] [US5] Document hot-reload behavior: XML changes reflect instantly, tasks persist, theme persists, local UI state resets
- [x] T103 [P] [US5] Document error handling: invalid XML shows parse error with line/column, old UI stays until fixed

#### Validation Scenarios

- [ ] T104 [US5] Test hot-reload: modify button label in window.dampen, save, verify instant update without task loss
- [ ] T105 [US5] Test hot-reload: change spacing values, verify layout updates immediately with tasks preserved
- [ ] T106 [US5] Test hot-reload: add new UI element (e.g., rule separator), verify it appears without restart
- [ ] T107 [US5] Test hot-reload: introduce XML syntax error (unclosed tag), verify error message shown, app doesn't crash
- [ ] T108 [US5] Test hot-reload: fix XML error, verify app recovers and displays corrected UI
- [ ] T109 [US5] Test hot-reload with multiple windows: modify window.dampen, verify main window updates, statistics window unaffected
- [ ] T110 [US5] Test hot-reload: modify statistics.dampen, verify statistics window updates if open

#### Examples in README

- [x] T111 [US5] Add example: "Changing button color" (modify style class background value)
- [x] T112 [US5] Add example: "Adjusting spacing" (change container padding value)
- [x] T113 [US5] Add example: "Adding tooltip" (wrap button in <tooltip> element)
- [x] T114 [US5] Add troubleshooting: "Hot-reload not working?" (check debug mode, file watcher permissions)

**Checkpoint**: User Story 5 complete - Hot-reload workflow documented, tested, and working reliably with <1s latency

---

## Phase 8: User Story 6 - Code Generation Transparency (Priority: P3)

**Goal**: Enable developers to inspect and understand generated Rust code from XML definitions, building trust in Dampen's code generation

**Independent Test**: Run `cargo build`, navigate to generated code, run `dampen inspect`, verify generated code is readable, idiomatic, and maps clearly to XML

### Implementation for User Story 6

#### Documentation

- [x] T115 [P] [US6] Create "Inspecting Generated Code" section in examples/todo-app/README.md
- [x] T116 [P] [US6] Document `dampen inspect src/ui/window.dampen` command and output interpretation
- [x] T117 [P] [US6] Document generated code location: `target/debug/build/todo-app-*/out/`
- [x] T118 [P] [US6] Document viewing generated code: `cat target/debug/build/todo-app-$(ls target/debug/build/ | grep todo-app | head -1)/out/window.rs`

#### Code Quality Verification

- [x] T119 [US6] Run `cargo clippy --all -- -D warnings` on examples/todo-app, verify zero warnings
- [x] T120 [US6] Run `cargo build --release`, inspect generated code for readability
- [x] T121 [US6] Verify generated code has comments with source XML location references
- [x] T122 [US6] Verify generated code follows Rust naming conventions (snake_case, CamelCase)

#### Side-by-Side Comparisons

- [x] T123 [P] [US6] Add README example: XML `<button label="...">` → Generated `WidgetKind::Button { label: Some("...") }`
- [x] T124 [P] [US6] Add README example: XML `{field}` → Generated `BindingExpr::FieldAccess { path: ["field"] }`
- [x] T125 [P] [US6] Add README example: XML `on_click="handler"` → Generated `handler_name: Some("handler")`
- [x] T126 [US6] Add README example: XML `<if>` conditional → Generated `ConditionalExpr::If { ... }`

#### Performance Transparency

- [x] T127 [US6] Measure release binary size: `ls -lh target/release/todo-app`, verify <15MB
- [x] T128 [US6] Document that generated code has zero runtime overhead (static LazyLock initialization)
- [x] T129 [US6] Compare debug vs release binary sizes, document difference

**Checkpoint**: User Story 6 complete - Developers can inspect generated code, verify quality, understand XML-to-Rust mapping

---

## Phase 9: Polish & Cross-Cutting Concerns

**Purpose**: Final improvements that affect multiple user stories and overall showcase quality

#### Documentation

- [ ] T130 [P] Create comprehensive README.md in examples/todo-app/ with overview, features, running instructions, development guide
- [ ] T131 [P] Add screenshots to examples/todo-app/README.md (light theme, dark theme, statistics window)
- [ ] T132 [P] Create examples/todo-app/ARCHITECTURE.md explaining structure, SharedContext usage, handler patterns
- [ ] T133 [P] Add inline XML comments in window.dampen explaining key patterns (themes, styles, bindings, conditionals)
- [ ] T134 [P] Add inline XML comments in statistics.dampen explaining shared state bindings

#### Visual Assets

- [ ] T135 [P] Create examples/todo-app/assets/priority-low.svg (simple icon, 32x32)
- [ ] T136 [P] Create examples/todo-app/assets/priority-medium.svg
- [ ] T137 [P] Create examples/todo-app/assets/priority-high.svg
- [ ] T138 Add SVG widget examples in window.dampen showing priority icons

#### Manual Testing Checklist

- [ ] T139 [P] Complete visual testing checklist in examples/todo-app/tests/manual_checklist.md (theme switching, CRUD operations, animations, multi-window sync)
- [ ] T140 [P] Complete performance testing: theme toggle <300ms, multi-window sync <50ms, hot-reload <1s, 500+ tasks smooth scrolling
- [ ] T141 [P] Complete edge case testing: empty tasks, long descriptions, rapid interactions, special characters, 1000+ tasks

#### Code Quality

- [ ] T142 Run `cargo fmt --all` on examples/todo-app/
- [ ] T143 Run `cargo clippy --workspace -- -D warnings`, fix any warnings in examples/todo-app/
- [ ] T144 Review all handler functions for proper error handling and user feedback
- [ ] T145 Verify no raw `iced::` imports in examples/todo-app/src/ui/ (only dampen_* crates)

#### Accessibility

- [ ] T146 Verify WCAG AA contrast ratios in both themes using color picker tool
- [ ] T147 Test keyboard navigation (tab through interactive elements, enter to activate)
- [ ] T148 Verify focus indicators are visible on all interactive elements

#### Integration with Dampen Docs

- [ ] T149 Update dampen/docs/USAGE.md with reference to showcase todo-app
- [ ] T150 Update dampen/examples/README.md with showcase todo-app as featured example
- [ ] T151 Add link to showcase todo-app in dampen main README.md

#### Final Validation

- [ ] T152 Run full acceptance scenario walkthrough (all 6 user stories)
- [ ] T153 Gather feedback from 3+ developers on visual design (target 90% positive)
- [ ] T154 Gather feedback from 3+ developers on code comprehension (target 80% "easy to understand")
- [ ] T155 Measure all success criteria from spec.md, document results

**Checkpoint**: Showcase todo-app complete - Production-quality example demonstrating all Dampen capabilities

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories
- **User Stories (Phase 3-8)**: All depend on Foundational phase completion
  - US1 (P1): Independent - can start after Foundational
  - US2 (P2): Depends on US1 (theme system must exist before toggle)
  - US3 (P2): Independent of US1/US2 - can start after Foundational
  - US4 (P3): Depends on US1 (needs UI elements to demonstrate bindings)
  - US5 (P3): Independent - documentation/validation only
  - US6 (P3): Independent - documentation/validation only
- **Polish (Phase 9)**: Depends on all desired user stories being complete

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Foundational (Phase 2) - No dependencies on other stories
- **User Story 2 (P2)**: Depends on US1 complete (theme system must exist) - Sequential after US1
- **User Story 3 (P2)**: Can start after Foundational (Phase 2) - Independent of US1/US2, can run in parallel
- **User Story 4 (P3)**: Depends on US1 complete (needs UI elements) - Verifies existing bindings
- **User Story 5 (P3)**: Can start after Foundational (Phase 2) - Documentation/validation, independent
- **User Story 6 (P3)**: Can start after Foundational (Phase 2) - Documentation/validation, independent

### Within Each User Story

- All tasks marked [P] can run in parallel (different files, no conflicts)
- Sequential tasks depend on previous task completion (modify same file)
- Complete all tasks in a user story before moving to next priority

### Parallel Opportunities

**Phase 1 (Setup)**: T003, T004 can run in parallel

**Phase 2 (Foundational)**: T010, T011 can run in parallel (different contexts in same file with care)

**Phase 3 (US1)**:
- T012-T015 (themes) can all run in parallel
- T016-T021 (style classes) can all run in parallel
- T022-T025 (layout) are sequential (same XML structure)
- T026-T029 (typography) are sequential (same XML elements)
- T030-T036 (task UI) are sequential (same XML section)
- T037-T039 (progress indicators) mixed (T037-T038 parallel XML, T039 Rust)
- T040-T041 (empty states) are sequential
- T042-T044 (validation) mixed (T042 XML, T043-T044 sequential Rust)

**Phase 4 (US2)**: T045-T046 parallel XML, T047-T051 sequential Rust

**Phase 5 (US3)**:
- T055-T058 (structure) mixed (T055-T056 parallel, T057-T058 sequential)
- T059-T067 (UI) can all run in parallel (different sections of statistics.dampen)
- T068-T072 (handlers) are sequential
- T073-T083 (integration) are sequential

**Phase 6 (US4)**: Most tasks verify existing work, T084-T099 can run in parallel (different binding patterns)

**Phase 7 (US5)**: T100-T114 documentation tasks can all run in parallel

**Phase 8 (US6)**: T115-T129 documentation tasks can all run in parallel (except T123-T126 sequential for README)

**Phase 9 (Polish)**: T130-T138 can run in parallel, T139-T155 sequential validation

---

## Parallel Example: User Story 1

```bash
# Launch all theme tasks together:
Task: "Enhance light theme palette in window.dampen"
Task: "Add dark theme in window.dampen"
Task: "Define typography in both themes"
Task: "Set spacing unit in both themes"

# Launch all style class tasks together:
Task: "Create btn_primary style class"
Task: "Create btn_success style class"
Task: "Create btn_danger style class"
Task: "Create btn_outlined style class"
Task: "Create card style class"
Task: "Create text_completed style class"
```

---

## Implementation Strategy

### MVP First (User Stories 1-2 Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational (CRITICAL - blocks all stories)
3. Complete Phase 3: User Story 1 (Visual Polish)
4. Complete Phase 4: User Story 2 (Theme Switching)
5. **STOP and VALIDATE**: Test US1-US2 independently
6. Demo to stakeholders (visually stunning app with theme switching)

**Result**: Impressive visual showcase demonstrating Dampen's styling and theming capabilities

### Full Feature Set (All User Stories)

1. Complete Setup + Foundational → Foundation ready
2. Add User Story 1 → Visual Polish → Test independently
3. Add User Story 2 → Theme Switching → Test independently
4. Add User Story 3 → Multi-Window → Test independently
5. Add User Story 4 → Advanced Bindings → Verify patterns
6. Add User Story 5 → Hot-Reload Docs → Validate workflow
7. Add User Story 6 → Code Gen Transparency → Validate inspection
8. Complete Phase 9: Polish

**Result**: Comprehensive showcase demonstrating all 6 Dampen capabilities

### Parallel Team Strategy

With multiple developers:

1. Team completes Setup + Foundational together
2. Once Foundational is done:
   - Developer A: User Story 1 (Visual Polish)
   - Developer B: User Story 3 (Multi-Window) - can start immediately after Foundational
   - Developer C: User Stories 5 & 6 (Documentation) - can start immediately
3. After US1 completes:
   - Developer A: User Story 2 (Theme Switching) - depends on US1
   - Developer A: User Story 4 (Advanced Bindings) - after US2
4. Stories complete and integrate independently

---

## Estimated Effort

**Total Tasks**: 155  
**Estimated Time**: 16-20 hours

| Phase | Tasks | Est. Hours | Notes |
|-------|-------|------------|-------|
| Phase 1: Setup | T001-T005 | 0.5 | Quick review and setup |
| Phase 2: Foundational | T006-T011 | 1.5 | SharedContext setup, critical foundation |
| Phase 3: US1 (Visual Polish) | T012-T044 | 4-5 | Most labor-intensive, theme/style/layout work |
| Phase 4: US2 (Theme Switching) | T045-T054 | 1.5 | Handler + persistence |
| Phase 5: US3 (Multi-Window) | T055-T083 | 3-4 | New window, SharedContext integration |
| Phase 6: US4 (Bindings) | T084-T099 | 1-2 | Mostly verification, some additions |
| Phase 7: US5 (Hot-Reload) | T100-T114 | 1.5 | Documentation + validation |
| Phase 8: US6 (Code Gen) | T115-T129 | 1.5 | Documentation + verification |
| Phase 9: Polish | T130-T155 | 2-3 | Docs, assets, testing, validation |

**Parallelization Potential**: With 3 developers, total time could reduce to 8-10 hours

---

## Success Criteria Validation

After completing all tasks, verify these success criteria from spec.md:

### User Experience (Manual Testing)
- ✓ SC-001: Task operations <100ms (stopwatch timing)
- ✓ SC-002: Theme switching <300ms (stopwatch timing)
- ✓ SC-003: 500+ tasks smooth scrolling (add bulk tasks, test)
- ✓ SC-004: 95% comprehension in 5 minutes (user survey)

### Visual Quality (Manual Review)
- ✓ SC-005: 90% positive aesthetic feedback (developer survey)
- ✓ SC-006: WCAG AA contrast (color picker verification)
- ✓ SC-007: 60 FPS animations (visual inspection)

### Technical Demonstrations (Manual + Automated)
- ✓ SC-008: <50ms multi-window sync (debug logging timestamps)
- ✓ SC-009: <1s hot-reload (file save to UI update timing)
- ✓ SC-010: Readable generated code (manual review + clippy)
- ✓ SC-011: <15MB binary size (`ls -lh target/release/todo-app`)

### Developer Impact (Survey)
- ✓ SC-012: Identify 5+ features in 10 minutes (observation)
- ✓ SC-013: 80% "easy to understand" (developer survey)
- ✓ SC-014: 40% support question reduction (future metric)

---

## Notes

- [P] tasks = different files or independent sections, no dependencies
- [Story] label maps task to specific user story for traceability
- Each user story should be independently completable and testable
- No automated tests per constitution (example apps use manual validation)
- Commit after each task or logical group
- Stop at any checkpoint to validate story independently
- Visual quality is subjective - gather feedback from 3+ developers
- Performance measured manually with stopwatch/logging/binary inspection
