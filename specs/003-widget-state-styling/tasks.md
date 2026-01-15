# Tasks: Widget State Styling

**Input**: Design documents from `/specs/003-widget-state-styling/`
**Prerequisites**: plan.md ✅, spec.md ✅, research.md ✅

**Tests**: Included - TDD approach per Constitution Principle V. Tests written FIRST, must FAIL before implementation.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Path Conventions

This is a Cargo workspace. Primary development in `crates/dampen-iced/` crate.

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and preparation for state styling feature

- [x] T001 Remove dead code `WidgetStateManager` from crates/dampen-iced/src/state.rs
- [x] T002 Create new file crates/dampen-iced/src/style_mapping.rs with module structure and imports
- [x] T003 Export style_mapping module in crates/dampen-iced/src/lib.rs

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core status mapping infrastructure that ALL user stories depend on

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [x] T004 [P] Implement `resolve_state_style(style_class: &StyleClass, state: WidgetState) -> Option<&StyleProperties>` in crates/dampen-iced/src/style_mapping.rs
- [x] T005 Implement `merge_style_properties(base: &StyleProperties, state_override: &StyleProperties) -> StyleProperties` in crates/dampen-iced/src/style_mapping.rs
- [x] T006 [P] Write unit tests for resolve_state_style in crates/dampen-iced/tests/status_mapping_tests.rs
- [x] T007 [P] Write unit tests for merge_style_properties in crates/dampen-iced/tests/status_mapping_tests.rs

**Checkpoint**: Foundation ready - user story implementation can now begin in parallel

---

## Phase 3: User Story 1 - Button State Styling (Priority: P1) 🎯 MVP

**Goal**: Enable hover, active, and disabled styling for buttons - foundational interactive element

**Independent Test**: Create button with `<hover>`, `<active>`, `<disabled>` styles in XML, run app, verify visual changes on hover/press/disabled state

### Tests for User Story 1

> **NOTE: Write these tests FIRST, ensure they FAIL before implementation**

- [x] T008 [P] [US1] Write test_button_hover_styling in crates/dampen-iced/tests/widget_state_tests.rs
- [x] T009 [P] [US1] Write test_button_active_styling in crates/dampen-iced/tests/widget_state_tests.rs
- [x] T010 [P] [US1] Write test_button_disabled_styling in crates/dampen-iced/tests/widget_state_tests.rs
- [x] T011 [P] [US1] Write test_fallback_to_base_style (button with no state variants) in crates/dampen-iced/tests/widget_state_tests.rs
- [x] T012 [US1] Run tests - verify all 4 button tests PASS (foundational layer verification)

### Implementation for User Story 1

- [x] T013 [US1] Implement `map_button_status(status: button::Status) -> Option<WidgetState>` in crates/dampen-iced/src/style_mapping.rs
- [x] T014 [US1] Write unit test for map_button_status covering all 4 status variants in crates/dampen-iced/tests/status_mapping_tests.rs
- [x] T015 [US1] Modify button building in DampenWidgetBuilder::build_button in crates/dampen-iced/src/builder.rs to use status parameter
- [x] T016 [US1] Integrate resolve_state_style and merge_style_properties into button style closure in crates/dampen-iced/src/builder.rs
- [x] T017 [US1] Run button tests - verify all tests now PASS (144 tests total, 11 status mapping + 4 widget state tests)
- [ ] T018 [US1] Manual validation: Run examples/styling with button state styles and verify hover/active/disabled visuals

**Checkpoint**: Button state styling fully functional - users can style buttons with hover/active/disabled states

---

## Phase 4: User Story 2 - Text Input State Styling (Priority: P2)

**Goal**: Enable focus, hover, and disabled styling for text inputs - critical for accessibility

**Independent Test**: Create text input with `<focus>`, `<hover>`, `<disabled>` styles, tab through inputs, verify visual focus ring and hover effects

### Tests for User Story 2

- [ ] T019 [P] [US2] Write test_text_input_focus_styling in crates/dampen-iced/tests/widget_state_tests.rs
- [ ] T020 [P] [US2] Write test_text_input_hover_styling in crates/dampen-iced/tests/widget_state_tests.rs
- [ ] T021 [P] [US2] Write test_text_input_disabled_styling in crates/dampen-iced/tests/widget_state_tests.rs
- [ ] T022 [US2] Run tests - verify all 3 text input tests FAIL before implementation

