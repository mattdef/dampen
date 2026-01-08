# Implementation Plan: Add Radio Widget

**Branch**: `007-add-radio-widget` | **Date**: 2026-01-08 | **Spec**: [link](../spec.md)
**Input**: Feature specification from `/specs/007-add-radio-widget/spec.md`

## Summary

Add a Radio widget to the Gravity declarative UI framework. The radio widget enables users to select a single option from a group of mutually exclusive choices. This follows the existing widget patterns (similar to Checkbox, Slider, PickList) and integrates with the XML-based declarative UI definition system.

## Technical Context

**Language/Version**: Rust Edition 2024, MSRV 1.75 (per constitution)  
**Primary Dependencies**: `iced` 0.14+ (reference backend), `gravity-core`, `gravity-iced`  
**Storage**: N/A (UI widget, no persistence)  
**Testing**: `cargo test`, property-based tests for parser edge cases  
**Target Platform**: Windows, Linux, macOS (tier 1 per constitution)  
**Project Type**: Rust library crate (widget library)  
**Performance Goals**: < 10ms XML parse time for 1000 widgets (per constitution)  
**Constraints**: Backend-agnostic core, no Iced types in public API  
**Scale/Scope**: Single widget addition to existing framework (~5 files modified)  

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle | Status | Notes |
|-----------|--------|-------|
| I. Declarative-First | ✅ PASS | XML is source of truth for radio widget structure |
| II. Type Safety Preservation | ✅ PASS | Radio values map to concrete types, no `Any`-based dispatch |
| III. Dual-Mode Architecture | ✅ PASS | Works in both dev (runtime) and prod (static codegen) modes |
| IV. Backend Abstraction | ✅ PASS | Core trait defines radio, Iced backend implements it |
| V. Test-First Development | ✅ PASS | Contract tests for XML parsing and IR generation |

## Project Structure

### Documentation (this feature)

```text
specs/007-add-radio-widget/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output (skipped - no unknowns)
├── data-model.md        # Phase 1 output
├── quickstart.md        # Phase 1 output
├── contracts/           # Phase 1 output
└── tasks.md             # Phase 2 output (/speckit.tasks command)
```

### Source Code (repository root)

```text
crates/gravity-core/
├── src/
│   ├── traits/backend.rs        # Add radio() to Backend trait
│   └── ir/node.rs               # Add Radio to WidgetKind enum
│
crates/gravity-iced/
├── src/
│   ├── lib.rs                   # Implement radio for IcedBackend
│   └── widgets/                 # Radio widget implementation
│
crates/gravity-runtime/
├── src/
│   └── interpreter.rs           # Handle radio selection events
```

**Structure Decision**: Standard Gravity project structure following existing widget patterns. Radio widget integrates with the existing crate architecture without adding new crates.

## Complexity Tracking

*No constitution violations requiring justification.*
