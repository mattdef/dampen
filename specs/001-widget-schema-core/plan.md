# Implementation Plan: Widget Schema Migration to Core

**Branch**: `001-widget-schema-core` | **Date**: 2026-01-23 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/001-widget-schema-core/spec.md`

## Summary

Migrate widget attribute validation schema from `dampen-cli` to `dampen-core` to establish a single source of truth. The CLI will query the core for valid attributes instead of maintaining its own duplicate definitions. This eliminates synchronization bugs when adding new widget attributes.

## Technical Context

**Language/Version**: Rust Edition 2024, MSRV 1.85  
**Primary Dependencies**: dampen-core (internal), dampen-cli (internal), lazy_static (to be removed from CLI)  
**Storage**: N/A (compile-time constants, no persistence)  
**Testing**: cargo test --workspace  
**Target Platform**: Cross-platform (Linux, macOS, Windows)  
**Project Type**: Workspace with multiple crates  
**Performance Goals**: Zero runtime allocations for schema lookups, < 10ms for 1000 widget validations  
**Constraints**: No new dependencies, maintain backward compatibility with existing `dampen check` behavior  
**Scale/Scope**: ~25 widget types, ~50 unique attributes across all categories

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

### Pre-Design Check (Phase 0)

| Principle | Status | Notes |
|-----------|--------|-------|
| I. Declarative-First | PASS | No impact on XML parsing or UI definitions |
| II. Type Safety Preservation | PASS | Static typing preserved via `&'static str` slices |
| III. Production Mode | PASS | Schema is compile-time constant, no runtime overhead |
| IV. Backend Abstraction | PASS | Schema module in dampen-core has no Iced dependency |
| V. Test-First Development | PASS | Will write tests before implementation |

### Post-Design Check (Phase 1)

| Principle | Status | Notes |
|-----------|--------|-------|
| I. Declarative-First | PASS | Design confirms no impact on XML parsing |
| II. Type Safety Preservation | PASS | `WidgetSchema` uses static slices, `all_valid()` returns typed HashSet |
| III. Production Mode | PASS | Constants are compile-time, zero runtime parsing |
| IV. Backend Abstraction | PASS | `schema` module imports only `WidgetKind` from `ir::node`, no Iced |
| V. Test-First Development | PASS | Test plan defined in research.md |

**Quality Gates**:
- Tests: Will add unit tests for schema module in dampen-core
- Linting: Zero warnings enforced
- Formatting: cargo fmt compliant
- Documentation: All public items will have rustdoc comments

**Error Handling**: N/A (schema lookup is infallible - returns struct for any WidgetKind)

## Project Structure

### Documentation (this feature)

```text
specs/001-widget-schema-core/
├── plan.md              # This file
├── spec.md              # Feature specification
├── research.md          # Phase 0 output
├── data-model.md        # Phase 1 output
├── quickstart.md        # Phase 1 output
├── contracts/           # Phase 1 output (N/A for this feature)
└── tasks.md             # Phase 2 output
```

### Source Code (repository root)

```text
crates/
├── dampen-core/
│   └── src/
│       ├── lib.rs                    # Add: pub mod schema; pub use schema::*;
│       └── schema/
│           └── mod.rs                # NEW: WidgetSchema, constants, get_widget_schema()
│
└── dampen-cli/
    └── src/
        └── commands/
            └── check/
                ├── attributes.rs     # MODIFY: Remove definitions, import from core
                └── mod.rs            # Minor import adjustments
```

**Structure Decision**: Existing workspace structure preserved. New `schema` module added to dampen-core. dampen-cli refactored to import from dampen-core instead of defining locally.

## Complexity Tracking

> No violations requiring justification. This migration simplifies the codebase by removing duplication.

| Change | Rationale |
|--------|-----------|
| Using `&'static [&'static str]` instead of `HashSet` | Compile-time constant, zero allocations, matches constitution performance goals |
| Keeping `all_valid()` method returning `HashSet` | Maintains API compatibility with existing CLI code that uses `contains()` |