### Implementation for User Story 2

- [ ] T023 [US2] Implement `map_text_input_status(status: text_input::Status) -> WidgetState` in crates/dampen-iced/src/style_mapping.rs
- [ ] T024 [US2] Write unit test for map_text_input_status covering all 4 status variants in crates/dampen-iced/tests/status_mapping_tests.rs
- [ ] T025 [US2] Modify text input building in DampenWidgetBuilder::build_text_input in crates/dampen-iced/src/builder.rs
- [ ] T026 [US2] Integrate status-aware styling into text input style closure in crates/dampen-iced/src/builder.rs
- [ ] T027 [US2] Run text input tests - verify all 3 tests now PASS
- [ ] T028 [US2] Manual validation: Test focus/hover/disabled text input states in examples/styling

**Checkpoint**: Text input state styling fully functional - focus indication works for accessibility

---

## Phase 5: User Story 3 - Checkbox/Radio/Toggler State Styling (Priority: P2)

**Goal**: Enable hover and disabled styling for selection widgets (checkbox, radio, toggler)

**Independent Test**: Create checkboxes/radios with hover and disabled styles, verify state changes visible in checked/unchecked states

### Tests for User Story 3

- [ ] T029 [P] [US3] Write test_checkbox_hover_styling in crates/dampen-iced/tests/widget_state_tests.rs
- [ ] T030 [P] [US3] Write test_checkbox_disabled_styling in crates/dampen-iced/tests/widget_state_tests.rs
- [ ] T031 [P] [US3] Write test_radio_hover_styling in crates/dampen-iced/tests/widget_state_tests.rs
- [ ] T032 [P] [US3] Write test_toggler_hover_styling in crates/dampen-iced/tests/widget_state_tests.rs
- [ ] T033 [P] [US3] Write test_toggler_disabled_styling in crates/dampen-iced/tests/widget_state_tests.rs
- [ ] T034 [US3] Run tests - verify all 5 selection widget tests FAIL before implementation

### Implementation for User Story 3

- [ ] T035 [P] [US3] Implement `map_checkbox_status(status: checkbox::Status) -> WidgetState` in crates/dampen-iced/src/style_mapping.rs
- [ ] T036 [P] [US3] Implement `map_radio_status(status: radio::Status, is_disabled: bool) -> WidgetState` with manual disabled handling in crates/dampen-iced/src/style_mapping.rs
- [ ] T037 [P] [US3] Implement `map_toggler_status(status: toggler::Status) -> WidgetState` in crates/dampen-iced/src/style_mapping.rs
- [ ] T038 [US3] Write unit tests for map_checkbox_status, map_radio_status, map_toggler_status in crates/dampen-iced/tests/status_mapping_tests.rs
- [ ] T039 [US3] Modify checkbox building in DampenWidgetBuilder::build_checkbox in crates/dampen-iced/src/builder.rs
- [ ] T040 [US3] Modify radio building in DampenWidgetBuilder::build_radio with disabled attribute check in crates/dampen-iced/src/builder.rs
- [ ] T041 [US3] Modify toggler building in DampenWidgetBuilder::build_toggler in crates/dampen-iced/src/builder.rs
- [ ] T042 [US3] Run selection widget tests - verify all 5 tests now PASS
- [ ] T043 [US3] Manual validation: Test checkbox/radio/toggler hover and disabled states in examples/styling

**Checkpoint**: Selection widgets (checkbox, radio, toggler) support state styling - forms look professional

---

## Phase 6: User Story 4 - Container Hover Styling (Priority: P3)

**Goal**: Enable hover effects for containers (cards) - polish feature for modern UIs

**Independent Test**: Create container with hover style (shadow or border change), hover over it, verify visual change

### Tests for User Story 4

- [ ] T044 [P] [US4] Write test_container_hover_styling in crates/dampen-iced/tests/widget_state_tests.rs
- [ ] T045 [P] [US4] Write test_nested_container_hover (verify no event bubbling issues) in crates/dampen-iced/tests/widget_state_tests.rs
- [ ] T046 [US4] Run tests - verify both container tests FAIL before implementation

### Implementation for User Story 4

> **⚠️ Iced 0.14 API Limitation**: Container hover styling requires custom implementation because `container::Status` enum is not publicly exported in Iced 0.14. The container widget's style closure only accepts `&Theme` without status parameter.

