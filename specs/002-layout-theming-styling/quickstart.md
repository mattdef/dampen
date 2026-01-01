# Quickstart: Layout, Sizing, Theming, and Styling

**Feature**: 002-layout-theming-styling  
**Audience**: Gravity Framework Developers  
**Estimated Time**: 20 minutes

## Overview

This guide walks you through using Gravity's layout, sizing, theming, and styling capabilities to build a modern, responsive UI. You'll learn how to control widget sizing, apply themes, create reusable style classes, and make responsive layouts.

---

## Prerequisites

- Gravity Framework installed (v0.2.0+)
- Basic familiarity with Gravity's XML syntax
- Completed the hello-world or counter example

---

## Step 1: Basic Layout and Sizing (5 min)

### Create a New Project

```bash
gravity new styled-app
cd styled-app
```

### Add Layout Attributes

Edit `ui/main.gravity`:

```xml
<column padding="40" spacing="20" align_items="center">
  <text value="Styled Application" size="32" weight="bold" />
  
  <row spacing="10" width="fill" max_width="600">
    <button width="fill" label="Button 1" on_click="action1" />
    <button width="fill" label="Button 2" on_click="action2" />
  </row>
  
  <container 
    width="400" 
    height="200" 
    padding="20" 
    background="#ecf0f1">
    <text value="Fixed size container with padding" />
  </container>
</column>
```

### Run the App

```bash
gravity dev --ui ui --file main.gravity
```

**What you see:**
- Column with 40px padding around content, 20px spacing between children
- Centered children (due to `align_items="center"`)
- Buttons filling available width (capped at 600px)
- Fixed-size container with background color

---

## Step 2: Apply Inline Styles (5 min)

### Add Background and Border Styles

Update `ui/main.gravity`:

```xml
<column padding="40" spacing="20" align_items="center" background="#f8f9fa">
  <text 
    value="Styled Application" 
    size="32" 
    weight="bold" 
    color="#2c3e50" />
  
  <container 
    width="600" 
    padding="30"
    background="#ffffff"
    border_width="2"
    border_color="#3498db"
    border_radius="12"
    shadow="0 4 8 #00000020">
    
    <column spacing="15">
      <text value="Welcome!" size="24" color="#3498db" />
      <text value="This is a styled card with border, radius, and shadow." />
      
      <row spacing="10">
        <button 
          background="#3498db"
          color="#ffffff"
          padding="12 24"
          border_radius="6"
          label="Primary"
          on_click="primary_action" />
        
        <button 
          background="transparent"
          color="#3498db"
          border_width="2"
          border_color="#3498db"
          padding="12 24"
          border_radius="6"
          label="Secondary"
          on_click="secondary_action" />
      </row>
    </column>
  </container>
</column>
```

**What you see:**
- Light gray background on main column
- White card with blue border, rounded corners, and drop shadow
- Styled buttons with solid and outline variants

---

## Step 3: Use Gradients (3 min)

### Add Gradient Backgrounds

```xml
<container 
  width="600" 
  height="200"
  background="linear-gradient(135deg, #667eea 0%, #764ba2 100%)"
  border_radius="12"
  padding="30">
  
  <text 
    value="Gradient Background" 
    size="32" 
    color="#ffffff" 
    weight="bold" />
</container>
```

**Try different gradients:**
- `linear-gradient(90deg, #ff6b6b, #4ecdc4)`
- `linear-gradient(180deg, #f093fb 0%, #f5576c 100%)`
- `linear-gradient(45deg, red, yellow, green)`

---

## Step 4: Define a Theme (7 min)

### Create Theme Definition

Add to top of `ui/main.gravity`:

```xml
<gravity version="1.0">
  <themes>
    <theme name="app_theme">
      <palette
        primary="#3498db"
        secondary="#2ecc71"
        background="#ecf0f1"
        surface="#ffffff"
        text="#2c3e50"
        text_secondary="#7f8c8d"
      />
      <typography
        font_family="Roboto"
        font_size_base="16"
        font_weight="normal"
      />
      <spacing unit="4" />
    </theme>
  </themes>
  
  <!-- UI definition below -->
  <column theme="app_theme" padding="40" spacing="20">
    <!-- ... -->
  </column>
</gravity>
```

### Use Theme Colors

Reference theme colors in styles (future enhancement - manual for now):

```xml
<button background="#3498db" label="Primary Color" />
<!-- Future: background="$primary" -->
```

---

## Step 5: Create Reusable Style Classes (10 min)

### Define Style Classes

Add to `<gravity>` root:

