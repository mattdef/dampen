# Tasks: Inter-Window Communication

**Input**: Design documents from `/specs/001-inter-window-communication/`
**Prerequisites**: plan.md ✓, spec.md ✓, research.md ✓, data-model.md ✓, contracts/ ✓

**Tests**: TDD is mandatory per Constitution Principle V. Contract tests included.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (US1-US6)
- Include exact file paths in descriptions

## Path Conventions

- **Workspace root**: `crates/dampen-core/`, `crates/dampen-macros/`, `crates/dampen-iced/`, `crates/dampen-cli/`
- **Examples**: `examples/shared-state/`
- **Tests**: `crates/{crate}/tests/`, `tests/`

---

## Phase 1: Setup

**Purpose**: Create new modules and establish project structure for shared state

- [ ] T001 Create shared module directory at `crates/dampen-core/src/shared/`
- [ ] T002 [P] Create `crates/dampen-core/src/shared/mod.rs` with module structure and exports
- [ ] T003 [P] Add `pub mod shared;` export to `crates/dampen-core/src/lib.rs`
- [ ] T004 [P] Create example project structure at `examples/shared-state/Cargo.toml`
- [ ] T005 [P] Create test directory structure at `tests/contract/`, `tests/integration/`, `tests/parity/`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST be complete before ANY user story can be implemented

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

### SharedContext<S> Implementation

- [ ] T006 Implement `SharedContext<S>` struct with `Arc<RwLock<S>>` in `crates/dampen-core/src/shared/mod.rs`
- [ ] T007 Implement `SharedContext::new()`, `read()`, `write()` methods in `crates/dampen-core/src/shared/mod.rs`
- [ ] T008 [P] Implement `SharedContext::try_read()`, `try_write()` methods in `crates/dampen-core/src/shared/mod.rs`
- [ ] T009 [P] Implement `Clone` for `SharedContext<S>` in `crates/dampen-core/src/shared/mod.rs`
- [ ] T010 [P] Implement `SharedContext::<()>::empty()` for no-op context in `crates/dampen-core/src/shared/mod.rs`

### Unit Tests for SharedContext

- [ ] T011 [P] Test `SharedContext` read/write access in `crates/dampen-core/src/shared/tests.rs`
- [ ] T012 [P] Test `SharedContext` clone shares state in `crates/dampen-core/src/shared/tests.rs`
- [ ] T013 [P] Test `SharedContext` thread safety with concurrent access in `crates/dampen-core/src/shared/tests.rs`

### HandlerEntry Extension

- [ ] T014 Add `WithShared` variant to `HandlerEntry` enum in `crates/dampen-core/src/handler/mod.rs`
- [ ] T015 [P] Add `WithValueAndShared` variant to `HandlerEntry` enum in `crates/dampen-core/src/handler/mod.rs`
- [ ] T016 [P] Add `WithCommandAndShared` variant to `HandlerEntry` enum in `crates/dampen-core/src/handler/mod.rs`

### HandlerRegistry Extension

- [ ] T017 Implement `register_with_shared()` method in `crates/dampen-core/src/handler/mod.rs`
- [ ] T018 [P] Implement `register_with_value_and_shared()` method in `crates/dampen-core/src/handler/mod.rs`
- [ ] T019 [P] Implement `register_with_command_and_shared()` method in `crates/dampen-core/src/handler/mod.rs`
- [ ] T020 Implement `dispatch_with_shared()` method with all variant handling in `crates/dampen-core/src/handler/mod.rs`

### AppState<M, S> Extension

- [ ] T021 Add second generic parameter `S` to `AppState<M, S>` with default `()` in `crates/dampen-core/src/state/mod.rs`
- [ ] T022 Add `shared_context: Option<SharedContext<S>>` field to `AppState` in `crates/dampen-core/src/state/mod.rs`
- [ ] T023 Implement `AppState::with_shared()` constructor in `crates/dampen-core/src/state/mod.rs`
- [ ] T024 [P] Implement `AppState::shared()` and `shared_mut()` accessors in `crates/dampen-core/src/state/mod.rs`
- [ ] T025 Verify existing `AppState` constructors still compile (backward compatibility) in `crates/dampen-core/tests/appstate_tests.rs`

### BindingExpr Extension

