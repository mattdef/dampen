# Implementation Plan: Widget State Styling

**Branch**: `003-widget-state-styling` | **Date**: 2026-01-15 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `/specs/003-widget-state-styling/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/commands/plan.md` for the execution workflow.

## Summary

Enable visual state styling (hover, focus, active, disabled) for widgets in Dampen framework. The XML parser and IR layer already support state variants (e.g., `<hover>`, `<active>`), but the Iced backend ignores these definitions. This feature connects the existing IR to Iced's native status system by mapping widget-specific status enums (e.g., `button::Status`) to Dampen's generic `WidgetState`, then resolving styles from `StyleClass.state_variants`. Implementation is backend-only (no changes to `dampen-core`), maintains zero breaking changes, and uses Iced's built-in state tracking rather than external state management.

## Technical Context

**Language/Version**: Rust Edition 2024, MSRV 1.85 (minimum for Edition 2024)  
**Primary Dependencies**: `iced` 0.14+ (UI framework), `roxmltree` 0.19+ (XML parsing, already in use)  
**Storage**: N/A (feature affects in-memory styling only)  
**Testing**: `cargo test --workspace` (TDD approach per Constitution Principle V)  
**Target Platform**: Linux, macOS, Windows (desktop applications via Iced)
**Project Type**: Library/Framework (Cargo workspace with multiple crates)  
**Performance Goals**: 
  - State style resolution: < 1ms per widget (HashMap lookup)
  - Render frame budget: < 16ms at 60fps (target: state changes apply within one frame)
  - Zero measurable performance regression in hot path
**Constraints**: 
  - Zero breaking changes (FR-010, SC-004) - critical requirement
  - Backend abstraction (Constitution IV) - all changes in `dampen-iced`, not `dampen-core`
  - Type safety preservation (Constitution II) - no runtime type erasure for status enums
  - Hot-reload must preserve interaction states (FR-011, SC-005)
**Scale/Scope**: 
  - 9 widget types to implement (Button, TextInput, Checkbox, Radio, Toggler, Slider, Container, PickList, ComboBox)
  - 15+ integration tests required (SC-006)
  - 1 comprehensive example application (`examples/styling/`)
  - Estimated 16 hours implementation time across 5 phases

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

### ✅ I. Declarative-First
**Status**: COMPLIANT  
**Rationale**: Feature uses existing XML state variant syntax (e.g., `<hover>`, `<active>`) already parsed by `dampen-core`. No changes to XML schema or declarative UI model. Implementation connects declarative state definitions to visual rendering.

### ✅ II. Type Safety Preservation
**Status**: COMPLIANT  
**Rationale**: Uses Iced's type-safe status enums (e.g., `button::Status::Hovered`, `text_input::Status::Focused`) and maps them to Dampen's `WidgetState` enum. No type erasure - all state transitions are statically typed. Style resolution uses strongly-typed `StyleProperties`.

### ✅ III. Production Mode
**Status**: COMPLIANT  
**Rationale**: Feature works in both interpreted (hot-reload) and codegen modes. State styling logic runs identically in production builds since it operates on pre-parsed IR (`StyleClass.state_variants`). No runtime XML parsing required.

### ✅ IV. Backend Abstraction
**Status**: COMPLIANT  
**Rationale**: All implementation in `dampen-iced` crate. Core types (`WidgetState`, `StateSelector`, `StyleClass`) already exist in `dampen-core` without Iced dependency. Feature demonstrates proper backend abstraction - future backends (egui, druid) can implement their own status mapping without touching core.

### ✅ V. Test-First Development
**Status**: COMMITTED  
**Rationale**: Implementation plan includes Phase 0 (research) and Phase 1 (design/contracts) before Phase 2 (implementation). Contract tests will be written first for status mapping functions and style resolution logic. Target: 15+ integration tests covering all P1/P2 widgets (SC-006).

**Constitution Compliance**: ✅ ALL PRINCIPLES SATISFIED - No violations detected.

## Project Structure

### Documentation (this feature)

```text
specs/[###-feature]/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output (/speckit.plan command)
├── data-model.md        # Phase 1 output (/speckit.plan command)
├── quickstart.md        # Phase 1 output (/speckit.plan command)
├── contracts/           # Phase 1 output (/speckit.plan command)
└── tasks.md             # Phase 2 output (/speckit.tasks command - NOT created by /speckit.plan)
```

## Project Structure

### Documentation (this feature)

```text
specs/003-widget-state-styling/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output: Iced 0.14 status API research
├── data-model.md        # Phase 1 output: WidgetState, StateSelector, StyleClass docs
├── quickstart.md        # Phase 1 output: Developer guide for using state styling
├── contracts/           # Phase 1 output: API contracts for status mapping functions
│   ├── status-mapping.md    # Iced status → WidgetState mapping contracts
│   ├── style-resolution.md  # Style resolution from state_variants contracts
│   └── widget-builder.md    # Widget builder integration contracts
└── tasks.md             # Phase 2 output (/speckit.tasks command - NOT created by /speckit.plan)
```

