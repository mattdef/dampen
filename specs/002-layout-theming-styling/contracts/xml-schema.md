# XML Schema: Layout, Sizing, Theming, and Styling

**Feature**: 002-layout-theming-styling  
**Version**: 1.0.0  
**Date**: 2026-01-01

## Overview

This document defines the XML schema extensions for layout, sizing, theming, and styling attributes in Gravity `.gravity` files. All attributes are optional unless marked **required**.

---

## Layout Attributes

### Container Layout (`<column>`, `<row>`, `<container>`)

| Attribute | Type | Values | Default | Description |
|-----------|------|--------|---------|-------------|
| `padding` | String | `"<all>"` \| `"<v> <h>"` \| `"<t> <r> <b> <l>"` | `"0"` | Inner spacing (pixels) |
| `spacing` | Number | `0.0+` | `0.0` | Gap between children (pixels) |
| `align_items` | Enum | `start` \| `center` \| `end` \| `stretch` | `stretch` | Cross-axis child alignment |
| `justify_content` | Enum | `start` \| `center` \| `end` \| `space_between` \| `space_around` \| `space_evenly` | `start` | Main-axis child distribution |
| `direction` | Enum | `horizontal` \| `horizontal_reverse` \| `vertical` \| `vertical_reverse` | Context-specific | Layout direction |

**Examples:**
```xml
<column padding="20" spacing="10" align_items="center">
  <text value="Centered in column" />
</column>

<row padding="16 24" justify_content="space_between">
  <button label="Left" />
  <button label="Right" />
</row>

<container padding="10 20 30 40">
  <!-- top: 10, right: 20, bottom: 30, left: 40 -->
</container>
```

---

## Sizing Attributes

### All Widgets

| Attribute | Type | Values | Default | Description |
|-----------|------|--------|---------|-------------|
| `width` | Length | `<number>` \| `fill` \| `shrink` \| `fill_portion(<n>)` \| `<n>%` | `shrink` | Horizontal size |
| `height` | Length | `<number>` \| `fill` \| `shrink` \| `fill_portion(<n>)` \| `<n>%` | `shrink` | Vertical size |
| `min_width` | Number | `0.0+` | - | Minimum width constraint |
| `max_width` | Number | `0.0+` | - | Maximum width constraint |
| `min_height` | Number | `0.0+` | - | Minimum height constraint |
| `max_height` | Number | `0.0+` | - | Maximum height constraint |

**Length Values:**
- `<number>`: Fixed pixel value (e.g., `"200"`)
- `fill`: Expand to fill available space
- `shrink`: Minimize to content size
- `fill_portion(<n>)`: Proportional fill (e.g., `fill_portion(2)` takes 2x space vs `fill_portion(1)`)
- `<n>%`: Percentage of parent (e.g., `"50%"` = half of parent width/height)

**Examples:**
```xml
<button width="200" height="50" label="Fixed Size" />

<text width="fill" value="Fills available width" />

<container width="80%" max_width="800">
  <!-- 80% of parent, capped at 800px -->
</container>

<row>
  <button width="fill_portion(1)" label="1x" />
  <button width="fill_portion(2)" label="2x" />
  <!-- Second button is 2x wider than first -->
</row>
```

---

## Alignment Attributes

### Individual Widget Alignment

| Attribute | Type | Values | Default | Description |
|-----------|------|--------|---------|-------------|
| `align_self` | Enum | `start` \| `center` \| `end` \| `stretch` | Inherit from parent | Override parent `align_items` |

**Example:**
```xml
<column align_items="start">
  <text value="Left-aligned" />
  <text align_self="center" value="Centered" />
  <text align_self="end" value="Right-aligned" />
</column>
```

---

## Styling Attributes

### Background

| Attribute | Type | Values | Description |
|-----------|------|--------|-------------|
| `background` | Color \| Gradient \| Image | See formats below | Widget background |

**Color Formats:**
- Hex: `"#3498db"`, `"#3498dbff"` (with alpha)
- RGB: `"rgb(52, 152, 219)"`, `"rgba(52, 152, 219, 0.8)"`
- HSL: `"hsl(204, 70%, 53%)"`, `"hsla(204, 70%, 53%, 0.8)"`
- Named: `"red"`, `"blue"`, `"transparent"`, etc.

**Gradient Syntax:**
```
linear-gradient(<angle>, <color-stop>, <color-stop>, ...)

<angle> ::= <number>deg | <number>rad | <number>turn
<color-stop> ::= <color> [<percentage>]?
```

**Examples:**
```xml
<container background="#3498db">
  <text value="Blue background" />
</container>

<button background="linear-gradient(90deg, #3498db, #2ecc71)" label="Gradient" />

<container background="linear-gradient(45deg, red 0%, yellow 50%, green 100%)">
  <text value="Color stops" />
</container>
```