- [ ] T026 Add `SharedFieldAccess(Vec<String>)` variant to `BindingExpr` enum in `crates/dampen-core/src/expr/ast.rs`
- [ ] T027 [P] Implement `BindingExpr::uses_shared()` method in `crates/dampen-core/src/expr/ast.rs`

**Checkpoint**: Foundation ready - user story implementation can now begin

---

## Phase 3: User Story 1 - Share User Preferences Across Views (Priority: P1) 🎯 MVP

**Goal**: Enable shared state that persists user preferences (theme, language) across all views

**Independent Test**: Create two-view app where changing preference in View A immediately updates View B

### Contract Tests for US1

- [ ] T028 [P] [US1] Contract test: SharedContext changes visible across clones in `tests/contract/shared_state_contracts.rs`
- [ ] T029 [P] [US1] Contract test: Handler modifies shared state, all views see change in `tests/contract/shared_state_contracts.rs`

### Implementation for US1

- [ ] T030 [US1] Create `SharedState` struct for example in `examples/shared-state/src/shared.rs`
- [ ] T031 [P] [US1] Create window view with `{shared.theme}` binding in `examples/shared-state/src/ui/window.dampen`
- [ ] T032 [P] [US1] Create settings view with theme toggle in `examples/shared-state/src/ui/settings.dampen`
- [ ] T033 [US1] Implement window.rs with shared-aware AppState in `examples/shared-state/src/ui/window.rs`
- [ ] T034 [US1] Implement settings.rs with `register_with_shared()` handlers in `examples/shared-state/src/ui/settings.rs`
- [ ] T035 [US1] Create ui/mod.rs exporting both views in `examples/shared-state/src/ui/mod.rs`
- [ ] T036 [US1] Create main.rs wiring up SharedContext across views in `examples/shared-state/src/main.rs`
- [ ] T037 [US1] Verify theme change in settings immediately reflects in window view (manual test)

**Checkpoint**: User Story 1 complete - shared preferences work across views

---

## Phase 4: User Story 2 - Display Shared Data in XML Bindings (Priority: P1)

**Goal**: Enable `{shared.field}` syntax in XML to declaratively display shared state values

**Independent Test**: XML with `{shared.user.name}` binding displays correct value

### Contract Tests for US2

- [ ] T038 [P] [US2] Contract test CT-SB-001: Simple shared binding renders value in `tests/contract/shared_state_contracts.rs`
- [ ] T039 [P] [US2] Contract test CT-SB-002: Nested shared binding resolves correctly in `tests/contract/shared_state_contracts.rs`
- [ ] T040 [P] [US2] Contract test CT-SB-003: Missing field returns empty string in `tests/contract/shared_state_contracts.rs`
- [ ] T041 [P] [US2] Contract test CT-SB-004: Mixed bindings (model + shared) work together in `tests/contract/shared_state_contracts.rs`

### Implementation for US2

- [ ] T042 [US2] Extend expression parser to recognize `shared.` prefix in `crates/dampen-core/src/expr/parser.rs`
- [ ] T043 [US2] Create `SharedFieldAccess` AST nodes during parsing in `crates/dampen-core/src/expr/parser.rs`
- [ ] T044 [US2] Extend `DampenWidgetBuilder` with shared_context field in `crates/dampen-iced/src/builder.rs`
- [ ] T045 [US2] Implement `evaluate_binding()` to resolve `{shared.}` expressions in `crates/dampen-iced/src/builder.rs`
- [ ] T046 [US2] Handle missing shared fields gracefully (return empty string) in `crates/dampen-iced/src/builder.rs`
- [ ] T047 [P] [US2] Add dev-mode warning for missing shared bindings in `crates/dampen-iced/src/builder.rs`

**Checkpoint**: User Story 2 complete - shared bindings work in XML

---

## Phase 5: User Story 3 - Modify Shared State from Event Handlers (Priority: P1)

**Goal**: Enable event handlers to access and modify shared state

**Independent Test**: Handler increments shared counter, counter value updates globally

### Contract Tests for US3

- [ ] T048 [P] [US3] Contract test CT-HA-001: Simple handler still works in `tests/contract/shared_state_contracts.rs`
- [ ] T049 [P] [US3] Contract test CT-HA-002: WithShared handler receives shared context in `tests/contract/shared_state_contracts.rs`
- [ ] T050 [P] [US3] Contract test CT-HA-003: WithValueAndShared receives all parameters in `tests/contract/shared_state_contracts.rs`
- [ ] T051 [P] [US3] Contract test CT-HA-005: Shared state changes persist across views in `tests/contract/shared_state_contracts.rs`

