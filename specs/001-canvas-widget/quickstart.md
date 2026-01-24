# Quickstart: Canvas Widget

**Feature**: 001-canvas-widget  
**Date**: 2026-01-24

## Overview

The Canvas widget enables drawing custom graphics in Dampen applications. It supports two modes:

1. **Declarative Mode**: Define shapes directly in XML
2. **Custom Mode**: Bind to a Rust Program implementation

---

## Quick Examples

### Hello Canvas (Static Shapes)

```xml
<!-- src/ui/main.dampen -->
<canvas width="400" height="300">
    <rect x="50" y="50" width="300" height="200" fill="#3498db" radius="10" />
    <circle cx="200" cy="150" radius="50" fill="#e74c3c" />
    <canvas_text x="200" y="260" size="20" color="white">
        Hello Canvas!
    </canvas_text>
</canvas>
```

### Interactive Canvas (With Events)

```xml
<!-- src/ui/main.dampen -->
<canvas width="600" height="400" on_click="add_point" on_drag="move_last_point">
    <for each="point" in="{points}">
        <circle cx="{point.x}" cy="{point.y}" radius="8" fill="#3498db" />
    </for>
</canvas>
```

```rust
// src/main.rs
use dampen::prelude::*;

#[derive(UiModel, Default)]
struct Model {
    points: Vec<Point>,
}

#[derive(Clone)]
struct Point { x: f32, y: f32 }

fn add_point(model: &mut Model, event: CanvasEvent) {
    model.points.push(Point { x: event.x, y: event.y });
}

fn move_last_point(model: &mut Model, event: CanvasEvent) {
    if let Some(last) = model.points.last_mut() {
        last.x = event.x;
        last.y = event.y;
    }
}
```

### Data Visualization (Dynamic Binding)

```xml
<canvas width="600" height="400">
    <!-- Background grid -->
    <rect x="0" y="0" width="600" height="400" fill="#f9f9f9" />
    
    <!-- Bar chart -->
    <for each="bar" in="{bars}">
        <rect
            x="{bar.x}"
            y="{400 - bar.height}"
            width="40"
            height="{bar.height}"
            fill="{bar.color}"
        />
        <canvas_text x="{bar.x + 20}" y="390" size="12" color="#333">
            {bar.label}
        </canvas_text>
    </for>
</canvas>
```

---

## Available Shapes

| Shape | Purpose | Key Attributes |
|-------|---------|----------------|
| `<rect>` | Rectangle | x, y, width, height, fill, stroke, radius |
| `<circle>` | Circle | cx, cy, radius, fill, stroke |
| `<line>` | Line segment | x1, y1, x2, y2, stroke, stroke_width |
| `<canvas_text>` | Text | x, y, size, color, (content as children) |
| `<group>` | Transform container | transform |

---

## Events

| Event | Handler Signature | Data Provided |
|-------|-------------------|---------------|
| `on_click` | `fn(model, CanvasEvent)` | x, y coordinates |
| `on_drag` | `fn(model, CanvasEvent)` | x, y, delta_x, delta_y |
| `on_move` | `fn(model, CanvasEvent)` | x, y coordinates |
| `on_release` | `fn(model, CanvasEvent)` | x, y coordinates |

---

## Transformations

Apply transformations to groups of shapes:

```xml
<group transform="translate(100, 50)">
    <!-- Shapes moved by (100, 50) -->
    <rect x="0" y="0" width="50" height="50" fill="blue" />
</group>

<group transform="rotate(0.785)">
    <!-- Shapes rotated 45 degrees -->
    <rect x="0" y="0" width="50" height="50" fill="red" />
</group>

<group transform="scale(2)">
    <!-- Shapes scaled 2x -->
    <circle cx="25" cy="25" radius="10" fill="green" />
</group>
```

---

## Color Formats

All color attributes accept these formats:

- Hex: `#RGB`, `#RRGGBB`, `#RRGGBBAA`
- RGB: `rgb(255, 128, 0)`
- RGBA: `rgba(255, 128, 0, 0.5)`
- HSL: `hsl(30, 100%, 50%)`
- Named: `red`, `blue`, `transparent`

---

## Custom Programs

For complex graphics beyond declarative shapes:

```xml
<canvas width="800" height="600" program="{my_program}" />
```

```rust
use iced::widget::canvas::{self, Geometry, Frame};

struct MyProgram;

impl canvas::Program<Message> for MyProgram {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &canvas::Renderer,
        _theme: &iced::Theme,
        bounds: iced::Rectangle,
        _cursor: canvas::Cursor,
    ) -> Vec<Geometry> {
        let mut frame = Frame::new(renderer, bounds.size());
        // Custom drawing...
        vec![frame.into_geometry()]
    }
}
```

---

## Running

```bash
# Development (hot-reload)
dampen run

# Production build
dampen release
```

---

## Next Steps

- See [data-model.md](./data-model.md) for complete attribute reference
- See [contracts/canvas-xml.md](./contracts/canvas-xml.md) for XML schema details
- Check `examples/canvas-demo/` for working examples