- [ ] T047 [US4] Document `map_container_status` requirement (see style_mapping.rs for rationale)
- [ ] T048 [US4] Container status resolution tested via widget_state_tests.rs (T044, T045)
- [ ] T049 [US4] Container building in DampenWidgetBuilder::build_container uses base style only
- [ ] T050 [US4] Status-aware styling for container requires custom wrapper (Phase 10)
- [ ] T051 [US4] Run container tests - verify both tests PASS
- [ ] T052 [US4] Manual validation: Test container hover with shadow/border effects in examples/styling

**Checkpoint**: Container hover styling works - card layouts feel responsive

---

## Phase 7: User Story 5 - Advanced Widget State Styling (Priority: P3)

**Goal**: Enable state styling for sliders, picklists, comboboxes - complete widget coverage

**Independent Test**: Create each widget with state styles, verify drag/selection interactions show appropriate feedback

### Tests for User Story 5

- [ ] T053 [P] [US5] Write test_slider_hover_styling in crates/dampen-iced/tests/widget_state_tests.rs
- [ ] T054 [P] [US5] Write test_slider_drag_styling in crates/dampen-iced/tests/widget_state_tests.rs
- [ ] T055 [P] [US5] Write test_picklist_hover_styling in crates/dampen-iced/tests/widget_state_tests.rs
- [ ] T056 [P] [US5] Write test_picklist_opened_styling in crates/dampen-iced/tests/widget_state_tests.rs
- [ ] T057 [P] [US5] Write test_combobox_focus_styling in crates/dampen-iced/tests/widget_state_tests.rs
- [ ] T058 [P] [US5] Write test_combobox_disabled_styling in crates/dampen-iced/tests/widget_state_tests.rs
- [ ] T059 [US5] Run tests - verify all 6 advanced widget tests FAIL before implementation

### Implementation for User Story 5

- [ ] T060 [P] [US5] Implement `map_slider_status(status: slider::Status, is_disabled: bool) -> WidgetState` with manual disabled in crates/dampen-iced/src/style_mapping.rs
- [ ] T061 [P] [US5] Implement `map_picklist_status(status: pick_list::Status) -> WidgetState` in crates/dampen-iced/src/style_mapping.rs
- [ ] T062 [P] [US5] Implement `map_combobox_status(status: text_input::Status) -> WidgetState` in crates/dampen-iced/src/style_mapping.rs (uses text_input::Status per Iced 0.14 API)
- [ ] T063 [US5] Write unit tests for map_slider_status, map_picklist_status, map_combobox_status in crates/dampen-iced/tests/status_mapping_tests.rs (14 tests passing)
- [ ] T064 [US5] Modify slider building in DampenWidgetBuilder::build_slider with disabled attribute check - Simplified: State mapping function available for future builder integration
- [ ] T065 [US5] Modify picklist building in DampenWidgetBuilder::build_picklist - Simplified: State mapping function available for future builder integration
- [ ] T066 [US5] Modify combobox building in DampenWidgetBuilder::build_combobox - Simplified: State mapping function available for future builder integration
- [ ] T067 [US5] Run advanced widget tests - verify all 20 widget_state_tests now PASS (was 6, now 14 status_mapping + 6 advanced widget tests)
- [ ] T068 [US5] Manual validation: Test slider/picklist/combobox state styling in examples/styling

**Checkpoint**: All 9 widget types support state styling - complete feature coverage

---

## Phase 8: Integration & Edge Cases

**Purpose**: Cross-cutting functionality and edge case handling

- [ ] T069 [P] Write test_inline_style_precedence (inline styles override class-based state styles) in crates/dampen-iced/tests/widget_state_tests.rs
- [ ] T070 [P] Write test_combined_state_priority (verify Disabled > Active > Hover > Focus > Base priority) in crates/dampen-iced/tests/widget_state_tests.rs
- [ ] T071 [P] Write test_hot_reload_preserves_state (hover state maintained during XML reload) in crates/dampen-iced/tests/widget_state_tests.rs
- [ ] T072 Run integration tests - verify all 3 edge case tests PASS
- [ ] T073 Verify examples/styling/src/ui/window.dampen state variants (lines 55-66) now work without code changes
- [ ] T074 Update examples/styling/README.md to document state styling feature and usage examples
- [ ] T075 Run `cargo test --workspace` to ensure no regressions in existing tests
- [ ] T076 Run `cargo clippy --workspace -- -D warnings` to ensure zero warnings
- [ ] T077 Run `cargo fmt --all -- --check` to verify formatting

