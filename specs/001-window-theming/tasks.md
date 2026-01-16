# Tasks: Window Theming

**Input**: Design documents from `/specs/001-window-theming/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/theme-api.md

**Tests**: Contract tests included per Constitution Principle V (Test-First Development)

**Organization**: Tasks grouped by user story (P1-P4) for independent implementation

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story (US1-US5)
- All paths relative to repository root

---

## Phase 1: Setup

**Purpose**: Project structure and test fixtures for theming feature

- [ ] T001 Create test fixtures directory at tests/contract/fixtures/
- [ ] T002 [P] Create valid_theme.dampen fixture in tests/contract/fixtures/valid_theme.dampen
- [ ] T003 [P] Create invalid_theme.dampen fixtures (missing colors, invalid values) in tests/contract/fixtures/
- [ ] T004 [P] Add `dark_light` crate dependency to crates/dampen-dev/Cargo.toml for system theme detection

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core types and infrastructure that ALL user stories depend on

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

### Contract Tests (TDD - Write First, Must Fail)

- [ ] T005 [P] Contract test: parse valid theme document in tests/contract/theme_contracts.rs
- [ ] T006 [P] Contract test: validation errors for invalid themes in tests/contract/theme_contracts.rs
- [ ] T007 [P] Contract test: theme file discovery in tests/contract/theme_contracts.rs
- [ ] T008 [P] Contract test: backward compatibility (no theme file) in tests/contract/theme_contracts.rs

### Core Types (dampen-core)

- [ ] T009 Add ThemeDocument struct to crates/dampen-core/src/ir/theme.rs
- [ ] T010 Add ThemeDocument::validate() method in crates/dampen-core/src/ir/theme.rs
- [ ] T011 Add ThemeDocument::effective_default() method in crates/dampen-core/src/ir/theme.rs
- [ ] T012 [P] Add ThemeError enum to crates/dampen-core/src/ir/theme.rs
- [ ] T013 Add parse_theme_document() function to crates/dampen-core/src/parser/theme_parser.rs
- [ ] T014 Add parse validation for missing colors with THEME_003 error in crates/dampen-core/src/parser/theme_parser.rs
- [ ] T015 Add parse validation for invalid default theme with THEME_002 error in crates/dampen-core/src/parser/theme_parser.rs

### Theme Context (dampen-core)

- [ ] T016 Create ThemeContext struct in crates/dampen-core/src/state/theme_context.rs (NEW FILE)
- [ ] T017 Implement ThemeContext::from_document() in crates/dampen-core/src/state/theme_context.rs
- [ ] T018 Implement ThemeContext::active() in crates/dampen-core/src/state/theme_context.rs
- [ ] T019 Export ThemeContext from crates/dampen-core/src/state/mod.rs
- [ ] T020 Export ThemeContext from crates/dampen-core/src/lib.rs

### Theme Adapter (dampen-iced)

- [ ] T021 [P] Contract test: Iced theme conversion in tests/contract/theme_contracts.rs
- [ ] T022 Implement ThemePalette::to_iced_palette() in crates/dampen-core/src/ir/theme.rs
- [ ] T023 Implement ThemeAdapter::to_iced_theme() (replace placeholder) in crates/dampen-iced/src/theme_adapter.rs

### Theme Discovery (dampen-dev)

- [ ] T024 Create discover_theme_file() function in crates/dampen-dev/src/theme_loader.rs (NEW FILE)
- [ ] T025 Create load_theme_context() function in crates/dampen-dev/src/theme_loader.rs
- [ ] T026 Export theme_loader module from crates/dampen-dev/src/lib.rs

**Checkpoint**: Foundation ready - run `cargo test --workspace` to verify all contract tests pass

---

## Phase 3: User Story 1 - Apply a Built-in Theme (Priority: P1) 🎯 MVP

**Goal**: Developer can create theme.dampen with light/dark theme and app uses it

**Independent Test**: Create `src/ui/theme/theme.dampen` with "light" theme, run app, verify widgets show theme colors

### Contract Tests for US1

- [ ] T027 [P] [US1] Contract test: theme context creation with default selection in tests/contract/theme_contracts.rs

### Implementation for US1

- [ ] T028 [US1] Add built-in light theme palette constants to crates/dampen-core/src/ir/theme.rs
- [ ] T029 [US1] Add built-in dark theme palette constants to crates/dampen-core/src/ir/theme.rs
- [ ] T030 [US1] Implement default theme loading in #[dampen_app] macro in crates/dampen-macros/src/dampen_app.rs
- [ ] T031 [US1] Add theme context to AppState in crates/dampen-core/src/state/mod.rs
- [ ] T032 [US1] Pass theme to Iced application in crates/dampen-macros/src/dampen_app.rs
- [ ] T033 [US1] Update DampenWidgetBuilder to use theme context in crates/dampen-iced/src/builder/mod.rs
- [ ] T034 [US1] Add system theme detection via dark_light crate in crates/dampen-dev/src/theme_loader.rs
- [ ] T035 [US1] Create theme.dampen.template for new projects in crates/dampen-cli/templates/new/src/ui/theme/theme.dampen.template

### Integration Test for US1

- [ ] T036 [US1] Integration test: app with theme.dampen loads theme in tests/integration/theme_e2e.rs

**Checkpoint**: User Story 1 complete - app can load and display themes from theme.dampen

---

## Phase 4: User Story 2 - Switch Themes at Runtime (Priority: P2)

**Goal**: End user can switch between themes without restarting app

**Independent Test**: Add theme toggle button, click it, verify all widgets update within 200ms

### Contract Tests for US2

- [ ] T037 [P] [US2] Contract test: runtime theme switching in tests/contract/theme_contracts.rs

### Implementation for US2

- [ ] T038 [US2] Implement ThemeContext::set_theme() in crates/dampen-core/src/state/theme_context.rs
- [ ] T039 [US2] Add set_theme handler action parsing in crates/dampen-core/src/parser/mod.rs
- [ ] T040 [US2] Implement set_theme message handling in crates/dampen-macros/src/dampen_app.rs
- [ ] T041 [US2] Add theme binding expression support (theme="{model.theme}") in crates/dampen-core/src/parser/mod.rs
- [ ] T042 [US2] Update view rebuild to propagate theme changes in crates/dampen-macros/src/dampen_app.rs

### Integration Test for US2

- [ ] T043 [US2] Integration test: runtime theme switch updates all widgets in tests/integration/theme_e2e.rs

**Checkpoint**: User Story 2 complete - runtime theme switching works

---

## Phase 5: User Story 5 - Hot-Reload Theme Changes (Priority: P2)

**Goal**: Developer can edit theme.dampen and see changes live without restart

**Independent Test**: Run `dampen run`, edit theme.dampen, save, verify UI updates within 500ms

### Contract Tests for US5

- [ ] T044 [P] [US5] Contract test: hot-reload theme update in tests/contract/theme_contracts.rs

### Implementation for US5

- [ ] T045 [US5] Implement ThemeContext::reload() in crates/dampen-core/src/state/theme_context.rs
- [ ] T046 [US5] Add theme.dampen to file watcher in crates/dampen-dev/src/watcher.rs
- [ ] T047 [US5] Handle theme file change events in crates/dampen-dev/src/reload.rs
- [ ] T048 [US5] Trigger theme reload on file change in crates/dampen-macros/src/dampen_app.rs

### Integration Test for US5

- [ ] T049 [US5] Integration test: theme hot-reload in tests/hot-reload-integration/theme_hot_reload.rs

**Checkpoint**: User Story 5 complete - theme hot-reload works in development mode

---

## Phase 6: User Story 3 - Create Custom Themes (Priority: P3)

**Goal**: Developer can define custom branded themes with their own colors

**Independent Test**: Define custom theme with brand colors in theme.dampen, verify exact colors appear

### Implementation for US3

- [ ] T050 [US3] Add theme inheritance (extends attribute) parsing in crates/dampen-core/src/parser/theme_parser.rs
- [ ] T051 [US3] Implement theme property inheritance resolution in crates/dampen-core/src/ir/theme.rs
- [ ] T052 [US3] Add detailed validation error messages for custom themes in crates/dampen-core/src/parser/theme_parser.rs
- [ ] T053 [US3] Update documentation with custom theme examples in docs/STYLING.md

### Integration Test for US3

- [ ] T054 [US3] Integration test: custom theme with inheritance in tests/integration/theme_e2e.rs

**Checkpoint**: User Story 3 complete - custom themes with inheritance work

---

## Phase 7: User Story 4 - Widget-Level Theme Overrides (Priority: P4)

**Goal**: Developer can override theme properties on specific widgets

**Independent Test**: Apply style override to one button, verify only that button differs from theme

### Implementation for US4

- [ ] T055 [US4] Ensure style class precedence over theme in crates/dampen-iced/src/builder/helpers.rs
- [ ] T056 [US4] Ensure inline styles precedence over style class in crates/dampen-iced/src/builder/helpers.rs
- [ ] T057 [US4] Update widget builders to merge theme + class + inline styles in crates/dampen-iced/src/builder/widgets/button.rs
- [ ] T058 [US4] Apply style merging to all widget builders in crates/dampen-iced/src/builder/widgets/*.rs

### Integration Test for US4

- [ ] T059 [US4] Integration test: widget-level overrides in tests/integration/theme_e2e.rs

**Checkpoint**: User Story 4 complete - widget-level overrides work correctly

---

## Phase 8: Codegen Support

**Goal**: Theme compiles into production binary (no runtime parsing)

**Purpose**: Required for Constitution Principle III (Production Mode)

### Contract Tests for Codegen

- [ ] T060 [P] Contract test: codegen theme function generation in tests/contract/theme_contracts.rs

### Implementation

- [ ] T061 Create generate_theme_code() function in crates/dampen-core/src/codegen/theme.rs (NEW FILE)
- [ ] T062 Generate app_theme() function from ThemeDocument in crates/dampen-core/src/codegen/theme.rs
- [ ] T063 Export theme codegen from crates/dampen-core/src/codegen/mod.rs
- [ ] T064 Integrate theme codegen into build process in crates/dampen-cli/src/commands/build.rs
- [ ] T065 Add theme to generated application code in crates/dampen-core/src/codegen/application.rs

### Integration Test for Codegen

- [ ] T066 Integration test: codegen build with theme in tests/integration/theme_e2e.rs

**Checkpoint**: Codegen complete - production builds include compiled themes

---

## Phase 9: Polish & Cross-Cutting Concerns

**Purpose**: Documentation, examples, and cleanup

- [ ] T067 [P] Update examples/styling to use separate theme.dampen file
- [ ] T068 [P] Create examples/theming example showcasing all theme features
- [ ] T069 [P] Update docs/STYLING.md with complete theming documentation
- [ ] T070 [P] Update docs/USAGE.md with theme quickstart
- [ ] T071 [P] Add theme section to README.md
- [ ] T072 Run quickstart.md validation (manual test)
- [ ] T073 Run `cargo clippy --workspace -- -D warnings`
- [ ] T074 Run `cargo fmt --all -- --check`
- [ ] T075 Run `cargo test --workspace` final verification

---

## Dependencies & Execution Order

### Phase Dependencies

```
Phase 1 (Setup)
    ↓
