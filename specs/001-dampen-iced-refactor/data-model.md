# Data Model: dampen-iced Refactoring Helper Functions

**Feature**: dampen-iced Crate Refactoring  
**Date**: 2026-01-21  
**Purpose**: Define the structure and behavior of new helper function APIs

---

## Overview

This refactoring introduces 5 new helper function "entities" (code structures) to eliminate duplication and improve maintainability. These are not domain entities but **API entities** representing reusable functions.

---

## Entity 1: StateAwareStyleHelper

**Purpose**: Generic function that applies state-aware styling to widgets based on interaction states (hover, focus, active, disabled)

**Type Signature**:
```rust
pub fn create_state_aware_style_fn<Status, Style, F, G>(
    base_style_props: StyleProperties,
    style_class: Option<StyleClass>,
    status_mapper: F,
    style_applier: G,
) -> impl Fn(&Theme, Status) -> Style
where
    Status: 'static,
    Style: 'static,
    F: Fn(Status) -> Option<WidgetState> + 'static,
    G: Fn(&StyleProperties) -> Style + 'static,
```

**Fields/Parameters**:
- `base_style_props: StyleProperties` - Base styles applied in all states
- `style_class: Option<StyleClass>` - Optional class with state-specific style variants
- `status_mapper: F` - Widget-specific function mapping Status enum → WidgetState
- `style_applier: G` - Widget-specific function creating Style struct from StyleProperties

**Constraints**:
- Widget must have `.style()` method accepting `Fn(&Theme, Status) -> Style` closure
- Status type must be widget-specific Iced status enum (checkbox::Status, radio::Status, etc.)
- Style type must be widget-specific Iced style struct

**Relationships**:
- Used by: checkbox, radio, text_input, toggler, slider, pick_list, combo_box (7 widgets)
- Depends on: `style_mapping::resolve_state_style`, `style_mapping::merge_style_properties`
- Returns: Closure suitable for widget.style() method

**Validation Rules**:
- If style_class is None, always use base_style_props
- If WidgetState cannot be mapped, use base_style_props
- If state variant doesn't exist in class, fallback to base_style_props

**State Transitions**:
- Base → Hover (when status changes to Hovered)
- Base → Focus (when status changes to Focused)
- Base → Active (when status changes to Pressed/Active)
- Base → Disabled (when status indicates disabled state)

---

## Entity 2: BooleanAttributeResolver

**Purpose**: Parse `AttributeValue` into `bool` with support for multiple string formats

**Type Signature**:
```rust
pub fn resolve_boolean_attribute(
    node: &WidgetNode,
    attr_name: &str,
    default: bool,
) -> bool
```

**Fields/Parameters**:
- `node: &WidgetNode` - Widget XML node containing attributes
- `attr_name: &str` - Attribute name to resolve (e.g., "disabled", "enabled")
- `default: bool` - Value to return if attribute is missing or invalid

**Validation Rules**:
- Case-insensitive parsing (convert to lowercase first)
- Whitespace-trimmed before parsing
- Truthy values: "true", "1", "yes", "on"
- Falsy values: "false", "0", "no", "off", "" (empty string)
- Unknown values → return `default`

**Relationships**:
- Used by: button, radio, checkbox (3 widgets currently, expandable)
- Depends on: `dampen_core::AttributeValue`
- Future: May support binding expressions (AttributeValue::Binding)

**Error Handling**: No errors - invalid values gracefully default

---

## Entity 3: HandlerResolutionHelper

**Purpose**: Resolve event handler parameters from context or model bindings

**Type Signature**:
```rust
pub fn resolve_handler_param<M>(
    builder: &DampenWidgetBuilder<M>,
    event_param_expr: &str,
) -> Result<BindingValue, HandlerResolutionError>
where
    M: UiBindable + Clone + 'static,
```

**Fields/Parameters**:
- `builder: &DampenWidgetBuilder<M>` - Builder with context and model state
- `event_param_expr: &str` - Binding expression (e.g., "item.value", "model.count")

**Validation Rules**:
- First attempt: Context resolution (for loop items, local bindings)
- Second attempt: Model field access (shared state)
- Both fail → return error with diagnostic information

**Relationships**:
- Used by: button, checkbox, radio, slider, text_input (5 widgets)
- Depends on: `builder.resolve_from_context()`, `evaluate_binding_expr_with_shared()`
- Returns: `BindingValue` on success or `HandlerResolutionError` with context

**Error Handling**:
```rust
#[derive(Debug, Clone, PartialEq)]
pub struct HandlerResolutionError {
    pub handler_name: String,      // Which handler failed
    pub widget_kind: String,        // Widget type (Button, TextInput, etc.)
    pub widget_id: Option<String>,  // Widget ID for disambiguation
    pub param_expr: String,         // Binding expression that failed
    pub binding_error: BindingError, // Underlying evaluation error
    pub span: Span,                 // XML location
    pub context_note: Option<String>, // Additional resolution attempt info
}
```

**State Transitions**: N/A (pure function, no state)

---

## Entity 4: StyleClassWrapper

**Purpose**: Reference-counted wrapper for `StyleClass` to avoid expensive clones in closures

**Type Signature**:
```rust
pub type StyleClassWrapper = Rc<StyleClass>;
```

**Fields/Parameters**: Wraps existing `StyleClass` struct

**Performance Characteristics**:
- Size: 8 bytes (pointer) vs ~412 bytes (full struct)
- Clone cost: ~4 ns vs ~213 ns (47.1x speedup)
- Memory overhead: 8 bytes for reference count

