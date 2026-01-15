# Feature Specification: Widget State Styling

**Feature Branch**: `003-widget-state-styling`  
**Created**: 2026-01-15  
**Status**: Draft  
**Input**: User description: "Implement widget state styling (hover, focus, active, disabled) in Dampen framework"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Button State Styling (Priority: P1)

As a UI developer using Dampen, I need buttons to visually respond to user interactions (hover, press, disabled) so that users can understand when buttons are interactive and receive immediate visual feedback.

**Why this priority**: Buttons are the most critical interactive element in any UI. Without state styling, applications feel unresponsive and unprofessional. This is the foundational capability that must work before other widgets.

**Independent Test**: Can be fully tested by creating a button with hover/active/disabled styles in XML, running the application, and verifying visual changes occur when hovering, pressing, and when the button is disabled. Delivers immediate professional visual feedback.

**Acceptance Scenarios**:

1. **Given** a button with `<hover>` style defined in XML, **When** user hovers mouse over the button, **Then** the button displays the hover style (e.g., lighter background color)
2. **Given** a button with `<active>` style defined in XML, **When** user presses the button, **Then** the button displays the active/pressed style (e.g., darker background color)
3. **Given** a button with `<disabled>` style and `enabled="{false}"` attribute, **When** the button renders, **Then** the button displays disabled styling (e.g., reduced opacity) and does not respond to clicks
4. **Given** a button with only `<base>` style (no state variants), **When** user interacts with the button, **Then** the button uses Iced's default state styling
5. **Given** a button with state styles applied, **When** XML file is modified and hot-reloaded, **Then** the new state styles take effect immediately without losing interaction state

---

### User Story 2 - Text Input State Styling (Priority: P2)

As a UI developer, I need text input fields to show visual feedback for focus and hover states so that users can clearly see which input is active and ready for text entry.

**Why this priority**: Text inputs are the second most common interactive element in forms. Focus indication is critical for accessibility and user experience, but buttons are more universally used, so this is P2.

**Independent Test**: Can be tested by creating a text input with focus/hover styles, tabbing through inputs or clicking them, and verifying the visual focus ring and hover effects appear correctly.

**Acceptance Scenarios**:

1. **Given** a text input with `<focus>` style defined, **When** user clicks or tabs to the input, **Then** the input displays the focus style (e.g., blue border)
2. **Given** a text input with `<hover>` style defined, **When** user hovers over the input without clicking, **Then** the input displays the hover style
3. **Given** a text input with `<disabled>` style and `disabled="true"`, **When** the input renders, **Then** the input shows disabled styling and cannot be focused or edited
4. **Given** a focused text input with `<focus>` style that includes `is_hovered` context, **When** user hovers over the focused input, **Then** the focus style reflects both focus and hover state appropriately

---

### User Story 3 - Checkbox/Radio/Toggler State Styling (Priority: P2)

As a UI developer, I need selection widgets (checkbox, radio, toggler) to display appropriate state styling so that users can distinguish between selected/unselected states and see hover feedback.

**Why this priority**: Selection widgets are common in forms and settings panels. They have unique requirements (checked vs unchecked context) but are less critical than buttons and text inputs.

**Independent Test**: Can be tested by creating checkboxes/radios with hover and disabled styles, interacting with them, and verifying state changes are visible in both checked and unchecked states.

**Acceptance Scenarios**:

1. **Given** a checkbox with `<hover>` style, **When** user hovers over the checkbox, **Then** the checkbox displays hover styling regardless of its checked state
2. **Given** a radio button with `<hover>` style, **When** user hovers over the radio, **Then** the radio displays hover styling regardless of its selected state
3. **Given** a toggler with `<active>` style, **When** the toggler is in "on" state, **Then** the toggler applies active styling to indicate it's toggled on
4. **Given** a disabled checkbox with `<disabled>` style, **When** the checkbox renders, **Then** it shows disabled styling and cannot be toggled

---

### User Story 4 - Container Hover Styling (Priority: P3)

As a UI developer, I need containers (like cards) to respond to hover interactions so that I can create interactive card layouts that feel responsive and clickable.

**Why this priority**: Container hover effects are common in modern UIs (e.g., card grids) but are a polish feature rather than core functionality. Nice to have but not critical for MVP.

**Independent Test**: Can be tested by creating a container with hover style (e.g., shadow or border change), hovering over it, and verifying the visual change occurs.

**Acceptance Scenarios**:

1. **Given** a container with `<hover>` style (e.g., shadow increase), **When** user hovers over the container, **Then** the container displays the hover style
2. **Given** multiple nested containers with hover styles, **When** user hovers over inner container, **Then** only the inner container's hover style applies (no event bubbling issues)

