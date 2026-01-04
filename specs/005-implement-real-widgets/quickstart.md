# Quickstart: Real Iced Widgets

**Feature Branch**: `005-implement-real-widgets`  
**Date**: 2026-01-04

## Overview

This guide shows how to use the six interactive widgets implemented in this feature: `text_input`, `checkbox`, `toggler`, `pick_list`, `slider`, and `image`.

## Prerequisites

- Gravity framework installed
- Rust 1.75+
- An existing Gravity project with `gravity-iced` dependency

## Basic Usage

### 1. Text Input

Capture user text input with real-time updates:

```xml
<!-- ui/form.gravity -->
<column spacing="10">
    <text value="Enter your name:" />
    <text_input 
        placeholder="Type here..."
        value="{name}"
        on_input="update_name"
    />
    <text value="Hello, {name}!" />
</column>
```

```rust
// src/main.rs
#[derive(UiModel, Serialize, Deserialize, Clone)]
struct Model {
    name: String,
}

#[ui_handler]
fn update_name(model: &mut Model, value: String) {
    model.name = value;
}
```

### 2. Checkbox

Toggle boolean values with a labeled checkbox:

```xml
<column spacing="10">
    <checkbox 
        label="Accept Terms and Conditions"
        checked="{accepted}"
        on_toggle="toggle_accepted"
    />
    <button 
        label="Continue"
        on_click="continue"
        enabled="{accepted}"
    />
</column>
```

```rust
#[derive(UiModel, Serialize, Deserialize, Clone)]
struct Model {
    accepted: bool,
}

#[ui_handler]
fn toggle_accepted(model: &mut Model, value: String) {
    model.accepted = value == "true";
}
```

### 3. Toggler

Modern switch-style toggle:

```xml
<row spacing="15">
    <text value="Dark Mode" />
    <toggler 
        active="{dark_mode}"
        on_toggle="toggle_dark_mode"
    />
</row>
```

```rust
#[ui_handler]
fn toggle_dark_mode(model: &mut Model, value: String) {
    model.dark_mode = value == "true";
}
```

### 4. Pick List

Dropdown selection from options:

```xml
<column spacing="10">
    <text value="Select Priority:" />
    <pick_list 
        options="Low,Medium,High,Critical"
        selected="{priority}"
        placeholder="Choose priority..."
        on_select="set_priority"
    />
</column>
```

```rust
#[derive(UiModel, Serialize, Deserialize, Clone)]
struct Model {
    priority: String,
}

#[ui_handler]
fn set_priority(model: &mut Model, value: String) {
    model.priority = value;
}
```

### 5. Slider

Numeric value selection:

```xml
<column spacing="10">
    <text value="Volume: {volume}%" />
    <slider 
        min="0"
        max="100"
        value="{volume}"
        on_change="set_volume"
    />
</column>
```

```rust
#[derive(UiModel, Serialize, Deserialize, Clone)]
struct Model {
    volume: f32,
}

#[ui_handler]
fn set_volume(model: &mut Model, value: String) {
    if let Ok(v) = value.parse::<f32>() {
        model.volume = v;
    }
}
```

### 6. Image

Display images from files:

```xml
<column spacing="10" align="center">
    <image src="assets/logo.png" width="200" height="100" />
    <text value="Welcome to Our App" />
</column>
```

No handler needed - images are display-only.

## Complete Example

Here's a complete settings form using multiple widgets:

```xml
<!-- ui/settings.gravity -->
<container padding="20">
    <column spacing="20">
        <text value="Settings" size="24" weight="bold" />
        
        <!-- User Profile -->
        <column spacing="10">
            <text value="Profile" size="18" weight="bold" />
            <text_input 
                placeholder="Your name"
                value="{user_name}"
                on_input="update_name"
            />
            <image src="{avatar_path}" width="64" height="64" />
        </column>
        
        <!-- Preferences -->
        <column spacing="10">
            <text value="Preferences" size="18" weight="bold" />
            
            <toggler 
                label="Dark Mode"
                active="{dark_mode}"
                on_toggle="toggle_dark_mode"
            />
            
            <toggler 
                label="Notifications"
                active="{notifications}"
                on_toggle="toggle_notifications"
            />
        </column>
        
        <!-- Audio Settings -->
        <column spacing="10">
            <text value="Audio" size="18" weight="bold" />
            
            <row spacing="10">
                <text value="Volume:" />
                <slider 
                    min="0" max="100"
                    value="{volume}"
                    on_change="set_volume"
                />
                <text value="{volume}%" />
            </row>
        </column>
        
        <!-- Theme Selection -->
        <column spacing="10">
            <text value="Theme" size="18" weight="bold" />
            <pick_list 
                options="Light,Dark,System,Custom"
                selected="{theme}"
                on_select="set_theme"
            />
        </column>
        
        <!-- Agreement -->
        <checkbox 
            label="I agree to the terms of service"
            checked="{agreed}"
            on_toggle="toggle_agreed"
        />
        
        <button 
            label="Save Settings"
            on_click="save_settings"
        />
    </column>
</container>
```

## Handler Registration

Register handlers with the `HandlerRegistry`:

```rust
let registry = HandlerRegistry::new();

// Value handlers (receive String payload)
registry.register_with_value("update_name", |model: &mut dyn Any, value: Box<dyn Any>| {
    let model = model.downcast_mut::<Model>().unwrap();
    if let Ok(text) = value.downcast::<String>() {
        model.user_name = *text;
    }
});

// Simple handlers (no payload)
registry.register_simple("save_settings", |model: &mut dyn Any| {
    let model = model.downcast_mut::<Model>().unwrap();
    save_to_file(model);
});
```

## Tips

1. **Boolean values**: Handlers receive `"true"` or `"false"` strings, parse with `value == "true"`

2. **Numeric values**: Handlers receive string representations, parse with `.parse::<f32>()`

3. **Default values**: Initialize model fields with sensible defaults - widgets use these on first render

4. **Validation**: Validate values in handlers before updating model

5. **Verbose mode**: Enable with `.with_verbose(true)` on the builder to debug binding issues
