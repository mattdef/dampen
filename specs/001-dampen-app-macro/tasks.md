# Tasks: Auto-Discovery Multi-View Application with #[dampen_app] Macro

**Input**: Design documents from `/home/matt/Documents/Dev/dampen/specs/001-dampen-app-macro/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/macro-api.md, quickstart.md

**Tests**: Following TDD (Test-First Development) as required by Dampen Constitution Principle V. All test tasks MUST be completed and FAIL before implementation.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `- [ ] [ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2)
- Include exact file paths in descriptions

## Path Conventions

Multi-crate workspace structure (from plan.md):
- **Macro crate**: `crates/dampen-macros/src/`
- **Tests**: `crates/dampen-macros/tests/`
- **Fixtures**: `crates/dampen-macros/tests/fixtures/`
- **Integration**: `tests/integration/`
- **Examples**: `examples/widget-showcase/`

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and dependency configuration

- [ ] T001 Add `glob = "0.3.3"` dependency to crates/dampen-macros/Cargo.toml
- [ ] T002 Add `trybuild = "1.0"` dev-dependency to crates/dampen-macros/Cargo.toml
- [ ] T003 [P] Create test fixtures directory structure at crates/dampen-macros/tests/fixtures/
- [ ] T004 [P] Create trybuild UI tests directory at crates/dampen-macros/tests/ui/

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core data structures and utilities that ALL user stories depend on

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

### Data Structures (from data-model.md)

- [ ] T005 [P] Create ViewInfo struct in crates/dampen-macros/src/discovery.rs with fields: view_name, variant_name, field_name, module_path, dampen_file, rs_file
- [ ] T006 [P] Create MacroAttributes struct in crates/dampen-macros/src/dampen_app.rs with fields: ui_dir, message_type, handler_variant, hot_reload_variant, dismiss_error_variant, exclude
- [ ] T007 [P] Implement to_pascal_case helper function in crates/dampen-macros/src/discovery.rs for snake_case to PascalCase conversion
- [ ] T008 [P] Implement Display trait for ViewInfo in crates/dampen-macros/src/discovery.rs for debugging

### Attribute Parsing (R3 research - syn::parse_nested_meta)

- [ ] T009 Implement MacroAttributes::parse() in crates/dampen-macros/src/dampen_app.rs using syn::parse_nested_meta()
- [ ] T010 Add validation for required attributes (ui_dir, message_type, handler_variant) in MacroAttributes::parse()
- [ ] T011 Add validation for optional attributes (hot_reload_variant, dismiss_error_variant, exclude) in MacroAttributes::parse()

### File System Access (R1 research)

- [ ] T012 Implement resolve_ui_dir() function in crates/dampen-macros/src/discovery.rs using std::env::var("CARGO_MANIFEST_DIR")
- [ ] T013 Add ui_dir existence validation in resolve_ui_dir() with clear error messages

**Checkpoint**: Foundation ready - user story implementation can now begin in parallel

---

## Phase 3: User Story 1 - Automatic View Discovery and Initialization (Priority: P1) 🎯 MVP

**Goal**: Automatically discover all `.dampen` UI files and generate necessary initialization code, eliminating manual view registration

**Independent Test**: Create a multi-view application with 3-5 `.dampen` files in `src/ui/`, apply `#[dampen_app]` macro, compile, and verify all views are discoverable with generated `CurrentView` enum and `AppState` fields

**Acceptance Scenarios**:
1. 3 `.dampen` files → generates `CurrentView` enum with 3 variants + app struct with typed `AppState` fields
2. Nested UI files (e.g., `src/ui/widgets/button/button.dampen`) → discovers and generates appropriate module paths
3. 20 `.dampen` files → compilation completes in <5 seconds (discovery overhead < 200ms)

### Tests for User Story 1 (TDD - WRITE FIRST, ENSURE THEY FAIL)

