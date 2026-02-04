# Dampen Styling System

**Version**: 1.2.0  
**Last Updated**: 2026-01-30

This guide covers the complete styling system for Dampen UI, including themes, style classes, inline styles, and state-based styling.

---

## Table of Contents

1. [Themes](#themes)
2. [Global Theme File](#global-theme-file)
3. [Inline Styles](#inline-styles)
4. [Layout & Alignment](#layout--alignment)
5. [Style Classes](#style-classes)
6. [State-Based Styling](#state-based-styling)
7. [Responsive Design](#responsive-design)
8. [Best Practices](#best-practices)

---

## Themes

Themes provide a consistent look and feel across your application.

### Defining a Theme

```xml
<dampen>
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
                font_size_large="20"
                font_weight="normal"
                line_height="1.5" />
            <spacing unit="8" />
        </theme>
    </themes>
    
    <default_theme name="custom" />
</dampen>
```

### Palette Colors

| Color | Usage |
|-------|-------|
| `primary` | Main brand color, primary actions |
| `secondary` | Secondary actions, accents |
| `success` | Success messages, positive actions |
| `warning` | Warnings, cautionary actions |
| `danger` | Errors, destructive actions |
| `background` | Page/app background |
| `surface` | Card/container backgrounds |
| `text` | Primary text color |
| `text_secondary` | Secondary/muted text |

### Typography

- `font_family`: Font stack (e.g., "Inter, sans-serif")
- `font_size_base`: Base size in px (e.g., "16")
- `font_size_small`: Small text size
- `font_size_large`: Large text size
- `font_weight`: thin, light, normal, medium, bold, black
- `line_height`: Line height multiplier

### Spacing

Defines the spacing scale used throughout the app.

```xml
<spacing unit="8" />
```

All spacing values are multiples of this unit.

### Custom Themes with Inheritance

Themes can inherit from other themes using the `extends` attribute.

```xml
<dampen>
    <themes>
        <!-- Base theme with all colors defined -->
        <theme name="base">
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
                font_size_large="20"
                font_weight="normal"
                line_height="1.5" />
            <spacing unit="8" />
        </theme>

        <!-- Dark theme - only override what's different -->
        <theme name="dark" extends="base">
            <palette
                background="#2c3e50"
                surface="#34495e"
                text="#ecf0f1"
                text_secondary="#95a5a6" />
        </theme>

        <!-- High contrast variant of dark -->
        <theme name="dark_contrast" extends="dark">
            <palette
                primary="#5dade2"
                secondary="#58d68d" />
        </theme>
    </themes>

    <default_theme name="dark" />
</dampen>
```

#### Inheritance Rules

1. **Child theme values take precedence** - Any property defined in the child overrides the parent's value
2. **Missing values are inherited** - Properties not defined in the child use the parent's value
3. **Maximum depth: 5 levels** - Prevents circular dependencies
4. **No circular inheritance** - Theme A cannot extend B if B extends A

#### Validation

Dampen validates theme inheritance at parse time:

```xml
<!-- Circular inheritance - will fail validation -->
<theme name="a" extends="b">
<theme name="b" extends="a">
```

Error message:
```
THEME_007: Circular theme inheritance detected: a → b → a
```

```xml
<!-- Missing parent theme - will fail validation -->
<theme name="child" extends="nonexistent">
```

Error message:
```
THEME_006: Parent theme 'nonexistent' not found for theme 'child'
```

---

## Global Theme File

For applications with complex theming needs, you can define themes in a separate `theme.dampen` file.

### File Location

Place your theme file at:

```
src/ui/theme/theme.dampen
```

This location is:
- Automatically discovered by Dampen
- Watched for hot-reload in development mode
- Shared across all windows in your application

### Creating a Theme File

```xml
<?xml version="1.0" encoding="UTF-8" ?>
<dampen version="1.0">
    <themes>
        <theme name="light">
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
                font_size_large="20" />
            <spacing unit="8" />
        </theme>

        <theme name="dark" extends="light">
            <palette
                background="#1a1a2e"
                surface="#16213e"
                text="#eaeaea"
                text_secondary="#a0a0a0" />
        </theme>
    </themes>

    <default_theme name="light" />
    <follow_system enabled="true" />
</dampen>
```

### Default Theme Selection

The `<default_theme>` element specifies which theme to use when no preference is set:

```xml
<default_theme name="light" />
```

### System Theme Detection

Enable automatic dark/light mode detection:

```xml
<follow_system enabled="true" />
```

When enabled, Dampen will:
1. Detect the system's dark/light preference at startup
2. Use matching theme name ("dark" or "light") if available
3. Fall back to `default_theme` if system preference doesn't match any theme

### Runtime Theme Switching

Switch themes at runtime using the `set_theme()` handler action:

```xml
<row spacing="10">
    <button label="Light" on_click="set_theme('light')" />
    <button label="Dark" on_click="set_theme('dark')" />
</row>
```

Or use data binding to switch themes reactively:

```rust
#[derive(UiModel)]
struct Model {
    current_theme: String,
}
```

```xml
<window theme="{current_theme}">
    <!-- content -->
</window>
```

### Project Structure with Theme File

```
my-app/
├── src/
│   ├── main.rs
│   └── ui/
│       ├── mod.rs
│       ├── window.dampen      # UI without inline themes
│       └── theme/
│           └── theme.dampen   # All theme definitions
└── Cargo.toml
```

### Backward Compatibility

If `src/ui/theme/theme.dampen` doesn't exist:
- Existing inline `<themes>` sections continue to work
- Applications without theming use Iced's default theme
- No changes required to existing applications

---

## Inline Styles

Override theme defaults directly on widgets.

### Basic Properties

```xml
<button 
    background="#e74c3c"
    color="#ffffff"
    padding="12 24"
    border_radius="4"
    border_width="2"
    border_color="#c0392b"
    width="120" />
```

### Supported Properties

| Property | Type | Example | Description |
|----------|------|---------|-------------|
| `background` | Color/Gradient | `#ffffff`, `linear-gradient(...)` | Background fill |
| `color` | Color | `#000000` | Text color |
| `padding` | Spacing | `10 20`, `10 20 30 40` | Padding (top/right/bottom/left) |
| `border_width` | Length | `2` | Border thickness |
| `border_color` | Color | `#000000` | Border color |
| `border_radius` | Length | `4` or `4 4 4 4` | Corner rounding |
| `border_style` | String | `solid`, `dashed`, `dotted` | Border style |
| `shadow` | Shadow | `2 2 4 #00000040` | Offset-x offset-y blur color |
| `opacity` | Float | `0.8` | Transparency (0.0-1.0) |
| `transform` | Transform | `scale(1.2)`, `rotate(45)` | Transformations |
| `width` | Length | `200`, `fill`, `shrink` | Widget width |
| `height` | Length | `100`, `fill`, `shrink` | Widget height |
| `spacing` | Length | `10` | Child spacing |
| `align_x` | Alignment | `start`, `center`, `end` | Horizontal alignment |
| `align_y` | Alignment | `start`, `center`, `end` | Vertical alignment |

### Length Values

- Fixed: `200` (pixels)
- Fill: `fill` (expand to fill)
- Shrink: `shrink` (fit content)
- Fill Portion: `fill_portion(2)` (flex ratio)
- Percentage: `50%` (relative to parent)

### Color Formats

```xml
<!-- Hex -->
background="#ff0000"

<!-- RGB -->
background="rgb(255, 0, 0)"

<!-- RGBA -->
background="rgba(255, 0, 0, 0.5)"

<!-- Named colors -->
background="red"
```

### Gradients

```xml
<!-- Linear -->
background="linear-gradient(90deg, #ff0000 0%, #0000ff 100%)"

<!-- Radial -->
background="radial-gradient(circle, #ff0000 0%, #0000ff 100%)"
```

### Border Radius

```xml
<!-- Single value (all corners) -->
border_radius="8"

<!-- Four values (top-left, top-right, bottom-right, bottom-left) -->
border_radius="8 8 8 8"
```

### Transform

```xml
<!-- Scale -->
transform="scale(1.2)"
transform="scale(1.2, 0.8)"

<!-- Rotate -->
transform="rotate(45)"

<!-- Translate -->
transform="translate(10, 20)"

<!-- Matrix -->
transform="matrix(1,0,0,1,0,0)"
```

---

## Layout & Alignment

Control the positioning of widgets and their children.

### Alignment Values

All alignment properties accept these values:

| Value | Description |
|-------|-------------|
| `start` | Align to start (left for horizontal, top for vertical) |
| `center` | Center alignment |
| `end` | Align to end (right for horizontal, bottom for vertical) |
| `stretch` | Stretch to fill available space |

### Container Alignment

Center content within a container:

```xml
<container width="200" height="100" align_x="center" align_y="center">
    <text value="Centered Content" />
</container>
```

### Column Alignment

Align children horizontally within a column:

```xml
<column width="fill" align_x="center">
    <text value="Centered Text" />
    <button label="Centered Button" />
</column>
```

### Row Alignment

Align children vertically within a row:

```xml
<row height="fill" align_y="center">
    <text value="Vertically Centered" />
    <button label="Aligned" />
</row>
```

### Text Alignment

Align text content within its bounds:

```xml
<container width="200">
    <text value="Right Aligned" align_x="end" />
    <text value="Centered" align_x="center" />
</container>
```

### Alignment by Widget Type

| Widget | `align_x` | `align_y` |
|--------|-----------|-----------|
| `container` | Horizontal position of child | Vertical position of child |
| `column` | Horizontal alignment of children | Vertical alignment within container |
| `row` | Horizontal alignment within container | Vertical alignment of children |
| `text` | Text horizontal alignment | Text vertical alignment |

### Layout Attributes

```xml
<container
    width="fill"
    height="200"
    min_width="100"
    max_width="500"
    padding="20"
    position="relative"
    top="10"
    left="5"
    z_index="1"
    direction="horizontal">
    <!-- content -->
</container>
```

### Flexbox-like Properties

```xml
<column
    align_items="center"
    justify_content="space_between"
    direction="vertical">
    <text value="Item 1" />
    <text value="Item 2" />
    <text value="Item 3" />
</column>
```

---

## Style Classes

Define reusable styles that can be applied to multiple widgets.

### Defining Classes

```xml
<style_classes>
    <!-- Base button style -->
    <class name="button_base" 
        padding="12 24" 
        border_radius="6" 
        border_width="2" 
        background="#ffffff"
        color="#2c3e50" />
    
    <!-- Primary button - extends base -->
    <class name="button_primary" 
        extends="button_base"
        background="#3498db"
        color="#ffffff"
        border_color="#2980b9" />
    
    <!-- Danger button -->
    <class name="button_danger" 
        extends="button_base"
        background="#e74c3c"
        color="#ffffff"
        border_color="#c0392b" />
    
    <!-- Card style -->
    <class name="card" 
        background="#ffffff" 
        padding="20" 
        border_radius="8" 
        border_width="1" 
        border_color="#e0e0e0"
        shadow="0 2 4 #00000010" />
</style_classes>
```

### Applying Classes

```xml
<column>
    <button class="button_primary" label="Save" on_click="save" />
    <button class="button_danger" label="Delete" on_click="delete" />
    
    <container class="card">
        <text value="Card content" />
    </container>
</column>
```

### Multiple Classes

```xml
<button class="button_primary large bold" label="Submit" />
```

Classes are merged in order (later classes override earlier ones).

### Class Inheritance

Classes can extend other classes:

```xml
<class name="button_primary" extends="button_base">
    <!-- Overrides and additions -->
</class>
```

Maximum inheritance depth: 5 levels.

---

## State-Based Styling

Automatically change appearance based on user interaction.

### State Variants

Four states are supported:
- `hover`: Mouse over widget
- `focus`: Keyboard focus (inputs)
- `active`: Mouse button pressed
- `disabled`: Widget is disabled

### Format 1: Child Elements

```xml
<class name="button_primary" 
    background="#3498db"
    color="#ffffff"
    padding="12 24"
    border_radius="6">
    <hover background="#2980b9" />
    <active background="#21618c" />
    <disabled opacity="0.5" />
</class>
```

### Format 2: Colon Prefix (Recommended)

The parser automatically preprocesses colon-prefixed attributes:

```xml
<class name="button_primary"
    background="#3498db"
    color="#ffffff"
    padding="12 24"
    border_radius="6"
    hover:background="#2980b9"
    active:background="#21618c"
    disabled:opacity="0.5" />
```

**Note:** The parser converts `hover:background` to `hover_state_background` internally to avoid XML namespace issues.

### Format 3: Underscore State Prefix

Use the `_state_` separator for direct state attributes:

```xml
<class name="button_primary"
    background="#3498db"
    color="#ffffff"
    padding="12 24"
    border_radius="6"
    hover_state_background="#2980b9"
    active_state_background="#21618c"
    disabled_state_opacity="0.5" />
```

### Combined States

Support for multiple states simultaneously:

```xml
<!-- Colon syntax (recommended) -->
<button label="Click"
    background="#3498db"
    hover:active:background="#1a5276" />

<!-- Underscore state syntax -->
<button label="Click"
    background="#3498db"
    hover_state_active_state_background="#1a5276" />
```

### Inline State Styles

```xml
<button
    background="#3498db"
    hover:background="#2980b9"
    active:background="#21618c"
    label="Interactive" />
```

Or using underscore syntax:

```xml
<button
    background="#3498db"
    hover_state_background="#2980b9"
    active_state_background="#21618c"
    label="Interactive" />
```

### Disabled State

```xml
<button 
    class="btn"
    disabled="true"
    on_click="handler" />
```

When disabled:
- `disabled` state style applies
- Event handlers are prevented from firing
- Visual feedback shows disabled appearance

---

## Responsive Design

Apply different styles based on viewport size.

### Breakpoints

- `mobile`: < 640px
- `tablet`: 640px - 1024px
- `desktop`: > 1024px

### Breakpoint-Prefixed Attributes

```xml
<column 
    mobile:spacing="10"
    tablet:spacing="15"
    desktop:spacing="20">
    
    <text 
        mobile:size="18"
        tablet:size="24"
        desktop:size="32"
        value="Responsive Text" />
</column>
```

### Breakpoint-Prefixed Classes

```xml
<class name="responsive_card"
    mobile:padding="10"
    tablet:padding="15"
    desktop:padding="20" />
```

### How It Works

1. Viewport width is tracked
2. When crossing breakpoint thresholds, attributes are updated
3. Only changed attributes trigger re-render (performance optimized)

---

## Complete Example

```xml
<?xml version="1.0" encoding="UTF-8"?>
<dampen version="1.0">
    <!-- Theme Definition -->
    <themes>
        <theme name="app_theme">
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
                font_size_base="16" />
            <spacing unit="8" />
        </theme>
    </themes>
    
    <!-- Style Classes -->
    <style_classes>
        <!-- Button with states using colon syntax -->
        <class name="btn" 
            padding="12 24" 
            border_radius="6" 
            border_width="2" 
            background="#3498db"
            color="#ffffff"
            border_color="#2980b9"
            hover:background="#2980b9"
            active:background="#21618c"
            disabled:opacity="0.5" />
        
        <!-- Card with responsive padding -->
        <class name="card" 
            background="#ffffff" 
            border_radius="8" 
            border_width="1" 
            border_color="#e0e0e0"
            shadow="0 2 4 #00000010"
            mobile:padding="15"
            desktop:padding="25" />
        
        <!-- Danger variant using underscore state syntax -->
        <class name="btn_danger" 
            extends="btn"
            background="#e74c3c"
            border_color="#c0392b"
            hover_state_background="#c0392b"
            active_state_background="#a52714" />
    </style_classes>
    
    <default_theme name="app_theme" />
    
    <!-- UI -->
    <column padding="40" spacing="20">
        <text value="State-Based Styling Demo" size="32" weight="bold" />
        
        <container class="card" spacing="12">
            <text value="Interactive Buttons" size="20" weight="bold" />
            <row spacing="12">
                <button class="btn" label="Primary" on_click="primary" />
                <button class="btn_danger" label="Danger" on_click="danger" />
                <button class="btn" label="Disabled" disabled="true" />
            </row>
        </container>
        
        <container class="card" spacing="12">
            <text value="Responsive Container" size="20" weight="bold" />
            <text value="Resize window to see padding change" color="#7f8c8d" />
        </container>
    </column>
</dampen>
```

---

## Best Practices

### 1. Use Themes for Consistency
Define your color palette and typography once in themes.

### 2. Use Classes for Reusability
Create classes for common patterns (buttons, cards, inputs).

### 3. Use State Styles for UX
Always provide hover/active feedback for interactive elements.

### 4. Use Inline Styles Sparingly
Only use inline styles for one-off exceptions.

### 5. Test All States
Verify hover, focus, active, and disabled states work correctly.

### 6. Consider Accessibility
- Ensure sufficient color contrast
- Don't rely solely on color for information
- Provide focus indicators

### 7. Performance
- Minimize inline styles
- Use classes for shared styles
- Breakpoint changes are optimized

---

## API Reference

### Theme Attributes

All theme attributes can be used in:
- `<palette>` - color definitions
- `<typography>` - text styling
- `<spacing>` - spacing scale

### Style Class Attributes

All inline style attributes plus:
- `extends` - inherit from another class
- `hover:*` - hover state variants (e.g., `hover:background="#..."`)
- `focus:*` - focus state variants (e.g., `focus:border_color="#..."`)
- `active:*` - active state variants (e.g., `active:background="#..."`)
- `disabled:*` - disabled state variants (e.g., `disabled:opacity="0.5"`)

### Widget Attributes

All style class attributes plus:
- `class` - apply style classes (space-separated)
- `theme` - apply theme to widget
- `disabled` - boolean to disable widget
- `mobile:*`, `tablet:*`, `desktop:*` - responsive variants
- `align_x` - horizontal alignment (start, center, end)
- `align_y` - vertical alignment (start, center, end)
- `align_items` - flex alignment of children
- `justify_content` - flex justification of children
- `align_self` - override alignment for single child
- `direction` - layout direction (horizontal, vertical)
- `position` - positioning mode (relative, absolute)
- `top`, `right`, `bottom`, `left` - position offsets
- `z_index` - stacking order

### State Variants

All style properties can have state variants using either syntax:

**Colon syntax (recommended):**
- `hover:background`, `hover:color`, `hover:opacity`, etc.
- `focus:border_color`, `focus:shadow`, etc.
- `active:background`, `active:transform`, etc.
- `disabled:opacity`, `disabled:color`, etc.
- Combined: `hover:active:background`, etc.

**Underscore state syntax:**
- `hover_state_background`, `hover_state_color`, etc.
- `focus_state_border_color`, etc.
- `active_state_background`, etc.
- `disabled_state_opacity`, etc.
- Combined: `hover_state_active_state_background`, etc.

### Widget-Specific Styling

#### Progress Bar Styling

The `progress_bar` widget supports extensive customization:

**Predefined Styles:**
```xml
<!-- Using theme-based styles -->
<progress_bar value="{progress}" style="primary" />
<progress_bar value="{progress}" style="success" />
<progress_bar value="{progress}" style="warning" />
<progress_bar value="{progress}" style="danger" />
<progress_bar value="{progress}" style="secondary" />
```

**Custom Colors:**
```xml
<!-- Hex colors -->
<progress_bar 
    value="{progress}" 
    bar_color="#FF8C00" 
    background_color="#E0E0E0" 
/>

<!-- RGB/RGBA colors -->
<progress_bar 
    value="{progress}" 
    bar_color="rgb(0, 200, 100)" 
    background_color="rgba(50, 50, 50, 0.8)" 
/>

<!-- With alpha channel -->
<progress_bar 
    value="{progress}" 
    bar_color="#00FF0080" 
    background_color="#00000040" 
/>
```

**Border Radius (Rounded Corners):**
```xml
<!-- Subtle rounding -->
<progress_bar value="{progress}" border_radius="4" />

<!-- Medium rounding -->
<progress_bar value="{progress}" border_radius="10" />

<!-- Full rounding (pill shape) -->
<progress_bar value="{progress}" border_radius="20" />
```

**Custom Height:**
```xml
<!-- Thin bar -->
<progress_bar value="{progress}" height="8" />

<!-- Standard height -->
<progress_bar value="{progress}" height="20" />

<!-- Thick bar -->
<progress_bar value="{progress}" height="40" />
```

**Complete Examples:**
```xml
<!-- Modern rounded progress bar -->
<progress_bar 
    value="{progress}" 
    style="success"
    border_radius="10"
    height="12"
/>

<!-- Custom branded progress bar -->
<progress_bar 
    value="{progress}" 
    bar_color="#6366F1"
    background_color="#E0E7FF"
    border_radius="6"
    height="8"
/>

<!-- High contrast progress bar -->
<progress_bar 
    value="{progress}" 
    bar_color="#000000"
    background_color="#FFFFFF"
    border_radius="0"
    height="4"
/>
```

**Color Formats Supported:**
- Hex: `#RRGGBB` (e.g., `#FF5733`)
- Hex with alpha: `#RRGGBBAA` (e.g., `#FF573380`)
- RGB: `rgb(r, g, b)` (e.g., `rgb(255, 87, 51)`)
- RGBA: `rgba(r, g, b, a)` (e.g., `rgba(255, 87, 51, 0.5)`)

**Note:** Custom colors (`bar_color`, `background_color`) override the predefined `style` colors. If both are specified, custom colors take precedence.

### Layout Attributes

- `width`, `height` - dimensions (pixels, fill, shrink, fill_portion(n), %)
- `min_width`, `max_width`, `min_height`, `max_height` - constraints
- `padding` - inner spacing (1-4 values)
- `spacing` - gap between children
- `border_width`, `border_color`, `border_radius`, `border_style` - borders

---

## Troubleshooting

### State styles not applying
- Verify state prefix is correct (`hover:`, not `hover_`)
- When using underscore syntax, use `hover_state_background` (not `hover_background`)
- Check that the state name is valid: `hover`, `focus`, `active`, `disabled`
- Use `--verbose` flag to see parsing errors

### Classes not merging
- Verify class names are space-separated
- Check for typos in class names
- Ensure classes are defined before use

### Responsive not working
- Verify viewport width is being tracked
- Check breakpoint thresholds (640px, 1024px)
- Use `--verbose` flag to see breakpoint changes

---

**Next**: See [XML Schema Reference](XML_SCHEMA.md) for complete attribute list.
