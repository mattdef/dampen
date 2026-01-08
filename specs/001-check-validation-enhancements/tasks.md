---

description: "Task list for Check Validation Enhancements feature implementation"
---

# Tasks: Check Validation Enhancements

**Input**: Design documents from `/specs/001-check-validation-enhancements/`
**Prerequisites**: plan.md (required), spec.md (required), research.md, data-model.md, contracts/
**Tests**: Required per constitution - Contract tests for each validation type, property-based tests for Levenshtein

**Organization**: Tasks grouped by user story to enable independent implementation and testing of each story

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story (US1-US7)
- Include exact file paths in descriptions

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Create validation module structure and shared components

- [X] T001 Create check subdirectory structure in crates/gravity-cli/src/commands/check/
- [X] T002 Create check subdirectory for tests in crates/gravity-cli/tests/check/
- [X] T003 Create suggestions.rs with Levenshtein distance algorithm in crates/gravity-cli/src/commands/check/suggestions.rs
- [X] T004 [P] Create attributes.rs with WidgetAttributeSchema in crates/gravity-cli/src/commands/check/attributes.rs
- [X] T005 [P] Add property-based tests for Levenshtein distance in crates/gravity-cli/tests/check/suggestions_tests.rs

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core error types and validator infrastructure that all user stories depend on

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [X] T006 Create errors.rs with all CheckError variants in crates/gravity-cli/src/commands/check/errors.rs
- [X] T007 Create mod.rs to re-export all check validators in crates/gravity-cli/src/commands/check/mod.rs
- [X] T008 Update CheckArgs to add --handlers, --model, --custom-widgets, --strict flags in crates/gravity-cli/src/commands/check.rs
- [X] T009 Create error collection infrastructure to gather all errors before reporting in crates/gravity-cli/src/commands/check.rs
- [X] T010 Create custom_widgets.rs for custom widget attribute configuration in crates/gravity-cli/src/commands/check/custom_widgets.rs

**Checkpoint**: Foundation ready - user story implementation can now begin

---

## Phase 3: User Story 1 - Unknown Attribute Detection (Priority: P1) 🎯 MVP

**Goal**: Detect and report unknown attributes on widgets with Levenshtein distance suggestions

**Independent Test**: Run `gravity check` on XML with misspelled attributes (e.g., `on_clik` instead of `on_click`) and verify errors with suggestions

### Tests for User Story 1 ⚠️

> **NOTE: Write these tests FIRST, ensure they FAIL before implementation**

- [X] T011 [P] [US1] Unit test for unknown attribute detection on button in crates/gravity-cli/tests/check/attributes_tests.rs
- [X] T012 [P] [US1] Unit test for unknown attribute detection on text input in crates/gravity-cli/tests/check/attributes_tests.rs
- [X] T013 [P] [US1] Property-based test for Levenshtein distance suggestions in crates/gravity-cli/tests/check/suggestions_tests.rs
- [X] T014 [P] [US1] Integration test for unknown attribute with strict mode in crates/gravity-cli/tests/check/integration_tests.rs

### Implementation for User Story 1

- [X] T015 [US1] Implement unknown attribute detection in attributes.rs calling suggestions.rs for Levenshtein
- [X] T016 [US1] Add UnknownAttribute error variant to CheckError in errors.rs
- [X] T017 [US1] Integrate attribute validation into main check.rs validator loop

**Checkpoint**: User Story 1 fully functional - run `cargo test -p gravity-cli` to verify

---

## Phase 4: User Story 2 - Handler Registry Validation (Priority: P1)

**Goal**: Validate that event handlers referenced in XML exist in the registered handler set

**Independent Test**: Create XML with handler reference not in registry and verify error with available handlers list

### Tests for User Story 2 ⚠️

- [X] T018 [P] [US2] Unit test for handler registry JSON loading in crates/gravity-cli/tests/check/handler_tests.rs
- [X] T019 [P] [US2] Unit test for unknown handler detection in crates/gravity-cli/tests/check/handler_tests.rs
- [X] T020 [P] [US2] Unit test for handler suggestion with Levenshtein in crates/gravity-cli/tests/check/handler_tests.rs
- [X] T021 [P] [US2] Integration test for handler validation with missing file in crates/gravity-cli/tests/check/integration_tests.rs