```xml
<styles>
  <style name="card">
    <base
      background="#ffffff"
      padding="20"
      border_width="1"
      border_color="#ddd"
      border_radius="8"
      shadow="0 2 4 #00000010"
    />
  </style>
  
  <style name="button_primary">
    <base
      background="#3498db"
      color="#ffffff"
      padding="12 24"
      border_radius="6"
    />
    <hover background="#2980b9" />
    <active background="#21618c" />
    <disabled opacity="0.5" />
  </style>
  
  <style name="button_secondary">
    <base
      background="transparent"
      color="#3498db"
      border_width="2"
      border_color="#3498db"
      padding="12 24"
      border_radius="6"
    />
    <hover background="rgba(52, 152, 219, 0.1)" />
    <active background="rgba(52, 152, 219, 0.2)" />
  </style>
</styles>
```

### Apply Style Classes

```xml
<column padding="40" spacing="20">
  <container class="card" width="600">
    <column spacing="15">
      <text value="Styled with Classes" size="24" />
      
      <row spacing="10">
        <button class="button_primary" label="Primary" on_click="action1" />
        <button class="button_secondary" label="Secondary" on_click="action2" />
      </row>
    </column>
  </container>
  
  <container class="card" width="600">
    <text value="Another card with the same styling!" />
  </container>
</column>
```

**Benefits:**
- Reusable styles across multiple widgets
- Hover/focus/active states defined once
- Easy to maintain (change class definition, all widgets update)

---

## Step 6: Make it Responsive (5 min)

### Add Breakpoint Attributes

```xml
<column 
  mobile:padding="16"
  tablet:padding="24"
  desktop:padding="40"
  spacing="20"
  align_items="center">
  
  <text 
    mobile:size="24" 
    desktop:size="32" 
    value="Responsive Heading" 
    weight="bold" />
  
  <row 
    mobile:direction="vertical"
    desktop:direction="horizontal"
    mobile:spacing="10"
    desktop:spacing="20"
    width="fill"
    max_width="800">
    
    <container 
      class="card" 
      mobile:width="fill"
      desktop:width="fill_portion(1)">
      <text value="Card 1" />
    </container>
    
    <container 
      class="card" 
      mobile:width="fill"
      desktop:width="fill_portion(1)">
      <text value="Card 2" />
    </container>
  </row>
</column>
```

**Test Responsiveness:**
- Resize window to < 640px (mobile): Vertical layout, smaller text, less padding
- Resize to 640-1024px (tablet): Medium padding
- Resize to > 1024px (desktop): Horizontal layout, larger text, more padding

---

## Step 7: Add State-Based Styling (3 min)

### Interactive Button with States

```xml
<button
  background="#3498db"
  color="#ffffff"
  padding="16 32"
  border_radius="8"
  transform="scale(1.0)"
  
  hover:background="#2980b9"
  hover:transform="scale(1.05)"
  
  active:background="#21618c"
  active:transform="scale(0.95)"
  
  disabled:opacity="0.5"
  disabled:transform="scale(1.0)"
  
  label="Hover and Click Me"
  on_click="action"
/>
```

**Behavior:**
- Default: Blue background, normal size
- Hover: Darker blue, 105% size
- Active (pressed): Even darker, 95% size
- Disabled: 50% opacity, normal size

---

## Complete Example

**`ui/main.gravity`:**

```xml
<gravity version="1.0">
  <themes>
    <theme name="modern">
      <palette
        primary="#667eea"
        secondary="#764ba2"
        background="#f8f9fa"
        surface="#ffffff"
        text="#2c3e50"
      />
      <typography font_family="Inter" font_size_base="16" />
      <spacing unit="4" />
    </theme>
  </themes>
  
  <styles>
    <style name="card">
      <base
        background="#ffffff"
        padding="24"
        border_radius="12"
        shadow="0 4 12 #00000015"
      />
    </style>
    
    <style name="btn_gradient">
      <base
        background="linear-gradient(135deg, #667eea 0%, #764ba2 100%)"
        color="#ffffff"
        padding="16 32"
        border_radius="8"
      />
      <hover opacity="0.9" transform="scale(1.05)" />
      <active opacity="0.8" transform="scale(0.95)" />
    </style>
  </styles>
  
  <column 
    theme="modern"
    mobile:padding="20"
    desktop:padding="60"
    spacing="30"
    align_items="center"
    background="#f8f9fa">
    
    <text 
      mobile:size="28" 
      desktop:size="40" 
      value="Styled App" 
      weight="bold" 
      color="#2c3e50" />
    
    <container 
      class="card" 
      mobile:width="fill"
      desktop:width="800">
      
      <column spacing="20">
        <text value="Welcome to styled Gravity!" size="20" />
        <text 
          value="This example demonstrates layout, sizing, theming, responsive design, and state-based styling." 
          color="#7f8c8d" />
        
        <row 
          mobile:direction="vertical"
          desktop:direction="horizontal"
          spacing="12">
          
          <button 
            class="btn_gradient" 
            width="fill"
            label="Get Started" 
            on_click="start" />
          
          <button 
            background="transparent"
            color="#667eea"
            border_width="2"
            border_color="#667eea"
            padding="16 32"
            border_radius="8"
            hover:background="rgba(102, 126, 234, 0.1)"
            width="fill"
            label="Learn More" 
            on_click="learn" />
        </row>
      </column>
    </container>
  </column>
</gravity>
```

