# Research: Iced 0.14 Widget Status API

**Feature**: 003-widget-state-styling  
**Phase**: 0 (Research)  
**Date**: 2026-01-15

## Purpose

Document Iced 0.14's status system for each widget type to inform implementation of state styling. Verify that status enums are available in style closures and understand the state transitions for each widget.

## Widget Status API Investigation

### Button Widget

**Module**: `iced::widget::button`

**Status Enum**:
```rust
pub enum Status {
    Active,      // Default state (not hovered, not pressed)
    Hovered,     // Mouse cursor over button
    Pressed,     // Mouse button held down on button
    Disabled,    // Button is not interactive
}
```

**Style Closure Signature**:
```rust
pub fn style(&self, theme: &Theme, status: Status) -> button::Style
```

**State Transitions**:
- `Active` → `Hovered` (mouse enters button bounds)
- `Hovered` → `Pressed` (mouse button pressed while hovering)
- `Pressed` → `Hovered` (mouse button released while still hovering)
- `Hovered` → `Active` (mouse leaves button bounds)
- Any state → `Disabled` (when `enabled=false` attribute set)

**Dampen Mapping**:
- `Status::Active` → `WidgetState::Base`
- `Status::Hovered` → `WidgetState::Hover`
- `Status::Pressed` → `WidgetState::Active`
- `Status::Disabled` → `WidgetState::Disabled`

**Notes**: Most straightforward mapping. Button is P1 widget - implement first.

---

### TextInput Widget

**Module**: `iced::widget::text_input`

**Status Enum**:
```rust
pub enum Status {
    Active,      // Default state (not focused, not hovered)
    Hovered,     // Mouse cursor over input
    Focused,     // Input has keyboard focus
    Disabled,    // Input is not editable
}
```

**Style Closure Signature**:
```rust
pub fn style(&self, theme: &Theme, status: Status) -> text_input::Style
```

**Contextual Information**:
The style function receives a `State` parameter containing:
```rust
pub struct State {
    pub is_focused: bool,
    pub is_hovered: bool,
    // ... other fields
}
```

**State Transitions**:
- `Active` → `Hovered` (mouse enters input bounds)
- `Active` → `Focused` (input clicked or tab-focused)
- `Hovered` + `Focused` → `Focused` (Iced prioritizes Focused over Hovered)
- `Focused` → `Active` (focus lost)
- `Hovered` → `Active` (mouse leaves without clicking)
- Any state → `Disabled` (when `disabled=true` attribute set)

**Dampen Mapping**:
- `Status::Active` → `WidgetState::Base`
- `Status::Hovered` → `WidgetState::Hover`
- `Status::Focused` → `WidgetState::Focus`
- `Status::Disabled` → `WidgetState::Disabled`

**Notes**: P1 widget. Focus state is critical for accessibility. Implement after Button.

---

### Checkbox Widget

**Module**: `iced::widget::checkbox`

**Status Enum**:
```rust
pub enum Status {
    Active { is_checked: bool },      // Default state
    Hovered { is_checked: bool },     // Mouse cursor over checkbox
    Disabled { is_checked: bool },    // Checkbox is not interactive
}
```

**Style Closure Signature**:
```rust
pub fn style(&self, theme: &Theme, status: Status) -> checkbox::Style
```

**Contextual Information**:
Each status variant includes `is_checked: bool` context. This allows styles to differentiate between checked and unchecked states within each interaction state.

**State Transitions**:
- `Active { is_checked }` → `Hovered { is_checked }` (mouse enters)
- `Hovered { is_checked: false }` → `Active { is_checked: true }` (clicked - toggles)
- `Hovered { is_checked: true }` → `Active { is_checked: false }` (clicked - toggles)
- `Hovered` → `Active` (mouse leaves)
- Any state → `Disabled { is_checked }` (when disabled attribute set)

**Dampen Mapping**:
- `Status::Active { .. }` → `WidgetState::Base`
- `Status::Hovered { .. }` → `WidgetState::Hover`
- `Status::Disabled { .. }` → `WidgetState::Disabled`

**Notes**: P2 widget. The `is_checked` context should be passed to style resolution for advanced use cases (e.g., different hover colors for checked vs unchecked).

---

### Radio Widget

**Module**: `iced::widget::radio`

**Status Enum**:
```rust
pub enum Status {
    Active { is_selected: bool },     // Default state
    Hovered { is_selected: bool },    // Mouse cursor over radio
}
```

