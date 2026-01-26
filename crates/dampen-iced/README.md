# Dampen Iced

Iced backend implementation for the Dampen declarative UI framework.

## Overview

Dampen Iced provides automatic interpretation of parsed Dampen markup into Iced widgets, eliminating manual conversion boilerplate. It handles bindings, events, styles, and layouts automatically through the `DampenWidgetBuilder`.

## Features

- ✅ **Automatic Widget Building**: Convert XML markup to Iced widgets with one line
- ✅ **Binding Support**: Evaluate expressions like `{count}` or `{user.name}`
- ✅ **Event Handling**: Connect handlers via `on_click`, `on_input`, etc.
- ✅ **Style & Layout**: Apply padding, spacing, colors, borders automatically
- ✅ **Recursive Processing**: Handles nested widget trees automatically
- ✅ **State-Aware Styling**: Hover, focus, active, and disabled states for interactive widgets
- ✅ **Compile-Time Logging**: Debug output only in debug builds (no runtime overhead)

## Quick Start

### Basic Usage

```rust
use dampen_core::{parse, HandlerRegistry};
use dampen_iced::{DampenWidgetBuilder, HandlerMessage};
use dampen_macros::UiModel;
use iced::{Element, Task};
use serde::{Deserialize, Serialize};
use std::any::Any;

#[derive(Default, UiModel, Serialize, Deserialize, Clone, Debug)]
struct Model {
    count: i32,
}

type Message = HandlerMessage;

fn increment(model: &mut Model) {
    model.count += 1;
}

fn decrement(model: &mut Model) {
    model.count -= 1;
}

struct AppState {
    model: Model,
    document: dampen_core::DampenDocument,
    handler_registry: HandlerRegistry,
}

impl AppState {
    fn new() -> Self {
        let xml = include_str!("../ui/main.dampen");
        let document = parse(xml).expect("Failed to parse XML");
        
        let handler_registry = HandlerRegistry::new();
        handler_registry.register_simple("increment", |m: &mut dyn Any| {
            let model = m.downcast_mut::<Model>().unwrap();
            increment(model);
        });
        handler_registry.register_simple("decrement", |m: &mut dyn Any| {
            let model = m.downcast_mut::<Model>().unwrap();
            decrement(model);
        });
        
        Self {
            model: Model::default(),
            document,
            handler_registry,
        }
    }
}

fn view(state: &AppState) -> Element<'_, Message> {
    // Single line to build the entire UI!
    DampenWidgetBuilder::new(
        &state.document.root,
        &state.model,
        Some(&state.handler_registry),
    )
    .build()
}
```

### UI File (ui/main.dampen)

```xml
<column padding="40" spacing="20">
    <text value="Count: {count}" size="24" />
    <row spacing="10">
        <button label="Increment" on_click="increment" />
        <button label="Decrement" on_click="decrement" />
    </row>
</column>
```

## API Reference

### DampenWidgetBuilder

#### Constructor

```rust
// Using HandlerMessage (recommended for most cases)
DampenWidgetBuilder::new(
    node: &WidgetNode,
    model: &dyn UiBindable,
    handler_registry: Option<&HandlerRegistry>,
) -> Self

// From complete document
DampenWidgetBuilder::from_document(
    document: &DampenDocument,
    model: &dyn UiBindable,
    handler_registry: Option<&HandlerRegistry>,
) -> Self

// With custom message factory
DampenWidgetBuilder::new_with_factory<F>(
    node: &WidgetNode,
    model: &dyn UiBindable,
    handler_registry: Option<&HandlerRegistry>,
    message_factory: F,
) -> Self
where F: Fn(&str) -> Message + 'a
```

#### Configuration

```rust
// Add style classes for theme support
builder.with_style_classes(&style_classes_map)

// Access state manager for event handlers
let manager = builder.state_manager();
```

#### Execution

```rust
// Build the widget tree
let element: Element<'_, Message> = builder.build();
```

### Supported Widgets

| Widget | XML Tag | Key Attributes |
|--------|---------|----------------|
| Text | `<text />` | `value`, `size`, `weight`, `color` |
| Button | `<button />` | `label`, `on_click`, `enabled` |
| Column | `<column />` | `spacing`, `padding` |
| Row | `<row />` | `spacing`, `padding` |
| Container | `<container />` | `width`, `height`, `padding` |
| TextInput | `<text_input />` | `value`, `placeholder`, `on_input` |
| Checkbox | `<checkbox />` | `label`, `checked`, `on_toggle` |
| Slider | `<slider />` | `min`, `max`, `value`, `on_change` |
| PickList | `<pick_list />` | `options`, `selected`, `on_select` |
| Toggler | `<toggler />` | `label`, `active`, `on_toggle` |
| Image | `<image />` | `src` |
| Scrollable | `<scrollable />` | (wraps children) |
| Stack | `<stack />` | (layered children) |
| Space | `<space />` | (flexible spacer) |
| Rule | `<rule />` | (horizontal/vertical line) |

### Binding Expressions

```xml
<!-- Simple field access -->
<text value="{count}" />

<!-- Nested fields -->
<text value="{user.profile.name}" />

<!-- Method calls -->
<text value="{items.len()}" />

<!-- Binary operations -->
<text value="{count * 2}" />

<!-- Conditionals -->
<text value="{if count > 10 then 'High' else 'Low'}" />

<!-- Interpolated strings -->
<text value="Count: {count}, User: {username}" />
```

