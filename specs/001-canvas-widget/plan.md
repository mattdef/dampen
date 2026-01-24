# Implementation Plan: Canvas Widget

**Branch**: `001-canvas-widget` | **Date**: 2026-01-24 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/001-canvas-widget/spec.md`

## Summary

Implement a declarative Canvas widget for Dampen v1.1 that supports both static shape definitions in XML (rect, circle, line, text, group) and custom Rust drawing programs via binding. The canvas operates in two mutually exclusive modes: declarative shapes or program binding. All events (click, drag, move, release) are handled at the canvas level with coordinate data. Implementation must work identically in interpreted (hot-reload) and codegen (production) modes.

## Technical Context

**Language/Version**: Rust Edition 2024, MSRV 1.88  
**Primary Dependencies**: iced 0.14+ (canvas widget, Program trait), roxmltree 0.19+, dampen-core, dampen-iced, dampen-macros  
**Storage**: N/A (no persistence required)  
**Testing**: cargo test, proptest 1.0+, insta 1.0+ (snapshots)  
**Target Platform**: Desktop (Linux, macOS, Windows via Iced)  
**Project Type**: Multi-crate workspace (existing structure)  
**Performance Goals**: <16ms frame time (60fps), 100 shapes with bindings, <10ms parse for 1000 widgets  
**Constraints**: Zero unsafe in generated code, no Iced dependency in dampen-core  
**Scale/Scope**: Single widget type with 4 shape primitives + group + 4 event types

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

### Pre-Research Check (2026-01-24)

| Principle | Status | Notes |
|-----------|--------|-------|
| I. Declarative-First | ✅ PASS | Canvas shapes defined in XML; XML is source of truth |
| II. Type Safety Preservation | ✅ PASS | Shape IR types statically typed; bindings validated |
| III. Production Mode | ✅ PASS | Codegen mode generates Rust code; no runtime XML parsing |
| IV. Backend Abstraction | ✅ PASS | Shape IR in dampen-core (no Iced); rendering in dampen-iced |
| V. Test-First Development | ✅ PASS | TDD workflow; contract tests before implementation |

### Post-Design Re-Check (2026-01-24)

| Principle | Status | Design Validation |
|-----------|--------|-------------------|
| I. Declarative-First | ✅ PASS | `data-model.md` defines ShapeNode IR; `canvas-xml.md` contract specifies XML syntax |
| II. Type Safety Preservation | ✅ PASS | ShapeKind enum, typed CanvasEvent struct; BindingExpr for dynamic values |
| III. Production Mode | ✅ PASS | `research.md` §7 confirms codegen generates DeclarativeProgram at compile time |
| IV. Backend Abstraction | ✅ PASS | Shape IR types are Iced-free; DeclarativeProgram in dampen-iced only |
| V. Test-First Development | ✅ PASS | `tasks.md` structures tests before implementation per story |

**All gates passed. Ready for Phase 2 (task decomposition).**

**Quality Gates**:
- `cargo test --workspace` must pass
- `cargo clippy --workspace -- -D warnings` zero warnings
- `cargo fmt --all -- --check` properly formatted
- All public items documented with rustdoc

**Error Handling**:
- Result<T, E> for all fallible operations
- Custom error types with thiserror
- Span (line/column) in parse errors
- Actionable error messages with fix suggestions

## Project Structure

### Documentation (this feature)

```text
specs/001-canvas-widget/
├── plan.md              # This file
├── research.md          # Phase 0 output
├── data-model.md        # Phase 1 output
├── quickstart.md        # Phase 1 output
├── contracts/           # Phase 1 output
└── tasks.md             # Phase 2 output (NOT created by /speckit.plan)
```

### Source Code (repository root)

```text
crates/
├── dampen-core/
│   └── src/
│       ├── ir/
│       │   ├── node.rs          # WidgetKind::Canvas, shape IR types
│       │   └── canvas.rs        # NEW: ShapeNode, ShapeKind, TransformNode
│       ├── parser/
│       │   └── canvas.rs        # NEW: Canvas/shape XML parsing
│       ├── schema/
│       │   └── mod.rs           # Canvas + shape widget schemas
│       └── codegen/
│           └── canvas.rs        # NEW: Canvas codegen with shapes
├── dampen-iced/
│   └── src/
│       └── builder/
│           └── widgets/
│               └── canvas.rs    # Canvas widget builder (existing, to extend)
├── dampen-macros/
│   └── src/
│       └── canvas.rs            # NEW: Canvas proc-macro support
└── dampen-cli/
    └── src/
        └── commands/
            └── check/           # Validation updates for canvas

tests/
├── contract/
│   └── canvas/                  # NEW: Canvas contract tests
├── integration/
│   └── canvas/                  # NEW: Canvas integration tests
└── fixtures/
    └── canvas/                  # NEW: Canvas test fixtures (.dampen files)
```

**Structure Decision**: Existing multi-crate workspace. Canvas IR types in dampen-core (backend-agnostic), rendering in dampen-iced (Iced-specific), schema validation updated in both.

## Complexity Tracking

> No constitution violations requiring justification.

| Decision | Rationale |
|----------|-----------|
| Shape IR in dampen-core | Required by Principle IV (Backend Abstraction) |
| Separate canvas.rs modules | Follows existing pattern (widgets have dedicated files) |
| Mutually exclusive modes | Simplifies implementation, avoids z-order ambiguity |
