# Tasks: Implement Real Iced Widgets

**Feature Branch**: `005-implement-real-widgets`  
**Date**: 2026-01-04  
**Plan**: [plan.md](./plan.md)  
**Spec**: [spec.md](./spec.md)

## Overview

This feature implements six interactive Iced widgets to replace placeholders in `crates/gravity-iced/src/builder.rs`. All widgets use the existing `HandlerMessage::Handler(name, Option<String>)` pattern for events.

## Implementation Strategy

**MVP Scope**: User Story 1 (text_input) - can be delivered independently  
**Delivery Order**: P1 widgets → P2 widgets → P3 widget + integration  
**Testing**: Each user story includes unit tests and can be validated independently

## Dependencies

```text
Setup → Foundational → US1 → US2 → US3 → US4 → US5 → US6 → Polish
```

All user stories are independent after foundational work. US1-3 (P1) are required for todo-app validation.

## Phase 1: Setup

Initialize the project structure and verify environment.

- [X] T001 Verify Rust toolchain (1.75+) and Iced 0.14 availability  
  File: `crates/gravity-iced/Cargo.toml`
- [X] T002 Review existing builder.rs placeholder methods  
  File: `crates/gravity-iced/src/builder.rs`
- [X] T003 Review existing test patterns in gravity-iced tests  
  File: `crates/gravity-iced/tests/builder_basic_tests.rs`

## Phase 2: Foundational

Prerequisites that block all user stories.

- [X] T004 [P] Verify Iced 0.14 widget imports are available  
  File: `crates/gravity-iced/src/builder.rs` (imports section)
- [X] T005 [P] Review HandlerMessage usage patterns in existing code  
  File: `crates/gravity-iced/src/lib.rs`
- [X] T006 [P] Review evaluate_attribute method for binding support  
  File: `crates/gravity-iced/src/builder.rs:457`

## Phase 3: User Story 1 - Text Input Widget (P1)

**Goal**: Implement `build_text_input` with placeholder, value binding, and on_input event support.

**Independent Test**: Create `.gravity` file with text input bound to model field. Typing should update model, model changes should update display.

**Acceptance Scenarios**:
1. Renders with value from model binding
2. Calls handler with new text on input
3. Shows placeholder when empty
4. Applies style classes

### Tests (if TDD approach)

- [X] T007 [P] [US1] Create unit test for text_input with static value  
  File: `crates/gravity-iced/tests/widget_tests.rs`
- [X] T008 [P] [US1] Create test for text_input with binding evaluation  
  File: `crates/gravity-iced/tests/widget_tests.rs`
- [X] T009 [P] [US1] Create test for text_input on_input event handler  
  File: `crates/gravity-iced/tests/widget_tests.rs`

### Implementation

- [X] T010 [P] [US1] Implement build_text_input method in builder.rs  
  File: `crates/gravity-iced/src/builder.rs:933`
- [X] T011 [P] [US1] Add placeholder attribute evaluation  
  File: `crates/gravity-iced/src/builder.rs:933`
- [X] T012 [P] [US1] Add value binding evaluation  
  File: `crates/gravity-iced/src/builder.rs:933`
- [X] T013 [P] [US1] Add on_input event handler connection  
  File: `crates/gravity-iced/src/builder.rs:933`
- [X] T014 [US1] Apply style and layout to text_input widget  
  File: `crates/gravity-iced/src/builder.rs:933`
- [X] T015 [US1] Run tests for text_input widget  
  File: `crates/gravity-iced/tests/widget_tests.rs`

## Phase 4: User Story 2 - Checkbox Widget (P1)

**Goal**: Implement `build_checkbox` with label, checked binding, and on_toggle event support.

**Independent Test**: Create `.gravity` file with checkbox bound to boolean model field. Clicking should toggle model value.

**Acceptance Scenarios**:
1. Renders with checked state from model
2. Calls handler with new boolean value on toggle
3. Shows label text next to checkbox

