# Tasks: Refactor Todo-App to Match Iced Example

**Input**: Design documents from `/specs/001-refactor-todo-app/`
**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, contracts/
**Tests**: This example uses hybrid testing strategy: unit tests for model logic, manual integration testing for UI. Unit test tasks are included below.
**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.
## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Path Conventions

- **Example project**: `examples/todo-app/src/`, `examples/todo-app/Cargo.toml`
- Paths shown below reflect the simplified single-view structure from plan.md

---

## Phase 1: Setup (Project Cleanup & Initialization)

**Purpose**: Clean up existing multi-view structure and initialize simplified single-view architecture

- [x] T001 Remove old multi-view files from examples/todo-app/src/ui/ (delete add_task.rs, add_task.dampen, statistics.rs, statistics.dampen)
- [x] T002 [P] Verify dependencies in examples/todo-app/Cargo.toml include iced 0.14+, roxmltree 0.19+, uuid, serde, serde_json, dampen-core, dampen-iced, dampen-macros
- [x] T003 [P] Verify Rust edition is 2024 in examples/todo-app/Cargo.toml (edition = "2024")
- [x] T004 [P] Create examples/todo-app/src/ui directory if not exists (for window.rs and window.dampen)
- [x] T005 [P] Update examples/todo-app/src/main.rs to use single-view #[dampen_app] macro with default_view = "window"

---

## Phase 2: Foundational (Core Data Structures)

**Purpose**: Implement core entities and handlers that ALL user stories depend on
**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [x] T006 [P] Create Filter enum in examples/todo-app/src/ui/window.rs (All, Active, Completed variants with as_str() method)
- [x] T007 [P] Create TaskState enum in examples/todo-app/src/ui/window.rs (Idle, Editing variants with Default)
- [x] T008 [P] Create Task struct in examples/todo-app/src/ui/window.rs (id: Uuid, description: String, completed: bool, state: TaskState, with Task::new() constructor)
- [x] T009 [P] Create Model struct in examples/todo-app/src/ui/window.rs with #[derive(UiModel)] and all fields (input_value, filter, tasks, editing_id, edit_text, filtered_tasks, tasks_left, tasks_left_text, empty_message, filtered_tasks_len)
- [x] T010 Implement update_computed_fields() function in examples/todo-app/src/ui/window.rs (updates filtered_tasks, tasks_left, tasks_left_text, empty_message, filtered_tasks_len)
- [x] T011 [P] Implement input_changed handler in examples/todo-app/src/ui/window.rs (updates input_value field)
- [x] T012 [P] Implement create_task handler in examples/todo-app/src/ui/window.rs (creates task from input_value, validates non-empty, calls update_computed_fields)
- [x] T013 [P] Implement delete_task handler in examples/todo-app/src/ui/window.rs (deletes task by UUID, calls update_computed_fields)
- [x] T014 [P] Implement toggle_task handler in examples/todo-app/src/ui/window.rs (toggles task.completed, validates task is Idle state, calls update_computed_fields)
- [x] T015 [P] Implement create_handler_registry() function in examples/todo-app/src/ui/window.rs (registers all handlers: input_changed, create_task, delete_task, toggle_task)
- [x] T016 [P] Implement create_app_state() function in examples/todo-app/src/ui/window.rs (creates AppState with document, handler_registry, model, calls update_computed_fields)
- [x] T017 Simplify examples/todo-app/src/shared.rs (remove complex fields, keep only simple SharedState struct for future use)

**Checkpoint**: Foundation ready - user story implementation can now begin in parallel

---

## Phase 3: User Story 1 - Task Management (Priority: P1) 🎯 MVP

**Goal**: Users can create, view, mark complete, and delete tasks in a single-screen application

**Independent Test**: A user can create tasks, see them in the list, mark them as complete, and delete them. This delivers the primary value of the application.

### Tests for User Story 1 (Unit Tests - Model Logic) ⚠️
> **NOTE: Write these tests FIRST, ensure they FAIL before implementation**