### Implementation for User Story 2

- [X] T022 [US2] Create handlers.rs with HandlerRegistry and HandlerDefinition in crates/gravity-cli/src/commands/check/handlers.rs
- [X] T023 [US2] Add UnknownHandler error variant to CheckError in errors.rs
- [X] T024 [US2] Integrate handler validation into main check.rs validator loop
- [X] T025 [US2] Add handler loading logic in check.rs for --handlers flag

**Checkpoint**: User Stories 1 AND 2 should both work independently

---

## Phase 5: User Story 3 - Binding Validation Against Model (Priority: P1)

**Goal**: Validate that binding paths reference existing fields in the model definition

**Independent Test**: Create XML with binding to non-existent field and verify error with available fields list

### Tests for User Story 3 ⚠️

- [X] T026 [P] [US3] Unit test for model info JSON loading in crates/gravity-cli/tests/check/binding_tests.rs
- [X] T027 [P] [US3] Unit test for simple field binding validation in crates/gravity-cli/tests/check/binding_tests.rs
- [X] T028 [P] [US3] Unit test for nested field binding validation in crates/gravity-cli/tests/check/binding_tests.rs
- [X] T029 [P] [US3] Integration test for binding validation with missing file in crates/gravity-cli/tests/check/integration_tests.rs

### Implementation for User Story 3

- [X] T030 [US3] Create model.rs with ModelInfo and ModelField in crates/gravity-cli/src/commands/check/model.rs
- [X] T031 [US3] Add InvalidBindingField error variant to CheckError in errors.rs
- [X] T032 [US3] Integrate binding validation into main check.rs validator loop
- [X] T033 [US3] Add model loading logic in check.rs for --model flag

**Checkpoint**: User Stories 1, 2, AND 3 should all work independently

---

## Phase 6: User Story 4 - Cross-Widget Radio Group Validation (Priority: P2)

**Goal**: Validate radio button groups for duplicate values and inconsistent handlers

**Independent Test**: Create radio group with duplicate values and verify error with both locations

### Tests for User Story 4 ⚠️

- [ ] T034 [P] [US4] Unit test for duplicate radio value detection in crates/gravity-cli/tests/check/radio_tests.rs
- [ ] T035 [P] [US4] Unit test for inconsistent handler detection in crates/gravity-cli/tests/check/radio_tests.rs
- [ ] T036 [P] [US4] Integration test for valid radio group in crates/gravity-cli/tests/check/integration_tests.rs

### Implementation for User Story 4

- [ ] T037 [US4] Create cross_widget.rs with RadioGroup collection and validation in crates/gravity-cli/src/commands/check/cross_widget.rs
- [ ] T038 [US4] Add DuplicateRadioValue and InconsistentRadioHandlers error variants to CheckError in errors.rs
- [ ] T039 [US4] Integrate radio group validation into main check.rs after widget validation

**Checkpoint**: Radio group validation working - test with duplicate values and inconsistent handlers

---

## Phase 7: User Story 5 - Theme Property Validation (Priority: P2)

**Goal**: Validate theme properties and detect circular dependencies

**Independent Test**: Create theme with invalid property and circular dependency, verify errors reported

### Tests for User Story 5 ⚠️

- [ ] T040 [P] [US5] Unit test for invalid theme property detection in crates/gravity-cli/tests/check/theme_tests.rs
- [ ] T041 [P] [US5] Unit test for circular dependency detection in crates/gravity-cli/tests/check/theme_tests.rs
- [ ] T042 [P] [US5] Integration test for valid theme in crates/gravity-cli/tests/check/integration_tests.rs

### Implementation for User Story 5

