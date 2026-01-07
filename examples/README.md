# Gravity Examples

This directory contains comprehensive examples demonstrating all features of the Gravity UI framework.

## Available Examples

### 1. Hello World (`hello-world/`)
**Basic introduction to Gravity**

The simplest possible Gravity application - displays text in a column layout.

```bash
cargo run -p hello-world
```

**What you'll learn:**
- Basic XML structure
- Text widgets
- Column layout
- Minimal Rust integration

---

### 2. Counter (`counter/`)
**Interactive UI with event handlers**

A click counter demonstrating event handling and state management.

```bash
cargo run -p counter
```

**What you'll learn:**
- Button widgets with `on_click` handlers
- Data binding with `{count}` syntax
- State management in Rust
- Message passing pattern

---

### 3. Todo App (`todo-app/`)
**Modern application with advanced widgets** ⭐ **NEW: Advanced Widget Showcase**

Full-featured todo application demonstrating ProgressBar, Canvas, Tooltip, PickList, and Image widgets with category management, priority levels, and completion tracking.

```bash
cargo run -p todo-app
```

**What you'll learn:**
- `#[derive(UiModel)]` for data binding
- **ProgressBar** for visual progress tracking
- **Canvas** with custom `canvas::Program` for statistics visualization
- **Tooltip** for contextual help
- **PickList** for dropdown selections
- **Image** widgets for priority indicators
- Complex state management with computed properties
- Event handlers with parameters
- CRUD operations (Create, Read, Update, Delete)
- Category and priority management
- Data filtering and statistics

**Featured Widgets:**
- ProgressBar (completion tracking)
- Canvas (7-day trend chart)
- Tooltip (help text on buttons)
- PickList (category, priority, filter dropdowns)
- Image (priority icons)
- Toggler (dark mode)
- Standard widgets (Text, Button, TextInput, Row, Column, Scrollable, Rule)

See [todo-app/README.md](todo-app/README.md) for detailed feature documentation.

---

### 4. Widget Showcase (`widget-showcase/`)
**Comprehensive widget reference** 📚 **NEW: Complete Widget Catalog**

Individual examples for each Gravity widget type with feature demonstrations.

```bash
cargo run -p widget-showcase
```

**What you'll learn:**
- All available Gravity widgets
- Widget-specific attributes and features
- Event handling patterns for each widget
- Best practices and usage examples
- Testing widget implementations

**Included Widgets:**
- ProgressBar (with all style variants)
- Tooltip (all positions and delays)
- Canvas (custom drawing examples)
- PickList (dropdown selections)
- ComboBox (searchable dropdown - XML only)
- Grid (multi-column layouts - XML only)
- Float (positioned overlays - XML only)

**Note**: Some widgets (ComboBox, Grid, Float) have XML examples but rendering is not yet implemented.

See [widget-showcase/README.md](widget-showcase/README.md) for detailed widget documentation.

---

### 6. Styling (`styling/`)
**Comprehensive styling showcase** ⭐ **START HERE FOR STYLING REFERENCE**

Complete demonstration of ALL layout, sizing, theming, and styling features.

```bash
# Main comprehensive showcase
cargo run --bin styling

# State transitions demo
cargo run --bin state-demo
```

**What you'll learn:**
- Layout attributes (padding, spacing, alignment)
- Sizing modes (fixed, fill, shrink, percentage)
- Inline styles (colors, borders, shadows, gradients)
- Theming system (palettes, typography, spacing)
- Style classes with state variants
- State-based styling (hover, active, disabled, combined states)
- Complete feature reference

**Files:**
- `src/main.rs` - Main comprehensive example
- `src/state_demo.rs` - State transitions demo
- `ui/main.gravity` - Comprehensive showcase UI
- `ui/state_demo.gravity` - State demo UI
- `ui/theme_demo.gravity` - Theme switching UI
- `README.md` - **Detailed styling documentation** 📚

---

### 7. Responsive (`responsive/`)
**Responsive layouts with breakpoints**

Demonstrates responsive design with mobile/tablet/desktop breakpoints.

```bash
cargo run -p responsive
```

**What you'll learn:**
- Breakpoint-based styling (`mobile:`, `tablet:`, `desktop:`)
- Viewport-aware layouts
- Responsive attribute resolution
- Window resize handling

---

### 8. Hot Reload Test (`hot-reload-test/`)
**Development workflow demo**

Tests hot-reload functionality for rapid UI iteration.

```bash
cargo run -p hot-reload-test
```

**What you'll learn:**
- File watching for `.gravity` files
- Live UI updates without recompilation
- State preservation across reloads
- Development workflow optimization

---

### 9. Builder Demo (`builder-demo/`)
**Style classes and inheritance**

Advanced styling with reusable style classes and inheritance.

```bash
cargo run -p class-demo
```

**What you'll learn:**
- Style class definitions
- Class inheritance with `extends`
- Cascade resolution
- Reusable component styling

---

## Feature Comparison Matrix

