# Data Model: Layout, Sizing, Theming, and Styling System

**Feature**: 002-layout-theming-styling  
**Date**: 2026-01-01  
**Status**: Design Phase

## Overview

This document defines the data structures (IR entities) for layout, sizing, theming, and styling in the Gravity framework. All types are backend-agnostic and defined in `gravity-core`.

---

## Core IR Extensions

### WidgetNode (Extended)

**Location**: `gravity-core/src/ir/node.rs`

```rust
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WidgetNode {
    pub kind: WidgetKind,
    pub id: Option<String>,
    pub attributes: HashMap<String, AttributeValue>,
    pub events: Vec<EventBinding>,
    pub children: Vec<WidgetNode>,
    pub span: Span,
    
    // NEW: Styling extensions
    pub style: Option<StyleProperties>,
    pub layout: Option<LayoutConstraints>,
    pub theme_ref: Option<String>,  // Reference to theme name
    pub classes: Vec<String>,       // Style class names
    pub breakpoint_attributes: HashMap<Breakpoint, HashMap<String, AttributeValue>>,
}
```

**Fields**:
- `style`: Inline style properties (background, border, shadow, opacity, transform)
- `layout`: Layout constraints (sizing, padding, spacing, alignment)
- `theme_ref`: Name of theme to apply (overrides global theme for this subtree)
- `classes`: List of style class names to inherit properties from
- `breakpoint_attributes`: Responsive attributes keyed by breakpoint

**Relationships**:
- Each `WidgetNode` can reference 0-N style classes via `classes`
- Each `WidgetNode` can reference 0-1 theme via `theme_ref`
- `style` and `layout` are resolved by merging: inline > classes > theme > defaults

**Validation Rules**:
- `theme_ref` must exist in `GravityDocument.themes` or be one of built-in themes (`light`, `dark`, `default`)
- `classes` must exist in `GravityDocument.style_classes`
- `style` values (colors, lengths) must be valid per type constraints
- `breakpoint_attributes` can override any attribute except `id`, `kind`, `span`

---

## Layout System Types

### LayoutConstraints

**Location**: `gravity-core/src/ir/layout.rs`

```rust
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct LayoutConstraints {
    pub width: Option<Length>,
    pub height: Option<Length>,
    pub min_width: Option<f32>,
    pub max_width: Option<f32>,
    pub min_height: Option<f32>,
    pub max_height: Option<f32>,
    pub padding: Option<Padding>,
    pub spacing: Option<f32>,
    pub align_items: Option<Alignment>,
    pub justify_content: Option<Justification>,
    pub align_self: Option<Alignment>,
    pub direction: Option<Direction>,
}
```

**Fields**:
- `width`/`height`: Primary sizing (Length enum: Fixed, Fill, Shrink, FillPortion, Percentage)
- `min_width`/`max_width`/`min_height`/`max_height`: Size constraints in pixels
- `padding`: Space inside widget borders (Padding struct)
- `spacing`: Gap between child widgets in containers (pixels)
- `align_items`: Cross-axis alignment for children
- `justify_content`: Main-axis distribution for children
- `align_self`: Override parent's align_items for this widget
- `direction`: Layout direction (horizontal/vertical, normal/reverse)

**Validation Rules**:
- `min_width` ≤ `max_width` (if both specified)
- `min_height` ≤ `max_height` (if both specified)
- `spacing` ≥ 0.0
- `padding` values ≥ 0.0
- `fill_portion` must be positive integer (1-255)
- Percentage values must be 0.0-100.0

### Length

```rust
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Length {
    Fixed(f32),              // Exact pixel value
    Fill,                    // Expand to fill available space
    Shrink,                  // Minimize to content size
    FillPortion(u8),         // Proportional fill (1-255)
    Percentage(f32),         // Percentage of parent (0.0-100.0)
}
```

**Parsing from XML**:
- `"200"` → `Fixed(200.0)`
- `"fill"` → `Fill`
- `"shrink"` → `Shrink`
- `"fill_portion(3)"` → `FillPortion(3)`
- `"50%"` → `Percentage(50.0)`

### Padding

```rust
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Padding {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

impl Padding {
    // Parse from attribute: "10" (all sides) or "10 20" (v h) or "10 20 30 40" (t r b l)
    pub fn parse(s: &str) -> Result<Self, ParseError>;
}
```

