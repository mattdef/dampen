# Builder API Contract: State-Aware Style Resolution

**Component**: `dampen-iced::builder`  
**Files**: 
- `crates/dampen-iced/src/builder/helpers.rs`
- `crates/dampen-iced/src/builder/mod.rs`
- Widget-specific files (`button.rs`, `text_input.rs`, etc.)

---

## Function: DampenWidgetBuilder::new (Modified)

### Current Signature

```rust
pub fn new(
    document: &'a Document,
    model: &'a M,
    theme_document: Option<&'a ThemeDocument>,
) -> Self
```

### New Signature

```rust
pub fn new(
    document: &'a Document,
    model: &'a M,
    theme_document: Option<&'a ThemeDocument>,
    viewport_width: Option<f32>,  // NEW: For responsive breakpoint resolution
) -> Self
```

### Contract

| Parameter | Type | Description |
|-----------|------|-------------|
| `document` | `&'a Document` | Parsed Dampen document |
| `model` | `&'a M` | Application model for bindings |
| `theme_document` | `Option<&'a ThemeDocument>` | Optional theme definitions |
| `viewport_width` | `Option<f32>` | Viewport width in pixels for breakpoint resolution |

### Breakpoint Resolution

```rust
impl<'a, M> DampenWidgetBuilder<'a, M> {
    fn active_breakpoint(&self) -> Breakpoint {
        self.viewport_width
            .map(Breakpoint::from_viewport_width)
            .unwrap_or(Breakpoint::Desktop)  // Default to desktop if unknown
    }
}
```

| viewport_width | Resolved Breakpoint |
|----------------|---------------------|
| `None` | `Desktop` (default) |
| `Some(320.0)` | `Mobile` |
| `Some(768.0)` | `Tablet` |
| `Some(1920.0)` | `Desktop` |

---

## Function: resolve_complete_styles_with_states (New)

**Location**: `crates/dampen-iced/src/builder/helpers.rs`

### Signature

```rust
pub(super) fn resolve_complete_styles_with_states(
    &self,
    node: &WidgetNode,
) -> (Option<StyleProperties>, HashMap<WidgetState, StyleProperties>)
```

### Contract

**Returns**: Tuple of (base_style, state_variants_map)

**Precedence Layers** (lowest to highest):
1. Theme styles for widget kind
2. Class styles from `node.classes`
3. Class state variants from resolved classes
4. Inline base styles from `node.style`
5. Inline state variants from `node.inline_state_variants`

### Resolution Algorithm

```rust
pub(super) fn resolve_complete_styles_with_states(
    &self,
    node: &WidgetNode,
) -> (Option<StyleProperties>, HashMap<WidgetState, StyleProperties>) {
    // Step 1: Resolve base styles (theme → class → inline)
    let theme_style = self.resolve_theme_styles(node.kind.clone());
    let class_style = self.resolve_class_styles(node);
    let inline_style = node.style.clone();
    
    let base = merge_layers(theme_style, class_style, inline_style);
    
    // Step 2: Resolve state variants for each state
    let mut state_map = HashMap::new();
    
    for state in [Hover, Focus, Active, Disabled] {
        // Start with base
        let mut state_style = base.clone();
        
        // Apply class state variants
        if let Some(class_state) = self.resolve_class_state_variant(node, state) {
            state_style = merge_styles(state_style, class_state);
        }
        
        // Apply inline state variants (highest priority)
        if let Some(inline_state) = node.inline_state_variants.get(&state) {
            state_style = merge_styles(state_style, inline_state.clone());
        }
        
        state_map.insert(state, state_style);
    }
    
    (base, state_map)
}
```

### Example

**Input**:
```xml
<button 
    class="btn-primary" 
    background="#blue"
    hover:background="#red"
/>
```

**Theme** (btn-primary class):
```yaml
.btn-primary:
  background: "#333"
  hover:
    background: "#444"
```