- [ ] T014 [P] [US1] Unit test for discover_dampen_files() with flat structure fixture in crates/dampen-macros/tests/dampen_app_tests.rs
- [ ] T015 [P] [US1] Unit test for discover_dampen_files() with nested structure fixture in crates/dampen-macros/tests/dampen_app_tests.rs
- [ ] T016 [P] [US1] Unit test for ViewInfo::from_path() field derivation in crates/dampen-macros/tests/dampen_app_tests.rs
- [ ] T017 [P] [US1] Unit test for ViewInfo validation (VR-001: valid Rust identifier) in crates/dampen-macros/tests/dampen_app_tests.rs
- [ ] T018 [P] [US1] Unit test for ViewInfo validation (VR-002: unique variant names) in crates/dampen-macros/tests/dampen_app_tests.rs
- [ ] T019 [P] [US1] Unit test for ViewInfo validation (VR-003: .rs file exists) in crates/dampen-macros/tests/dampen_app_tests.rs
- [ ] T020 [P] [US1] Snapshot test for generated CurrentView enum in crates/dampen-macros/tests/dampen_app_tests.rs using insta crate
- [ ] T021 [P] [US1] Snapshot test for generated app struct fields in crates/dampen-macros/tests/dampen_app_tests.rs using insta crate
- [ ] T022 [P] [US1] Snapshot test for generated init() method in crates/dampen-macros/tests/dampen_app_tests.rs using insta crate
- [ ] T023 [P] [US1] Create test fixture multi_view with 3 views in crates/dampen-macros/tests/fixtures/multi_view/
- [ ] T024 [P] [US1] Create test fixture nested_views with nested structure in crates/dampen-macros/tests/fixtures/nested_views/

### Implementation for User Story 1 (File Discovery)

- [ ] T025 [US1] Implement discover_dampen_files() in crates/dampen-macros/src/discovery.rs using walkdir (R2 research)
- [ ] T026 [US1] Add .dampen extension filtering in discover_dampen_files()
- [ ] T027 [US1] Add alphabetical sorting for deterministic behavior (FR-016) in discover_dampen_files()
- [ ] T028 [US1] Implement ViewInfo::from_path() constructor in crates/dampen-macros/src/discovery.rs

### Implementation for User Story 1 (ViewInfo Validation)

- [ ] T029 [US1] Implement VR-001 validation (valid Rust identifier) in crates/dampen-macros/src/discovery.rs
- [ ] T030 [US1] Implement VR-002 validation (unique variant names) in crates/dampen-macros/src/discovery.rs
- [ ] T031 [US1] Implement VR-003 validation (.rs file exists) in crates/dampen-macros/src/discovery.rs

### Implementation for User Story 1 (Code Generation)

- [ ] T032 [US1] Implement generate_current_view_enum() in crates/dampen-macros/src/dampen_app.rs using quote! (FR-003)
- [ ] T033 [US1] Implement generate_app_struct() in crates/dampen-macros/src/dampen_app.rs to generate AppState fields (FR-004)
- [ ] T034 [US1] Implement generate_init_method() in crates/dampen-macros/src/dampen_app.rs (FR-005)
- [ ] T035 [US1] Implement generate_new_method() in crates/dampen-macros/src/dampen_app.rs as alias to init()
- [ ] T036 [US1] Implement main dampen_app proc macro entry point in crates/dampen-macros/src/dampen_app.rs

### Implementation for User Story 1 (Integration)

- [ ] T037 [US1] Export #[dampen_app] macro in crates/dampen-macros/src/lib.rs
- [ ] T038 [US1] Add integration test with 3 views in crates/dampen-macros/tests/dampen_app_tests.rs
- [ ] T039 [US1] Verify all snapshot tests pass for User Story 1

**Checkpoint**: At this point, User Story 1 should be fully functional - views are discovered, CurrentView enum generated, AppState fields created, init() works

---

## Phase 4: User Story 2 - View Switching Without Manual Routing (Priority: P1)

**Goal**: View switching works automatically via generated handlers, eliminating manual routing logic in update() method

**Independent Test**: Create a multi-view app with `switch_to_*` handlers, trigger view switches via button clicks, verify current view changes correctly without manual `match` statements in user code

**Acceptance Scenarios**:
1. App with views `window` and `settings` → button triggers `switch_to_settings` → view changes to settings
2. Current view is `settings` → trigger `switch_to_window` → app switches back to window view
3. App with 20 views → view transition completes in <100ms