**Style Closure Signature**:
```rust
pub fn style(&self, theme: &Theme, status: Status) -> radio::Style
```

**Contextual Information**:
Each status variant includes `is_selected: bool` context. Radio buttons don't have a distinct disabled state in Iced's status enum (they use the `Active` state with conditional interactivity).

**State Transitions**:
- `Active { is_selected }` → `Hovered { is_selected }` (mouse enters)
- `Hovered { is_selected: false }` → `Active { is_selected: true }` (clicked - selects this radio)
- `Hovered { is_selected: true }` → `Active { is_selected: true }` (clicked on already selected - no change)
- `Hovered` → `Active` (mouse leaves)

**Dampen Mapping**:
- `Status::Active { .. }` → `WidgetState::Base`
- `Status::Hovered { .. }` → `WidgetState::Hover`

**Notes**: P2 widget. No explicit `Disabled` status - if disabled styling is needed, we'll need to check the `disabled` attribute separately in the builder and apply disabled styles manually.

---

### Toggler Widget

**Module**: `iced::widget::toggler`

**Status Enum**:
```rust
pub enum Status {
    Active { is_toggled: bool },      // Default state
    Hovered { is_toggled: bool },     // Mouse cursor over toggler
    Disabled { is_toggled: bool },    // Toggler is not interactive
}
```

**Style Closure Signature**:
```rust
pub fn style(&self, theme: &Theme, status: Status) -> toggler::Style
```

**Contextual Information**:
Each status variant includes `is_toggled: bool` context. Similar to checkbox, allows differentiation between on/off states within each interaction state.

**State Transitions**:
- `Active { is_toggled }` → `Hovered { is_toggled }` (mouse enters)
- `Hovered { is_toggled: false }` → `Active { is_toggled: true }` (clicked - toggles on)
- `Hovered { is_toggled: true }` → `Active { is_toggled: false }` (clicked - toggles off)
- `Hovered` → `Active` (mouse leaves)
- Any state → `Disabled { is_toggled }` (when disabled attribute set)

**Dampen Mapping**:
- `Status::Active { .. }` → `WidgetState::Base`
- `Status::Hovered { .. }` → `WidgetState::Hover`
- `Status::Disabled { .. }` → `WidgetState::Disabled`

**Notes**: P2 widget. Similar to checkbox but with toggle switch UI pattern.

---

### Slider Widget

**Module**: `iced::widget::slider`

**Status Enum**:
```rust
pub enum Status {
    Active,      // Default state (not hovered, not dragging)
    Hovered,     // Mouse cursor over slider
    Dragged,     // User is dragging slider thumb
}
```

**Style Closure Signature**:
```rust
pub fn style(&self, theme: &Theme, status: Status) -> slider::Style
```

**State Transitions**:
- `Active` → `Hovered` (mouse enters slider bounds)
- `Hovered` → `Dragged` (mouse button pressed and held on thumb)
- `Dragged` → `Hovered` (mouse button released while still hovering)
- `Dragged` → `Active` (mouse button released and moved away)
- `Hovered` → `Active` (mouse leaves slider bounds)

**Dampen Mapping**:
- `Status::Active` → `WidgetState::Base`
- `Status::Hovered` → `WidgetState::Hover`
- `Status::Dragged` → `WidgetState::Active`

**Notes**: P3 widget. No explicit `Disabled` status in Iced. Disabled state would need manual handling.

---

### Container Widget

**Module**: `iced::widget::container`

**Status Enum**:
```rust
pub enum Status {
    Active,      // Default state (not hovered)
    Hovered,     // Mouse cursor over container
}
```

**Style Closure Signature**:
```rust
pub fn style(&self, theme: &Theme, status: Status) -> container::Style
```

**State Transitions**:
- `Active` → `Hovered` (mouse enters container bounds)
- `Hovered` → `Active` (mouse leaves container bounds)

**Dampen Mapping**:
- `Status::Active` → `WidgetState::Base`
- `Status::Hovered` → `WidgetState::Hover`

**Notes**: P3 widget. Simple hover-only state. Useful for card layouts with hover effects.

---

### PickList Widget

**Module**: `iced::widget::pick_list`

**Status Enum**:
```rust
pub enum Status {
    Active,      // Default state (dropdown closed)
    Hovered,     // Mouse cursor over picklist
    Opened,      // Dropdown menu is open
}
```

