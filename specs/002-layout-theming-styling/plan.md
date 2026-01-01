# Implementation Plan: Layout, Sizing, Theming, and Styling System

**Branch**: `002-layout-theming-styling` | **Date**: 2026-01-01 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `/specs/002-layout-theming-styling/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/commands/plan.md` for the execution workflow.

## Summary

Add comprehensive layout, sizing, theming, and styling capabilities to Gravity's declarative XML UI framework. This feature extends the existing widget IR with layout attributes (padding, spacing, alignment), sizing constraints (fill, shrink, min/max), theming system (color palettes, typography, light/dark modes), inline styles (background, border, shadow), reusable style classes, responsive breakpoints, and state-based styling (hover, focus, active, disabled). The implementation must maintain the dual-mode architecture (runtime interpretation for dev hot-reload + static code generation for production) while preserving type safety and backend abstraction principles.

## Technical Context

**Language/Version**: Rust Edition 2024, MSRV stable (no nightly features in public API)  
**Primary Dependencies**: 
  - `iced` 0.14+ (backend implementation)
  - `roxmltree` 0.19+ (XML parsing)
  - `serde`, `serde_json` 1.0+ (serialization for hot-reload state)
  - `notify` 6.0+ (file watching)
  - `syn` 2.0+, `quote` 2.0+, `proc-macro2` 2.0+ (proc macros for codegen)
  - NEEDS CLARIFICATION: Color parsing library (e.g., `palette`, `csscolorparser`)
  - NEEDS CLARIFICATION: CSS gradient parsing approach (custom parser vs library)
  - NEEDS CLARIFICATION: Responsive breakpoint implementation (viewport queries in Iced)

**Storage**: File-based (XML UI definitions, optional separate style files), serialized state in `.gravity-state.json`  
**Testing**: `cargo test`, `proptest` 1.0+ (property-based), `insta` 1.0+ (snapshot testing)  
**Target Platform**: Windows, Linux, macOS (tier 1 desktop support via Iced)  
**Project Type**: Single Rust workspace with 5 crates (gravity-core, gravity-macros, gravity-runtime, gravity-iced, gravity-cli)  
**Performance Goals**: 
  - XML parse time < 10ms for 1000-widget files (including new style attributes)
  - Hot-reload latency < 500ms from file save to UI update
  - Theme switching < 100ms for UI re-render
  - State-based style transitions < 50ms (hover/focus response)
  - 60 fps rendering for animated transitions

**Constraints**: 
  - Must maintain backend abstraction (no Iced types in gravity-core)
  - Zero runtime overhead in production mode (styles compiled to static Rust code)
  - Style precedence: inline > classes > theme (deterministic cascade)
  - All style attributes must be serializable for hot-reload
  - Generated code must remain human-readable

**Scale/Scope**: 
  - Support 50+ style attributes across layout/sizing/theming/styling categories
  - Handle files with 1000+ widgets with complex nested styling
  - Support 10+ concurrent theme definitions
  - Enable 100+ reusable style classes per application
  - NEEDS CLARIFICATION: Maximum nesting depth for style class inheritance
  - NEEDS CLARIFICATION: Performance impact of responsive attribute re-evaluation on window resize

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

### I. Declarative-First ✓

**Status**: PASS

- Layout, sizing, theming, and styling attributes are defined in XML markup
- IR nodes extended to capture style properties (no runtime Rust context needed for parsing)
- Separation maintained: visual appearance (XML) vs behavior (Rust handlers)
- Parser produces complete style AST before any backend rendering

### II. Type Safety Preservation ✓

**Status**: PASS

- Theme definitions use strongly-typed Color, Length, Font enums in IR
- Style attributes parse to typed IR nodes (not string bags)
- State-based styles (hover/focus/active) compile to typed widget state handlers
- No runtime type erasure; production codegen produces concrete Iced types

### III. Dual-Mode Architecture ✓

**Status**: PASS

**Development Mode**:
- Runtime interpretation: Parse style attributes, evaluate theme references, apply cascading
- Hot-reload: Style changes reflected within 500ms without recompilation
- Diagnostic overlay: Display style parsing errors and invalid theme references

**Production Mode**:
- Static code generation: Inline all resolved styles as Iced widget styling code
- Zero runtime parsing: Theme colors, computed styles baked into generated code
- Compile-time verification: Invalid style values caught during codegen phase

### IV. Backend Abstraction ✓

**Status**: PASS

- Style IR defined in `gravity-core` (backend-agnostic: Color, Length, Alignment, etc.)
- Backend trait extended with style mapping: `fn apply_style(&self, widget, style: &StyleProperties)`
- Iced backend implements trait to map IR styles to `iced::widget::container::Style`, `iced::Color`, etc.
- No Iced types in core style parsing or IR definitions

### V. Test-First Development ✓

**Status**: PASS (to be verified during implementation)

- Contract tests: XML with style attributes → IR with correct StyleProperties
- Integration tests: Full pipeline with theming, cascading, and rendering
- Property-based tests: Style attribute parsing edge cases (malformed colors, invalid values)
- Snapshot tests: Generated code with inline styles matches expected Rust output
- Hot-reload tests: Style changes preserve application state