### Foreground Color

| Attribute | Type | Values | Description |
|-----------|------|--------|-------------|
| `color` | Color | Same as background | Text/foreground color |

**Example:**
```xml
<text color="#2c3e50" value="Dark gray text" />
<text color="rgba(255, 0, 0, 0.7)" value="Semi-transparent red" />
```

### Border

| Attribute | Type | Values | Default | Description |
|-----------|------|--------|---------|-------------|
| `border_width` | Number | `0.0+` | `0.0` | Border thickness (pixels) |
| `border_color` | Color | Color format | `#000000` | Border color |
| `border_radius` | String | `<all>` \| `<tl> <tr> <br> <bl>` | `0` | Corner radius (pixels) |
| `border_style` | Enum | `solid` \| `dashed` \| `dotted` | `solid` | Border line style |

**Examples:**
```xml
<container border_width="2" border_color="#3498db" border_radius="8">
  <text value="Rounded blue border" />
</container>

<button border_width="1" border_color="#000" border_radius="4 4 0 0" label="Top corners rounded" />
```

### Shadow

| Attribute | Type | Syntax | Description |
|-----------|------|--------|-------------|
| `shadow` | String | `"<offset_x> <offset_y> <blur> <color>"` | Drop shadow |

**Example:**
```xml
<container shadow="2 2 4 #00000040">
  <!-- offset_x: 2px, offset_y: 2px, blur: 4px, color: rgba(0,0,0,0.25) -->
  <text value="Drop shadow" />
</container>
```

### Opacity

| Attribute | Type | Values | Default | Description |
|-----------|------|--------|---------|-------------|
| `opacity` | Number | `0.0` - `1.0` | `1.0` | Widget opacity (0 = transparent, 1 = opaque) |

**Example:**
```xml
<container opacity="0.5">
  <text value="50% transparent" />
</container>
```

### Transform

| Attribute | Type | Syntax | Description |
|-----------|------|--------|-------------|
| `transform` | String | `scale(<n>)` \| `rotate(<deg>)` \| `translate(<x>, <y>)` | Visual transformation |

**Examples:**
```xml
<button transform="scale(1.2)" label="120% size" />
<text transform="rotate(45)" value="Rotated 45°" />
<container transform="translate(10, 20)">
  <!-- Offset 10px right, 20px down -->
</container>
```

---

## Theming Attributes

### Theme Reference

| Attribute | Type | Values | Description |
|-----------|------|--------|-------------|
| `theme` | String | Theme name | Override global theme for this widget and children |

**Example:**
```xml
<column theme="dark">
  <!-- This column and all children use dark theme -->
  <text value="Dark themed" />
</column>
```

### Theme Definition

```xml
<theme name="custom_theme">
  <palette
    primary="#3498db"
    secondary="#2ecc71"
    background="#ecf0f1"
    text="#2c3e50"
    text_secondary="#7f8c8d"
  />
  <typography
    font_family="Roboto"
    font_size_base="16"
    font_weight="normal"
  />
  <spacing unit="4" />
</theme>
```

---

## Style Classes

### Class Attribute

| Attribute | Type | Values | Description |
|-----------|------|--------|-------------|
| `class` | String | Space-separated class names | Apply style classes |

**Example:**
```xml
<button class="primary large" label="Styled Button" />
<container class="card">
  <text value="Card content" />
</container>
```

### Class Definition

```xml
<style name="button_primary">
  <base
    background="#3498db"
    color="#ffffff"
    padding="12 24"
    border_radius="4"
  />
  <hover background="#2980b9" />
  <active background="#21618c" />
  <disabled opacity="0.5" />
</style>

<style name="card" extends="panel">
  <base
    background="#ffffff"
    padding="20"
    border_width="1"
    border_color="#ddd"
    border_radius="8"
    shadow="0 2 4 #00000010"
  />
</style>
```

**Class Attributes:**
- `name` (**required**): Unique class identifier
- `extends`: Space-separated list of parent classes (max depth 5)

**State Variants:**
- `<hover>`: Applied when mouse hovers over widget
- `<focus>`: Applied when widget has keyboard focus
- `<active>`: Applied when widget is pressed
- `<disabled>`: Applied when `disabled="true"`

---

## Responsive Attributes

### Breakpoint Prefixes

Prefix any attribute with `mobile:`, `tablet:`, or `desktop:` to apply only at that breakpoint.

**Breakpoints:**
- `mobile`: Viewport width < 640px
- `tablet`: Viewport width 640px - 1024px
- `desktop`: Viewport width >= 1024px

**Examples:**
```xml
<column 
  mobile:spacing="8" 
  tablet:spacing="12" 
  desktop:spacing="20"
  mobile:padding="16"
  desktop:padding="32">
  
  <text 
    mobile:size="14" 
    desktop:size="18" 
    value="Responsive text" />
  
  <row 
    mobile:direction="vertical"
    desktop:direction="horizontal">
    <button label="Button 1" />
    <button label="Button 2" />
  </row>
</column>
```