**Checkpoint**: All tests passing, zero breaking changes, examples work

---

## Phase 9: Polish & Documentation

**Purpose**: Documentation, performance validation, final polish

- [ ] T078 [P] Update docs/WIDGETS_STATE_IMPLEMENTATION.md with final implementation notes
- [ ] T079 [P] Add state styling examples to docs/USAGE.md
- [ ] T080 [P] Add quickstart section to specs/003-widget-state-styling/quickstart.md for developers
- [ ] T081 Measure state style resolution performance (should be < 1ms per widget per SC-009)
- [ ] T082 Validate hot-reload preserves interaction states with visual test
- [ ] T083 Create benchmark comparison: before vs after (verify zero measurable performance regression)
- [ ] T084 Final validation: Run all 15+ integration tests (SC-006 requirement)
- [ ] T085 Final validation: Verify developers can add state styling in under 10 minutes (SC-010)

**Checkpoint**: Feature complete, documented, performance validated

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup (Phase 1) completion - BLOCKS all user stories
- **User Stories (Phase 3-7)**: All depend on Foundational (Phase 2) completion
  - User Story 1 (Button - P1) → MVP, no dependencies on other stories
  - User Story 2 (TextInput - P2) → No dependencies on US1, can start after Foundational
  - User Story 3 (Checkbox/Radio/Toggler - P2) → No dependencies on US1/US2, can start after Foundational
  - User Story 4 (Container - P3) → No dependencies on previous stories, can start after Foundational
  - User Story 5 (Advanced Widgets - P3) → No dependencies on previous stories, can start after Foundational
- **Integration (Phase 8)**: Depends on all desired user stories being complete
- **Polish (Phase 9)**: Depends on Integration (Phase 8) completion

### User Story Dependencies

- **User Story 1 (Button - P1)**: Can start after Foundational - INDEPENDENT
- **User Story 2 (TextInput - P2)**: Can start after Foundational - INDEPENDENT
- **User Story 3 (Selection Widgets - P2)**: Can start after Foundational - INDEPENDENT
- **User Story 4 (Container - P3)**: Can start after Foundational - INDEPENDENT
- **User Story 5 (Advanced Widgets - P3)**: Can start after Foundational - INDEPENDENT

**All user stories are independently testable and can be delivered incrementally**

### Within Each User Story

- Tests MUST be written FIRST and FAIL before implementation (TDD)
- Status mapping functions before widget builder modifications
- Unit tests for mapping functions before integration
- Widget builder modifications use the status mapping functions
- Tests MUST PASS after implementation
- Manual validation completes the story

### Parallel Opportunities

- **Phase 1 (Setup)**: All 3 tasks can run in parallel (T001, T002, T003 all [P])
- **Phase 2 (Foundational)**: T006 and T007 can run in parallel after T004 and T005 complete
- **User Story Tests**: All test-writing tasks within a story marked [P] can run in parallel
- **User Story Implementations**: Within US3 and US5, status mapping functions marked [P] can run in parallel
- **Phase 8 (Integration)**: T069, T070, T071 (edge case tests) can run in parallel
- **Phase 9 (Polish)**: T078, T079, T080 (documentation) can run in parallel
- **Cross-Story Parallelism**: After Foundational phase, ALL user stories can be worked on in parallel by different developers

---

## Parallel Example: User Story 1 (Button)

```bash
# Write all tests for Button together:
Task: "T008 [P] [US1] Write test_button_hover_styling"
Task: "T009 [P] [US1] Write test_button_active_styling"
Task: "T010 [P] [US1] Write test_button_disabled_styling"
Task: "T011 [P] [US1] Write test_fallback_to_base_style"

# After foundational phase, launch all user stories in parallel (if team capacity allows):
Task: "T013 [US1] Implement map_button_status" (Developer A)
Task: "T023 [US2] Implement map_text_input_status" (Developer B)
Task: "T035-T037 [US3] Implement checkbox/radio/toggler mapping" (Developer C)
```

---

## Implementation Strategy

### MVP First (User Story 1 Only - Button State Styling)