### Summary

**All constitutional gates PASS.** No violations. Feature design aligns with all five core principles.

---

## Post-Design Constitution Re-Check ✓

**Date**: 2026-01-01 (Phase 1 Complete)  
**Status**: All gates still PASS after detailed design

### Design Artifacts Reviewed
- ✅ `data-model.md`: All IR types are backend-agnostic, serializable, strongly-typed
- ✅ `contracts/xml-schema.md`: Declarative-first XML schema, no implementation leakage
- ✅ `quickstart.md`: Demonstrates dual-mode workflow (dev hot-reload + prod codegen)

### Specific Validations

**I. Declarative-First** ✓
- All 50+ style attributes defined in XML schema
- IR types capture complete styling semantics without runtime Rust context
- Hot-reload demonstrations in quickstart confirm XML as source of truth

**II. Type Safety Preservation** ✓
- `Color`, `Length`, `Gradient` are strongly-typed enums (not stringly-typed)
- Parse-time validation prevents invalid values from reaching IR
- Production codegen compiles to concrete Iced types (no runtime `Any` casting)

**III. Dual-Mode Architecture** ✓
- `gravity-runtime` provides runtime interpretation with style cascading
- `gravity-cli build` generates static code with inlined styles
- Hot-reload preserves state through serde serialization (confirmed in data model)

**IV. Backend Abstraction** ✓
- All style IR types in `gravity-core` (no Iced dependencies)
- `gravity-iced` implements backend trait for style mapping to Iced widgets
- Custom parser for gradients maps to generic IR, then Iced-specific types in backend

**V. Test-First Development** ✓
- Contract tests defined for style attribute parsing → IR
- Snapshot tests for generated code with inline styles
- Property-based tests for gradient/color parsing edge cases
- Integration tests for theme switching and hot-reload (specified in data model)

**Final Verdict**: Design maintains full constitutional compliance. Ready for implementation.

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

### Source Code (repository root)

```text
crates/
├── gravity-core/
│   ├── src/
│   │   ├── ir/
│   │   │   ├── node.rs          # EXTEND: Add StyleProperties, LayoutConstraints
│   │   │   ├── style.rs         # NEW: Style IR types (Color, Length, Border, Shadow)
│   │   │   ├── theme.rs         # NEW: Theme definitions (ThemePalette, ThemeProperties)
│   │   │   └── layout.rs        # NEW: Layout types (Alignment, Spacing, Sizing)
│   │   ├── parser/
│   │   │   ├── mod.rs           # EXTEND: Parse style/layout attributes
│   │   │   ├── style_parser.rs  # NEW: Parse style attributes (background, border, etc.)
│   │   │   └── theme_parser.rs  # NEW: Parse theme definitions
│   │   └── expr/
│   │       └── eval.rs          # EXTEND: Evaluate theme color references
│   └── tests/
│       ├── style_parser_tests.rs     # NEW: Test style attribute parsing
│       └── snapshots/                # NEW: Style IR snapshots
│
├── gravity-macros/
│   └── src/
│       └── lib.rs               # EXTEND: Support style attributes in codegen
│
├── gravity-runtime/
│   ├── src/
│   │   ├── interpreter.rs       # EXTEND: Apply runtime styles, handle theme switching
│   │   ├── theme_manager.rs     # NEW: Runtime theme resolution and switching
│   │   └── style_cascade.rs     # NEW: Compute cascaded styles (inline > class > theme)
│   └── tests/
│       └── theme_tests.rs       # NEW: Test theme application and hot-reload
│
├── gravity-iced/
│   ├── src/
│   │   ├── lib.rs               # EXTEND: Backend trait with style mapping
│   │   ├── style_mapping.rs     # NEW: Map IR styles to Iced widget styles
│   │   ├── theme_adapter.rs     # NEW: Convert Gravity themes to iced::Theme
│   │   └── widgets/
│   │       └── styled.rs        # NEW: Wrapper widgets with style application
│   └── tests/
│       └── style_integration_tests.rs  # NEW: End-to-end style rendering
│
└── gravity-cli/
    ├── src/
    │   └── commands/
    │       ├── dev.rs           # EXTEND: Hot-reload for style changes
    │       └── build.rs         # EXTEND: Generate style code
    └── templates/
        └── new/
            └── theme.gravity.template  # NEW: Default theme template

examples/
├── styling/                     # NEW: Comprehensive styling showcase
│   ├── src/
│   │   └── main.rs
│   └── ui/
│       ├── main.gravity
│       └── themes.gravity
│
└── responsive/                  # NEW: Responsive layout example
    ├── src/
    │   └── main.rs
    └── ui/
        └── main.gravity

docs/
└── STYLING.md                   # NEW: Styling system documentation
```

**Structure Decision**: Single Rust workspace structure with 5 existing crates. This feature primarily extends `gravity-core` (IR types, parser), `gravity-runtime` (theme management, cascading), and `gravity-iced` (style-to-Iced mapping). No new crates required; styling is a core framework capability, not a separate module.

## Complexity Tracking

**No violations** - This section is not applicable as all Constitution Check gates pass.
