# Tasks: Widget Schema Migration to Core

**Input**: Design documents from `/specs/001-widget-schema-core/`
**Prerequisites**: plan.md (required), spec.md (required), research.md, data-model.md

**Tests**: Following TDD approach per Constitution (Principle V). Tests written first.

**Organization**: Tasks grouped by user story for independent implementation and testing.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (US1, US2, US3, US4)

## Path Conventions

- **dampen-core**: `crates/dampen-core/src/`
- **dampen-cli**: `crates/dampen-cli/src/`
- **Tests**: Inline `#[cfg(test)]` modules per Rust convention

---

## Phase 1: Setup

**Purpose**: Prepare the module structure in dampen-core

- [x] T001 Create schema module directory at `crates/dampen-core/src/schema/`
- [x] T002 Create empty module file at `crates/dampen-core/src/schema/mod.rs`
- [x] T003 Add `pub mod schema;` declaration in `crates/dampen-core/src/lib.rs`
- [x] T004 Add re-exports in `crates/dampen-core/src/lib.rs` for schema types

---

## Phase 2: Foundational (Core Schema Module)

**Purpose**: Implement the core schema infrastructure that ALL user stories depend on

**CRITICAL**: No user story work can begin until this phase is complete

- [x] T005 Define `WidgetSchema` struct with static slice fields in `crates/dampen-core/src/schema/mod.rs`
- [x] T006 [P] Define `COMMON_STYLE_ATTRIBUTES` constant (14 attributes) in `crates/dampen-core/src/schema/mod.rs`
- [x] T007 [P] Define `COMMON_LAYOUT_ATTRIBUTES` constant (22 attributes including align_x, align_y, align_self) in `crates/dampen-core/src/schema/mod.rs`
- [x] T008 [P] Define `COMMON_EVENTS` constant (9 attributes) in `crates/dampen-core/src/schema/mod.rs`
- [x] T009 Implement `WidgetSchema::all_valid()` method returning `HashSet<&'static str>` in `crates/dampen-core/src/schema/mod.rs`
- [x] T010 Implement `WidgetSchema::all_valid_names()` method returning `Vec<&'static str>` in `crates/dampen-core/src/schema/mod.rs`

**Checkpoint**: Schema struct and common constants ready - widget-specific schemas can now be implemented

---

## Phase 3: User Story 2 - CLI Queries Core for Attributes (Priority: P1)

**Goal**: Implement schema retrieval for all 25 widget types so CLI can query dampen-core

**Independent Test**: Call `get_widget_schema()` for each widget type and verify correct attributes returned

**Why US2 first**: US2 provides the mechanism (schema API) that US1 depends on. Must be complete before US1 can be tested.

### Tests for User Story 2

- [x] T011 [P] [US2] Write test `test_button_schema_contains_expected_attributes` in `crates/dampen-core/src/schema/mod.rs`
- [x] T012 [P] [US2] Write test `test_container_schema_includes_layout_attributes` in `crates/dampen-core/src/schema/mod.rs`
- [x] T013 [P] [US2] Write test `test_textinput_schema_includes_size` in `crates/dampen-core/src/schema/mod.rs`
- [x] T014 [P] [US2] Write test `test_custom_widget_returns_permissive_schema` in `crates/dampen-core/src/schema/mod.rs`
- [x] T015 [P] [US2] Write test `test_all_widget_kinds_have_schema` in `crates/dampen-core/src/schema/mod.rs`

### Implementation for User Story 2

- [x] T016 [US2] Implement `WidgetKind::schema(&self) -> WidgetSchema` method in `crates/dampen-core/src/schema/mod.rs`
- [x] T017 [P] [US2] Add schema for Text widget (required: value, optional: size/weight/color) in `crates/dampen-core/src/schema/mod.rs`
- [x] T018 [P] [US2] Add schema for Image widget (required: src) in `crates/dampen-core/src/schema/mod.rs`
- [x] T019 [P] [US2] Add schema for Button widget (events: on_click/on_press/on_release) in `crates/dampen-core/src/schema/mod.rs`
- [x] T020 [P] [US2] Add schema for TextInput widget (optional includes size) in `crates/dampen-core/src/schema/mod.rs`
- [x] T021 [P] [US2] Add schema for Checkbox widget (optional includes size) in `crates/dampen-core/src/schema/mod.rs`
- [x] T022 [P] [US2] Add schema for Radio widget (required: label/value) in `crates/dampen-core/src/schema/mod.rs`
- [x] T023 [P] [US2] Add schema for Slider widget in `crates/dampen-core/src/schema/mod.rs`
- [x] T024 [P] [US2] Add schema for Column/Row/Container widgets in `crates/dampen-core/src/schema/mod.rs`
- [x] T025 [P] [US2] Add schema for Scrollable/Stack widgets in `crates/dampen-core/src/schema/mod.rs`
- [x] T026 [P] [US2] Add schema for Svg widget (required: src) in `crates/dampen-core/src/schema/mod.rs`
- [x] T027 [P] [US2] Add schema for PickList/ComboBox widgets in `crates/dampen-core/src/schema/mod.rs`
- [x] T028 [P] [US2] Add schema for Toggler widget in `crates/dampen-core/src/schema/mod.rs`
- [x] T029 [P] [US2] Add schema for Space/Rule widgets in `crates/dampen-core/src/schema/mod.rs`
- [x] T030 [P] [US2] Add schema for ProgressBar widget (optional includes style) in `crates/dampen-core/src/schema/mod.rs`
- [x] T031 [P] [US2] Add schema for Tooltip widget (no layout attributes) in `crates/dampen-core/src/schema/mod.rs`
- [x] T032 [P] [US2] Add schema for Grid/Canvas/Float widgets in `crates/dampen-core/src/schema/mod.rs`
- [x] T033 [P] [US2] Add schema for For/If control flow widgets in `crates/dampen-core/src/schema/mod.rs`
- [x] T034 [P] [US2] Add schema for Custom widget (empty/permissive) in `crates/dampen-core/src/schema/mod.rs`
- [x] T035 [US2] Implement standalone `get_widget_schema(kind: &WidgetKind) -> WidgetSchema` function in `crates/dampen-core/src/schema/mod.rs`
- [x] T036 [US2] Run `cargo test -p dampen-core` and verify all US2 tests pass

