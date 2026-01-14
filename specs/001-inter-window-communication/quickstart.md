# Quickstart: Shared State in Dampen

**Feature**: Inter-Window Communication  
**Time to Complete**: ~5 minutes

---

## Overview

This guide shows you how to share state between views in a Dampen multi-view application. You'll learn to:

1. Define a shared state type
2. Configure the `#[dampen_app]` macro
3. Use `{shared.field}` bindings in XML
4. Modify shared state from handlers

---

## Prerequisites

- Existing Dampen application with multiple views
- Dampen v0.2.4 or later

---

## Step 1: Define Your Shared State

Create a new file for your shared state type:

```rust
// src/shared.rs
use dampen_macros::UiModel;
use serde::{Deserialize, Serialize};

/// Application-wide shared state.
///
/// This state is accessible from all views in your application.
#[derive(Default, Clone, Debug, UiModel, Serialize, Deserialize)]
pub struct SharedState {
    /// Current theme ("light" or "dark")
    pub theme: String,
    
    /// Currently logged-in user (if any)
    pub user: Option<User>,
    
    /// Global notification count
    pub notification_count: u32,
}

#[derive(Clone, Debug, Default, UiModel, Serialize, Deserialize)]
pub struct User {
    pub name: String,
    pub email: String,
}
```

**Requirements**:
- `#[derive(UiModel)]` - Enables field access in XML bindings
- `Send + Sync` - Automatic for types without interior mutability
- `'static` - No borrowed references

---

## Step 2: Configure the Macro

Update your `main.rs` to include the shared state:

```rust
// src/main.rs
use dampen_macros::dampen_app;

mod shared;  // Add the shared module
mod ui;

use shared::SharedState;

#[derive(Clone, Debug)]
pub enum Message {
    Handler(dampen_core::HandlerMessage),
    SwitchToView(CurrentView),
    #[cfg(debug_assertions)]
    HotReload(notify::Event),
}

#[dampen_app(
    ui_dir = "src/ui",
    message_type = "Message",
    handler_variant = "Handler",
    switch_view_variant = "SwitchToView",
    #[cfg(debug_assertions)]
    hot_reload_variant = "HotReload",
    
    // NEW: Enable shared state
    shared_model = "SharedState",
)]
struct MyApp;

fn main() -> iced::Result {
    MyApp::run()
}
```

---

## Step 3: Use Shared Bindings in XML

Access shared state in your `.dampen` files using the `{shared.}` prefix:

```xml
<!-- src/ui/window.dampen -->
<column padding="20" spacing="10">
    <!-- Display shared user name -->
    <text value="Welcome, {shared.user.name}!" size="24" />
    
    <!-- Show notification badge -->
    <text value="Notifications: {shared.notification_count}" />
    
    <!-- Conditional visibility based on shared state -->
    <button 
        label="Log In" 
        visible="{!shared.user.is_some()}"
        on_click="show_login" 
    />
    
    <!-- Theme indicator -->
    <text value="Current theme: {shared.theme}" />
</column>
```

```xml
<!-- src/ui/settings.dampen -->
<column padding="20" spacing="10">
    <text value="Settings" size="28" weight="bold" />
    
    <!-- Theme selector affects all views -->
    <row spacing="10">
        <text value="Theme:" />
        <button label="Light" on_click="set_light_theme" />
        <button label="Dark" on_click="set_dark_theme" />
    </row>
    
    <!-- Current theme from shared state -->
    <text value="Active: {shared.theme}" />
</column>
```

---

## Step 4: Modify Shared State from Handlers

Update your view modules to use shared-aware handlers:

