# Tasks: Gravity Widget Builder

**Feature**: 003-widget-builder  
**Branch**: `003-widget-builder`  
**Date**: 2026-01-03  
**Plan**: [plan.md](plan.md) | **Spec**: [spec.md](spec.md) | **Research**: [research.md](research.md)

---

## Implementation Strategy

**MVP Approach**: Implement core builder functionality first (US1), then expand to full feature set (US3), with centralization (US2) as a natural outcome.

**Key Principle**: Build upon existing infrastructure rather than duplicating it:
- ✅ Reuse existing `style_mapping.rs` functions
- ✅ Reuse existing `render()` and `render_with_layout()` patterns
- ✅ Reuse existing `HandlerRegistry` and `evaluate_binding_expr`
- ✅ Extend existing `gravity-iced/src/lib.rs` exports
- ❌ DO NOT duplicate manual rendering logic from examples

---

## Dependencies

**User Story Order** (must complete in sequence):
1. **US1** (P1): Simplify markup interpretation - Foundation
2. **US3** (P1): Support all current use cases - Validation
3. **US2** (P2): Centralize interpretation logic - Refinement

**Parallel Opportunities**:
- All conversion implementations can be done in parallel
- Widget-specific builders can be parallelized after core builder
- Example simplifications can happen once builder is complete

---

## Phase 1: Setup

- [ ] T001 Verify project structure and existing files
- [ ] T002 Create `crates/gravity-iced/src/builder.rs` (empty file)
- [ ] T003 Create `crates/gravity-iced/src/convert.rs` (empty file)
- [ ] T004 Update `crates/gravity-iced/src/lib.rs` to export builder

---

## Phase 2: Foundational (Prerequisites for All User Stories)

- [ ] T005 [P] Implement `From<StyleProperties>` for `iced::widget::container::Style` in convert.rs
- [ ] T006 [P] Implement `From<Length>` for `iced::Length` in convert.rs
- [ ] T007 [P] Implement `From<Color>` for `iced::Color` in convert.rs
- [ ] T008 [P] Implement `From<Background>` for `iced::Background` in convert.rs
- [ ] T009 [P] Implement `From<Border>` for `iced::Border` in convert.rs
- [ ] T010 [P] Implement `From<Shadow>` for `iced::Shadow` in convert.rs
- [ ] T011 [P] Implement `From<Padding>` for `iced::Padding` in convert.rs
- [ ] T012 [P] Implement `From<BorderRadius>` for `iced::border::Radius` in convert.rs
- [ ] T013 [P] Implement `From<Transform>` for Iced transform (if available) or document limitation
- [ ] T014 Implement `GravityWidgetBuilder::new()` constructor in builder.rs
- [ ] T015 Implement `GravityWidgetBuilder::with_verbose()` configuration method
- [ ] T016 Implement `GravityWidgetBuilder::build()` entry point

---

## Phase 3: User Story 1 - Simplify Markup Interpretation (P1)

**Goal**: Enable single-line UI rendering with automatic binding/evaluation

**Independent Test**: Create a test that renders a simple UI with bindings using only `GravityWidgetBuilder::new(...).build()`

### 3.1: Core Builder Logic

- [ ] T017 [US1] Implement `GravityWidgetBuilder::build_widget()` - recursive dispatcher
- [ ] T018 [US1] Implement `GravityWidgetBuilder::build_text()` - text widget handler
- [ ] T019 [US1] Implement `GravityWidgetBuilder::build_button()` - button widget handler
- [ ] T020 [US1] Implement `GravityWidgetBuilder::build_column()` - column layout handler
- [ ] T021 [US1] Implement `GravityWidgetBuilder::build_row()` - row layout handler
- [ ] T022 [US1] Implement `GravityWidgetBuilder::build_container()` - container handler

### 3.2: Binding Evaluation

- [ ] T023 [US1] Implement `GravityWidgetBuilder::evaluate_property()` for bindings
- [ ] T024 [US1] Integrate `evaluate_binding_expr` from gravity-core
- [ ] T025 [US1] Handle interpolated strings (e.g., "Count: {count}")
- [ ] T026 [US1] Add graceful error handling for binding failures

### 3.3: Event Handling

- [ ] T027 [US1] Implement `GravityWidgetBuilder::connect_events()` for button widgets
- [ ] T028 [US1] Map event names to handler registry lookups
- [ ] T029 [US1] Handle missing handlers gracefully (log warning if verbose)
- [ ] T030 [US1] Support optional handler registry (None = no events)

### 3.4: Style & Layout Application

