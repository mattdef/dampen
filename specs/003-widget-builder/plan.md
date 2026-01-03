# Implementation Plan: Gravity Widget Builder

**Branch**: `003-widget-builder` | **Date**: 2026-01-03 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `/specs/003-widget-builder/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/commands/plan.md` for the execution workflow.

## Summary

Implement a `GravityWidgetBuilder` in `gravity-iced` that automatically interprets parsed Gravity markup into Iced widgets, eliminating manual conversion boilerplate. The builder will handle bindings, events, styles, layouts, and recursive child processing through automatic type conversions, enabling single-line UI rendering in examples.

## Technical Context

**Language/Version**: Rust Edition 2024, Stable Rust (no nightly features)  
**Primary Dependencies**: 
- `gravity-core` (existing, provides IR types: WidgetNode, StyleProperties, LayoutConstraints)
- `iced` 0.14+ (existing, rendering backend)
- `HandlerRegistry` (existing, from gravity-runtime)
- `evaluate_binding_expr` (existing, from gravity-runtime)
- `From` trait implementations (to be created)

**Storage**: N/A (runtime interpretation, no persistence required)  
**Testing**: cargo test, insta (snapshot testing), proptest (property-based)  
**Target Platform**: Desktop (Windows, Linux, macOS) via Iced  
**Project Type**: Library crate (gravity-iced) + example applications  
**Performance Goals**: 50ms render time for 1000 widgets (from SC-006)  
**Constraints**: 
- Backend-agnostic core (gravity-core must not depend on Iced)
- <10% build time increase
- Equivalent runtime performance to manual implementation
- Hot-reload compatible (<500ms latency)

**Scale/Scope**: 
- Supports all common widget types (Text, Button, Column, Row, Container)
- Handles nested widgets recursively
- Manages bindings, events, styles, layouts automatically
- Examples reduced to <50 lines

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

### Compliance Verification

| Constitution Principle | Compliance Status | Evidence |
|------------------------|-------------------|----------|
| **I. Declarative-First** | ✅ PASS | Feature uses parsed XML/IR as input, no structure in Rust code |
| **II. Type Safety Preservation** | ✅ PASS | Uses From trait conversions, no type erasure, compile-time verified |
| **III. Dual-Mode Architecture** | ⚠️ PARTIAL | Dev mode (runtime interpretation) addressed; Production mode (code generation) deferred |
| **IV. Backend Abstraction** | ✅ PASS | Builder in gravity-iced, core remains backend-agnostic |
| **V. Test-First Development** | ✅ PASS | Spec has comprehensive acceptance scenarios, tests required |

### Gate Decision

**PASSED** - All principles are satisfied or have justified partial compliance:
- Dual-mode architecture: Production code generation is a separate phase after this feature (dev mode interpreter first)
- All other principles fully satisfied by the design

### Violations to Track

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| Production mode deferred | Dev mode interpreter is MVP; code generation is separate feature | Can't build code generator without working interpreter first |

## Project Structure

### Documentation (this feature)

```text
specs/003-widget-builder/
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
│   └── src/
│       ├── ir/          # Existing: WidgetNode, StyleProperties, LayoutConstraints
│       └── binding/     # Existing: UiBindable trait
├── gravity-iced/
│   └── src/
│       ├── lib.rs       # Export builder
│       ├── builder.rs   # NEW: GravityWidgetBuilder struct
│       ├── convert.rs   # NEW: From implementations IR → Iced
│       └── widgets/     # Existing: widget mappings
├── gravity-runtime/
│   └── src/
│       ├── interpreter.rs  # Existing: HandlerRegistry, evaluate_binding_expr
│       └── state.rs        # Existing: model handling
└── gravity-cli/
    └── src/
        └── commands/
            └── dev.rs      # Existing: --verbose flag support

examples/
├── styling/
│   └── src/
│       ├── main.rs      # TO BE SIMPLIFIED: 410 → <50 lines
│       └── state_demo.rs # TO BE SIMPLIFIED: 200 → <50 lines
└── counter/             # Existing handler example
```

**Structure Decision**: Library crate extension (gravity-iced) with existing repository structure. No new projects or directories needed. All changes contained within gravity-iced/src/ (new files) and examples/ (simplification).

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| Production mode deferred | Dev mode interpreter is MVP; code generation is separate feature | Can't build code generator without working interpreter first; interpreter validates approach before investing in codegen
