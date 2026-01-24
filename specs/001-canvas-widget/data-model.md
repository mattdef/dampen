# Data Model: Canvas Widget

**Feature**: 001-canvas-widget  
**Date**: 2026-01-24

## Entity Overview

```
Canvas
  ├── CanvasShape (0..n)
  │     ├── RectShape
  │     ├── CircleShape
  │     ├── LineShape
  │     ├── TextShape
  │     └── GroupShape (contains CanvasShape 0..n)
  │
  ├── CanvasEvent (runtime, transient)
  │
  └── DrawingProgram (optional, user-provided)
```

---

## Core Entities

### Canvas

The root container for canvas-based rendering.

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| width | f32 | No (default: 400.0) | Canvas width in pixels |
| height | f32 | No (default: 300.0) | Canvas height in pixels |
| program | BindingExpr | No | Reference to custom Program |
| cache | CacheStrategy | No (default: OnChange) | Caching behavior |
| children | Vec\<CanvasShape\> | No | Declarative shape children |

**Events**:
- `on_click` → CanvasEvent(Click)
- `on_drag` → CanvasEvent(Drag)
- `on_move` → CanvasEvent(Move)
- `on_release` → CanvasEvent(Release)

**Validation Rules**:
- If `program` is set, `children` should be empty (custom mode)
- If `children` is non-empty, `program` should be unset (declarative mode)
- Width and height must be positive numbers

---

### CanvasShape

Abstract union type representing any drawable shape.

```
CanvasShape = RectShape | CircleShape | LineShape | TextShape | GroupShape
```

---

### RectShape

Rectangle with optional rounded corners.

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| x | f32 \| BindingExpr | Yes | Top-left x coordinate |
| y | f32 \| BindingExpr | Yes | Top-left y coordinate |
| width | f32 \| BindingExpr | Yes | Rectangle width |
| height | f32 \| BindingExpr | Yes | Rectangle height |
| fill | Color \| BindingExpr | No | Fill color |
| stroke | Color \| BindingExpr | No | Stroke color |
| stroke_width | f32 \| BindingExpr | No (default: 1.0) | Stroke width |
| radius | f32 \| BindingExpr | No | Corner radius for rounded corners |

**Validation Rules**:
- Width and height must be non-negative
- If stroke is set without stroke_width, default to 1.0
- Radius must be non-negative if set

---

### CircleShape

Circle defined by center point and radius.

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| cx | f32 \| BindingExpr | Yes | Center x coordinate |
| cy | f32 \| BindingExpr | Yes | Center y coordinate |
| radius | f32 \| BindingExpr | Yes | Circle radius |
| fill | Color \| BindingExpr | No | Fill color |
| stroke | Color \| BindingExpr | No | Stroke color |
| stroke_width | f32 \| BindingExpr | No (default: 1.0) | Stroke width |

**Validation Rules**:
- Radius must be non-negative

---

### LineShape

Line segment between two points.

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| x1 | f32 \| BindingExpr | Yes | Start x coordinate |
| y1 | f32 \| BindingExpr | Yes | Start y coordinate |
| x2 | f32 \| BindingExpr | Yes | End x coordinate |
| y2 | f32 \| BindingExpr | Yes | End y coordinate |
| stroke | Color \| BindingExpr | No (default: black) | Stroke color |
| stroke_width | f32 \| BindingExpr | No (default: 1.0) | Stroke width |

