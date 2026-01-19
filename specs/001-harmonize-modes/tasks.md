# Tasks: Harmonize Modes

**Feature**: Harmonize Modes (Interpreted vs Codegen)
**Branch**: `001-harmonize-modes`
**Status**: Planned

## Implementation Strategy

This plan prioritizes "Test-First" by building the visual regression harness (Phase 2) before attempting complex changes. The work is split into logical increments: first, unified attribute contract (Phase 3), then the critical layout unification (Phase 4), and finally the complex state-aware styling codegen (Phase 5).

Each phase is independently verifiable. Phase 3 verifies parsing/IR. Phase 4 verifies geometry. Phase 5 verifies interactivity.

## Phase 1: Setup

- [X] T001 Create `specs/001-harmonize-modes` directory structure and artifacts
- [X] T002 [P] Create `crates/dampen-visual-tests` new crate
- [X] T003 [P] Add `dampen-visual-tests` to workspace members in `Cargo.toml`
- [X] T004 Add dependencies to `crates/dampen-visual-tests/Cargo.toml` (`iced`, `wgpu`, `image`, `tokio`)

## Phase 2: Foundational (Visual Test Harness)

*Goal: Establish the mechanism to verify visual parity before making changes.*

- [X] T005 Implement offscreen rendering helper in `crates/dampen-visual-tests/src/renderer.rs`
- [X] T006 Implement image comparison logic with tolerance in `crates/dampen-visual-tests/src/compare.rs`
- [X] T007 Create basic test runner in `crates/dampen-visual-tests/src/lib.rs` that accepts Dampen XML and outputs PNGs
- [X] T008 Create "baseline" snapshot generator script `scripts/generate_baselines.sh` using Interpreted mode
- [X] T009 Verify harness works by running a simple "Hello World" test case

## Phase 3: Standardized Attribute Contract (US1)

*Goal: Enforce strict attribute naming and types in IR.*

- [X] T010 [P] [US1] Update `WidgetNode` struct in `crates/dampen-core/src/ir/node.rs` to add standardized layout fields
- [X] T011 [US1] Refactor `crates/dampen-core/src/parser/mod.rs` to enforce unified attribute names (e.g., `src` vs `path`)
- [X] T012 [P] [US1] Implement `for item in items` loop syntax parsing in `crates/dampen-core/src/parser/mod.rs`
- [X] T013 [P] [US1] Add parser validation/warnings for deprecated attributes (e.g., `active` -> `toggled`)
- [X] T014 [US1] Update `WidgetKind` in `crates/dampen-core/src/ir/node.rs` to support new standard (if needed)

## Phase 4: Layout Unification (US1)

*Goal: Ensure Codegen wraps widgets in Containers when layout attributes are used.*

- [X] T015 [US1] Create `maybe_wrap_in_container` helper in `crates/dampen-macros/src/codegen/view.rs`
- [X] T016 [US1] Update `generate_text` to use `maybe_wrap_in_container`
- [X] T017 [US1] Update `generate_image` and `generate_svg` to use `maybe_wrap_in_container`
- [X] T018 [US1] Implement wrapper logic for `Column`/`Row` when `align_x`/`align_y` are present
- [X] T019 [US1] Update `generate_scrollable` to properly handle `width`/`height` via wrapper if natively unsupported
- [X] T020 [US1] Add visual regression test case for "Complex Nested Layout" (Cols inside Rows with alignment)

## Phase 5: State-Aware Styling in Codegen (US3)

*Goal: Generate static Rust code for hover/focus/active states.*

- [X] T021 [US3] Port status mapping logic from `dampen-iced` to `crates/dampen-core/src/codegen/status_mapping.rs`
- [X] T022 [US3] Update `generate_inline_style_closure` in `crates/dampen-core/src/codegen/view.rs` to accept Status param
- [X] T023 [US3] Implement `generate_button_style_match` to output `match status { ... }` logic
- [X] T024 [US3] Implement `generate_text_input_style_match` for `placeholder` color and state styling
- [X] T025 [US3] Implement `generate_checkbox_style` and `generate_toggler_style` support
- [X] T026 [US3] Implement `generate_slider_style` support
- [X] T027 [US3] Verify styling parity with visual test case "Interactive States" (hovering buttons/inputs)

## Phase 6: Widget Specifics & Polish (US1)

*Goal: Close gaps for specific widgets.*

- [X] T028 [P] [US1] Implement `password` support for `TextInput` in `crates/dampen-macros/src/codegen/view.rs`
- [X] T029 [P] [US1] Implement `step` support for `Slider` in `crates/dampen-macros/src/codegen/view.rs`
- [X] T030 [P] [US1] Update `Image`/`Svg` generators to handle unified `src` attribute
- [X] T031 [US1] Verify `for` loop code generation matches new parser syntax

## Phase 7: Final Verification (US2)

*Goal: Ensure the visual testing infrastructure prevents future regressions.*

- [X] T032 [US2] Run full visual regression suite against standard examples
- [X] T033 [US2] Document visual testing workflow in `CONTRIBUTING.md`
- [X] T034 [US2] Add visual tests to CI pipeline (e.g., GitHub Actions workflow file)

## Dependencies

- Phase 4 (Layout) depends on Phase 3 (IR changes)
- Phase 5 (Styling) depends on Phase 3 (IR changes)
- Phase 7 (Verification) depends on Phase 2 (Harness) and Phases 3-6 implementation

## Parallel Execution Examples

- **Phase 3**: T010, T012, T013 can be done in parallel by different agents working on separate files.
- **Phase 6**: T028, T029, T030 are independent widget updates.
