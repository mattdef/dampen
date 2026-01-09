# Dampen Styling System

**Version**: 1.0.0  
**Last Updated**: 2026-01-02

This guide covers the complete styling system for Dampen UI, including themes, style classes, inline styles, and state-based styling.

---

## Table of Contents

1. [Themes](#themes)
2. [Inline Styles](#inline-styles)
3. [Style Classes](#style-classes)
4. [State-Based Styling](#state-based-styling)
5. [Responsive Design](#responsive-design)
6. [Best Practices](#best-practices)

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
                font_size_large="24"
                font_weight="normal"
                line_height="1.5" />
            <spacing unit="8" />
        </theme>
    </themes>
    
    <global_theme name="custom" />
    
    <!-- Your UI here -->
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
- `font_weight`: normal, bold, etc.
- `line_height`: Line height multiplier

### Spacing

Defines the spacing scale used throughout the app.

```xml
<spacing unit="8" />
```

All spacing values are multiples of this unit.

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
| `border_radius` | Length | `4` | Corner rounding |
| `shadow` | Shadow | `2 2 4 #00000040` | Offset-x offset-y blur color |
| `opacity` | Float | `0.8` | Transparency (0.0-1.0) |
| `width` | Length | `200`, `fill`, `shrink` | Widget width |
| `height` | Length | `100`, `fill`, `shrink` | Widget height |
| `spacing` | Length | `10` | Child spacing |

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

---

## Style Classes

Define reusable styles that can be applied to multiple widgets.

### Defining Classes

```xml
<style_classes>
    <!-- Base button style -->
    <style name="button_base" 
        padding="12 24" 
        border_radius="6" 
        border_width="2" 
        background="#ffffff"
        color="#2c3e50" />
    
    <!-- Primary button - extends base -->
    <style name="button_primary" 
        extends="button_base"
        background="#3498db"
        color="#ffffff"
        border_color="#2980b9" />
    
    <!-- Danger button -->
    <style name="button_danger" 
        extends="button_base"
        background="#e74c3c"
        color="#ffffff"
        border_color="#c0392b" />
    
    <!-- Card style -->
    <style name="card" 
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

### Inheritance

Classes can extend other classes:

```xml
<style name="button_primary" extends="button_base">
    <!-- Overrides and additions -->
</style>
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

### Format 1: Child Elements (XML Schema Compliant)

```xml
<style name="button_primary" 
    background="#3498db"
    color="#ffffff"
    padding="12 24"
    border_radius="6">
    <hover background="#2980b9" />
    <active background="#21618c" />
    <disabled opacity="0.5" />
</style>
```

### Format 2: Prefixed Attributes

```xml
<style name="button_primary" 
    background="#3498db"
    color="#ffffff"
    padding="12 24"
    border_radius="6"
    hover_background="#2980b9"
    active_background="#21618c"
    disabled_opacity="0.5" />
```

### Using State Classes

```xml
<style_classes>
    <style name="btn" 
        background="#3498db"
        hover_background="#2980b9"
        active_background="#21618c"
        disabled_opacity="0.5" />
</style_classes>

<column>
    <button class="btn" label="Click Me" on_click="handler" />
    <button class="btn" label="Disabled" disabled="true" />
</column>
```

### Inline State Styles

```xml
<button 
    background="#3498db"
    hover_background="#2980b9"
    active_background="#21618c"
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
<style name="responsive_card"
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
<dampen>
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
        <!-- Button with states -->
        <style name="btn" 
            padding="12 24" 
            border_radius="6" 
            border_width="2" 
            background="#3498db"
            color="#ffffff"
            border_color="#2980b9">
            <hover background="#2980b9" />
            <active background="#21618c" />
            <disabled opacity="0.5" />
        </style>
        
        <!-- Card with responsive padding -->
        <style name="card" 
            background="#ffffff" 
            border_radius="8" 
            border_width="1" 
            border_color="#e0e0e0"
            shadow="0 2 4 #00000010"
            mobile:padding="15"
            desktop:padding="25" />
        
        <!-- Danger variant -->
        <style name="btn_danger" 
            extends="btn"
            background="#e74c3c"
            border_color="#c0392b">
            <hover background="#c0392b" />
            <active background="#a52714" />
        </style>
    </style_classes>
    
    <global_theme name="app_theme" />
    
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

## Migration from v1.0

If upgrading from before state-based styling:

**Before:**
```xml
<button background="#3498db" label="Click" />
```

**After:**
```xml
<style_classes>
    <style name="btn" 
        background="#3498db"
        hover_background="#2980b9">
        <active background="#21618c" />
    </style>
</style_classes>

<button class="btn" label="Click" on_click="handler" />
```

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
- `hover_*` - hover state variants
- `focus_*` - focus state variants
- `active_*` - active state variants
- `disabled_*` - disabled state variants

### Widget Attributes

All style class attributes plus:
- `class` - apply style classes (space-separated)
- `theme_ref` - apply local theme
- `disabled` - boolean to disable widget
- `mobile:*`, `tablet:*`, `desktop:*` - responsive variants

### State Variants

All style properties can have state variants:
- `hover:background`, `hover:color`, `hover:opacity`, etc.
- `focus:border_color`, `focus:shadow`, etc.
- `active:background`, `active:transform`, etc.
- `disabled:opacity`, `disabled:color`, etc.

---

## Troubleshooting

### State styles not applying
- Check that widget has an `id` attribute
- Verify state prefix is correct (`hover:`, not `hover_`)
- Check browser/dev tools for errors

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