### Tests for User Story 2 (TDD - WRITE FIRST, ENSURE THEY FAIL)

- [ ] T040 [P] [US2] Snapshot test for generated update() method with handler dispatch in crates/dampen-macros/tests/dampen_app_tests.rs
- [ ] T041 [P] [US2] Snapshot test for generated view() method with view rendering in crates/dampen-macros/tests/dampen_app_tests.rs
- [ ] T042 [P] [US2] Snapshot test for generated switch_to_*() helper methods in crates/dampen-macros/tests/dampen_app_tests.rs
- [ ] T043 [P] [US2] Integration test for view switching logic with 2 views in crates/dampen-macros/tests/dampen_app_tests.rs

### Implementation for User Story 2 (Code Generation)

- [ ] T044 [US2] Implement generate_update_method() in crates/dampen-macros/src/dampen_app.rs with handler dispatch (FR-006)
- [ ] T045 [US2] Implement generate_view_method() in crates/dampen-macros/src/dampen_app.rs to render current view (FR-007)
- [ ] T046 [US2] Implement generate_switch_methods() in crates/dampen-macros/src/dampen_app.rs to create switch_to_*() helpers
- [ ] T047 [US2] Add match arms for current_view in update() method generation
- [ ] T048 [US2] Add match arms for current_view in view() method generation

### Implementation for User Story 2 (Integration)

- [ ] T049 [US2] Add integration test with view switching between 2 views in crates/dampen-macros/tests/dampen_app_tests.rs
- [ ] T050 [US2] Verify all snapshot tests pass for User Story 2

**Checkpoint**: At this point, User Stories 1 AND 2 should both work - views discovered AND view switching works without manual routing

---

## Phase 5: User Story 3 - Hot-Reload for Multi-View Apps (Priority: P2)

**Goal**: Hot-reload works seamlessly across all discovered views for fast iteration without restart

**Independent Test**: Run app in dev mode, edit a `.dampen` file, verify view updates in running application without restart

**Acceptance Scenarios**:
1. App running with 5 views → developer edits one `.dampen` file → only that view reloads (not entire app), change visible within 500ms
2. Running app → developer adds new `.dampen` file → system detects recompilation needed and provides feedback
3. App with error in `.dampen` file → hot-reload triggers → error overlay displays parse error with file path and line number, dismissible via `DismissError` message

### Tests for User Story 3 (TDD - WRITE FIRST, ENSURE THEY FAIL)

- [ ] T051 [P] [US3] Snapshot test for generated subscription() method in crates/dampen-macros/tests/dampen_app_tests.rs
- [ ] T052 [P] [US3] Snapshot test for hot-reload handling in update() method in crates/dampen-macros/tests/dampen_app_tests.rs
- [ ] T053 [P] [US3] Snapshot test for error overlay handling in view() method in crates/dampen-macros/tests/dampen_app_tests.rs
- [ ] T054 [P] [US3] Snapshot test for DismissError handling in update() method in crates/dampen-macros/tests/dampen_app_tests.rs

### Implementation for User Story 3 (Code Generation)

- [ ] T055 [US3] Implement generate_subscription_method() in crates/dampen-macros/src/dampen_app.rs (FR-008)
- [ ] T056 [US3] Add hot_reload_variant conditional generation in generate_subscription_method()
- [ ] T057 [US3] Add #[cfg(debug_assertions)] gating for hot-reload code
- [ ] T058 [US3] Add HotReload message handling in generate_update_method() (FR-019, FR-020)
- [ ] T059 [US3] Add error_overlay field to generated app struct when dismiss_error_variant specified
- [ ] T060 [US3] Add error overlay rendering in generate_view_method()
- [ ] T061 [US3] Add DismissError message handling in generate_update_method()

### Implementation for User Story 3 (Integration)

- [ ] T062 [US3] Verify all snapshot tests pass for User Story 3

**Checkpoint**: All P1 and P2 user stories should now be independently functional - discovery, routing, and hot-reload all work

---

## Phase 6: User Story 4 - Selective View Exclusion (Priority: P3)