- [ ] T031 [US1] Implement `GravityWidgetBuilder::apply_styles()` using existing style_mapping
- [ ] T032 [US1] Implement `GravityWidgetBuilder::apply_layout()` using existing style_mapping
- [ ] T033 [US1] Wrap widgets in containers with layout/style when needed

### 3.5: Additional Widget Support

- [ ] T034 [US1] Implement `GravityWidgetBuilder::build_text_input()` - text input handler
- [ ] T035 [US1] Implement `GravityWidgetBuilder::build_checkbox()` - checkbox handler
- [ ] T036 [US1] Implement `GravityWidgetBuilder::build_slider()` - slider handler
- [ ] T037 [US1] Implement `GravityWidgetBuilder::build_pick_list()` - pick list handler
- [ ] T038 [US1] Implement `GravityWidgetBuilder::build_toggler()` - toggler handler
- [ ] T039 [US1] Implement `GravityWidgetBuilder::build_image()` - image handler
- [ ] T040 [US1] Implement `GravityWidgetBuilder::build_scrollable()` - scrollable handler
- [ ] T041 [US1] Implement `GravityWidgetBuilder::build_stack()` - stack handler

### 3.6: Verbose Logging

- [ ] T042 [US1] Add verbose logging to all builder methods
- [ ] T043 [US1] Log binding evaluation results
- [ ] T044 [US1] Log event handler connections
- [ ] T045 [US1] Log style/layout applications
- [ ] T046 [US1] Log errors and warnings

### 3.7: Testing & Validation

- [ ] T047 [US1] Create unit tests for all From conversions
- [ ] T048 [US1] Create integration test for full widget tree building
- [ ] T049 [US1] Test binding evaluation with complex expressions
- [ ] T050 [US1] Test event handler connection
- [ ] T051 [US1] Test verbose logging output
- [ ] T052 [US1] Test graceful degradation (no registry, binding errors)

---

## Phase 4: User Story 3 - Support All Current Use Cases (P1)

**Goal**: Ensure builder handles all scenarios from existing examples

**Independent Test**: Convert `examples/styling/src/main.rs` and `examples/styling/src/state_demo.rs` to use builder, verify identical behavior

### 4.1: State-Based Styling Support

- [ ] T053 [US3] Support hover/active/disabled states in builder
- [ ] T054 [US3] Integrate `WidgetStateManager` from gravity-iced
- [ ] T055 [US3] Apply state-specific styles during rendering
- [ ] T056 [US3] Handle state transitions in event handlers

### 4.2: Complex Binding Scenarios

- [ ] T057 [US3] Test nested field access (e.g., `{user.name}`)
- [ ] T058 [US3] Test method calls in bindings (e.g., `{items.len()}`)
- [ ] T059 [US3] Test binary operations (e.g., `{count * 2}`)
- [ ] T060 [US3] Test conditionals (e.g., `{if active 'Online' 'Offline'}`)
- [ ] T061 [US3] Test formatted strings (e.g., `"Count: {count}"`)

### 4.3: Event Handler Variations

- [ ] T062 [US3] Support simple handlers (no payload)
- [ ] T063 [US3] Support value handlers (with payload)
- [ ] T064 [US3] Support command handlers (returning messages)
- [ ] T065 [US3] Test handler signature validation

### 4.4: Layout & Style Edge Cases

- [ ] T066 [US3] Test missing attributes (use defaults)
- [ ] T067 [US3] Test invalid attribute values (graceful fallback)
- [ ] T068 [US3] Test deeply nested widgets
- [ ] T069 [US3] Test empty containers
- [ ] T070 [US3] Test mixed widget types

### 4.5: Performance Validation

- [ ] T071 [US3] Benchmark 1000 widget rendering
- [ ] T072 [US3] Verify < 50ms target
- [ ] T073 [US3] Profile binding evaluation overhead
- [ ] T074 [US3] Profile event connection overhead

### 4.6: Example Simplification

- [ ] T075 [US3] Simplify `examples/styling/src/main.rs` to < 50 lines
- [ ] T076 [US3] Simplify `examples/styling/src/state_demo.rs` to < 50 lines
- [ ] T077 [US3] Simplify `examples/counter/src/main.rs` to < 50 lines
- [ ] T078 [US3] Simplify `examples/todo-app/src/main.rs` to < 50 lines
- [ ] T079 [US3] Verify all simplified examples work identically

---

## Phase 5: User Story 2 - Centralize Interpretation Logic (P2)

**Goal**: Ensure all interpretation logic is in builder, not duplicated

**Independent Test**: Add a new widget type to gravity-core IR, verify only builder needs updating

### 5.1: Architecture Validation

- [ ] T080 [US2] Audit all examples for interpretation logic
- [ ] T081 [US2] Document any remaining manual rendering code
- [ ] T082 [US2] Refactor any remaining manual code to use builder

