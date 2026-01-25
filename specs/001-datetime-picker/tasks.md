# Tasks: DatePicker & TimePicker Widgets

**Input**: Design documents from `/specs/001-datetime-picker/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/

**Tests**: Tests are included following TDD approach per Constitution Principle V.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (US1, US2, US3, US4, US5)
- Include exact file paths in descriptions

## Path Conventions

- **dampen-core**: `crates/dampen-core/src/`
- **dampen-iced**: `crates/dampen-iced/src/`
- **Tests**: `crates/*/tests/`

---

## Phase 1: Setup

**Purpose**: Add dependencies and configure workspace for datetime widgets

- [ ] T001 Add `chrono = { version = "0.4", features = ["serde"] }` dependency to `crates/dampen-core/Cargo.toml`
- [ ] T002 Add `iced_aw = { version = "0.13", default-features = false, features = ["date_picker", "time_picker"] }` and `chrono` to `crates/dampen-iced/Cargo.toml`
- [ ] T003 [P] Verify workspace compiles with `cargo build --workspace`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core IR and schema definitions that ALL user stories depend on

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [ ] T004 Add `DatePicker` and `TimePicker` variants to `WidgetKind` enum in `crates/dampen-core/src/ir/node.rs`
- [ ] T005 Add `"date_picker"` and `"time_picker"` tag names to `WidgetKind::all_standard()` method in `crates/dampen-core/src/ir/node.rs`
- [ ] T006 Add `minimum_version()` entries returning `Version::new(1, 1, 0)` for DatePicker and TimePicker in `crates/dampen-core/src/ir/node.rs`
- [ ] T007 [P] Add DatePicker schema to `get_widget_schema()` with optional attrs `["value", "format", "show", "min_date", "max_date"]` and events `["on_submit", "on_cancel"]` in `crates/dampen-core/src/schema/mod.rs`
- [ ] T008 [P] Add TimePicker schema to `get_widget_schema()` with optional attrs `["value", "format", "show", "use_24h", "show_seconds"]` and events `["on_submit", "on_cancel"]` in `crates/dampen-core/src/schema/mod.rs`
- [ ] T009 Add widget kind name mappings for `DatePicker => "date_picker"` and `TimePicker => "time_picker"` in `crates/dampen-core/src/parser/mod.rs`
- [ ] T010 Add child count validation requiring exactly 1 child for DatePicker/TimePicker in parser validation in `crates/dampen-core/src/parser/mod.rs`

**Checkpoint**: Foundation ready - both widget types recognized by parser, schema validated

---

## Phase 3: User Story 1 - Select a Date from Calendar (Priority: P1) 🎯 MVP

**Goal**: Enable `<date_picker>` XML element with calendar overlay and on_submit/on_cancel events

**Independent Test**: Define `<date_picker>` with button underlay, verify calendar renders and date selection fires event

### Tests for User Story 1

- [ ] T011 [P] [US1] Create parser test for valid `<date_picker>` with child button in `crates/dampen-core/tests/parse_date_picker.rs`
- [ ] T012 [P] [US1] Create parser test for `<date_picker>` with zero children returns error in `crates/dampen-core/tests/parse_date_picker.rs`
- [ ] T013 [P] [US1] Create parser test for `<date_picker>` with multiple children returns error in `crates/dampen-core/tests/parse_date_picker.rs`

### Implementation for User Story 1

- [ ] T014 [US1] Create `build_date_picker()` method in `crates/dampen-iced/src/builder/widgets/date_picker.rs` that wraps child as underlay
- [ ] T015 [US1] Implement `show` attribute binding resolution for overlay visibility in `crates/dampen-iced/src/builder/widgets/date_picker.rs`
- [ ] T016 [US1] Implement `on_submit` event handler that serializes Date to ISO 8601 string in `crates/dampen-iced/src/builder/widgets/date_picker.rs`
- [ ] T017 [US1] Implement `on_cancel` event handler in `crates/dampen-iced/src/builder/widgets/date_picker.rs`
- [ ] T018 [US1] Add `WidgetKind::DatePicker => self.build_date_picker(node)` match arm in `crates/dampen-iced/src/builder/mod.rs`
- [ ] T019 [US1] Export `date_picker` module from `crates/dampen-iced/src/builder/widgets/mod.rs`
- [ ] T020 [US1] Add codegen for DatePicker in `crates/dampen-core/src/codegen/view.rs` generating iced_aw::DatePicker construction

**Checkpoint**: DatePicker renders calendar overlay, fires on_submit with ISO date string

---

## Phase 4: User Story 2 - Select a Time (Priority: P1)

**Goal**: Enable `<time_picker>` XML element with time selector overlay, use_24h, and show_seconds options

**Independent Test**: Define `<time_picker>` with button underlay, verify time picker renders with 12/24h formats

### Tests for User Story 2

- [ ] T021 [P] [US2] Create parser test for valid `<time_picker>` with child button in `crates/dampen-core/tests/parse_time_picker.rs`
- [ ] T022 [P] [US2] Create parser test for `<time_picker use_24h="true">` attribute parsing in `crates/dampen-core/tests/parse_time_picker.rs`
- [ ] T023 [P] [US2] Create parser test for `<time_picker show_seconds="true">` attribute parsing in `crates/dampen-core/tests/parse_time_picker.rs`

### Implementation for User Story 2

- [ ] T024 [US2] Create `build_time_picker()` method in `crates/dampen-iced/src/builder/widgets/time_picker.rs` that wraps child as underlay
- [ ] T025 [US2] Implement `use_24h` boolean attribute resolution calling `.use_24h()` method in `crates/dampen-iced/src/builder/widgets/time_picker.rs`
- [ ] T026 [US2] Implement `show_seconds` boolean attribute resolution calling `.show_seconds()` method in `crates/dampen-iced/src/builder/widgets/time_picker.rs`
- [ ] T027 [US2] Implement `on_submit` event handler that serializes Time to HH:MM:SS string in `crates/dampen-iced/src/builder/widgets/time_picker.rs`
- [ ] T028 [US2] Add `WidgetKind::TimePicker => self.build_time_picker(node)` match arm in `crates/dampen-iced/src/builder/mod.rs`
- [ ] T029 [US2] Export `time_picker` module from `crates/dampen-iced/src/builder/widgets/mod.rs`
- [ ] T030 [US2] Add codegen for TimePicker in `crates/dampen-core/src/codegen/view.rs` generating iced_aw::TimePicker construction

**Checkpoint**: TimePicker renders with 12/24h toggle and optional seconds

---

## Phase 5: User Story 3 - Custom Date/Time Formats (Priority: P2)

**Goal**: Support `format` attribute for parsing non-ISO static date/time values

**Independent Test**: Define DatePicker with `value="23/01/2026" format="%d/%m/%Y"`, verify correct parsing

### Tests for User Story 3

- [ ] T031 [P] [US3] Create test for DatePicker with custom format `%d/%m/%Y` in `crates/dampen-core/tests/parse_date_picker.rs`
- [ ] T032 [P] [US3] Create test for TimePicker with custom format `%I:%M %p` in `crates/dampen-core/tests/parse_time_picker.rs`
- [ ] T033 [P] [US3] Create test for invalid date format producing actionable error in `crates/dampen-core/tests/parse_date_picker.rs`

### Implementation for User Story 3

- [ ] T034 [US3] Implement static date parsing with `format` attribute using chrono in `crates/dampen-iced/src/builder/widgets/date_picker.rs`
- [ ] T035 [US3] Implement static time parsing with `format` attribute using chrono in `crates/dampen-iced/src/builder/widgets/time_picker.rs`
- [ ] T036 [US3] Add `ParseError::InvalidDateFormat` variant with value, format, and suggestion in `crates/dampen-core/src/parser/error.rs`
- [ ] T037 [US3] Add `ParseError::InvalidTimeFormat` variant with value, format, and suggestion in `crates/dampen-core/src/parser/error.rs`
- [ ] T038 [US3] Update codegen to handle format attribute in generated date/time parsing in `crates/dampen-core/src/codegen/view.rs`

**Checkpoint**: Custom formats parse correctly, invalid formats produce helpful errors

---

## Phase 6: User Story 4 - Data Binding for Dates and Times (Priority: P2)

**Goal**: Support binding expressions for `value` attribute to sync with application state

**Independent Test**: Define DatePicker with `value="{model.date}"`, verify binding resolves and updates

### Tests for User Story 4

- [ ] T039 [P] [US4] Create test for DatePicker with binding `value="{selected_date}"` in `crates/dampen-core/tests/parse_date_picker.rs`
- [ ] T040 [P] [US4] Create test for TimePicker with binding `value="{selected_time}"` in `crates/dampen-core/tests/parse_time_picker.rs`

### Implementation for User Story 4

- [ ] T041 [US4] Implement `value` attribute binding resolution for DatePicker (binding vs static detection) in `crates/dampen-iced/src/builder/widgets/date_picker.rs`
- [ ] T042 [US4] Implement `value` attribute binding resolution for TimePicker in `crates/dampen-iced/src/builder/widgets/time_picker.rs`
- [ ] T043 [US4] Update codegen to generate binding access for dynamic date/time values in `crates/dampen-core/src/codegen/view.rs`

**Checkpoint**: Bound date/time values resolve from model, updates propagate

---

## Phase 7: User Story 5 - Date Range Constraints (Priority: P3)

**Goal**: Support `min_date` and `max_date` attributes to constrain selectable dates

**Independent Test**: Define DatePicker with `min_date="2026-01-01" max_date="2026-12-31"`, verify constraint enforcement

### Tests for User Story 5

- [ ] T044 [P] [US5] Create test for DatePicker with `min_date` attribute in `crates/dampen-core/tests/parse_date_picker.rs`
- [ ] T045 [P] [US5] Create test for DatePicker with `max_date` attribute in `crates/dampen-core/tests/parse_date_picker.rs`
- [ ] T046 [P] [US5] Create test for `min_date > max_date` producing validation error in `crates/dampen-core/tests/parse_date_picker.rs`

### Implementation for User Story 5

- [ ] T047 [US5] Implement `min_date` attribute parsing and validation in `crates/dampen-iced/src/builder/widgets/date_picker.rs`
- [ ] T048 [US5] Implement `max_date` attribute parsing and validation in `crates/dampen-iced/src/builder/widgets/date_picker.rs`
- [ ] T049 [US5] Add `ParseError::InvalidDateRange` variant for min > max case in `crates/dampen-core/src/parser/error.rs`
- [ ] T050 [US5] Update codegen to pass min/max constraints to iced_aw DatePicker in `crates/dampen-core/src/codegen/view.rs`

**Checkpoint**: Date range constraints validated and enforced in UI

---

## Phase 8: Polish & Cross-Cutting Concerns

**Purpose**: Example application, documentation, and final validation

- [ ] T051 [P] Create example application scaffold in `examples/datetime-picker/Cargo.toml`
- [ ] T052 [P] Create example main.rs with Model containing date/time state in `examples/datetime-picker/src/main.rs`
- [ ] T053 Create example window.dampen demonstrating both pickers in `examples/datetime-picker/src/ui/window.dampen`
- [ ] T054 Add rustdoc comments to `build_date_picker()` function in `crates/dampen-iced/src/builder/widgets/date_picker.rs`
- [ ] T055 [P] Add rustdoc comments to `build_time_picker()` function in `crates/dampen-iced/src/builder/widgets/time_picker.rs`
- [ ] T056 Run `cargo clippy --workspace -- -D warnings` and fix any warnings
- [ ] T057 Run `cargo fmt --all` to format code
- [ ] T058 Run `cargo test --workspace` to verify all tests pass
- [ ] T059 Validate quickstart.md examples compile and run correctly

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup - BLOCKS all user stories
- **User Stories (Phases 3-7)**: All depend on Foundational phase completion
- **Polish (Phase 8)**: Depends on at least US1+US2 being complete

### User Story Dependencies

- **User Story 1 (P1)**: DatePicker core - Can start after Foundational
- **User Story 2 (P1)**: TimePicker core - Can start after Foundational (parallel with US1)
- **User Story 3 (P2)**: Custom formats - Depends on US1+US2 implementation files existing
- **User Story 4 (P2)**: Data binding - Depends on US1+US2 implementation files existing
- **User Story 5 (P3)**: Date range - Depends on US1 DatePicker implementation

### Within Each User Story

- Tests MUST be written and FAIL before implementation
- Builder implementation before mod.rs export
- mod.rs export before codegen
- Core implementation before integration

### Parallel Opportunities

- T007 + T008 (both schemas) can run in parallel
- T011 + T012 + T013 (US1 tests) can run in parallel
- T021 + T022 + T023 (US2 tests) can run in parallel
- US1 and US2 can be developed in parallel after Foundational
- T051 + T052 + T054 + T055 (Polish) can run in parallel

---

## Parallel Example: User Stories 1 & 2 (After Foundational)

```text
# Both stories can start in parallel:

# Developer A - User Story 1 (DatePicker):
Task: T011 [P] [US1] Create parser test for valid <date_picker>
Task: T012 [P] [US1] Create parser test for zero children
Task: T013 [P] [US1] Create parser test for multiple children
# Then sequential: T014 → T015 → T016 → T017 → T018 → T019 → T020

# Developer B - User Story 2 (TimePicker):
Task: T021 [P] [US2] Create parser test for valid <time_picker>
Task: T022 [P] [US2] Create parser test for use_24h attribute
Task: T023 [P] [US2] Create parser test for show_seconds attribute
# Then sequential: T024 → T025 → T026 → T027 → T028 → T029 → T030
```

---

## Implementation Strategy

### MVP First (User Stories 1 + 2 Only)

1. Complete Phase 1: Setup (T001-T003)
2. Complete Phase 2: Foundational (T004-T010)
3. Complete Phase 3: User Story 1 - DatePicker (T011-T020)
4. Complete Phase 4: User Story 2 - TimePicker (T021-T030)
5. **STOP and VALIDATE**: Both pickers work with static values and events
6. Deploy/demo if ready - This is a functional MVP

### Incremental Delivery

1. Setup + Foundational → Foundation ready
2. Add User Stories 1+2 → Basic pickers work → MVP!
3. Add User Story 3 → Custom formats supported
4. Add User Story 4 → Data binding works
5. Add User Story 5 → Date constraints enforced
6. Each story adds value without breaking previous stories

### Suggested MVP Scope

**Minimum**: User Stories 1 + 2 (DatePicker + TimePicker core functionality)
- Both pickers render with underlay pattern
- on_submit and on_cancel events work
- Static values supported with ISO defaults
- ~30 tasks (T001-T030)

---

## Notes

- [P] tasks = different files, no dependencies
- [Story] label maps task to specific user story
- Each user story is independently completable and testable
- Verify tests fail before implementing (TDD per Constitution)
- Commit after each task or logical group
- Stop at any checkpoint to validate story independently
- Run `cargo clippy` frequently to catch issues early
