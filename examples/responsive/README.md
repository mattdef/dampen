# Responsive Example

This example demonstrates **responsive layout** with constraints using external `.gravity` files.

## Features Demonstrated

### Layout Constraints
- `width="400"` - Fixed width
- `width="fill"` - Fill available space
- `min_width="300"` - Minimum width constraint
- `max_width="600"` - Maximum width constraint
- `align_items="center"` - Alignment
- `spacing="10"` - Child spacing

### Container Types
- Fixed width containers
- Fill width containers
- Constrained containers (min/max)
- Nested layouts

## File Structure

```
examples/responsive/
├── Cargo.toml
├── src/
│   └── main.rs          # Runtime
└── ui/
    └── main.gravity     # Responsive UI definition
```

## Running the Example

```bash
cargo run -p responsive
```

## Example UI (ui/main.gravity)

```xml
<gravity>
    <column padding="40" spacing="20">
        <text value="Responsive Layout Example" size="32" weight="bold" />
        
        <!-- Fixed width -->
        <container width="400" padding="20" background="#ecf0f1">
            <text value="Fixed Width (400px)" size="18" weight="bold" />
        </container>
        
        <!-- Fill width -->
        <container width="fill" padding="20" background="#d5dbdb">
            <text value="Fill Width" size="18" weight="bold" />
        </container>
        
        <!-- Min/Max constraints -->
        <container width="fill" min_width="300" max_width="600"
                   padding="20" background="#e8f4f8" border_width="2">
            <text value="Min: 300px, Max: 600px" size="18" />
        </container>
        
        <!-- Alignment -->
        <row spacing="10" align_items="center">
            <button label="Left" on_click="left" width="100" />
            <button label="Center" on_click="center" width="100" />
            <button label="Right" on_click="right" width="100" />
        </row>
        
        <!-- Nested -->
        <container padding="20" background="#ffffff">
            <text value="Nested Layout" size="18" weight="bold" />
            <column spacing="10">
                <container padding="10" background="#f8f9fa">
                    <text value="Level 1" size="14" />
                </container>
                <container padding="10" background="#e9ecef">
                    <text value="Level 2" size="14" />
                </container>
            </column>
        </container>
    </column>
</gravity>
```

## Understanding Constraints

### Fixed Width
```xml
<container width="400">...</container>
```
Always 400px wide, regardless of parent.

### Fill Width
```xml
<container width="fill">...</container>
```
Fills all available space in parent.

### Min/Max Constraints
```xml
<container width="fill" min_width="300" max_width="600">...</container>
```
- Fills available space
- But never smaller than 300px
- And never larger than 600px

### Percentage Width
```xml
<container width="80%">...</container>
```
80% of parent width.

## Testing Responsiveness

1. Run the example
2. Resize the window
3. Observe how containers adapt:
   - Fixed width stays the same
   - Fill width expands/shrinks
   - Constrained width respects min/max

## Key Concepts

### Layout Resolution
The parser converts XML attributes to `LayoutConstraints`:
```rust
// XML: width="400"
layout.width = Some(Length::Fixed(400.0));

// XML: width="fill"
layout.width = Some(Length::Fill);

// XML: min_width="300"
layout.min_width = Some(300.0);
```

### Backend Mapping
The Iced backend maps constraints to Iced types:
```rust
match length {
    Length::Fixed(pixels) => iced::Length::Fixed(pixels),
    Length::Fill => iced::Length::Fill,
    Length::Shrink => iced::Length::Shrink,
    Length::FillPortion(n) => iced::Length::FillPortion(n),
}
```

## Try It Yourself

Edit `ui/main.gravity` and experiment:

1. **Change fixed width**: `width="400"` → `width="500"`
2. **Add constraints**: Add `min_width="200"` to fill container
3. **Change alignment**: `align_items="center"` → `align_items="end"`
4. **Add nesting**: Create another level of containers

Save and run to see changes!

## Architecture

```
ui/main.gravity (XML)
    ↓ (parse)
GravityDocument (IR)
    ↓ (render)
Iced Widgets (UI)
```

The separation allows:
- UI designers to work on `.gravity` files
- Developers to work on Rust logic
- Hot-reload without recompilation