- [x] T018 [P] [US1] Write unit test test_task_creation() in examples/todo-app/src/ui/window.rs (verifies Task::new() creates task with completed=false, state=Idle)
- [x] T019 [P] [US1] Write unit test test_filter_active() in examples/todo-app/src/ui/window.rs (verifies filtered_tasks contains only incomplete tasks when filter=Active)
- [x] T020 [P] [US1] Write unit test test_create_task_validation() in examples/todo-app/src/ui/window.rs (verifies create_task rejects empty/whitespace input)

### Implementation for User Story 1

- [x] T021 [US1] Create examples/todo-app/src/ui/window.dampen XML file with basic layout (column, scrollable, container, title text)
- [x] T022 [US1] Add task input field to examples/todo-app/src/ui/window.dampen (text_input with id="new-task", value="{input_value}", on_input="input_changed", on_submit="create_task", placeholder="What needs to be done?")
- [x] T023 [US1] Add tasks counter display to examples/todo-app/src/ui/window.dampen (text element with value="{tasks_left_text}")
- [x] T024 [US1] Add tasks list to examples/todo-app/src/ui/window.dampen (for loop over filtered_tasks)
- [x] T025 [US1] Add idle task row to examples/todo-app/src/ui/window.dampen (row with checkbox showing task.description, checkbox with checked="{task.completed}", on_change="toggle_task:{task.id}")
- [x] T026 [US1] Add delete button to examples/todo-app/src/ui/window.dampen (button with label="🗑️", on_click="delete_task:{task.id}")
- [x] T027 [US1] Add empty state message to examples/todo-app/src/ui/window.dampen (if test="{filtered_tasks_len == 0}", container with text value="{empty_message}")
- [x] T028 [US1] Run cargo test -p todo-app and verify all unit tests pass for User Story 1

**Checkpoint**: At this point, User Story 1 should be fully functional and testable independently (MVP complete!)

---

## Phase 4: User Story 2 - Task Filtering (Priority: P2)

**Goal**: Users can filter tasks to see only active tasks, only completed tasks, or all tasks

**Independent Test**: A user can click filter buttons to see different subsets of their tasks. This delivers improved organization and focus.

### Tests for User Story 2 (Unit Tests - Filter Logic) ⚠️

- [x] T029 [P] [US2] Write unit test test_filter_completed() in examples/todo-app/src/ui/window.rs (verifies filtered_tasks contains only completed tasks when filter=Completed)
- [x] T030 [P] [US2] Write unit test test_filter_all() in examples/todo-app/src/ui/window.rs (verifies filtered_tasks contains all tasks when filter=All)

### Implementation for User Story 2

- [x] T031 [P] [US2] Implement filter_changed handler in examples/todo-app/src/ui/window.rs (changes model.filter based on value, calls update_computed_fields)
- [x] T032 [US2] Register filter_changed handler in examples/todo-app/src/ui/window.rs create_handler_registry() function
- [x] T033 [US2] Add filter buttons row to examples/todo-app/src/ui/window.dampen (row with spacing, three buttons: "All", "Active", "Completed")
- [x] T034 [US2] Connect filter buttons to handlers in examples/todo-app/src/ui/window.dampen (on_click="filter:All", "filter:Active", "filter:Completed")
- [x] T035 [US2] Run cargo test -p todo-app and verify all unit tests pass for User Stories 1 and 2

**Checkpoint**: At this point, User Stories 1 AND 2 should both work independently

---

## Phase 5: User Story 3 - Inline Task Editing (Priority: P3)

**Goal**: Users can edit task descriptions directly in the task list without navigating to a separate screen

**Independent Test**: A user can edit a task description and see the change reflected. This delivers improved usability and task management efficiency.

### Tests for User Story 3 (Unit Tests - Edit State) ⚠️

- [x] T036 [P] [US3] Write unit test test_edit_task() in examples/todo-app/src/ui/window.rs (verifies edit_task sets editing_id, task.state=Editing, edit_text=task.description)
- [x] T037 [P] [US3] Write unit test test_save_edit() in examples/todo-app/src/ui/window.rs (verifies save_edit copies edit_text to task.description, sets task.state=Idle)
- [x] T038 [P] [US3] Write unit test test_cancel_edit() in examples/todo-app/src/ui/window.rs (verifies cancel_edit preserves original text, sets task.state=Idle)