**Output**:
```rust
(
    // Base style
    Some(StyleProperties { 
        background: Some(#blue),  // inline overrides class
        ...
    }),
    // State variants
    {
        Hover: StyleProperties { 
            background: Some(#red),  // inline hover overrides all
            ...
        },
        Focus: StyleProperties { 
            background: Some(#blue),  // no focus override, uses base
            ...
        },
        Active: StyleProperties { ... },
        Disabled: StyleProperties { ... },
    }
)
```

---

## Function: resolve_breakpoint_attributes (New)

**Location**: `crates/dampen-iced/src/builder/helpers.rs`

### Signature

```rust
pub(super) fn resolve_breakpoint_attributes(
    &self,
    node: &WidgetNode,
) -> HashMap<String, AttributeValue>
```

### Contract

Returns attributes for the currently active breakpoint, merged with base attributes.

### Resolution Algorithm

```rust
pub(super) fn resolve_breakpoint_attributes(
    &self,
    node: &WidgetNode,
) -> HashMap<String, AttributeValue> {
    let active_bp = self.active_breakpoint();
    
    // Start with base attributes
    let mut result = node.attributes.clone();
    
    // Override with active breakpoint attributes
    if let Some(bp_attrs) = node.breakpoint_attributes.get(&active_bp) {
        for (key, value) in bp_attrs {
            result.insert(key.clone(), value.clone());
        }
    }
    
    result
}
```

### Example

**Input**:
```xml
<column spacing="20" mobile-spacing="10" tablet-spacing="15" />
```

**With viewport_width = 500.0** (Mobile):
```rust
{ "spacing": Static("10") }
```

**With viewport_width = 800.0** (Tablet):
```rust
{ "spacing": Static("15") }
```

**With viewport_width = 1200.0** (Desktop):
```rust
{ "spacing": Static("20") }  // Falls back to base
```

---

## Widget Builder Updates

Each interactive widget builder must be updated to use state-aware style closures.

### Pattern for Widget Builders

```rust
// In button.rs
pub(super) fn build_button<'a, M>(
    &self,
    node: &WidgetNode,
) -> Element<'a, M> {
    // Resolve styles with state variants
    let (base_style, state_variants) = self.resolve_complete_styles_with_states(node);
    
    // Create state-aware style closure
    let style_closure = move |_theme: &Theme, status: button::Status| {
        let state = map_button_status(status);
        
        match state {
            Some(widget_state) => {
                if let Some(state_style) = state_variants.get(&widget_state) {
                    convert_to_iced_button_style(state_style)
                } else if let Some(ref base) = base_style {
                    convert_to_iced_button_style(base)
                } else {
                    button::Style::default()
                }
            }
            None => {
                // Active/default state
                base_style.as_ref()
                    .map(convert_to_iced_button_style)
                    .unwrap_or_default()
            }
        }
    };
    
    button(content)
        .style(style_closure)
        .on_press(message)
        .into()
}
```

### Widgets Requiring Update

| Widget | File | Iced Status Type |
|--------|------|------------------|
| Button | `button.rs` | `button::Status` |
| TextInput | `text_input.rs` | `text_input::Status` |
| Checkbox | `checkbox.rs` | `checkbox::Status` |
| Slider | `slider.rs` | `slider::Status` + disabled flag |
| Toggler | `toggler.rs` | `toggler::Status` |
| Radio | `radio.rs` | `radio::Status` |
| PickList | `pick_list.rs` | `pick_list::Status` |

### Non-Interactive Widgets (No Changes)

| Widget | Reason |
|--------|--------|
| Container | No Status type in Iced |
| Column | Layout-only widget |
| Row | Layout-only widget |
| Stack | Layout-only widget |
| Text | Static content widget |
| Space | Whitespace widget |
| Rule | Divider widget |
| Scrollable | Container widget |

For non-interactive widgets with `inline_state_variants`, log a debug message:

```rust
if !node.inline_state_variants.is_empty() {
    log::debug!(
        "Widget {:?} at {} has inline state styles but doesn't support states; ignoring",
        node.kind, node.span
    );
}
```
