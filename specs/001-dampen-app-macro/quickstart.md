# Quickstart Guide: #[dampen_app] Macro

**Feature Branch**: `001-dampen-app-macro`  
**Date**: 2026-01-12  
**Status**: Complete

## Overview

This guide shows you how to use the `#[dampen_app]` procedural macro to eliminate boilerplate code in multi-view Dampen applications. By the end of this guide, you'll have a working 3-view application with less than 100 lines of code.

**What you'll learn**:
- How to apply the `#[dampen_app]` macro
- File structure conventions for multi-view apps
- How to add new views with zero boilerplate
- Common pitfalls and how to avoid them

---

## Prerequisites

- Rust 1.85+ (Edition 2024)
- Basic familiarity with Dampen framework
- Completion of Dampen [QUICKSTART.md](../../../docs/QUICKSTART.md) (hello-world example)

---

## Installation

The `#[dampen_app]` macro is part of `dampen-macros` crate (already installed if you have Dampen):

```toml
[dependencies]
dampen-core = "0.1"
dampen-iced = "0.1"
dampen-macros = "0.1"
iced = "0.14"

[dependencies.dampen-dev]
version = "0.1"
optional = true

[features]
default = ["dev"]
dev = ["dampen-dev"]
```

No additional dependencies required! 🎉

---

## Basic Example: 3-View Application

Let's build a simple application with three views: Main, Settings, and About.

### Step 1: Project Structure

Create the following file structure:

```
my-app/
├── Cargo.toml
├── build.rs
└── src/
    ├── main.rs
    └── ui/
        ├── mod.rs
        ├── main_view.dampen
        ├── main_view.rs
        ├── settings.dampen
        ├── settings.rs
        ├── about.dampen
        └── about.rs
```

### Step 2: Define Your Message Enum

In `src/main.rs`:

```rust
use dampen_core::HandlerMessage;
use dampen_dev::FileEvent;

#[derive(Clone, Debug)]
pub enum Message {
    Handler(HandlerMessage),
    #[cfg(debug_assertions)]
    HotReload(FileEvent),
    #[cfg(debug_assertions)]
    DismissError,
}
```

**Important**: The message enum needs variants for:
- `Handler(HandlerMessage)` - Required for UI event handling
- `HotReload(FileEvent)` - Optional for hot-reload support (debug builds)
- `DismissError` - Optional for error overlay dismissal (debug builds)

### Step 3: Apply the Macro

In `src/main.rs`:

```rust
use dampen_macros::dampen_app;

#[dampen_app(
    ui_dir = "src/ui",
    message_type = "Message",
    handler_variant = "Handler",
    hot_reload_variant = "HotReload",
    dismiss_error_variant = "DismissError"
)]
pub struct MyApp;
```

**That's it!** The macro generates all the boilerplate code for you:
- ✅ `CurrentView` enum with 3 variants (MainView, Settings, About)
- ✅ App struct with 3 `AppState` fields
- ✅ `init()` and `new()` methods
- ✅ `update()` method with view switching and handler dispatch
- ✅ `view()` method with error overlay support
- ✅ `subscription()` method for hot-reload
- ✅ Helper methods: `switch_to_main_view()`, `switch_to_settings()`, `switch_to_about()`

### Step 4: Create View Definitions

**`src/ui/main_view.dampen`**:
```xml
<dampen>
    <column padding="20" spacing="10">
        <text value="Welcome to My App!" size="32" />
        <button label="Go to Settings" on_click="switch_to_settings" />
        <button label="About" on_click="switch_to_about" />
    </column>
</dampen>
```

**`src/ui/main_view.rs`**:
```rust
use dampen_macros::{dampen_ui, UiModel};
use dampen_core::HandlerRegistry;

#[derive(UiModel)]
pub struct Model {
    // Empty model for static view
}

#[dampen_ui("main_view.dampen")]
mod _main_view {}

pub fn create_handlers() -> HandlerRegistry {
    let mut registry = HandlerRegistry::new();
    
    registry.register_simple("switch_to_settings", || {
        // Handler automatically routed by macro
        println!("Switching to settings...");
    });
    
    registry.register_simple("switch_to_about", || {
        println!("Switching to about...");
    });
    
    registry
}
```

**`src/ui/settings.dampen`**:
```xml
<dampen>
    <column padding="20" spacing="10">
        <text value="Settings" size="32" />
        <text value="Customize your experience" />
        <button label="Back to Main" on_click="switch_to_main_view" />
    </column>
</dampen>
```

