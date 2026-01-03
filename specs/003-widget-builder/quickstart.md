# Quickstart: Gravity Widget Builder

**Date**: 2026-01-03  
**Feature**: 003-widget-builder  
**Audience**: Developers using Gravity framework

---

## Installation

### Prerequisites

- Rust Edition 2024 or later
- `gravity-core` crate (existing)
- `gravity-runtime` crate (existing)
- `gravity-iced` crate (will include builder)
- `iced` 0.14+ (existing)

### Add Dependency

In your `Cargo.toml`:

```toml
[dependencies]
gravity-iced = { path = "../crates/gravity-iced" }
gravity-core = { path = "../crates/gravity-core" }
gravity-runtime = { path = "../crates/gravity-runtime" }
iced = "0.14"
```

---

## Basic Usage

### Step 1: Parse Your UI

```rust
use gravity_core::parser::parse_ui_file;

let document = parse_ui_file("ui/main.gravity")?;
// Returns: Document { root: WidgetNode, ... }
```

### Step 2: Define Your State

```rust
use gravity_core::binding::UiBindable;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, UiBindable)]
struct AppState {
    count: i32,
    username: String,
}

impl AppState {
    fn increment(&mut self) {
        self.count += 1;
    }
}
```

### Step 3: Create Handler Registry

```rust
use gravity_runtime::handler::{HandlerRegistry, Handler};

let mut registry = HandlerRegistry::new();

registry.register("increment", Handler::new(
    |state: &mut AppState, _payload| {
        state.increment();
        None
    }
));

registry.register("reset", Handler::new(
    |state: &mut AppState, _payload| {
        state.count = 0;
        None
    }
));
```

### Step 4: Build Your View

```rust
use gravity_iced::GravityWidgetBuilder;

fn view(state: &AppState) -> Element<'_, Message> {
    GravityWidgetBuilder::new(
        &document.root,
        state,
        Some(®istry)
    ).build()
}
```

That's it! **10 lines vs 410 lines**.

---

## Complete Example

### File: `src/main.rs`

```rust
use iced::{Application, Command, Element, Settings, Theme};
use gravity_core::parser::parse_ui_file;
use gravity_core::binding::UiBindable;
use gravity_iced::GravityWidgetBuilder;
use gravity_runtime::handler::{HandlerRegistry, Handler};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, UiBindable, Clone)]
struct AppState {
    count: i32,
    username: String,
}

#[derive(Clone)]
enum Message {
    Increment,
    Reset,
    UpdateUsername(String),
}

impl Application for MyApp {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let document = parse_ui_file("ui/main.gravity").unwrap();
        let mut registry = HandlerRegistry::new();
        
        registry.register("increment", Handler::new(
            |state: &mut AppState, _payload| {
                state.count += 1;
                None
            }
        ));
        
        registry.register("reset", Handler::new(
            |state: &mut AppState, _payload| {
                state.count = 0;
                None
            }
        ));
        
        (
            MyApp {
                state: AppState { 
                    count: 0, 
                    username: "User".to_string() 
                },
                document,
                registry,
            },
            Command::none()
        )
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Increment => self.state.increment(),
            Message::Reset => self.state.count = 0,
            Message::UpdateUsername(name) => self.state.username = name,
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        GravityWidgetBuilder::new(
            &self.document.root,
            &self.state,
            Some(&self.registry)
        ).build()
    }
}

fn main() -> iced::Result {
    MyApp::run(Settings::default())
}
```

### File: `ui/main.gravity`

```xml
<column padding="40" spacing="20">
    <text value="Counter App" size="32" weight="bold" />
    <text value="User: {username}" size="18" />
    <text value="Count: {count}" size="24" color="#3498db" />
    
    <row spacing="10">
        <button label="Increment" on_click="increment" 
                background="#2ecc71" />
        <button label="Reset" on_click="reset" 
                background="#e74c3c" />
    </row>
</column>
```

**Result**: 60 lines of Rust vs 410 lines manually.

---

## Advanced Usage

### Verbose Logging (Debug Mode)

```rust
fn view(state: &AppState) -> Element<'_, Message> {
    GravityWidgetBuilder::new(
        &document.root,
        state,
        Some(®istry)
    )
    .with_verbose(true)  // Enable debug output
    .build()
}
```

**Output**:
```
[GravityWidgetBuilder] Processing column widget
[GravityWidgetBuilder] Binding: {username} → "Alice"
[GravityWidgetBuilder] Event: increment → Handler found
[GravityWidgetBuilder] Style: background #3498db → Color { r: 0.204, ... }
```

### Static UI (No Interactivity)

```rust
fn view(state: &AppState) -> Element<'_, Message> {
    GravityWidgetBuilder::new(
        &document.root,
        state,
        None  // No handlers
    ).build()
}
```

### Custom Model (Manual UiBindable)

```rust
struct CustomState {
    data: HashMap<String, String>,
}

impl UiBindable for CustomState {
    fn get_field(&self, path: &[&str]) -> Option<BindingValue> {
        if path.len() == 1 {
            self.data.get(path[0])
                .map(|s| BindingValue::String(s.clone()))
        } else {
            None
        }
    }
    
    fn available_fields() -> Vec<String> {
        vec!["key1".to_string(), "key2".to_string()]
    }
}
```

