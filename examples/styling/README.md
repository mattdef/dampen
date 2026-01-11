# Dampen Styling Example

This example provides a **comprehensive showcase** of all layout, sizing, theming, and styling capabilities currently available in Dampen.

## Overview

Dampen is a declarative UI framework for Rust that uses XML-based `.dampen` files to define user interfaces. This example demonstrates every feature implemented in the framework, including:

- ✅ Layout attributes (padding, spacing, alignment)
- ✅ Sizing modes (fixed, fill, shrink, percentage)
- ✅ Inline styling (colors, borders, shadows, gradients)
- ✅ Theming system (palettes, typography, spacing scales)
- ✅ Style classes with inheritance
- ✅ State-based styling (hover, active, disabled, combined states)
- ✅ Data binding with `{expression}` syntax
- ✅ Event handlers for user interactions

## Available Examples

This directory contains multiple examples:

```bash
# Main comprehensive showcase (this file)
cargo run --bin styling

# State transitions demo (hover, active, combined states)
cargo run --bin state-demo

# Theme switching demo
cargo run -p styling  # (loads ui/theme_demo.dampen)
```

## File Structure

```
examples/styling/
├── Cargo.toml
├── README.md (this file)
├── src/
│   ├── main.rs          # Main comprehensive example
│   ├── state_demo.rs    # State transitions demo
│   └── theme_demo.rs    # Theme switching demo
└── ui/
    ├── main.dampen         # Main comprehensive UI showcase
    ├── state_demo.dampen   # State transitions UI
    └── theme_demo.dampen   # Theme switching UI
```

## Features Demonstrated

### 1. Layout Attributes

| Attribute | Description | Example |
|-----------|-------------|---------|
| `padding` | Inner spacing within widgets | `padding="20"` or `padding="10 20"` |
| `spacing` | Gap between child widgets | `spacing="15"` |
| `align_items` | Cross-axis alignment | `align_items="center"` |
| `justify_content` | Main-axis distribution | `justify_content="space-between"` |
| `align_self` | Override parent alignment | `align_self="end"` |
| `direction` | Layout direction | `direction="horizontal"` |

### 2. Sizing Modes

| Mode | Description | Example |
|------|-------------|---------|
| **Fixed** | Exact pixel value | `width="200"` |
| **Fill** | Expand to available space | `width="fill"` |
| **Shrink** | Minimize to content size | `width="shrink"` |
| **Fill Portion** | Proportional fill | `width="fill_portion(2)"` |
| **Percentage** | Percentage of parent | `width="50%"` |
| **Constraints** | Min/max limits | `min_width="100" max_width="400"` |

### 3. Style Attributes

#### Colors
```xml
<!-- Hex colors -->
<text color="#3498db" background="#ecf0f1" />

<!-- RGB/RGBA -->
<text color="rgb(52, 152, 219)" />
<text color="rgba(52, 152, 219, 0.8)" />

<!-- Named colors -->
<text color="blue" background="transparent" />
```

#### Borders
```xml
<container 
    border_width="2"
    border_color="#3498db"
    border_radius="8"
    border_style="solid" />
```

**Border Radius Syntax:**
- Single value: `border_radius="8"` (all corners)
- Two values: `border_radius="8 4"` (top-left/bottom-right, top-right/bottom-left)
- Four values: `border_radius="8 4 8 4"` (top-left, top-right, bottom-right, bottom-left)

#### Shadows
```xml
<!-- Syntax: offset-x offset-y blur-radius color -->
<container shadow="2 4 8 #00000030" />
<container shadow="0 4 12 rgba(0,0,0,0.2)" />
```

#### Gradients
```xml
<!-- Linear gradient: angle, color stops -->
<container background="linear-gradient(90deg, #3498db, #2ecc71)" />
<container background="linear-gradient(135deg, #667eea 0%, #764ba2 100%)" />
```

#### Opacity & Transform
```xml
<container opacity="0.8" />
<container transform="scale(1.2)" />
<container transform="rotate(45)" />
<container transform="translate(10, 20)" />
```

### 4. Theming System

Define reusable themes with color palettes, typography, and spacing:

