# XML Schema Reference: Gravity UI Markup

**Version**: 1.0.0  
**Last Updated**: 2025-12-30

This document defines the complete XML schema for Gravity UI markup files (`.gravity`).

---

## Document Structure

### Root Element

Every Gravity file must have a single root element:

```xml
<?xml version="1.0" encoding="UTF-8"?>
<gravity version="1.0" xmlns="https://gravity-ui.dev/schema/1.0">
    <!-- UI content here -->
</gravity>
```

**Attributes:**
- `version` (required): Schema version (e.g., "1.0")
- `xmlns` (optional): Namespace URI for validation tools

**Alternative (simple files):**
```xml
<column>
    <text value="Hello, World!" />
</column>
```

---

## Layout Widgets

### `<column>` - Vertical Layout

Stacks children top-to-bottom.

```xml
<column spacing="10" padding="20" align="center">
    <text value="First" />
    <text value="Second" />
</column>
```

**Attributes:**
| Attribute | Type | Default | Description |
|-----------|------|---------|-------------|
| `spacing` | length | 0 | Space between children |
| `padding` | length/box | 0 | Inner padding |
| `align` | align | start | Horizontal: start, center, end |
| `width` | length | auto | Width constraint |
| `height` | length | auto | Height constraint |

### `<row>` - Horizontal Layout

Places children left-to-right.

```xml
<row spacing="10" align="center">
    <button label="Cancel" />
    <button label="OK" />
</row>
```

**Attributes:**
| Attribute | Type | Default | Description |
|-----------|------|---------|-------------|
| `spacing` | length | 0 | Space between children |
| `padding` | length/box | 0 | Inner padding |
| `align` | align | start | Vertical: start, center, end |
| `width` | length | auto | Width constraint |
| `height` | length | auto | Height constraint |

### `<container>` - Single Child Container

Wraps a single child with padding and alignment.

```xml
<container padding="20" align_x="center" align_y="center">
    <text value="Centered content" />
</container>
```

**Attributes:**
| Attribute | Type | Default | Description |
|-----------|------|---------|-------------|
| `padding` | length/box | 0 | Inner padding |
| `align_x` | align | start | Horizontal alignment |
| `align_y` | align | start | Vertical alignment |
| `width` | length | auto | Width constraint |
| `height` | length | auto | Height constraint |

### `<scrollable>` - Scrollable Container

For overflow content.

```xml
<scrollable direction="vertical" height="300">
    <column>
        <!-- Long content -->
    </column>
</scrollable>
```

**Attributes:**
| Attribute | Type | Default | Description |
|-----------|------|---------|-------------|
| `direction` | direction | vertical | vertical, horizontal, both |
| `width` | length | auto | Width constraint |
| `height` | length | auto | Height constraint |

### `<stack>` - Layered Container

Children overlap in layers.

```xml
<stack>
    <image src="background.png" />
    <text value="Overlay text" />
</stack>
```

**Attributes:**
| Attribute | Type | Default | Description |
|-----------|------|---------|-------------|
| `width` | length | auto | Width constraint |
| `height` | length | auto | Height constraint |

---

## Content Widgets

### `<text>` - Text Display

Displays text content.

```xml
<text value="Hello, World!" size="20" color="#333333" />
<text value="Count: {counter}" />
```

**Attributes:**
| Attribute | Type | Default | Description |
|-----------|------|---------|-------------|
| `value` | string/binding | "" | Text content |
| `size` | number | 16 | Font size in pixels |
| `color` | color | inherit | Text color |
| `font` | font-ref | default | Font family |
| `weight` | weight | normal | normal, bold, light |
| `style` | style-ref | - | Style reference |

### `<image>` - Image Display

Displays an image from file or URL.

```xml
<image src="logo.png" width="100" height="100" />
<image src="{user.avatar}" content_fit="cover" />
```

**Attributes:**
| Attribute | Type | Default | Description |
|-----------|------|---------|-------------|
| `src` | string/binding | required | Image path or URL |
| `width` | length | auto | Width constraint |
| `height` | length | auto | Height constraint |
| `content_fit` | fit | contain | contain, cover, fill, none |

