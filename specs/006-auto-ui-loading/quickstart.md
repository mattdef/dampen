# Quickstart: Auto-Loading UI Files

**Feature**: 006-auto-ui-loading
**Date**: 2026-01-06

## Overview

This guide shows how to create Gravity UI applications using the new auto-loading pattern. The `#[gravity_ui]` macro automatically loads `.gravity` XML files when compiling corresponding `.gravity.rs` files.

## Prerequisites

- Rust 1.75 or later
- Gravity crates installed (see [README](../../../../README.md))

## New Project Structure

```
my-gravity-app/
├── Cargo.toml
├── src/
│   └── main.rs
└── ui/
    ├── mod.rs
    ├── app.gravity.rs
    └── app.gravity
```

## Step 1: Create Project

```bash
cargo new my-gravity-app
cd my-gravity-app
```

## Step 2: Add Dependencies

Edit `Cargo.toml`:

```toml
[dependencies]
gravity-core = { path = "../gravity/crates/gravity-core" }
gravity-iced = { path = "../gravity/crates/gravity-iced" }
gravity-macros = { path = "../gravity/crates/gravity-macros" }
iced = "0.14"
```

## Step 3: Create UI Directory

```bash
mkdir ui
```

## Step 4: Define UI (app.gravity)

Create `ui/app.gravity`:

```xml
<?xml version="1.0" encoding="UTF-8"?>
<gravity>
    <column padding="40" spacing="20">
        <text value="Hello, Gravity!" size="32" weight="bold"/>
        <text value="This UI is auto-loaded!" size="16"/>
        <button label="Click me!" on_click="greet"/>
    </column>
</gravity>
```

## Step 5: Create Loader Module (app.gravity.rs)

Create `ui/app.gravity.rs`:

```rust
use gravity_macros::gravity_ui;

#[gravity_ui("ui/app.gravity")]
mod app_ui {}

use gravity_core::{AppState, HandlerRegistry, parse};

#[derive(gravity_macros::UiModel, Default)]
struct Model {
    message: String,
}

pub fn create_app_state() -> AppState<Model> {
    let document = app_ui::document;
    let handler_registry = create_handlers();
    AppState::with_handlers(document, handler_registry)
}

fn create_handlers() -> HandlerRegistry {
    let mut registry = HandlerRegistry::new();

    registry.register_simple("greet", |model: &mut dyn std::any::Any| {
        if let Some(m) = model.downcast_mut::<Model>() {
            m.message = "Button clicked!".to_string();
        }
    });

    registry
}
```

## Step 6: Export from Module (mod.rs)

Create `ui/mod.rs`:

```rust
pub mod app_gravity;
pub use app_gravity::{create_app_state, Model};
```

## Step 7: Create Main Entry Point

Edit `src/main.rs`:

```rust
use my_gravity_app_ui::{create_app_state, Model};
use gravity_iced::{GravityWidgetBuilder, HandlerMessage};
use iced::{Application, Element, Settings, Task};

#[derive(Debug, Clone)]
enum Message {
    Handler(String, Option<iced::Value>),
}

struct GravityApp {
    state: AppState<Model>,
}

impl Application for GravityApp {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = iced::Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Task<Self::Message>) {
        let state = create_app_state();
        (Self { state }, Task::none())
    }

    fn title(&self) -> String {
        "Gravity App".to_string()
    }

    fn update(&mut self, message: Self::Message) -> Task<Self::Message> {
        match message {
            Message::Handler(handler_name, value) => {
                if let Some(gravity_core::HandlerEntry::Simple(h)) =
                    self.state.handler_registry.get(&handler_name)
                {
                    h(&mut self.state.model);
                }
            }
        }
        Task::none()
    }

    fn view(&self) -> Element<Self::Message> {
        GravityWidgetBuilder::new(
            &self.state.document,
            &self.state.model,
            Some(&self.state.handler_registry),
        )
        .build()
    }
}

fn main() -> iced::Result {
    GravityApp::run(Settings::default())
}
```

## With Multiple Views

### Add Another UI (settings.gravity)

```
ui/
├── mod.rs
├── app.gravity.rs
├── app.gravity
├── settings.gravity.rs    # NEW
└── settings.gravity       # NEW
```

### Create settings.gravity:

```xml
<?xml version="1.0" encoding="UTF-8"?>
<gravity>
    <column padding="20">
        <text value="Settings" size="24" weight="bold"/>
        <text value="Configure your app here" size="14"/>
    </column>
</gravity>
```

### Create settings.gravity.rs:

```rust
use gravity_macros::gravity_ui;

#[gravity_ui("ui/settings.gravity")]
mod settings_ui {}

pub fn create_settings_state() -> AppState {
    AppState::new(settings_ui::document)
}
```

### Export from mod.rs:

```rust
pub mod app_gravity;
pub use app_gravity::{create_app_state as create_default_app_state, Model};

pub mod settings_gravity;  // NEW
pub use settings_gravity::create_app_state as create_settings_state;  // NEW
```

## Common Patterns

### Static UI (No Model, No Handlers)

```rust
use gravity_macros::gravity_ui;

#[gravity_ui("ui/static.gravity")]
mod static_ui;

pub fn create_static_state() -> AppState {
    AppState::new(static_ui::document)
}
```

### UI with Model Only (No Handlers)

```rust
#[derive(UiModel, Default)]
struct Counter {
    count: i32,
}

pub fn create_counter_state() -> AppState<Counter> {
    AppState::with_model(counter_ui::document, Counter::default())
}
```

### Custom File Path

```rust
#[gravity_ui(path = "views/dashboard.gravity")]
mod dashboard_ui;
```

## Troubleshooting

### "Gravity UI file not found"

Check that:
1. The path is relative to `CARGO_MANIFEST_DIR`
2. The file exists
3. The extension is `.gravity`

### "Handler not registered"

Ensure the handler is registered in `create_*_state()`:
```rust
registry.register_simple("handler_name", |model| {
    // Handler logic
});
```

### "Unknown widget 'X'"

The widget name must match the IR definitions. Check for typos.

## Next Steps

- [ ] Read the full [specification](../spec.md)
- [ ] Review [data model](../data-model.md)
- [ ] Check [API contracts](../contracts/xml-schema.md)
- [Explore examples](examples/hello-world)

## Migration from Old Pattern

Old pattern:
```rust
struct AppState {
    document: GravityDocument,
    model: Model,
    handler_registry: HandlerRegistry,
}

impl AppState {
    fn new() -> Self {
        let xml = include_str!("../ui/main.gravity");
        let document = parse(xml).expect("Failed to parse");
        // ...
    }
}
```

New pattern:
```rust
// Just use AppState<Model> directly
type AppState = gravity_core::AppState<Model>;

// Or create with custom handlers
let state = AppState::with_handlers(document, handlers);
```
