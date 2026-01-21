# Tasks: dampen-iced Crate Refactoring

**Feature**: dampen-iced Crate Refactoring  
**Date**: 2026-01-21  
**Input**: Design documents from `/specs/001-dampen-iced-refactor/`  
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/

**Organization**: Tasks follow research-recommended order (P1→P4→P5→P2→P3→P6) in 3-4 PRs for optimal risk management.

**Tests**: NOT REQUESTED - This refactoring relies on existing 148 tests. No new test tasks included.

---

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (US1-US6, based on spec priorities P1-P6)
- Include exact file paths in descriptions

---

## Phase 1: Setup & Baseline

**Purpose**: Establish baseline and prepare for refactoring

- [ ] T001 Run baseline test suite: `cargo test --workspace` (expect 148/148 pass)
- [ ] T002 [P] Run baseline clippy: `cargo clippy --workspace -- -D warnings` (expect 0 warnings)
- [ ] T003 [P] Measure baseline LOC: Count lines in `crates/dampen-iced/src/`
- [ ] T004 [P] Run baseline benchmarks (if available): Document current render times for 1000 widgets

**Checkpoint**: Baseline metrics captured for comparison

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Create helper module structure that all user stories will use

**⚠️ CRITICAL**: No user story implementation can begin until this phase is complete

- [ ] T005 Create `crates/dampen-iced/src/builder/helpers.rs` with module documentation
- [ ] T006 Add `pub(crate) mod helpers;` to `crates/dampen-iced/src/builder/mod.rs`
- [ ] T007 Add error type `HandlerResolutionError` struct to `crates/dampen-iced/src/builder/helpers.rs` per contract
- [ ] T008 Implement `Display` and `Error` traits for `HandlerResolutionError` per contract

**Checkpoint**: Foundation ready - user story implementation can now begin

---

## Phase 3: User Story 1 - Remove Legacy IcedBackend (Priority: P1) 🎯 PR #1

**Goal**: Remove 128 lines of legacy IcedBackend trait code

**Independent Test**: Verify examples still compile with `cargo build --examples`

### Implementation for User Story 1

- [ ] T009 [US1] Add deprecation warning to `IcedBackend` struct in `crates/dampen-iced/src/lib.rs:27-41`
  ```rust
  #[deprecated(since = "0.2.7", note = "Use DampenWidgetBuilder instead. See migration guide in docs.")]
  pub struct IcedBackend { ... }
  ```

- [ ] T010 [US1] Add deprecation warning to `Backend` impl in `crates/dampen-iced/src/lib.rs:63-190`
  ```rust
  #[allow(deprecated)]
  impl Backend for IcedBackend { ... }
  ```

- [ ] T011 [US1] Update public API re-exports in `crates/dampen-iced/src/lib.rs` to mark IcedBackend as deprecated

- [ ] T012 [US1] Search workspace for IcedBackend usage: `rg "IcedBackend" --type rust`

- [ ] T013 [US1] Migrate test file `crates/dampen-iced/tests/backend_tests.rs` from IcedBackend to DampenWidgetBuilder

- [ ] T014 [US1] Migrate test file `crates/dampen-iced/tests/integration_tests.rs` from IcedBackend to DampenWidgetBuilder

- [ ] T015 [US1] Migrate test file `crates/dampen-iced/tests/radio_widget_tests.rs` from IcedBackend to DampenWidgetBuilder

- [ ] T016 [US1] Run test suite: `cargo test --workspace` (expect 148/148 pass with deprecation warnings)

- [ ] T017 [US1] Run clippy: `cargo clippy --workspace -- -D warnings` (expect 0 errors, deprecation warnings OK)

- [ ] T018 [US1] Update `crates/dampen-iced/CHANGELOG.md` with deprecation notice for v0.2.7

**Checkpoint**: IcedBackend deprecated, all tests migrated and passing

**Note**: Full removal deferred to v0.3.0 per research decision R6

---

## Phase 4: User Story 4 - Extract Boolean Attribute Helper (Priority: P4) 🎯 PR #2

**Goal**: Eliminate ~50 lines of duplicated boolean parsing logic

**Independent Test**: Test button disabled attribute with various formats

### Implementation for User Story 4

- [ ] T019 [US4] Implement `parse_boolean_string` private helper function in `crates/dampen-iced/src/builder/helpers.rs` per contract
  - Support truthy: "true", "1", "yes", "on" (case-insensitive)
  - Support falsy: "false", "0", "no", "off", "" (case-insensitive)
  - Trim whitespace before parsing
  - Unknown values → return default