**`src/ui/settings.rs`**:
```rust
use dampen_macros::{dampen_ui, UiModel};
use dampen_core::HandlerRegistry;

#[derive(UiModel)]
pub struct Model {}

#[dampen_ui("settings.dampen")]
mod _settings {}

pub fn create_handlers() -> HandlerRegistry {
    let mut registry = HandlerRegistry::new();
    
    registry.register_simple("switch_to_main_view", || {
        println!("Switching to main view...");
    });
    
    registry
}
```

**`src/ui/about.dampen`**:
```xml
<dampen>
    <column padding="20" spacing="10">
        <text value="About My App" size="32" />
        <text value="Version 1.0.0" />
        <text value="Built with Dampen Framework" />
        <button label="Back" on_click="switch_to_main_view" />
    </column>
</dampen>
```

**`src/ui/about.rs`**:
```rust
use dampen_macros::{dampen_ui, UiModel};
use dampen_core::HandlerRegistry;

#[derive(UiModel)]
pub struct Model {}

#[dampen_ui("about.dampen")]
mod _about {}

pub fn create_handlers() -> HandlerRegistry {
    let mut registry = HandlerRegistry::new();
    
    registry.register_simple("switch_to_main_view", || {
        println!("Switching to main view...");
    });
    
    registry
}
```

**`src/ui/mod.rs`**:
```rust
pub mod main_view;
pub mod settings;
pub mod about;
```

### Step 5: Wire Up Iced Application

In `src/main.rs`:

```rust
use iced::{Application, Settings};

fn main() -> iced::Result {
    MyApp::run(Settings::default())
}

impl Application for MyApp {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = iced::Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Task<Self::Message>) {
        MyApp::init()
    }

    fn title(&self) -> String {
        "My Multi-View App".to_string()
    }

    fn update(&mut self, message: Self::Message) -> iced::Task<Self::Message> {
        self.update(message)
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        self.view()
    }

    #[cfg(debug_assertions)]
    fn subscription(&self) -> iced::Subscription<Self::Message> {
        self.subscription()
    }
}
```

### Step 6: Add Build Script

**`build.rs`**:
```rust
fn main() {
    // Tell Cargo to re-run when UI files change
    println!("cargo:rerun-if-changed=src/ui/");
}
```

### Step 7: Run!

```bash
cargo run
```

You now have a fully functional 3-view application with:
- **Zero manual routing logic** ✅
- **Automatic view discovery** ✅
- **Hot-reload support** (in debug mode) ✅
- **Error overlay** (in debug mode) ✅

---

## Adding a New View

To add a new view (e.g., "Help"):

1. **Create two files**:
   - `src/ui/help.dampen` (UI definition)
   - `src/ui/help.rs` (Model and handlers)

2. **Add to `src/ui/mod.rs`**:
   ```rust
   pub mod help;
   ```

3. **Recompile**:
   ```bash
   cargo build
   ```

**That's it!** The macro automatically:
- ✅ Discovers the new view
- ✅ Adds `Help` variant to `CurrentView` enum
- ✅ Adds `help_state` field to app struct
- ✅ Generates `switch_to_help()` method
- ✅ Includes in hot-reload subscription

**No manual wiring required!** 🎉

---

## File Structure Best Practices

### Flat Structure (3-10 views)

```
src/ui/
├── mod.rs
├── view1.dampen
├── view1.rs
├── view2.dampen
└── view2.rs
```

**Pros**: Simple, easy to navigate  
**Cons**: Gets messy with many views

### Nested Structure (10+ views)

```
src/ui/
├── mod.rs
├── screens/
│   ├── mod.rs
│   ├── home.dampen
│   ├── home.rs
│   ├── profile.dampen
│   └── profile.rs
├── dialogs/
│   ├── mod.rs
│   ├── confirm.dampen
│   └── confirm.rs
└── widgets/
    ├── mod.rs
    ├── sidebar.dampen
    └── sidebar.rs
```

**Pros**: Organized by purpose  
**Cons**: Slightly more module boilerplate

**Note**: Module paths are preserved (e.g., `ui::screens::home`), but view names still use the filename only (`home`, not `screens_home`).

---

## Common Pitfalls & Solutions

### ❌ Pitfall 1: Missing .rs File

**Error**:
```
error: No matching Rust module found for '/path/to/src/ui/view.dampen'
       help: Create a file at '/path/to/src/ui/view.rs' with a Model struct
```

**Solution**: Create the corresponding `.rs` file:
```bash
touch src/ui/view.rs
```

Add minimal content:
```rust
use dampen_macros::{dampen_ui, UiModel};

#[derive(UiModel)]
pub struct Model {}

#[dampen_ui("view.dampen")]
mod _view {}
```