### Tests (if TDD approach)

- [X] T016 [P] [US2] Create test for checkbox with static checked state  
  File: `crates/gravity-iced/tests/widget_tests.rs`
- [X] T017 [P] [US2] Create test for checkbox with binding evaluation  
  File: `crates/gravity-iced/tests/widget_tests.rs`
- [X] T018 [P] [US2] Create test for checkbox on_toggle event handler  
  File: `crates/gravity-iced/tests/widget_tests.rs`

### Implementation

- [X] T019 [P] [US2] Implement build_checkbox method in builder.rs  
  File: `crates/gravity-iced/src/builder.rs:940`
- [X] T020 [P] [US2] Add label attribute evaluation  
  File: `crates/gravity-iced/src/builder.rs:940`
- [X] T021 [P] [US2] Add checked binding evaluation  
  File: `crates/gravity-iced/src/builder.rs:940`
- [X] T022 [P] [US2] Add on_toggle event handler connection  
  File: `crates/gravity-iced/src/builder.rs:940`
- [X] T023 [US2] Apply style and layout to checkbox widget  
  File: `crates/gravity-iced/src/builder.rs:940`
- [X] T024 [US2] Run tests for checkbox widget  
  File: `crates/gravity-iced/tests/widget_tests.rs`

## Phase 5: User Story 3 - Toggler Widget (P1)

**Goal**: Implement `build_toggler` with label, active binding, and on_toggle event support.

**Independent Test**: Create `.gravity` file with toggler bound to boolean model field. Sliding should update model value.

**Acceptance Scenarios**:
1. Renders with active state from model
2. Calls handler with new boolean value on toggle
3. Shows label text next to toggler

### Tests (if TDD approach)

- [X] T025 [P] [US3] Create test for toggler with static active state  
  File: `crates/gravity-iced/tests/widget_tests.rs`
- [X] T026 [P] [US3] Create test for toggler with binding evaluation  
  File: `crates/gravity-iced/tests/widget_tests.rs`
- [X] T027 [P] [US3] Create test for toggler on_toggle event handler  
  File: `crates/gravity-iced/tests/widget_tests.rs`

### Implementation

- [X] T028 [P] [US3] Implement build_toggler method in builder.rs  
  File: `crates/gravity-iced/src/builder.rs:947`
- [X] T029 [P] [US3] Add label attribute evaluation  
  File: `crates/gravity-iced/src/builder.rs:947`
- [X] T030 [P] [US3] Add active binding evaluation  
  File: `crates/gravity-iced/src/builder.rs:947`
- [X] T031 [P] [US3] Add on_toggle event handler connection  
  File: `crates/gravity-iced/src/builder.rs:947`
- [X] T032 [US3] Apply style and layout to toggler widget  
  File: `crates/gravity-iced/src/builder.rs:947`
- [X] T033 [US3] Run tests for toggler widget  
  File: `crates/gravity-iced/tests/widget_tests.rs`

## Phase 6: User Story 4 - Pick List Widget (P2)

**Goal**: Implement `build_pick_list` with options parsing, selected binding, and on_select event support.

**Independent Test**: Create `.gravity` file with pick_list showing options. Selecting should update model field.

**Acceptance Scenarios**:
1. Renders dropdown with parsed options
2. Shows selected option from model
3. Calls handler with selected value
4. Shows placeholder when nothing selected

### Tests (if TDD approach)

- [X] T034 [P] [US4] Create test for pick_list with options parsing  
  File: `crates/gravity-iced/tests/widget_tests.rs`
- [X] T035 [P] [US4] Create test for pick_list with selected binding  
  File: `crates/gravity-iced/tests/widget_tests.rs`
- [X] T036 [P] [US4] Create test for pick_list on_select event handler  
  File: `crates/gravity-iced/tests/widget_tests.rs`

### Implementation