- [ ] T020 [US4] Implement `resolve_boolean_attribute` public function in `crates/dampen-iced/src/builder/helpers.rs` per contract
  - Signature: `pub fn resolve_boolean_attribute(node: &WidgetNode, attr_name: &str, default: bool) -> bool`
  - Handle `AttributeValue::Static` → parse with helper
  - Handle `AttributeValue::Binding` → return default (TODO for future)
  - Handle missing attribute → return default

- [ ] T021 [US4] Add rustdoc with examples to `resolve_boolean_attribute` function per contract documentation

- [ ] T022 [US4] Migrate `crates/dampen-iced/src/builder/widgets/button.rs` to use `resolve_boolean_attribute` for "disabled" attribute
  - Replace ~15 lines of manual parsing with single helper call
  - Import: `use crate::builder::helpers::resolve_boolean_attribute;`

- [ ] T023 [US4] Migrate `crates/dampen-iced/src/builder/widgets/radio.rs` to use `resolve_boolean_attribute` for "selected" attribute
  - Replace ~15 lines of manual parsing
  - Import helper function

- [ ] T024 [US4] Migrate `crates/dampen-iced/src/builder/widgets/checkbox.rs` to use `resolve_boolean_attribute` for "checked" attribute
  - Replace ~15 lines of manual parsing
  - Import helper function

- [ ] T025 [US4] Run test suite: `cargo test --workspace` (expect 148/148 pass)

- [ ] T026 [US4] Run clippy: `cargo clippy --workspace -- -D warnings` (expect 0 warnings)

- [ ] T027 [US4] Measure LOC reduction: Count lines removed from 3 widget files

**Checkpoint**: Boolean attribute helper working in 3 widgets, ~50 lines removed

---

## Phase 5: User Story 5 - Extract Handler Resolution Helper (Priority: P5) 🎯 PR #2

**Goal**: Eliminate ~120 lines of duplicated handler parameter resolution logic

**Independent Test**: Test button on_click with context and model bindings

### Implementation for User Story 5

- [ ] T028 [US5] Implement `resolve_handler_param` public function in `crates/dampen-iced/src/builder/helpers.rs` per contract
  - Signature: `pub fn resolve_handler_param<M>(builder: &DampenWidgetBuilder<M>, event_param_expr: &str) -> Result<BindingValue, HandlerResolutionError>`
  - Attempt 1: Context resolution via `builder.resolve_from_context()`
  - Attempt 2: Model evaluation via `evaluate_binding_expr_with_shared()`
  - Return detailed error with context on failure

- [ ] T029 [US5] Add rustdoc with examples to `resolve_handler_param` function per contract documentation

- [ ] T030 [P] [US5] Migrate `crates/dampen-iced/src/builder/widgets/button.rs` to use `resolve_handler_param` for on_click handler
  - Replace ~25 lines of manual resolution
  - Enrich error with handler_name, widget_kind, widget_id, span
  - Import: `use crate::builder::helpers::resolve_handler_param;`

- [ ] T031 [P] [US5] Migrate `crates/dampen-iced/src/builder/widgets/checkbox.rs` to use `resolve_handler_param` for on_change handler
  - Replace ~25 lines of manual resolution
  - Enrich error with widget context

- [ ] T032 [P] [US5] Migrate `crates/dampen-iced/src/builder/widgets/radio.rs` to use `resolve_handler_param` for on_select handler
  - Replace ~25 lines of manual resolution
  - Enrich error with widget context

- [ ] T033 [P] [US5] Migrate `crates/dampen-iced/src/builder/widgets/slider.rs` to use `resolve_handler_param` for on_change handler
  - Replace ~25 lines of manual resolution
  - Enrich error with widget context

- [ ] T034 [P] [US5] Migrate `crates/dampen-iced/src/builder/widgets/text_input.rs` to use `resolve_handler_param` for on_change handler
  - Replace ~25 lines of manual resolution
  - Enrich error with widget context

- [ ] T035 [US5] Run test suite: `cargo test --workspace` (expect 148/148 pass)

- [ ] T036 [US5] Run clippy: `cargo clippy --workspace -- -D warnings` (expect 0 warnings)

- [ ] T037 [US5] Measure LOC reduction: Count lines removed from 5 widget files

**Checkpoint**: Handler resolution helper working in 5 widgets, ~120 lines removed

