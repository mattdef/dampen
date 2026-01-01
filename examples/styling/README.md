# Styling Example

This example demonstrates Gravity's layout and styling capabilities using **external `.gravity` files**.

## Features Demonstrated

### Layout Attributes
- `padding` - Inner spacing within widgets
- `spacing` - Gap between child widgets
- `width` - Fixed, fill, shrink, percentage
- `height` - Fixed, fill, shrink, percentage
- `min_width`, `max_width`, `min_height`, `max_height` - Constraints
- `align_items`, `justify_content` - Alignment

### Style Attributes
- `background` - Color, gradient, or image
- `color` - Text color
- `border_width`, `border_color`, `border_radius`, `border_style`
- `shadow` - Offset, blur, color
- `opacity` - Transparency
- `transform` - Scale, rotate, translate

### Binding Support
- Dynamic values with `{expression}` syntax
- Example: `text value="Count: {count}"`

## File Structure

```
examples/styling/
├── Cargo.toml
├── src/
│   └── main.rs          # Runtime that loads and renders the UI
└── ui/
    ├── main.gravity     # Main UI definition
    └── themes.gravity   # Theme definitions
```

## Running the Example

```bash
cargo run -p styling
```

## How It Works

1. **Startup**: The Rust code loads `ui/main.gravity` and parses it
2. **Rendering**: The parsed IR is converted to Iced widgets
3. **Interaction**: Button clicks update the counter
4. **Hot-Reload**: The example can reload the UI file (minimal implementation)

## Example UI (ui/main.gravity)

```xml
<?xml version="1.0" encoding="UTF-8"?>
<gravity>
    <themes>
        <theme name="custom">
            <palette 
                primary="#3498db" 
                secondary="#2ecc71" 
                background="#ecf0f1" 
                text="#2c3e50" />
            <typography font_family="Inter, sans-serif" font_size_base="16" />
            <spacing unit="8" />
        </theme>
    </themes>
    
    <global_theme name="custom" />
    
    <column padding="40" spacing="20">
        <text value="Styled App" size="32" weight="bold" color="#2c3e50" />
        <text value="Count: {count}" size="24" color="#3498db" background="#ffffff" 
              padding="10 20" border_radius="8" />
        
        <row spacing="10">
            <button label="Increment" on_click="increment" 
                    background="#27ae60" color="#ffffff" 
                    padding="12 24" border_radius="4" width="120" />
            <button label="Decrement" on_click="decrement" 
                    background="#e74c3c" color="#ffffff" 
                    padding="12 24" border_radius="4" width="120" />
        </row>
        
        <button label="Reset" on_click="reset" 
                background="transparent" color="#3498db"
                border_width="2" border_color="#3498db"
                padding="12 24" border_radius="4" />
        
        <container background="#ffffff" padding="20" border_radius="8" 
                   width="fill" border_width="1" border_color="#e0e0e0">
            <text value="This container demonstrates background, padding, border, and width" 
                  color="#6c757d" size="14" />
        </container>
    </column>
</gravity>
```

### Structure Breakdown

- **`<gravity>`**: Root element containing themes and widgets
- **`<themes>`**: Defines theme definitions (palettes, typography, spacing)
- **`<global_theme>`**: Sets which theme to use globally
- **`<column>`**: Root widget with layout attributes
- **Widgets**: Standard widgets with style and layout attributes

## Editing the UI

1. Open `ui/main.gravity` in your editor
2. Make changes (e.g., change padding, colors, text)
3. Save the file
4. The example will reload on next interaction

## Key Takeaways

✅ **Declarative UI**: All layout and style in XML  
✅ **Type Safety**: Attributes are validated at parse time  
✅ **Binding Support**: Dynamic values with `{expression}`  
✅ **Separation of Concerns**: UI in `.gravity`, logic in Rust  
✅ **Hot-Reload Ready**: Can reload UI without recompiling

## Next Steps

Try modifying `ui/main.gravity`:
- Change `padding="40"` to `padding="60"`
- Change `background="#3498db"` to `background="#e74c3c"`
- Add `width="50%"` to a container
- Add `shadow="4 4 8 #00000030"` to a button

Then run the example again to see your changes!
