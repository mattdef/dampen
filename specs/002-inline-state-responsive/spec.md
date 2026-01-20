# Feature Specification: Inline State Styles & Responsive Design

**Feature Branch**: `002-inline-state-responsive`  
**Created**: 2026-01-19  
**Status**: Draft  
**Input**: Implementation plan for inline state styles (e.g., `hover:background`) and responsive design (e.g., `mobile-spacing`)

## User Scenarios & Testing

### User Story 1 - Inline Hover Styling (Priority: P1)

A developer wants to add hover effects directly on a button without creating a style class. They write `<button hover:background="#ff0000" label="Click me" />` and the button background changes to red when hovered.

**Why this priority**: Inline state styles are the most common use case for quick prototyping and single-use components. This provides immediate visual feedback during development.

**Independent Test**: Can be fully tested by rendering a button with `hover:background`, hovering over it, and verifying the background color changes. Delivers immediate value for rapid UI development.

**Acceptance Scenarios**:

1. **Given** a button with `hover:background="#ff0000"`, **When** the user hovers over the button, **Then** the background changes to red
2. **Given** a button with `hover:background` and a base `background` attribute, **When** hovering, **Then** the hover style overrides the base style
3. **Given** a button with `hover:background` and a class with a hover style, **When** hovering, **Then** the inline hover style takes precedence over the class style

---

### User Story 2 - Multiple Inline State Styles (Priority: P1)

A developer wants to define different styles for multiple states on a single widget. They write `<button hover:background="#blue" active:background="#green" disabled:opacity="0.5" label="Multi-state" />` and each state applies correctly.

**Why this priority**: Complete state coverage is essential for production-ready UI components.

**Independent Test**: Can be tested by rendering a button with multiple state attributes and triggering each state (hover, click/active, disable) to verify correct styles apply.

**Acceptance Scenarios**:

1. **Given** a button with `hover:background`, `active:background`, and `disabled:opacity`, **When** each state is triggered, **Then** the corresponding style applies
2. **Given** multiple state attributes, **When** the widget is in default state, **Then** only base styles apply (no state styles)

---

### User Story 3 - Responsive Layout with Breakpoints (Priority: P2)

A developer wants a column to have different spacing on mobile vs desktop. They write `<column mobile-spacing="10" desktop-spacing="40">` and the spacing adjusts based on viewport width.

**Why this priority**: Responsive design is critical for cross-device applications but depends on viewport tracking infrastructure.

**Independent Test**: Can be tested by rendering a column with breakpoint-specific spacing, resizing the viewport, and verifying spacing changes at breakpoint thresholds (640px, 1024px).

**Acceptance Scenarios**:

1. **Given** a column with `mobile-spacing="10"` and `desktop-spacing="40"`, **When** viewport is < 640px, **Then** spacing is 10
2. **Given** a column with `mobile-spacing="10"` and `desktop-spacing="40"`, **When** viewport is >= 1024px, **Then** spacing is 40
3. **Given** breakpoint attributes without a base attribute, **When** no breakpoint matches, **Then** the widget uses default values

---

### User Story 4 - Interpreted Mode Hot-Reload (Priority: P2)

A developer using `dampen run` (interpreted mode) modifies inline state styles in the XML and sees changes reflected immediately without restart.

**Why this priority**: Hot-reload is the primary development experience benefit.

**Independent Test**: Run app in interpreted mode, modify `hover:background` value, verify change appears on next hover without app restart.

**Acceptance Scenarios**:

1. **Given** interpreted mode running, **When** inline state style is modified in XML, **Then** the change is visible after hot-reload
2. **Given** interpreted mode running, **When** breakpoint attribute is modified, **Then** the responsive behavior updates

---

### User Story 5 - Codegen Mode Type Safety (Priority: P3)

A developer building for production with `dampen build` has inline state styles compiled to Rust code with full type safety.

**Why this priority**: Production deployments require codegen mode for performance and type safety.

**Independent Test**: Build with codegen, verify generated Rust code contains state-aware style closures, run compiled binary and verify state styles work.

**Acceptance Scenarios**:

1. **Given** XML with inline state styles, **When** codegen runs, **Then** generated Rust code includes style closures matching Iced's status patterns
2. **Given** invalid state prefix (e.g., `invalid:background`), **When** codegen runs, **Then** a compile-time error is raised

---

### Edge Cases

- What happens when an invalid state prefix is used (e.g., `unknown:background`)? Parser should emit a warning or error.
- How does system handle conflicting breakpoint attributes (e.g., both `mobile-spacing` and `spacing` defined)? Base attribute should be fallback, breakpoint-specific overrides.
- What happens with state styles on non-interactive widgets (e.g., `<text hover:color="#red">`)? Should be ignored or documented as unsupported.
- What happens when viewport tracking is unavailable? Desktop breakpoint should be assumed as default.

## Requirements

### Functional Requirements

- **FR-001**: Parser MUST detect state-prefixed attributes using colon separator (e.g., `hover:background`, `focus:border`)
- **FR-002**: Parser MUST validate state prefixes against known states: `hover`, `focus`, `active`, `disabled`
- **FR-003**: Parser MUST store inline state styles in a structured `inline_state_variants` field on `WidgetNode`
- **FR-004**: Builder MUST merge inline state styles with the existing precedence chain: Theme → Class → Inline Base → Inline State
- **FR-005**: Builder MUST generate state-aware style closures for interactive widgets (button, text_input, checkbox, slider, toggler, radio, pick_list)
- **FR-006**: Builder MUST resolve breakpoint-specific attributes based on current viewport width
- **FR-007**: System MUST support viewport width injection into the builder for responsive calculations
- **FR-008**: Codegen MUST generate Rust code that applies inline state styles at compile time
- **FR-009**: Codegen MUST generate Rust code that handles breakpoint resolution at runtime
- **FR-010**: Both modes (interpreted and codegen) MUST produce visually identical results for the same XML

### Key Entities

- **WidgetState**: Enum representing interactive states (Hover, Focus, Active, Disabled) - already exists in `dampen-core/src/ir/theme.rs`
- **Breakpoint**: Enum representing viewport breakpoints (Mobile, Tablet, Desktop) - already exists in `dampen-core/src/ir/layout.rs`
- **WidgetNode.inline_state_variants**: New HashMap field mapping WidgetState to StyleProperties
- **WidgetNode.breakpoint_attributes**: Existing HashMap field mapping Breakpoint to attribute overrides

## Success Criteria

### Measurable Outcomes

- **SC-001**: All 7 interactive widgets support inline state styles in both interpreted and codegen modes
- **SC-002**: Parser correctly identifies 100% of valid state-prefixed attributes
- **SC-003**: Breakpoint resolution accurately applies at documented thresholds (640px, 1024px)
- **SC-004**: Visual parity test passes between interpreted and codegen modes for identical XML
- **SC-005**: All existing tests continue to pass (backward compatibility)
- **SC-006**: New unit tests achieve >90% coverage for added functionality
- **SC-007**: XML parse time remains < 10ms for 1000 widgets (per constitution performance budget)
