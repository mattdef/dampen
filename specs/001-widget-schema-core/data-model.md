# Data Model: Widget Schema Migration to Core

**Feature**: 001-widget-schema-core  
**Date**: 2026-01-23  
**Status**: Complete

## Entities

### WidgetSchema (NEW - in dampen-core)

Represents the validation contract for a single widget type.

| Field | Type | Description |
|-------|------|-------------|
| required | `&'static [&'static str]` | Attributes that MUST be present |
| optional | `&'static [&'static str]` | Attributes that MAY be present |
| events | `&'static [&'static str]` | Valid event handler attributes |
| style_attributes | `&'static [&'static str]` | Valid styling attributes |
| layout_attributes | `&'static [&'static str]` | Valid layout attributes |

**Methods**:
- `all_valid() -> HashSet<&'static str>` - Returns union of all attribute categories
- `all_valid_names() -> Vec<&'static str>` - Returns all valid names as vector

**Validation Rules**:
- No attribute should appear in multiple categories (enforced by design)
- Empty slices are valid (e.g., Space widget has no required attributes)

### Common Attribute Constants (NEW - in dampen-core)

```rust
pub const COMMON_STYLE_ATTRIBUTES: &[&str] = &[
    "background", "color", "border_color", "border_width", "border_radius",
    "border_style", "shadow", "opacity", "transform", "style", "text_color",
    "shadow_color", "shadow_offset", "shadow_blur_radius",
];

pub const COMMON_LAYOUT_ATTRIBUTES: &[&str] = &[
    "width", "height", "min_width", "max_width", "min_height", "max_height",
    "padding", "spacing", "align_items", "justify_content", "align",
    "align_x", "align_y", "align_self", "direction", "position",
    "top", "right", "bottom", "left", "z_index", "class", "theme", "theme_ref",
];

pub const COMMON_EVENTS: &[&str] = &[
    "on_click", "on_press", "on_release", "on_change", "on_input",
    "on_submit", "on_select", "on_toggle", "on_scroll",
];
```

## Widget-Specific Schemas

### Text
| Category | Attributes |
|----------|------------|
| required | `value` |
| optional | `size`, `weight`, `color` |
| events | COMMON_EVENTS |
| style | COMMON_STYLE_ATTRIBUTES |
| layout | COMMON_LAYOUT_ATTRIBUTES |

### Image
| Category | Attributes |
|----------|------------|
| required | `src` |
| optional | `width`, `height`, `fit`, `filter_method`, `path` |
| events | COMMON_EVENTS |
| style | COMMON_STYLE_ATTRIBUTES |
| layout | COMMON_LAYOUT_ATTRIBUTES |

### Button
| Category | Attributes |
|----------|------------|
| required | (none) |
| optional | `label`, `enabled` |
| events | `on_click`, `on_press`, `on_release` |
| style | COMMON_STYLE_ATTRIBUTES |
| layout | COMMON_LAYOUT_ATTRIBUTES |

### TextInput
| Category | Attributes |
|----------|------------|
| required | (none) |
| optional | `placeholder`, `value`, `password`, `icon`, `size` |
| events | `on_input`, `on_submit`, `on_change`, `on_paste` |
| style | COMMON_STYLE_ATTRIBUTES |
| layout | COMMON_LAYOUT_ATTRIBUTES |

### Checkbox
| Category | Attributes |
|----------|------------|
| required | (none) |
| optional | `checked`, `label`, `icon`, `size` |
| events | `on_toggle` |
| style | COMMON_STYLE_ATTRIBUTES |
| layout | COMMON_LAYOUT_ATTRIBUTES |

### Radio
| Category | Attributes |
|----------|------------|
| required | `label`, `value` |
| optional | `selected`, `disabled`, `size`, `text_size`, `text_line_height`, `text_shaping` |
| events | `on_select` |
| style | COMMON_STYLE_ATTRIBUTES |
| layout | COMMON_LAYOUT_ATTRIBUTES |

### Slider
| Category | Attributes |
|----------|------------|
| required | (none) |
| optional | `min`, `max`, `value`, `step` |
| events | `on_change`, `on_release` |
| style | COMMON_STYLE_ATTRIBUTES |
| layout | COMMON_LAYOUT_ATTRIBUTES |

### Column / Row / Container
| Category | Attributes |
|----------|------------|
| required | (none) |
| optional | (none) |
| events | COMMON_EVENTS |
| style | COMMON_STYLE_ATTRIBUTES |
| layout | COMMON_LAYOUT_ATTRIBUTES |