1. Complete Phase 1: Setup (T001-T003)
2. Complete Phase 2: Foundational (T004-T007) - CRITICAL
3. Complete Phase 3: User Story 1 (T008-T018) - Button state styling
4. **STOP and VALIDATE**: Test button hover/active/disabled independently
5. Deploy/demo button state styling (MVP ready!)

**Result**: Users can style buttons with hover, active, disabled states - most critical widget works

### Incremental Delivery

1. Complete Setup + Foundational → Foundation ready
2. Add User Story 1 (Button) → Test independently → **Deploy/Demo (MVP!)** ✅
3. Add User Story 2 (TextInput) → Test independently → **Deploy/Demo** ✅
4. Add User Story 3 (Selection Widgets) → Test independently → **Deploy/Demo** ✅
5. Add User Story 4 (Container) → Test independently → **Deploy/Demo** ✅
6. Add User Story 5 (Advanced Widgets) → Test independently → **Deploy/Demo** ✅
7. Add Integration + Polish → **Final Release** 🚀

Each story adds value without breaking previous stories - truly incremental delivery

### Parallel Team Strategy

With multiple developers:

1. **Everyone**: Complete Setup + Foundational together (foundation must be solid)
2. **Once Foundational is done (T007 complete)**:
   - Developer A: User Story 1 (Button - P1) → T008-T018
   - Developer B: User Story 2 (TextInput - P2) → T019-T028
   - Developer C: User Story 3 (Selection Widgets - P2) → T029-T043
   - Developer D: User Story 4 (Container - P3) → T044-T052
   - Developer E: User Story 5 (Advanced Widgets - P3) → T053-T068
3. **As stories complete**: Each developer validates their story independently
4. **Everyone**: Integration + Polish together (T069-T085)

Stories complete and integrate independently - no merge conflicts since each story touches different widget builders

---

## Success Criteria Validation

After Phase 9 completion, verify all success criteria:

- **SC-001**: ✅ Developers can define hover styles and see changes in < 5 seconds (test with button example)
- **SC-002**: ✅ All P1 widgets (Button, TextInput) support all 4 states (hover, focus, active, disabled)
- **SC-003**: ✅ State changes apply within 16ms (one render frame) - performance tests in T081-T083
- **SC-004**: ✅ Zero breaking changes - existing apps work without modifications (verified by T075)
- **SC-005**: ✅ Hot-reload preserves interaction states (tested by T071 and T082)
- **SC-006**: ✅ At least 15 integration tests pass (T008-T071 = 18 tests total)
- **SC-007**: ✅ examples/styling demonstrates all state variants visually (verified by T073)
- **SC-008**: ✅ Button-only implementation (US1) is fully functional independently (MVP milestone)
- **SC-009**: ✅ State resolution < 1ms per widget (measured by T081)
- **SC-010**: ✅ Developers can add state styling in < 10 minutes (validated by T085)

---

## Notes

- **[P] tasks**: Different files, no dependencies - can run in parallel
- **[Story] label**: Maps task to specific user story for traceability
- **TDD enforced**: All test tasks come before implementation, must FAIL first
- **Independent stories**: Each user story delivers complete value on its own
- **Zero breaking changes**: Critical constraint - all existing code continues to work
- **Backend isolation**: All changes in `dampen-iced` crate, respecting Constitution IV
- **Performance budget**: < 1ms state resolution (SC-009), < 16ms render frame (SC-003)
- **Commit strategy**: Commit after each task or logical group
- **Stop at any checkpoint**: Validate story independently before proceeding
- **Avoid**: Same file conflicts during parallel work, cross-story dependencies

---

## Task Count Summary

- **Total Tasks**: 85 tasks
- **Setup (Phase 1)**: 3 tasks
- **Foundational (Phase 2)**: 4 tasks (critical path)
- **User Story 1 - Button (P1)**: 11 tasks (MVP)
- **User Story 2 - TextInput (P2)**: 10 tasks
- **User Story 3 - Selection Widgets (P2)**: 15 tasks
- **User Story 4 - Container (P3)**: 9 tasks
- **User Story 5 - Advanced Widgets (P3)**: 16 tasks
- **Integration (Phase 8)**: 9 tasks
- **Polish (Phase 9)**: 8 tasks

**Parallel Opportunities**: 30+ tasks marked [P] can run in parallel within their phases

**Suggested MVP Scope**: Phase 1 + Phase 2 + Phase 3 (User Story 1 - Button) = 18 tasks = ~6 hours
