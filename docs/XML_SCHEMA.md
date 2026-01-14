# XML Schema Reference: Dampen UI Markup

**Version**: 1.0.0  
**Last Updated**: 2025-12-30

This document defines the complete XML schema for Dampen UI markup files (`.dampen`).

---

## Document Structure

### Root Element

Every Dampen file must have a single root element:

```xml
<?xml version="1.0" encoding="UTF-8"?>
<dampen version="1.0" xmlns="https://dampen-ui.dev/schema/1.0">
    <!-- UI content here -->
</dampen>
```

**Attributes:**

| Attribute | Type | Required | Description |
|-----------|------|----------|-------------|
| `version` | string | **Yes** | Schema version in `major.minor` format (e.g., "1.0"). Specifies which Dampen schema version the file uses. See [Schema Versioning](#schema-versioning) for details. |
| `xmlns` | URI | No | Namespace URI for XML validation tools (e.g., "https://dampen-ui.dev/schema/1.0") |

**Version Attribute Details:**
- **Format**: `major.minor` (e.g., "1.0", "1.1", "2.0")
- **Current Supported**: 1.0 (all core widgets)
- **Validation**: Parser rejects files with unsupported future versions
- **Default Behavior**: Files without `version` attribute default to 1.0 for backward compatibility

**Valid Examples:**
```xml
<!-- Explicit version (recommended) -->
<dampen version="1.0">
    <column><text value="Hello!" /></column>
</dampen>

<!-- With namespace -->
<dampen version="1.0" xmlns="https://dampen-ui.dev/schema/1.0">
    <column><text value="Hello!" /></column>
</dampen>
```

**Alternative (simple files without `<dampen>` wrapper):**
```xml
<!-- Implicit version 1.0 -->
<column>
    <text value="Hello, World!" />
</column>
```
Note: Files starting directly with widgets (without `<dampen>` root) implicitly use version 1.0.

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

### `<radio>` - Radio Button Group

```xml
<radio_group name="selection">
    <radio label="Option A" value="a" />
    <radio label="Option B" value="b" />
    <radio label="Option C" value="c" />
</radio_group>
```

Or with inline definition:

```xml
<column>
    <radio label="Small" value="small" selected="{size}" on_select="set_size" />
    <radio label="Medium" value="medium" selected="{size}" on_select="set_size" />
    <radio label="Large" value="large" selected="{size}" on_select="set_size" />
</column>
```

**Radio Attributes:**
| Attribute | Type | Default | Description |
|-----------|------|---------|-------------|
| `label` | string/binding | "" | Radio button label |
| `value` | string/binding | required | Value when selected |
| `selected` | any/binding | none | Currently selected value |
| `on_select` | handler | - | Selection handler |
| `disabled` | bool/binding | false | Disabled state |

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

### `<progress_bar>` - Progress Indicator

```xml
<progress_bar
    min="0"
    max="100"
    value="{progress}"
/>
```

**Attributes:**
| Attribute | Type | Default | Description |
|-----------|------|---------|-------------|
| `min` | number | 0 | Minimum value |
| `max` | number | 100 | Maximum value |
| `value` | number/binding | 0 | Current progress |

### `<for>` - Iteration Widget

Render a list of items by iterating over a collection.

```xml
<column>
    <for each="item" in="items">
        <text value="{item}" />
    </for>
</column>
```

**Attributes:**
| Attribute | Type | Default | Description |
|-----------|------|---------|-------------|
| `each` | string | required | Iterator variable name |
| `in` | binding | required | Collection to iterate over |

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

### Shared State Bindings

**NEW in v0.2.4!** Access application-wide shared state from any view.

**Syntax:**
```xml
<text value="{shared.field}" />
```

**Local vs Shared Bindings:**

```xml
<!-- Local model binding (view-specific state) -->
<text value="{message}" />
<text value="{user.email}" />

<!-- Shared state binding (cross-view state) -->
<text value="{shared.theme}" />
<text value="{shared.username}" />

<!-- Mixed usage -->
<column>
    <text value="Welcome, {shared.username}!" />
    <text value="{local_status}" />
</column>
```

**Common Use Cases:**

```xml
<!-- User preferences -->
<text value="Theme: {shared.theme}" />
<text value="Language: {shared.language}" />

<!-- Session data -->
<text value="Logged in as: {shared.current_user}" />

<!-- Application settings -->
<toggler 
    label="Dark Mode"
    toggled="{shared.dark_mode}"
    on_toggle="toggle_dark_mode"
/>
```

**Nested Field Access:**

```xml
<!-- Access nested shared fields -->
<text value="{shared.user.profile.name}" />
<text value="{shared.settings.theme.primary_color}" />
```

**Requirements:**

1. **Shared model** must derive `UiModel`:
   ```rust
   #[derive(Clone, UiModel, Serialize, Deserialize)]
   pub struct SharedState {
       pub theme: String,
       pub username: String,
   }
   ```

2. **AppState** must have shared context:
   ```rust
   let shared = SharedContext::new(SharedState::default());
   let state = AppState::with_handlers(document, handlers)
       .with_shared_context(shared);
   ```

3. **Handlers** can modify shared state:
   ```rust
   registry.register_with_value_and_shared(
       "update_theme",
       |model, shared, theme| {
           if let Some(s) = shared.downcast_ref::<SharedContext<SharedState>>() {
               s.write(|state| state.theme = theme);
           }
       }
   );
   ```

**Behavior:**

- **Thread-safe**: Multiple views can read simultaneously
- **Hot-reload preserved**: Shared state survives XML reloads
- **Type-safe**: Compile-time verification via UiModel trait
- **Null-safe**: Missing fields render as empty string

**See also:** `docs/USAGE.md` "Shared State for Multi-View Applications" section

---

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
<dampen version="1.0">
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
</dampen>
```

---

## Schema Versioning

Dampen uses semantic versioning for its XML schema to ensure compatibility and enable evolution of the UI framework.

### Version Format

Version numbers follow the `major.minor` format:
- **Major version**: Breaking changes or removal of features (e.g., 2.0)
- **Minor version**: Backward-compatible additions (e.g., 1.1)

### Supported Versions

**Version 1.0** (Current): Initial release with core widgets
- Layout: column, row, container, scrollable, stack
- Content: text, image, svg
- Interactive: button, text_input, checkbox, slider, pick_list, toggler, radio, progress_bar
- Control flow: for
- Decorative: space, rule
- Bindings: field access, method calls, conditionals, formatting
- Events: click, input, change, toggle, submit, select

**Version 1.1** (Experimental): Additional features
- Canvas widget for custom drawing (⚠️ experimental, not fully functional)
- Grid layout widget (planned)
- Tooltip widget (planned)
- ComboBox widget (planned)

**Note**: Canvas widget is currently marked as v1.1 to indicate it's experimental. While parseable in v1.0 documents, it will produce a validation warning and may not render correctly.

### Version Validation

The parser validates schema versions at parse time:

1. **Declared Version Check**: Parser reads the `version` attribute from `<dampen>` root
2. **Support Validation**: Compares against maximum supported version (currently 1.0)
3. **Error Handling**: Rejects files declaring unsupported future versions with clear error messages
4. **Widget Version Warnings**: `dampen check` warns about widgets requiring higher versions than declared (e.g., Canvas in v1.0 documents)

### Backward Compatibility

- Files without a `version` attribute default to version 1.0
- All version 1.0 files will continue to work when version 1.1 is released
- Future versions will maintain backward compatibility within the same major version

### Best Practices

1. **Always declare version explicitly**: `<dampen version="1.0">` makes intent clear
2. **Use the latest supported version**: Currently 1.0
3. **Validate before commits**: Run `dampen check` to catch version errors early
4. **Don't mix versions**: All `.dampen` files in a project should use the same version

---

## Troubleshooting

### Version-Related Errors

#### Error: "Schema version X.Y is not supported"

**Cause**: Your file declares a version newer than your installed Dampen framework supports.

**Example**:
```
Error: Schema version 2.0 is not supported. Maximum supported version: 1.0
  --> src/ui/window.dampen:1:9
   |
 1 | <dampen version="2.0">
   |         ^^^^^^^^^^^^^^ unsupported version
   |
Suggestion: Upgrade dampen-core to support v2.0, or use version="1.0"
```

**Solutions**:
1. **Upgrade Dampen**: Update to the latest version that supports the schema version:
   ```bash
   cargo update dampen-core dampen-iced
   ```
2. **Downgrade Schema Version**: Change the version attribute to a supported version:
   ```xml
   <dampen version="1.0">
   ```

#### Error: "Invalid version format"

**Cause**: Version attribute is not in the correct `major.minor` format.

**Example**:
```
Error: Invalid version format '1'. Expected 'major.minor' (e.g., '1.0')
  --> src/ui/window.dampen:1:9
   |
 1 | <dampen version="1">
   |         ^^^^^^^^^^^ invalid format
   |
Suggestion: Use format: version="1.0"
```

**Common Invalid Formats**:
```xml
<!-- ❌ Missing minor version -->
<dampen version="1">

<!-- ❌ Version prefix -->
<dampen version="v1.0">

<!-- ❌ Patch version not supported -->
<dampen version="1.0.5">

<!-- ❌ Prerelease suffix -->
<dampen version="1.0-beta">

<!-- ❌ Non-numeric -->
<dampen version="one.zero">

<!-- ✅ Correct format -->
<dampen version="1.0">
```

**Solutions**:
- Use the format `major.minor`: `version="1.0"`
- Don't include prefixes (v), suffixes (-beta), or patch numbers (.5)
- Both parts must be numeric

#### Warning: File has no version attribute

**Note**: This is currently allowed (defaults to v1.0), but explicit versioning is recommended.

**Example**:
```xml
<!-- Implicit version 1.0 (works but not recommended) -->
<dampen>
    <column><text value="Hello" /></column>
</dampen>
```

**Recommendation**:
```xml
<!-- Explicit version (recommended) -->
<dampen version="1.0">
    <column><text value="Hello" /></column>
</dampen>
```

**Why explicit is better**:
- Makes schema version intent clear
- Easier migration when new versions are released
- Consistent with best practices
- Future versions may warn about missing version

#### Warning: "Widget requires higher schema version"

**Cause**: You're using a widget that was introduced in a newer schema version than your document declares.

**Example**:
```
Warning: Widget 'canvas' requires schema v1.1 but document declares v1.0 in src/ui/window.dampen:153:21
  Suggestion: Update to <dampen version="1.1"> or remove this widget
```

**What this means**:
- The widget you're using requires a schema version newer than what your document declares
- Currently affects: **Canvas** widget (requires v1.1, experimental/non-functional)
- All other widgets are v1.0 and fully functional

**Example file with warning**:
```xml
<dampen version="1.0">
    <column>
        <!-- Canvas requires v1.1 but document declares v1.0 -->
        <canvas width="400" height="200" program="{chart}" />
    </column>
</dampen>
```

**Solutions**:

1. **Upgrade schema version** (when v1.1 is officially supported):
   ```xml
   <dampen version="1.1">
       <canvas width="400" height="200" program="{chart}" />
   </dampen>
   ```

2. **Replace with compatible widget**:
   Use v1.0 alternatives (image, custom drawing with image bindings)

3. **Ignore warning** (if you know what you're doing):
   The warning is informational. Canvas will be parsed but may not render correctly since it's experimental.

**Why warnings instead of errors**:
- Allows gradual migration to new schema versions
- Developers can test experimental widgets before official v1.1 release
- Non-blocking for development workflows

**Check widget versions**:
```bash
dampen check --show-widget-versions
```

This displays a table of all widgets with their minimum required versions:
```
Widget               Min Version Status
-------------------- ---------- ------------------------------
canvas               1.1        Experimental (not fully functional)
column               1.0        Stable
button               1.0        Stable
...
```

### Validation Commands

**Check all `.dampen` files in your project**:
```bash
dampen check
```

**Check a specific file**:
```bash
dampen check src/ui/window.dampen
```

**Check with verbose output**:
```bash
dampen check --verbose
```

**Show widget version requirements**:
```bash
dampen check --show-widget-versions
```

### Common Issues

#### Issue: "File works in development but fails in CI"

**Possible Causes**:
1. Different Dampen versions between local and CI
2. Missing version attribute causing version mismatch

**Solutions**:
- Pin Dampen version in `Cargo.toml`:
  ```toml
  [dependencies]
  dampen-core = "=0.2.0"
  dampen-iced = "=0.2.0"
  ```
- Always declare `version` explicitly in all `.dampen` files
- Run `dampen check` as part of CI pipeline

#### Issue: "Mixed version errors across files"

**Cause**: Different `.dampen` files using different schema versions.

**Solution**: Standardize all files to the same version:
```bash
# Find all .dampen files without version 1.0
grep -r '<dampen' src/ examples/ | grep -v 'version="1.0"'

# Update all to use version 1.0
# (Manual edit or search-replace in your editor)
```

### Getting Help

If you encounter version-related issues not covered here:

1. **Check the version**: Verify your Dampen installation version:
   ```bash
   cargo tree | grep dampen-core
   ```

2. **Read the quickstart**: See `docs/QUICKSTART.md` for version usage examples

3. **Report bugs**: If you believe the version validation is incorrect:
   - GitHub Issues: https://github.com/dampen-ui/dampen/issues
   - Include: Error message, `.dampen` file content, Dampen version
