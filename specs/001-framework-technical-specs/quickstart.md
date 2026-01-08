# Quickstart: Gravity Declarative UI Framework

**Feature**: 001-framework-technical-specs  
**Date**: 2025-12-30

## Prerequisites

- Rust 1.75 or later (Edition 2021/2024)
- Cargo installed
- Basic familiarity with Iced concepts (Message, Command)

## Installation

Add Gravity to your `Cargo.toml`:

```toml
[dependencies]
gravity = "0.1"
gravity-iced = "0.1"
```

## Project Structure

```text
my-app/
├── Cargo.toml
├── src/
│   └── main.rs
└── ui/
    └── main.gravity    # Your UI definition
```

## Hello World (5 Minutes)

### Step 1: Create the UI File

Create `ui/main.gravity`:

```xml
<column padding="40" spacing="20" align="center">
    <text value="Hello, Gravity!" size="32" weight="bold" />
    <text value="Welcome to declarative Rust UI" />
    <button label="Click me!" on_click="greet" />
</column>
```

### Step 2: Define Your Model and Handlers

Create `src/main.rs`:

```rust
use gravity::prelude::*;
use gravity_iced::IcedBackend;

// Define your application state
#[derive(Default, UiModel)]
struct Model {
    greeting: String,
}

// Define your message type
#[derive(Clone)]
enum Message {
    Greet,
}

// Mark event handlers
#[ui_handler]
fn greet(model: &mut Model) {
    model.greeting = "Hello from Gravity!".to_string();
}

fn main() {
    // Run the application
    gravity::run::<Model, Message, IcedBackend>(
        "ui/main.gravity",
        Model::default(),
    );
}

### Step 3: Run

```bash
cargo run
```

## Counter Example (10 Minutes)

### UI File: `ui/counter.gravity`

```xml
<column padding="40" spacing="20" align="center">
    <text value="Counter: {count}" size="48" />
    
    <row spacing="20">
        <button label="-" on_click="decrement" enabled="{count > 0}" />
        <button label="+" on_click="increment" />
    </row>
    
    <button label="Reset" on_click="reset" />
</column>
```

### Model and Handlers: `src/main.rs`

```rust
use gravity::prelude::*;
use gravity_iced::IcedBackend;

#[derive(Default, UiModel, Serialize, Deserialize)]
struct Model {
    count: i32,
}

#[derive(Clone)]
enum Message {
    Increment,
    Decrement,
    Reset,
}

#[ui_handler]
fn increment(model: &mut Model) {
    model.count += 1;
}

#[ui_handler]
fn decrement(model: &mut Model) {
    model.count -= 1;
}

#[ui_handler]
fn reset(model: &mut Model) {
    model.count = 0;
}

fn main() {
    gravity::run::<Model, Message, IcedBackend>(
        "ui/counter.gravity",
        Model::default(),
    );
}
```

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

## Event Handlers

### Simple Handler

```rust
#[ui_handler]
fn do_something(model: &mut Model) {
    // Modify model state
}
```

### Handler with Value

For `on_input`, `on_change`, etc.:

```rust
#[ui_handler]
fn update_name(model: &mut Model, value: String) {
    model.name = value;
}
```

### Handler with Command (Async)

For side effects like API calls:

```rust
#[ui_handler]
fn fetch_data(model: &mut Model) -> Command<Message> {
    Command::perform(
        async { api::fetch_items().await },
        Message::ItemsLoaded,
    )
}
```

## Troubleshooting

### "Unknown handler: xyz"

Ensure your handler has the `#[ui_handler]` attribute and the function name matches the XML reference.

### "Field not found: xyz"

Check that:
1. The field exists on your Model struct
2. The field is not marked `#[ui_skip]`
3. Spelling matches exactly (case-sensitive)

### Build errors with generated code

Run `gravity check` first to validate your XML before building.
