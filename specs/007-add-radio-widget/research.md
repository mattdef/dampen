# Research: Add Radio Widget

**Feature**: 007-add-radio-widget
**Date**: 2026-01-08
**Status**: Complete (no unknowns - all decisions based on existing patterns)

## Technical Decisions

### Decision 1: Radio Widget Architecture

**Decision**: Follow the existing Checkbox widget pattern for radio implementation.

**Rationale**:
- Radio buttons share similar structure with checkboxes (label + state + on_change handler)
- Checkbox is already implemented and tested, providing a proven template
- Consistent API across widgets improves developer experience
- The Iced radio widget signature matches the checkbox pattern closely

**Alternatives Considered**:
- Create a dedicated RadioGroup container (rejected - adds unnecessary complexity for simple single-choice use cases)
- Use pick_list dropdown instead (rejected - different UI paradigm, radio buttons are explicit and visible)

### Decision 2: Backend Trait Addition

**Decision**: Add `radio()` function to the Backend trait with signature:
```rust
fn radio<'a>(
    &self,
    label: &str,
    value: &str,
    selected: Option<&str>,
    on_select: Option<Self::Message>,
) -> Self::Widget<'a>;
```

**Rationale**:
- Consistent with checkbox signature pattern
- String-based values are flexible for type-safe bindings in XML
- Optional selected value supports both default selection and no-selection states
- Optional on_select supports read-only display mode

**Alternatives Considered**:
- Generic value type (rejected - complicates trait, string is sufficient for IR)
- Required on_select (rejected - some use cases need read-only radio groups)

### Decision 3: WidgetKind Enum Variant

**Decision**: Add `Radio` variant to `WidgetKind` enum with no special attributes struct.

**Rationale**:
- Radio buttons are atomic widgets (like Button, Text), not container widgets
- Simple label/value/selected state fits standard WidgetNode structure
- No need for complex attribute structures like PickList or ComboBox

**Alternatives Considered**:
- RadioGroup container (rejected - over-engineering for single-widget behavior)

### Decision 4: Event Handling

**Decision**: Reuse existing `Select` event kind from EventKind enum.

**Rationale**:
- Radio selection is semantically a "select" operation
- Consistent with PickList's on_select event
- Avoids creating new event kinds unnecessarily

### Decision 5: XML Syntax

**Decision**: Use `<radio label="Option Text" value="option_a" selected="option_a" on_select="handleSelection"/>` syntax.

**Rationale**:
- Consistent with existing widget XML syntax
- Self-contained single element (no child radio elements needed)
- Matches Iced's flat radio button API

**Alternatives Considered**:
- Nested radio buttons with parent RadioGroup (rejected - more verbose, no functional benefit)

## Key References

- Iced Radio Widget Documentation: https://docs.rs/iced/latest/iced/widget/radio/index.html
- Existing Checkbox implementation for API reference
- Backend trait pattern from gravity-core/src/traits/backend.rs
