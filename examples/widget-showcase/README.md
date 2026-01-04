# Widget Showcase Example

A comprehensive demonstration of all Gravity widgets with individual examples for each widget type.

## Purpose

This example serves as a reference implementation and testing ground for all Gravity widgets. Each widget has its own `.gravity` file demonstrating its features and usage patterns.

## Widget Examples

### Implemented Widgets

The following widget examples are available in the `ui/` directory:

#### ProgressBar (`ui/progressbar.gravity`)
Demonstrates progress indicators with different styles and value ranges.

**Features**:
- Multiple style variants (primary, success, warning, danger, secondary)
- Custom value ranges
- Percentage display
- Value clamping behavior

#### Tooltip (`ui/tooltip.gravity`)
Shows contextual help text on hover with different positioning options.

**Features**:
- Multiple position variants (top, bottom, left, right, follow_cursor)
- Custom delay settings
- Wrapping different widget types
- Hover interaction

#### Canvas (`ui/canvas.gravity`)
Custom drawing surface for graphics and visualizations.

**Features**:
- Custom `canvas::Program` implementation
- Drawing primitives (paths, fills, strokes)
- Interactive click handling
- Real-time rendering

#### PickList (`ui/picklist.gravity`)
Dropdown selection from a list of options.

**Features**:
- Static option lists
- Selected value binding
- Event handling on selection change
- Placeholder text

#### ComboBox (`ui/combobox.gravity`)
Searchable dropdown with type-ahead functionality.

**Features**:
- Search filtering
- Dynamic option list
- Selected value binding
- Placeholder support

**Note**: ComboBox rendering is not yet implemented. This file demonstrates the XML syntax.

#### Float (`ui/float.gravity`)
Positioned overlay elements like floating action buttons.

**Features**:
- Corner positioning (top-left, top-right, bottom-left, bottom-right)
- Custom offset control
- Z-index layering
- Visibility toggling

**Note**: Float rendering is not yet implemented. This file demonstrates the XML syntax.

#### Grid (`ui/grid.gravity`)
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
cargo run
```

The application will display examples of all implemented widgets.

## Hot Reload Development

For rapid UI iteration:

```bash
# Run the app
cargo run &

# In another terminal, start hot-reload
gravity dev --ui ui --file main.gravity
```

Edit any `.gravity` file in the `ui/` directory and see changes instantly!

## File Structure

```
widget-showcase/
├── src/
│   └── main.rs           # Application entry point with widget programs
├── ui/
│   ├── main.gravity      # Main layout switching between widgets
│   ├── progressbar.gravity
│   ├── tooltip.gravity
│   ├── canvas.gravity
│   ├── picklist.gravity
│   ├── combobox.gravity  # Not yet rendered
│   ├── float.gravity     # Not yet rendered
│   └── grid.gravity      # Not yet rendered
├── Cargo.toml
└── README.md             # This file
```

## Adding New Widget Examples

To add a new widget example:

1. Create a new `.gravity` file in `ui/`
2. Define the widget with all its attributes
3. Add event handlers in `src/main.rs` if needed
4. Register handlers in the `HandlerRegistry`
5. Update this README

Example:

```xml
<!-- ui/my_widget.gravity -->
<my_widget
    attribute1="value"
    attribute2="{binding}"
    on_event="handler"
/>
```

```rust
// src/main.rs
#[ui_handler]
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

These widgets can be added to `.gravity` files to test parsing, but the application will panic when trying to render them.

### Workarounds

- Use **PickList** instead of ComboBox for dropdown functionality
- Use **Row/Column** instead of Grid for layout
- Use **Container** with absolute positioning instead of Float (limited support)

## Contributing

When implementing new widget renderers:

1. Add the rendering logic to `crates/gravity-iced/src/builder.rs`
2. Add tests to `crates/gravity-iced/tests/widget_rendering_tests.rs`
3. Create an example in this showcase
4. Update this README

## References

- [Gravity Documentation](../../docs/)
- [Widget XML Schema](../../specs/004-advanced-widgets-todo/contracts/xml-schema.md)
- [Iced Widget Documentation](https://docs.rs/iced/latest/iced/widget/)

## License

This example is part of the Gravity framework and follows the same licensing terms.