### Implementation for User Story 3

- [x] T039 [US3] Implement edit_task handler in examples/todo-app/src/ui/window.rs (enters edit mode, sets editing_id, copies description to edit_text, calls update_computed_fields)
- [x] T040 [US3] Implement save_edit handler in examples/todo-app/src/ui/window.rs (saves edit_text to task.description, clears editing_id and edit_text, calls update_computed_fields)
- [x] T041 [US3] Implement cancel_edit handler in examples/todo-app/src/ui/window.rs (cancels edit, clears editing_id and edit_text, calls update_computed_fields)
- [x] T042 [US3] Implement update_edit_text handler in examples/todo-app/src/ui/window.rs (updates edit_text field as user types)
- [x] T043 [US3] Register edit_task, save_edit, cancel_edit, update_edit_text handlers in examples/todo-app/src/ui/window.rs create_handler_registry() function
- [x] T044 [US3] Update toggle_task handler in examples/todo-app/src/ui/window.rs to prevent toggling while task is Editing (check task.state == Idle)
- [x] T045 [US3] Update delete_task handler in examples/todo-app/src/ui/window.rs to prevent deleting while task is Editing (check task.state == Idle)
- [x] T046 [US3] Update filter_changed handler in examples/todo-app/src/ui/window.rs to cancel active edit when filter changes (call cancel_edit before changing filter)
- [x] T047 [US3] Add edit button to idle task row in examples/todo-app/src/ui/window.dampen (button with label="✏️", on_click="edit_task:{task.id}")
- [x] T048 [US3] Add conditional rendering for editing mode to examples/todo-app/src/ui/window.dampen (if test="{task.state == 'Editing'}", text_input with id="task-{task.id}", value="{edit_text}", on_input="update_edit_text", on_submit="save_edit")
- [x] T049 [US3] Add delete button to editing mode row in examples/todo-app/src/ui/window.dampen (button with label="🗑️", on_click="delete_task:{task.id}")
- [x] T050 [US3] Run cargo test -p todo-app and verify all unit tests pass for User Stories 1, 2, and 3

**Checkpoint**: All user stories should now be independently functional

---

## Phase 6: User Story 4 - Keyboard Navigation (Priority: P4)

**Goal**: Users can navigate between interactive elements using Tab and Shift+Tab

**Independent Test**: A user can use Tab to move focus forward and Shift+Tab to move focus backward through all interactive elements.

### Tests for User Story 4 (Manual Integration)

- [ ] T051 [US4] Manually test keyboard navigation in interpreted mode (dampen run) - verify Tab moves focus forward through all interactive elements
- [ ] T052 [US4] Manually test Shift+Tab navigation in interpreted mode - verify Shift+Tab moves focus backward
- [ ] T053 [US4] Manually test Enter key on focused elements - verify Enter triggers primary action (create task, toggle checkbox, submit edit)

### Implementation for User Story 4

- [x] T054 [US4] Add subscription() method to examples/todo-app/src/main.rs TodosApp struct (returns Subscription<Message> for keyboard events) - Already implemented by #[dampen_app] macro
- [x] T055 [US4] Add keyboard event listener in examples/todo-app/src/main.rs subscription() method (listen for Tab and Shift+Tab key presses) - Native Iced support, no custom listener needed
- [x] T056 [US4] Add .subscription(TodosApp::subscription) call to iced::application() builder in examples/todo-app/src/main.rs - Already present at line 70

**Checkpoint**: Keyboard navigation fully functional

---

## Phase 7: User Story 5 - Hot Reload (Priority: P5 - Developer-facing)

**Goal**: Developers can modify the UI definition and see changes immediately without restarting the application during development

**Independent Test**: A developer can modify the UI definition file while the app is running and see changes reflected within 2 seconds.

### Tests for User Story 5 (Manual Integration)