**PR #2 Complete**: US4 + US5 combined (~170 lines removed total)

---

## Phase 6: User Story 2 - Extract State-Aware Styling Pattern (Priority: P2) 🎯 PR #3

**Goal**: Extract ~200 lines of duplicated state-aware styling logic into reusable helper

**Independent Test**: Verify checkbox hover/focus states still work visually

### Implementation for User Story 2

- [ ] T038 [US2] Implement `create_state_aware_style_fn` generic function in `crates/dampen-iced/src/builder/helpers.rs` per contract
  - Signature: `pub fn create_state_aware_style_fn<Status, Style, F, G>(...) -> impl Fn(&Theme, Status) -> Style`
  - Type parameters: `Status: 'static`, `Style: 'static`, `F: Fn(Status) -> Option<WidgetState> + 'static`, `G: Fn(&StyleProperties) -> Style + 'static`
  - Logic: Map status → WidgetState → resolve state variant → merge styles → apply to widget Style
  - Use existing: `resolve_state_style()`, `merge_style_properties()` from `style_mapping.rs`

- [ ] T039 [US2] Add comprehensive rustdoc with examples to `create_state_aware_style_fn` per contract documentation

- [ ] T040 [US2] Create `apply_checkbox_style` function in `crates/dampen-iced/src/builder/widgets/checkbox.rs` per quickstart example
  - Signature: `fn apply_checkbox_style(props: &StyleProperties) -> checkbox::Style`
  - Apply: background, icon_color, border, text_color from props
  - ~20-30 lines, separate from builder logic

- [ ] T041 [US2] Migrate `crates/dampen-iced/src/builder/widgets/checkbox.rs` to use `create_state_aware_style_fn`
  - Replace ~60 lines of inline closure with helper call
  - Pass: `base_style_props`, `style_class`, `map_checkbox_status`, `apply_checkbox_style`
  - Import: `use crate::builder::helpers::create_state_aware_style_fn;`

- [ ] T042 [US2] Create `apply_radio_style` function in `crates/dampen-iced/src/builder/widgets/radio.rs`
  - Signature: `fn apply_radio_style(props: &StyleProperties) -> radio::Style`
  - Apply: background, dot_color, border, text_color from props

- [ ] T043 [US2] Migrate `crates/dampen-iced/src/builder/widgets/radio.rs` to use `create_state_aware_style_fn`
  - Replace ~60 lines of inline closure
  - Pass: `base_style_props`, `style_class`, `map_radio_status`, `apply_radio_style`

- [ ] T044 [US2] Create `apply_text_input_style` function in `crates/dampen-iced/src/builder/widgets/text_input.rs`
  - Signature: `fn apply_text_input_style(props: &StyleProperties) -> text_input::Style`
  - Apply: background, border, icon_color, placeholder_color, value_color, selection_color from props

- [ ] T045 [US2] Migrate `crates/dampen-iced/src/builder/widgets/text_input.rs` to use `create_state_aware_style_fn`
  - Replace ~60 lines of inline closure
  - Pass: `base_style_props`, `style_class`, `map_text_input_status`, `apply_text_input_style`

- [ ] T046 [US2] Create `apply_toggler_style` function in `crates/dampen-iced/src/builder/widgets/toggler.rs`
  - Signature: `fn apply_toggler_style(props: &StyleProperties) -> toggler::Style`
  - Apply: background, background_border, foreground, foreground_border from props

- [ ] T047 [US2] Migrate `crates/dampen-iced/src/builder/widgets/toggler.rs` to use `create_state_aware_style_fn`
  - Replace ~60 lines of inline closure
  - Pass: `base_style_props`, `style_class`, `map_toggler_status`, `apply_toggler_style`

- [ ] T048 [US2] Run test suite: `cargo test --workspace` (expect 148/148 pass)

- [ ] T049 [US2] Run clippy: `cargo clippy --workspace -- -D warnings` (expect 0 warnings)

- [ ] T050 [US2] Measure LOC reduction: Count lines removed from 4 widget files (expect ~200 lines)

- [ ] T051 [US2] Visual regression test: Run examples and verify hover/focus/active states work for checkbox, radio, text_input, toggler

**Checkpoint**: State-aware styling helper extracted and working in 4 existing widgets

---

## Phase 7: User Story 3 - Implement Missing State-Aware Styling (Priority: P3) 🎯 PR #3

**Goal**: Add state-aware styling to slider, pick_list, combo_box using extracted helper