**Resolution Order:**
- Breakpoint-specific attribute overrides base attribute
- Most specific breakpoint wins
- Falls back to base if no breakpoint match

---

## State-Based Styling (Inline)

### State Prefixes

Prefix style attributes with `hover:`, `focus:`, `active:`, or `disabled:` for state-specific styling.

**Examples:**
```xml
<button
  background="#3498db"
  hover:background="#2980b9"
  active:background="#21618c"
  disabled:opacity="0.5"
  label="Stateful Button"
/>

<text_input
  border_color="#ddd"
  focus:border_color="#3498db"
  placeholder="Focus for blue border"
/>

<container
  transform="scale(1.0)"
  hover:transform="scale(1.05)"
  active:transform="scale(0.95)">
  <text value="Hover and click me" />
</container>
```

**Combined States:**
```xml
<button
  hover:active:background="#1a5276"
  label="Hover + Active"
/>
```

---

## Complete Example

```xml
<gravity version="1.0">
  <!-- Theme definitions -->
  <themes>
    <theme name="light">
      <palette
        primary="#3498db"
        background="#ecf0f1"
        text="#2c3e50"
      />
      <typography font_family="Roboto" font_size_base="16" />
      <spacing unit="4" />
    </theme>
  </themes>
  
  <!-- Style class definitions -->
  <styles>
    <style name="card">
      <base
        background="#ffffff"
        padding="20"
        border_radius="8"
        shadow="0 2 4 #00000010"
      />
    </style>
    
    <style name="button_primary">
      <base
        background="#3498db"
        color="#ffffff"
        padding="12 24"
        border_radius="4"
      />
      <hover background="#2980b9" />
      <active background="#21618c" />
    </style>
  </styles>
  
  <!-- UI definition -->
  <container 
    theme="light"
    class="card"
    mobile:padding="16"
    desktop:padding="32"
    width="80%"
    max_width="800">
    
    <column spacing="20" align_items="center">
      <text
        mobile:size="18"
        desktop:size="24"
        color="#2c3e50"
        value="Styled Application"
      />
      
      <row 
        mobile:direction="vertical"
        desktop:direction="horizontal"
        spacing="10">
        
        <button
          class="button_primary"
          width="fill"
          label="Primary Action"
          on_click="primary_action"
        />
        
        <button
          background="transparent"
          color="#3498db"
          border_width="2"
          border_color="#3498db"
          hover:background="rgba(52, 152, 219, 0.1)"
          width="fill"
          label="Secondary"
          on_click="secondary_action"
        />
      </row>
    </column>
  </container>
</gravity>
```

---

## Validation Rules

### General
- All color values must be valid CSS color strings
- All numeric values must be non-negative (unless specified otherwise)
- Class names must exist in `<styles>` section
- Theme names must exist in `<themes>` section or be built-in (`light`, `dark`, `default`)

### Constraints
- `min_width` ≤ `max_width` (if both specified)
- `min_height` ≤ `max_height` (if both specified)
- `opacity` must be 0.0 - 1.0
- `fill_portion` must be 1-255
- Gradient must have 2-8 color stops
- Gradient color stop offsets must be 0.0-1.0 and sorted
- Style class inheritance depth ≤ 5 levels
- No circular class dependencies

### Parse-Time Errors

**Invalid color:**
```xml
<text color="#ggg" value="Invalid" />
<!-- Error: Invalid color '#ggg': not a valid hex color -->
```

**Invalid gradient:**
```xml
<container background="linear-gradient(90, red, blue)">
<!-- Error: Invalid gradient angle: '90' (missing unit, expected '90deg') -->
```

**Missing theme:**
```xml
<column theme="nonexistent">
<!-- Error: Theme 'nonexistent' not found. Available: light, dark, custom_theme -->
```

**Circular class dependency:**
```xml
<style name="A" extends="B" />
<style name="B" extends="A" />
<!-- Error: Circular style class dependency detected: A → B → A -->
```

---

## Migration from Previous Version

No breaking changes - all new attributes are optional. Existing `.gravity` files continue to work without modification.

**Opt-in adoption:**
1. Add layout attributes: `padding`, `spacing`, `width`, `height`
2. Add styling attributes: `background`, `color`, `border_*`
3. Define themes in `<themes>` section
4. Define reusable classes in `<styles>` section
5. Add responsive variants with `mobile:`, `tablet:`, `desktop:` prefixes

---

## Schema Version

**Current**: `1.0.0`  
**Gravity XML Namespace**: `http://gravity-ui.org/schema/1.0`  
**Compatibility**: Gravity Framework v0.2.0+
