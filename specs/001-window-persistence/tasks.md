# Tasks: Window State Persistence

**Input**: Design documents from `/specs/001-window-persistence/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/

**Tests**: Tests are included per Constitution Principle V (Test-First Development).

**Organization**: Tasks are grouped by user story to enable independent implementation and testing.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (US1, US2, etc.)
- Exact file paths included in descriptions

## Path Conventions

Based on plan.md structure:
- Core module: `crates/dampen-dev/src/persistence/`
- Macro updates: `crates/dampen-macros/src/`
- CLI templates: `crates/dampen-cli/templates/`
- Examples: `examples/*/src/main.rs`
- Tests: `crates/dampen-dev/tests/`

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Create persistence module structure in dampen-dev

- [ ] T001 Create persistence module directory at `crates/dampen-dev/src/persistence/`
- [ ] T002 Create module entry point at `crates/dampen-dev/src/persistence/mod.rs` with public exports
- [ ] T003 Update `crates/dampen-dev/src/lib.rs` to export persistence module
- [ ] T004 [P] Add `tracing` dependency to `crates/dampen-dev/Cargo.toml` if not present

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core data structures and error types that ALL user stories depend on

**CRITICAL**: No user story work can begin until this phase is complete

- [ ] T005 Create PersistenceError enum with thiserror derives at `crates/dampen-dev/src/persistence/error.rs`
- [ ] T006 Create WindowState struct with serde derives at `crates/dampen-dev/src/persistence/window_state.rs`
- [ ] T007 Implement WindowState::with_defaults() constructor
- [ ] T008 Implement WindowState::validate() method with bounds checking (100-16384)
- [ ] T009 [P] Implement WindowState::size() -> iced::Size conversion method
- [ ] T010 [P] Implement WindowState::position() -> Option<iced::Point> conversion method
- [ ] T011 Create storage utilities module at `crates/dampen-dev/src/persistence/storage.rs`
- [ ] T012 Implement get_config_path() using directories crate in storage.rs

**Checkpoint**: Foundation ready - user story implementation can now begin

---

## Phase 3: User Story 1 - Restore Window Position on App Restart (Priority: P1)

**Goal**: Users can close and reopen an application with window state (size, position) restored

**Independent Test**: Resize/move app window, close it, reopen it, verify window appears at saved dimensions and position

### Tests for User Story 1

> **NOTE: Write these tests FIRST, ensure they FAIL before implementation**

- [ ] T013 [P] [US1] Unit test for load_or_default with missing file in `crates/dampen-dev/tests/persistence_tests.rs`
- [ ] T014 [P] [US1] Unit test for load_or_default with valid JSON file in `crates/dampen-dev/tests/persistence_tests.rs`
- [ ] T015 [P] [US1] Unit test for load_or_default with corrupted JSON file in `crates/dampen-dev/tests/persistence_tests.rs`
- [ ] T016 [P] [US1] Unit test for save_window_state creating directories in `crates/dampen-dev/tests/persistence_tests.rs`
- [ ] T017 [P] [US1] Unit test for save_window_state writing valid JSON in `crates/dampen-dev/tests/persistence_tests.rs`

### Implementation for User Story 1

- [ ] T018 [US1] Implement load_window_state() internal function in `crates/dampen-dev/src/persistence/storage.rs`
- [ ] T019 [US1] Implement load_or_default() public API function in `crates/dampen-dev/src/persistence/storage.rs`
- [ ] T020 [US1] Implement save_window_state() public API function in `crates/dampen-dev/src/persistence/storage.rs`
- [ ] T021 [US1] Add tracing::warn! logging for load failures in storage.rs
- [ ] T022 [US1] Add tracing::warn! logging for save failures in storage.rs
- [ ] T023 [US1] Implement atomic write (temp file + rename) in save_window_state()
- [ ] T024 [US1] Re-export public API functions from `crates/dampen-dev/src/persistence/mod.rs`
- [ ] T025 [US1] Re-export persistence module from `crates/dampen-dev/src/lib.rs`

**Checkpoint**: User Story 1 complete - size/position persistence is functional

---

## Phase 4: User Story 2 - Restore Maximized State (Priority: P1)

**Goal**: Users can have maximized window state persisted and restored

**Independent Test**: Maximize window, close app, reopen, verify window opens maximized

### Tests for User Story 2

- [ ] T026 [P] [US2] Unit test for WindowState serialization with maximized=true in `crates/dampen-dev/tests/persistence_tests.rs`
- [ ] T027 [P] [US2] Unit test for WindowState deserialization with maximized=true in `crates/dampen-dev/tests/persistence_tests.rs`

### Implementation for User Story 2

- [ ] T028 [US2] Verify maximized field is correctly serialized/deserialized in WindowState (already in struct)
- [ ] T029 [US2] Add documentation for maximized field usage in rustdoc at `crates/dampen-dev/src/persistence/window_state.rs`

**Checkpoint**: User Story 2 complete - maximized state persistence is functional

---

## Phase 5: User Story 3 - Developer Enables Persistence (Priority: P1)

**Goal**: Developers can enable persistence with minimal code changes (single line for macro, simple API for manual)

**Independent Test**: Add persistence config to existing app, verify state saved/restored without additional code

### Tests for User Story 3

- [ ] T030 [P] [US3] Integration test for manual persistence integration pattern in `crates/dampen-dev/tests/integration_tests.rs`

### Implementation for User Story 3

- [ ] T031 [US3] Add `persistence` attribute support to `#[dampen_app]` macro at `crates/dampen-macros/src/dampen_app.rs`
- [ ] T032 [US3] Add `app_name` attribute support (required when persistence=true) at `crates/dampen-macros/src/dampen_app.rs`
- [ ] T033 [US3] Generate window event subscription when persistence=true in macro
- [ ] T034 [US3] Generate CloseRequested handler with save_window_state call in macro
- [ ] T035 [US3] Generate Resized/Moved event handlers to track current geometry in macro
- [ ] T036 [US3] Add window state fields to generated AppState when persistence=true
- [ ] T037 [US3] Load persisted state in generated init() function when persistence=true
- [ ] T038 [US3] Update CLI template at `crates/dampen-cli/templates/main.rs.template` with persistence boilerplate comments

**Checkpoint**: User Story 3 complete - developers can enable persistence with single attribute

---

## Phase 6: User Story 4 - Graceful Handling of Disconnected Monitors (Priority: P2)

**Goal**: Windows reposition to visible monitor when saved position is off-screen

**Independent Test**: Save position on secondary monitor, disconnect monitor, reopen app, verify visible on primary

### Tests for User Story 4

- [ ] T039 [P] [US4] Unit test for validate() with position exceeding reasonable bounds in `crates/dampen-dev/tests/persistence_tests.rs`
- [ ] T040 [P] [US4] Unit test for validate() with valid position in `crates/dampen-dev/tests/persistence_tests.rs`

### Implementation for User Story 4

- [ ] T041 [US4] Create monitor validation module at `crates/dampen-dev/src/persistence/monitor.rs`
- [ ] T042 [US4] Implement position_is_reasonable() function checking against reasonable screen bounds
- [ ] T043 [US4] Integrate position validation into load_or_default() - return None position if unreasonable
- [ ] T044 [US4] Export monitor module from `crates/dampen-dev/src/persistence/mod.rs`
- [ ] T045 [US4] Document fallback behavior in rustdoc at storage.rs

**Checkpoint**: User Story 4 complete - off-screen windows are repositioned

---

## Phase 7: User Story 5 - Cross-Platform Position Handling (Priority: P2)

**Goal**: Persistence works on all platforms with graceful degradation on Wayland

**Independent Test**: On Wayland, verify size/maximized persist while position is handled by compositor

### Tests for User Story 5

- [ ] T046 [P] [US5] Unit test for WindowState with x=None, y=None (Wayland scenario) in `crates/dampen-dev/tests/persistence_tests.rs`
- [ ] T047 [P] [US5] Unit test for position() returning None when coordinates missing in `crates/dampen-dev/tests/persistence_tests.rs`

### Implementation for User Story 5

- [ ] T048 [US5] Ensure position fields use #[serde(skip_serializing_if = "Option::is_none")] in window_state.rs
- [ ] T049 [US5] Document platform behavior in rustdoc at window_state.rs (Wayland limitations)
- [ ] T050 [US5] Add platform notes to quickstart.md at `specs/001-window-persistence/quickstart.md`

**Checkpoint**: User Story 5 complete - cross-platform handling verified

---

## Phase 8: Polish & Cross-Cutting Concerns

**Purpose**: Examples, documentation, and final validation

- [ ] T051 [P] Add persistence to hello-world example at `examples/hello-world/src/main.rs`
- [ ] T052 [P] Add persistence to todo-app example at `examples/todo-app/src/main.rs`
- [ ] T053 [P] Add persistence to theming example at `examples/theming/src/main.rs`
- [ ] T054 Add rustdoc module documentation to `crates/dampen-dev/src/persistence/mod.rs`
- [ ] T055 [P] Run `cargo clippy --workspace -- -D warnings` and fix any warnings
- [ ] T056 [P] Run `cargo fmt --all` to ensure formatting
- [ ] T057 Run `cargo test --workspace` to verify all tests pass
- [ ] T058 Validate quickstart.md instructions work end-to-end

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories
- **User Stories (Phases 3-7)**: All depend on Foundational phase completion
  - US1 and US2 can run in parallel (both P1, no interdependencies)
  - US3 depends on US1 completion (needs save/load functions)
  - US4 and US5 can run in parallel (both P2, no interdependencies)
- **Polish (Phase 8)**: Depends on all user stories being complete

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Foundational - Core persistence implementation
- **User Story 2 (P1)**: Can start after Foundational - Maximized state (parallel with US1)
- **User Story 3 (P1)**: Depends on US1 - Macro integration uses save/load functions
- **User Story 4 (P2)**: Can start after Foundational - Monitor validation (parallel with US5)
- **User Story 5 (P2)**: Can start after Foundational - Platform handling (parallel with US4)

### Within Each User Story

- Tests MUST be written and FAIL before implementation (TDD)
- Core implementation before integration
- Story complete before moving to next priority

### Parallel Opportunities

- T003, T004 can run in parallel (different files)
- T009, T010 can run in parallel (different methods, same file is OK)
- All tests within a user story (T013-T017, T026-T027, etc.) can run in parallel
- T051, T052, T053, T055, T056 can run in parallel (different files/operations)

---

## Parallel Example: User Story 1 Tests

```bash
# Launch all tests for User Story 1 together:
Task: "Unit test for load_or_default with missing file"
Task: "Unit test for load_or_default with valid JSON file"
Task: "Unit test for load_or_default with corrupted JSON file"
Task: "Unit test for save_window_state creating directories"
Task: "Unit test for save_window_state writing valid JSON"
```

---

## Parallel Example: Polish Phase

```bash
# Launch all example updates together:
Task: "Add persistence to hello-world example"
Task: "Add persistence to todo-app example"
Task: "Add persistence to theming example"

# Launch all validation together:
Task: "Run cargo clippy --workspace"
Task: "Run cargo fmt --all"
```

---

## Implementation Strategy

### MVP First (User Stories 1-3 Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational (CRITICAL - blocks all stories)
3. Complete Phase 3: User Story 1 (core persistence)
4. Complete Phase 4: User Story 2 (maximized state)
5. Complete Phase 5: User Story 3 (developer API)
6. **STOP and VALIDATE**: Test all P1 stories independently
7. Deploy/demo if ready - this is a functional MVP

### Incremental Delivery

1. Complete Setup + Foundational → Foundation ready
2. Add US1 + US2 → Core persistence works → Test independently
3. Add US3 → Macro integration works → Deploy MVP
4. Add US4 + US5 → Platform polish → Final release

### Parallel Team Strategy

With two developers:

1. Team completes Setup + Foundational together
2. Once Foundational is done:
   - Developer A: User Story 1 → User Story 3 (serial dependency)
   - Developer B: User Story 2 → User Story 4 → User Story 5
3. Both join for Polish phase

---

## Notes

- [P] tasks = different files, no dependencies
- [Story] label maps task to specific user story for traceability
- Each user story should be independently completable and testable
- Verify tests fail before implementing (Constitution Principle V)
- Commit after each task or logical group
- Stop at any checkpoint to validate story independently
- Counter example is NOT updated (stays minimal as reference)
