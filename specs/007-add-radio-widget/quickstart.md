# Radio Widget Quickstart

**Feature**: 007-add-radio-widget
**Date**: 2026-01-08

## Overview

The Radio widget allows users to select a single option from a group of mutually exclusive choices. This guide covers adding radio buttons to your Gravity UI.

## Basic Usage

### Step 1: Define Your Model

```rust
use gravity_macros::UiModel;

#[derive(UiModel)]
pub struct OrderForm {
    pub size: Option<String>,
    pub shipping: String,
}
```

### Step 2: Create XML Layout

```xml
<!-- src/ui/order.gravity -->
<container padding="20">
  <column spacing="15">
    <text value="Select your size:"/>
    
    <radio label="Small (8oz)" 
           value="small" 
           selected={size} 
           on_select="setSize"/>
           
    <radio label="Medium (12oz)" 
           value="medium" 
           selected={size} 
           on_select="setSize"/>
           
    <radio label="Large (16oz)" 
           value="large" 
           selected={size} 
           on_select="setSize"/>
  </column>
</container>
```

### Step 3: Handle Selection Events

```rust
// src/handlers.rs
use gravity_core::HandlerRegistry;

pub fn create_handler_registry() -> HandlerRegistry {
    let mut registry = HandlerRegistry::new();
    
    // Simple handler - updates size field
    registry.simple("setSize", |state: &mut OrderForm, value: String| {
        state.size = Some(value);
    });
    
    registry
}
```

## Common Patterns

### Radio with Default Selection

```xml
<column spacing="10">
  <text value="Choose your preference:"/>
  <radio label="Option A" value="a" selected={selection} on_select="setSelection"/>
  <radio label="Option B" value="b" selected={selection} on_select="setSelection"/>
  <radio label="Option C" value="c" selected={selection} on_select="setSelection"/>
</column>
```

### Conditionally Disabled Options

```xml
<column spacing="10">
  <text value="Select shipping:"/>
  <radio label="Standard Shipping" 
         value="standard" 
         selected={shipping} 
         on_select="setShipping"/>
  <radio label="Express Shipping (+$10)" 
         value="express" 
         selected={shipping} 
         on_select="setShipping"
         disabled={!is_premium_member}/>
  <radio label="Priority Overnight" 
         value="overnight" 
         selected={shipping} 
         on_select="setShipping"
         disabled={!is_premium_member}/>
</column>
```

### Radio in a Row Layout

```xml
<row spacing="20">
  <text value="Color:"/>
  <radio label="Red" value="red" selected={color} on_select="setColor"/>
  <radio label="Green" value="green" selected={color} on_select="setColor"/>
  <radio label="Blue" value="blue" selected={color} on_select="setColor"/>
</row>
```

## Styling

Radio widgets inherit theme styling. To customize:

```css
/* theme.css */
.radio-option {
  spacing: 8px;
}

.radio-option:disabled {
  opacity: 0.5;
}
```

## Complete Example

```rust
// src/main.rs
use gravity_iced::Sandbox;
use gravity_macros::{gravity_ui, UiModel};

#[derive(UiModel)]
pub struct AppModel {
    selected_size: Option<String>,
}

#[gravity_ui("app.gravity")]
mod _app {}

enum AppMessage {
    SetSize(String),
}

struct App;

impl Sandbox for App {
    type Message = AppMessage;
    
    fn new() -> (Self, gravity_core::AppState<AppModel>) {
        let state = gravity_core::AppState::with_model(_app::document(), AppModel {
            selected_size: Some("medium".to_string()),
        });
        (App, state)
    }
    
    fn title(&self) -> String {
        "Radio Demo".to_string()
    }
    
    fn update(&mut self, message: Self::Message) {
        match message {
            AppMessage::SetSize(size) => {
                self.state.model.selected_size = Some(size);
            }
        }
    }
    
    fn view(&self) -> iced::Element<'_, Self::Message> {
        _app::view(&self.state)
    }
}
```

## Best Practices

1. **Use clear labels**: Make radio labels descriptive
2. **Limit options**: 2-5 options work best; use dropdown for more
3. **Default selection**: Pre-select the most common option when appropriate
4. **Logical grouping**: Use containers to group related radio buttons
5. **Vertical layout**: Stack radios vertically for better readability

## API Reference

### XML Attributes

| Attribute | Type | Required | Description |
|-----------|------|----------|-------------|
| `label` | String | Yes | Radio button label |
| `value` | String | Yes | Selection value |
| `selected` | Binding | No | Currently selected value |
| `on_select` | Handler | No | Selection handler |
| `disabled` | Binding | No | Disable interaction |

### Handler Signature

```rust
fn handler(state: &mut Model, value: String)
```

The handler receives the `value` string of the selected radio option.

## Troubleshooting

| Issue | Solution |
|-------|----------|
| Radio not responding to clicks | Check `on_select` handler is registered |
| Wrong radio appears selected | Verify `selected` binding matches radio `value` |
| All radios can be selected | Ensure radios share the same `selected` binding |
| Disabled radio still clickable | Check `disabled` binding evaluates correctly |
