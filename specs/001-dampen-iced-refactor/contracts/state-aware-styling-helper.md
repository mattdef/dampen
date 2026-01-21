# Contract: State-Aware Styling Helper

**Function**: `create_state_aware_style_fn`  
**Location**: `crates/dampen-iced/src/builder/helpers.rs`  
**Purpose**: Generic helper that creates state-aware styling closures for widgets

---

## Function Signature

```rust
/// Creates a state-aware styling closure for widgets that support status-based theming.
///
/// This helper eliminates ~60-70 lines of duplicated code per widget by extracting
/// the common pattern of:
/// 1. Mapping widget-specific Status → unified WidgetState
/// 2. Resolving state-specific styles from StyleClass (hover, focus, active, disabled)
/// 3. Merging state styles with base styles
/// 4. Applying final styles to widget-specific Style struct
///
/// # Type Parameters
///
/// * `Status` - Iced widget status enum (e.g., `checkbox::Status`, `radio::Status`)
/// * `Style` - Iced widget style struct (e.g., `checkbox::Style`, `radio::Style`)
/// * `F` - Status mapper function type (Status → Option<WidgetState>)
/// * `G` - Style applier function type (StyleProperties → Style)
///
/// # Arguments
///
/// * `base_style_props` - Base styles to apply in all states
/// * `style_class` - Optional class with state-specific style variants (hover, focus, etc.)
/// * `status_mapper` - Widget-specific function that maps Status to WidgetState
/// * `style_applier` - Widget-specific function that creates Style from StyleProperties
///
/// # Returns
///
/// A closure `Fn(&Theme, Status) -> Style` suitable for passing to `widget.style()`.
///
/// # Example
///
/// ```rust
/// use crate::builder::helpers::create_state_aware_style_fn;
/// use crate::style_mapping::map_checkbox_status;
///
/// let style_fn = create_state_aware_style_fn(
///     base_style_props,
///     style_class,
///     map_checkbox_status,  // Widget-specific mapper
///     apply_checkbox_style, // Widget-specific applier
/// );
///
/// let checkbox = checkbox.style(style_fn);
/// ```
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
{
    move |_theme: &Theme, status: Status| {
        use crate::style_mapping::{merge_style_properties, resolve_state_style};

        // Map widget-specific status to unified WidgetState
        let widget_state = status_mapper(status);

        // Resolve state-specific style if available
        let final_style_props =
            if let (Some(ref class), Some(state)) = (&style_class, widget_state) {
                // Try to get state-specific style from class
                if let Some(state_style) = resolve_state_style(class, state) {
                    // Merge state style with base style (state overrides base)
                    merge_style_properties(&base_style_props, state_style)
                } else {
                    // No state variant defined in class, use base style
                    base_style_props.clone()
                }
            } else {
                // No style class or no state mapping, use base style
                base_style_props.clone()
            };

        // Apply final resolved style to widget-specific Style struct
        style_applier(&final_style_props)
    }
}
```

---

## Behavior Specification

### Input Validation

| Parameter | Validation | Behavior if Invalid |
|-----------|------------|---------------------|
| `base_style_props` | Must be valid StyleProperties | Compile error if wrong type |
| `style_class` | Can be None | If None, always use base styles |
| `status_mapper` | Must map Status → Option<WidgetState> | If returns None, use base styles |
| `style_applier` | Must return widget-specific Style | Compile error if wrong type |

### Processing Flow

1. **Closure Creation** (at widget build time):
   - Move `base_style_props` and `style_class` into closure
   - Return closure that captures these values

2. **Closure Execution** (at render time, per frame):
   - Receive current widget `status` from Iced
   - Call `status_mapper(status)` to get `Option<WidgetState>`
   - If `style_class.is_some()` AND `widget_state.is_some()`:
     - Call `resolve_state_style(class, state)` to get state-specific styles
     - If state variant exists: merge it with base styles using `merge_style_properties()`
     - If no state variant: use base styles unchanged
   - Else: use base styles unchanged
   - Call `style_applier(final_style_props)` to create widget Style struct
   - Return Style to Iced for rendering

### Error Handling

**No errors** - This function is designed for graceful degradation:
- Missing style class → use base styles
- Unmappable status → use base styles
- Missing state variant → use base styles
- Invalid style properties → widget applies defaults

### Performance Characteristics

- **Memory**: Clones `base_style_props` and `style_class` once per widget (not per frame)
- **Time**: O(1) lookup for state variant, O(n) merge where n = number of style properties
- **Optimization**: Future versions may use `Rc<StyleClass>` to avoid clone

---

## Contract Guarantees

### Preconditions

- Widget type must have `.style()` method accepting `Fn(&Theme, Status) -> Style`
- `status_mapper` must be deterministic (same Status → same WidgetState)
- `style_applier` must be pure (same StyleProperties → same Style)

### Postconditions

- Returned closure is valid for widget's `.style()` method
- Closure correctly handles all widget status states
- State-specific styles override base styles when available
- Base styles are preserved when no state variant exists

### Invariants

- Base styles are never modified
- State resolution is consistent (same state always resolves to same styles)
- Inline styles take precedence over class styles (enforced by merge_style_properties)

---

## Test Cases

### TC1: Widget with Base Styles Only

**Input**:
- `base_style_props`: Background = red, Color = white
- `style_class`: None
- Widget status changes: Active → Hovered → Active

**Expected**:
- All states render with red background, white color (no state variants)

---

### TC2: Widget with Hover State

**Input**:
- `base_style_props`: Background = blue
- `style_class`: Contains hover variant (background = light_blue)
- Widget status: Active → Hovered

**Expected**:
- Active status: blue background
- Hovered status: light_blue background

---

### TC3: Widget with Focus State

**Input**:
- `base_style_props`: Border = 1px solid gray
- `style_class`: Contains focus variant (border = 2px solid blue)
- Widget status: Active → Focused

**Expected**:
- Active: 1px gray border
- Focused: 2px blue border

---

### TC4: Widget with Disabled State

**Input**:
- `base_style_props`: Color = black
- `style_class`: Contains disabled variant (color = gray, background = light_gray)
- Widget status: Active → Disabled

**Expected**:
- Active: black color, no background
- Disabled: gray color, light_gray background

---

### TC5: Widget with Class + Inline Styles

**Input**:
- `base_style_props`: Includes inline background = yellow
- `style_class`: Contains hover variant (background = orange)
- Widget status: Hovered

**Expected**:
- Inline style overrides class style (CSS precedence rules)
- Result: yellow background (inline wins)

---

### TC6: Widget with Missing State Variant

**Input**:
- `base_style_props`: Background = white
- `style_class`: Has focus variant but NO hover variant
- Widget status: Hovered

**Expected**:
- Fallback to base styles (white background)
- No error, graceful degradation

---

### TC7: Status Mapper Returns None

**Input**:
- `status_mapper` returns None for a particular Status value
- `style_class`: Has all state variants defined

**Expected**:
- Use base styles (no state resolution attempted)

---

## Widget-Specific Style Appliers

Each widget needs a companion `apply_*_style` function:

### Checkbox Example

```rust
fn apply_checkbox_style(props: &StyleProperties) -> checkbox::Style {
    use iced::{Background, Border, Color};
    
    let mut style = checkbox::Style {
        background: Background::Color(Color::WHITE),
        icon_color: Color::BLACK,
        border: Border::default(),
        text_color: None,
    };
    
    // Apply background
    if let Some(ref bg) = props.background {
        if let dampen_core::ir::style::Background::Color(color) = bg {
            style.background = Background::Color(Color {
                r: color.r,
                g: color.g,
                b: color.b,
                a: color.a,
            });
        }
    }
    
    // Apply color (maps to icon_color for checkbox)
    if let Some(color) = props.color {
        style.icon_color = Color {
            r: color.r,
            g: color.g,
            b: color.b,
            a: color.a,
        };
    }
    
    // Apply border
    if let Some(ref border_props) = props.border {
        style.border = Border {
            color: Color {
                r: border_props.color.r,
                g: border_props.color.g,
                b: border_props.color.b,
                a: border_props.color.a,
            },
            width: border_props.width,
            radius: border_props.radius.into(),
        };
    }
    
    // Apply text_color
    if let Some(text_color) = props.text_color {
        style.text_color = Some(Color {
            r: text_color.r,
            g: text_color.g,
            b: text_color.b,
            a: text_color.a,
        });
    }
    
    style
}
```

---

## Migration Guide

### Before (70 lines per widget)

```rust
// checkbox.rs - OLD APPROACH
if let Some(base_style_props) = resolved_base_style {
    let base_style_props = base_style_props.clone();
    let style_class = style_class.cloned();

    checkbox = checkbox.style(move |_theme, status| {
        use crate::style_mapping::{
            map_checkbox_status, merge_style_properties, resolve_state_style,
        };
        use iced::widget::checkbox;
        use iced::{Background, Border, Color};

        let widget_state = map_checkbox_status(status);
        
        let final_style_props =
            if let (Some(class), Some(state)) = (&style_class, widget_state) {
                if let Some(state_style) = resolve_state_style(class, state) {
                    merge_style_properties(&base_style_props, state_style)
                } else {
                    base_style_props.clone()
                }
            } else {
                base_style_props.clone()
            };

        let mut style = checkbox::Style {
            background: Background::Color(Color::WHITE),
            icon_color: Color::BLACK,
            border: Border::default(),
            text_color: None,
        };

        // ... 40+ more lines applying properties
        
        style
    });
}
```

### After (< 10 lines per widget)

```rust
// checkbox.rs - NEW APPROACH
use crate::builder::helpers::create_state_aware_style_fn;
use crate::style_mapping::map_checkbox_status;

if let Some(base_style_props) = resolved_base_style {
    let style_fn = create_state_aware_style_fn(
        base_style_props,
        style_class,
        map_checkbox_status,
        apply_checkbox_style,  // Separate function (20-30 lines, reusable)
    );
    
    checkbox = checkbox.style(style_fn);
}
```

**Lines Saved**: ~60 lines per widget × 7 widgets = **~420 lines total**

---

## Dependencies

### Required Imports

```rust
use crate::style_mapping::{merge_style_properties, resolve_state_style, WidgetState};
use crate::ir::style::StyleProperties;
use crate::ir::style::StyleClass;
use iced::Theme;
```

### Related Functions

- `resolve_state_style(class: &StyleClass, state: WidgetState) -> Option<&StyleProperties>`
- `merge_style_properties(base: &StyleProperties, override: &StyleProperties) -> StyleProperties`
- Widget-specific status mappers: `map_checkbox_status`, `map_radio_status`, etc.

---

## Version History

- **v0.1.0** (2026-01-21): Initial contract definition