**Goal**: Exclude certain `.dampen` files from auto-discovery (e.g., experimental or debug views) for fine-grained control

**Independent Test**: Add `exclude = ["debug_view"]` to macro attributes, verify excluded view doesn't appear in generated `CurrentView` enum, app compiles successfully

**Acceptance Scenarios**:
1. Project with `debug_view.dampen` → add `exclude = ["debug_view"]` → `CurrentView` enum does not contain `DebugView` variant
2. `exclude = ["experimental/*"]` pattern → create `experimental/new_feature.dampen` → excluded from discovery
3. Excluded view → developer removes from exclusion list → recompilation generates appropriate variant and fields

### Tests for User Story 4 (TDD - WRITE FIRST, ENSURE THEY FAIL)

- [ ] T063 [P] [US4] Unit test for glob pattern matching with single file exclusion in crates/dampen-macros/tests/dampen_app_tests.rs
- [ ] T064 [P] [US4] Unit test for glob pattern matching with directory wildcard in crates/dampen-macros/tests/dampen_app_tests.rs
- [ ] T065 [P] [US4] Unit test for invalid glob pattern error in crates/dampen-macros/tests/dampen_app_tests.rs
- [ ] T066 [P] [US4] Integration test for exclusion filtering in crates/dampen-macros/tests/dampen_app_tests.rs

### Implementation for User Story 4 (Exclusion Logic)

- [ ] T067 [US4] Implement is_excluded() function in crates/dampen-macros/src/discovery.rs using glob::Pattern (R4 research - FR-011)
- [ ] T068 [US4] Add exclusion filtering in discover_dampen_files() before ViewInfo creation
- [ ] T069 [US4] Add glob pattern validation in MacroAttributes::parse() (VAR-003)
- [ ] T070 [US4] Add error reporting for invalid glob patterns with suggestions

### Implementation for User Story 4 (Integration)

- [ ] T071 [US4] Verify all tests pass for User Story 4

**Checkpoint**: All user stories (P1, P2, P3) should now be independently functional

---

## Phase 7: User Story 5 - Clear Compile-Time Error Messages (Priority: P2)

**Goal**: Macro provides clear, actionable error messages when conventions are violated, preventing developer frustration

**Independent Test**: Intentionally create violations (e.g., `.dampen` file without matching `.rs` file), attempt to compile, verify error messages include file paths, problem descriptions, and suggested fixes

**Acceptance Scenarios**:
1. `.dampen` file without matching `.rs` file → compiler error shows file path, states "No matching Rust module found", suggests creating `.rs` file or excluding it
2. `ui_dir = "src/nonexistent"` → error states "UI directory does not exist" with suggestion to check path
3. Two `.dampen` files with same name in different directories → compilation fails with "View naming conflict" listing both paths

### Tests for User Story 5 (TDD - trybuild compile-fail tests)

- [ ] T072 [P] [US5] Compile-fail test for missing required attribute (E1) in crates/dampen-macros/tests/ui/missing_required_attr.rs
- [ ] T073 [P] [US5] Compile-fail test for invalid ui_dir (E2) in crates/dampen-macros/tests/ui/invalid_ui_dir.rs
- [ ] T074 [P] [US5] Compile-fail test for missing .rs file (E3) in crates/dampen-macros/tests/ui/missing_rs_file.rs
- [ ] T075 [P] [US5] Compile-fail test for view naming conflict (E4) in crates/dampen-macros/tests/ui/naming_conflict.rs
- [ ] T076 [P] [US5] Compile-fail test for invalid view name (E5) in crates/dampen-macros/tests/ui/invalid_view_name.rs
- [ ] T077 [P] [US5] Compile-fail test for invalid glob pattern (E6) in crates/dampen-macros/tests/ui/invalid_glob_pattern.rs
- [ ] T078 [P] [US5] Compile-warning test for no views discovered (E7) in crates/dampen-macros/tests/ui/no_views_warning.rs
- [ ] T079 [US5] Create trybuild test harness in crates/dampen-macros/tests/dampen_app_tests.rs

### Implementation for User Story 5 (Error Reporting)

