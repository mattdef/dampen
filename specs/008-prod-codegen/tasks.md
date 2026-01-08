---

description: "Task list for Production Mode with Static Code Generation feature implementation"
---

# Tasks: Production Mode with Static Code Generation

**Input**: Design documents from `/specs/008-prod-codegen/`
**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, quickstart.md

**Tests**: Not explicitly requested in spec.md - implementing TDD pattern with contract tests

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3, US4, US5)
- Include exact file paths in descriptions

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and code generation infrastructure

- [X] T001 Add HandlerInfo and HandlerSignatureType structures to gravity-core/src/codegen/mod.rs
- [X] T002 [P] Add GeneratedApplication struct for code generation output in gravity-core/src/codegen/mod.rs
- [X] T003 [P] Add HandlerCallGraph struct for circular dependency detection in gravity-core/src/handler/mod.rs (NEW FILE)
- [X] T004 Add generate_application function signature to gravity-core/src/codegen/mod.rs
- [X] T005 [P] Create build.rs template in crates/gravity-cli/templates/build.rs.template

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST be complete before ANY user story can be implemented

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [X] T006 Implement HandlerInfo and HandlerSignatureType structures in gravity-core/src/codegen/mod.rs
- [X] T007 [P] Implement HandlerCallGraph with circular dependency detection in gravity-core/src/handler/mod.rs
- [X] T008 [P] Implement basic generate_application function stub in gravity-core/src/codegen/mod.rs
- [X] T009 Create build.rs template in crates/gravity-cli/templates/build.rs.template
- [X] T010 Update gravity-cli/src/commands/new.rs to copy build.rs template on project creation

**Checkpoint**: Foundation ready - user story implementation can now begin in parallel

---

## Phase 3: User Story 1 - Production Build Command (Priority: P1) 🎯 MVP

**Goal**: Implement `gravity build --prod` command that generates production-ready binaries with static code generation

**Independent Test**: Run `gravity build --prod` on a sample project and verify output binary is generated and runs without XML parsing at runtime

### Tests for User Story 1 ⚠️

> **Write these tests FIRST, ensure they FAIL before implementation**

- [X] T011 [P] [US1] Contract test for handler metadata extraction in crates/gravity-macros/tests/handler_metadata_tests.rs
- [X] T012 [P] [US1] Integration test for build.rs code generation in crates/gravity-cli/tests/build_production_tests.rs

### Implementation for User Story 1

- [X] T013 [US1] Enhance #[ui_handler] macro to emit HandlerInfo metadata in crates/gravity-macros/src/ui_handler.rs
- [X] T014 [P] [US1] Add HandlerInfo emission to macro output with name, signature_type, param_types, return_type
- [X] T015 [P] [US1] Add source_file and source_line tracking for error reporting
- [X] T016 [US1] Implement code generation logic in gravity-core/src/codegen/mod.rs (generate widget code from parsed XML)
- [X] T017 [US1] Implement handler registry generation in generate_application function
- [X] T018 [US1] Add verbose build progress output (number of parsed files, handlers found, generated code location)
- [X] T019 [US1] Update gravity-cli/src/commands/build.rs to add --prod flag and execute production build
- [X] T020 [US1] Implement execute_production_build function in gravity-cli/src/commands/build.rs

**Checkpoint**: At this point, User Story 1 should be fully functional and testable independently

---

## Phase 4: User Story 2 - New Project Setup (Priority: P1)

**Goal**: New projects created with `gravity new` are automatically configured for production builds with build.rs

**Independent Test**: Create new project with `gravity new` and verify build.rs exists and Cargo.toml references it

### Tests for User Story 2 ⚠️

- [X] T021 [P] [US2] Integration test for new project generation in crates/gravity-cli/tests/new_tests.rs

### Implementation for User Story 2

- [X] T022 [US2] Add build.rs template generation to gravity new command in crates/gravity-cli/src/commands/new.rs
- [X] T023 [US2] Ensure build = "build.rs" is added to Cargo.toml for new projects
- [X] T024 [P] [US2] Verify generated project includes src/ui/ directory structure
- [X] T025 [P] [US2] Add integration test for build.rs generation in new projects

