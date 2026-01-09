# Dampen Quick Start Guide

**Version**: 1.0.0  
**Last Updated**: 2026-01-09

This guide will walk you through building your first Dampen application in 10 minutes.

---

## Table of Contents

1. [Introduction](#introduction)
2. [Prerequisites](#prerequisites)
3. [Installation](#installation)
4. [Your First App: Counter](#your-first-app-counter)
5. [Project Structure](#project-structure)
6. [Next Steps](#next-steps)

---

## Introduction

Dampen is a declarative UI framework for Rust that lets you define user interfaces in XML and render them using Iced. Key features:

- **XML-based UI definitions** - Clean, readable markup
- **Type-safe bindings** - Access model fields directly from XML
- **Event handlers** - Connect UI events to Rust functions
- **Theming & styling** - Themes, style classes, and state-based styles
- **Responsive design** - Breakpoint-prefixed attributes

---

## Prerequisites

- Rust 1.75 or later
- Cargo installed

---

## Installation

### Option 1: Using `dampen new` (Recommended)

```bash
# Install the CLI
cargo install dampen-cli

# Create a new project
dampen new my-counter-app
cd my-counter-app
```

### Option 2: Manual Setup

```bash
cargo new my-counter-app
cd my-counter-app
```

Edit `Cargo.toml`:

```toml
[package]
name = "my-counter-app"
version = "0.1.0"
edition = "2021"

[dependencies]
dampen-core = "0.1"
dampen-macros = "0.1"
dampen-iced = "0.1"
iced = "0.14"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

---

## Your First App: Counter

This example creates a simple counter with increment, decrement, and reset buttons.

### Step 1: Create the UI File

Create `src/ui/window.dampen`:

```xml
<?xml version="1.0" encoding="UTF-8"?>
<dampen>
    <column padding="40" spacing="20" align="center">
        <text value="Counter: {count}" size="48" weight="bold" />

        <row spacing="20">
            <button label="-" on_click="decrement" enabled="{count > 0}" />
            <button label="+" on_click="increment" />
        </row>

        <button label="Reset" on_click="reset" />
    </column>
</dampen>
```

### Step 2: Create the Model and Handlers

Create `src/ui/window.rs`:

```rust
use dampen_core::{AppState, HandlerRegistry};
use dampen_macros::{dampen_ui, UiModel};
use serde::{Deserialize, Serialize};

#[derive(Default, UiModel, Serialize, Deserialize, Clone, Debug)]
pub struct Model {
    pub count: i32,
}

#[dampen_ui("window.dampen")]
mod _app {}

pub fn create_app_state() -> AppState<Model> {
    let document = _app::document();
    let handler_registry = create_handler_registry();
    AppState::with_handlers(document, handler_registry)
}

pub fn create_handler_registry() -> HandlerRegistry {
    let registry = HandlerRegistry::new();

    registry.register_simple("increment", |model: &mut dyn std::any::Any| {
        if let Some(m) = model.downcast_mut::<Model>() {
            m.count += 1;
        }
    });

    registry.register_simple("decrement", |model: &mut dyn std::any::Any| {
        if let Some(m) = model.downcast_mut::<Model>() {
            if m.count > 0 {
                m.count -= 1;
            }
        }
    });

    registry.register_simple("reset", |model: &mut dyn std::any::Any| {
        if let Some(m) = model.downcast_mut::<Model>() {
            m.count = 0;
        }
    });

    registry
}
```

### Step 3: Create the Main Entry Point

Edit `src/main.rs`:

```rust
mod ui;

use dampen_core::AppState;
use dampen_iced::{DampenWidgetBuilder, HandlerMessage};
use iced::{Element, Task};

struct CounterApp {
    window_state: AppState<ui::window::Model>,
}

fn dispatch_handler(app: &mut CounterApp, handler_name: &str, value: Option<String>) {
    let (model, registry) = (
        &mut app.window_state.model as &mut dyn std::any::Any,
        &app.window_state.handler_registry,
    );
    registry.dispatch(handler_name, model, value);
}

fn update(app: &mut CounterApp, message: HandlerMessage) -> Task<HandlerMessage> {
    match message {
        HandlerMessage::Handler(handler_name, value) => {
            dispatch_handler(app, &handler_name, value);
        }
    }
    Task::none()
}

fn view(app: &CounterApp) -> Element<'_, HandlerMessage> {
    DampenWidgetBuilder::from_app_state(&app.window_state).build()
}

fn init() -> (CounterApp, Task<HandlerMessage>) {
    (
        CounterApp {
            window_state: ui::window::create_app_state(),
        },
        Task::none(),
    )
}

pub fn main() -> iced::Result {
    iced::application(init, update, view).run()
}
```

### Step 4: Create the UI Module

Create `src/ui/mod.rs`:

```rust
pub mod window;
```

### Step 5: Run Your App

```bash
cargo run
```

You should see a window with a counter that responds to button clicks!

---

## Binding Expressions

### Simple Binding

```xml
<text value="{username}" />
```

Displays the value of `model.username`.

### Formatted Binding

```xml
<text value="Hello, {name}! You have {messages.len()} messages." />
```

Interpolates multiple values.

### Conditional Binding

```xml
<button enabled="{items.len() > 0}" />
<text value="{if is_loading then 'Loading...' else 'Ready'}" />
```

### Nested Field Access

```xml
<text value="{user.profile.display_name}" />
```

---

## Event Handlers

### Simple Handler

```rust
registry.register_simple("increment", |model: &mut dyn std::any::Any| {
    if let Some(m) = model.downcast_mut::<Model>() {
        m.count += 1;
    }
});
```

### Handler with Value

```rust
registry.register_with_value("update_name", |model: &mut dyn std::any::Any, value: String| {
    if let Some(m) = model.downcast_mut::<Model>() {
        m.name = value;
    }
});
```

### Handler with Command (Async)

```rust
use iced::Task;

registry.register_with_command("fetch_data", |model: &mut dyn std::any::Any| {
    if let Some(m) = model.downcast_mut::<Model>() {
        // Return a Task for async operations
        Task::perform(
            async { /* fetch data */ },
            |result| HandlerMessage::Custom("data_loaded".to_string(), Some(result)),
        )
    } else {
        Task::none()
    }
});
```

---

## Project Structure

### Small Projects

```
my-app/
├── Cargo.toml
├── src/
│   ├── main.rs
│   └── ui/
│       ├── mod.rs
│       ├── window.rs
│       └── window.dampen
```

### Medium Projects

```
my-app/
├── Cargo.toml
├── src/
│   ├── main.rs
│   └── ui/
│       ├── mod.rs
│       ├── window.rs
│       ├── window.dampen
│       └── components/
│           ├── header.rs
│           ├── header.dampen
│           ├── footer.rs
│           └── footer.dampen
```

### Large Projects

```
my-app/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── model/
│   │   ├── mod.rs
│   │   ├── user.rs
│   │   └── todo.rs
│   ├── handlers/
│   │   ├── mod.rs
│   │   ├── user.rs
│   │   └── todo.rs
│   └── ui/
│       ├── mod.rs
│       ├── window.rs
│       └── window.dampen
```

---

## CLI Commands

### Validate XML

```bash
dampen check
```

### Inspect IR

```bash
# View IR tree
dampen inspect --file src/ui/window.dampen

# View generated code
dampen inspect --file src/ui/window.dampen --codegen

# JSON output for tooling
dampen inspect --file src/ui/window.dampen --format json
```

### Build for Production

```bash
dampen build --ui ui --output src/ui_generated.rs
```

---

## Troubleshooting

### "Unknown handler: xyz"

Ensure your handler is registered in the HandlerRegistry with the matching name.

### "Field not found: xyz"

Check:
- Field exists on Model
- Not marked `#[ui_skip]`
- Spelling matches exactly

### Build errors

Run `dampen check` first to validate your XML.

---

## Next Steps

1. **Read the XML Schema Reference** - See `docs/XML_SCHEMA.md`
2. **Learn about Styling** - See `docs/STYLING.md`
3. **Explore Examples** - See `examples/` directory
4. **API Documentation** - https://docs.rs/dampen-core

---

**Ready to build something amazing? Let's go! 🚀**
