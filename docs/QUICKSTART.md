# Gravity Quick Start Guide

This guide will walk you through building your first Gravity application in 10 minutes.

## Prerequisites

- Rust 1.75 or later
- Cargo installed

## Installation

### Step 1: Create a New Project

```bash
cargo new my-gravity-app
cd my-gravity-app
```

### Step 2: Add Dependencies

Edit `Cargo.toml`:

```toml
[package]
name = "my-gravity-app"
version = "0.1.0"
edition = "2021"

[dependencies]
gravity = "0.1"
gravity-iced = "0.1"

# For development mode with hot-reload
gravity-runtime = { version = "0.1", optional = true }

[features]
default = []
dev = ["gravity-runtime"]
```

### Step 3: Create UI Directory

```bash
mkdir ui
```

## Your First App: Counter

### Step 1: Create the UI File

Create `ui/main.gravity`:

```xml
<?xml version="1.0" encoding="UTF-8"?>
<column padding="40" spacing="20" align="center">
    <text value="Counter: {count}" size="48" weight="bold" />
    
    <row spacing="20">
        <button label="-" on_click="decrement" enabled="{count > 0}" />
        <button label="+" on_click="increment" />
    </row>
    
    <button label="Reset" on_click="reset" />
</column>
```

### Step 2: Create the Model and Handlers

Edit `src/main.rs`:

```rust
use gravity::prelude::*;
use gravity_iced::IcedBackend;
use serde::{Serialize, Deserialize};

// Define your application state
#[derive(Default, UiModel, Serialize, Deserialize)]
struct Model {
    count: i32,
}

// Define your message type
#[derive(Clone, Debug)]
enum Message {
    Increment,
    Decrement,
    Reset,
}

// Define event handlers
fn increment(model: &mut Model) {
    model.count += 1;
}

fn decrement(model: &mut Model) {
    model.count -= 1;
}

fn reset(model: &mut Model) {
    model.count = 0;
}

fn main() {
    // Run in development mode with hot-reload
    gravity::run::<Model, Message, IcedBackend>(
        "ui/main.gravity",
        Model::default(),
    );
}
```

### Step 3: Run Your App

```bash
# Development mode (with hot-reload)
cargo run --features dev

# Production build
cargo run --release
```

You should see a window with a counter that responds to button clicks!

## Development Workflow

### Hot-Reload Mode

When running with `--features dev`:

1. **Edit** `ui/main.gravity`
2. **Save** the file
3. **See** changes instantly (<500ms)
4. **State** is preserved across reloads

Try it:
- Change the text size from `48` to `64`
- Add a new button
- Save and watch the UI update!

### Error Handling

If you make a mistake in your XML, an error overlay appears:

```xml
<!-- Try this broken XML -->
<column>
    <buton label="Broken" />  <!-- Typo: buton instead of button -->
</column>
```

The overlay will show:
```
Parse Error
Line 2, Column 5:
Unknown widget: <buton>

Did you mean: <button>?

[Dismiss]
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
fn do_something(model: &mut Model) {
    model.some_field = new_value;
}
```

### Handler with Value

For `on_input`, `on_change`, etc.:

```rust
fn update_name(model: &mut Model, value: String) {
    model.name = value;
}
```

### Handler with Command (Async)

For side effects like API calls:

```rust
fn fetch_data(model: &mut Model) -> Command<Message> {
    Command::perform(
        async { api::fetch_items().await },
        Message::ItemsLoaded,
    )
}
```

## Advanced Example: Todo App

### UI File: `ui/todo.gravity`

```xml
<column padding="40" spacing="20">
    <text value="Todo List" size="32" weight="bold" />
    
    <!-- Add new item -->
    <row spacing="10">
        <text_input 
            value="{new_item}"
            on_input="update_new_item"
            placeholder="What needs to be done?"
        />
        <button label="Add" on_click="add_item" enabled="{new_item.len() > 0}" />
    </row>
    
    <!-- List of items -->
    <column spacing="5">
        <text value="{if items.len() == 0 then 'No items yet!' else ''}" />
        <text value="Items: {items.len()}" />
    </column>
    
    <!-- Actions -->
    <row spacing="10">
        <button label="Clear All" on_click="clear_all" enabled="{items.len() > 0}" />
    </row>
</column>
```

### Model: `src/main.rs`

```rust
#[derive(Default, UiModel, Serialize, Deserialize)]
struct Model {
    new_item: String,
    items: Vec<String>,
}

#[derive(Clone, Debug)]
enum Message {
    UpdateNewItem(String),
    AddItem,
    ClearAll,
}

fn update_new_item(model: &mut Model, value: String) {
    model.new_item = value;
}

fn add_item(model: &mut Model) {
    if !model.new_item.is_empty() {
        model.items.push(model.new_item.clone());
        model.new_item.clear();
    }
}

fn clear_all(model: &mut Model) {
    model.items.clear();
}
```

## CLI Commands

### Development

```bash
gravity dev --ui ui --file main.gravity --verbose
```

### Build

```bash
gravity build --ui ui --output src/ui_generated.rs
```

### Validate

```bash
gravity check --ui ui
```

### Inspect

```bash
# View IR tree
gravity inspect --file ui/main.gravity

# View generated code
gravity inspect --file ui/main.gravity --codegen --handlers increment,decrement,reset

# JSON output for tooling
gravity inspect --file ui/main.gravity --format json
```

## Project Structure Best Practices

### Small Projects

```
my-app/
├── Cargo.toml
├── src/
│   └── main.rs
└── ui/
    └── main.gravity
```

### Medium Projects

