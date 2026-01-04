# XML Schema: Widget Definitions

**Feature Branch**: `005-implement-real-widgets`  
**Date**: 2026-01-04

## Widget Elements

### text_input

Text input field for user text entry.

```xml
<text_input 
    placeholder="Enter text..."
    value="{model_field}"
    on_input="handler_name"
/>
```

**Attributes**:

| Attribute | Type | Required | Description |
|-----------|------|----------|-------------|
| `placeholder` | `string` | No | Text shown when input is empty |
| `value` | `string \| binding` | No | Current input value |
| `on_input` | `handler` | No | Handler receiving new text on each keystroke |

**Events**:
- `on_input`: Triggered on every keystroke, passes new complete text value

**Example**:
```xml
<text_input 
    placeholder="Enter your name"
    value="{user.name}"
    on_input="update_name"
/>
```

---

### checkbox

Checkbox for boolean toggle with label.

```xml
<checkbox 
    label="Label text"
    checked="{model_field}"
    on_toggle="handler_name"
/>
```

**Attributes**:

| Attribute | Type | Required | Description |
|-----------|------|----------|-------------|
| `label` | `string` | No | Text displayed next to checkbox |
| `checked` | `bool \| binding` | No | Whether checkbox is checked |
| `on_toggle` | `handler` | No | Handler receiving new boolean state |

**Events**:
- `on_toggle`: Triggered on click, passes new boolean state (`"true"` or `"false"`)

**Example**:
```xml
<checkbox 
    label="Accept Terms"
    checked="{user.accepted_terms}"
    on_toggle="toggle_terms"
/>
```

---

### toggler

Modern toggle switch for boolean values.

```xml
<toggler 
    label="Label text"
    active="{model_field}"
    on_toggle="handler_name"
/>
```

**Attributes**:

| Attribute | Type | Required | Description |
|-----------|------|----------|-------------|
| `label` | `string` | No | Text displayed next to toggler |
| `active` | `bool \| binding` | No | Whether toggler is active |
| `on_toggle` | `handler` | No | Handler receiving new boolean state |

**Events**:
- `on_toggle`: Triggered on click/slide, passes new boolean state (`"true"` or `"false"`)

**Example**:
```xml
<toggler 
    label="Dark Mode"
    active="{settings.dark_mode}"
    on_toggle="toggle_dark_mode"
/>
```

---

### pick_list

Dropdown selection from predefined options.

```xml
<pick_list 
    options="Option1,Option2,Option3"
    selected="{model_field}"
    placeholder="Select..."
    on_select="handler_name"
/>
```

**Attributes**:

| Attribute | Type | Required | Description |
|-----------|------|----------|-------------|
| `options` | `string` | Yes | Comma-separated list of options |
| `selected` | `string \| binding` | No | Currently selected option |
| `placeholder` | `string` | No | Text shown when nothing selected |
| `on_select` | `handler` | No | Handler receiving selected option |

**Events**:
- `on_select`: Triggered on selection, passes selected option string

**Example**:
```xml
<pick_list 
    options="Low,Medium,High"
    selected="{task.priority}"
    placeholder="Select priority"
    on_select="update_priority"
/>
```

---

### slider

Slider for numeric value selection within a range.

```xml
<slider 
    min="0"
    max="100"
    value="{model_field}"
    on_change="handler_name"
/>
```

**Attributes**:

| Attribute | Type | Required | Description |
|-----------|------|----------|-------------|
| `min` | `number` | No | Minimum value (default: 0.0) |
| `max` | `number` | No | Maximum value (default: 100.0) |
| `value` | `number \| binding` | No | Current slider value |
| `on_change` | `handler` | No | Handler receiving new numeric value |

**Events**:
- `on_change`: Triggered during drag, passes new numeric value as string

**Example**:
```xml
<slider 
    min="0"
    max="100"
    value="{audio.volume}"
    on_change="set_volume"
/>
```

---

### image

Display image from file path.

```xml
<image 
    src="path/to/image.png"
    width="100"
    height="100"
/>
```

**Attributes**:

| Attribute | Type | Required | Description |
|-----------|------|----------|-------------|
| `src` | `string` | Yes | Path to image file |
| `width` | `number` | No | Display width in pixels |
| `height` | `number` | No | Display height in pixels |

**Events**: None (display-only widget)

**Example**:
```xml
<image 
    src="assets/logo.png"
    width="200"
    height="80"
/>
```

## Event Value Encoding

All event handlers receive values as strings via `HandlerMessage::Handler(name, Some(value))`:

| Widget | Event | Value Format |
|--------|-------|--------------|
| text_input | on_input | Raw text string |
| checkbox | on_toggle | `"true"` or `"false"` |
| toggler | on_toggle | `"true"` or `"false"` |
| pick_list | on_select | Selected option string |
| slider | on_change | Float as string (e.g., `"42.5"`) |

## Binding Expressions

All widgets support binding expressions in `value`, `checked`, `active`, `selected` attributes:

```xml
<!-- Simple field binding -->
<text_input value="{name}" />

<!-- Nested field binding -->
<text_input value="{user.profile.name}" />

<!-- Method call binding -->
<pick_list selected="{get_priority()}" />

<!-- Interpolated string (text_input only for value) -->
<text_input placeholder="Hello {name}!" />
```