### 5.2: Extensibility

- [ ] T083 [US2] Test adding new widget type to IR
- [ ] T084 [US2] Verify only builder.rs needs updating
- [ ] T085 [US2] Verify examples don't need changes
- [ ] T086 [US2] Document extension pattern for future widgets

### 5.3: Backend Abstraction

- [ ] T087 [US2] Verify no Iced types leak into gravity-core
- [ ] T088 [US2] Verify builder is only in gravity-iced
- [ ] T089 [US2] Document pattern for alternative backends

---

## Phase 6: Polish & Cross-Cutting Concerns

### 6.1: Documentation

- [ ] T090 Update `crates/gravity-iced/README.md` with builder usage
- [ ] T091 Add rustdoc comments to all public builder items
- [ ] T092 Create example in `examples/builder-demo/`
- [ ] T093 Update `docs/QUICKSTART.md` with builder example

### 6.2: Error Handling

- [ ] T094 Implement error overlay support (FR-014)
- [ ] T095 Add error types for builder failures
- [ ] T096 Test error display in verbose mode
- [ ] T097 Test error overlay in dev mode

### 6.3: Performance Optimization

- [ ] T098 Profile and optimize hot paths
- [ ] T099 Add memoization for repeated conversions
- [ ] T100 Verify no unnecessary allocations
- [ ] T101 Run full benchmark suite

### 6.4: Code Quality

- [ ] T102 Run `cargo clippy --workspace`
- [ ] T103 Run `cargo fmt --all -- --check`
- [ ] T104 Fix all warnings and errors
- [ ] T105 Ensure 90%+ test coverage

### 6.5: Integration

- [ ] T106 Test with hot-reload (verify reload still works)
- [ ] T107 Test with CLI dev command
- [ ] T108 Test with existing examples
- [ ] T109 Verify no breaking changes

---

## Parallel Execution Examples

### Parallel Block 1: From Conversions (Phase 2)
```bash
# Can be done in parallel - no dependencies between them
T005: From<StyleProperties>
T006: From<Length>
T007: From<Color>
T008: From<Background>
T009: From<Border>
T010: From<Shadow>
T011: From<Padding>
T012: From<BorderRadius>
T013: From<Transform>
```

### Parallel Block 2: Widget Handlers (Phase 3)
```bash
# Can be done in parallel after core builder is ready
T018: build_text()
T019: build_button()
T020: build_column()
T021: build_row()
T022: build_container()
T034: build_text_input()
T035: build_checkbox()
T036: build_slider()
T037: build_pick_list()
T038: build_toggler()
T039: build_image()
T040: build_scrollable()
T041: build_stack()
```

### Parallel Block 3: Example Simplification (Phase 4)
```bash
# Can be done in parallel once builder is complete
T075: Simplify styling/main.rs
T076: Simplify styling/state_demo.rs
T077: Simplify counter/main.rs
T078: Simplify todo-app/main.rs
```

---

## Success Criteria Checklist

- [ ] All tasks follow format: `- [ ] TXXX [P?] [US?] Description with file path`
- [ ] Each user story has independent test criteria
- [ ] Tasks are organized by story for independent implementation
- [ ] Parallel tasks are marked with [P]
- [ ] No duplication of existing code
- [ ] All file paths are absolute and correct
- [ ] MVP scope is clear (US1 only for initial delivery)
- [ ] Dependencies are documented

---

## MVP Scope

**Minimum Viable Product**: Phase 3 (User Story 1) only

**Deliverables**:
- `GravityWidgetBuilder` with core functionality
- Support for basic widgets (Text, Button, Column, Row, Container)
- Binding evaluation
- Event handling
- Style/layout application
- Verbose logging
- Simplified `examples/styling/src/main.rs`

**Out of MVP**:
- All widgets from Phase 4 (can be added incrementally)
- State-based styling (Phase 4)
- Example simplifications beyond main one (Phase 4)
- Performance optimization (Phase 6)
- Error overlay (Phase 6)

**Rationale**: US1 provides immediate value (single-line UI rendering) and validates the approach before investing in full feature coverage.

---

## Next Steps

1. **Start with Phase 1** (Setup) - 4 tasks
2. **Proceed to Phase 2** (Foundational) - 12 tasks, all parallel
3. **Implement Phase 3** (US1) - 36 tasks, sequential within story
4. **Validate with US3** tests
5. **Refine with US2** if needed
6. **Polish in Phase 6**

**Estimated Effort**: ~100 tasks, ~2-3 weeks for full implementation
**MVP Effort**: ~52 tasks, ~1 week
