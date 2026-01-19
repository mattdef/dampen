# Research: Harmonize Modes

**Feature**: Harmonize Modes (Interpreted vs Codegen)
**Date**: 2026-01-19

## 1. State-Aware Styling in Codegen

### Findings
- **Iced 0.14 Styling API**: Styles are defined using closures that return a Style struct, e.g., `Fn(&Theme, Status) -> Style`.
- **Status Support**:
  - Supported: `Button`, `TextInput`, `Checkbox`, `Radio`, `Toggler`, `PickList`, `Slider`.
  - NOT Supported: `Container`, `Column`, `Row`, `Scrollable` (no `Status` in style closure).
- **Current Codegen**: `dampen-macros` already has `generate_inline_style_closure`, but it only supports `button`, `container` (base only), and `text_input`.
- **Dampen-Iced Runtime**: Already contains robust mapping logic (`style_mapping.rs`) for `WidgetState` -> `Status`.

### Decision
- **Implementation Strategy**: Port the logic from `dampen-iced::style_mapping` into `dampen-macros`.
- **Pattern**: Expand `generate_inline_style_closure` to generate `match status { ... }` expressions that correspond to the static `impl` logic.
- **Container Hover**: Accept limitation for now (no native hover support in Iced 0.14 Container). Do NOT implement complex wrapper workaround in this phase to strictly follow Iced upstream capabilities, unless it breaks critical parity (spec says "Visual Parity" is key). *Correction*: Spec allows breaking changes. If Interpreted mode *already* limits this, parity is preserved.

## 2. Layout Unification (Wrapping)

### Findings
- **Current Implementation**: `generate_text` in `view.rs` already manually wraps `Text` widgets in a `Container` if layout attributes (`width`, `padding`, `align_x`) are present.
- **Missing Support**:
  - `Column`/`Row` do not support `align_x`/`align_y` (content alignment) directly in the same way `Container` does (they have `align_items` for cross-axis).
  - `Scrollable` supports `width`/`height` but `generate_container` logic for it might be incomplete regarding alignment.

### Decision
- **Generalize Wrapper**: Extract the "wrap in container if needed" logic from `generate_text` into a reusable `maybe_wrap_in_container` helper in `view.rs`.
- **Apply to All**: Apply this helper to `Image`, `Svg`, `Button` (if it lacks native layout support for specific attrs), `ProgressBar`, etc.
- **Column/Row Alignment**: If `align_x`/`align_y` are used on a Column/Row, wrap the *entire* Column/Row in a Container to provide that positioning within the parent.

## 3. Visual Regression Testing

### Findings
- **Existing Tests**: Unit tests exist for state mapping, but no visual rendering tests.
- **Headless Rendering**: Iced doesn't have a built-in "render to png" function exposed easily without running the application loop.
- **Alternative**: `iced_winit` + `wgpu` can be used to manually drive a frame and capture the buffer.

### Decision
- **Tooling**: Create a separate crate `dampen-visual-tests` that depends on `iced`, `wgpu`, `image`.
- **Mechanism**:
  1. Instantiate the specific widget/view.
  2. Setup a `wgpu` offscreen surface.
  3. Manually trigger a render pass using Iced's renderer.
  4. Read back the texture buffer.
  5. Save/Compare PNGs.
- **Fallback**: If manual wgpu setup is too complex, spawn the app in a hidden window (xvfb on CI) and take a screenshot, but prefer offscreen rendering for stability.

## 4. Widget Specifics

- **TextInput**: `password` is supported in Iced (`secure: bool`), just missing in codegen mapping. `color` implies text color; `TextInput` style has `value` color.
- **Slider**: `step` is supported in Iced slider constructor.
- **Loops**: `for item in items` is a syntax change in `dampen-core` parser. Need to ensure `dampen-macros` handles the variable binding correctly.

## 5. Resolved Clarifications

- **Unknowns**: None remaining that block design.
- **Dependencies**: No new major dependencies (except dev-dependencies for visual tests).