**Checkpoint**: User Stories 1 AND 2 should both work independently

---

## Phase 5: User Story 3 - Handler Registration (Priority: P2)

**Goal**: UI handlers are automatically discovered and registered during build process without manual registry maintenance

**Independent Test**: Define handlers with #[ui_handler] attribute and verify they are called when corresponding UI events are triggered

### Tests for User Story 3 ⚠️

- [X] T026 [P] [US3] Contract test for handler metadata completeness in crates/gravity-macros/tests/handler_metadata_tests.rs
- [X] T027 [P] [US3] Integration test for handler discovery in generated code

### Implementation for User Story 3

- [X] T028 [US3] Enhance #[ui_handler] macro to detect handler signature type (Simple, WithValue, WithCommand)
- [X] T029 [P] [US3] Add parameter type extraction for handler metadata
- [X] T030 [US3] Implement handler registry construction from metadata in generated code
- [X] T031 [US3] Connect handler references in XML to registered handlers in generated application
- [X] T032 [P] [US3] Add error handling for handler signature mismatches

**Checkpoint**: User Stories 1, 2, AND 3 should all work independently

---

## Phase 6: User Story 4 - Development Mode Compatibility (Priority: P2)

**Goal**: Development mode with #[gravity_ui] continues to work for rapid iteration alongside production mode

**Independent Test**: Run project with development pattern and verify application loads correctly

### Tests for User Story 4 ⚠️

- [X] T033 [P] [US4] Regression test for #[gravity_ui] mode in gravity-runtime tests

### Implementation for User Story 4

- [X] T034 [US4] Ensure #[gravity_ui] macro and runtime interpretation are unchanged
- [X] T035 [P] [US4] Verify production and development modes can coexist in same project
- [X] T036 [P] [US4] Test that handlers defined with #[ui_handler] work in both modes
- [X] T037 [US4] Add documentation for using both modes in quickstart.md

**Checkpoint**: All modes compatible - development iteration still works

---

## Phase 7: User Story 5 - Example Migration (Priority: P3)

**Goal**: All example projects demonstrate production mode as templates for new users

**Independent Test**: Build each migrated example with `cargo build --release` and verify application runs correctly

### Implementation for User Story 5

- [X] T038 [P] [US5] Migrate hello-world example: add build.rs, update main.rs, update Cargo.toml
- [X] T039 [P] [US5] Migrate counter example: add build.rs, update main.rs, update Cargo.toml
- [X] T040 [P] [US5] Migrate todo-app example: add build.rs, update main.rs, update Cargo.toml
- [X] T041 [P] [US5] Migrate settings example: add build.rs, update main.rs, update Cargo.toml
- [X] T042 [P] [US5] Migrate styling example: add build.rs, update main.rs, update Cargo.toml
- [X] T043 [P] [US5] Migrate widget-showcase example: add build.rs, update main.rs, update Cargo.toml
- [X] T044 [P] [US5] Migrate responsive example: add build.rs, update main.rs, update Cargo.toml

**Checkpoint**: All examples migrated and working in production mode

---

## Phase 8: Polish & Cross-Cutting Concerns

**Purpose**: Improvements that affect multiple user stories

- [X] T045 [P] Implement handler not found error messages with suggestions (FR-007)
- [X] T046 [P] Implement invalid XML validation using gravity check (FR-011)
- [X] T047 [P] Implement duplicate handler name detection with source locations (FR-013)
- [X] T048 [P] Implement circular dependency detection at compile time (FR-014)
- [X] T049 [P] Update AGENTS.md with new technologies from this feature
- [X] T050 Run cargo clippy --workspace -- -D warnings and fix any issues
- [X] T051 Run cargo fmt --all -- --check and format any unformatted code
- [X] T052 Run cargo test --workspace and ensure all tests pass
- [X] T053 [P] Validate quickstart.md examples work correctly

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories
- **User Stories (Phase 3-7)**: All depend on Foundational phase completion
  - User stories can proceed in parallel (if staffed)
  - Or sequentially in priority order (P1 → P2 → P3)
