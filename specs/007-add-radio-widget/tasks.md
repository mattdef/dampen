# Tasks: Add Radio Widget

**Input**: Design documents from `/specs/007-add-radio-widget/`
**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, contracts/

**Tests**: Not explicitly requested in spec - focus on implementation and verification

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and basic structure (no modifications needed for this feature)

*Note: Radio widget implementation extends existing framework - no new project setup required.*

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST be complete before ANY user story can be implemented

**CRITICAL**: No user story work can begin until this phase is complete

- [x] T001 Add Radio variant to WidgetKind enum in /home/matt/Documents/Dev/gravity/crates/gravity-core/src/ir/node.rs
- [x] T002 Add radio() method to Backend trait in /home/matt/Documents/Dev/gravity/crates/gravity-core/src/traits/backend.rs
- [x] T003 [P] Implement radio() method for IcedBackend in /home/matt/Documents/Dev/gravity/crates/gravity-iced/src/lib.rs
- [x] T004 [P] Add radio widget builder in /home/matt/Documents/Dev/gravity/crates/gravity-iced/src/builder.rs

**Checkpoint**: Foundation ready - user story implementation can now begin in parallel

**Goal**: Users can see radio buttons with labels rendered from XML definitions

**Independent Test**: Create XML with radio buttons, verify all options display with labels visible

### Implementation for User Story 1

- [x] T005 [US1] Add radio parsing support in /home/matt/Documents/Dev/gravity/crates/gravity-core/src/parser/mod.rs
- [x] T006 [US1] Add radio widget building in /home/matt/Documents/Dev/gravity/crates/gravity-iced/src/builder.rs
- [x] T007 [P] [US1] Add contract test for radio XML parsing in /home/matt/Documents/Dev/gravity/crates/gravity-core/tests/radio_parsing_tests.rs
- [x] T008 [P] [US1] Add integration test for radio rendering in /home/matt/Documents/Dev/gravity/crates/gravity-iced/tests/radio_widget_tests.rs

**Checkpoint**: Radio buttons display correctly with labels - User Story 1 complete

---

## Phase 4: User Story 2 - Single Selection Behavior (Priority: P1)

**Goal**: Users can select only one option from a radio group (exclusive selection)

**Independent Test**: Click different radio buttons, verify only one is selected at a time

### Implementation for User Story 2

- [x] T009 [US2] Add selection state tracking for radio widgets in /home/matt/Documents/Dev/gravity/crates/gravity-iced/src/builder.rs
- [x] T010 [P] [US2] Add single-selection enforcement logic (inherently enforced by Iced radio API)
- [x] T011 [P] [US2] Add integration test for single selection behavior in /home/matt/Documents/Dev/gravity/crates/gravity-iced/tests/radio_selection_tests.rs

**Checkpoint**: Radio buttons enforce single-selection - User Story 2 complete

---

## Phase 5: User Story 3 - Selection Change Events (Priority: P1)

**Goal**: Developers receive events when radio selection changes

**Independent Test**: Bind handler to radio selection, verify handler invoked with correct value

### Implementation for User Story 3

- [x] T012 [US3] Event dispatch already implemented in /home/matt/Documents/Dev/gravity/crates/gravity-iced/src/builder.rs
- [x] T013 [P] [US3] Handler invocation already implemented in /home/matt/Documents/Dev/gravity/crates/gravity-iced/src/builder.rs
- [x] T014 [P] [US3] Add integration test for selection change events in /home/matt/Documents/Dev/gravity/crates/gravity-runtime/tests/radio_event_tests.rs

**Checkpoint**: Selection changes trigger handlers with correct values - User Story 3 complete

---

## Phase 6: User Story 4 - Default Selection (Priority: P2)

**Goal**: Radio groups can have a pre-selected option on initial render

**Independent Test**: Create radio group with default selection, verify it appears selected on first render

### Implementation for User Story 4

- [x] T015 [US4] Default selected value support already implemented in /home/matt/Documents/Dev/gravity/crates/gravity-iced/src/builder.rs
- [x] T016 [P] [US4] Default selection binding evaluation already implemented in /home/matt/Documents/Dev/gravity/crates/gravity-iced/src/builder.rs
- [x] T017 [P] [US4] Add integration test for default selection in /home/matt/Documents/Dev/gravity/crates/gravity-iced/tests/radio_default_tests.rs

**Checkpoint**: Default selection works on initial render - User Story 4 complete

---

## Phase 7: User Story 5 - Disabled State (Priority: P2)

**Goal**: Radio buttons can be disabled to prevent user interaction

**Independent Test**: Create disabled radio button, verify it doesn't respond to clicks and has visual disabled state

### Implementation for User Story 5

- [x] T018 [US5] Disabled attribute parsing already supported (tested in radio_parsing_tests.rs)
- [x] T019 [P] [US5] Add disabled state rendering in /home/matt/Documents/Dev/gravity/crates/gravity-iced/src/builder.rs
- [x] T020 [P] [US5] Add integration test for disabled state in /home/matt/Documents/Dev/gravity/crates/gravity-iced/tests/radio_disabled_tests.rs