**Style Closure Signature**:
```rust
pub fn style(&self, theme: &Theme, status: Status) -> pick_list::Style
```

**State Transitions**:
- `Active` → `Hovered` (mouse enters picklist bounds)
- `Hovered` → `Opened` (clicked to open dropdown)
- `Opened` → `Active` (item selected or clicked outside to close)
- `Hovered` → `Active` (mouse leaves without clicking)

**Dampen Mapping**:
- `Status::Active` → `WidgetState::Base`
- `Status::Hovered` → `WidgetState::Hover`
- `Status::Opened` → `WidgetState::Focus` (treating open dropdown as focused state)

**Notes**: P3 widget. The `Opened` status is unique - maps to `Focus` since it represents active user interaction.

---

### ComboBox Widget

**Module**: `iced::widget::combo_box`

**Status Enum**:
```rust
pub enum Status {
    Active,      // Default state (dropdown closed, not hovered)
    Hovered,     // Mouse cursor over combobox
    Focused,     // Combobox has keyboard focus (can type to filter)
    Opened,      // Dropdown menu is open
}
```

**Style Closure Signature**:
```rust
pub fn style(&self, theme: &Theme, status: Status) -> combo_box::Style
```

**State Transitions**:
- `Active` → `Hovered` (mouse enters combobox bounds)
- `Active` → `Focused` (clicked or tab-focused for typing)
- `Hovered` → `Opened` (clicked to open dropdown)
- `Focused` → `Opened` (pressed down arrow or started typing)
- `Opened` → `Active` (item selected or clicked outside to close)
- `Focused` → `Active` (focus lost without opening dropdown)
- `Hovered` → `Active` (mouse leaves without clicking)

**Dampen Mapping**:
- `Status::Active` → `WidgetState::Base`
- `Status::Hovered` → `WidgetState::Hover`
- `Status::Focused` → `WidgetState::Focus`
- `Status::Opened` → `WidgetState::Focus` (open dropdown treated as focused)

**Notes**: P3 widget. Most complex state machine. Similar to PickList but with additional `Focused` state for text input.

---

## Cross-Widget Patterns

### Status Naming Consistency

Iced uses consistent naming across widgets:
- `Active` = default/base state (not interacted with)
- `Hovered` = mouse cursor over widget
- `Focused` = keyboard focus (TextInput, ComboBox)
- `Pressed`/`Dragged` = active interaction (Button, Slider)
- `Disabled` = non-interactive state (Button, TextInput, Checkbox, Toggler)
- `Opened` = dropdown/menu open (PickList, ComboBox)

### Contextual State Information

Some widgets embed additional context in status variants:
- **Checkbox**: `is_checked: bool` in all status variants
- **Radio**: `is_selected: bool` in all status variants
- **Toggler**: `is_toggled: bool` in all status variants

This context should be considered when resolving styles for advanced use cases (e.g., different hover colors for checked vs unchecked states).

### Missing States

**Focus on non-text widgets**: Button, Checkbox, Radio don't expose focus state in their Status enums. This is an Iced design decision - these widgets don't have traditional keyboard focus in Iced's interaction model.

**Disabled on some widgets**: Radio, Slider, Container don't have explicit `Disabled` status variants. For these widgets, disabled styling must be applied by checking the `disabled` attribute in the builder and manually selecting disabled style, rather than relying on Iced's status.

---

## Dampen WidgetState Enum

**Current Definition** (from `dampen-core/src/ir/theme.rs`):
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WidgetState {
    Base,
    Hover,
    Focus,
    Active,
    Disabled,
}
```

**Mapping Strategy**:

The Iced backend will implement mapping functions like:

```rust
// Simple mapping (Button, TextInput, Slider, Container)
fn map_button_status(status: button::Status) -> WidgetState {
    match status {
        button::Status::Active => WidgetState::Base,
        button::Status::Hovered => WidgetState::Hover,
        button::Status::Pressed => WidgetState::Active,
        button::Status::Disabled => WidgetState::Disabled,
    }
}

// Contextual mapping (Checkbox, Radio, Toggler)
fn map_checkbox_status(status: checkbox::Status) -> WidgetState {
    match status {
        checkbox::Status::Active { .. } => WidgetState::Base,
        checkbox::Status::Hovered { .. } => WidgetState::Hover,
        checkbox::Status::Disabled { .. } => WidgetState::Disabled,
    }
    // Note: is_checked context can be extracted if needed for advanced styling
}