**`src/main.rs`:**

```rust
use gravity_runtime::{GravityApp, run_dev};

#[derive(Default, UiModel, Serialize, Deserialize)]
struct AppModel {
    // Your state here
}

#[ui_handler]
fn start(model: &mut AppModel) {
    println!("Get Started clicked!");
}

#[ui_handler]
fn learn(model: &mut AppModel) {
    println!("Learn More clicked!");
}

fn main() {
    run_dev::<AppModel>("ui/main.gravity").unwrap();
}
```

---

## Hot-Reload Testing

With `gravity dev` running, try modifying:

1. **Layout**: Change `padding="40"` to `padding="80"` - UI updates instantly
2. **Colors**: Change `background="#3498db"` to `background="#e74c3c"` - red button appears
3. **Gradients**: Adjust gradient colors - background updates
4. **Classes**: Modify `<style name="card">` padding - all cards update
5. **Responsive**: Change `desktop:padding="60"` - resize window to see effect

**State Preservation**: Your application state (model) is preserved across all hot-reloads!

---

## Next Steps

### Advanced Topics

1. **Theme Switching**: Implement runtime theme switching (light/dark mode)
2. **Complex Gradients**: Multi-stop gradients with precise color positions
3. **Style Class Inheritance**: Create class hierarchies (`button_large` extends `button_primary`)
4. **Custom Breakpoints**: Define your own viewport thresholds
5. **Animations**: Combine transforms with Iced's animation system

### Production Build

When ready for production:

```bash
gravity build
cargo build --release
```

Styles are compiled to static Rust code - zero runtime parsing overhead!

---

## Troubleshooting

### Issue: Colors not appearing

**Solution**: Check color format - must be valid CSS color:
- ✅ `#3498db`, `rgb(52, 152, 219)`, `hsl(204, 70%, 53%)`
- ❌ `3498db` (missing #), `#ggg` (invalid hex)

### Issue: Gradient not rendering

**Solution**: Verify syntax:
- ✅ `linear-gradient(90deg, red, blue)`
- ❌ `linear-gradient(90, red, blue)` (missing unit on angle)

### Issue: Style class not found

**Solution**: Ensure class is defined in `<styles>` section before use:
```xml
<styles>
  <style name="my_class">
    <base background="#fff" />
  </style>
</styles>
<!-- Later: -->
<button class="my_class" label="OK" />
```

### Issue: Responsive attributes not working

**Solution**: Check window size against breakpoints:
- `mobile:` applies when width < 640px
- `tablet:` applies when 640px ≤ width < 1024px
- `desktop:` applies when width ≥ 1024px

---

## Best Practices

1. **Use Style Classes** for repeated patterns (buttons, cards, panels)
2. **Define Themes** for consistent color palettes and typography
3. **Mobile-First Responsive** - define mobile base, override for larger screens
4. **Minimize Inline Styles** - prefer classes for maintainability
5. **Test Responsiveness** - manually resize window during development
6. **Leverage Hot-Reload** - iterate quickly on styles without recompiling

---

## Summary

You've learned:
- ✅ Layout control with padding, spacing, alignment
- ✅ Sizing with fill, shrink, fixed, and percentages
- ✅ Inline styling with colors, borders, shadows, gradients
- ✅ Theme definitions for consistent design
- ✅ Reusable style classes with state variants
- ✅ Responsive layouts with breakpoint attributes
- ✅ State-based styling for interactive widgets

**Time Spent**: ~20 minutes  
**Skills Gained**: Complete styling system mastery  
**Next**: Build a production-ready styled application!

---

## Resources

- [XML Schema Reference](contracts/xml-schema.md) - Full attribute documentation
- [Data Model](data-model.md) - IR type definitions
- [Iced Styling Examples](https://github.com/iced-rs/iced/tree/master/examples/styling) - Inspiration
- Gravity Examples:
  - `examples/styling/` - Comprehensive styling showcase
  - `examples/responsive/` - Responsive layout patterns

**Happy Styling! 🎨**