Phase 2 (Foundational) ← BLOCKS ALL user stories
    ↓
┌───────────────────────────────────────────┐
│  User Stories can run in parallel:        │
│  Phase 3 (US1) ← MVP                      │
│  Phase 4 (US2) ← depends on US1 complete  │
│  Phase 5 (US5) ← depends on US1 complete  │
│  Phase 6 (US3) ← depends on US1 complete  │
│  Phase 7 (US4) ← depends on US1 complete  │
└───────────────────────────────────────────┘
    ↓
Phase 8 (Codegen) ← depends on US1 complete
    ↓
Phase 9 (Polish) ← depends on all desired stories
```

### User Story Dependencies

| Story | Depends On | Can Start After |
|-------|------------|-----------------|
| US1 (P1) | Foundational | Phase 2 complete |
| US2 (P2) | US1 | Phase 3 complete |
| US5 (P2) | US1 | Phase 3 complete |
| US3 (P3) | US1 | Phase 3 complete |
| US4 (P4) | US1 | Phase 3 complete |

### Parallel Opportunities

**Within Phase 2 (Foundational)**:
- T005, T006, T007, T008 (contract tests) - all parallel
- T009-T015 (core types) - sequential (same file dependencies)
- T021 (adapter test) - parallel with core types

**After Phase 2**:
- US2, US5, US3, US4 can all start in parallel once US1 is complete

---

## Parallel Example: Foundational Contract Tests

```bash
# Launch all contract tests in parallel:
Task: "Contract test: parse valid theme document in tests/contract/theme_contracts.rs"
Task: "Contract test: validation errors for invalid themes in tests/contract/theme_contracts.rs"
Task: "Contract test: theme file discovery in tests/contract/theme_contracts.rs"
Task: "Contract test: backward compatibility in tests/contract/theme_contracts.rs"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup (T001-T004)
2. Complete Phase 2: Foundational (T005-T026)
3. Complete Phase 3: US1 - Apply Theme (T027-T036)
4. **STOP and VALIDATE**: Run `cargo test`, test with example app
5. Demo: App loads theme from theme.dampen

### Incremental Delivery

| Increment | Stories | Deliverable |
|-----------|---------|-------------|
| MVP | US1 | Basic theme loading |
| +Runtime | US1+US2 | Theme toggle |
| +Hot-Reload | US1+US2+US5 | Developer experience |
| +Custom | US1+US2+US5+US3 | Brand customization |
| +Overrides | All | Fine-grained control |
| +Codegen | All + Phase 8 | Production ready |

### Task Count Summary

| Phase | Tasks | Parallel |
|-------|-------|----------|
| Setup | 4 | 3 |
| Foundational | 22 | 6 |
| US1 (P1) | 10 | 1 |
| US2 (P2) | 7 | 1 |
| US5 (P2) | 6 | 1 |
| US3 (P3) | 5 | 0 |
| US4 (P4) | 5 | 0 |
| Codegen | 7 | 1 |
| Polish | 9 | 5 |
| **Total** | **75** | **18** |

---

## Notes

- Contract tests MUST fail before implementation (TDD)
- Verify `cargo clippy --workspace -- -D warnings` after each phase
- Commit after each logical task group
- US1 is the MVP - stop there for minimum viable theming
- All crate modifications must maintain backward compatibility
