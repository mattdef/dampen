# Theming Example

This example demonstrates the complete Dampen theming system including:

- **Runtime theme switching** - Switch themes instantly without restarting
- **Theme inheritance** - Themes can extend other themes
- **Palette colors** - Full color palette with 9 color definitions
- **Typography** - Custom font family, sizes, and line height
- **Spacing scale** - Base unit with multipliers
- **Widget overrides** - Override theme properties on individual widgets
- **Hot-reload** - Edit theme.dampen and see changes instantly

## Running the Example

```bash
# Development mode with hot-reload
cargo run -p theming

# Or using dampen CLI
cd theming
dampen run
```

## Theme File

The theme is defined in `src/ui/theme/theme.dampen`. Try:

1. Changing color values
2. Adding a new theme with `extends`
3. Modifying typography or spacing

Changes hot-reload instantly in development mode!

## Available Themes

| Theme | Description |
|-------|-------------|
| `light` | Default light theme |
| `dark` | Dark variant (extends base) |
| `brand` | Custom branded colors |
| `high_contrast` | Accessibility-focused high contrast |

## Key Files

- `src/ui/theme/theme.dampen` - Theme definitions
- `src/ui/window.dampen` - UI with theme showcase
- `src/ui/window.rs` - Model and handlers
