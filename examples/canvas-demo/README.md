# Dampen Canvas Demo

This example demonstrates the usage of the `canvas` widget in Dampen, including both declarative shapes and custom drawing programs.

## Features

- **Declarative Shapes**: Shapes defined directly in XML (`<rect>`, `<circle>`, `<canvas_text>`, `<group>`).
- **Dynamic Binding**: Shape attributes bound to model state (e.g., circle position, rectangle color).
- **Interactivity**: Mouse events (`on_click`, `on_drag`, `on_release`) handled by model functions.
- **Custom Programs**: Integration with standard Iced `canvas::Program` for custom drawing (a simple clock hand).
- **Transformations**: Grouping shapes and applying transformations like `translate`.

## Running the Demo

### Development Mode (Interpreted)

Run with hot-reload enabled. You can edit `src/ui/window.dampen` while the app is running to see changes instantly.

```bash
dampen run -p canvas-demo
```

### Production Mode (Codegen)

Build and run with optimized pre-generated code.

```bash
dampen run --release -p canvas-demo
```

## UI Structure (`window.dampen`)

The UI is defined using XML:

```xml
<canvas
    width="400"
    height="300"
    on_click="on_canvas_click"
    on_drag="on_canvas_drag"
    on_release="on_canvas_release"
>
    <!-- Background Rect -->
    <rect x="50" y="50" width="300" height="200" fill="{rect_color}" radius="10" />

    <!-- Interactive Circle -->
    <circle cx="{circle_x}" cy="{circle_y}" radius="{circle_radius}" fill="#e74c3c" />

    <!-- Custom Program Integration -->
    <group transform="translate(40, 40)">
        <rect x="60" y="60" width="30" height="30" fill="#f1c40f" />
    </group>
</canvas>

<canvas width="200" height="200" program="{clock}" />
```
