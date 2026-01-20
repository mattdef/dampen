# Codegen API Contract: State-Aware Code Generation

**Component**: `dampen-macros`  
**File**: `crates/dampen-macros/src/dampen_app.rs`

---

## Overview

The codegen system must generate Rust code that:
1. Applies inline state styles via Iced's style closure pattern
2. Handles breakpoint resolution at runtime
3. Produces code identical in behavior to interpreted mode

---

## Function: generate_widget_with_states (New/Modified)

### Purpose

Generates Rust code for widgets that have inline state variants.

### Input

`WidgetNode` with populated `inline_state_variants`:

```rust
WidgetNode {
    kind: WidgetKind::Button,
    attributes: { "label": Static("Click") },
    style: Some(StyleProperties { background: Some(Color(0, 0, 255)) }),
    inline_state_variants: {
        Hover: StyleProperties { background: Some(Color(255, 0, 0)) },
        Active: StyleProperties { background: Some(Color(0, 255, 0)) },
    },
}
```

### Generated Code Pattern

```rust
// Generated for <button label="Click" background="#0000ff" hover:background="#ff0000" active:background="#00ff00" />
{
    let __base_style = iced::widget::button::Style {
        background: Some(iced::Background::Color(iced::Color::from_rgb8(0x00, 0x00, 0xff))),
        text_color: iced::Color::WHITE,
        border: iced::Border::default(),
        shadow: iced::Shadow::default(),
    };
    
    iced::widget::button(iced::widget::text("Click"))
        .style(move |_theme: &iced::Theme, status: iced::widget::button::Status| {
            match status {
                iced::widget::button::Status::Hovered => iced::widget::button::Style {
                    background: Some(iced::Background::Color(
                        iced::Color::from_rgb8(0xff, 0x00, 0x00)
                    )),
                    ..__base_style
                },
                iced::widget::button::Status::Pressed => iced::widget::button::Style {
                    background: Some(iced::Background::Color(
                        iced::Color::from_rgb8(0x00, 0xff, 0x00)
                    )),
                    ..__base_style
                },
                iced::widget::button::Status::Disabled => __base_style,
                iced::widget::button::Status::Active => __base_style,
            }
        })
        .on_press(Message::ButtonClicked)
}
```

### Code Generation Rules

1. **Base Style**: Always generate a base style binding even if only state variants exist
2. **Match Arms**: Generate match arms for all Iced status variants
3. **State Mapping**: Map Dampen `WidgetState` to Iced `Status`:
   - `Hover` → `Status::Hovered`
   - `Active` → `Status::Pressed`
   - `Disabled` → `Status::Disabled`
   - `Focus` → Not directly mappable for Button (use default)
4. **Fallback**: States without inline variants use base style
5. **Struct Update**: Use `..base_style` syntax for fields not overridden

---

## Function: generate_breakpoint_resolution (New)

### Purpose

Generates runtime breakpoint resolution code for responsive attributes.

### Input

`WidgetNode` with populated `breakpoint_attributes`:

```rust
WidgetNode {
    kind: WidgetKind::Column,
    attributes: { "spacing": Static("20") },
    breakpoint_attributes: {
        Mobile: { "spacing": Static("10") },
        Tablet: { "spacing": Static("15") },
    },
}
```

### Generated Code Pattern

```rust
// Generated for <column spacing="20" mobile-spacing="10" tablet-spacing="15">
{
    let __spacing = match __viewport_width {
        w if w < 640.0 => 10.0,   // Mobile
        w if w < 1024.0 => 15.0,  // Tablet
        _ => 20.0,                 // Desktop (base)
    };
    
    iced::widget::column![
        // children...
    ]
    .spacing(__spacing)
}
```

### Viewport Width Access

The generated code assumes a `__viewport_width: f32` variable is in scope. This must be provided by the application's view function:

```rust
// In generated application code
fn view(&self) -> Element<Message> {
    let __viewport_width = self.viewport_width; // User must track this
    
    // Generated widget code uses __viewport_width
}
```

### Alternative: Static Resolution

If viewport tracking is not available, generate desktop-default code:

```rust
// Fallback when viewport_width not provided to codegen
let __spacing = 20.0;  // Desktop default
```

---

## Code Generation Templates

### Button with States

```rust
// Template variables:
// $LABEL: Button label expression
// $BASE_STYLE: Base style struct
// $HOVER_STYLE: Hover state style (if present)
// $ACTIVE_STYLE: Active state style (if present)
// $DISABLED_STYLE: Disabled state style (if present)
// $ON_PRESS: Message expression

iced::widget::button(iced::widget::text($LABEL))
    .style(move |_theme, status| {
        let base = $BASE_STYLE;
        match status {
            iced::widget::button::Status::Hovered => $HOVER_STYLE,
            iced::widget::button::Status::Pressed => $ACTIVE_STYLE,
            iced::widget::button::Status::Disabled => $DISABLED_STYLE,
            iced::widget::button::Status::Active => base,
        }
    })
    .on_press($ON_PRESS)
```

### TextInput with States

```rust
// TextInput has different status variants
iced::widget::text_input($PLACEHOLDER, $VALUE)
    .style(move |_theme, status| {
        let base = $BASE_STYLE;
        match status {
            iced::widget::text_input::Status::Hovered => $HOVER_STYLE,
            iced::widget::text_input::Status::Focused => $FOCUS_STYLE,
            iced::widget::text_input::Status::Disabled => $DISABLED_STYLE,
            iced::widget::text_input::Status::Active => base,
        }
    })
    .on_input($ON_INPUT)
```

---

## Error Handling

### Compile-Time Validation

Codegen should validate and report errors for:

| Condition | Error Message |
|-----------|---------------|
| Invalid state prefix in XML | `error: Unknown state 'unknown' in 'unknown:background' at line X` |
| Invalid style attribute | `error: Unknown style attribute 'foo' in 'hover:foo' at line X` |
| State on non-interactive widget | `warning: Widget 'text' does not support state styles; ignoring at line X` |

### Generated Error Handling

```rust
// For invalid color values, generate compile-time error
compile_error!("Invalid color value '#xyz' in hover:background at line 5");
```

---

## Testing Contract

### Snapshot Tests

Generated code must match expected snapshots:

```rust
#[test]
fn test_codegen_button_with_hover() {
    let xml = r#"<button hover:background="#ff0000" label="Test" />"#;
    let doc = parse(xml).unwrap();
    let generated = generate_application(&doc).unwrap();
    
    insta::assert_snapshot!(generated.code);
}
```

### Visual Parity Tests

Generated code must produce identical visual output to interpreted mode:

```rust
#[test]
fn test_visual_parity_button_hover() {
    let xml = r#"<button hover:background="#ff0000" label="Test" />"#;
    
    // Interpreted mode
    let interpreted_element = build_interpreted(xml);
    
    // Codegen mode
    let codegen_element = build_from_generated(xml);
    
    // Compare rendered output
    assert_visual_equal(interpreted_element, codegen_element);
}
```