- [ ] T043 [US5] Create themes.rs with theme property validation and cycle detection in crates/gravity-cli/src/commands/check/themes.rs
- [ ] T044 [US5] Add InvalidThemeProperty and ThemeCircularDependency error variants to CheckError in errors.rs
- [ ] T045 [US5] Integrate theme validation into main check.rs after widget validation

**Checkpoint**: Theme validation complete - test with invalid properties and circular dependencies

---

## Phase 8: User Story 6 - Strict Mode for Warnings as Errors (Priority: P3)

**Goal**: Add --strict flag to treat all warnings as errors for CI/CD quality gates

**Independent Test**: Run with and without --strict on file with warnings, verify different exit codes

### Tests for User Story 6 ⚠️

- [ ] T046 [P] [US6] Integration test for strict mode exit code in crates/gravity-cli/tests/check/integration_tests.rs
- [ ] T047 [P] [US6] Integration test for strict mode with no warnings in crates/gravity-cli/tests/check/integration_tests.rs

### Implementation for User Story 6

- [ ] T048 [US6] Implement strict mode logic in check.rs to exit with code 1 on any warning
- [ ] T049 [US6] Update error formatting to distinguish warnings from errors when in strict mode

**Checkpoint**: Strict mode working - test with `gravity check --strict` on files with warnings

---

## Phase 9: User Story 7 - Required Attribute Validation (Priority: P2)

**Goal**: Report missing required attributes for widgets

**Independent Test**: Create Text widget without value attribute, verify error message

### Tests for User Story 7 ⚠️

- [ ] T050 [P] [US7] Unit test for missing required attribute on Text in crates/gravity-cli/tests/check/attributes_tests.rs
- [ ] T051 [P] [US7] Unit test for missing required attribute on Image in crates/gravity-cli/tests/check/attributes_tests.rs
- [ ] T052 [P] [US7] Unit test for missing required attribute on Radio in crates/gravity-cli/tests/check/attributes_tests.rs
- [ ] T053 [P] [US7] Integration test for required attribute validation in crates/gravity-cli/tests/check/integration_tests.rs

### Implementation for User Story 7

- [ ] T054 [US7] Enhance attributes.rs to check required attributes per widget type
- [ ] T055 [US7] Add MissingRequiredAttribute error variant to CheckError in errors.rs
- [ ] T056 [US7] Integrate required attribute validation into main check.rs validator loop

**Checkpoint**: Required attribute validation complete - test with widgets missing required attrs

---

## Phase 10: Polish & Cross-Cutting Concerns

**Purpose**: Integration, documentation, and performance validation

- [ ] T057 [P] Add integration test for complete validation pipeline with all flags in crates/gravity-cli/tests/check/integration_tests.rs
- [ ] T058 [P] Update existing check_tests.rs to verify backward compatibility in crates/gravity-cli/tests/check_tests.rs
- [ ] T059 [P] Verify performance target (100-500 widgets < 1 second) with benchmark test
- [ ] T060 Run cargo clippy and cargo fmt to ensure code quality
- [ ] T061 [P] Update quickstart.md with any new CLI flag documentation
- [ ] T062 [P] Run all gravity-cli tests: cargo test -p gravity-cli
- [ ] T063 Validate against contracts/ error-messages.md format specification

---

## Dependencies & Execution Order

### Phase Dependencies

| Phase | Depends On | Blocks |
|-------|------------|--------|
| Phase 1: Setup | None | Phase 2 |
| Phase 2: Foundational | Phase 1 | All User Stories (3-9) |
| Phase 3: US1 | Phase 2 | Polish |
| Phase 4: US2 | Phase 2 | Polish |
| Phase 5: US3 | Phase 2 | Polish |
| Phase 6: US4 | Phase 2 | Polish |
| Phase 7: US5 | Phase 2 | Polish |
| Phase 8: US6 | Phase 2 | Polish |
| Phase 9: US7 | Phase 2 | Polish |
| Phase 10: Polish | All User Stories | None |

### User Story Dependencies