- [ ] T080 [US5] Implement error reporting for missing .rs file (E3) in crates/dampen-macros/src/discovery.rs with file paths and suggestions (FR-012)
- [ ] T081 [US5] Implement error reporting for invalid ui_dir (E2) in resolve_ui_dir() with suggestions (FR-013)
- [ ] T082 [US5] Implement error reporting for naming conflicts (E4) in crates/dampen-macros/src/discovery.rs with file paths (FR-014)
- [ ] T083 [US5] Implement warning for no views discovered (E7) in crates/dampen-macros/src/discovery.rs (FR-015)
- [ ] T084 [US5] Implement error reporting for invalid view name (E5) in ViewInfo validation
- [ ] T085 [US5] Implement error reporting for invalid glob pattern (E6) in MacroAttributes::parse()

### Implementation for User Story 5 (Integration)

- [ ] T086 [US5] Verify all trybuild compile-fail tests pass with expected error messages
- [ ] T087 [US5] Verify error message format matches contracts/macro-api.md specification

**Checkpoint**: All user stories should now be complete with comprehensive error handling

---

## Phase 8: Integration & Migration (widget-showcase)

**Purpose**: Validate macro works with real-world application (20 views)

**Success Criteria**: SC-001 (85% boilerplate reduction), SC-002 (discovery < 200ms), SC-007 (zero runtime overhead), SC-008 (successful migration)

- [ ] T088 Create backup of examples/widget-showcase/src/main.rs before migration
- [ ] T089 Apply #[dampen_app] macro to widget-showcase in examples/widget-showcase/src/main.rs
- [ ] T090 Remove manual CurrentView enum from examples/widget-showcase/src/main.rs
- [ ] T091 Remove manual app struct fields from examples/widget-showcase/src/main.rs
- [ ] T092 Remove manual init() method from examples/widget-showcase/src/main.rs
- [ ] T093 Remove manual update() method from examples/widget-showcase/src/main.rs
- [ ] T094 Remove manual view() method from examples/widget-showcase/src/main.rs
- [ ] T095 Remove manual subscription() method from examples/widget-showcase/src/main.rs
- [ ] T096 Update Iced Application trait impl to call generated methods in examples/widget-showcase/src/main.rs
- [ ] T097 Compile widget-showcase and verify zero errors
- [ ] T098 Run widget-showcase and verify all 20 views render correctly
- [ ] T099 Test view switching between all 20 views
- [ ] T100 Test hot-reload functionality in widget-showcase
- [ ] T101 Measure compilation time overhead (MUST be < 200ms for 20 views per SC-002)
- [ ] T102 Count lines of code before and after (MUST achieve ~85% reduction per SC-001)
- [ ] T103 Create integration test in tests/integration/macro_integration_tests.rs for widget-showcase migration

**Checkpoint**: widget-showcase successfully migrated, all views work, performance targets met

---

## Phase 9: Polish & Cross-Cutting Concerns

**Purpose**: Improvements that affect multiple user stories, final quality checks

- [ ] T104 [P] Add rustdoc comments for #[dampen_app] macro in crates/dampen-macros/src/dampen_app.rs
- [ ] T105 [P] Add rustdoc comments for ViewInfo in crates/dampen-macros/src/discovery.rs
- [ ] T106 [P] Add rustdoc comments for MacroAttributes in crates/dampen-macros/src/dampen_app.rs
- [ ] T107 [P] Add rustdoc comments for all public functions in discovery.rs
- [ ] T108 [P] Update docs/USAGE.md with #[dampen_app] macro usage section
- [ ] T109 [P] Create docs/migration/multi-view-macro.md migration guide
- [ ] T110 Run cargo clippy --workspace -- -D warnings and fix all warnings
- [ ] T111 Run cargo fmt --all and verify formatting
- [ ] T112 Run cargo test --workspace and verify all tests pass
- [ ] T113 Validate quickstart.md example compiles and runs correctly
- [ ] T114 Run final benchmark suite and document performance metrics
- [ ] T115 Review all generated code for code quality and readability
- [ ] T116 Final review against requirements checklist in specs/001-dampen-app-macro/checklists/requirements.md

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories
- **User Stories (Phase 3-7)**: All depend on Foundational phase completion
  - User Story 1 (P1) can start after Foundational - No dependencies on other stories
  - User Story 2 (P1) DEPENDS ON User Story 1 (needs view discovery and CurrentView enum)
  - User Story 3 (P2) DEPENDS ON User Story 2 (needs update/view methods)
  - User Story 4 (P3) can start after Foundational - Independent of US2/US3
  - User Story 5 (P2) can start after Foundational - Independent (error handling)
