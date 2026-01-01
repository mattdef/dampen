# XML Schema Specification: Gravity UI Markup

**Feature**: 001-framework-technical-specs  
**Version**: 1.0.0  
**Date**: 2025-12-30

## Overview

This document defines the XML schema for Gravity UI markup files (`.gravity`). The schema describes all supported elements, attributes, and binding syntax.

---

## Document Structure

### Root Element

Every Gravity file must have a single root element, typically a layout container.

```xml
<?xml version="1.0" encoding="UTF-8"?>
<gravity version="1.0" xmlns="https://gravity-ui.dev/schema/1.0">
    <!-- UI content here -->
</gravity>
```

| Attribute | Required | Description |
|-----------|----------|-------------|
| `version` | Yes | Schema version (e.g., "1.0") |
| `xmlns` | No | Namespace URI (for validation tools) |

### Alternative: Direct Root Widget

For simple files, the wrapper is optional:

```xml
<column>
    <text value="Hello, World!" />
</column>
```

---

## Layout Widgets

### `<column>`

Vertical layout container. Children are stacked top-to-bottom.

```xml
<column spacing="10" padding="20" align="center">
    <text value="First" />
    <text value="Second" />
</column>
```

| Attribute | Type | Default | Description |
|-----------|------|---------|-------------|
| `spacing` | length | 0 | Space between children |
| `padding` | length/box | 0 | Inner padding |
| `align` | align | start | Horizontal alignment: start, center, end |
| `width` | length | auto | Width constraint |
| `height` | length | auto | Height constraint |

### `<row>`

Horizontal layout container. Children are placed left-to-right.

```xml
<row spacing="10" align="center">
    <button label="Cancel" />
    <button label="OK" />
</row>
```

| Attribute | Type | Default | Description |
|-----------|------|---------|-------------|
| `spacing` | length | 0 | Space between children |
| `padding` | length/box | 0 | Inner padding |
| `align` | align | start | Vertical alignment: start, center, end |
| `width` | length | auto | Width constraint |
| `height` | length | auto | Height constraint |

### `<container>`

Single-child container with padding and alignment.

```xml
<container padding="20" align_x="center" align_y="center">
    <text value="Centered content" />
</container>
```

| Attribute | Type | Default | Description |
|-----------|------|---------|-------------|
| `padding` | length/box | 0 | Inner padding |
| `align_x` | align | start | Horizontal alignment |
| `align_y` | align | start | Vertical alignment |
| `width` | length | auto | Width constraint |
| `height` | length | auto | Height constraint |
| `style` | style-ref | - | Style reference |

### `<scrollable>`

Scrollable container for overflow content.

```xml
<scrollable direction="vertical" height="300">
    <column>
        <!-- Long content -->
    </column>
</scrollable>
```

| Attribute | Type | Default | Description |
|-----------|------|---------|-------------|
| `direction` | direction | vertical | Scroll direction: vertical, horizontal, both |
| `width` | length | auto | Width constraint |
| `height` | length | auto | Height constraint |

### `<stack>`

Layered container where children overlap.

```xml
<stack>
    <image src="background.png" />
    <text value="Overlay text" />
</stack>
```

| Attribute | Type | Default | Description |
|-----------|------|---------|-------------|
| `width` | length | auto | Width constraint |
| `height` | length | auto | Height constraint |

---

## Content Widgets

### `<text>`

Displays text content.

```xml
<text value="Hello, World!" size="20" color="#333333" />
<text value="Count: {counter}" />
```

| Attribute | Type | Default | Description |
|-----------|------|---------|-------------|
| `value` | string/binding | "" | Text content |
| `size` | number | 16 | Font size in pixels |
| `color` | color | inherit | Text color |
| `font` | font-ref | default | Font family |
| `weight` | weight | normal | Font weight: normal, bold, light |
| `style` | style-ref | - | Style reference |

### `<image>`

Displays an image from file or URL.

```xml
<image src="logo.png" width="100" height="100" />
<image src="{user.avatar}" content_fit="cover" />
```

| Attribute | Type | Default | Description |
|-----------|------|---------|-------------|
| `src` | string/binding | required | Image path or URL |
| `width` | length | auto | Width constraint |
| `height` | length | auto | Height constraint |
| `content_fit` | fit | contain | Scaling: contain, cover, fill, none |

### `<svg>`

Displays SVG content.

```xml
<svg src="icon.svg" width="24" height="24" />
```

| Attribute | Type | Default | Description |
|-----------|------|---------|-------------|
| `src` | string | required | SVG file path |
| `width` | length | auto | Width constraint |
| `height` | length | auto | Height constraint |
| `color` | color | inherit | Fill color override |