### Source Code (repository root)

```text
crates/
├── dampen-core/              # ❌ NO CHANGES (backend abstraction)
│   ├── src/
│   │   ├── ir/
│   │   │   └── theme.rs      # ✅ Already has WidgetState, StateSelector, StyleClass
│   │   └── expr/             # ✅ Already has expression evaluation for bindings
│   └── tests/                # ✅ Existing tests for IR types (no changes needed)
│
├── dampen-iced/              # ✅ ALL CHANGES HERE (Iced backend implementation)
│   ├── src/
│   │   ├── lib.rs            # Export new status mapping functions
│   │   ├── state.rs          # ❌ REMOVE WidgetStateManager (dead code)
│   │   ├── style_mapping.rs  # ✅ NEW: Status mapping and style resolution functions
│   │   │   # Functions to add:
│   │   │   # - map_button_status(button::Status) -> WidgetState
│   │   │   # - map_text_input_status(text_input::Status, &TextInputContext) -> WidgetState
│   │   │   # - map_checkbox_status(checkbox::Status) -> WidgetState
│   │   │   # - map_radio_status(radio::Status) -> WidgetState
│   │   │   # - map_toggler_status(toggler::Status) -> WidgetState
│   │   │   # - map_slider_status(slider::Status) -> WidgetState
│   │   │   # - map_container_status(container::Status) -> WidgetState
│   │   │   # - map_picklist_status(pick_list::Status) -> WidgetState
│   │   │   # - map_combobox_status(combo_box::Status) -> WidgetState
│   │   │   # - resolve_state_style(StyleClass, WidgetState) -> StyleProperties
│   │   │   # - merge_style_properties(base, state_override) -> StyleProperties
│   │   ├── builder.rs        # ✅ MODIFY: Update widget building to use status-aware styles
│   │   │   # Changes per widget:
│   │   │   # - Pass status parameter to map_*_status() function
│   │   │   # - Use resolve_state_style() to get final StyleProperties
│   │   │   # - Apply merged styles to Iced widget
│   │   └── theme.rs          # ✅ MODIFY: Update theme integration if needed
│   └── tests/
│       ├── widget_state_tests.rs  # ✅ NEW: Integration tests for state styling
│       │   # Test structure:
│       │   # - test_button_hover_styling()
│       │   # - test_button_active_styling()
│       │   # - test_button_disabled_styling()
│       │   # - test_text_input_focus_styling()
│       │   # - test_checkbox_hover_styling()
│       │   # - test_radio_hover_styling()
│       │   # - test_toggler_active_styling()
│       │   # - test_slider_drag_styling()
│       │   # - test_container_hover_styling()
│       │   # - test_picklist_focus_styling()
│       │   # - test_combobox_disabled_styling()
│       │   # - test_fallback_to_base_style()
│       │   # - test_inline_style_precedence()
│       │   # - test_combined_state_priority()
│       │   # - test_hot_reload_preserves_state()
│       └── status_mapping_tests.rs  # ✅ NEW: Unit tests for status mapping functions
│
└── dampen-cli/               # ❌ NO CHANGES (CLI tools unaffected)

examples/
├── styling/                  # ✅ VERIFY: Existing example should "just work" with new feature
│   ├── src/ui/
│   │   └── window.dampen    # ✅ Already has state variants (lines 55-66), currently non-functional
│   └── README.md             # ✅ UPDATE: Document state styling feature in example
└── state-styling-demo/       # ✅ NEW (OPTIONAL): Dedicated demo for all widget states
    ├── src/
    │   ├── main.rs
    │   └── ui/
    │       └── demo.dampen  # Comprehensive state styling showcase
    └── README.md

tests/
└── integration/
    └── state_styling_integration.rs  # ✅ NEW: End-to-end tests with full application

docs/
└── WIDGETS_STATE_IMPLEMENTATION.md  # ✅ Already created (implementation guide)
```

**Structure Decision**: This is a Cargo workspace (not a single project). The feature modifies only the `dampen-iced` backend crate, respecting the backend abstraction principle (Constitution IV). Core IR types in `dampen-core` already exist and require no changes. Primary development occurs in:
1. `crates/dampen-iced/src/style_mapping.rs` (new file) - core logic
2. `crates/dampen-iced/src/builder.rs` (modifications) - integration with widget building
3. `crates/dampen-iced/tests/` (new test files) - TDD contract tests
4. `examples/styling/` (verification) - validate existing example works without changes

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

**Status**: N/A - No constitution violations detected. All principles satisfied.

This feature maintains the framework's design principles:
- Uses existing IR types (no new abstractions)
- Respects backend abstraction (changes only in `dampen-iced`)
- Preserves type safety (strongly-typed status enums)
- Works in both interpreted and codegen modes
- Follows TDD approach with contract tests first