- [X] T037 [P] [US4] Implement build_pick_list method in builder.rs  
  File: `crates/gravity-iced/src/builder.rs:954`
- [X] T038 [P] [US4] Add options attribute parsing (comma-separated)  
  File: `crates/gravity-iced/src/builder.rs:954`
- [X] T039 [P] [US4] Add selected binding evaluation  
  File: `crates/gravity-iced/src/builder.rs:954`
- [X] T040 [P] [US4] Add placeholder attribute evaluation  
  File: `crates/gravity-iced/src/builder.rs:954`
- [X] T041 [P] [US4] Add on_select event handler connection  
  File: `crates/gravity-iced/src/builder.rs:954`
- [X] T042 [US4] Apply style and layout to pick_list widget  
  File: `crates/gravity-iced/src/builder.rs:954`
- [X] T043 [US4] Run tests for pick_list widget  
  File: `crates/gravity-iced/tests/widget_tests.rs`

## Phase 7: User Story 5 - Slider Widget (P2)

**Goal**: Implement `build_slider` with min/max/value binding, and on_change event support.

**Independent Test**: Create `.gravity` file with slider bound to numeric model field. Dragging should update model value.

**Acceptance Scenarios**:
1. Renders with correct range and initial value
2. Calls handler with new numeric value during drag
3. Clamps values to min/max range

### Tests (if TDD approach)

- [X] T044 [P] [US5] Create test for slider with min/max/value attributes  
  File: `crates/gravity-iced/tests/widget_tests.rs`
- [X] T045 [P] [US5] Create test for slider with binding evaluation  
  File: `crates/gravity-iced/tests/widget_tests.rs`
- [X] T046 [P] [US5] Create test for slider on_change event handler  
  File: `crates/gravity-iced/tests/widget_tests.rs`

### Implementation

- [X] T047 [P] [US5] Implement build_slider method in builder.rs  
  File: `crates/gravity-iced/src/builder.rs:961`
- [X] T048 [P] [US5] Add min/max attribute parsing  
  File: `crates/gravity-iced/src/builder.rs:961`
- [X] T049 [P] [US5] Add value binding evaluation  
  File: `crates/gravity-iced/src/builder.rs:961`
- [X] T050 [P] [US5] Add on_change event handler connection  
  File: `crates/gravity-iced/src/builder.rs:961`
- [X] T051 [US5] Add value clamping to [min, max] range  
  File: `crates/gravity-iced/src/builder.rs:961`
- [X] T052 [US5] Apply style and layout to slider widget  
  File: `crates/gravity-iced/src/builder.rs:961`
- [X] T053 [US5] Run tests for slider widget  
  File: `crates/gravity-iced/tests/widget_tests.rs`

## Phase 8: User Story 6 - Image Widget (P3)

**Goal**: Implement `build_image` with src path and optional width/height.

**Independent Test**: Create `.gravity` file with image element pointing to valid file. Image should display at specified dimensions.

**Acceptance Scenarios**:
1. Renders image from file path
2. Sizes to specified width/height
3. Handles invalid paths gracefully

### Tests (if TDD approach)

- [X] T054 [P] [US6] Create test for image with src path  
  File: `crates/gravity-iced/tests/widget_tests.rs`
- [X] T055 [P] [US6] Create test for image with width/height  
  File: `crates/gravity-iced/tests/widget_tests.rs`
- [X] T056 [P] [US6] Create test for image with invalid path  
  File: `crates/gravity-iced/tests/widget_tests.rs`

### Implementation

- [X] T057 [P] [US6] Implement build_image method in builder.rs  
  File: `crates/gravity-iced/src/builder.rs:968`
- [X] T058 [P] [US6] Add src attribute evaluation  
  File: `crates/gravity-iced/src/builder.rs:968`
- [X] T059 [P] [US6] Add width/height attribute parsing  
  File: `crates/gravity-iced/src/builder.rs:968`