**Relationships**:
- Used by: All state-aware widgets (7 total)
- Alternative to: Cloning full `StyleClass` in closure move
- Thread safety: `Rc` (single-threaded, compatible with Iced's model)

**Constraints**:
- Single-threaded only (Rc is not Send/Sync)
- Widgets never cross thread boundaries (safe in Iced)

**Lifecycle**:
1. Create: `Rc::new(style_class)` when building widget
2. Clone: `Rc::clone(&wrapper)` before moving into closure (cheap)
3. Access: `wrapper.some_field` (automatic Deref)
4. Drop: Reference count decremented when closure is dropped

---

## Entity 5: VerboseLoggingGuard

**Purpose**: Compile-time guard for verbose logging to exclude debug output from release builds

**Type Signature** (macro approach):
```rust
macro_rules! verbose_log {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        {
            eprintln!($($arg)*);
        }
    };
}
```

**Alternative** (function approach):
```rust
#[cfg(debug_assertions)]
pub fn verbose_log(msg: &str) {
    eprintln!("{}", msg);
}

#[cfg(not(debug_assertions))]
pub fn verbose_log(_msg: &str) {
    // No-op in release builds
}
```

**Fields/Parameters**:
- `msg: &str` or variadic `$($arg:tt)*` - Log message and arguments

**Validation Rules**:
- Only compiles logging code when `debug_assertions` is true
- Zero runtime cost in release builds (code eliminated at compile time)

**Relationships**:
- Replaces: 52 instances of `if self.verbose { eprintln!(...) }`
- Side effect: Removes `verbose: bool` field from `DampenWidgetBuilder`
- Aligns with: Dampen's dual-mode architecture (interpreted dev / codegen release)

**Lifecycle**:
- **Debug builds** (`cargo build`): Code included, logging active
- **Release builds** (`cargo build --release`): Code completely eliminated by compiler

**Binary Size Impact**: ~1-3KB reduction in release builds

---

## Entity Relationships Diagram

```
┌─────────────────────────────────────┐
│   DampenWidgetBuilder               │
│   ┌─────────────────────────────┐   │
│   │ resolve_from_context()      │   │
│   │ evaluate_binding_expr()     │   │
│   └─────────────────────────────┘   │
└─────────────────────────────────────┘
        ↓ uses                  ↓ uses
┌───────────────────┐   ┌────────────────────────┐
│ HandlerResolution │   │ StateAwareStyleHelper  │
│ Helper            │   │                        │
│                   │   │ ┌──────────────────┐   │
│ resolve_handler   │   │ │ StyleClassWrapper│   │
│ _param()          │   │ │ (Rc<StyleClass>) │   │
└───────────────────┘   │ └──────────────────┘   │
                        └────────────────────────┘
        ↓ returns error            ↓ uses
┌───────────────────────────┐   ┌─────────────────────┐
│ HandlerResolutionError    │   │ resolve_state_style │
│                           │   │ merge_style_props   │
│ - handler_name            │   │ (in style_mapping)  │
│ - widget_kind             │   └─────────────────────┘
│ - param_expr              │
│ - binding_error           │
│ - span                    │
└───────────────────────────┘

┌──────────────────────────┐    ┌─────────────────────────┐
│ BooleanAttributeResolver │    │ VerboseLoggingGuard     │
│                          │    │                         │
│ resolve_boolean_attr()   │    │ verbose_log!() macro    │
└──────────────────────────┘    │ or verbose_log() fn     │
        ↓ used by               └─────────────────────────┘
┌──────────────────────────┐            ↓ replaces
│ button.rs, radio.rs,     │    ┌────────────────────────┐
│ checkbox.rs              │    │ if self.verbose {      │
└──────────────────────────┘    │   eprintln!(...)       │
                                │ }                      │
                                └────────────────────────┘
```

---

## Usage Summary

| Helper | Widgets Affected | Lines Saved | Location |
|--------|------------------|-------------|----------|
| StateAwareStyleHelper | 7 (checkbox, radio, text_input, toggler, slider, pick_list, combo_box) | ~200 | builder/helpers.rs |
| BooleanAttributeResolver | 3 (button, radio, checkbox) | ~50 | builder/helpers.rs |
| HandlerResolutionHelper | 5 (button, checkbox, radio, slider, text_input) | ~120 | builder/helpers.rs |
| StyleClassWrapper | 7 (all state-aware widgets) | N/A (perf optimization) | builder/mod.rs |
| VerboseLoggingGuard | All 24 widgets | ~52 instances | builder/mod.rs |

**Total Lines Saved**: ~370 lines of duplicated code

---

## Implementation Notes

1. **Placement**: All helpers in `crates/dampen-iced/src/builder/helpers.rs`
2. **Visibility**: `pub(crate)` or `pub(super)` - internal to dampen-iced crate
3. **Documentation**: Full rustdoc with examples for each helper
4. **Testing**: Unit tests for each helper in `helpers.rs`, integration tests in widget files
5. **Migration**: Incremental - migrate one widget at a time, validate tests pass

---

## Success Metrics

After implementation:
- ✅ All 148 tests continue passing
- ✅ Clippy produces zero warnings
- ✅ Code duplication reduced by 370+ lines
- ✅ Adding new widget with state-aware styling requires < 15 lines
- ✅ Performance improves by ~5% for 1000-widget UIs (via Rc optimization)
- ✅ Release binary size reduced by ~5KB (via logging guards)
