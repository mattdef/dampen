# Feature Specification: Harmonize Modes

**Feature Branch**: `001-harmonize-modes`
**Created**: 2026-01-19
**Status**: Draft
**Input**: Plan d'Harmonisation Dampen (Interpreted vs Codegen)

## 1. Context & Problem Statement

Dampen currently suffers from a significant divergence between its Interpreted (Development) and Codegen (Production) modes. This "dual-mode" architecture, while beneficial for hot-reloading, has led to a fragmented ecosystem where features available in development (like state-aware styling and flexible layouts) are missing or behave differently in production.

This divergence undermines confidence in the development process, as `dampen run` does not accurately preview `dampen build`. The goal of this feature is to achieve strict 100% parity between the two modes, ensuring that the same XML input produces identical visual and behavioral output in both environments.

## 2. User Scenarios & Testing *(mandatory)*

### User Story 1 - Reliable Development-to-Production Workflow (Priority: P1)

A developer using `dampen run` (Interpreted) to build a complex UI with specific layouts and hover states must see the exact same result when building for production with `dampen build` (Codegen).

**Why this priority**: The core value proposition of Dampen is broken if dev and prod modes diverge. Developers cannot trust their tools if the preview differs from the final artifact.

**Independent Test**: Can be tested by creating a sample app with complex layouts and styles, running it in both modes, and verifying they match visually and functionally.

**Acceptance Scenarios**:

1. **Given** a `.dampen` file with a complex nested layout (Columns inside Rows with alignment), **When** running `dampen run` and `dampen build`, **Then** the visual geometry (positions, sizes) is identical in both outputs.
2. **Given** a widget with `background:hover` styling, **When** compiled with `dampen build`, **Then** the compiled application reacts to mouse hover events exactly as the interpreted version does.

---

### User Story 2 - Visual Regression Testing Infrastructure (Priority: P1)

Maintainers need an automated way to verify that changes to the core framework do not introduce visual regressions or divergence between modes.

**Why this priority**: Without automated visual verification, maintaining parity during refactors is impossible and regression-prone.

**Independent Test**: Can be tested by running the visual test suite and confirming it detects differences when a mode is intentionally broken.

**Acceptance Scenarios**:

1. **Given** a set of standard examples, **When** the visual test suite runs, **Then** it generates screenshots for both Interpreted and Codegen modes and asserts they are pixel-identical (within tolerance).
2. **Given** a known divergence (e.g., breaking the layout in one mode), **When** the test suite runs, **Then** it correctly reports a failure and outputs a diff image.

---

### User Story 3 - State-Aware Styling in Production (Priority: P2)

Developers want to ship performant native applications that support rich interactivity (hover, focus, active states) defined in their declarative XML.

**Why this priority**: Currently, Codegen mode ignores state-based styles, making production apps feel lifeless compared to their dev prototypes.

**Independent Test**: Can be tested by inspecting the generated code or the final binary behavior for specific widgets (Button, TextInput).

**Acceptance Scenarios**:

1. **Given** a `Button` with `border-color:focused="..."`, **When** building for production, **Then** the generated code implements the necessary native logic for the `focused` state.
2. **Given** a `TextInput` with `color:placeholder="..."`, **When** building for production, **Then** the placeholder text uses the specified color.

### Edge Cases

- What happens if an attribute is valid in the underlying library but not yet standardized in Dampen? (Should be rejected by parser).
- How does the system handle conflicting styles (e.g., inline style vs class)? (Precedence: Inline > Class > Default).
- What if a layout attribute (e.g., `width`) is applied to a widget that doesn't support it natively? (Codegen must wrap it in a Container).

## 3. Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST define and enforce a standardized attribute contract ("Source of Truth") for all widgets, resolving naming conflicts (e.g., `active` vs `toggled`).
- **FR-002**: System MUST unify layout behavior so that `Column`, `Row`, `Container`, and `Scrollable` support `width`, `height`, `padding`, `align_x`, and `align_y` in both modes.
- **FR-003**: Codegen mode MUST automatically generate wrapper layout containers when layout attributes are present on widgets that do not support them natively.
- **FR-004**: Codegen mode MUST generate efficient static code implementing state-aware styling (`hover`, `active`, `focused`, `disabled`).
- **FR-005**: System MUST support the standardized loop syntax `for item in items` in both modes.
- **FR-006**: `TextInput` MUST support `password` and `color` attributes in Codegen.
- **FR-007**: `Slider` MUST support the `step` attribute in Codegen.
- **FR-008**: `Image` and `Svg` widgets MUST accept unified `src` and `path` attributes in both modes.
- **FR-009**: A visual testing harness MUST be implemented to capture and compare screenshots of Interpreted vs Codegen outputs.

### Key Entities

- **WidgetNode**: The Intermediate Representation (IR) node, updated to hold standardized layout and style properties.
- **AttributeRegistry**: The conceptual source of truth defining valid attributes for each widget type.

## 4. Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: 100% of the standard widget gallery examples produce pixel-identical output (within <1% anti-aliasing tolerance) in both Interpreted and Codegen modes.
- **SC-002**: All interactive widgets (Button, TextInput, Checkbox, Radio, Toggler) support `hover`, `focus`, `active`, and `disabled` states in Codegen mode.
- **SC-003**: The standard loop syntax `for item in items` compiles and runs correctly in 100% of test cases in both modes.
- **SC-004**: Layout attributes (`width`, `height`, `padding`, `align`) function correctly on all container widgets in Codegen mode.

## 5. Assumptions & Constraints

- **Breaking Changes**: This feature will introduce breaking changes to the XML schema (attribute renames). Users will be expected to update their code.
- **Baseline Validity**: The current Interpreted mode's rendering is considered the "correct" baseline for visual appearance.
- **Underlying Library Dependency**: We are constrained by what is possible within the underlying UI library's styling API for the Codegen implementation.
- **Performance**: We assume that the increased code size from static style generation will be acceptable for production builds.

## 6. Questions & Clarifications

None. The feature plan provided was comprehensive and decisive regarding key architectural choices (Source of Truth, Breaking Changes, Hot-Reload behavior).