```xml
<themes>
    <theme name="light">
        <!-- Color palette -->
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
        
        <!-- Typography settings -->
        <typography 
            font_family="Inter, sans-serif"
            font_size_base="16"
            font_size_small="12"
            font_size_large="20"
            font_weight="normal"
            line_height="1.5" />
        
        <!-- Spacing scale (base unit in pixels) -->
        <spacing unit="8" />
    </theme>
</themes>

<!-- Set active theme -->
<global_theme name="light" />
```

**Theme Properties:**
- **Palette**: 9 semantic colors (primary, secondary, success, warning, danger, background, surface, text, text_secondary)
- **Typography**: font_family, font_size_base/small/large, font_weight, line_height
- **Spacing**: base unit for consistent spacing (multiples like 1x, 2x, 3x)

### 5. Style Classes

Define reusable style classes with state variants:

```xml
<styles>
    <style name="btn_primary">
        <!-- Base state (default) -->
        <base 
            background="#3498db"
            color="#ffffff"
            padding="12 24"
            border_radius="6" />
        
        <!-- State variants -->
        <hover background="#5dade2" />
        <active background="#2874a6" />
        <focus border_color="#85c1e9" />
        <disabled opacity="0.5" />
        
        <!-- Combined states (most specific) -->
        <hover:active background="#1c5d8a" />
    </style>
</styles>

<!-- Use the class -->
<button label="Click Me" class="btn_primary" on_click="action" />
```

**State Variants:**
- `hover:` - Mouse is over the widget
- `active:` - Widget is being clicked/pressed
- `focus:` - Widget has keyboard focus
- `disabled:` - Widget is disabled
- **Combined states** - Infrastructure in place, XML syntax in development (e.g., `hover:active`)

**Cascade Precedence** (highest to lowest):
1. Class single state variants (e.g., `<hover>`, `<active>`, `<disabled>`)
2. Inline base styles (e.g., `background="#..."`)
3. Class base styles (e.g., `<base>`)
4. Theme defaults

**Note**: Combined state variants (e.g., `hover:active`) are implemented in the IR and parser but awaiting XML syntax that doesn't conflict with XML namespaces.

### 6. Data Binding

Use `{expression}` syntax for dynamic values:

```xml
<!-- Simple binding -->
<text value="Count: {count}" />

<!-- In attributes -->
<container width="{dynamic_width}" />

<!-- Expression evaluation -->
<text value="Double: {count * 2}" />
```

**Supported in:**
- Text content (`value` attribute)
- All style and layout attributes
- Attribute values are re-evaluated on state changes

### 7. Event Handlers

Connect UI events to Rust message handlers:

```xml
<button label="Click Me" on_click="increment" />
<button label="Submit" on_click="submit_form" />
```

**Handler Registration** (in Rust):
```rust
#[derive(Clone, Debug)]
pub enum Message {
    Increment,
    Decrement,
    SubmitForm,
}

fn update(state: &mut AppState, message: Message) -> Task<Message> {
    match message {
        Message::Increment => state.count += 1,
        Message::Decrement => state.count -= 1,
        Message::SubmitForm => { /* ... */ }
    }
    Task::none()
}
```

## Running the Examples

### Main Showcase
```bash
cd examples/styling
cargo run --bin styling
```

**What it demonstrates:**
- All layout attributes (padding, spacing, sizing)
- All style attributes (colors, borders, shadows, gradients)
- Theming system with light/dark themes
- Style classes with state variants
- Data binding with counter
- Complete feature reference

### State Transitions Demo
```bash
cd examples/styling
cargo run --bin state-demo
```

**What it demonstrates:**
- Hover state styling
- Active (pressed) state styling
- Combined hover:active states
- Disabled state styling
- Visual feedback for all state transitions
- Educational explanations of each state

### Theme Switching Demo
```bash
cd examples/styling
cargo run --bin theme-demo  # Not yet implemented
```

## XML Schema Reference

### Root Structure
```xml
<?xml version="1.0" encoding="UTF-8"?>
<dampen>
    <!-- Optional: Theme definitions -->
    <themes>
        <theme name="...">...</theme>
    </themes>
    
    <!-- Optional: Style class definitions -->
    <styles>
        <style name="...">...</style>
    </styles>
    
    <!-- Optional: Set global theme -->
    <global_theme name="theme_name" />
    
    <!-- Required: Root widget -->
    <column|row|container|...>
        <!-- Child widgets -->
    </column>
</dampen>
```