```
my-app/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── model.rs      # Model definitions
│   └── handlers.rs   # Handler implementations
└── ui/
    ├── main.gravity
    └── components/
        ├── header.gravity
        └── footer.gravity
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
│   └── api.rs        # External API calls
└── ui/
    ├── main.gravity
    ├── pages/
    │   ├── home.gravity
    │   ├── todo.gravity
    │   └── settings.gravity
    └── components/
        ├── header.gravity
        ├── sidebar.gravity
        └── footer.gravity
```

## Common Patterns

### Form Validation

```xml
<column spacing="10">
    <text_input 
        value="{email}"
        on_input="update_email"
        placeholder="Email"
    />
    <text value="{if email_error then email_error else ''}" color="red" />
    
    <text_input 
        value="{password}"
        on_input="update_password"
        placeholder="Password"
        password="true"
    />
    
    <button 
        label="Sign Up" 
        on_click="sign_up"
        enabled="{email.len() > 0 && password.len() > 5 && !is_loading}"
    />
</column>
```

### Loading States

```xml
<column>
    <text value="{if is_loading then 'Loading...' else ''}" />
    
    <if condition="{!is_loading}">
        <text value="Content loaded!" />
    </if>
</column>
```

### Conditional Rendering

```xml
<column>
    <text value="{if error then 'Error occurred!' else ''}" color="red" />
    
    <if condition="{success}">
        <text value="Success!" color="green" />
    </if>
</column>
```

## Debugging Tips

### 1. Use `gravity inspect`

```bash
# See what the parser produces
gravity inspect --file ui/main.gravity

# Check generated code
gravity inspect --file ui/main.gravity --codegen
```

### 2. Use Verbose Mode

```bash
gravity dev --ui ui --file main.gravity --verbose
```

### 3. Check Bindings

If a binding doesn't work:
1. Verify field name matches exactly (case-sensitive)
2. Check field is not marked `#[ui_skip]`
3. Use `gravity check` to validate

### 4. Handler Issues

If handlers don't fire:
1. Check handler name matches XML `on_click="handler_name"`
2. Ensure handler is registered in HandlerRegistry
3. Verify handler signature matches expected type

## Adding Styles

Now let's make our app look better with themes and state-based styling.

### Step 1: Define a Theme

Update `ui/main.gravity`:

```xml
<?xml version="1.0" encoding="UTF-8"?>
<gravity>
    <themes>
        <theme name="app_theme">
            <palette 
                primary="#3498db" 
                secondary="#2ecc71"
                background="#ecf0f1" 
                surface="#ffffff"
                text="#2c3e50" />
            <typography font_family="Inter, sans-serif" />
            <spacing unit="8" />
        </theme>
    </themes>
    
    <global_theme name="app_theme" />
    
    <column padding="40" spacing="20" align="center">
        <text value="Counter: {count}" size="48" weight="bold" color="{theme.primary}" />
        
        <row spacing="20">
            <button label="-" on_click="decrement" enabled="{count > 0}" />
            <button label="+" on_click="increment" />
        </row>
        
        <button label="Reset" on_click="reset" />
    </column>
</gravity>
```

### Step 2: Add Style Classes

```xml
<style_classes>
    <style name="btn" 
        padding="12 24" 
        border_radius="6" 
        border_width="2" 
        background="{theme.primary}"
        color="#ffffff"
        border_color="#2980b9">
        <hover background="#2980b9" />
        <active background="#21618c" />
        <disabled opacity="0.5" />
    </style>
</style_classes>

<column padding="40" spacing="20" align="center">
    <text value="Counter: {count}" size="48" weight="bold" />
    
    <row spacing="20">
        <button class="btn" label="-" on_click="decrement" enabled="{count > 0}" />
        <button class="btn" label="+" on_click="increment" />
    </row>
    
    <button class="btn" label="Reset" on_click="reset" />
</column>
```

### Step 3: Make it Responsive

```xml
<column 
    padding="40" 
    mobile:padding="20"
    spacing="20"
    mobile:spacing="10"
    align="center">
    
    <text 
        value="Counter: {count}" 
        size="48"
        mobile:size="32"
        weight="bold" />
    
    <row spacing="20" mobile:spacing="10">
        <button class="btn" label="-" on_click="decrement" enabled="{count > 0}" />
        <button class="btn" label="+" on_click="increment" />
    </row>
    
    <button class="btn" label="Reset" on_click="reset" />
</column>
```

Now your app has:
- ✅ Custom theme with colors and typography
- ✅ Reusable button styles with hover/active states
- ✅ Responsive padding and sizing
- ✅ Automatic state transitions on hover/click

## Performance Tips

1. **Keep XML files small**: Split into components
2. **Use codegen for production**: Zero runtime overhead
3. **Minimize bindings**: Each binding adds evaluation cost
4. **Cache computed values**: In your model, not in XML
5. **Use classes**: Reuse styles instead of repeating attributes

## Next Steps

1. **Read the XML Schema Reference** - All widgets and attributes
2. **Explore Examples** - See more complex applications
3. **Join the Community** - Get help and share ideas
4. **Contribute** - Help improve the framework

## Troubleshooting

### "Unknown handler: xyz"

Ensure your handler is registered in the HandlerRegistry with the matching name.

### "Field not found: xyz"

Check:
- Field exists on Model
- Not marked `#[ui_skip]`
- Spelling matches exactly

### Hot-reload not working

1. Verify `--features dev` is enabled
2. Check file permissions
3. Ensure `.gravity` file is in watched directory

### Build errors

Run `gravity check` first to validate your XML.

## Support

- **Documentation**: https://docs.rs/gravity-core
- **GitHub Issues**: https://github.com/your-org/gravity/issues
- **Examples**: See `examples/` directory

---

**Ready to build something amazing? Let's go! 🚀**