---

### User Story 5 - Advanced Widget State Styling (Priority: P3)

As a UI developer, I need sliders, picklists, and comboboxes to support state styling so that all interactive widgets have consistent visual feedback.

**Why this priority**: These widgets are less commonly used than buttons and inputs, and have more complex interaction states (dragging, dropdown open/closed). Can be implemented after core widgets are stable.

**Independent Test**: Can be tested independently by creating each widget type with state styles and verifying drag/selection interactions show appropriate visual feedback.

**Acceptance Scenarios**:

1. **Given** a slider with `<hover>` and `<active>` styles, **When** user hovers then drags the slider thumb, **Then** appropriate state styles apply during hover and drag
2. **Given** a picklist with `<focus>` style, **When** user opens the dropdown, **Then** the picklist shows focus styling while dropdown is open
3. **Given** a combobox with `<disabled>` style, **When** the combobox is disabled, **Then** it shows disabled styling and dropdown cannot be opened

---

### Edge Cases

- What happens when a widget has only some state variants defined (e.g., hover but no active)? → System falls back to base style for undefined states
- What happens when state styles conflict with inline styles on the same widget? → Inline styles take precedence over class-based state styles (CSS specificity rules)
- What happens when XML defines combined state syntax (e.g., `<hover:active>`) but Iced only reports single states? → System resolves by priority: Disabled > Active > Hover > Focus > Base
- What happens when a widget changes state very rapidly (e.g., rapid hover on/off)? → Iced's rendering handles state changes at frame rate; styles update on next render cycle
- What happens to state styling during hot-reload? → Base styles update immediately; current interaction state (e.g., if button is being hovered) is preserved by Iced
- What happens when custom opacity is defined in disabled state? → Opacity is applied via color alpha channels in Iced 0.14 (no direct opacity property)
- What happens for widgets with contextual states (e.g., checkbox with `is_checked`)? → State resolution considers both interaction state (hover/disabled) and widget-specific context (checked/unchecked)

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST apply `<hover>` styles from XML to widgets when user hovers mouse cursor over them
- **FR-002**: System MUST apply `<active>` styles from XML to widgets when user presses/interacts with them (e.g., button press, slider drag)
- **FR-003**: System MUST apply `<focus>` styles from XML to text input widgets when they receive keyboard focus
- **FR-004**: System MUST apply `<disabled>` styles from XML to widgets when they are in disabled state (via `enabled="false"` or `disabled="true"` attributes)
- **FR-005**: System MUST fall back to `<base>` style when no state-specific style is defined for the current widget state
- **FR-006**: System MUST support state styling for these P1 widgets: Button, TextInput
- **FR-007**: System MUST support state styling for these P2 widgets: Checkbox, Radio, Toggler
- **FR-008**: System MUST support state styling for these P3 widgets: Slider, Container, PickList, ComboBox
- **FR-009**: System MUST resolve combined state syntax (e.g., `<hover:active>`) by priority order: Disabled > Active > Hover > Focus > Base
- **FR-010**: System MUST NOT require changes to existing user models or application code (zero breaking changes)
- **FR-011**: System MUST preserve interaction states (hover, focus) during hot-reload of XML files
- **FR-012**: System MUST use Iced's native status system (from style closure parameters) rather than external state tracking
- **FR-013**: System MUST map Iced widget-specific status enums to Dampen's generic WidgetState enum
- **FR-014**: System MUST handle contextual states (e.g., `is_checked` for checkbox, `is_hovered` on TextInput focus) when resolving styles
- **FR-015**: System MUST support all existing style properties (background, color, border, shadow, opacity) in state variants
- **FR-016**: Inline styles MUST take precedence over class-based state styles (standard CSS specificity)
- **FR-017**: System MUST allow widgets with no state variants defined to use Iced's default styling behavior

### Key Entities

- **WidgetState**: Represents interaction states (Hover, Focus, Active, Disabled) in a backend-agnostic way. Already exists in IR layer.
- **StateSelector**: Represents single or combined state selectors (e.g., "hover", "hover:active"). Already exists with matching logic.
- **StyleProperties**: Contains visual properties (background, color, border, shadow, etc.) applied to widgets. Already exists.
- **StyleClass**: Contains base style and state_variants HashMap mapping WidgetState to StyleProperties. Already exists with state variants parsed but unused.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Developers can define hover styles in XML and see visual changes when hovering over buttons in under 5 seconds of interaction
- **SC-002**: All P1 widgets (Button, TextInput) support all four state types (hover, focus, active, disabled) with visual feedback
- **SC-003**: State style changes apply within one render frame (16ms at 60fps) of user interaction
- **SC-004**: Zero breaking changes - all existing Dampen applications continue to work without code modifications
- **SC-005**: Hot-reload preserves interaction states - hovering over a button while reloading XML maintains hover state with new styles
- **SC-006**: At least 15 integration tests pass covering all P1/P2 widgets with state styling
- **SC-007**: The styling example application (`examples/styling/`) demonstrates all state variants visually with clear hover/press/disabled feedback
- **SC-008**: Developers can test state styling independently per widget - implementing only button state styling provides a complete, functional feature
- **SC-009**: System processes state style resolution in under 1ms per widget (negligible performance impact)
- **SC-010**: Documentation and examples enable developers to add state styling to new widgets in under 10 minutes