### Implementation for US3

- [ ] T052 [US3] Update Iced backend to call `dispatch_with_shared()` in `crates/dampen-iced/src/lib.rs`
- [ ] T053 [US3] Pass SharedContext to dispatch in generated app `update()` method in `crates/dampen-macros/src/dampen_app.rs`
- [ ] T054 [US3] Add integration test: handler modifies shared, other view sees change in `tests/integration/shared_state_e2e.rs`

**Checkpoint**: User Story 3 complete - handlers can modify shared state

---

## Phase 6: User Story 4 - Preserve Shared State During Hot-Reload (Priority: P2)

**Goal**: Shared state persists when XML files are hot-reloaded during development

**Independent Test**: Modify XML while app running, verify shared state unchanged

### Contract Tests for US4

- [ ] T055 [P] [US4] Contract test: SharedContext survives document replacement in `tests/contract/shared_state_contracts.rs`

### Implementation for US4

- [ ] T056 [US4] Verify hot-reload only replaces `document` field, not `shared_context` in `crates/dampen-iced/src/lib.rs`
- [ ] T057 [US4] Add integration test: modify XML, verify shared state preserved in `tests/integration/shared_state_e2e.rs`

**Checkpoint**: User Story 4 complete - hot-reload preserves shared state

---

## Phase 7: User Story 5 - Opt-In Shared State Configuration (Priority: P2)

**Goal**: Shared state is opt-in via `shared_model` attribute, existing apps unchanged

**Independent Test**: Run hello-world, counter, todo-app examples without modification

### Contract Tests for US5

- [ ] T058 [P] [US5] Contract test: AppState without shared_context works (backward compat) in `tests/contract/shared_state_contracts.rs`
- [ ] T059 [P] [US5] Contract test: existing handlers work via dispatch_with_shared in `tests/contract/shared_state_contracts.rs`

### Implementation for US5

- [ ] T060 [US5] Parse `shared_model` attribute in `#[dampen_app]` macro in `crates/dampen-macros/src/dampen_app.rs`
- [ ] T061 [US5] Generate `SharedContext::new()` initialization when `shared_model` present in `crates/dampen-macros/src/dampen_app.rs`
- [ ] T062 [US5] Generate `SharedContext::<()>::empty()` when `shared_model` absent in `crates/dampen-macros/src/dampen_app.rs`
- [ ] T063 [US5] Verify hello-world example compiles and runs unchanged in `examples/hello-world/`
- [ ] T064 [P] [US5] Verify counter example compiles and runs unchanged in `examples/counter/`
- [ ] T065 [P] [US5] Verify todo-app example compiles and runs unchanged in `examples/todo-app/`
- [ ] T066 [P] [US5] Verify settings example compiles and runs unchanged in `examples/settings/`

**Checkpoint**: User Story 5 complete - 100% backward compatible

---

## Phase 8: User Story 6 - Parity Between Interpreted and Codegen Modes (Priority: P2)

**Goal**: Shared state works identically in interpreted (dev) and codegen (prod) modes

**Independent Test**: Run same app in both modes, compare output for identical inputs

### Contract Tests for US6

- [ ] T067 [P] [US6] Parity test: shared binding in interpreted mode in `tests/parity/shared_mode_parity.rs`
- [ ] T068 [P] [US6] Parity test: shared binding in codegen mode in `tests/parity/shared_mode_parity.rs`
- [ ] T069 [US6] Parity test: compare outputs for identical state in `tests/parity/shared_mode_parity.rs`

### Implementation for US6

- [ ] T070 [US6] Implement codegen for `{shared.}` bindings in `crates/dampen-core/src/codegen/mod.rs`
- [ ] T071 [US6] Generate lock acquisition at view start in codegen in `crates/dampen-core/src/codegen/mod.rs`
- [ ] T072 [US6] Generate lock release before widget tree construction in `crates/dampen-core/src/codegen/mod.rs`

**Checkpoint**: User Story 6 complete - interpreted/codegen parity verified

---

## Phase 9: Polish & Cross-Cutting Concerns

**Purpose**: Documentation, cleanup, and performance validation

### Documentation

