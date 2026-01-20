# Implementation Plan: Inline State Styles & Responsive Design

**Branch**: `002-inline-state-responsive` | **Date**: 2026-01-19 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/002-inline-state-responsive/spec.md`

## Summary

Implement two documented but non-functional features for the Dampen UI framework:
1. **Inline State Styles**: Parse and apply state-prefixed attributes (e.g., `hover:background="#ff0000"`) directly on widgets
2. **Responsive Design**: Utilize existing `breakpoint_attributes` infrastructure to apply viewport-aware styles

Both features must work in interpreted mode (`dampen run`) and codegen mode (`dampen build`/`dampen release`) with visual parity.

## Technical Context

**Language/Version**: Rust Edition 2024, MSRV 1.85  
**Primary Dependencies**: iced 0.14+, roxmltree 0.19+, syn 2.0+, quote 2.0+, proc-macro2 2.0+  
**Storage**: N/A (UI framework, no persistence)  
**Testing**: cargo test, insta (snapshots), proptest  
**Target Platform**: Desktop (Linux, macOS, Windows via Iced)  
**Project Type**: Workspace with multiple crates (dampen-core, dampen-iced, dampen-macros, dampen-cli)  
**Performance Goals**: XML parse time < 10ms for 1000 widgets, code generation < 5s  
**Constraints**: Zero unsafe in generated code, runtime memory < 50MB baseline  
**Scale/Scope**: Framework feature affecting parser, IR, builder, and codegen subsystems

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle | Status | Notes |
|-----------|--------|-------|
| **I. Declarative-First** | PASS | XML remains source of truth; inline state styles are declarative attributes |
| **II. Type Safety Preservation** | PASS | WidgetState enum is typed; no runtime type erasure |
| **III. Production Mode** | PASS | Codegen will compile state styles to static Rust code |
| **IV. Backend Abstraction** | PASS | dampen-core changes remain backend-agnostic; Iced-specific logic in dampen-iced |
| **V. Test-First Development** | PASS | Plan includes test phases before implementation |

**Quality Gates Compliance**:
- Tests: Unit tests for parser, integration tests for builder, snapshot tests for codegen
- Linting: No clippy warnings allowed
- Documentation: Public APIs will have rustdoc comments
- Error Handling: ParseError with Span for invalid state prefixes

**Complexity Justification**: None required - leveraging existing infrastructure (WidgetState, Breakpoint, breakpoint_attributes).

## Project Structure

### Documentation (this feature)

```text
specs/002-inline-state-responsive/
├── plan.md              # This file
├── spec.md              # Feature specification
├── research.md          # Phase 0 output
├── data-model.md        # Phase 1 output
├── quickstart.md        # Phase 1 output
├── contracts/           # Phase 1 output (internal APIs)
└── tasks.md             # Phase 2 output (via /speckit.tasks)
```

### Source Code (repository root)

```text
crates/
├── dampen-core/
│   └── src/
│       ├── ir/
│       │   ├── node.rs          # Add inline_state_variants field
│       │   ├── theme.rs         # WidgetState (exists), add from_prefix()
│       │   └── layout.rs        # Breakpoint (exists)
│       └── parser/
│           └── mod.rs           # Parse state-prefixed attributes
├── dampen-iced/
│   └── src/
│       └── builder/
│           ├── helpers.rs       # resolve_complete_styles_with_states()
│           ├── button.rs        # State-aware style closure
│           ├── text_input.rs    # State-aware style closure
│           ├── checkbox.rs      # State-aware style closure
│           ├── slider.rs        # State-aware style closure
│           ├── toggler.rs       # State-aware style closure
│           ├── radio.rs         # State-aware style closure
│           └── pick_list.rs     # State-aware style closure
└── dampen-macros/
    └── src/
        └── dampen_app.rs        # Codegen for state-aware closures

tests/
├── crates/dampen-core/tests/
│   └── parser_inline_states.rs  # Parser unit tests
└── crates/dampen-iced/tests/
    └── builder_state_styles.rs  # Builder integration tests
```

**Structure Decision**: Using existing workspace structure. Changes span 3 crates (dampen-core, dampen-iced, dampen-macros) following the established crate dependency hierarchy.

## Complexity Tracking

No violations to justify - implementation uses existing infrastructure:
- `WidgetState` enum exists in `dampen-core/src/ir/theme.rs`
- `Breakpoint` enum exists in `dampen-core/src/ir/layout.rs`
- `breakpoint_attributes` field exists in `WidgetNode`
- State mapping functions exist in `dampen-iced/src/style_mapping.rs`

---

## Constitution Check (Post-Design)

*Re-evaluated after Phase 1 design completion.*

| Principle | Status | Design Verification |
|-----------|--------|---------------------|
| **I. Declarative-First** | PASS | State styles declared in XML (`hover:background`), parsed to IR, no imperative code in UI definition |
| **II. Type Safety Preservation** | PASS | `WidgetState` enum typed, `HashMap<WidgetState, StyleProperties>` preserves types, no dynamic typing |
| **III. Production Mode** | PASS | Codegen contract defines static match expressions, no runtime XML parsing in production |
| **IV. Backend Abstraction** | PASS | `WidgetState`, `inline_state_variants` in dampen-core (no Iced); style closures in dampen-iced only |
| **V. Test-First Development** | PASS | Test contracts defined in `contracts/`, test examples in `quickstart.md` |

**Quality Gates Verification**:
- Error Handling: `ParseError` with `Span` for invalid prefixes (defined in `contracts/parser-api.md`)
- Performance: No new parsing overhead beyond attribute iteration (O(n) where n = attributes)
- Documentation: Public APIs documented in contracts and quickstart

**Design Artifacts Generated**:
- `research.md` - 10 technical decisions documented
- `data-model.md` - Entity changes and relationships
- `contracts/parser-api.md` - Parser contract
- `contracts/builder-api.md` - Builder contract  
- `contracts/codegen-api.md` - Codegen contract
- `quickstart.md` - Implementation guide

**Ready for Phase 2**: Tasks generation via `/speckit.tasks`