| Feature | hello | counter | todo-app | showcase | styling | responsive | hot-reload |
|---------|-------|---------|----------|----------|---------|------------|------------|
| **Basic Widgets** | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| **Advanced Widgets** | ❌ | ❌ | ✅ | ✅ | ❌ | ❌ | ❌ |
| **ProgressBar** | ❌ | ❌ | ✅ | ✅ | ❌ | ❌ | ❌ |
| **Canvas** | ❌ | ❌ | ✅ | ✅ | ❌ | ❌ | ❌ |
| **Tooltip** | ❌ | ❌ | ✅ | ✅ | ❌ | ❌ | ❌ |
| **PickList** | ❌ | ❌ | ✅ | ✅ | ❌ | ❌ | ❌ |
| **Image** | ❌ | ❌ | ✅ | ❌ | ❌ | ❌ | ❌ |
| **Event Handlers** | ❌ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| **Data Binding** | ❌ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| **Layout Attributes** | ❌ | ❌ | ❌ | ❌ | ✅ | ✅ | ❌ |
| **Inline Styles** | ❌ | ❌ | ❌ | ❌ | ✅ | ✅ | ❌ |
| **Theming** | ❌ | ❌ | ❌ | ❌ | ✅ | ✅ | ❌ |
| **Style Classes** | ❌ | ❌ | ❌ | ❌ | ✅ | ✅ | ❌ |
| **State Variants** | ❌ | ❌ | ❌ | ❌ | ✅ | ❌ | ❌ |
| **Breakpoints** | ❌ | ❌ | ❌ | ❌ | ❌ | ✅ | ❌ |
| **Hot Reload** | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ✅ |

## Learning Path

### For Beginners
1. Start with **hello-world** to understand basic structure
2. Try **counter** to learn event handling
3. Explore **todo-app** for data binding

### For UI Designers
1. Jump to **styling** for comprehensive styling reference 📚
2. Check **responsive** for adaptive layouts
3. Try **class-demo** for reusable styles

### For Developers
1. Review **counter** for event patterns
2. Study **todo-app** for state management
3. Use **hot-reload-test** for development workflow

## Running All Examples

Build all examples:
```bash
cargo build --examples
```

Run a specific example:
```bash
cargo run -p <example-name>
# or
cargo run --example <example-name>
```

List all examples:
```bash
ls examples/
```

## Complete Feature List

### Layout System
- ✅ Padding (single value or TRBL)
- ✅ Spacing between children
- ✅ Width/height (fixed, fill, shrink, fill_portion, percentage)
- ✅ Min/max width/height constraints
- ✅ Alignment (align_items, justify_content, align_self)
- ✅ Direction (horizontal, vertical, reverse)

### Styling System
- ✅ Background (solid colors, gradients, images)
- ✅ Text color
- ✅ Borders (width, color, radius, style)
- ✅ Shadows (offset, blur, color)
- ✅ Opacity
- ✅ Transform (scale, rotate, translate)

### Theming
- ✅ Theme definitions (palette, typography, spacing)
- ✅ Multiple themes (light, dark, custom)
- ✅ Global theme selection
- ✅ Local theme overrides
- ✅ Built-in themes (light, dark, default)

### Style Classes
- ✅ Reusable style definitions
- ✅ State variants (hover, focus, active, disabled)
- ✅ Combined states (hover:active, etc.)
- ✅ Class inheritance with `extends`
- ✅ Cascade resolution (inline > class > theme)

### Responsive Design
- ✅ Breakpoint-based attributes (mobile, tablet, desktop)
- ✅ Viewport-aware rendering
- ✅ Window resize detection
- ✅ Breakpoint threshold configuration

### Data Binding
- ✅ Expression syntax `{field_name}`
- ✅ Nested field access `{user.name}`
- ✅ Method calls `{user.display()}`
- ✅ Binary operations `{count * 2}`
- ✅ Conditionals `{if active "Yes" else "No"}`

### Event Handling
- ✅ `on_click` handlers
- ✅ Typed message passing
- ✅ Handler registry
- ✅ Type-safe dispatch

### Development Tools
- ✅ Hot-reload with file watching
- ✅ Parse error overlays
- ✅ State persistence across reloads
- ✅ Verbose logging mode
- ✅ UI validation (`gravity check`)

## File Structure Reference

```
examples/<example>/
├── Cargo.toml          # Package manifest
├── README.md           # Example-specific docs (optional)
├── src/
│   └── main.rs         # Rust application code
└── ui/
    └── main.gravity    # UI definition
```

## Why External .gravity Files?

### Benefits
1. **Separation of Concerns**: UI designers work on XML, developers on Rust
2. **Hot-Reload**: Modify UI without recompiling (< 500ms reload time)
3. **Readability**: XML is more readable for UI structure
4. **Tooling**: Can use XML editors, validators, formatters
5. **Collaboration**: Designers and developers work independently

### Trade-offs
- Requires file I/O at runtime (minimal performance impact)
- Need to manage file paths
- Slightly more complex initial setup

## Getting Started

### 1. Explore Styling Example (Recommended)
```bash
cd examples/styling
cat ui/main.gravity         # See the comprehensive UI
cat README.md               # Read detailed docs
cargo run --bin styling     # Run the showcase
```

### 2. Modify the UI
Edit `ui/main.gravity` and change:
- `padding="40"` → `padding="60"`
- `background="#3498db"` → `background="#e74c3c"`
- Add `shadow="4 4 8 #00000030"` to containers
- Modify state variants in style classes

### 3. Run Again
```bash
cargo run --bin styling
```

See your changes immediately!

## Next Steps

After exploring the examples:

1. **Read the docs**: See `docs/` directory for detailed guides
   - `QUICKSTART.md` - Getting started guide
   - `STYLING.md` - Complete styling reference
   - `XML_SCHEMA.md` - Full XML schema documentation

2. **Create your own app**: Use `gravity new my-app`

3. **Try hot-reload mode**: `gravity dev --ui ui --file main.gravity`

4. **Join the community**: Share your creations and get help

## Documentation

- [Quick Start Guide](../docs/QUICKSTART.md)
- [Styling Guide](../docs/STYLING.md) - **Complete reference** 📚
- [XML Schema Reference](../docs/XML_SCHEMA.md)
- [Styling Example README](styling/README.md) - **Detailed feature guide** 📚
- [API Documentation](https://docs.rs/gravity-core)

## Contributing

Found a bug or have an example idea? Open an issue or PR!

## License

All examples are provided under the same MIT OR Apache-2.0 license as Gravity.