- **Integration (Phase 8)**: Depends on User Stories 1, 2, 3 being complete
- **Polish (Phase 9)**: Depends on all user stories and integration being complete

### User Story Dependencies

- **User Story 1 (P1)**: File discovery and initialization - Foundation for all other stories
- **User Story 2 (P1)**: View switching - DEPENDS ON US1 (needs CurrentView enum and AppState fields)
- **User Story 3 (P2)**: Hot-reload - DEPENDS ON US2 (needs update/view methods to extend)
- **User Story 4 (P3)**: Exclusions - Independent, can run in parallel with US2/US3 after US1
- **User Story 5 (P2)**: Error messages - Independent, can run in parallel with US2/US3/US4

### Recommended Execution Order

1. **Phase 1**: Setup (T001-T004)
2. **Phase 2**: Foundational (T005-T013) - CRITICAL BLOCKER
3. **Phase 3**: User Story 1 (T014-T039) - MVP CORE
4. **Phase 4**: User Story 2 (T040-T050) - DEPENDS ON US1
5. **Phase 5**: User Story 3 (T051-T062) - DEPENDS ON US2
6. **Phase 6 & 7**: User Story 4 and 5 in parallel (T063-T087) - Independent
7. **Phase 8**: Integration (T088-T103) - Depends on US1, US2, US3
8. **Phase 9**: Polish (T104-T116)

### Parallel Opportunities

#### Within Foundational Phase (after T001-T004)
```bash
# Can run in parallel:
Task T005 (ViewInfo struct)
Task T006 (MacroAttributes struct)
Task T007 (to_pascal_case helper)
Task T008 (Display trait)
```

#### Within User Story 1 - Tests (after T013)
```bash
# All test creation can run in parallel:
Task T014 (flat structure test)
Task T015 (nested structure test)
Task T016 (field derivation test)
Task T017 (VR-001 test)
Task T018 (VR-002 test)
Task T019 (VR-003 test)
Task T020 (CurrentView enum snapshot)
Task T021 (app struct snapshot)
Task T022 (init method snapshot)
Task T023 (multi_view fixture)
Task T024 (nested_views fixture)
```

#### Within User Story 5 - Error Tests (after T079)
```bash
# All compile-fail tests can run in parallel:
Task T072 (missing_required_attr.rs)
Task T073 (invalid_ui_dir.rs)
Task T074 (missing_rs_file.rs)
Task T075 (naming_conflict.rs)
Task T076 (invalid_view_name.rs)
Task T077 (invalid_glob_pattern.rs)
Task T078 (no_views_warning.rs)
```

#### Within Polish Phase
```bash
# All documentation can run in parallel:
Task T104 (rustdoc for macro)
Task T105 (rustdoc for ViewInfo)
Task T106 (rustdoc for MacroAttributes)
Task T107 (rustdoc for functions)
Task T108 (USAGE.md update)
Task T109 (migration guide)
```

---

## Parallel Example: User Story 1 Tests

```bash
# Launch all test creation tasks for User Story 1 together:
Task: "Unit test for discover_dampen_files() with flat structure fixture"
Task: "Unit test for discover_dampen_files() with nested structure fixture"
Task: "Unit test for ViewInfo::from_path() field derivation"
Task: "Unit test for ViewInfo validation (VR-001: valid Rust identifier)"
Task: "Unit test for ViewInfo validation (VR-002: unique variant names)"
Task: "Unit test for ViewInfo validation (VR-003: .rs file exists)"
Task: "Snapshot test for generated CurrentView enum"
Task: "Snapshot test for generated app struct fields"
Task: "Snapshot test for generated init() method"
Task: "Create test fixture multi_view with 3 views"
Task: "Create test fixture nested_views with nested structure"
```