### `<svg>` - SVG Display

Displays SVG content.

```xml
<svg src="icon.svg" width="24" height="24" />
```

**Attributes:**
| Attribute | Type | Default | Description |
|-----------|------|---------|-------------|
| `src` | string | required | SVG file path |
| `width` | length | auto | Width constraint |
| `height` | length | auto | Height constraint |
| `color` | color | inherit | Fill color override |

---

## Interactive Widgets

### `<button>` - Clickable Button

```xml
<button label="Click me" on_click="handle_click" />
<button on_click="submit" style="primary">
    <text value="Submit" />
</button>
```

**Attributes:**
| Attribute | Type | Default | Description |
|-----------|------|---------|-------------|
| `label` | string/binding | - | Button text |
| `on_click` | handler | - | Click handler name |
| `on_press` | handler | - | Press handler |
| `enabled` | bool/binding | true | Interactive state |
| `style` | style-ref | default | Style reference |
| `width` | length | auto | Width constraint |

### `<text_input>` - Text Input Field

```xml
<text_input 
    placeholder="Enter name..."
    value="{name}"
    on_input="update_name"
    on_submit="save_name"
/>
```

**Attributes:**
| Attribute | Type | Default | Description |
|-----------|------|---------|-------------|
| `value` | string/binding | "" | Current value |
| `placeholder` | string | "" | Placeholder text |
| `on_input` | handler | - | Keystroke handler |
| `on_submit` | handler | - | Enter key handler |
| `password` | bool | false | Mask as password |
| `enabled` | bool/binding | true | Editable state |
| `width` | length | auto | Width constraint |

### `<checkbox>` - Toggle Checkbox

```xml
<checkbox 
    label="Accept terms"
    checked="{accepted}"
    on_toggle="toggle_accepted"
/>
```

**Attributes:**
| Attribute | Type | Default | Description |
|-----------|------|---------|-------------|
| `label` | string/binding | "" | Label text |
| `checked` | bool/binding | false | Checked state |
| `on_toggle` | handler | - | Toggle handler |
| `enabled` | bool/binding | true | Interactive state |

### `<slider>` - Numeric Slider

```xml
<slider 
    min="0" 
    max="100" 
    value="{volume}"
    on_change="set_volume"
    step="1"
/>
```

**Attributes:**
| Attribute | Type | Default | Description |
|-----------|------|---------|-------------|
| `min` | number | 0 | Minimum value |
| `max` | number | 100 | Maximum value |
| `value` | number/binding | min | Current value |
| `step` | number | 1 | Step increment |
| `on_change` | handler | - | Change handler |
| `enabled` | bool/binding | true | Interactive state |
| `width` | length | auto | Width constraint |

### `<pick_list>` - Dropdown Selection

```xml
<pick_list 
    options="{items}"
    selected="{current_item}"
    on_select="select_item"
    placeholder="Choose..."
/>
```

**Attributes:**
| Attribute | Type | Default | Description |
|-----------|------|---------|-------------|
| `options` | list/binding | required | List of options |
| `selected` | any/binding | none | Selected value |
| `on_select` | handler | - | Selection handler |
| `placeholder` | string | "" | Placeholder |
| `enabled` | bool/binding | true | Interactive state |
| `width` | length | auto | Width constraint |

### `<toggler>` - Toggle Switch

```xml
<toggler 
    label="Dark mode"
    toggled="{dark_mode}"
    on_toggle="toggle_dark_mode"
/>
```

**Attributes:**
| Attribute | Type | Default | Description |
|-----------|------|---------|-------------|
| `label` | string/binding | "" | Label text |
| `toggled` | bool/binding | false | Toggle state |
| `on_toggle` | handler | - | Toggle handler |
| `enabled` | bool/binding | true | Interactive state |

---

## Decorative Widgets

### `<space>` - Empty Space

```xml
<row>
    <text value="Left" />
    <space width="fill" />
    <text value="Right" />
</row>
```