**Independent Test**: Verify slider shows hover state when mouse hovers

**Depends on**: US2 (P2) must be complete - needs `create_state_aware_style_fn` helper

### Implementation for User Story 3

- [ ] T052 [US3] Create status mapper `map_slider_status` in `crates/dampen-iced/src/style_mapping.rs` if not exists
  - Map `slider::Status::Active` → `WidgetState::Base`
  - Map `slider::Status::Hovered` → `WidgetState::Hover`
  - Map `slider::Status::Dragged` → `WidgetState::Active`

- [ ] T053 [US3] Create `apply_slider_style` function in `crates/dampen-iced/src/builder/widgets/slider.rs`
  - Signature: `fn apply_slider_style(props: &StyleProperties) -> slider::Style`
  - Apply: rail colors, handle shape/color/border from props

- [ ] T054 [US3] Add state-aware styling to `crates/dampen-iced/src/builder/widgets/slider.rs` using `create_state_aware_style_fn`
  - Add: style resolution and helper call (~10 lines)
  - Pass: `base_style_props`, `style_class`, `map_slider_status`, `apply_slider_style`

- [ ] T055 [US3] Create status mapper `map_pick_list_status` in `crates/dampen-iced/src/style_mapping.rs` if not exists
  - Map pick_list status variants to WidgetState (check Iced docs for Status enum)

- [ ] T056 [US3] Create `apply_pick_list_style` function in `crates/dampen-iced/src/builder/widgets/pick_list.rs`
  - Signature: `fn apply_pick_list_style(props: &StyleProperties) -> pick_list::Style`
  - Apply: background, text_color, placeholder_color, handle_color, border from props

- [ ] T057 [US3] Add state-aware styling to `crates/dampen-iced/src/builder/widgets/pick_list.rs` using `create_state_aware_style_fn`
  - Add: style resolution and helper call (~10 lines)
  - Pass: `base_style_props`, `style_class`, `map_pick_list_status`, `apply_pick_list_style`

- [ ] T058 [US3] Create status mapper `map_combo_box_status` in `crates/dampen-iced/src/style_mapping.rs` if not exists
  - Map combo_box status variants to WidgetState (check Iced docs for Status enum)

- [ ] T059 [US3] Create `apply_combo_box_style` function in `crates/dampen-iced/src/builder/widgets/combo_box.rs`
  - Signature: `fn apply_combo_box_style(props: &StyleProperties) -> combo_box::Style`
  - Apply: background, text_color, placeholder_color, border, icon_size from props

- [ ] T060 [US3] Add state-aware styling to `crates/dampen-iced/src/builder/widgets/combo_box.rs` using `create_state_aware_style_fn`
  - Add: style resolution and helper call (~10 lines)
  - Pass: `base_style_props`, `style_class`, `map_combo_box_status`, `apply_combo_box_style`

- [ ] T061 [US3] Run test suite: `cargo test --workspace` (expect 148/148 pass)

- [ ] T062 [US3] Run clippy: `cargo clippy --workspace -- -D warnings` (expect 0 warnings)

- [ ] T063 [US3] Visual regression test: Run examples and verify hover/focus/active states work for slider, pick_list, combo_box

**Checkpoint**: State-aware styling now implemented for all 7 widgets (4 existing + 3 new)

**PR #3 Complete**: US2 + US3 combined (~200 lines removed, state styling coverage complete)

---

## Phase 8: User Story 6 - Optimize Clone Performance (Priority: P6) 🎯 PR #4 (or defer)

**Goal**: Reduce memory usage by ~200KB and improve rendering speed by ~5% for 1000+ widget UIs

**Independent Test**: Run benchmarks before/after to verify performance improvement

**⚠️ HIGH RISK**: This touches all state-aware widgets - should be last after all refactors stable

### Implementation for User Story 6

- [ ] T064 [US6] Run baseline benchmark: Measure clone time and memory usage for StyleClass in state-aware widgets

- [ ] T065 [US6] Add type alias `pub(crate) type StyleClassWrapper = Rc<StyleClass>;` to `crates/dampen-iced/src/builder/mod.rs` or `helpers.rs`

- [ ] T066 [US6] Update `create_state_aware_style_fn` in `crates/dampen-iced/src/builder/helpers.rs` to accept `Option<Rc<StyleClass>>`
  - Change parameter: `style_class: Option<Rc<StyleClass>>`
  - Update closure to use Rc clone (automatic Deref)

