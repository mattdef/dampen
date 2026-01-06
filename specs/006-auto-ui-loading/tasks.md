# Implementation Tasks: Automatic UI File Loading with AppState Structure

**Feature**: 006-auto-ui-loading
**Branch**: `006-auto-ui-loading`
**Date**: 2026-01-06
**Spec**: [spec.md](spec.md)
**Plan**: [plan.md](plan.md)

## Dependencies

```
Phase 1 (Setup)
    │
    ▼
Phase 2 (Foundational)
    │
    ├─────────────────────────────────────────────────────────┐
    │                                                         │
    ▼                                                         ▼
Phase 3 (US1)                                           Phase 4 (US2)
Auto-loading mechanism                                  AppState struct
    │                                                         │
    │                                                         ▼
    │                                                 Phase 5 (US3)
    │                                                 Boilerplate reduction
    │                                                         │
    └─────────────────────────────────────────────────────────┤
                                                              │
                                                              ▼
                                                    Phase 6 (US4)
                                                    Multiple views
                                                              │
                                                              ▼
                                                    Phase 7 (Polish)
                                                    Tests & Migration
```

## Parallel Execution Opportunities

| Stories | Tasks | Reason |
|---------|-------|--------|
| US1, US2 | T003-T012, T013-T020 | Different crates (gravity-macros vs gravity-core), no shared dependencies |
| US3 | T021-T024 | Depends on US1+US2 completion |
| US4 | T025-T028 | Depends only on US1 (macro working) |

## Phase 1: Setup

**Goal**: Initialize project structure for the feature implementation

- [X] T001 Create `gravity-core/src/state/mod.rs` module file with basic module declaration
- [X] T002 Create `gravity-macros/src/ui_loader.rs` file for #[gravity_ui] macro
- [X] T003 Create `gravity-macros/build.rs` file for file discovery

## Phase 2: Foundational

**Goal**: Implement shared components that all user stories depend on

### HandlerRegistry Default Implementation

- [X] T004 [P] Add `Default` trait implementation for `HandlerRegistry` in `gravity-core/src/handler/mod.rs`

### ParseError Compile-Time Support

- [X] T005 [P] Add `to_compile_error()` method to `ParseError` in `gravity-core/src/parser/error.rs`

## Phase 3: User Story 1 - Auto-Loading Mechanism

**Priority**: P1
**Goal**: Automatically load `.gravity` files when corresponding `.gravity.rs` files are compiled
**Independent Test**: Run `gravity check` on a project with `ui/app.gravity` and `ui/app.gravity.rs` to verify XML is valid and document is generated

### Build.rs File Discovery

- [X] T006 [P] [US1] Implement file discovery in `gravity-macros/build.rs` to find all `.gravity` files in `ui/` directory
- [X] T007 [P] [US1] Add `cargo:rerun-if-changed` directives for discovered `.gravity` files in `gravity-macros/build.rs`

### #[gravity_ui] Macro

- [X] T008 [P] [US1] Implement `#[gravity_ui]` attribute macro in `gravity-macros/src/ui_loader.rs` with file path parsing
- [X] T009 [P] [US1] Add XML file existence validation with error code G001 in `gravity-macros/src/ui_loader.rs`
- [X] T010 [P] [US1] Add XML parsing with error code G002 in `gravity-macros/src/ui_loader.rs`
- [X] T011 [P] [US1] Generate `pub static document: GravityDocument` in macro output `gravity-macros/src/ui_loader.rs`
- [X] T012 [P] [US1] Add custom path support `#[gravity_ui(path = "...")]` in `gravity-macros/src/ui_loader.rs`

## Phase 4: User Story 2 - AppState Structure

**Priority**: P1
**Goal**: Define AppState with GravityDocument (mandatory), Model (optional), HandlerRegistry (optional)
**Independent Test**: Create AppState with only GravityDocument, another with Model, another with all three. Each should compile and work with GravityWidgetBuilder

### AppState Struct

- [X] T013 [P] [US2] Create `AppState<M: UiBindable = ()>` struct in `gravity-core/src/state/mod.rs`
- [X] T014 [P] [US2] Add `document: GravityDocument` field in `gravity-core/src/state/mod.rs`
- [X] T015 [P] [US2] Add `model: M` field with `PhantomData<M>` marker in `gravity-core/src/state/mod.rs`
- [X] T016 [P] [US2] Add `handler_registry: HandlerRegistry` field in `gravity-core/src/state/mod.rs`
- [X] T017 [P] [US2] Implement `AppState::new()` constructor in `gravity-core/src/state/mod.rs`
- [X] T018 [P] [US2] Implement `AppState::with_model()` constructor in `gravity-core/src/state/mod.rs`
- [X] T019 [P] [US2] Implement `AppState::with_handlers()` constructor in `gravity-core/src/state/mod.rs`

### Module Exports