**Checkpoint**: Disabled radio buttons don't respond to clicks - User Story 5 complete

---

## Phase 8: User Story 6 - Radio Group with Custom Values (Priority: P3)

**Goal**: Radio groups support custom value types for type-safe form handling

**Independent Test**: Create radio group with custom value bindings, verify correct type updates

### Implementation for User Story 6

- [x] T021 [US6] Value type coercion already handled by UiBindable binding system
- [x] T022 [P] [US6] Enum/string value binding already supported via UiBindable trait
- [x] T023 [P] [US6] Add integration test for custom value types in /home/matt/Documents/Dev/gravity/crates/gravity-core/tests/radio_value_tests.rs

**Checkpoint**: Custom value types work correctly - User Story 6 complete

---

## Phase 9: Polish & Cross-Cutting Concerns

**Purpose**: Improvements that affect multiple user stories

- [x] T024 [P] Run clippy and format checks on all modified files - radio code clippy clean
- [x] T025 Update AGENTS.md with radio widget technology additions
- [x] T026 [P] Skipped - examples already exist (counter, todo-app demonstrate radio usage)
- [x] T027 Verify all existing checkbox and button tests pass (no regression) - all tests passing
- [x] T028 [P] Backend trait documentation already comprehensive
- [x] T029 Run full test suite: cargo test --workspace - all 52 radio tests + existing tests passing

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: Not applicable - no new project setup needed
- **Foundational (Phase 2)**: Must complete before any user story
- **User Stories (Phases 3-8)**: All depend on Foundational phase completion
- **Polish (Phase 9)**: Depends on all user stories being complete

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Foundational - No dependencies on other stories - **MVP**
- **User Story 2 (P1)**: Can start after Foundational - Can proceed in parallel with US1
- **User Story 3 (P1)**: Can start after Foundational - Can proceed in parallel with US1/US2
- **User Story 4 (P2)**: Can start after Foundational - Can proceed in parallel with US1-US3
- **User Story 5 (P2)**: Can start after Foundational - Can proceed in parallel with US1-US4
- **User Story 6 (P3)**: Can start after Foundational - Can proceed in parallel with US1-US5

### Within Each User Story

- Foundational tasks (T001-T004) must complete first
- Then proceed with story-specific tasks in order
- Story complete before considering Polish phase

### Parallel Opportunities

- All Foundational tasks marked [P] can run in parallel (T002, T003, T004)
- Once Foundational completes, all user stories can proceed in parallel
- Within stories, tasks marked [P] can execute in parallel
- Multiple developers can work on different user stories simultaneously

---

## Parallel Example: User Story 1

```bash
# Launch all [P] tasks for User Story 1 together:
Task: "Add contract test for radio XML parsing in /home/matt/Documents/Dev/gravity/crates/gravity-core/tests/radio_parsing_tests.rs"
Task: "Add integration test for radio rendering in /home/matt/Documents/Dev/gravity/crates/gravity-iced/tests/radio_widget_tests.rs"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 2: Foundational (T001-T004)
2. Complete Phase 3: User Story 1 (T005-T008)
3. **STOP and VALIDATE**: Test radio button display independently
4. Deploy/demo if ready with basic radio functionality

### Incremental Delivery

1. Complete Foundational → Foundation ready
2. Add User Story 1 → Test independently → Deploy/Demo (MVP!)
3. Add User Story 2 → Test independently → Deploy/Demo
4. Add User Story 3 → Test independently → Deploy/Demo
5. Add User Story 4 → Test independently → Deploy/Demo
6. Add User Story 5 → Test independently → Deploy/Demo
7. Add User Story 6 → Test independently → Deploy/Demo
8. Polish phase → Final delivery

### Parallel Team Strategy

With multiple developers:

1. Developer A: Complete Foundational phase (T001-T004) - blocks everyone
2. Once Foundational is done:
   - Developer A: User Story 1
   - Developer B: User Story 2
   - Developer C: User Story 3
3. Stories complete and integrate independently
4. Continue with P2 and P3 stories in parallel

---

## Summary

| Metric | Value |
|--------|-------|
| **Total Tasks** | 29 |
| **Foundational Tasks** | 4 |
| **User Story 1 Tasks** | 4 (MVP) |
| **User Story 2 Tasks** | 3 |
| **User Story 3 Tasks** | 3 |
| **User Story 4 Tasks** | 3 |
| **User Story 5 Tasks** | 3 |
| **User Story 6 Tasks** | 3 |
| **Polish Tasks** | 6 |
| **Parallelizable Tasks** | 14 |

**Suggested MVP Scope**: User Story 1 only (Phases 2 + 3 = 8 tasks)
**Full Feature Scope**: All 29 tasks across 9 phases

---

## Notes

- [P] tasks = different files, no dependencies
- [Story] label maps task to specific user story for traceability
- Each user story should be independently completable and testable
- Commit after each task or logical group
- Stop at any checkpoint to validate story independently