| Story | Priority | Dependencies |
|-------|----------|--------------|
| US1: Unknown Attribute Detection | P1 | Phase 2 (Foundational) |
| US2: Handler Registry Validation | P1 | Phase 2 (Foundational) |
| US3: Binding Validation Against Model | P1 | Phase 2 (Foundational) |
| US4: Radio Group Validation | P2 | Phase 2 (Foundational) |
| US5: Theme Property Validation | P2 | Phase 2 (Foundational) |
| US6: Strict Mode | P3 | Phase 2 (Foundational) |
| US7: Required Attribute Validation | P2 | Phase 2 (Foundational) |

**All user stories are independent of each other** - they can be implemented in parallel after Phase 2 completes.

### Within Each User Story

- Tests (T011-T053) MUST be written and FAIL before implementation
- Error variants (T006, T015, T022, T030, T037, T043, T048, T054) before integration
- Integration into check.rs after component implementation
- Story complete before moving to Polish phase

### Parallel Opportunities

- All Setup tasks (T001-T005) can run in parallel
- All Foundational tasks (T006-T010) can run in parallel
- Once Foundational is done, all user stories (US1-US7) can proceed in parallel
- All tests within a story marked [P] can run in parallel
- Different user stories can be worked on by different developers

---

## Parallel Example: User Story 1

```bash
# Launch all tests for User Story 1 together:
Task T011: Unit test for unknown attribute detection on button
Task T012: Unit test for unknown attribute detection on text input  
Task T013: Property-based test for Levenshtein distance
Task T014: Integration test for strict mode

# Implementation tasks for User Story 1:
Task T015: Implement unknown attribute detection
Task T016: Add UnknownAttribute error variant
Task T017: Integrate into main validator loop
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup (T001-T005)
2. Complete Phase 2: Foundational (T006-T010) - CRITICAL
3. Complete Phase 3: User Story 1 (T011-T017)
4. **STOP and VALIDATE**: cargo test -p gravity-cli -- check::attributes
5. Deploy/demo if ready - this delivers unknown attribute detection

### Incremental Delivery

1. Complete Setup + Foundational → Foundation ready
2. Add User Story 1 → Test independently → Deploy/Demo (MVP!)
3. Add User Story 2 → Test independently → Deploy/Demo
4. Add User Story 3 → Test independently → Deploy/Demo
5. Add User Story 4 → Test independently → Deploy/Demo
6. Add User Story 5 → Test independently → Deploy/Demo
7. Add User Story 6 → Test independently → Deploy/Demo
8. Add User Story 7 → Test independently → Deploy/Demo
9. Polish → Final release

Each story adds value without breaking previous stories.

### Parallel Team Strategy

With multiple developers:

1. Team completes Setup + Foundational together
2. Once Foundational is done:
   - Developer A: User Story 1 (Unknown Attributes)
   - Developer B: User Story 2 (Handler Validation)
   - Developer C: User Story 3 (Binding Validation)
3. Stories complete and integrate independently

---

## Task Summary

| Phase | Task Count | Description |
|-------|------------|-------------|
| Phase 1: Setup | 5 | Module structure, Levenshtein, attribute schema |
| Phase 2: Foundational | 5 | Error types, CLI args, validator infrastructure |
| Phase 3: US1 (Unknown Attr) | 7 | Detection + suggestions |
| Phase 4: US2 (Handler) | 8 | Registry validation |
| Phase 5: US3 (Binding) | 8 | Model validation |
| Phase 6: US4 (Radio) | 6 | Cross-widget validation |
| Phase 7: US5 (Theme) | 6 | Property + cycle detection |
| Phase 8: US6 (Strict) | 4 | --strict flag implementation |
| Phase 9: US7 (Required) | 7 | Required attribute checks |
| Phase 10: Polish | 7 | Integration, docs, tests |
| **Total** | **63** | |

---

## Quick Start

```bash
# Run all validation tasks
cargo test -p gravity-cli

# Run specific story tests
cargo test -p gravity-cli -- check::attributes
cargo test -p gravity-cli -- check::handlers
cargo test -p gravity-cli -- check::binding

# Run integration tests
cargo test -p gravity-cli -- check::integration
```