---

## Implementation Strategy

### MVP First (User Stories 1 + 2 Only)

**Justification**: US1 (view discovery) + US2 (view switching) provide core value proposition (85% boilerplate reduction, zero manual routing). This is the minimum viable product.

1. Complete Phase 1: Setup (T001-T004)
2. Complete Phase 2: Foundational (T005-T013) - CRITICAL
3. Complete Phase 3: User Story 1 (T014-T039) - File discovery working
4. Complete Phase 4: User Story 2 (T040-T050) - View switching working
5. **STOP and VALIDATE**: Test US1+US2 independently with small example
6. Deploy/demo if ready (core macro functionality complete)

### Incremental Delivery

1. **Foundation** (Phases 1-2) → Data structures and parsing ready
2. **MVP** (Phases 3-4) → US1+US2 complete → Discovery + Routing working → Demo!
3. **Enhanced** (Phase 5) → US3 complete → Hot-reload working → Demo!
4. **Feature Complete** (Phases 6-7) → US4+US5 complete → Exclusions + Errors → Demo!
5. **Production Ready** (Phases 8-9) → Integration + Polish → widget-showcase migrated → Release!

### Parallel Team Strategy

With multiple developers after Foundational phase:

**Team A (Critical Path)**:
- User Story 1 (T014-T039) - Must complete first
- Then User Story 2 (T040-T050) - Depends on US1
- Then User Story 3 (T051-T062) - Depends on US2

**Team B (Independent Work)**:
- User Story 4 (T063-T071) - Can start after Foundational
- User Story 5 (T072-T087) - Can start after Foundational

**Team A and B Merge**: Integration & Polish (T088-T116)

---

## TDD Workflow Checklist

For each user story phase:

### 1. RED Phase - Write Failing Tests
- [ ] Write all unit tests for the story (marked [P] can run in parallel)
- [ ] Write all snapshot tests for code generation
- [ ] Write all compile-fail tests (for US5)
- [ ] Run tests and verify they FAIL (if they pass, tests are wrong!)
- [ ] Commit failing tests: `git commit -m "test(US#): add failing tests for [story]"`

### 2. GREEN Phase - Implement Minimal Code
- [ ] Implement data structures
- [ ] Implement core logic to make tests pass
- [ ] Run tests frequently, fix until all GREEN
- [ ] Commit working implementation: `git commit -m "feat(US#): implement [story]"`

### 3. REFACTOR Phase - Improve Quality
- [ ] Extract common patterns
- [ ] Improve error messages
- [ ] Optimize performance if needed
- [ ] Run tests to ensure still GREEN
- [ ] Commit refactoring: `git commit -m "refactor(US#): improve [aspect]"`

---

## Success Metrics Validation

At Phase 8 completion, validate these success criteria:

- **SC-001**: Count LOC before/after widget-showcase migration → MUST achieve 85% reduction (500→<100 lines)
- **SC-002**: Measure compilation time with 20 views → Discovery overhead MUST be < 200ms
- **SC-003**: Verify adding new view requires only 2 files (`.dampen` + `.rs`) with zero wiring
- **SC-004**: Verify zero manual `match` statements for view switching in user code
- **SC-005**: Measure hot-reload latency → MUST be < 500ms per file change
- **SC-006**: Review all error messages → MUST include file paths + suggestions (100%)
- **SC-007**: Compare runtime performance → MUST be identical to manual code (zero overhead)
- **SC-008**: Confirm widget-showcase migration successful without behavior changes

---

## Notes

- **[P] tasks** = different files, no dependencies within phase
- **[Story] label** maps task to specific user story for traceability
- **TDD is mandatory** per Dampen Constitution Principle V
- Each user story should be independently completable and testable
- Verify tests FAIL before implementing (Red-Green-Refactor)
- Commit after each logical group of tasks
- Stop at any checkpoint to validate story independently
- All paths are relative to repository root: `/home/matt/Documents/Dev/dampen/`
