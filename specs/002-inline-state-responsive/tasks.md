# Tasks: Inline State Styles & Responsive Design

**Input**: Design documents from `/specs/002-inline-state-responsive/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/

**Tests**: Included per constitution (Test-First Development, TDD required)

**Organization**: Tasks grouped by user story for independent implementation and testing.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2)
- Include exact file paths in descriptions

## Path Conventions

- **Workspace structure**: `crates/dampen-core/`, `crates/dampen-iced/`, `crates/dampen-macros/`
- **Tests**: Within each crate's `tests/` directory or inline modules

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Verify existing infrastructure and prepare for implementation

- [x] T001 Verify existing WidgetState enum in `crates/dampen-core/src/ir/theme.rs` has Hover, Focus, Active, Disabled variants
- [x] T002 Verify existing Breakpoint enum in `crates/dampen-core/src/ir/layout.rs` has Mobile, Tablet, Desktop with `from_viewport_width()`
- [x] T003 Verify existing `breakpoint_attributes` field in WidgetNode at `crates/dampen-core/src/ir/node.rs`
- [x] T004 [P] Verify existing state mapping functions in `crates/dampen-iced/src/style_mapping.rs`

---

## Phase 2: Foundational (Core IR Changes)

**Purpose**: Add `inline_state_variants` field and `WidgetState::from_prefix()` - MUST complete before any user story

**CRITICAL**: No user story work can begin until this phase is complete

### Tests for Foundational (TDD)

- [x] T005 [P] Write unit tests for `WidgetState::from_prefix()` in `crates/dampen-core/src/ir/theme.rs` (inline test module)
- [x] T006 [P] Write unit tests for `inline_state_variants` field serialization in `crates/dampen-core/src/ir/node.rs` (inline test module)

### Implementation for Foundational

- [x] T007 Add `WidgetState::from_prefix(s: &str) -> Option<Self>` method in `crates/dampen-core/src/ir/theme.rs`
- [x] T008 Add `inline_state_variants: HashMap<WidgetState, StyleProperties>` field to WidgetNode in `crates/dampen-core/src/ir/node.rs`
- [x] T009 Update WidgetNode Default impl to include empty `inline_state_variants` HashMap in `crates/dampen-core/src/ir/node.rs`
- [x] T010 Run `cargo test -p dampen-core` to verify foundational tests pass

**Checkpoint**: Core IR types ready - parser and builder work can begin

---

## Phase 3: User Story 1 & 2 - Inline State Styles (Priority: P1) MVP

**Goal**: Parse and apply single/multiple inline state styles (e.g., `hover:background="#ff0000"`)

**Independent Test**: Render a button with `hover:background`, verify style changes on hover state

### Tests for User Stories 1 & 2 (TDD)

> **NOTE: Write these tests FIRST, ensure they FAIL before implementation**

- [x] T011 [P] [US1] Write parser test for single state attribute `hover:background` in `crates/dampen-core/src/parser/mod.rs` (inline test module)
- [x] T012 [P] [US2] Write parser test for multiple state attributes `hover:background`, `active:background`, `disabled:opacity` in `crates/dampen-core/src/parser/mod.rs`
- [x] T013 [P] [US1] Write parser test for invalid state prefix `unknown:background` returns warning in `crates/dampen-core/src/parser/mod.rs`
- [x] T014 [P] [US1] Write builder test for button with hover state style in `crates/dampen-iced/tests/builder_state_styles.rs`
- [x] T015 [P] [US2] Write builder test for button with multiple state styles in `crates/dampen-iced/tests/builder_state_styles.rs`

### Parser Implementation for US1 & US2

- [x] T016 [US1] Add state-prefixed attribute detection using `split_once(':')` in `crates/dampen-core/src/parser/mod.rs` parse_node function
- [x] T017 [US1] Parse state-prefixed style attributes into `StyleProperties` using existing `parse_style_attributes` in `crates/dampen-core/src/parser/mod.rs`
- [x] T018 [US1] Store parsed state styles in `inline_state_variants` HashMap in `crates/dampen-core/src/parser/mod.rs`
- [x] T019 [US1] Add warning log for unknown state prefixes in `crates/dampen-core/src/parser/mod.rs`
- [x] T020 [US1] Run `cargo test -p dampen-core` to verify parser tests pass

### Builder Implementation for US1 & US2

- [ ] T021 [US1] Add `resolve_complete_styles_with_states()` helper function in `crates/dampen-iced/src/builder/helpers.rs`
- [ ] T022 [US1] Implement style precedence: Theme -> Class -> Class State -> Inline Base -> Inline State in `crates/dampen-iced/src/builder/helpers.rs`
- [ ] T023 [US1] Update button builder to use state-aware style closure in `crates/dampen-iced/src/builder/button.rs`
- [ ] T024 [P] [US2] Update text_input builder to use state-aware style closure in `crates/dampen-iced/src/builder/text_input.rs`
- [ ] T025 [P] [US2] Update checkbox builder to use state-aware style closure in `crates/dampen-iced/src/builder/checkbox.rs`
- [ ] T026 [P] [US2] Update slider builder to use state-aware style closure in `crates/dampen-iced/src/builder/slider.rs`
- [ ] T027 [P] [US2] Update toggler builder to use state-aware style closure in `crates/dampen-iced/src/builder/toggler.rs`
- [ ] T028 [P] [US2] Update radio builder to use state-aware style closure in `crates/dampen-iced/src/builder/radio.rs`
- [ ] T029 [P] [US2] Update pick_list builder to use state-aware style closure in `crates/dampen-iced/src/builder/pick_list.rs`
- [ ] T030 [US2] Add debug log for state styles on non-interactive widgets in `crates/dampen-iced/src/builder/helpers.rs`
- [ ] T031 [US2] Run `cargo test -p dampen-iced` to verify builder tests pass

**Checkpoint**: US1 & US2 complete - inline state styles work in interpreted mode

---

## Phase 4: User Story 3 - Responsive Layout with Breakpoints (Priority: P2)

**Goal**: Apply breakpoint-specific attributes based on viewport width (e.g., `mobile-spacing="10"`)

**Independent Test**: Render column with `mobile-spacing="10"` and `desktop-spacing="40"`, verify spacing changes at 640px threshold

### Tests for User Story 3 (TDD)

- [ ] T032 [P] [US3] Write builder test for viewport_width parameter in `crates/dampen-iced/tests/builder_responsive.rs`
- [ ] T033 [P] [US3] Write builder test for breakpoint attribute resolution in `crates/dampen-iced/tests/builder_responsive.rs`
- [ ] T034 [P] [US3] Write builder test for mobile breakpoint (<640px) in `crates/dampen-iced/tests/builder_responsive.rs`
- [ ] T035 [P] [US3] Write builder test for desktop fallback when no viewport in `crates/dampen-iced/tests/builder_responsive.rs`

### Implementation for User Story 3

- [ ] T036 [US3] Add `viewport_width: Option<f32>` field to DampenWidgetBuilder struct in `crates/dampen-iced/src/builder/mod.rs`
- [ ] T037 [US3] Update DampenWidgetBuilder::new() signature to accept viewport_width in `crates/dampen-iced/src/builder/mod.rs`
- [ ] T038 [US3] Add `active_breakpoint(&self) -> Breakpoint` helper method in `crates/dampen-iced/src/builder/mod.rs`
- [ ] T039 [US3] Add `resolve_breakpoint_attributes()` helper in `crates/dampen-iced/src/builder/helpers.rs`
- [ ] T040 [US3] Integrate breakpoint resolution in widget builders that use layout attributes in `crates/dampen-iced/src/builder/helpers.rs`
- [ ] T041 [US3] Update all call sites of DampenWidgetBuilder::new() to pass viewport_width in `crates/dampen-iced/src/lib.rs`
- [ ] T042 [US3] Run `cargo test -p dampen-iced` to verify responsive tests pass

**Checkpoint**: US3 complete - responsive breakpoints work in interpreted mode

---

## Phase 5: User Story 4 - Interpreted Mode Hot-Reload (Priority: P2)

**Goal**: Verify inline state styles and breakpoint attributes hot-reload correctly in `dampen run`

**Independent Test**: Modify `hover:background` in XML while app running, verify change appears without restart

### Tests for User Story 4

- [ ] T043 [US4] Manual test: Run counter example with `dampen run`, modify hover style, verify hot-reload
- [ ] T044 [US4] Manual test: Run counter example, modify breakpoint attribute, verify responsive change

### Implementation for User Story 4

- [ ] T045 [US4] Verify hot-reload already re-parses XML and rebuilds widgets (should work automatically)
- [ ] T046 [US4] Add example XML with inline state styles to `examples/counter/src/ui/main.dampen`
- [ ] T047 [US4] Document hot-reload behavior for inline states in `docs/STYLING.md`

**Checkpoint**: US4 complete - interpreted mode fully supports inline states and responsive design

---

## Phase 6: User Story 5 - Codegen Mode Type Safety (Priority: P3)

**Goal**: Generate Rust code with state-aware style closures for production builds

**Independent Test**: Run `dampen build`, verify generated code contains match expressions on Iced status enums

### Tests for User Story 5 (TDD)

- [ ] T048 [P] [US5] Write snapshot test for button codegen with hover state in `crates/dampen-macros/tests/codegen_state_styles.rs`
- [ ] T049 [P] [US5] Write snapshot test for button codegen with multiple states in `crates/dampen-macros/tests/codegen_state_styles.rs`
- [ ] T050 [P] [US5] Write snapshot test for codegen with breakpoint attributes in `crates/dampen-macros/tests/codegen_responsive.rs`
- [ ] T051 [P] [US5] Write test for invalid state prefix compile error in `crates/dampen-macros/tests/codegen_errors.rs`

### Implementation for User Story 5

- [ ] T052 [US5] Add state-aware code generation for button widget in `crates/dampen-macros/src/dampen_app.rs`
- [ ] T053 [US5] Generate match expressions mapping Iced Status to style variants in `crates/dampen-macros/src/dampen_app.rs`
- [ ] T054 [P] [US5] Add state-aware code generation for text_input widget in `crates/dampen-macros/src/dampen_app.rs`
- [ ] T055 [P] [US5] Add state-aware code generation for checkbox widget in `crates/dampen-macros/src/dampen_app.rs`
- [ ] T056 [P] [US5] Add state-aware code generation for remaining interactive widgets in `crates/dampen-macros/src/dampen_app.rs`
- [ ] T057 [US5] Add breakpoint resolution code generation in `crates/dampen-macros/src/dampen_app.rs`
- [ ] T058 [US5] Add compile-time error for invalid state prefixes in `crates/dampen-macros/src/dampen_app.rs`
- [ ] T059 [US5] Run `cargo test -p dampen-macros` to verify codegen tests pass
- [ ] T060 [US5] Run `insta review` to approve new snapshots

**Checkpoint**: US5 complete - codegen mode supports inline state styles and responsive design

---

## Phase 7: Polish & Cross-Cutting Concerns

**Purpose**: Documentation, validation, and final quality checks

- [ ] T061 [P] Update `docs/STYLING.md` with inline state styles documentation
- [ ] T062 [P] Update `docs/RESPONSIVE.md` or create if not exists with breakpoint documentation
- [ ] T063 [P] Add inline state style examples to `examples/` directory
- [ ] T064 Run `cargo clippy --workspace -- -D warnings` to verify no warnings
- [ ] T065 Run `cargo fmt --all -- --check` to verify formatting
- [ ] T066 Run `cargo test --workspace` to verify all tests pass
- [ ] T067 Run visual parity test between interpreted and codegen modes
- [ ] T068 Validate quickstart.md scenarios work as documented
- [ ] T069 Update CHANGELOG.md with feature additions

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - verification only
- **Foundational (Phase 2)**: Depends on Setup - BLOCKS all user stories
- **US1 & US2 (Phase 3)**: Depends on Foundational - core inline state functionality
- **US3 (Phase 4)**: Depends on Foundational - can run parallel to US1/US2
- **US4 (Phase 5)**: Depends on US1, US2, US3 - validation of interpreted mode
- **US5 (Phase 6)**: Depends on Foundational - can run parallel to US1-US4 (different crate)
- **Polish (Phase 7)**: Depends on all user stories complete

### User Story Dependencies

```
Foundational (Phase 2)
    │
    ├──────────────────┬──────────────────┐
    ▼                  ▼                  ▼