### Widget Types

| Widget | Description | Children |
|--------|-------------|----------|
| `<column>` | Vertical layout container | Multiple |
| `<row>` | Horizontal layout container | Multiple |
| `<container>` | Single-child wrapper | One |
| `<text>` | Text display | None |
| `<button>` | Interactive button | None |

### Common Attributes

**All widgets support:**
- Layout: `padding`, `spacing`, `width`, `height`, `min_width`, `max_width`, `min_height`, `max_height`, `align_items`, `justify_content`, `align_self`, `direction`
- Style: `background`, `color`, `border_width`, `border_color`, `border_radius`, `border_style`, `shadow`, `opacity`, `transform`
- Classes: `class="class_name"`
- States: `hover:*`, `active:*`, `focus:*`, `disabled:*`, `hover:active:*`

**Widget-specific:**
- `<text>`: `value`, `size`, `weight`
- `<button>`: `label`, `on_click`, `disabled`

## Editing the UI

1. **Open** `ui/main.dampen` in your editor
2. **Make changes**:
   - Change `padding="40"` to `padding="60"`
   - Change `background="#3498db"` to `background="#e74c3c"`
   - Add `shadow="4 4 8 #00000030"` to a container
   - Modify state variants in style classes
3. **Save** the file
4. **Re-run** the example to see your changes

## Development Workflow

### Hot-Reload (Future)
```bash
dampen run
```
Changes to `.dampen` files will automatically reload the UI without recompiling Rust code.

### Production Build (Future)
```bash
dampen build --release
```
Generates static Rust code for zero-runtime overhead.

## Architecture

**Dampen follows a dual-mode architecture:**

1. **Development Mode** (current):
   - Runtime interpretation of XML
   - Parse `.dampen` files at startup
   - State preservation across reloads
   - Fast iteration cycle

2. **Production Mode** (planned):
   - Static code generation
   - Compile-time type checking
   - Zero runtime parsing overhead
   - Optimal performance

## Key Takeaways

✅ **Declarative UI**: All layout and style defined in XML  
✅ **Type Safety**: Attributes validated at parse time  
✅ **Separation of Concerns**: UI in `.dampen`, logic in Rust  
✅ **State Management**: Automatic state transitions with visual feedback  
✅ **Theming**: Consistent design system with reusable themes  
✅ **Flexible Styling**: Inline styles + reusable classes + state variants  
✅ **Data Binding**: Dynamic values with `{expression}` syntax  
✅ **Backend Agnostic**: Core IR independent of Iced implementation

## Next Steps

### Try These Modifications

**Layout Experiments:**
```xml
<!-- Change container sizing -->
<container width="50%" />  <!-- Try: "fill", "200", "shrink" -->

<!-- Adjust spacing -->
<column spacing="30" padding="50">...</column>
```

**Style Experiments:**
```xml
<!-- Add gradient background -->
<container background="linear-gradient(45deg, #667eea, #764ba2)" />

<!-- Add shadow on hover -->
<container hover:shadow="0 8 16 #00000040" />

<!-- Combine multiple states -->
<button 
    hover:background="#5dade2" 
    active:background="#2874a6"
    hover:active:background="#1c5d8a" />
```

**Theme Experiments:**
```xml
<!-- Create a new theme -->
<theme name="ocean">
    <palette 
        primary="#006994"
        secondary="#0099cc"
        background="#e0f2f7" />
</theme>

<!-- Switch global theme -->
<global_theme name="ocean" />
```

## Troubleshooting

**Error: "Failed to parse UI"**
- Check XML syntax (closing tags, quotes, attribute names)
- Validate color formats (hex must start with `#`)
- Ensure theme/class names referenced actually exist

**Warning: Attribute not applied**
- Some attributes only work on specific widgets
- Check widget supports the attribute (e.g., `on_click` only on `<button>`)

**State styles not showing**
- State styles only apply during user interaction
- Hover requires mouse movement over widget
- Active requires mouse button held down
- Check browser console for parser warnings

## Documentation

- **Full XML Schema**: See `docs/XML_SCHEMA.md`
- **Styling Guide**: See `docs/STYLING.md`
- **Quick Start**: See `docs/QUICKSTART.md`

## License

This example is part of the Dampen framework, licensed under MIT OR Apache-2.0.
