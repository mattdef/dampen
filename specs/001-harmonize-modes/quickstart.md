# Quickstart: Unified Dampen Development

This guide introduces the harmonized development workflow, ensuring that your `dampen run` experience matches `dampen build` pixel-for-pixel.

## 1. Unified Layouts

You can now use layout attributes on ANY widget. Codegen will automatically handle the wrapping.

```xml
<Window title="My App">
    <!-- Center a button easily without a container -->
    <Button label="Click Me" 
            align_x="center" 
            align_y="center" 
            width="200" 
            height="50" />
            
    <!-- Spacing and padding on Columns -->
    <Column spacing="20" padding="30">
        <Text value="Hello" size="24" />
    </Column>
</Window>
```

## 2. State-Aware Styling in Production

Define your styles once, and they work in both modes.

```xml
<Style>
    .primary-btn {
        background: #007bff;
        color: white;
        border-radius: 4;
    }
    
    /* These hover states now work in release builds! */
    .primary-btn:hover {
        background: #0056b3;
        shadow-color: #00000040;
        shadow-offset: 0 2;
    }
    
    .primary-btn:active {
        background: #004494;
    }
</Style>

<Button label="Submit" class="primary-btn" />
```

## 3. Visual Regression Testing

To ensure your UI stays perfect, run the visual test suite:

```bash
# Run all visual tests
cargo test --package dampen-visual-tests

# Check specifically for regressions in production mode
dampen test visual
```

## 4. Migration Guide

If you have existing `.dampen` files, note the following attribute changes:

- **Looping**: Use `each="item" in="{items}"` instead of `for="item in items"`.
- **Toggler**: Use `toggled="{is_active}"` instead of `active="..."`.
- **Image**: Use `src="..."` instead of `path="..."`.
