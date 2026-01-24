# Research: Canvas Widget Implementation

**Feature**: 001-canvas-widget  
**Date**: 2026-01-24

## Executive Summary

Research confirms that Iced's canvas widget provides all necessary primitives for implementing Dampen's declarative canvas. The `Program` trait, `Frame` drawing API, and `Cache` performance mechanism align well with the hybrid (declarative + custom program) design.

---

## Research Topics

### 1. Iced Canvas Program Trait

**Decision**: Implement `DeclarativeProgram` struct that implements `iced::widget::canvas::Program<Message>`.

**Rationale**: The Program trait is the standard interface for canvas rendering in Iced. It provides:
- `draw()` for rendering shapes to geometry
- `update()` for handling mouse/keyboard events
- `mouse_interaction()` for cursor feedback

**Alternatives Considered**:
- Direct Frame manipulation without Program: Rejected - doesn't support events
- Custom widget from scratch: Rejected - duplicates Iced functionality

**Key Implementation Details**:
```rust
pub struct DeclarativeProgram<'a, M> {
    shapes: &'a [CanvasShape],
    event_handlers: Option<CanvasEventHandlers<M>>,
    cache: canvas::Cache,
}

impl<M> canvas::Program<M> for DeclarativeProgram<'_, M> {
    type State = CanvasState;
    
    fn draw(&self, state: &Self::State, renderer: &Renderer, theme: &Theme,
            bounds: Rectangle, cursor: Cursor) -> Vec<Geometry>;
    
    fn update(&self, state: &mut Self::State, event: Event,
              bounds: Rectangle, cursor: Cursor) -> Option<Message>;
}
```

---

### 2. Shape Drawing with Frame and Path

**Decision**: Use Iced's `Path` constructors for basic shapes, `Path::new()` builder for custom paths.

**Rationale**: Iced provides direct constructors matching our shape types:
- `Path::rectangle(point, size)` → `<rect>`
- `Path::rounded_rectangle(point, size, radius)` → `<rect radius="...">`
- `Path::circle(center, radius)` → `<circle>`
- `Path::line(from, to)` → `<line>`
- `frame.fill_text(Text { ... })` → `<canvas_text>`

**Shape to Iced Mapping**:

| Dampen Shape | Iced API |
|--------------|----------|
| `<rect x y width height>` | `Path::rectangle(Point::new(x, y), Size::new(w, h))` |
| `<rect ... radius>` | `Path::rounded_rectangle(...)` |
| `<circle cx cy radius>` | `Path::circle(Point::new(cx, cy), radius)` |
| `<line x1 y1 x2 y2>` | `Path::line(Point::new(x1, y1), Point::new(x2, y2))` |
| `<canvas_text>` | `frame.fill_text(canvas::Text { ... })` |

**Fill and Stroke**:
```rust
// Fill shape
frame.fill(&path, Color::from_rgba8(r, g, b, a));

// Stroke shape
let stroke = Stroke {
    style: Style::Solid(color),
    width: stroke_width,
    line_cap: LineCap::Round,
    ..Default::default()
};
frame.stroke(&path, stroke);
```

---

### 3. Transformations (Group with Transform)

**Decision**: Use Frame's `with_translation()`, `with_rotation()`, `with_scale()` for group transforms.

**Rationale**: Iced provides closure-based transformation composition that matches the group nesting pattern.

**Implementation Pattern**:
```rust
fn draw_group(&self, frame: &mut Frame, group: &CanvasGroup) {
    match &group.transform {
        Some(Transform::Translate(x, y)) => {
            frame.with_translation(Vector::new(*x, *y), |frame| {
                for child in &group.children {
                    self.draw_shape(frame, child);
                }
            });
        }
        Some(Transform::Rotate(angle)) => {
            frame.with_rotation(*angle, frame.center(), |frame| {
                for child in &group.children {
                    self.draw_shape(frame, child);
                }
            });
        }
        // ... scale, matrix
        None => {
            for child in &group.children {
                self.draw_shape(frame, child);
            }
        }
    }
}
```

**Note**: Text does not render well when rotated/scaled. Keep text outside transformed groups or warn users.

---

### 4. Event Handling with Coordinates

**Decision**: Use `cursor.position_in(bounds)` for relative coordinates, dispatch to registered handlers.

**Rationale**: Iced's cursor API provides precise coordinate tracking within bounds. The `Event` enum captures all mouse events needed.

**Event Mapping**:

| Dampen Event | Iced Event | Data |
|--------------|------------|------|
| `on_click` | `mouse::Event::ButtonPressed(Left)` | `(x, y)` |
| `on_drag` | `ButtonPressed` + `CursorMoved` | `(x, y, dx, dy)` |
| `on_move` | `mouse::Event::CursorMoved` | `(x, y)` |
| `on_release` | `mouse::Event::ButtonReleased` | `(x, y)` |

**State Tracking for Drag**:
```rust
#[derive(Default)]
pub struct CanvasState {
    is_dragging: bool,
    drag_start: Option<Point>,
    last_position: Option<Point>,
}
```