### Event Handlers

```rust
// Simple handler (no payload)
fn increment(model: &mut Model) {
    model.count += 1;
}

// Handler with value
fn update_text(model: &mut Model, value: String) {
    model.text = value;
}

// Handler with command (async)
fn fetch_data(model: &mut Model) -> Task<Message> {
    Task::perform(
        async { api::fetch().await },
        Message::DataLoaded,
    )
}
```

## Examples

See the `examples/` directory for complete working applications:

- **hello-world**: Minimal example showing basic usage
- **counter**: Interactive counter with event handlers
- **todo-app**: Full CRUD application with bindings
- **styling**: Theme and style class examples

## Architecture

### Design Principles

1. **Centralized Interpretation**: All widget rendering logic lives in the builder
2. **Type Safety**: No runtime type erasure, compile-time verified
3. **Backend Agnostic**: Core logic separate from Iced-specific widgets
4. **Zero Duplication**: Reuses existing style mapping and evaluation functions

### File Structure

```
crates/dampen-iced/
├── src/
│   ├── lib.rs           # Public API, IcedBackend trait
│   ├── builder.rs       # DampenWidgetBuilder implementation
│   ├── convert.rs       # IR → Iced type conversions
│   ├── state.rs         # Widget state management
│   ├── style_mapping.rs # Style and layout mapping
│   ├── theme_adapter.rs # Theme conversion
│   └── widgets/         # Widget-specific helpers
└── tests/               # Integration tests
```

## Performance

### Benchmarks

- **100 widgets**: ~0.027ms (37x faster than 1ms target)
- **1000 widgets**: ~0.284ms (175x faster than 50ms target)
- **Binding evaluation**: ~713ns per widget
- **Event connection**: ~784ns per widget

### Optimization Tips

1. **Reuse builders**: Builder instances are single-use, but cheap to create
2. **Minimize bindings**: Each binding adds ~700ns overhead
3. **Use style classes**: Reuse styles instead of inline attributes
4. **Debug in debug builds**: Verbose logging is automatically stripped from release builds

## Error Handling

### Debug Mode

Debug logging is automatically enabled in debug builds:

```rust
// Debug builds: see detailed logging
// Release builds: no logging overhead
let widget = DampenWidgetBuilder::new(&node, &model, Some(®istry))
    .build();
```

### Common Errors

1. **Missing handler**: Event is silently ignored (debug build logs warning)
2. **Binding error**: Returns empty string (debug build logs error)
3. **Invalid attribute**: Uses default value (debug build logs warning)
4. **Unsupported widget**: Returns empty placeholder (debug build logs error)

## Integration

### With CLI

```bash
# Development mode
dampen dev --ui ui --file main.dampen --verbose

# Validate XML
dampen check --ui ui

# Inspect IR
dampen inspect --file ui/main.dampen
```

## Best Practices

### 1. Model Design

```rust
#[derive(UiModel, Serialize, Deserialize, Clone)]
struct Model {
    // Simple fields work best
    count: i32,
    name: String,
    
    // Vec for lists
    items: Vec<String>,
    
    // Option for optional state
    selected: Option<usize>,
}
```

### 2. Handler Registration

```rust
// Register all handlers at once
let registry = HandlerRegistry::new();
registry.register_simple("increment", |m| /* ... */);
registry.register_simple("decrement", |m| /* ... */);
registry.register_with_value("update", |m, v| /* ... */);
```

### 3. XML Structure

```xml
<!-- Good: Clear hierarchy -->
<column spacing="20">
    <text value="Title" size="24" />
    <row spacing="10">
        <button label="OK" on_click="ok" />
        <button label="Cancel" on_click="cancel" />
    </row>
</column>

<!-- Avoid: Deep nesting without layout -->
<container>
    <container>
        <container>
            <text value="Too deep!" />
        </container>
    </container>
</container>
```

## Troubleshooting

### Builder doesn't update on changes

- Ensure you're using hot-reload mode
- Check file permissions
- Verify XML is valid

### Bindings not evaluating

- Check field names match exactly (case-sensitive)
- Verify model implements `UiBindable`
- Debug builds show evaluation errors automatically

### Events not firing

- Verify handler is registered in `HandlerRegistry`
- Check handler name matches XML attribute
- Ensure handler is registered in HandlerRegistry

## Contributing

See the [main repository CONTRIBUTING.md](https://github.com/your-org/dampen/blob/main/CONTRIBUTING.md) for guidelines.

## License

This project is licensed under either of:

- Apache License, Version 2.0
- MIT license

at your option.

## Status

**Current Version**: 0.2.7  
**Phase**: Phase 9 - Complete (All refactoring complete)  
**All Tests**: ✅ Passing  
**Examples**: ✅ Working (hello-world, counter, todo-app, styling)

## v0.2.7 Changelog

### Added
- Helper functions for boolean attribute parsing (`resolve_boolean_attribute`)
- Handler resolution helper with rich error context (`resolve_handler_param`)
- Generic state-aware styling helper (`create_state_aware_style_fn`)
- State-aware styling support for slider widget

### Changed
- Deprecated `IcedBackend` (removal in v0.3.0)
- Verbose logging now compile-time gated (no runtime overhead)
- Improved error messages with suggestions

### Performance
- Reduced code duplication by ~370+ lines
- Rc-wrapped StyleClass for efficient cloning in closures
- No verbose logging overhead in release builds