US1 & US2           US3 (P2)           US5 (P3)
(Phase 3)           (Phase 4)          (Phase 6)
    │                  │                  │
    └────────┬─────────┘                  │
             ▼                            │
          US4 (P2)                        │
          (Phase 5)                       │
             │                            │
             └────────────────────────────┘
                         │
                         ▼
                   Polish (Phase 7)
```

### Parallel Opportunities

**Within Phase 2 (Foundational)**:
- T005 and T006 can run in parallel (different test files)

**Within Phase 3 (US1 & US2)**:
- T011, T012, T013 can run in parallel (parser tests)
- T014, T015 can run in parallel (builder tests)
- T024-T029 can run in parallel (different widget files)

**Within Phase 4 (US3)**:
- T032, T033, T034, T035 can run in parallel (different test cases)

**Within Phase 6 (US5)**:
- T048, T049, T050, T051 can run in parallel (different test files)
- T054, T055, T056 can run in parallel (different widget code gen)

**Cross-Phase Parallelism**:
- US3 (Phase 4) can start after Foundational, parallel to US1/US2
- US5 (Phase 6) can start after Foundational, parallel to US1/US2/US3

---

## Parallel Example: Phase 3 (US1 & US2)

```bash
# Launch all parser tests together:
Task: "Write parser test for single state attribute in crates/dampen-core/src/parser/mod.rs"
Task: "Write parser test for multiple state attributes in crates/dampen-core/src/parser/mod.rs"
Task: "Write parser test for invalid state prefix in crates/dampen-core/src/parser/mod.rs"