**Attributes:**
| Attribute | Type | Default | Description |
|-----------|------|---------|-------------|
| `width` | length | 0 | Horizontal space |
| `height` | length | 0 | Vertical space |

### `<rule>` - Divider Line

```xml
<rule direction="horizontal" />
```

**Attributes:**
| Attribute | Type | Default | Description |
|-----------|------|---------|-------------|
| `direction` | direction | horizontal | Line direction |
| `color` | color | inherit | Line color |
| `thickness` | number | 1 | Line thickness |

---

## Binding Expression Syntax

### Basic Field Binding

```xml
<text value="{counter}" />
```

Binds to `model.counter`.

### Nested Field Access

```xml
<text value="{user.profile.name}" />
```

Binds to `model.user.profile.name`.

### Formatted Binding

```xml
<text value="Count: {counter}" />
<text value="Hello, {name}! You have {messages.len()} messages." />
```

Interpolates multiple values.

### Method Calls

```xml
<text value="{items.len()}" />
<text value="{name.to_uppercase()}" />
```

Calls methods on bound values.

### Conditional Binding

```xml
<button enabled="{counter > 0}" />
<text value="{if is_loading then 'Loading...' else 'Ready'}" />
<container style="{if is_error then 'error' else 'default'}" />
```

### Supported Operators

| Operator | Description | Example |
|----------|-------------|---------|
| `==` | Equality | `{status == 'active'}` |
| `!=` | Inequality | `{count != 0}` |
| `<` | Less than | `{age < 18}` |
| `<=` | Less or equal | `{score <= 100}` |
| `>` | Greater than | `{items.len() > 0}` |
| `>=` | Greater or equal | `{progress >= 100}` |
| `&&` | Logical AND | `{active && visible}` |
| `\|\|` | Logical OR | `{error \|\| warning}` |
| `!` | Logical NOT | `{!is_valid}` |

---

## Event Handler Syntax

### Handler Reference

```xml
<button on_click="increment" />
```

References handler function `increment` in Rust code.

### Handler Signatures

```rust
// Simple handler
fn increment(model: &mut Model) {
    model.counter += 1;
}

// Handler with value
fn update_name(model: &mut Model, value: String) {
    model.name = value;
}

// Handler returning Command
fn fetch_data(model: &mut Model) -> Command<Message> {
    Command::perform(async { fetch_api().await }, Message::DataReceived)
}
```

---

## Attribute Value Types

| Type | Format | Examples |
|------|--------|----------|
| `string` | Quoted text or binding | `"Hello"`, `{name}` |
| `number` | Integer or float | `10`, `3.14` |
| `length` | Number with optional unit | `100`, `50%`, `fill`, `auto` |
| `color` | Hex or named color | `#FF0000`, `red`, `{theme.primary}` |
| `bool` | Boolean or binding | `true`, `false`, `{is_enabled}` |
| `binding` | Expression in braces | `{counter}`, `{user.name}` |
| `handler` | Handler function name | `"handle_click"` |
| `style-ref` | Style name | `"primary"`, `"danger"` |
| `align` | Alignment value | `start`, `center`, `end` |
| `direction` | Direction value | `horizontal`, `vertical` |
| `fit` | Content fit mode | `contain`, `cover`, `fill`, `none` |

### Length Units

| Value | Description |
|-------|-------------|
| `100` | Fixed pixels |
| `50%` | Percentage of parent |
| `fill` | Fill remaining space |
| `auto` | Automatic sizing |

---

## Styling System

### Theme Definition

```xml
<themes>
    <theme name="custom">
        <palette 
            primary="#3498db" 
            secondary="#2ecc71"
            success="#27ae60"
            warning="#f39c12"
            danger="#e74c3c"
            background="#ecf0f1"
            surface="#ffffff"
            text="#2c3e50"
            text_secondary="#7f8c8d" />
        <typography 
            font_family="Inter, sans-serif"
            font_size_base="16"
            font_size_small="12"
            font_size_large="24"
            font_weight="normal"
            line_height="1.5" />
        <spacing unit="8" />
    </theme>
</themes>

<global_theme name="custom" />
```

