# Canvas Widget XML Contract

**Feature**: 001-canvas-widget  
**Date**: 2026-01-24

## Overview

This document defines the XML syntax contract for the Canvas widget and its child shapes.

---

## Canvas Element

```xml
<canvas
    width="400"                <!-- Optional: Canvas width (default: 400) -->
    height="300"               <!-- Optional: Canvas height (default: 300) -->
    program="{model.program}"  <!-- Optional: Custom Program binding -->
    cache="OnChange"           <!-- Optional: Cache strategy (Always|Never|OnChange) -->
    on_click="handle_click"    <!-- Optional: Click event handler -->
    on_drag="handle_drag"      <!-- Optional: Drag event handler -->
    on_move="handle_move"      <!-- Optional: Move event handler -->
    on_release="handle_release" <!-- Optional: Release event handler -->
>
    <!-- Child shape elements (declarative mode) -->
</canvas>
```

### Modes

| Mode | Condition | Description |
|------|-----------|-------------|
| Declarative | Has shape children, no `program` | Shapes defined in XML |
| Custom | Has `program` attribute | User provides Program impl |
| Empty | No children, no `program` | Transparent empty canvas |

---

## Shape Elements

### Rect

```xml
<rect
    x="10"                <!-- Required: Top-left X coordinate -->
    y="10"                <!-- Required: Top-left Y coordinate -->
    width="100"           <!-- Required: Width -->
    height="50"           <!-- Required: Height -->
    fill="#3498db"        <!-- Optional: Fill color -->
    stroke="#2c3e50"      <!-- Optional: Stroke color -->
    stroke_width="2"      <!-- Optional: Stroke width (default: 1) -->
    radius="8"            <!-- Optional: Corner radius -->
/>
```

**With Bindings**:
```xml
<rect
    x="{box.x}"
    y="{box.y}"
    width="{box.width}"
    height="{box.height}"
    fill="{box.color}"
/>
```

---

### Circle

```xml
<circle
    cx="200"              <!-- Required: Center X coordinate -->
    cy="150"              <!-- Required: Center Y coordinate -->
    radius="40"           <!-- Required: Radius -->
    fill="#e74c3c"        <!-- Optional: Fill color -->
    stroke="#c0392b"      <!-- Optional: Stroke color -->
    stroke_width="2"      <!-- Optional: Stroke width (default: 1) -->
/>
```

**With Bindings**:
```xml
<circle
    cx="{ball.x}"
    cy="{ball.y}"
    radius="{ball.radius}"
    fill="{ball.color}"
/>
```

---

### Line

```xml
<line
    x1="10"               <!-- Required: Start X coordinate -->
    y1="250"              <!-- Required: Start Y coordinate -->
    x2="390"              <!-- Required: End X coordinate -->
    y2="250"              <!-- Required: End Y coordinate -->
    stroke="#7f8c8d"      <!-- Optional: Stroke color (default: black) -->
    stroke_width="2"      <!-- Optional: Stroke width (default: 1) -->
/>
```

---

### Canvas Text

```xml
<canvas_text
    x="200"               <!-- Required: X coordinate -->
    y="280"               <!-- Required: Y coordinate -->
    size="16"             <!-- Optional: Font size (default: 16) -->
    color="#2c3e50"       <!-- Optional: Text color (default: black) -->
>
    Label Text            <!-- Required: Text content -->
</canvas_text>
```

**With Bindings**:
```xml
<canvas_text x="{label.x}" y="{label.y}" color="{label.color}">
    {label.text}
</canvas_text>
```

---

### Group

```xml
<group transform="translate(100, 50)">
    <rect x="0" y="0" width="50" height="50" fill="#9b59b6" />
    <circle cx="25" cy="25" radius="20" fill="#8e44ad" />
</group>
```

**Transform Syntax**:
```xml
transform="translate(x, y)"       <!-- Move by (x, y) -->
transform="rotate(radians)"       <!-- Rotate around center -->
transform="scale(factor)"         <!-- Uniform scale -->
transform="scale(x, y)"           <!-- Non-uniform scale -->
transform="matrix(a,b,c,d,e,f)"   <!-- 2D affine matrix -->
```

---

## Control Flow in Canvas

### For-Each Loop

```xml
<canvas width="400" height="300">
    <for each="point" in="{data_points}">
        <circle
            cx="{point.x}"
            cy="{point.y}"
            radius="5"
            fill="{point.color}"
        />
    </for>
</canvas>
```

