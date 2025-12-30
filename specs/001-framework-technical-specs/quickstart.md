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

# For development mode with hot-reload
gravity-runtime = { version = "0.1", optional = true }

[features]
default = []
dev = ["gravity-runtime"]
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
    // Run in development mode with hot-reload
    gravity::run::<Model, Message, IcedBackend>(
        "ui/main.gravity",
        Model::default(),
    );
}
```

### Step 3: Run

```bash
# Development mode (with hot-reload)
cargo run --features dev

# Production build
cargo run --release
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

## Hot-Reload Development

1. Run with dev feature: `cargo run --features dev`
2. Edit `.gravity` files
3. Save — UI updates automatically (<500ms)
4. Model state is preserved across reloads

### Error Overlay

If your XML has errors, they appear as an overlay:

```text
┌─────────────────────────────────────────┐
│  Parse Error                            │
│                                         │
│  Line 5, Column 12:                     │
│  Unknown widget: <buton>                │
│                                         │
│  Did you mean: <button>?                │
│                                         │
│  [Dismiss]                              │
└─────────────────────────────────────────┘
```

## CLI Commands

```bash
# Start development server with hot-reload
gravity dev

# Generate production code
gravity build

# Validate XML without running
gravity check

# Inspect parsed IR
gravity inspect ui/main.gravity

# Inspect with generated code
gravity inspect --codegen ui/main.gravity
```

## Configuration

### gravity.toml

```toml
[project]
name = "my-app"
version = "0.1.0"

[source]
ui_dir = "ui"              # Directory containing .gravity files
main = "main.gravity"      # Entry point

[dev]
hot_reload = true
overlay = true             # Show error overlay

[build]
output = "src/generated"   # Where to write generated code
optimize = true            # Enable optimizations
```

### Cargo.toml Metadata (Alternative)

```toml
[package.metadata.gravity]
ui_dir = "ui"
main = "main.gravity"
```

## Common Patterns

### Form Input

```xml
<column spacing="10">
    <text value="Email:" />
    <text_input 
        value="{email}"
        on_input="update_email"
        placeholder="you@example.com"
    />
    
    <text value="Password:" />
    <text_input 
        value="{password}"
        on_input="update_password"
        password="true"
    />
    
    <button 
        label="Sign In" 
        on_click="sign_in"
        enabled="{email.len() > 0 && password.len() > 0}"
    />
</column>
```

### Conditional Display

```xml
<column>
    <text value="{if is_loading then 'Loading...' else ''}" />
    
    <!-- Future: conditional rendering -->
    <!-- <if condition="{is_error}">
        <text value="{error_message}" color="red" />
    </if> -->
</column>
```

### Layout with Spacing

```xml
<row spacing="10">
    <button label="Cancel" on_click="cancel" />
    <space width="fill" />
    <button label="Save" on_click="save" style="primary" />
</row>
```

## Skipping Fields from Binding

```rust
#[derive(UiModel)]
struct Model {
    // Exposed to bindings
    username: String,
    
    // Hidden from bindings
    #[ui_skip]
    internal_cache: HashMap<String, Data>,
}
```

## Next Steps

1. Read the [XML Schema Reference](./contracts/xml-schema.md) for all widgets
2. Check the [Data Model](./data-model.md) for IR structure
3. See `examples/` for more complex applications
4. Read the [Implementation Plan](./plan.md) for project roadmap

## Troubleshooting

### "Unknown handler: xyz"

Ensure your handler has the `#[ui_handler]` attribute and the function name matches the XML reference.

### "Field not found: xyz"

Check that:
1. The field exists on your Model struct
2. The field is not marked `#[ui_skip]`
3. Spelling matches exactly (case-sensitive)

### Hot-reload not working

1. Verify `--features dev` is enabled
2. Check that file watcher has permissions
3. Ensure `.gravity` file is in the watched directory

### Build errors with generated code

Run `gravity check` first to validate your XML before building.