- [ ] T073 [P] Update `docs/USAGE.md` with shared state section
- [ ] T074 [P] Update `docs/XML_SCHEMA.md` with `{shared.}` binding documentation
- [ ] T075 [P] Add CHANGELOG.md entry for v0.2.4 inter-window communication
- [ ] T076 Update AGENTS.md with completed feature status

### Code Quality

- [ ] T077 Run `cargo clippy --workspace -- -D warnings` and fix any warnings
- [ ] T078 [P] Run `cargo fmt --all` to ensure formatting
- [ ] T079 [P] Ensure all public items have rustdoc comments
- [ ] T080 Run `cargo test --workspace` to verify all tests pass

### Performance Validation

- [ ] T081 [P] Benchmark shared state access time (<1ms target) in `benches/shared_state_bench.rs`
- [ ] T082 [P] Verify memory overhead <5% compared to baseline

### Final Validation

- [ ] T083 Run quickstart.md scenario end-to-end with shared-state example
- [ ] T084 Verify all success criteria (SC-001 through SC-008) from spec.md

---

## Dependencies & Execution Order

### Phase Dependencies

```
Phase 1 (Setup)
    ↓
Phase 2 (Foundational) ← BLOCKS ALL USER STORIES
    ↓
┌───┴───┬───────┬───────┐
↓       ↓       ↓       ↓
US1     US2     US3   (can run in parallel after Phase 2)
(P1)    (P1)    (P1)
↓       ↓       ↓
└───┬───┴───────┘
    ↓
┌───┴───┬───────┐
↓       ↓       ↓
US4     US5     US6   (can run in parallel, may depend on P1 stories)
(P2)    (P2)    (P2)
↓       ↓       ↓
└───┬───┴───────┘
    ↓
Phase 9 (Polish)
```

### User Story Dependencies

| Story | Depends On | Can Parallelize With |
|-------|------------|---------------------|
| US1 (P1) | Phase 2 | US2, US3 |
| US2 (P1) | Phase 2 | US1, US3 |
| US3 (P1) | Phase 2 | US1, US2 |
| US4 (P2) | US1 | US5, US6 |
| US5 (P2) | Phase 2 | US4, US6 |
| US6 (P2) | US2 | US4, US5 |

### Within Each User Story

1. Tests MUST be written and FAIL before implementation
2. Core implementation before integration
3. Verify tests PASS after implementation
4. Commit after each task or logical group

### Parallel Opportunities

**Phase 2 (Foundational)**:
```bash
# Can run in parallel:
T011, T012, T013  # SharedContext tests
T015, T016        # Handler variants
T018, T019        # Registry methods
T024              # AppState accessors
T027              # BindingExpr method
```

**Phase 4 (US2)**:
```bash
# Can run in parallel:
T038, T039, T040, T041  # All contract tests
T047                     # Dev-mode warning
```

**Phase 7 (US5)**:
```bash
# Can run in parallel:
T063, T064, T065, T066  # Backward compat verification
```

---

## Implementation Strategy

### MVP First (User Stories 1-3 Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational (CRITICAL - blocks all stories)
3. Complete Phase 3: User Story 1 (shared preferences)
4. Complete Phase 4: User Story 2 (XML bindings)
5. Complete Phase 5: User Story 3 (handler modification)
6. **STOP and VALIDATE**: Test shared-state example end-to-end
7. Deploy/demo if ready

### Incremental Delivery

1. Setup + Foundational → Foundation ready
2. Add US1 → Test independently → Demo (basic shared state!)
3. Add US2 → Test independently → Demo (XML bindings!)
4. Add US3 → Test independently → Demo (interactive handlers!)
5. Add US4-US6 → Polish features
6. Each story adds value without breaking previous stories

### Parallel Team Strategy

With multiple developers after Phase 2:

- **Developer A**: User Story 1 (shared preferences example)
- **Developer B**: User Story 2 (XML binding infrastructure)
- **Developer C**: User Story 3 (handler dispatch)

---

## Notes

- [P] tasks = different files, no dependencies
- [Story] label maps task to specific user story for traceability
- Each user story should be independently completable and testable
- Verify tests fail before implementing (TDD per Constitution V)
- Commit after each task or logical group
- Stop at any checkpoint to validate story independently
- Avoid: vague tasks, same file conflicts, cross-story dependencies that break independence