**Checkpoint**: Schema API complete and tested - CLI can now query dampen-core for widget schemas

---

## Phase 4: User Story 1 - Single Source of Truth (Priority: P1)

**Goal**: Refactor dampen-cli to use dampen-core schema, eliminating duplicate definitions

**Independent Test**: Add mock attribute to dampen-core schema, run `dampen check` without CLI changes, verify attribute recognized

### Tests for User Story 1

- [x] T037 [P] [US1] Write integration test: CLI recognizes attribute defined only in dampen-core in `crates/dampen-cli/tests/check_attributes.rs`
- [x] T038 [P] [US1] Write integration test: CLI reports unknown attribute not in dampen-core schema in `crates/dampen-cli/tests/check_attributes.rs`

### Implementation for User Story 1

- [x] T039 [US1] Create thin wrapper `WidgetAttributeSchema` struct delegating to `dampen_core::schema::WidgetSchema` in `crates/dampen-cli/src/commands/check/attributes.rs`
- [x] T040 [US1] Implement `WidgetAttributeSchema::for_widget()` delegating to `kind.schema()` in `crates/dampen-cli/src/commands/check/attributes.rs`
- [x] T041 [US1] Implement wrapper methods `all_valid()`, `all_valid_names()` delegating to inner schema in `crates/dampen-cli/src/commands/check/attributes.rs`
- [x] T042 [US1] Add public fields `required`, `optional`, `events`, `style_attributes`, `layout_attributes` to wrapper for test compatibility in `crates/dampen-cli/src/commands/check/attributes.rs`
- [x] T043 [US1] Remove `lazy_static!` block with `STYLE_COMMON`, `LAYOUT_COMMON`, `EVENTS_COMMON` from `crates/dampen-cli/src/commands/check/attributes.rs`
- [x] T044 [US1] Remove widget-specific match arms from old `for_widget()` implementation in `crates/dampen-cli/src/commands/check/attributes.rs`
- [x] T045 [US1] Update `validate_widget_attributes()` to use new wrapper in `crates/dampen-cli/src/commands/check/attributes.rs`
- [x] T046 [US1] Update `validate_required_attributes()` to use new wrapper in `crates/dampen-cli/src/commands/check/attributes.rs`
- [x] T047 [US1] Run existing CLI tests: `cargo test -p dampen-cli` and verify all pass without test logic changes
- [x] T048 [US1] Run `dampen check --input examples/todo-app/src/ui --verbose` and verify 0 errors

**Checkpoint**: CLI now uses single source of truth from dampen-core

---

## Phase 5: User Story 3 - Third-Party Tool Access (Priority: P2)

**Goal**: Ensure schema module is publicly accessible and documented for external tools

**Independent Test**: Import `dampen_core::schema` from external project, access all public types

### Tests for User Story 3

- [x] T049 [P] [US3] Write test verifying `WidgetSchema` is accessible via `dampen_core::schema::WidgetSchema` in `crates/dampen-core/src/schema/mod.rs`
- [x] T050 [P] [US3] Write test verifying constants are accessible via `dampen_core::schema::COMMON_*` in `crates/dampen-core/src/schema/mod.rs`

### Implementation for User Story 3

- [x] T051 [US3] Add rustdoc module documentation to `crates/dampen-core/src/schema/mod.rs`
- [x] T052 [US3] Add rustdoc for `WidgetSchema` struct with examples in `crates/dampen-core/src/schema/mod.rs`
- [x] T053 [US3] Add rustdoc for `get_widget_schema()` function with examples in `crates/dampen-core/src/schema/mod.rs`
- [x] T054 [US3] Add rustdoc for common attribute constants in `crates/dampen-core/src/schema/mod.rs`
- [x] T055 [US3] Run `cargo doc -p dampen-core --open` and verify schema module appears in documentation
- [x] T056 [US3] Add `pub use schema::*;` exports in `crates/dampen-core/src/lib.rs` for convenience access