### Alignment

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Alignment {
    Start,    // Top for column, left for row
    Center,   // Centered
    End,      // Bottom for column, right for row
    Stretch,  // Fill cross-axis
}
```

### Justification

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Justification {
    Start,         // Pack at start
    Center,        // Pack at center
    End,           // Pack at end
    SpaceBetween,  // First at start, last at end, evenly spaced
    SpaceAround,   // Equal space around each item
    SpaceEvenly,   // Equal space between items
}
```

### Direction

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Direction {
    Horizontal,
    HorizontalReverse,
    Vertical,
    VerticalReverse,
}
```

---

## Styling System Types

### StyleProperties

**Location**: `gravity-core/src/ir/style.rs`

```rust
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct StyleProperties {
    pub background: Option<Background>,
    pub color: Option<Color>,
    pub border: Option<Border>,
    pub shadow: Option<Shadow>,
    pub opacity: Option<f32>,
    pub transform: Option<Transform>,
}
```

**Fields**:
- `background`: Background fill (solid color, gradient, or image)
- `color`: Foreground/text color
- `border`: Border style (width, color, radius, style)
- `shadow`: Drop shadow effect
- `opacity`: Widget opacity (0.0 = transparent, 1.0 = opaque)
- `transform`: Visual transformations (scale, rotate, translate)

**Validation Rules**:
- `opacity` must be 0.0-1.0
- All color values must be valid (validated by `csscolorparser`)

### Background

```rust
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Background {
    Color(Color),
    Gradient(Gradient),
    Image { path: String, fit: ImageFit },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ImageFit {
    Fill,
    Contain,
    Cover,
    ScaleDown,
}
```

### Color

```rust
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Color {
    pub r: f32,  // 0.0-1.0
    pub g: f32,  // 0.0-1.0
    pub b: f32,  // 0.0-1.0
    pub a: f32,  // 0.0-1.0 (alpha)
}

impl Color {
    // Parse from CSS string: "#3498db", "rgb(52, 152, 219)", "rgba(...)", "hsl(...)", "red"
    pub fn parse(s: &str) -> Result<Self, ParseError>;
    
    // Convert to iced::Color
    pub fn to_iced(&self) -> iced::Color;
}
```

### Gradient

```rust
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Gradient {
    Linear {
        angle: f32,  // Degrees (0 = top, 90 = right, 180 = bottom, 270 = left)
        stops: Vec<ColorStop>,
    },
    Radial {
        shape: RadialShape,
        stops: Vec<ColorStop>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ColorStop {
    pub color: Color,
    pub offset: f32,  // 0.0-1.0
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RadialShape {
    Circle,
    Ellipse,
}
```

**Validation Rules**:
- `stops` must have 2-8 color stops (Iced limitation)
- `offset` must be 0.0-1.0 and sorted in ascending order
- `angle` normalized to 0.0-360.0

### Border

```rust
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Border {
    pub width: f32,
    pub color: Color,
    pub radius: BorderRadius,
    pub style: BorderStyle,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct BorderRadius {
    pub top_left: f32,
    pub top_right: f32,
    pub bottom_right: f32,
    pub bottom_left: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BorderStyle {
    Solid,
    Dashed,
    Dotted,
}
```

**Parsing**:
- `border_width="2"` → `width: 2.0`
- `border_color="#000"` → `color: Color::parse("#000")`
- `border_radius="8"` → all corners 8px
- `border_radius="8 4"` → top-left/bottom-right 8px, others 4px
- `border_style="solid"` → `BorderStyle::Solid`

### Shadow

```rust
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Shadow {
    pub offset_x: f32,
    pub offset_y: f32,
    pub blur_radius: f32,
    pub color: Color,
}
```

**Parsing**:
- `shadow="2 2 4 #00000040"` → `Shadow { offset_x: 2.0, offset_y: 2.0, blur_radius: 4.0, color: rgba(0,0,0,0.25) }`

### Transform

```rust
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Transform {
    Scale(f32),                    // Uniform scale
    ScaleXY { x: f32, y: f32 },   // Non-uniform scale
    Rotate(f32),                   // Degrees
    Translate { x: f32, y: f32 }, // Pixels
    Multiple(Vec<Transform>),      // Composed transforms
}
```

**Parsing**:
- `transform="scale(1.2)"` → `Scale(1.2)`
- `transform="rotate(45)"` → `Rotate(45.0)`
- `transform="translate(10, 20)"` → `Translate { x: 10.0, y: 20.0 }`

---

## Theming System Types

### GravityDocument (Extended)

**Location**: `gravity-core/src/ir/mod.rs`

```rust
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GravityDocument {
    pub version: SchemaVersion,
    pub root: WidgetNode,
    
    // NEW: Theming and styling
    pub themes: HashMap<String, Theme>,
    pub style_classes: HashMap<String, StyleClass>,
    pub global_theme: Option<String>,  // Default theme name
}
```

### Theme

**Location**: `gravity-core/src/ir/theme.rs`

```rust
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Theme {
    pub name: String,
    pub palette: ThemePalette,
    pub typography: Typography,
    pub spacing: SpacingScale,
    pub base_styles: HashMap<String, StyleProperties>,  // Default styles per widget type
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ThemePalette {
    pub primary: Color,
    pub secondary: Color,
    pub success: Color,
    pub warning: Color,
    pub danger: Color,
    pub background: Color,
    pub surface: Color,
    pub text: Color,
    pub text_secondary: Color,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Typography {
    pub font_family: String,
    pub font_size_base: f32,
    pub font_size_small: f32,
    pub font_size_large: f32,
    pub font_weight: FontWeight,
    pub line_height: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FontWeight {
    Thin,
    Light,
    Normal,
    Medium,
    Bold,
    Black,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpacingScale {
    pub unit: f32,  // Base spacing unit (e.g., 4px)
    // Derived: 1x = unit, 2x = unit*2, 3x = unit*3, etc.
}
```

**Relationships**:
- `GravityDocument.themes` contains all theme definitions
- `GravityDocument.global_theme` references a key in `themes`
- `WidgetNode.theme_ref` references a key in `themes` (overrides global)
- Built-in themes (`light`, `dark`, `default`) are always available

**Validation Rules**:
- `global_theme` must exist in `themes` or be built-in
- Theme names must be unique
- All palette colors must be valid
- `spacing.unit` must be > 0.0
- `font_size_*` must be > 0.0

### StyleClass

**Location**: `gravity-core/src/ir/theme.rs`

```rust
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StyleClass {
    pub name: String,
    pub style: StyleProperties,
    pub layout: Option<LayoutConstraints>,
    pub extends: Vec<String>,  // Inherit from other classes
    pub state_variants: HashMap<WidgetState, StyleProperties>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WidgetState {
    Hover,
    Focus,
    Active,
    Disabled,
}
```

**Fields**:
- `name`: Unique class identifier (e.g., `"button_primary"`)
- `style`: Base style properties
- `layout`: Optional layout constraints
- `extends`: List of parent class names (resolved in order, max depth 5)
- `state_variants`: State-specific style overrides (e.g., hover color)

**Relationships**:
- `extends` references other `StyleClass` names in `GravityDocument.style_classes`
- Circular dependencies are detected and rejected at parse time
- Resolution order: own style > extends[0] > extends[1] > ... > theme > defaults

**Validation Rules**:
- `name` must be unique within `style_classes`
- `extends` must not create circular dependency
- `extends` depth must not exceed 5 levels
- All referenced classes in `extends` must exist

---

## Responsive System Types

### Breakpoint

**Location**: `gravity-core/src/ir/layout.rs`

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Breakpoint {
    Mobile,    // < 640px
    Tablet,    // 640px - 1024px
    Desktop,   // >= 1024px
}

impl Breakpoint {
    pub fn from_viewport_width(width: f32) -> Self {
        match width {
            w if w < 640.0 => Breakpoint::Mobile,
            w if w < 1024.0 => Breakpoint::Tablet,
            _ => Breakpoint::Desktop,
        }
    }
}
```

**Usage**:
- Stored in `WidgetNode.breakpoint_attributes` keyed by `Breakpoint`
- Resolved at render time based on current viewport width
- Attribute resolution precedence: breakpoint-specific > base attribute

---

## State Transitions

### Widget State Machine

```
           ┌─────────┐
           │ Normal  │ (default state)
           └────┬────┘
                │
    ┌───────────┼───────────┐
    │           │           │
    ▼           ▼           ▼
┌────────┐  ┌────────┐  ┌──────────┐
│ Hover  │  │ Focus  │  │ Disabled │
└───┬────┘  └───┬────┘  └──────────┘
    │           │
    └─────┬─────┘
          ▼
     ┌────────┐
     │ Active │ (hover + pressed)
     └────────┘
```

**State Transitions**:
- `Normal` → `Hover`: Mouse enters widget
- `Hover` → `Active`: Mouse button pressed
- `Active` → `Hover`: Mouse button released
- `Hover` → `Normal`: Mouse leaves widget
- `Normal` → `Focus`: Widget receives keyboard focus
- `Focus` → `Normal`: Widget loses focus
- `Any` → `Disabled`: `disabled` attribute set to `true`
- `Disabled` → `Normal`: `disabled` attribute set to `false`

**State-Based Styling**:
- Each state can override style properties
- Applied via `StyleClass.state_variants`
- Inline state styles via XML: `hover:background="#2980b9"`

---

## Data Flow Diagrams

### Style Resolution Pipeline

```
XML Attribute
     ↓
Parse (parser/style_parser.rs)
     ↓
StyleProperties (IR)
     ↓
Cascade Resolution (runtime/style_cascade.rs)
     ↓
   ┌────────────────────┐
   │ Merge:             │
   │ 1. Inline styles   │ (highest priority)
   │ 2. Style classes   │
   │ 3. Theme defaults  │
   │ 4. Widget defaults │ (lowest priority)
   └────────┬───────────┘
            ↓
Final StyleProperties
     ↓
Backend Mapping (gravity-iced/src/style_mapping.rs)
     ↓
iced::widget::Style
     ↓
Rendered Widget
```

### Responsive Attribute Resolution

```
Window Resize Event
     ↓
Update viewport_width in Model
     ↓
View() Re-Run
     ↓
For each WidgetNode:
  ├─ Determine active Breakpoint
  ├─ Check breakpoint_attributes[Breakpoint]
  └─ Merge: breakpoint attrs > base attrs
     ↓
Resolved AttributeValue
     ↓
Widget Construction
```

### Theme Switching

```
User Action (Theme Change)
     ↓
Message::SetTheme(theme_name)
     ↓
Update Model.current_theme
     ↓
View() Re-Run
     ↓
For each WidgetNode:
  ├─ Check node.theme_ref (local override)
  └─ Fallback to Model.current_theme (global)
     ↓
Resolve Theme → ThemePalette
     ↓
Apply theme colors to StyleProperties
     ↓
Re-Render with new theme
```

---

## Serialization Format

All IR types implement `Serialize` and `Deserialize` for hot-reload state persistence.

**Example Serialized StyleProperties:**
```json
{
  "background": {
    "Gradient": {
      "Linear": {
        "angle": 90.0,
        "stops": [
          { "color": { "r": 0.2, "g": 0.6, "b": 0.85, "a": 1.0 }, "offset": 0.0 },
          { "color": { "r": 0.18, "g": 0.8, "b": 0.44, "a": 1.0 }, "offset": 1.0 }
        ]
      }
    }
  },
  "border": {
    "width": 2.0,
    "color": { "r": 0.0, "g": 0.0, "b": 0.0, "a": 1.0 },
    "radius": { "top_left": 8.0, "top_right": 8.0, "bottom_right": 8.0, "bottom_left": 8.0 },
    "style": "Solid"
  },
  "opacity": 1.0
}
```

---

## Summary

**Total New Types**: 20+

| Category | Types |
|----------|-------|
| Layout | LayoutConstraints, Length, Padding, Alignment, Justification, Direction |
| Styling | StyleProperties, Background, Color, Gradient, ColorStop, Border, Shadow, Transform |
| Theming | Theme, ThemePalette, Typography, SpacingScale, StyleClass |
| Responsive | Breakpoint |
| State | WidgetState |

**IR Extensions**: 2
- `WidgetNode` (added 5 fields)
- `GravityDocument` (added 3 fields)

**Design Principles**:
- ✅ Backend-agnostic (all types in gravity-core)
- ✅ Serializable (hot-reload support)
- ✅ Strongly-typed (no stringly-typed values)
- ✅ Validated (parse-time checks)
- ✅ Composable (cascading, inheritance, merging)