---

### ❌ Pitfall 2: View Naming Conflict

**Error**:
```
error: View naming conflict: 'Input' found in multiple locations:
       - /path/to/src/ui/form/input.dampen
       - /path/to/src/ui/dialog/input.dampen
```

**Solution A**: Rename one of the files:
```bash
mv src/ui/dialog/input.dampen src/ui/dialog/dialog_input.dampen
mv src/ui/dialog/input.rs src/ui/dialog/dialog_input.rs
```

**Solution B**: Exclude one of them:
```rust
#[dampen_app(
    ui_dir = "src/ui",
    message_type = "Message",
    handler_variant = "Handler",
    exclude = ["dialog/input"]  // Exclude the dialog variant
)]
```

---

### ❌ Pitfall 3: Invalid View Name

**Error**:
```
error: Invalid view name '123-view' in '/path/to/src/ui/123-view.dampen'
       help: View names must be valid Rust identifiers
```

**Solution**: Rename the file to use valid identifier characters:
```bash
mv src/ui/123-view.dampen src/ui/my_view_123.dampen
mv src/ui/123-view.rs src/ui/my_view_123.rs
```

Valid names: `my_view`, `main_window`, `settings_v2`  
Invalid names: `123-view`, `my-view`, `view name with spaces`

---

### ❌ Pitfall 4: Handler Not Switching Views

**Problem**: Button click doesn't switch views.

**Cause**: Handler name doesn't match generated method name.

**Solution**: Use the exact generated method name in your handler:

```rust
// Generated method: switch_to_settings()
// Handler name must be: "switch_to_settings"

registry.register_simple("switch_to_settings", || {
    // This will work! Macro routes to app.switch_to_settings()
});
```

**Pattern**: `switch_to_{view_name}` where `{view_name}` is the filename (snake_case).

---

### ❌ Pitfall 5: Hot-Reload Not Working

**Problem**: File changes don't trigger reload.

**Checklist**:
1. ✅ Is `hot_reload_variant` specified in macro attributes?
   ```rust
   #[dampen_app(
       hot_reload_variant = "HotReload",  // Must be present
       ...
   )]
   ```

2. ✅ Is `dampen-dev` dependency enabled?
   ```toml
   [dependencies.dampen-dev]
   version = "0.1"
   optional = true
   
   [features]
   default = ["dev"]
   dev = ["dampen-dev"]
   ```

3. ✅ Is `subscription()` method called in Iced `Application` impl?
   ```rust
   #[cfg(debug_assertions)]
   fn subscription(&self) -> iced::Subscription<Self::Message> {
       self.subscription()  // Must call this!
   }
   ```

4. ✅ Are you running in debug mode?
   ```bash
   cargo run  # Debug mode (hot-reload enabled)
   # NOT: cargo run --release (hot-reload disabled)
   ```

---

### ❌ Pitfall 6: `create_handlers()` Not Found

**Problem**: Compile error about missing `create_handlers()` function.

**Cause**: The function is optional, but the module structure might be wrong.

**Solution**: Ensure your view module has the correct exports:

```rust
// src/ui/my_view.rs
use dampen_macros::{dampen_ui, UiModel};
use dampen_core::HandlerRegistry;

#[derive(UiModel)]
pub struct Model {}

#[dampen_ui("my_view.dampen")]
mod _my_view {}

// This function is OPTIONAL but recommended
pub fn create_handlers() -> HandlerRegistry {
    HandlerRegistry::new()
}
```

If you don't need handlers, omit the function entirely - the macro will use an empty registry.

---

## Excluding Views

Use the `exclude` attribute to skip certain views:

### Example: Exclude Debug Views

```rust
#[dampen_app(
    ui_dir = "src/ui",
    message_type = "Message",
    handler_variant = "Handler",
    exclude = ["debug_panel", "test_view"]
)]
pub struct MyApp;
```

### Example: Exclude by Directory

```rust
#[dampen_app(
    ui_dir = "src/ui",
    message_type = "Message",
    handler_variant = "Handler",
    exclude = ["experimental/*", "debug/*"]
)]
pub struct MyApp;
```

**Glob Pattern Support**:
- `*` - Matches any characters except `/`
- `?` - Matches exactly one character
- `[abc]` - Matches one character in the set
- `**` - Matches any number of directories (not yet implemented)

---

## Production Build Considerations

### Disabling Hot-Reload

Hot-reload code is automatically excluded from release builds via `#[cfg(debug_assertions)]`. No manual configuration needed!

```bash
cargo build --release  # Hot-reload code stripped out
```

### Omitting Hot-Reload Variants

