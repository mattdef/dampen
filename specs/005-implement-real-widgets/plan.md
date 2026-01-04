# Implementation Plan: Implement Real Iced Widgets

**Branch**: `005-implement-real-widgets` | **Date**: 2026-01-04 | **Spec**: [spec.md](./spec.md)  
**Input**: Feature specification from `/specs/005-implement-real-widgets/spec.md`

## Summary

Replace six placeholder widget builders in `gravity-iced/src/builder.rs` with functional Iced widgets:
- `text_input`: Text entry with value binding and on_input events
- `checkbox`: Boolean toggle with label and on_toggle events
- `toggler`: Modern switch with label and on_toggle events
- `pick_list`: Dropdown selection with options and on_select events
- `slider`: Numeric range selection with on_change events
- `image`: Display image from file path

All widgets use the existing `HandlerMessage::Handler(name, Option<String>)` pattern for events, with values serialized to strings.

## Technical Context

**Language/Version**: Rust Edition 2021, MSRV 1.75  
**Primary Dependencies**: Iced 0.14 (with `image` feature enabled), gravity-core  
**Storage**: N/A (UI widgets only)  
**Testing**: `cargo test` with existing test patterns in `crates/gravity-iced/tests/`  
**Target Platform**: Windows, Linux, macOS (tier 1)  
**Project Type**: Rust workspace with multiple crates  
**Performance Goals**: Widget build < 1ms per widget (existing benchmark baseline)  
**Constraints**: No breaking changes to HandlerMessage API  
**Scale/Scope**: 6 widget implementations in single file

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle | Status | Notes |
|-----------|--------|-------|
| I. Declarative-First | PASS | Widgets defined in XML, builders interpret to Iced |
| II. Type Safety | PASS | Using existing HandlerMessage enum, no Any-based dispatch in core |
| III. Dual-Mode | PASS | Builder supports both dev (hot-reload) and prod modes |
| IV. Backend Abstraction | PASS | Changes only in gravity-iced crate, core unchanged |
| V. Test-First | PASS | Tests will be added for each widget builder |

**Post-Design Re-check**: All principles satisfied. No violations.

## Project Structure

### Documentation (this feature)

```text
specs/005-implement-real-widgets/
├── plan.md              # This file
├── spec.md              # Feature specification
├── research.md          # Iced widget API research
├── data-model.md        # Attribute and event mappings
├── quickstart.md        # Usage examples
├── contracts/
│   └── xml-schema.md    # XML element definitions
└── checklists/
    └── requirements.md  # Specification quality checklist
```

### Source Code (repository root)

```text
crates/
├── gravity-core/           # No changes required
│   └── src/
│       └── ir/
│           └── node.rs     # Existing EventKind, WidgetKind (unchanged)
│
├── gravity-iced/           # PRIMARY CHANGES HERE
│   ├── src/
│   │   ├── lib.rs          # HandlerMessage (unchanged)
│   │   └── builder.rs      # Widget builder implementations (6 methods)
│   └── tests/
│       ├── builder_basic_tests.rs      # Existing tests
│       └── widget_tests.rs             # NEW: Widget-specific tests
│
└── gravity-macros/         # No changes required

examples/
├── todo-app/               # Uses text_input, pick_list, toggler
│   └── ui/
│       └── main.gravity    # Validation that widgets work
└── widget-showcase/        # NEW: Demo for all widgets
    └── ui/
        └── inputs.gravity  # NEW: Input widget showcase
```

**Structure Decision**: Existing Rust workspace structure. All widget implementations go in `crates/gravity-iced/src/builder.rs` replacing the placeholder methods. Tests added to `crates/gravity-iced/tests/`.

## Complexity Tracking

No constitution violations. No complexity justification needed.

## Implementation Phases

### Phase 1: P1 Widgets (text_input, checkbox, toggler)

1. **build_text_input**: Implement with placeholder, value binding, on_input event
2. **build_checkbox**: Implement with label, checked binding, on_toggle event  
3. **build_toggler**: Implement with label, active binding, on_toggle event
4. **Tests**: Add tests for each widget builder

### Phase 2: P2 Widgets (pick_list, slider)

1. **build_pick_list**: Implement with options parsing, selected binding, on_select event
2. **build_slider**: Implement with min/max/value, on_change event
3. **Tests**: Add tests for each widget builder

### Phase 3: P3 Widget (image) & Integration

1. **build_image**: Implement with src path, width/height attributes
2. **Integration Tests**: Verify todo-app example works with new widgets
3. **Documentation**: Update examples and README if needed

## Dependencies

```text
Phase 1 ──► Phase 2 ──► Phase 3
   │           │           │
   └─ tests ─┴─ tests ─────┴─ integration tests
```

All phases are sequential. Each phase includes its own tests before proceeding.

## Risk Mitigation

| Risk | Mitigation |
|------|------------|
| Lifetime issues with owned strings | Use builder scope ownership pattern (see research.md) |
| PickList options parsing edge cases | Handle empty options, whitespace trimming |
| Image file not found | Log warning in verbose mode, let Iced handle display |
| Slider value outside range | Clamp to [min, max] before creating widget |