**Checkpoint**: Schema module fully documented and publicly accessible

---

## Phase 6: User Story 4 - Backward Compatibility (Priority: P2)

**Goal**: Verify identical behavior before and after migration

**Independent Test**: Compare `dampen check` output on same files before and after migration

### Tests for User Story 4

- [x] T057 [P] [US4] Write test verifying `dampen check` passes on todo-app with 0 errors in `crates/dampen-cli/tests/check_tests.rs`
- [x] T058 [P] [US4] Write test verifying error message format unchanged for unknown attributes in `crates/dampen-cli/tests/check_tests.rs`
- [x] T059 [P] [US4] Write test verifying suggestion logic still works (e.g., "align_xx" suggests "align_x") in `crates/dampen-cli/tests/check_tests.rs`

### Implementation for User Story 4

- [x] T060 [US4] Verify `validate_widget_attributes()` produces identical suggestions in `crates/dampen-cli/src/commands/check/attributes.rs`
- [x] T061 [US4] Verify `is_valid_attribute()` function works unchanged in `crates/dampen-cli/src/commands/check/attributes.rs`
- [x] T062 [US4] Run full CLI test suite: `cargo test -p dampen-cli --all-features`
- [x] T063 [US4] Run workspace tests: `cargo test --workspace`

**Checkpoint**: Backward compatibility verified - no regressions

---

## Phase 7: Polish & Cleanup

**Purpose**: Final cleanup and dependency removal

- [x] T064 [P] Check if `lazy_static` is used elsewhere in dampen-cli: `grep -r "lazy_static" crates/dampen-cli/`
- [x] T065 Remove `lazy_static` from `crates/dampen-cli/Cargo.toml` dependencies (if no other usages found)
- [x] T066 [P] Run `cargo clippy --workspace -- -D warnings` and fix any warnings
- [x] T067 [P] Run `cargo fmt --all -- --check` and fix any formatting issues
- [x] T068 Run `cargo build --release --workspace` and verify clean build
- [x] T069 Run quickstart.md validation: test example code snippets compile
- [x] T070 Update AGENTS.md if needed (already done via update-agent-context.sh)

---

## Dependencies & Execution Order

### Phase Dependencies

- **Phase 1 (Setup)**: No dependencies - start immediately
- **Phase 2 (Foundational)**: Depends on Phase 1 - BLOCKS all user stories
- **Phase 3 (US2)**: Depends on Phase 2 - Must complete before US1
- **Phase 4 (US1)**: Depends on Phase 3 (US2) - Uses schema API
- **Phase 5 (US3)**: Depends on Phase 3 (US2) - Can run parallel with US1
- **Phase 6 (US4)**: Depends on Phase 4 (US1) - Validates complete migration
- **Phase 7 (Polish)**: Depends on all user stories complete

### User Story Dependencies

```
Phase 2 (Foundational)
        |
        v
   Phase 3 (US2) ──────────────────┐
        |                          |
        v                          v
   Phase 4 (US1)             Phase 5 (US3)
        |                          |
        └──────────┬───────────────┘
                   v
             Phase 6 (US4)
                   |
                   v
             Phase 7 (Polish)
```

### Parallel Opportunities

**Within Phase 2 (Foundational)**:
- T006, T007, T008 can run in parallel (different constants)

**Within Phase 3 (US2)**:
- T011-T015 tests can run in parallel
- T017-T034 widget schemas can run in parallel (same file but independent match arms)

**Within Phase 4 (US1)**:
- T037, T038 tests can run in parallel

**Cross-Phase**:
- Phase 4 (US1) and Phase 5 (US3) can run in parallel after Phase 3 completes

---

## Parallel Example: Phase 3 Widget Schemas

```bash
# Launch all widget schema implementations together:
Task: "Add schema for Text widget in crates/dampen-core/src/schema/mod.rs"
Task: "Add schema for Image widget in crates/dampen-core/src/schema/mod.rs"
Task: "Add schema for Button widget in crates/dampen-core/src/schema/mod.rs"
# ... (all T017-T034 can be launched together)
```

---

## Implementation Strategy

### MVP First (US2 + US1)

1. Complete Phase 1: Setup (4 tasks)
2. Complete Phase 2: Foundational (6 tasks)
3. Complete Phase 3: US2 - Schema API (26 tasks)
4. Complete Phase 4: US1 - CLI Migration (12 tasks)
5. **STOP and VALIDATE**: Run `dampen check` on todo-app
6. Deploy/demo if ready

### Full Feature

1. MVP steps above
2. Add Phase 5: US3 - Documentation (8 tasks)
3. Add Phase 6: US4 - Compatibility verification (7 tasks)
4. Add Phase 7: Polish (7 tasks)

---

## Notes

- [P] tasks = different files or independent code sections
- [Story] label maps task to specific user story
- All tests written FIRST per TDD (Constitution Principle V)
- Commit after each logical group of tasks
- Run `cargo test --workspace` frequently to catch regressions
- Keep existing CLI test logic unchanged - only imports may change