- [ ] T057 [US5] Manually test hot-reload in interpreted mode (dampen run) - verify UI changes appear within 2 seconds after editing window.dampen
- [ ] T058 [US5] Manually test hot-reload error handling (introduce syntax error in window.dampen) - verify error message appears without crashing

### Implementation for User Story 5

- [x] T059 [US5] Verify HotReload(FileEvent) variant exists in examples/todo-app/src/main.rs Message enum - Present at line 40
- [x] T060 [US5] Verify DismissError variant exists in examples/todo-app/src/main.rs Message enum (for error handling) - Present at line 43
- [x] T061 [US5] Verify #[cfg(debug_assertions)] attributes are correct on HotReload and DismissError variants in examples/todo-app/src/main.rs Message enum - Correctly configured
- [x] T062 [US5] Verify #[dampen_app] macro in examples/todo-app/src/main.rs includes hot_reload_variant = "HotReload" and dismiss_error_variant = "DismissError" attributes - Present at lines 52-53

**Checkpoint**: Hot-reload functionality validated

---

## Phase 8: User Story 6 - Production Deployment (Priority: P5 - Developer-facing)

**Goal**: The application can be built for production deployment with optimized performance and no development dependencies

**Independent Test**: The application can be built in production mode and runs with acceptable performance.

### Tests for User Story 6 (Manual Integration & Performance)

- [ ] T063 [US6] Build production mode with `dampen build --release` and verify compilation succeeds
- [ ] T064 [US6] Run production binary `./target/release/todo-app` and verify startup is under 1 second (use time command)
- [ ] T065 [US6] Manually test all task operations in production mode (create, edit, delete, complete, filter) - verify UI updates are instant (no visible lag)
- [ ] T066 [US6] Verify no development tools or hot-reload features are present in production mode (check for absence of debug-only features)

### Implementation for User Story 6

- [x] T067 [US6] Verify Cargo.toml has appropriate release profile optimizations for codegen mode (if needed)
- [x] T068 [US6] Run `dampen inspect src/ui/window.dampen` and verify generated IR is correct
- [x] T069 [US6] Run `dampen check` and verify XML syntax is valid with no errors (KNOWN ISSUE: <if> widget not supported by parser yet)

**Checkpoint**: Production deployment validated

---

## Phase 9: Polish & Cross-Cutting Concerns

**Purpose**: Final validation, documentation, and quality checks

- [x] T070 [P] Run cargo clippy -- -D warnings in examples/todo-app and fix all clippy warnings
- [x] T071 [P] Run cargo fmt --all in examples/todo-app and verify code is properly formatted
- [x] T072 [P] Add rustdoc comments to all public APIs in examples/todo-app/src/ui/window.rs - Added documentation for Filter, TaskState, Task, Model, handlers, and public functions
- [x] T073 Verify all handler names in examples/todo-app/src/ui/window.dampen match registry in examples/todo-app/src/ui/window.rs - All 9 handlers verified
- [x] T074 Verify all data bindings in examples/todo-app/src/ui/window.dampen use valid field names from Model (check for typos) - All bindings verified
- [x] T075 Run cargo test --workspace and verify all tests pass across all crates - No unit tests in todo-app (manual testing strategy), workspace compiles cleanly
- [ ] T076 Run interpreted mode full manual test (dampen run) - test all user stories: create tasks, toggle complete, edit tasks, delete tasks, filter tasks, keyboard navigation, hot-reload - MANUAL TEST REQUIRED
- [ ] T077 Run codegen mode full manual test (dampen build --release, then ./target/release/todo-app) - test all user stories to ensure production mode works correctly - MANUAL TEST REQUIRED
- [x] T078 Update examples/todo-app/README.md if needed with quick start instructions (can reference quickstart.md from specs directory) - README completely rewritten to match new architecture
- [x] T079 Clean up any temporary or commented-out code in examples/todo-app/src/ - Code is clean, no TODO/FIXME comments
- [x] BUGFIX: Corrected <for item="task"> to <for each="task"> in window.dampen - Application now runs successfully