- **Polish (Phase 8)**: Depends on all desired user stories being complete

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Foundational (Phase 2) - No dependencies on other stories
- **User Story 2 (P1)**: Can start after Foundational (Phase 2) - No dependencies on other stories
- **User Story 3 (P2)**: Can start after Foundational (Phase 2) - No dependencies on other stories
- **User Story 4 (P2)**: Can start after Foundational (Phase 2) - No dependencies on other stories
- **User Story 5 (P3)**: Can start after Foundational (Phase 2) - Depends on US1, US2, US3 (handlers must work first)

### Within Each User Story

- Tests (if included) MUST be written and FAIL before implementation
- Models before code generation
- Core implementation before integration
- Story complete before moving to next priority

### Parallel Opportunities

- All Setup tasks marked [P] can run in parallel
- All Foundational tasks marked [P] can run in parallel (within Phase 2)
- Once Foundational phase completes, all user stories can start in parallel (if team capacity allows)
- All tests for a user story marked [P] can run in parallel
- Models within a story marked [P] can run in parallel
- Different user stories can be worked on in parallel by different team members

---

## Parallel Example: User Story 1

```bash
# Launch all tests for User Story 1 together:
Task: "Contract test for handler metadata extraction in crates/gravity-macros/tests/handler_metadata_tests.rs"
Task: "Integration test for build.rs code generation in crates/gravity-cli/tests/build_production_tests.rs"

# Launch all parallel implementation tasks:
Task: "Add HandlerInfo emission to macro output with name, signature_type, param_types, return_type"
Task: "Add source_file and source_line tracking for error reporting"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational (CRITICAL - blocks all stories)
3. Complete Phase 3: User Story 1
4. **STOP and VALIDATE**: Test User Story 1 independently with `gravity build --prod`
5. Deploy/demo if ready

### Incremental Delivery

1. Complete Setup + Foundational → Foundation ready
2. Add User Story 1 → Test independently → Deploy/Demo (MVP!)
3. Add User Story 2 → Test independently → Deploy/Demo
4. Add User Story 3 → Test independently → Deploy/Demo
5. Add User Story 4 → Test independently → Deploy/Demo
6. Add User Story 5 → Test independently → Deploy/Demo
7. Polish phase → Final release

### Parallel Team Strategy

With multiple developers:

1. Team completes Setup + Foundational together
2. Once Foundational is done:
   - Developer A: User Story 1 (Production Build Command)
   - Developer B: User Story 2 (New Project Setup)
   - Developer C: User Story 3 (Handler Registration)
3. Stories complete and integrate independently

---

## Task Summary

| Phase | Task Count | Description |
|-------|------------|-------------|
| Phase 1: Setup | 5 | Infrastructure and data structures |
| Phase 2: Foundational | 5 | Core blocking prerequisites |
| Phase 3: US1 Production Build | 10 | Core production build command |
| Phase 4: US2 New Project | 5 | New project setup |
| Phase 5: US3 Handler Registration | 7 | Handler metadata and discovery |
| Phase 6: US4 Dev Mode Compatibility | 4 | Backward compatibility |
| Phase 7: US5 Example Migration | 7 | 7 examples to migrate |
| Phase 8: Polish | 9 | Cross-cutting improvements |
| **Total** | **52** | |

### User Story Task Counts

- **User Story 1 (P1)**: 10 tasks
- **User Story 2 (P1)**: 5 tasks
- **User Story 3 (P2)**: 7 tasks
- **User Story 4 (P2)**: 4 tasks
- **User Story 5 (P3)**: 7 tasks
- **Setup/Foundational/Polish**: 19 tasks

---

## Notes

- [P] tasks = different files, no dependencies
- [Story] label maps task to specific user story for traceability
- Each user story should be independently completable and testable
- Verify tests fail before implementing
- Commit after each task or logical group
- Stop at any checkpoint to validate story independently
- Avoid: vague tasks, same file conflicts, cross-story dependencies that break independence