```rust
// src/ui/settings.rs
use dampen_core::{AppState, HandlerRegistry, SharedContext};
use dampen_macros::{dampen_ui, UiModel};
use std::any::Any;

use crate::shared::SharedState;

#[derive(Default, Clone, UiModel)]
pub struct Model {
    pub selected_theme: String,
}

#[dampen_ui("settings.dampen")]
mod _settings {}

pub fn create_app_state_with_shared(
    shared: SharedContext<SharedState>,
) -> AppState<Model, SharedState> {
    let document = _settings::document();
    let handlers = create_handler_registry();
    AppState::with_shared(document, Model::default(), handlers, shared)
}

fn create_handler_registry() -> HandlerRegistry {
    let registry = HandlerRegistry::new();
    
    // Handler that modifies shared state
    registry.register_with_shared("set_light_theme", |model, shared| {
        let shared = shared
            .downcast_ref::<SharedContext<SharedState>>()
            .expect("Invalid shared context type");
        
        // Modify shared state - all views will see this change
        shared.write().theme = "light".to_string();
    });
    
    registry.register_with_shared("set_dark_theme", |model, shared| {
        let shared = shared
            .downcast_ref::<SharedContext<SharedState>>()
            .expect("Invalid shared context type");
        
        shared.write().theme = "dark".to_string();
    });
    
    registry
}
```

---

## Step 5: Run and Test

```bash
# Run the application
dampen run

# Or with cargo
cargo run
```

**Expected Behavior**:
1. Click "Dark" in Settings view
2. Navigate to Window view
3. Theme displays as "dark" (shared state updated across views)

---

## Complete Example Structure

```
my-app/
├── Cargo.toml
├── src/
│   ├── main.rs              # #[dampen_app] with shared_model
│   ├── shared.rs            # SharedState definition
│   └── ui/
│       ├── mod.rs
│       ├── window.rs        # View with {shared.} bindings
│       ├── window.dampen
│       ├── settings.rs      # View with shared handlers
│       └── settings.dampen
└── tests/
    └── integration.rs
```

---

## Common Patterns

### Pattern 1: Read Shared State in Handler

```rust
registry.register_with_shared("check_auth", |model, shared| {
    let model = model.downcast_mut::<Model>().unwrap();
    let shared = shared.downcast_ref::<SharedContext<SharedState>>().unwrap();
    
    // Read shared state
    if shared.read().user.is_some() {
        model.show_profile = true;
    } else {
        model.show_login = true;
    }
});
```

### Pattern 2: Update Both Local and Shared State

```rust
registry.register_with_value_and_shared("update_username", |model, value, shared| {
    let model = model.downcast_mut::<Model>().unwrap();
    let value = value.downcast_ref::<String>().unwrap();
    let shared = shared.downcast_ref::<SharedContext<SharedState>>().unwrap();
    
    // Update local model
    model.input_value = value.clone();
    
    // Update shared state
    if let Some(ref mut user) = shared.write().user {
        user.name = value.clone();
    }
});
```

### Pattern 3: Conditional Binding

```xml
<!-- Show different content based on shared state -->
<column>
    <text 
        value="Hello, {shared.user.name}!"
        visible="{shared.user.is_some()}" 
    />
    <text 
        value="Please log in"
        visible="{!shared.user.is_some()}" 
    />
</column>
```

---

## Troubleshooting

### Binding shows empty value

**Cause**: Field doesn't exist or is `None`

**Solution**: Check field name matches SharedState definition

```xml
<!-- Typo: 'theme' not 'themes' -->
<text value="{shared.themes}" />  <!-- Empty! -->
<text value="{shared.theme}" />   <!-- Correct -->
```

### Downcast panic in handler

**Cause**: Wrong type in downcast

**Solution**: Ensure types match

```rust
// Wrong: Using Model instead of SharedContext
let shared = shared.downcast_ref::<Model>().unwrap(); // PANIC!

// Correct
let shared = shared.downcast_ref::<SharedContext<SharedState>>().unwrap();
```

### Changes not reflected in other views

**Cause**: Modifying local model instead of shared state

**Solution**: Use `shared.write()` to modify shared state

```rust
// Wrong: Only updates local model
model.theme = "dark".to_string();

// Correct: Updates shared state (visible to all views)
shared.write().theme = "dark".to_string();
```

---

## Next Steps

- See [data-model.md](./data-model.md) for type definitions
- See [contracts/](./contracts/) for detailed API contracts
- See [research.md](./research.md) for design decisions