- [X] T020 [P] [US2] Export `AppState` from `gravity-core/src/lib.rs`

## Phase 5: User Story 3 - Reduce Main.rs Boilerplate

**Priority**: P2
**Goal**: Main.rs imports AppState from ui/mod.rs with minimal configuration
**Independent Test**: Create main.rs that imports AppState from ui/mod.rs and runs identically to manually configured version

### Example Integration

- [X] T021 [US3] Create minimal main.rs template using AppState in `examples/hello-world/src/main.rs`
- [X] T022 [US3] Create `examples/hello-world/ui/mod.rs` with AppState export pattern
- [X] T023 [US3] Create `examples/hello-world/ui/app.gravity.rs` with #[gravity_ui] macro usage (macro applied in mod.rs)
- [X] T024 [US3] Create `examples/hello-world/ui/app.gravity` XML file

## Phase 6: User Story 4 - Multiple UI Views

**Priority**: P3
**Goal**: Support multiple UI views (app.gravity, settings.gravity) in same project
**Independent Test**: Create project with 3 different .gravity files and switch between them at runtime

### Multiple Views Pattern

- [X] T025 [P] [US4] Create `examples/settings/src/ui/app.rs` demonstrating multiple views pattern - Main view
- [X] T026 [P] [US4] Create `examples/settings/src/ui/app.gravity` XML file
- [X] T027 [P] [US4] Create `examples/settings/src/ui/settings.rs` demonstrating multiple views pattern - Settings view
- [X] T028 [P] [US4] Create `examples/settings/src/ui/settings.gravity` XML file
- [X] T029 [P] [US4] Create `examples/settings/src/ui/mod.rs` exporting multiple AppStates
- [X] T030 [P] [US4] Update `examples/settings/src/main.rs` demonstrating view switching

## Phase 7: Polish & Cross-Cutting Concerns

**Goal**: Complete testing, documentation updates, and existing example migration

### Contract Tests

- [X] T031 Create contract test for auto-loading mechanism in `gravity-macros/tests/auto_loading_tests.rs`
- [X] T032 Create contract test for AppState struct in `gravity-core/tests/appstate_tests.rs`
- [X] T033 Create integration test for hello-world example in `examples/hello-world/tests/integration.rs`

### Example Migration

- [X] T034 Migrate `examples/counter/` to new auto-loading pattern
- [X] T035 Migrate `examples/todo-app/` to new auto-loading pattern

### Documentation Updates

- [X] T036 Update AGENTS.md with AppState usage patterns
- [X] T037 Add auto-loading section to existing README.md and update existing examples

### Final Verification

- [X] T038 Run `cargo test --workspace` to verify all tests pass
- [X] T039 Run `cargo clippy --workspace -- -D warnings` to ensure lint compliance
- [X] T040 Run `cargo fmt --all` to format all code

## Task Summary

| Phase | Tasks | Description |
|-------|-------|-------------|
| Phase 1 | T001-T003 | Setup project structure |
| Phase 2 | T004-T005 | Foundational components |
| Phase 3 (US1) | T006-T012 | Auto-loading mechanism (7 tasks) |
| Phase 4 (US2) | T013-T020 | AppState struct (8 tasks) |
| Phase 5 (US3) | T021-T024 | Boilerplate reduction (4 tasks) |
| Phase 6 (US4) | T025-T03 | Multiple views (4 tasks) |
| Phase 7 | T031-T040 | Polish & cross-cutting (10 tasks) |
| **Total** | **38 tasks** | |

## Independent Test Criteria

| User Story | Test Criteria |
|------------|---------------|
| US1 | `gravity check` validates XML, AppState generates without include_str! |
| US2 | AppState<()>, AppState<Model>, AppState<Model> with handlers all compile |
| US3 | main.rs has 50% fewer lines than manual configuration |
| US4 | Project with 3+ views compiles and switches views at runtime |

## Implementation Strategy

### MVP Scope (User Story 1)

The MVP is User Story 1 (US1) + Foundational (Phase 1-2). This delivers:
- Auto-loading of `.gravity` files via `#[gravity_ui]` macro
- XML parsing and GravityDocument generation
- Basic error handling for missing/invalid files

### Incremental Delivery

1. **Iteration 1**: Phases 1-2 (Setup + Foundational) - 5 tasks
2. **Iteration 2**: Phase 3 (US1 - Auto-loading) - 7 tasks
3. **Iteration 3**: Phase 4 (US2 - AppState) - 8 tasks
4. **Iteration 4**: Phase 5 (US3 - Boilerplate) - 4 tasks
5. **Iteration 5**: Phase 6 (US4 - Multiple views) - 4 tasks
6. **Iteration 6**: Phase 7 (Polish) - 10 tasks

## Testing Notes

Tests are included in Phase 7 as contract tests and integration tests. Unit tests should be added inline with implementation tasks where appropriate. The Constitution requires TDD, so implement tests before or alongside implementation code.