## Assumptions

1. **Iced Status Availability**: We assume Iced 0.14 provides status enums for all targeted widgets in style closures. This has been verified for Button, TextInput, Checkbox, Radio, Toggler, Slider, Container, PickList, ComboBox.

2. **Single State at Once**: We assume Iced returns only one dominant state at a time (e.g., a button cannot be both "Hovered" and "Pressed" simultaneously in the status enum). This means combined states like `<hover:active>` will be resolved by priority rather than true combination.

3. **Default Styling Fallback**: We assume Iced provides reasonable default styling when no Dampen style is specified, so widgets remain usable even if developers don't define state variants.

4. **Hot-Reload Scope**: We assume hot-reload only needs to update style definitions, not interaction state tracking. Iced maintains its own internal state for hover/focus/press, which persists across hot-reloads.

5. **Performance**: We assume state style resolution (HashMap lookup + style property merging) has negligible performance impact compared to Iced's rendering pipeline.

6. **Opacity Implementation**: We assume opacity is handled via color alpha channels in Iced 0.14 (e.g., `rgba(255, 255, 255, 0.5)`) rather than a separate opacity property, based on Iced's API design.

7. **Focus Limitation**: We assume only TextInput, PickList, and ComboBox have meaningful focus states in Iced. Other widgets (Button, Checkbox) do not expose focus in their Status enum, so focus styling for these widgets is not supported in v1.0.

8. **Backward Compatibility Priority**: We assume maintaining zero breaking changes is more important than supporting every possible edge case. Advanced features (e.g., true combined states, custom focus tracking) can be added in future versions.

## Dependencies

- **Iced 0.14+**: Framework must provide status parameters in widget style closures (verified - already in use)
- **Existing IR Layer**: `WidgetState`, `StateSelector`, `StyleClass` with `state_variants` already implemented and tested
- **Existing Parser**: XML parsing for `<hover>`, `<active>`, `<focus>`, `<disabled>` already implemented in theme_parser.rs
- **DampenWidgetBuilder**: Existing builder infrastructure in `dampen-iced/src/builder.rs` must be extended to use status parameter
- **Hot-Reload System**: Existing hot-reload mechanism must continue to work with new state-aware styling

## Out of Scope

- Custom widget state tracking beyond Iced's native status system
- Animation/transition between states (instant state changes only)
- Focus tracking for widgets that don't expose focus in Iced Status enum (e.g., Button, Checkbox)
- True simultaneous combined states (e.g., hover AND active at same time) - priority-based resolution only
- Custom pseudo-classes beyond the four core states (hover, focus, active, disabled)
- State styling for widgets not in P1/P2/P3 list (Canvas, Grid, Tooltip, etc. - can be added later)
- Performance optimization beyond basic HashMap lookup (e.g., caching, memoization)
- Visual transition effects or animations when states change
- External state tracking using `#[ui_state]` macro or WidgetStateManager (simplified approach uses Iced's status only)

## Security & Privacy Considerations

- **No Security Impact**: This feature only affects visual styling based on user interaction states. No data storage, transmission, or authentication involved.
- **No Privacy Impact**: No user data is collected, stored, or processed. State changes are purely visual and ephemeral.
- **No XSS Risk**: Styles are defined in XML and parsed at compile-time (in production mode) or at parse-time (in interpreted mode), not from user input at runtime.

## Prior Art & References

- **CSS Pseudo-Classes**: The `:hover`, `:focus`, `:active`, `:disabled` syntax in Dampen XML mirrors CSS pseudo-classes for familiarity
- **Iced Examples**: Iced's button examples demonstrate status-based styling patterns that Dampen will adapt
- **SwiftUI State Modifiers**: SwiftUI's `.buttonStyle()` with state parameter inspired the state-aware styling approach
- **Existing Dampen Code**: `examples/styling/src/ui/window.dampen` already defines state variants (lines 55-66) that currently don't work - this feature makes them functional