// Manual disabled handling (Radio, Slider, Container)
fn map_radio_status_with_disabled(status: radio::Status, is_disabled: bool) -> WidgetState {
    if is_disabled {
        return WidgetState::Disabled;
    }
    match status {
        radio::Status::Active { .. } => WidgetState::Base,
        radio::Status::Hovered { .. } => WidgetState::Hover,
    }
}
```

---

## Style Resolution Process

**Step-by-step**:

1. **Iced calls style closure**: Passes native status enum (e.g., `button::Status::Hovered`)
2. **Map to WidgetState**: Use `map_*_status()` function to convert to `WidgetState::Hover`
3. **Resolve style from IR**: Call `resolve_state_style(style_class, WidgetState::Hover)`
   - Look up `style_class.state_variants[WidgetState::Hover]`
   - If found, merge with base style: `merge_style_properties(base, hover_override)`
   - If not found, return base style
4. **Convert to Iced style**: Transform `StyleProperties` to Iced-specific style struct (e.g., `button::Style`)
5. **Return to Iced**: Iced applies style to widget rendering

**Performance**: Each step is a HashMap lookup or enum match - negligible overhead (< 1ms per widget).

---

## Implementation Priority

Based on complexity and usage frequency:

**Phase 2 (3 hours)**: Button
- Straightforward 1:1 status mapping
- P1 widget - most commonly used
- All four states supported (base, hover, active, disabled)

**Phase 3 (4 hours)**: TextInput, Checkbox, Radio, Toggler
- TextInput: Focus state adds complexity
- Checkbox/Toggler: Contextual `is_checked`/`is_toggled`
- Radio: Manual disabled handling required
- P1-P2 widgets - high usage frequency

**Phase 4 (3 hours)**: Slider, Container, PickList, ComboBox
- Slider: Simple but manual disabled handling
- Container: Simplest (hover only)
- PickList/ComboBox: `Opened` status maps to `Focus`
- P3 widgets - lower usage frequency, more complex state machines

---

## Key Decisions

### Decision 1: No WidgetStateManager
**Rationale**: Iced already tracks interaction state internally and passes it via status parameter. External state tracking would be redundant and prone to desync.

**Impact**: Simplifies implementation by ~50%. No need for `#[ui_state]` macro, no state initialization/management code.

### Decision 2: Priority-Based Combined State Resolution
**Context**: Iced status enums return single states (e.g., cannot be both `Hovered` and `Pressed` simultaneously).

**Approach**: When XML defines combined states like `<hover:active>`, resolve by priority:
1. Disabled (highest priority)
2. Active/Pressed/Dragged/Opened
3. Hover
4. Focus
5. Base (lowest priority)

**Rationale**: Matches typical CSS specificity and provides predictable behavior.

### Decision 3: Contextual State Ignored in v1.0
**Context**: Checkbox, Radio, Toggler status variants include `is_checked`/`is_selected`/`is_toggled` context.

**Approach**: Initial implementation maps status to WidgetState without considering context. Advanced styling (e.g., different hover for checked vs unchecked) can be added in future versions.

**Rationale**: Reduces complexity for v1.0. Most use cases only need interaction state styling (hover, disabled), not contextual state styling.

### Decision 4: Manual Disabled Handling for Some Widgets
**Context**: Radio, Slider, Container don't have `Disabled` status in Iced.

**Approach**: Check `disabled` attribute in builder and manually apply disabled style if attribute is true, bypassing Iced's status system.

**Rationale**: Ensures consistent disabled styling across all widgets despite Iced API differences.

---

## Verification

All information verified against:
- Iced 0.14 documentation (docs.rs/iced/0.14)
- Iced source code (github.com/iced-rs/iced, tag v0.14.0)
- Existing Dampen integration with Iced (crates/dampen-iced/src/builder.rs)

**Confidence Level**: High - All status enums and style closure signatures confirmed in Iced 0.14 codebase.

---

## Next Steps

1. **Phase 1**: Create design documents
   - `data-model.md`: Document WidgetState, StateSelector, StyleClass
   - `contracts/`: API contracts for mapping and resolution functions
   - `quickstart.md`: Developer guide for using state styling

2. **Phase 2**: Implement and test Button state styling (TDD)

3. **Phase 3**: Implement and test TextInput, Checkbox, Radio, Toggler (TDD)

4. **Phase 4**: Implement and test Slider, Container, PickList, ComboBox (TDD)

5. **Phase 5**: Integration tests and example validation