**Validation Rules**:
- Stroke width must be positive
- Default stroke color is black (#000000)

---

### TextShape

Text rendered at a specific position.

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| x | f32 \| BindingExpr | Yes | Text anchor x coordinate |
| y | f32 \| BindingExpr | Yes | Text anchor y coordinate |
| content | String \| BindingExpr | Yes | Text content |
| size | f32 \| BindingExpr | No (default: 16.0) | Font size in pixels |
| color | Color \| BindingExpr | No (default: black) | Text color |

**Validation Rules**:
- Size must be positive
- Content cannot be empty

**Notes**:
- Text uses system default font
- Alignment is top-left at (x, y) position
- Text may not render correctly when inside transformed groups

---

### GroupShape

Container for multiple shapes with optional transformation.

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| transform | Transform | No | Transformation to apply |
| children | Vec\<CanvasShape\> | No | Child shapes |

**Validation Rules**:
- Children inherit the group's transformation
- Transformations compound (nested groups multiply transforms)

---

### Transform

Geometric transformation for groups.

```
Transform = 
  | Translate(x: f32, y: f32)
  | Rotate(angle: f32)           // radians
  | Scale(factor: f32)           // uniform scale
  | ScaleXY(x: f32, y: f32)      // non-uniform scale
  | Matrix([f32; 6])             // 2D affine matrix
```

**XML Syntax**:
```xml
transform="translate(100, 50)"
transform="rotate(0.785)"          <!-- 45 degrees in radians -->
transform="scale(2.0)"
transform="scale(2.0, 1.5)"
transform="matrix(1,0,0,1,10,20)"  <!-- affine matrix -->
```

---

### CanvasEvent

Runtime event carrying interaction data (not persisted).

| Field | Type | Description |
|-------|------|-------------|
| kind | CanvasEventKind | Event type |
| x | f32 | X coordinate relative to canvas origin |
| y | f32 | Y coordinate relative to canvas origin |
| delta_x | Option\<f32\> | X delta for drag events |
| delta_y | Option\<f32\> | Y delta for drag events |

**CanvasEventKind**:
- `Click` - Mouse button pressed
- `Drag` - Mouse moved while button held
- `Move` - Mouse moved (no button)
- `Release` - Mouse button released

---

### CacheStrategy

Controls when canvas geometry is recomputed.

| Value | Behavior |
|-------|----------|
| `Always` | Cache until explicit `clear()` |
| `Never` | Redraw every frame |
| `OnChange` (default) | Clear cache when bound values change |

---

### Color

Color value supporting multiple formats.

**Accepted Formats**:
- Hex: `#RGB`, `#RRGGBB`, `#RRGGBBAA`
- RGB: `rgb(255, 128, 0)`
- RGBA: `rgba(255, 128, 0, 0.5)`
- HSL: `hsl(30, 100%, 50%)`
- Named: `red`, `blue`, `transparent`, etc.

**Internal Representation**:
```rust
struct Color {
    r: f32,  // 0.0 - 1.0
    g: f32,
    b: f32,
    a: f32,
}
```

---

## WidgetKind Extensions

New variants for `WidgetKind` enum:

| Variant | Description |
|---------|-------------|
| `Canvas` | Already exists |
| `CanvasRect` | Rectangle shape |
| `CanvasCircle` | Circle shape |
| `CanvasLine` | Line shape |
| `CanvasText` | Text shape |
| `CanvasGroup` | Group container |

---

## EventKind Extensions

New variants for `EventKind` enum:

| Variant | Handler Receives |
|---------|------------------|
| `CanvasClick` | CanvasEvent |
| `CanvasDrag` | CanvasEvent |
| `CanvasMove` | CanvasEvent |
| `CanvasRelease` | CanvasEvent |

---

## State Transitions

### Canvas Rendering Lifecycle

```
[XML Parsed] 
    │
    ▼
[Shapes Extracted from Children]
    │
    ▼
[DeclarativeProgram Created]
    │
    ├──[Static] → Cache geometry
    │
    └──[Dynamic Bindings] → Re-evaluate on model change
                               │
                               ▼
                          [Clear Cache] → Re-draw
```

### Event Flow

```
[Mouse Event in Iced]
    │
    ▼
[cursor.position_in(bounds)]
    │
    ├──[None] → Event outside canvas, ignore
    │
    └──[Some(position)] → Create CanvasEvent
                              │
                              ▼
                         [Match event kind]
                              │
                              ▼
                         [Dispatch to handler via HandlerRegistry]
                              │
                              ▼
                         [Handler modifies model]
                              │
                              ▼
                         [UI re-renders if model changed]
```

---

## Relationships

| Parent | Relationship | Child |
|--------|--------------|-------|
| Canvas | contains | CanvasShape (0..n) |
| GroupShape | contains | CanvasShape (0..n) |
| CanvasShape | references | Color |
| CanvasShape | references | BindingExpr |
| Canvas | emits | CanvasEvent |
| CanvasEvent | dispatched to | HandlerEntry |

---

## Notes

1. **Binding Support**: All numeric and color fields support both static values and binding expressions.

2. **Z-Order**: Shapes render in document order (later shapes on top).

3. **Coordinate System**: Origin at top-left, Y increases downward (standard screen coordinates).

4. **Empty Canvas**: Canvas with no children and no program renders as empty transparent area.

5. **For-Each in Canvas**: `<for each="item" in="{items}">` creates shapes for each item in collection.