---

## Interactive Widgets

### `<button>`

Clickable button.

```xml
<button label="Click me" on_click="handle_click" />
<button on_click="submit" style="primary">
    <text value="Submit" />
</button>
```

| Attribute | Type | Default | Description |
|-----------|------|---------|-------------|
| `label` | string/binding | - | Button text (simple form) |
| `on_click` | handler | - | Click handler name |
| `on_press` | handler | - | Press handler (fires on press, not release) |
| `enabled` | bool/binding | true | Whether button is interactive |
| `style` | style-ref | default | Style reference |
| `width` | length | auto | Width constraint |

### `<text_input>`

Single-line text input field.

```xml
<text_input 
    placeholder="Enter name..."
    value="{name}"
    on_input="update_name"
    on_submit="save_name"
/>
```

| Attribute | Type | Default | Description |
|-----------|------|---------|-------------|
| `value` | string/binding | "" | Current value |
| `placeholder` | string | "" | Placeholder text |
| `on_input` | handler | - | Handler called on every keystroke |
| `on_submit` | handler | - | Handler called on Enter key |
| `password` | bool | false | Mask input as password |
| `enabled` | bool/binding | true | Whether input is editable |
| `width` | length | auto | Width constraint |

### `<checkbox>`

Toggle checkbox.

```xml
<checkbox 
    label="Accept terms"
    checked="{accepted}"
    on_toggle="toggle_accepted"
/>
```

| Attribute | Type | Default | Description |
|-----------|------|---------|-------------|
| `label` | string/binding | "" | Label text |
| `checked` | bool/binding | false | Checked state |
| `on_toggle` | handler | - | Handler called on toggle |
| `enabled` | bool/binding | true | Whether checkbox is interactive |

### `<slider>`

Numeric slider.

```xml
<slider 
    min="0" 
    max="100" 
    value="{volume}"
    on_change="set_volume"
    step="1"
/>
```

| Attribute | Type | Default | Description |
|-----------|------|---------|-------------|
| `min` | number | 0 | Minimum value |
| `max` | number | 100 | Maximum value |
| `value` | number/binding | min | Current value |
| `step` | number | 1 | Step increment |
| `on_change` | handler | - | Handler called on value change |
| `enabled` | bool/binding | true | Whether slider is interactive |
| `width` | length | auto | Width constraint |

### `<pick_list>`

Dropdown selection.

```xml
<pick_list 
    options="{items}"
    selected="{current_item}"
    on_select="select_item"
    placeholder="Choose..."
/>
```

| Attribute | Type | Default | Description |
|-----------|------|---------|-------------|
| `options` | list/binding | required | List of options |
| `selected` | any/binding | none | Currently selected value |
| `on_select` | handler | - | Handler called on selection |
| `placeholder` | string | "" | Placeholder when nothing selected |
| `enabled` | bool/binding | true | Whether picker is interactive |
| `width` | length | auto | Width constraint |

### `<toggler>`

Toggle switch.

```xml
<toggler 
    label="Dark mode"
    toggled="{dark_mode}"
    on_toggle="toggle_dark_mode"
/>
```

| Attribute | Type | Default | Description |
|-----------|------|---------|-------------|
| `label` | string/binding | "" | Label text |
| `toggled` | bool/binding | false | Toggle state |
| `on_toggle` | handler | - | Handler called on toggle |
| `enabled` | bool/binding | true | Whether toggler is interactive |

---

## Decorative Widgets

### `<space>`

Empty space for layout.

```xml
<row>
    <text value="Left" />
    <space width="fill" />
    <text value="Right" />
</row>
```

| Attribute | Type | Default | Description |
|-----------|------|---------|-------------|
| `width` | length | 0 | Horizontal space |
| `height` | length | 0 | Vertical space |

### `<rule>`

Horizontal or vertical divider line.

```xml
<rule direction="horizontal" />
```

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

Interpolates bindings within static text.

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

Evaluates boolean expressions.

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

Handlers must match one of these signatures:

```rust
// Simple handler
#[ui_handler]
fn increment(model: &mut Model) {
    model.counter += 1;
}

// Handler with value (for on_input, on_change)
#[ui_handler]
fn update_name(model: &mut Model, value: String) {
    model.name = value;
}

// Handler returning Command (for async operations)
#[ui_handler]
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
                <!-- Todo items would be rendered here via iteration (future feature) -->
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

The `version` attribute ensures forward compatibility:

- **1.0**: Initial release with core widgets
- **1.1**: (planned) Iteration support, custom widgets
- **2.0**: (future) Breaking changes if needed

Files without a version attribute are treated as version 1.0.