**Update Implementation**:
```rust
fn update(&self, state: &mut Self::State, event: Event, bounds: Rectangle, cursor: Cursor)
    -> Option<M>
{
    let position = cursor.position_in(bounds)?;
    
    match event {
        Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
            state.is_dragging = true;
            state.drag_start = Some(position);
            state.last_position = Some(position);
            self.emit_click(position)
        }
        Event::Mouse(mouse::Event::CursorMoved { .. }) if state.is_dragging => {
            let delta = state.last_position.map(|last| 
                (position.x - last.x, position.y - last.y)
            );
            state.last_position = Some(position);
            self.emit_drag(position, delta)
        }
        Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {
            state.is_dragging = false;
            self.emit_release(position)
        }
        // ... on_move
        _ => None
    }
}
```

---

### 5. Performance: Cache Strategy

**Decision**: Use single cache with `OnChange` default strategy; clear when bindings change.

**Rationale**: 
- Iced's `Cache` stores pre-computed geometry, avoiding frame-by-frame recomputation
- For static shapes, caching provides significant performance gains
- For dynamic bindings, cache must be cleared when bound values change

**Cache Strategies**:

| Strategy | Behavior | Use Case |
|----------|----------|----------|
| `Always` | Cache until explicit clear | Static diagrams, backgrounds |
| `Never` | Redraw every frame | Animations, frequent updates |
| `OnChange` (default) | Clear when bindings change | Most common case |

**Implementation**:
```rust
pub struct DeclarativeProgram {
    shapes: Vec<CanvasShape>,
    cache: canvas::Cache,
    needs_redraw: bool,
}

impl DeclarativeProgram {
    pub fn clear_cache(&mut self) {
        self.cache.clear();
        self.needs_redraw = true;
    }
    
    pub fn update_shapes(&mut self, shapes: Vec<CanvasShape>) {
        self.shapes = shapes;
        self.clear_cache();
    }
}
```

---

### 6. Custom Program Binding

**Decision**: Support `program="{model.canvas_program}"` binding that resolves to user's `impl Program`.

**Rationale**: Advanced users need escape hatch for complex visualizations (charts, procedural graphics) that can't be expressed declaratively.

**Challenge**: Iced's canvas widget is generic over the Program type, requiring type erasure or boxing.

**Solution**: Use `Box<dyn canvas::Program<M>>` for dynamic dispatch:
```rust
pub enum CanvasContent<'a, M> {
    Declarative(DeclarativeProgram<'a, M>),
    Custom(Box<dyn canvas::Program<M, State = ()> + 'a>),
}
```

**Alternative**: Require custom programs to implement a Dampen trait that wraps Program.

---

### 7. Codegen Mode Considerations

**Decision**: Generate Rust code that constructs shapes at compile time; delegate to `DeclarativeProgram` at runtime.

**Rationale**: Codegen mode must produce self-contained Rust code. Shapes become static data; Program implementation remains shared.

**Generated Code Pattern**:
```rust
// Generated by dampen-macros
fn build_canvas() -> Element<'_, Message, Theme, Renderer> {
    let shapes = vec![
        CanvasShape::Rect(RectShape {
            x: 10.0, y: 10.0, width: 100.0, height: 50.0,
            fill: Some(Color::from_rgb8(0x34, 0x98, 0xdb)),
            stroke: None,
            radius: Some(8.0),
        }),
        CanvasShape::Circle(CircleShape {
            cx: 200.0, cy: 150.0, radius: 40.0,
            fill: Some(Color::from_rgb8(0xe7, 0x4c, 0x3c)),
            stroke: None,
        }),
    ];
    
    let program = DeclarativeProgram::new(shapes);
    iced::widget::canvas(program)
        .width(Length::Fixed(400.0))
        .height(Length::Fixed(300.0))
        .into()
}
```

---

### 8. Handler Integration

**Decision**: Add `CanvasEvent` struct and `WithCanvasEvent` handler variant to existing handler system.

**Rationale**: Canvas events carry coordinate data, unlike simple click events. Requires new handler signature.

**New Types**:
```rust
#[derive(Debug, Clone)]
pub struct CanvasEvent {
    pub kind: CanvasEventKind,
    pub x: f32,
    pub y: f32,
    pub delta_x: Option<f32>,
    pub delta_y: Option<f32>,
}

pub enum CanvasEventKind {
    Click,
    Drag,
    Move,
    Release,
}

// In HandlerEntry enum
pub enum HandlerEntry {
    // ... existing variants ...
    WithCanvasEvent(Arc<dyn Fn(&mut dyn Any, CanvasEvent) + Send + Sync>),
}
```

---

## Open Questions (Resolved)

| Question | Resolution |
|----------|------------|
| How to handle text in transformed groups? | Document limitation: text doesn't render well rotated |
| Multi-cache vs single cache? | Single cache with explicit clear; simpler for declarative model |
| Type erasure for custom programs? | Use `Box<dyn Program>` with `State = ()` constraint |

---

## References

1. [iced::widget::canvas::Program](https://docs.iced.rs/iced/widget/canvas/trait.Program.html)
2. [iced::widget::canvas module](https://docs.iced.rs/iced_widget/canvas/index.html)
3. [Game of Life example](https://github.com/iced-rs/iced/blob/master/examples/game_of_life/src/main.rs)
4. [Canvas interactivity PR](https://github.com/iced-rs/iced/pull/325)