- [X] T060 [US6] Apply style and layout to image widget  
  File: `crates/gravity-iced/src/builder.rs:968`
- [X] T061 [US6] Run tests for image widget  
  File: `crates/gravity-iced/tests/widget_tests.rs`

## Phase 9: Integration & Validation

Verify all widgets work together and with existing examples.

- [X] T062 [P] Run all unit tests in gravity-iced  
  File: `crates/gravity-iced/tests/`
- [X] T063 [P] Verify todo-app example compiles  
  File: `examples/todo-app/Cargo.toml`
- [X] T064 [P] Verify todo-app UI renders without errors  
  File: `examples/todo-app/ui/main.gravity`
- [X] T065 [P] Run cargo clippy on gravity-iced crate  
  File: `crates/gravity-iced/`
- [X] T066 [P] Run cargo fmt check on gravity-iced crate  
  File: `crates/gravity-iced/`
- [X] T067 [P] Run existing benchmarks to verify no performance regression  
  File: `crates/gravity-iced/benches/builder_bench.rs`

## Phase 10: Polish & Cross-Cutting Concerns

- [X] T068 [P] Update verbose logging for all new widgets  
  File: `crates/gravity-iced/src/builder.rs`
- [X] T069 [P] Add documentation comments for new widget methods  
  File: `crates/gravity-iced/src/builder.rs`
- [X] T070 [P] Verify error handling for edge cases (empty options, invalid values)  
  File: `crates/gravity-iced/src/builder.rs`
- [X] T071 [P] Create widget showcase example (optional)  
  File: `examples/widget-showcase/ui/inputs.gravity`
- [X] T072 [P] Update any relevant README or documentation  
  File: `crates/gravity-iced/README.md`

## Parallel Execution Opportunities

**US1 (Text Input)** can run in parallel with US2 and US3:
```bash
# Terminal 1
cargo test -p gravity-iced --test widget_tests -- text_input

# Terminal 2 (parallel)
cargo test -p gravity-iced --test widget_tests -- checkbox

# Terminal 3 (parallel)
cargo test -p gravity-iced --test widget_tests -- toggler
```

**US4, US5, US6** can each run independently after US1-3:
```bash
# Each in separate terminal
cargo test -p gravity-iced --test widget_tests -- pick_list
cargo test -p gravity-iced --test widget_tests -- slider
cargo test -p gravity-iced --test widget_tests -- image
```

## Independent Test Criteria

| User Story | Independent Test File | Validation Method |
|------------|----------------------|-------------------|
| US1 - Text Input | `test_text_input.gravity` | Build app, type in input, verify model update |
| US2 - Checkbox | `test_checkbox.gravity` | Build app, click checkbox, verify toggle |
| US3 - Toggler | `test_toggler.gravity` | Build app, click toggler, verify toggle |
| US4 - Pick List | `test_pick_list.gravity` | Build app, select option, verify selection |
| US5 - Slider | `test_slider.gravity` | Build app, drag slider, verify value change |
| US6 - Image | `test_image.gravity` | Build app, verify image displays |

## MVP Recommendation

**MVP Scope**: Phase 3 (US1 - Text Input) only

**Rationale**: 
- Text input is the most fundamental widget
- Validates the entire pattern (binding, events, styling)
- Can be tested independently
- Provides value immediately

**Deliverable**: Working text_input widget with tests, ready for user validation before proceeding to remaining widgets.

## Task Count Summary

- **Total Tasks**: 72
- **Setup**: 3 tasks
- **Foundational**: 3 tasks
- **US1 (P1)**: 9 tasks (including tests)
- **US2 (P1)**: 9 tasks (including tests)
- **US3 (P1)**: 9 tasks (including tests)
- **US4 (P2)**: 9 tasks (including tests)
- **US5 (P2)**: 9 tasks (including tests)
- **US6 (P3)**: 9 tasks (including tests)
- **Integration**: 6 tasks
- **Polish**: 5 tasks