- [ ] T067 [P] [US6] Update `crates/dampen-iced/src/builder/widgets/checkbox.rs` to wrap StyleClass in Rc before passing to helper
  - Change: `let style_class = style_class.map(Rc::new);`

- [ ] T068 [P] [US6] Update `crates/dampen-iced/src/builder/widgets/radio.rs` to wrap StyleClass in Rc

- [ ] T069 [P] [US6] Update `crates/dampen-iced/src/builder/widgets/text_input.rs` to wrap StyleClass in Rc

- [ ] T070 [P] [US6] Update `crates/dampen-iced/src/builder/widgets/toggler.rs` to wrap StyleClass in Rc

- [ ] T071 [P] [US6] Update `crates/dampen-iced/src/builder/widgets/slider.rs` to wrap StyleClass in Rc

- [ ] T072 [P] [US6] Update `crates/dampen-iced/src/builder/widgets/pick_list.rs` to wrap StyleClass in Rc

- [ ] T073 [P] [US6] Update `crates/dampen-iced/src/builder/widgets/combo_box.rs` to wrap StyleClass in Rc

- [ ] T074 [US6] Run test suite: `cargo test --workspace` (expect 148/148 pass)

- [ ] T075 [US6] Run clippy: `cargo clippy --workspace -- -D warnings` (expect 0 warnings)

- [ ] T076 [US6] Run performance benchmark: Measure clone time (expect ~47x speedup: 4ns vs 213ns)

- [ ] T077 [US6] Run memory benchmark: Measure memory usage for 1000 widgets (expect ~200KB reduction)

- [ ] T078 [US6] Run rendering benchmark: Measure render time (expect ~5% improvement)

- [ ] T079 [US6] Visual regression test: Run all examples and verify no visual regressions

**Checkpoint**: Rc optimization complete, performance targets met

**PR #4 Complete**: US6 complete OR defer to future milestone if too risky

---

## Phase 9: Polish & Cross-Cutting Concerns

**Purpose**: Documentation, cleanup, and final validation

- [ ] T080 [P] Remove verbose logging: Replace all `if self.verbose { eprintln!(...) }` with `#[cfg(debug_assertions)] eprintln!(...)` in all widget files
  - Affects ~52 instances across 24 widget files in `crates/dampen-iced/src/builder/widgets/`

- [ ] T081 [P] Remove `verbose: bool` field from `DampenWidgetBuilder` in `crates/dampen-iced/src/builder/mod.rs`

- [ ] T082 [P] Update `DampenWidgetBuilder::new()` signature to remove `verbose` parameter

- [ ] T083 [P] Update all examples in `examples/` to remove verbose parameter from DampenWidgetBuilder::new() calls

- [ ] T084 Update `crates/dampen-iced/README.md` with refactoring benefits (lines saved, performance improvements)

- [ ] T085 Update `crates/dampen-iced/CHANGELOG.md` for v0.2.7 with all changes:
  - Deprecated: IcedBackend (removal in v0.3.0)
  - Added: Helper functions for boolean attributes, handler resolution, state-aware styling
  - Improved: Performance (Rc optimization), binary size (compile-time logging)
  - Removed: Verbose logging flag (replaced with debug_assertions)

- [ ] T086 Add migration guide to `crates/dampen-iced/docs/MIGRATION.md` for IcedBackend users

- [ ] T087 Run final test suite: `cargo test --workspace` (expect 148/148 pass)

- [ ] T088 Run final clippy: `cargo clippy --workspace -- -D warnings` (expect 0 warnings)

- [ ] T089 Run final formatting check: `cargo fmt --all -- --check`

- [ ] T090 Build all examples: `cargo build --examples`

- [ ] T091 Measure final LOC: Count lines in `crates/dampen-iced/src/` and compare to baseline (expect ~370 lines removed)

- [ ] T092 Measure final binary size: Compare release build size to baseline (expect ~5KB reduction)

- [ ] T093 Run quickstart.md validation: Follow quickstart guide and verify all examples work

- [ ] T094 Update AGENTS.md with new helper function APIs per plan.md section 1.4

**Checkpoint**: All refactoring complete, documented, and validated

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories
- **User Stories (Phase 3-8)**: All depend on Foundational phase completion
  - **US1 (P1)**: Can start after Foundational - No dependencies on other stories
  - **US4 (P4)**: Can start after Foundational - No dependencies on other stories
  - **US5 (P5)**: Can start after Foundational - No dependencies on other stories
  - **US2 (P2)**: Can start after Foundational - No dependencies on other stories
  - **US3 (P3)**: **DEPENDS ON US2** - Needs `create_state_aware_style_fn` helper
  - **US6 (P6)**: Can start after Foundational - High risk, should be LAST
