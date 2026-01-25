# Implementation Plan: DatePicker & TimePicker Widgets

**Branch**: `001-datetime-picker` | **Date**: 2026-01-25 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/001-datetime-picker/spec.md`

## Summary

Add DatePicker and TimePicker widgets to Dampen v1.1, enabling declarative date/time selection in XML. The implementation extends `dampen-core` with new WidgetKind variants and schema definitions, adds `iced_aw` as a backend dependency in `dampen-iced`, implements builder functions for both widgets, and updates codegen to generate equivalent Rust code for production builds.

## Technical Context

**Language/Version**: Rust Edition 2024, MSRV 1.85
**Primary Dependencies**: iced 0.14+, iced_aw 0.13+ (date_picker, time_picker features), chrono 0.4+ (serde feature)
**Storage**: N/A (UI framework, no persistence)
**Testing**: cargo test (unit tests, snapshot tests with insta)
**Target Platform**: Linux, macOS, Windows (desktop applications)
**Project Type**: Rust workspace (multi-crate)
**Performance Goals**: Widget render <100ms, XML parse <10ms per 1000 widgets
**Constraints**: Zero unsafe code, no runtime panics, actionable error messages
**Scale/Scope**: 2 new widget types, ~8 files modified, ~4 new files created

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle | Status | Notes |
|-----------|--------|-------|
| I. Declarative-First | ✅ PASS | Widgets defined in XML, parsed to IR |
| II. Type Safety Preservation | ✅ PASS | Date/Time types statically typed via chrono |
| III. Production Mode | ✅ PASS | Codegen generates compile-time Rust code |
| IV. Backend Abstraction | ✅ PASS | Core defines WidgetKind; iced_aw only in dampen-iced |
| V. Test-First Development | ✅ PASS | Tests written before implementation |

**Quality Gates**:
- `cargo test --workspace` - All tests pass
- `cargo clippy --workspace -- -D warnings` - Zero warnings
- `cargo fmt --all -- --check` - Properly formatted
- Public APIs documented with rustdoc

**Error Handling**:
- `Result<T, E>` for all fallible operations
- `Span` (line/column) in parse errors
- Actionable error messages with fix suggestions

## Project Structure

### Documentation (this feature)

```text
specs/001-datetime-picker/
├── plan.md              # This file
├── research.md          # Phase 0 output
├── data-model.md        # Phase 1 output
├── quickstart.md        # Phase 1 output
├── contracts/           # Phase 1 output (widget schema contracts)
└── tasks.md             # Phase 2 output (/speckit.tasks command)
```

### Source Code (repository root)

```text
crates/
├── dampen-core/
│   ├── src/
│   │   ├── ir/
│   │   │   └── node.rs           # WidgetKind enum (add DatePicker, TimePicker)
│   │   ├── schema/
│   │   │   └── mod.rs            # Widget schema definitions
│   │   ├── parser/
│   │   │   └── mod.rs            # Widget name mapping
│   │   └── codegen/
│   │       └── view.rs           # Code generation for new widgets
│   └── tests/
│       └── parse_datetime.rs     # New: parser tests for date/time widgets
│
├── dampen-iced/
│   ├── Cargo.toml                # Add iced_aw dependency
│   ├── src/
│   │   └── builder/
│   │       ├── mod.rs            # Add match arms for new widgets
│   │       └── widgets/
│   │           ├── mod.rs        # Export new widget modules
│   │           ├── date_picker.rs  # New: DatePicker builder
│   │           └── time_picker.rs  # New: TimePicker builder
│   └── tests/
│       └── datetime_builder.rs   # New: builder tests
│
└── dampen-macros/
    └── src/
        └── lib.rs                # Codegen macro updates (if needed)

examples/
└── datetime-picker/              # New: example application
    ├── Cargo.toml
    └── src/
        ├── main.rs
        └── ui/
            └── window.dampen
```

**Structure Decision**: Follows existing Dampen workspace structure with dampen-core for backend-agnostic types/parsing and dampen-iced for Iced-specific widget building.

## Complexity Tracking

> No violations - implementation follows established patterns.

| Aspect | Approach | Rationale |
|--------|----------|-----------|
| iced_aw dependency | Add to dampen-iced only | Maintains backend abstraction (Principle IV) |
| chrono in dampen-core | Optional feature | Allows core to remain lightweight when dates not needed |
| Underlay pattern | Single required child | Matches iced_aw API; validated in parser |