---

## Supported Widgets

### Text

```xml
<text value="Hello {name}" size="16" weight="bold" color="#333" />
```

**Properties**:
- `value`: Text content (supports bindings)
- `size`: Font size in px
- `weight`: normal | bold
- `color`: Hex color

### Button

```xml
<button label="Click me" on_click="handler_name" 
        background="#3498db" padding="10" />
```

**Properties**:
- `label`: Button text
- `on_click`: Handler name (optional)
- `background`: Hex color
- `padding`: Spacing around label

### Column

```xml
<column spacing="20" padding="10">
    <!-- children -->
</column>
```

**Properties**:
- `spacing`: Space between children
- `padding`: Space around container

### Row

```xml
<row spacing="10" padding="5">
    <!-- children -->
</row>
```

**Properties**:
- `spacing`: Space between children
- `padding`: Space around container

### Container

```xml
<container width="300" height="200" background="#f0f0f0">
    <!-- single child -->
</container>
```

**Properties**:
- `width`: Fixed width or "fill" or "auto"
- `height`: Fixed height or "fill" or "auto"
- `background`: Hex color

---

## Binding Examples

### Simple Field

```xml
<text value="{count}" />
```

### Nested Field

```xml
<text value="{user.name}" />
```

### Expression

```xml
<text value="Total: {count * price}" />
```

### Conditional (in expression)

```xml
<text value="{if active 'Online' 'Offline'}" />
```

---

## Event Handling

### Handler Signature

```rust
fn handler(state: &mut AppState, payload: Option<String>) -> Option<Message>
```

**Return**: Optional message to trigger additional actions

### Registration

```rust
registry.register("increment", Handler::new(|state, payload| {
    state.count += 1;
    if state.count >= 10 {
        Some(Message::ShowAlert)
    } else {
        None
    }
}));
```

### XML Usage

```xml
<button label="Add" on_click="increment" />
```

---

## Style Reference

### Colors

```xml
<!-- Hex -->
<text color="#ff0000" />
<text color="#f00" />

<!-- Named (if supported) -->
<text color="red" />
```

### Lengths

```xml
<!-- Fixed pixels -->
<container width="300" />

<!-- Percentage -->
<container width="50%" />

<!-- Fill available space -->
<container width="fill" />

<!-- Automatic -->
<container width="auto" />
```

### Padding/Spacing

```xml
<!-- Single value (all sides) -->
<column padding="20" />

<!-- Multiple values (top right bottom left) -->
<column padding="10 20 10 20" />
```

### Borders

```xml
<container border="2 5 #333333" />
<!-- width radius color -->
```

---

## Performance Tips

### 1. Reuse Builder

```rust
// BAD: Creates new builder every frame
fn view(&self) -> Element<'_, Message> {
    GravityWidgetBuilder::new(...).build()
}

// GOOD: Cache if document doesn't change
fn view(&self) -> Element<'_, Message> {
    self.cached_builder.build()
}
```

### 2. Verbose Mode Only in Dev

```rust
.with_verbose(cfg!(debug_assertions))
```

### 3. Optimize Model

```rust
// Use references instead of cloning
impl UiBindable for MyState {
    fn get_field(&self, path: &[&str]) -> Option<BindingValue> {
        // Fast path: direct field access
        if path == ["count"] {
            return Some(BindingValue::Int(self.count));
        }
        // Slow path: complex lookup
        // ...
    }
}
```

---

## Troubleshooting

### Issue: Widget not rendering

**Check**:
1. Is the XML valid?
2. Are all required attributes present?
3. Is verbose mode enabled?

```rust
.with_verbose(true)
```

### Issue: Binding not updating

**Check**:
1. Does model implement `UiBindable`?
2. Is the field path correct?
3. Is the model being cloned (should be reference)?

### Issue: Event not firing

**Check**:
1. Is handler registered with exact name?
2. Is registry passed to builder?
3. Is verbose mode showing handler lookup?

### Issue: Performance slow

**Check**:
1. Are you rebuilding on every frame unnecessarily?
2. Is model `get_field` implementation efficient?
3. Run with `with_verbose(true)` to see timing

---

## Migration Checklist

- [ ] Parse XML to get document
- [ ] Implement `UiBindable` on state (or use derive)
- [ ] Create `HandlerRegistry` and register handlers
- [ ] Replace manual rendering with `GravityWidgetBuilder::new(...).build()`
- [ ] Test all bindings work
- [ ] Test all events fire
- [ ] Verify styles apply correctly
- [ ] Check performance meets targets
- [ ] Enable verbose mode to debug issues

---

## Next Steps

1. **Read the spec**: `specs/003-widget-builder/spec.md`
2. **See the plan**: `specs/003-widget-builder/plan.md`
3. **Check API**: `specs/003-widget-builder/contracts/api.md`
4. **Review data model**: `specs/003-widget-builder/data-model.md`

---

## Support

For issues or questions:
- Check verbose logging output
- Review edge cases in spec
- Consult existing examples in repository
