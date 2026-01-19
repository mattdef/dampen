# Data Model: Harmonize Modes

## 1. WidgetNode Updates (IR)

The `WidgetNode` struct in `dampen-core` remains the primary data structure, but its semantic usage changes.

### Standardized Attributes

The `attributes` map in `WidgetNode` will now strictly enforce the following standard keys. Attributes outside this list (per widget) may be rejected or ignored, but for parity, we enforce strictness.

| Attribute | Type | Description | Supported Widgets |
|-----------|------|-------------|-------------------|
| `width` | Length | Width (px, fill, shrink) | All (via wrapper if needed) |
| `height` | Length | Height (px, fill, shrink) | All (via wrapper if needed) |
| `padding` | float | Internal padding | Container, Column, Row, Scrollable, Text (via wrapper) |
| `spacing` | float | Spacing between children | Column, Row |
| `align_x` | Alignment | Horizontal alignment (start, center, end) | All (via wrapper/container) |
| `align_y` | Alignment | Vertical alignment (start, center, end) | All (via wrapper/container) |
| `visible` | bool | Visibility toggle | All |

### Widget-Specific Standards

#### Interactive
- **Button**: `on_click`, `label`, `enabled`
- **TextInput**: `value`, `placeholder`, `on_input`, `password` (bool), `secure` (alias to password)
- **Checkbox**: `checked`, `label`, `on_toggle` (unified from on_change)
- **Toggler**: `active` (unified alias for is_toggled), `label`, `on_toggle`
- **Slider**: `value`, `min`, `max`, `step`, `on_change`

#### Display
- **Image**: `src` (unified from path), `fit` (contain, cover, fill)
- **Svg**: `src` (unified from path)

#### Loop
- **For**: `item` (variable name), `items` (collection) -> **UNIFIED TO**: `for="item in items"` (attribute `for` containing the expression) OR `each="item"` + `in="items"`.
- **Decision**: Use `each="item"` and `in="items"` as the standard to avoid parsing complex strings in attributes.

## 2. Style Properties (IR)

`StyleProperties` struct needs to map to Iced 0.14 Style structs.

```rust
pub struct StyleProperties {
    pub background: Option<Background>,
    pub color: Option<Color>, // Text color
    pub border: Option<Border>,
    pub shadow: Option<Shadow>,
    // Removed: transform, opacity (not easily supported in static codegen yet)
}
```

## 3. Visual Test Artifacts

New entities for the visual test harness:

```rust
pub struct VisualTestCase {
    pub name: String,
    pub damp_xml: String,
    pub tolerance: f32,
    pub expected_diff: f32, // For negative tests
}
```