- **Polish (Phase 9)**: Depends on all desired user stories being complete

### Recommended Execution Order (from Research R7)

**PR #1**: US1 (P1) - Remove legacy code  
**PR #2**: US4 (P4) + US5 (P5) - Extract helpers (no dependencies)  
**PR #3**: US2 (P2) + US3 (P3) - State-aware styling (P3 depends on P2)  
**PR #4**: US6 (P6) - Optimization (high risk, defer if needed)  

**Rationale**: This order minimizes risk, respects dependencies, and enables incremental value delivery.

### Within Each User Story

- Foundation tasks (helpers.rs, error types) before widget migrations
- Widget style applier functions before using `create_state_aware_style_fn`
- Test suite run after each widget migration group
- Visual regression tests after completing each user story

### Parallel Opportunities

- All Setup tasks (T001-T004) can run in parallel
- All Foundational tasks (T005-T008) can run in parallel
- Within US5: All 5 widget migrations (T030-T034) can run in parallel [P]
- Within US6: All 7 widget Rc migrations (T067-T073) can run in parallel [P]
- Within Polish: Documentation tasks (T080-T086) can run in parallel [P]

---

## Validation Checkpoints

After each user story phase, run:

```bash
# Test suite (MUST pass 148/148 tests)
cargo test --workspace

# Clippy (MUST produce 0 warnings)
cargo clippy --workspace -- -D warnings

# Format check (MUST be formatted)
cargo fmt --all -- --check

# Examples (MUST build successfully)
cargo build --examples
```

---

## Implementation Strategy

### Incremental Delivery (Recommended)

1. Complete **Phase 1 + Phase 2** → Foundation ready
2. Complete **Phase 3 (US1)** → Legacy code removed → Test → Commit
3. Complete **Phase 4 + Phase 5 (US4+US5)** → Helpers extracted → Test → **PR #2**
4. Complete **Phase 6 + Phase 7 (US2+US3)** → State styling complete → Test → **PR #3**
5. Complete **Phase 8 (US6)** → Optimization done → Test → **PR #4** (or defer)
6. Complete **Phase 9** → Polish & document → Final validation → Done

Each phase adds value without breaking previous work.

### Timeline Estimate

Based on research (from plan.md):

- **Phase 1-2** (Setup + Foundation): 0.5 days
- **US1 (P1)**: 0.5 days
- **US4 (P4)**: 0.25 days
- **US5 (P5)**: 0.5 days
- **US2 (P2)**: 1 day
- **US3 (P3)**: 0.5 days
- **US6 (P6)**: 0.5 days
- **Phase 9** (Polish): 0.75 days

**Total**: ~5 days (1 developer week)

---

## Success Metrics

Track throughout implementation:

| Metric | Baseline | Target | Actual |
|--------|----------|--------|--------|
| Total LOC | 10,265 | < 9,900 | _TBD_ |
| Duplicated LOC | ~400 | < 50 | _TBD_ |
| Test Pass Rate | 148/148 (100%) | 148/148 (100%) | _TBD_ |
| Clippy Warnings | 0 | 0 | _TBD_ |
| Render Time (1000w) | 0.284ms | < 0.270ms | _TBD_ |
| Memory (1000w) | Baseline | -100KB | _TBD_ |
| Binary Size (release) | Baseline | -5KB | _TBD_ |

**Acceptance Criteria** (from spec.md):
- ✅ All 148 existing tests pass
- ✅ Zero clippy warnings
- ✅ Lines of code reduced by 370+
- ✅ State-aware styling implemented for slider, pick_list, combo_box
- ✅ Performance improved by ~5% for 1000-widget UIs
- ✅ Memory usage reduced by ~100KB for 1000-widget UIs

---

## Notes

- **[P]** tasks = different files, no dependencies - can run in parallel
- **[Story]** label maps task to specific user story for traceability
- Each user story is independently completable and testable
- Commit after completing each user story phase
- Stop at any checkpoint to validate story independently
- **Critical**: US3 depends on US2 - do not start US3 until US2 helper is complete
- **Risk Management**: US6 is highest risk - consider deferring to separate milestone if unstable
- **Tests**: This refactoring relies on existing 148 tests - NO new tests requested