For production-only applications, omit the hot-reload attributes entirely:

```rust
#[dampen_app(
    ui_dir = "src/ui",
    message_type = "Message",
    handler_variant = "Handler"
    // No hot_reload_variant or dismiss_error_variant
)]
pub struct MyApp;
```

This prevents generation of `subscription()` method and error overlay logic.

---

## Advanced Usage

### Custom Message Types

You can use any message enum name:

```rust
#[derive(Clone, Debug)]
pub enum AppMessage {
    UiEvent(HandlerMessage),
}

#[dampen_app(
    ui_dir = "src/ui",
    message_type = "AppMessage",   // Custom name
    handler_variant = "UiEvent"    // Custom variant
)]
pub struct MyApp;
```

### Multiple UI Directories (Not Yet Supported)

Currently, the macro only supports a single `ui_dir`. To work around this:

**Option A**: Use a single directory with subdirectories:
```
src/ui/
├── app/
└── components/
```

**Option B**: Use multiple macro invocations on different structs (advanced).

---

## Migrating from Manual Code

If you have an existing multi-view app with manual boilerplate:

### Before (Manual Code - ~500 lines for 20 views)

```rust
pub struct MyApp {
    current_view: CurrentView,
    view1_state: AppState<ui::view1::Model>,
    view2_state: AppState<ui::view2::Model>,
    // ... 18 more fields
}

impl MyApp {
    pub fn new() -> (Self, Task<Message>) {
        let view1_state = /* initialize */;
        let view2_state = /* initialize */;
        // ... 18 more initializations
        // ... 50 lines of boilerplate
    }
    
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Handler(h) => {
                match self.current_view {
                    CurrentView::View1 => self.view1_state.dispatch_handler(/* ... */),
                    CurrentView::View2 => self.view2_state.dispatch_handler(/* ... */),
                    // ... 18 more branches (80 lines)
                }
            }
            // ... more match arms
        }
    }
    
    pub fn view(&self) -> Element<'_, Message> {
        match self.current_view {
            CurrentView::View1 => /* render view1 */,
            CurrentView::View2 => /* render view2 */,
            // ... 18 more branches (100 lines)
        }
    }
    
    pub fn subscription(&self) -> Subscription<Message> {
        Subscription::batch(vec![
            self.view1_state.watch().map(Message::HotReload),
            self.view2_state.watch().map(Message::HotReload),
            // ... 18 more lines
        ])
    }
}
```

### After (With Macro - ~50 lines)

```rust
#[dampen_app(
    ui_dir = "src/ui",
    message_type = "Message",
    handler_variant = "Handler",
    hot_reload_variant = "HotReload",
    dismiss_error_variant = "DismissError"
)]
pub struct MyApp;

impl Application for MyApp {
    // ... Iced trait implementation (same as before)
}
```

**Savings**: 450 lines eliminated! 🎉

---

## Troubleshooting

### Macro Not Expanding?

Run with verbose output to see macro expansion:

```bash
cargo build -vv
```

Or use `cargo expand` (requires `cargo-expand` tool):

```bash
cargo install cargo-expand
cargo expand
```

### Type Errors After Macro?

Common causes:
1. Missing `pub mod` declaration in `src/ui/mod.rs`
2. `Model` struct not public (`pub struct Model`)
3. Missing `#[derive(UiModel)]` on Model

### Compilation Slow?

The macro runs during compilation. For 20 views, expect <200ms overhead. If slower:
- Check for excessive file I/O (large directories)
- Consider excluding unnecessary views
- Profile with `cargo build --timings`

---

## Next Steps

- **Read the full specification**: [spec.md](./spec.md)
- **Explore examples**: See `examples/widget-showcase` for a 20-view app
- **Contribute**: Check [CONTRIBUTING.md](../../../docs/CONTRIBUTING.md)

---

## Summary

The `#[dampen_app]` macro eliminates 85% of boilerplate code in multi-view Dampen applications:

✅ **Automatic view discovery** - Just create `.dampen` + `.rs` files  
✅ **Zero routing logic** - View switching handled automatically  
✅ **Hot-reload support** - File changes trigger instant reloads  
✅ **Type-safe** - All generated code is strongly typed  
✅ **Production-ready** - Hot-reload code stripped in release builds  

**Key Takeaways**:
1. Apply macro to app struct with required attributes
2. Create `.dampen` + `.rs` pairs in `ui_dir`
3. Define `Model` struct with `#[derive(UiModel)]`
4. Use `#[dampen_ui]` for XML loading
5. Register handlers in `create_handlers()` function
6. Call generated methods from Iced `Application` trait

Happy coding! 🚀
