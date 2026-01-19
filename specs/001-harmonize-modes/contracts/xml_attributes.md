# Contract: Dampen XML Attributes Standard

**Status**: Proposed
**Enforcement**: Strict (Parser should warn/error on violations, Codegen should support all)

## Global Layout Attributes
Supported by **ALL** widgets (via Container wrapping if necessary).

- `width`: `fill`, `shrink`, `123`, `50%` (fill portion approximation)
- `height`: `fill`, `shrink`, `123`, `50%`
- `align_x`: `start`, `center`, `end`
- `align_y`: `start`, `center`, `end`
- `padding`: `10`, `10 20` (top/bottom left/right)

## Widget Specific Attributes

### Container
- `max_width`: number
- `max_height`: number

### Column / Row
- `spacing`: number
- `align_items`: `start`, `center`, `end` (cross-axis alignment)

### Scrollable
- `direction`: `vertical` (default), `horizontal`, `both` (future)

### Text
- `size`: number
- `color`: hex/name
- `weight`: `thin`, `extra_light`, `light`, `normal`, `medium`, `semibold`, `bold`, `extra_bold`, `black`
- `font`: string (family)

### Button
- `on_click`: event handler
- `padding`: number (internal)

### Image / Svg
- `src`: path string (asset relative or absolute)
- `content_fit`: `contain`, `cover`, `fill`, `none`, `scale_down`

### TextInput
- `value`: binding
- `placeholder`: string
- `on_input`: event handler
- `password`: boolean (masks characters)
- `on_submit`: event handler

### Slider
- `value`: binding
- `min`: number
- `max`: number
- `step`: number
- `on_change`: event handler
- `on_release`: event handler

### Checkbox
- `checked`: boolean binding
- `label`: string
- `on_toggle`: event handler

### Toggler
- `toggled`: boolean binding (standardized name)
- `label`: string
- `on_toggle`: event handler

### PickList
- `selected`: binding
- `options`: binding (Vec<String> or similar)
- `on_selected`: event handler

## Loops
- `each`: variable name (e.g., "user")
- `in`: collection binding (e.g., "{model.users}")
