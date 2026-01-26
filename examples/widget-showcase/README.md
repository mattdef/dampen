# Widget Showcase Example

A comprehensive demonstration of all Dampen widgets with individual examples for each widget type.

## Purpose

This example serves as a reference implementation and testing ground for all Dampen widgets. Each widget has its own `.dampen` file demonstrating its features and usage patterns.

## Widget Examples

### Implemented Widgets

The following widget examples are available in the `ui/` directory:

#### ProgressBar (`ui/progressbar.dampen`)
Demonstrates progress indicators with different styles and value ranges.

**Features**:
- Multiple style variants (primary, success, warning, danger, secondary)
- Custom value ranges
- Percentage display
- Value clamping behavior

#### Tooltip (`ui/tooltip.dampen`)
Shows contextual help text on hover with different positioning options.

**Features**:
- Multiple position variants (top, bottom, left, right, follow_cursor)
- Custom delay settings
- Wrapping different widget types
- Hover interaction

#### Canvas (`ui/canvas.dampen`)
Custom drawing surface for graphics and visualizations.

**Features**:
- Custom `canvas::Program` implementation
- Drawing primitives (paths, fills, strokes)
- Interactive click handling
- Real-time rendering

#### PickList (`ui/picklist.dampen`)
Dropdown selection from a list of options.

**Features**:
- Static option lists
- Selected value binding
- Event handling on selection change
- Placeholder text

#### ComboBox (`ui/combobox.dampen`)
Searchable dropdown with type-ahead functionality.

**Features**:
- Search filtering
- Dynamic option list
- Selected value binding
- Placeholder support

**Note**: ComboBox rendering is not yet implemented. This file demonstrates the XML syntax.

#### Float (`ui/float.dampen`)
Positioned overlay elements like floating action buttons.

**Features**:
- Corner positioning (top-left, top-right, bottom-left, bottom-right)
- Custom offset control
- Z-index layering
- Visibility toggling

**Note**: Float rendering is not yet implemented. This file demonstrates the XML syntax.

#### Grid (`ui/grid.dampen`)
Multi-column responsive layout.

**Features**:
- Configurable column count
- Automatic wrapping
- Spacing and padding
- Equal-width columns

**Note**: Grid rendering is not yet implemented. This file demonstrates the XML syntax.

## Running the Examples

To run the widget showcase:

```bash
cd examples/widget-showcase
dampen run
```

The application will display examples of all implemented widgets.

### Running in Different Modes

**Development Mode (Interpreted with Hot-Reload):**
```bash
cd examples/widget-showcase
dampen run
```

The UI will reload automatically when you modify `.dampen` files.

**Production Mode (Codegen):**
```bash
# Debug build
dampen build -p widget-showcase

# Release build (optimized)
dampen build --release -p widget-showcase
# or equivalently:
dampen release -p widget-showcase

# Run
./target/release/widget-showcase
```

**Framework Development (using cargo directly):**
```bash
# Interpreted mode
cargo run -p widget-showcase

# Codegen mode
cargo build -p widget-showcase --release --no-default-features --features codegen
./target/release/widget-showcase
```

## File Structure

```
widget-showcase/
├── src/
│   └── main.rs           # Application entry point with widget programs
├── ui/
│   ├── main.dampen      # Main layout switching between widgets
│   ├── progressbar.dampen
│   ├── tooltip.dampen
│   ├── canvas.dampen
│   ├── picklist.dampen
│   ├── combobox.dampen  # Not yet rendered
│   ├── float.dampen     # Not yet rendered
│   └── grid.dampen      # Not yet rendered
├── Cargo.toml
└── README.md             # This file
```

## Adding New Widget Examples

To add a new widget example:

1. Create a new `.dampen` file in `ui/`
2. Define the widget with all its attributes
3. Add event handlers in `src/main.rs` if needed
4. Register handlers in the `HandlerRegistry`
5. Update this README

Example:

```xml
<!-- ui/my_widget.dampen -->
<my_widget
    attribute1="value"
    attribute2="{binding}"
    on_event="handler"
/>
```

```rust
// src/main.rs
fn handler(model: &mut Model) {
    // Handle event
}

// In main()
registry.register_simple("handler", |model: &mut dyn Any| {
    let model = model.downcast_mut::<Model>().unwrap();
    handler(model);
});
```

## Testing Widgets

Each widget example should demonstrate:

1. **Basic Usage**: Minimal configuration
2. **All Attributes**: Complete attribute set
3. **Event Handling**: Interactive behavior
4. **Data Binding**: Dynamic values from model
5. **Edge Cases**: Min/max values, empty states, etc.

## Known Limitations

### Not Yet Implemented

The following widgets are defined but not yet implemented in the widget builder:

- **ComboBox**: Parsing works, rendering shows `todo!()` error
- **Grid**: Parsing works, rendering shows `todo!()` error
- **Float**: Parsing works, rendering shows `todo!()` error

These widgets can be added to `.dampen` files to test parsing, but the application will panic when trying to render them.

### Workarounds

- Use **PickList** instead of ComboBox for dropdown functionality
- Use **Row/Column** instead of Grid for layout
- Use **Container** with absolute positioning instead of Float (limited support)

## Contributing

When implementing new widget renderers:

1. Add the rendering logic to `crates/dampen-iced/src/builder.rs`
2. Add tests to `crates/dampen-iced/tests/widget_rendering_tests.rs`
3. Create an example in this showcase
4. Update this README

## References

- [Dampen Documentation](../../docs/)
- [Widget XML Schema](../../specs/004-advanced-widgets-todo/contracts/xml-schema.md)
- [Iced Widget Documentation](https://docs.rs/iced/latest/iced/widget/)

## License

This example is part of the Dampen framework and follows the same licensing terms.