# After parser impl complete, launch all widget builder updates together:
Task: "Update text_input builder in crates/dampen-iced/src/builder/text_input.rs"
Task: "Update checkbox builder in crates/dampen-iced/src/builder/checkbox.rs"
Task: "Update slider builder in crates/dampen-iced/src/builder/slider.rs"
Task: "Update toggler builder in crates/dampen-iced/src/builder/toggler.rs"
Task: "Update radio builder in crates/dampen-iced/src/builder/radio.rs"
Task: "Update pick_list builder in crates/dampen-iced/src/builder/pick_list.rs"
```

---

## Implementation Strategy

### MVP First (User Stories 1 & 2 Only)

1. Complete Phase 1: Setup (verification)
2. Complete Phase 2: Foundational (core IR changes)
3. Complete Phase 3: User Stories 1 & 2 (inline state styles)
4. **STOP and VALIDATE**: Test button with `hover:background` in interpreted mode
5. Deploy/demo MVP

### Incremental Delivery

1. Foundation + US1/US2 → Inline state styles work (MVP!)
2. Add US3 → Responsive breakpoints work → Demo responsive features
3. Add US4 → Hot-reload validated → Demo development workflow
4. Add US5 → Production codegen ready → Demo production build
5. Polish → Documentation complete → Release ready

### Parallel Team Strategy

With 2 developers:
- **Developer A**: US1/US2 (dampen-core parser + dampen-iced builder)
- **Developer B**: US5 (dampen-macros codegen) - can start after Foundational

With 3 developers:
- **Developer A**: US1/US2 (parser + builder)
- **Developer B**: US3 (responsive builder)
- **Developer C**: US5 (codegen)

---

## Notes

- [P] tasks = different files, no dependencies
- [Story] label maps task to specific user story
- US1 and US2 combined because they share all infrastructure
- Tests use TDD per constitution (write tests first, verify they fail)
- Verify `cargo clippy` and `cargo fmt` pass before completing each phase
- Constitution requires >90% test coverage for dampen-core