### Style Classes

```xml
<style_classes>
    <style name="button_primary" 
        extends="button_base"
        background="#3498db"
        color="#ffffff"
        padding="12 24"
        border_radius="6">
        <hover background="#2980b9" />
        <active background="#21618c" />
        <disabled opacity="0.5" />
    </style>
</style_classes>
```

### Inline Style Attributes

**Layout:**
- `width`: fixed, fill, shrink, fill_portion(n), percentage
- `height`: fixed, fill, shrink, fill_portion(n), percentage
- `padding`: spacing value (e.g., "10 20" or "10 20 30 40")
- `spacing`: child spacing value
- `align_items`: start, center, end, stretch
- `justify_content`: start, center, end, space_between, space_around, space_evenly
- `position`: relative, absolute
- `top`, `right`, `bottom`, `left`: offset values
- `z_index`: integer

**Style:**
- `background`: color or gradient
- `color`: text color
- `border_width`: thickness
- `border_color`: color
- `border_radius`: corner rounding
- `shadow`: "offset_x offset_y blur color"
- `opacity`: 0.0-1.0
- `transform`: transform operations

**State Variants (prefixed):**
- `hover:*`: hover state (e.g., `hover:background`, `hover:color`)
- `focus:*`: focus state (e.g., `focus:border_color`)
- `active:*`: active state (e.g., `active:background`)
- `disabled:*`: disabled state (e.g., `disabled:opacity`)

**Responsive (prefixed):**
- `mobile:*`: < 640px
- `tablet:*`: 640px - 1024px
- `desktop:*`: > 1024px

### Widget Attributes

**All Widgets:**
- `class`: space-separated style class names
- `theme_ref`: apply local theme
- `disabled`: boolean
- `id`: identifier for state tracking

**Interactive Widgets:**
- `on_click`: handler name
- `on_input`: handler name (text_input)
- `on_change`: handler name (checkbox, slider, pick_list)
- `on_toggle`: handler name (toggler)
- `on_submit`: handler name (text_input)

### State-Based Styling

State variants can be defined using child elements or prefixed attributes:

```xml
<!-- Child elements -->
<style name="btn" background="#3498db">
    <hover background="#2980b9" />
    <active background="#21618c" />
</style>

<!-- Prefixed attributes -->
<style name="btn" 
    background="#3498db"
    hover_background="#2980b9"
    active_background="#21618c" />
```

### Responsive Design

Breakpoint-prefixed attributes override base values:

```xml
<column mobile:spacing="10" desktop:spacing="20">
    <text mobile:size="18" desktop:size="32" value="Responsive" />
</column>
```

---

## Complete Example

```xml
<?xml version="1.0" encoding="UTF-8"?>
<gravity version="1.0">
    <column padding="20" spacing="10">
        <text value="Todo List" size="24" weight="bold" />
        
        <row spacing="10">
            <text_input 
                placeholder="New todo..."
                value="{new_todo}"
                on_input="update_new_todo"
                on_submit="add_todo"
                width="fill"
            />
            <button label="Add" on_click="add_todo" enabled="{new_todo.len() > 0}" />
        </row>
        
        <rule />
        
        <scrollable height="300">
            <column spacing="5">
                <text value="{items.len()} items" color="#888888" />
            </column>
        </scrollable>
        
        <row spacing="10">
            <text value="{completed_count} completed" />
            <space width="fill" />
            <button 
                label="Clear completed" 
                on_click="clear_completed"
                enabled="{completed_count > 0}"
            />
        </row>
    </column>
</gravity>
```

---

## Schema Versioning

**Version 1.0**: Initial release with core widgets
- Layout: column, row, container, scrollable, stack
- Content: text, image, svg
- Interactive: button, text_input, checkbox, slider, pick_list, toggler
- Decorative: space, rule
- Bindings: field access, method calls, conditionals, formatting
- Events: click, input, change, toggle, submit

**Future Versions**:
- 1.1: Iteration support, custom widgets
- 2.0: Breaking changes if needed

Files without a version attribute are treated as version 1.0.