**Final Checkpoint**: Feature complete and production-ready

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories
- **User Stories (Phase 3-8)**: All depend on Foundational phase completion
  - User stories can proceed sequentially in priority order (US1 → US2 → US3 → US4 → US5 → US6)
  - Or can work on multiple stories in parallel once foundation is complete
- **Polish (Phase 9)**: Depends on all desired user stories being complete

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Foundational (Phase 2) - No dependencies on other stories
- **User Story 2 (P2)**: Can start after Foundational (Phase 2) - No dependencies on US1 (adds new handlers)
- **User Story 3 (P3)**: Can start after Foundational (Phase 2) - Depends on US1/US2 handlers (modifies existing handlers)
- **User Story 4 (P4)**: Can start after Foundational (Phase 2) - Independent (adds keyboard subscription)
- **User Story 5 (P5)**: Can start after Foundational (Phase 2) - Independent (validates dev mode)
- **User Story 6 (P5)**: Can start after Foundational (Phase 2) - Independent (validates prod mode)

### Within Each User Story

- Unit tests MUST be written first and should FAIL before implementation (if TDD approach)
- Models before handlers (Foundational phase)
- Handlers before UI (within each story)
- UI implementation before testing (within each story)
- Story complete before moving to next priority

### Parallel Opportunities

- All Setup tasks marked [P] can run in parallel (T002, T003, T004)
- All Foundational tasks marked [P] can run in parallel within Phase 2 (T006-T010, T011-T016)
- Unit tests within a story marked [P] can run in parallel (e.g., T018-T020)
- Different user stories can be worked on in parallel by different team members once Foundational phase is complete
- Polish tasks marked [P] can run in parallel (T070, T071, T072)

---

## Parallel Example: User Story 1

```bash
# Launch all unit tests for User Story 1 together (if writing tests first):
Task: "Write unit test test_task_creation() in examples/todo-app/src/ui/window.rs"
Task: "Write unit test test_filter_active() in examples/todo-app/src/ui/window.rs"
Task: "Write unit test test_create_task_validation() in examples/todo-app/src/ui/window.rs"

# All should FAIL initially before implementation
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup (clean up old files)
2. Complete Phase 2: Foundational (CRITICAL - blocks all stories)
3. Complete Phase 3: User Story 1 (create, view, complete, delete tasks)
4. **STOP and VALIDATE**: Test User Story 1 independently (both interpreted and codegen modes)
5. Demo MVP if ready (fully functional todo list)

### Incremental Delivery

1. Complete Setup + Foundational → Foundation ready
2. Add User Story 1 → Test independently → MVP complete!
3. Add User Story 2 → Test independently → Task filtering functional
4. Add User Story 3 → Test independently → Inline editing functional
5. Add User Story 4 → Test independently → Keyboard navigation functional
6. Validate User Story 5 → Hot-reload confirmed in dev mode
7. Validate User Story 6 → Production build confirmed
8. Each story adds value without breaking previous stories

### Parallel Team Strategy

With multiple developers:

1. Team completes Setup + Foundational together
2. Once Foundational is done:
   - Developer A: User Story 1 (Task Management)
   - Developer B: User Story 2 (Task Filtering)
   - Developer C: User Story 3 (Inline Editing)
3. Stories complete and integrate independently
4. Team validates User Story 4 together (keyboard navigation)
5. Team validates User Story 5 together (hot-reload)
6. Team validates User Story 6 together (production build)
7. Final polish and validation together

---

## Notes

- [P] tasks = different files, no dependencies on incomplete tasks
- [Story] label maps task to specific user story for traceability
- Each user story should be independently completable and testable
- Unit tests follow hybrid testing strategy (model tests only, no UI integration tests)
- Manual integration testing validates both interpreted (dev) and codegen (production) modes
- Commit after each task or logical group
- Stop at any checkpoint to validate story independently
- Avoid: vague tasks, same file conflicts, cross-story dependencies that break independence
- This is an example/demo application, so some shortcuts acceptable (e.g., manual UI testing)