### Conditional Rendering

```xml
<canvas width="400" height="300">
    <if condition="{show_grid}">
        <!-- Grid lines -->
        <line x1="0" y1="0" x2="400" y2="0" stroke="#ccc" />
        <!-- ... more lines ... -->
    </if>
    
    <!-- Always visible -->
    <circle cx="200" cy="150" radius="50" fill="#3498db" />
</canvas>
```

---

## Event Handler Signatures

### Click Handler

```rust
fn handle_click(model: &mut Model, event: CanvasEvent) {
    // event.x, event.y contain click coordinates
}
```

### Drag Handler

```rust
fn handle_drag(model: &mut Model, event: CanvasEvent) {
    // event.x, event.y - current position
    // event.delta_x, event.delta_y - movement since last event
}
```

### Move Handler

```rust
fn handle_move(model: &mut Model, event: CanvasEvent) {
    // event.x, event.y - current cursor position
}
```

### Release Handler

```rust
fn handle_release(model: &mut Model, event: CanvasEvent) {
    // event.x, event.y - release position
}
```

---

## Complete Examples

### Static Diagram

```xml
<canvas width="400" height="300">
    <!-- Background -->
    <rect x="0" y="0" width="400" height="300" fill="#f5f5f5" />
    
    <!-- Shapes -->
    <rect x="10" y="10" width="100" height="50" fill="#3498db" radius="8" />
    <circle cx="200" cy="150" radius="40" fill="#e74c3c" />
    <line x1="10" y1="250" x2="390" y2="250" stroke="#7f8c8d" stroke_width="2" />
    
    <!-- Label -->
    <canvas_text x="200" y="280" size="16" color="#2c3e50">
        My Canvas
    </canvas_text>
</canvas>
```

### Interactive Drawing

```xml
<canvas 
    width="800" 
    height="600"
    on_click="add_point"
    on_drag="move_point"
>
    <for each="point" in="{points}">
        <circle 
            cx="{point.x}" 
            cy="{point.y}" 
            radius="10" 
            fill="{if point.selected then '#e74c3c' else '#3498db'}"
        />
    </for>
</canvas>
```

### Data Visualization

```xml
<canvas width="600" height="400">
    <!-- Y-axis -->
    <line x1="50" y1="20" x2="50" y2="380" stroke="#333" stroke_width="2" />
    
    <!-- X-axis -->
    <line x1="50" y1="380" x2="580" y2="380" stroke="#333" stroke_width="2" />
    
    <!-- Data bars -->
    <for each="bar" in="{chart_data}">
        <rect 
            x="{bar.x}" 
            y="{bar.y}" 
            width="{bar.width}" 
            height="{bar.height}"
            fill="{bar.color}"
        />
        <canvas_text x="{bar.label_x}" y="{bar.label_y}" size="12">
            {bar.label}
        </canvas_text>
    </for>
</canvas>
```

### Custom Program

```xml
<canvas 
    width="800" 
    height="600" 
    program="{chart_program}"
    on_click="chart_click"
/>
```

```rust
// In application code
struct Model {
    chart_program: ChartProgram,
}

struct ChartProgram { /* ... */ }

impl canvas::Program<Message> for ChartProgram {
    type State = ChartState;
    
    fn draw(&self, /* ... */) -> Vec<Geometry> {
        // Custom drawing logic
    }
}
```

---

## Validation Rules

| Rule | Error |
|------|-------|
| Shape outside canvas | "Shape elements must be children of canvas" |
| Missing required attribute | "Missing required attribute '{name}' on {element}" |
| Invalid color format | "Invalid color format: '{value}'" |
| Invalid transform syntax | "Invalid transform: '{value}'" |
| Negative dimension | "{attribute} must be non-negative" |
| Both program and children | "Canvas cannot have both program attribute and shape children" |

---

## Schema Summary

| Element | Required Attrs | Optional Attrs | Events |
|---------|----------------|----------------|--------|
| canvas | - | width, height, program, cache | on_click, on_drag, on_move, on_release |
| rect | x, y, width, height | fill, stroke, stroke_width, radius | - |
| circle | cx, cy, radius | fill, stroke, stroke_width | - |
| line | x1, y1, x2, y2 | stroke, stroke_width | - |
| canvas_text | x, y | size, color | - |
| group | - | transform | - |
