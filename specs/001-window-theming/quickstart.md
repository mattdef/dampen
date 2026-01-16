# Quickstart: Window Theming

**Feature**: 001-window-theming  
**Audience**: Dampen application developers

---

## Overview

Dampen's theming system provides customizable window appearance through a global theme file. Define your colors, typography, and spacing once, and they apply to all windows in your application.

## Getting Started

### 1. Create the Theme File

Create `src/ui/theme/theme.dampen`:

```xml
<?xml version="1.0" encoding="UTF-8" ?>
<dampen version="1.0">
    <themes>
        <!-- Light theme -->
        <theme name="light">
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
            <typography 
                font_family="Inter, sans-serif"
                font_size_base="16"
                font_size_small="12"
                font_size_large="24" />
            <spacing unit="8" />
        </theme>
        
        <!-- Dark theme -->
        <theme name="dark">
            <palette 
                primary="#5dade2" 
                secondary="#52be80"
                success="#27ae60"
                warning="#f39c12"
                danger="#ec7063"
                background="#2c3e50"
                surface="#34495e"
                text="#ecf0f1"
                text_secondary="#95a5a6" />
            <typography 
                font_family="Inter, sans-serif"
                font_size_base="16"
                font_size_small="12"
                font_size_large="24" />
            <spacing unit="8" />
        </theme>
    </themes>
    
    <!-- Default theme (optional - follows system if omitted) -->
    <default_theme name="light" />
</dampen>
```

### 2. Run Your Application

That's it! Your application will now use the theme.

```bash
# Development mode with hot-reload
dampen run

# Production build
dampen build
```

## Theme Structure

### Palette Colors

| Color | Purpose |
|-------|---------|
| `primary` | Main brand color, primary buttons |
| `secondary` | Secondary accents |
| `success` | Success messages, confirmations |
| `warning` | Warnings, caution states |
| `danger` | Errors, destructive actions |
| `background` | Application background |
| `surface` | Card/container backgrounds |
| `text` | Primary text color |
| `text_secondary` | Muted/secondary text |

### Typography

| Property | Description | Default |
|----------|-------------|---------|
| `font_family` | Font stack | `"sans-serif"` |
| `font_size_base` | Base text size (px) | `16` |
| `font_size_small` | Small text size | `12` |
| `font_size_large` | Large/heading size | `20` |
| `font_weight` | Default weight | `"normal"` |
| `line_height` | Line height multiplier | `1.5` |

### Spacing

| Property | Description | Default |
|----------|-------------|---------|
| `unit` | Base spacing unit (px) | `8` |

Spacing multipliers: `1x` = 8px, `2x` = 16px, `3x` = 24px, etc.

## Runtime Theme Switching

### Option 1: Binding Expression

Bind the theme to a model field:

```rust
// In your Model
#[derive(UiModel)]
struct Model {
    current_theme: String,
}
```

```xml
<!-- In any .dampen file -->
<button label="Toggle Theme" on_click="toggle_theme" />
```

```rust
// Handler
fn toggle_theme(model: &mut Model) {
    model.current_theme = if model.current_theme == "light" {
        "dark".to_string()
    } else {
        "light".to_string()
    };
}
```

### Option 2: Handler Action

Use the built-in `set_theme` action:

```xml
<row spacing="10">
    <button label="Light" on_click="set_theme('light')" />
    <button label="Dark" on_click="set_theme('dark')" />
</row>
```

## System Theme Detection

By default, Dampen follows the system's dark/light mode preference. To disable:

```xml
<follow_system enabled="false" />
```

Or to always start with a specific theme regardless of system:

```xml
<default_theme name="light" />
<follow_system enabled="false" />
```

## Hot-Reload

When running in development mode (`dampen run`), theme changes are applied instantly:

1. Edit `src/ui/theme/theme.dampen`
2. Save the file
3. Your app updates automatically (within 500ms)

No restart required!

## Existing Apps (Backward Compatibility)

If your app doesn't have a `theme.dampen` file:
- It continues to work exactly as before
- Uses Iced's default light theme
- No changes required to your code

You can add theming incrementally without breaking anything.

## Common Patterns

### Brand-Specific Theme

```xml
<theme name="brand">
    <palette 
        primary="#FF6B35"      <!-- Your brand orange -->
        secondary="#004E89"    <!-- Complementary blue -->
        success="#27ae60"
        warning="#f39c12"
        danger="#e74c3c"
        background="#FAFAFA"
        surface="#FFFFFF"
        text="#1A1A2E"
        text_secondary="#6B7280" />
</theme>
```

### High Contrast Theme

```xml
<theme name="high-contrast">
    <palette 
        primary="#0000FF"
        secondary="#FF00FF"
        success="#00FF00"
        warning="#FFFF00"
        danger="#FF0000"
        background="#000000"
        surface="#000000"
        text="#FFFFFF"
        text_secondary="#FFFFFF" />
</theme>
```

### Theme with Custom Typography

```xml
<theme name="modern">
    <palette><!-- ... --></palette>
    <typography 
        font_family="'JetBrains Mono', monospace"
        font_size_base="14"
        font_size_small="11"
        font_size_large="20"
        font_weight="normal"
        line_height="1.6" />
</theme>
```

## Troubleshooting

### Theme not applying?

1. Check file location: Must be `src/ui/theme/theme.dampen`
2. Check XML syntax: Run `dampen check` to validate
3. Check theme name: Ensure `<default_theme name="...">` matches a defined theme

### Hot-reload not working?

1. Ensure you're running `dampen run` (not `dampen build`)
2. Check for syntax errors: Invalid XML stops hot-reload
3. Check terminal for error messages

### Colors look wrong?

1. Verify hex format: Use `#RRGGBB` (6 digits) or `#RGB` (3 digits)
2. Check for typos in color names
3. Use a color picker to verify values

## Next Steps

- See [STYLING.md](/docs/STYLING.md) for style classes and state-based styling
- See [XML_SCHEMA.md](/docs/XML_SCHEMA.md) for complete attribute reference
- Check the `examples/styling` project for a complete themed application