### Scrollable
| Category | Attributes |
|----------|------------|
| required | (none) |
| optional | (none) |
| events | `on_scroll` |
| style | COMMON_STYLE_ATTRIBUTES |
| layout | COMMON_LAYOUT_ATTRIBUTES |

### Stack
| Category | Attributes |
|----------|------------|
| required | (none) |
| optional | (none) |
| events | COMMON_EVENTS |
| style | COMMON_STYLE_ATTRIBUTES |
| layout | COMMON_LAYOUT_ATTRIBUTES |

### Svg
| Category | Attributes |
|----------|------------|
| required | `src` |
| optional | `width`, `height`, `path` |
| events | COMMON_EVENTS |
| style | COMMON_STYLE_ATTRIBUTES |
| layout | COMMON_LAYOUT_ATTRIBUTES |

### PickList
| Category | Attributes |
|----------|------------|
| required | (none) |
| optional | `placeholder`, `selected`, `options` |
| events | `on_select` |
| style | COMMON_STYLE_ATTRIBUTES |
| layout | COMMON_LAYOUT_ATTRIBUTES |

### Toggler
| Category | Attributes |
|----------|------------|
| required | (none) |
| optional | `checked`, `active`, `label` |
| events | `on_toggle` |
| style | COMMON_STYLE_ATTRIBUTES |
| layout | COMMON_LAYOUT_ATTRIBUTES |

### Space / Rule
| Category | Attributes |
|----------|------------|
| required | (none) |
| optional | (none) |
| events | (none) |
| style | COMMON_STYLE_ATTRIBUTES |
| layout | COMMON_LAYOUT_ATTRIBUTES |

### ComboBox
| Category | Attributes |
|----------|------------|
| required | (none) |
| optional | `placeholder`, `value`, `options` |
| events | `on_input`, `on_select` |
| style | COMMON_STYLE_ATTRIBUTES |
| layout | COMMON_LAYOUT_ATTRIBUTES |

### ProgressBar
| Category | Attributes |
|----------|------------|
| required | (none) |
| optional | `value`, `min`, `max`, `style` |
| events | (none) |
| style | COMMON_STYLE_ATTRIBUTES |
| layout | COMMON_LAYOUT_ATTRIBUTES |

### Tooltip
| Category | Attributes |
|----------|------------|
| required | (none) |
| optional | `message`, `position`, `delay` |
| events | COMMON_EVENTS |
| style | COMMON_STYLE_ATTRIBUTES |
| layout | (none - special case) |

### Grid
| Category | Attributes |
|----------|------------|
| required | (none) |
| optional | `columns` |
| events | COMMON_EVENTS |
| style | COMMON_STYLE_ATTRIBUTES |
| layout | COMMON_LAYOUT_ATTRIBUTES |

### Canvas
| Category | Attributes |
|----------|------------|
| required | (none) |
| optional | `program` |
| events | `on_draw` |
| style | COMMON_STYLE_ATTRIBUTES |
| layout | COMMON_LAYOUT_ATTRIBUTES |

### Float
| Category | Attributes |
|----------|------------|
| required | (none) |
| optional | (none) |
| events | COMMON_EVENTS |
| style | COMMON_STYLE_ATTRIBUTES |
| layout | COMMON_LAYOUT_ATTRIBUTES |

### For (Control Flow)
| Category | Attributes |
|----------|------------|
| required | `each`, `in` |
| optional | `template` |
| events | (none) |
| style | COMMON_STYLE_ATTRIBUTES |
| layout | COMMON_LAYOUT_ATTRIBUTES |

### If (Control Flow)
| Category | Attributes |
|----------|------------|
| required | `condition` |
| optional | (none) |
| events | (none) |
| style | COMMON_STYLE_ATTRIBUTES |
| layout | COMMON_LAYOUT_ATTRIBUTES |

### Custom (User-Defined)
| Category | Attributes |
|----------|------------|
| required | (none) |
| optional | (none - permissive) |
| events | (none - permissive) |
| style | (none - permissive) |
| layout | (none - permissive) |

**Note**: Custom widgets return empty schemas, allowing any attributes. Validation for custom widgets should be handled by user-provided configuration (out of scope for this feature).

## Relationships

```
WidgetKind (existing) ----has-one----> WidgetSchema (new)
     |
     |-- Column
     |-- Row
     |-- Container
     |-- Text
     |-- Button
     |-- ... (25 total widget types)
     |-- Custom(String)
```

## State Transitions

N/A - Schema is immutable compile-time constant.
