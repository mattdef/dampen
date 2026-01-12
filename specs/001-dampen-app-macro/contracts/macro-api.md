# API Contract: #[dampen_app] Macro

**Feature Branch**: `001-dampen-app-macro`  
**Date**: 2026-01-12  
**Status**: Complete

## Overview

This document defines the complete public API surface of the `#[dampen_app]` procedural macro, including:
- Macro attribute syntax
- Input requirements
- Generated code structure
- Error cases with expected messages

This contract serves as the specification for implementation and the basis for compile-fail tests.

---

## Macro Signature

### Attribute Syntax

```rust
#[dampen_app(
    ui_dir = "src/ui",                    // Required: String literal
    message_type = "Message",             // Required: Identifier
    handler_variant = "Handler",          // Required: Identifier
    hot_reload_variant = "HotReload",     // Optional: Identifier
    dismiss_error_variant = "DismissError", // Optional: Identifier
    exclude = ["debug_view", "experimental/*"] // Optional: Array of string literals
)]
```

### Attribute Parameters

| Parameter | Required | Type | Description | Example |
|-----------|----------|------|-------------|---------|
| `ui_dir` | ✅ Yes | String literal | Directory to scan for `.dampen` files (relative to crate root) | `"src/ui"` |
| `message_type` | ✅ Yes | Identifier | Name of the user's Message enum | `"Message"` |
| `handler_variant` | ✅ Yes | Identifier | Message variant for `HandlerMessage` dispatch | `"Handler"` |
| `hot_reload_variant` | ❌ No | Identifier | Message variant for `FileEvent` hot-reload | `"HotReload"` |
| `dismiss_error_variant` | ❌ No | Identifier | Message variant for error overlay dismissal | `"DismissError"` |
| `exclude` | ❌ No | Array of string literals | Glob patterns to exclude from discovery | `["debug_*", "test/*"]` |

### Parameter Constraints

- **`ui_dir`**: Must be a valid path that exists on the file system
- **Identifiers**: Must be valid Rust identifiers (not keywords, alphanumeric + underscores)
- **`exclude`**: Each pattern must be a valid glob pattern (supports `*`, `?`, `[abc]`, `**`)
- **Order**: Parameters can appear in any order
- **Duplicates**: Last value wins (no error, but warning recommended)

---

## Input Contract

### Annotated Item

The macro must be applied to a **struct definition**:

```rust
#[dampen_app(/* attributes */)]
struct MyApp;
```

**Constraints**:
- MUST be a struct (tuple struct, unit struct, or regular struct all accepted)
- Struct MAY have visibility modifiers (`pub`, `pub(crate)`, etc.)
- Struct MAY have generics (though not recommended)
- Struct body will be REPLACED by generated code

### File System Requirements

For each discovered `.dampen` file, the following structure is required:

```
src/ui/
├── view1.dampen          # UI definition
├── view1.rs              # MUST exist
│   └── pub struct Model  # MUST exist
├── view2.dampen
└── view2.rs
    └── pub struct Model
```

**Required in each `.rs` file**:
- `pub struct Model` with `#[derive(UiModel)]` (or manual `UiBindable` impl)
- Optional: Handler functions registered via `create_handlers()`
- Optional: Custom initialization logic

---

## Output Contract

### Generated Code Structure

Given the following input:

```rust
#[dampen_app(
    ui_dir = "src/ui",
    message_type = "Message",
    handler_variant = "Handler",
    hot_reload_variant = "HotReload",
    dismiss_error_variant = "DismissError"
)]
struct MyApp;
```

And the following discovered files:
- `src/ui/window.dampen` + `src/ui/window.rs`
- `src/ui/settings.dampen` + `src/ui/settings.rs`

The macro generates:

### 1. CurrentView Enum

```rust
#[derive(Clone, Debug, PartialEq, Eq)]
enum CurrentView {
    Window,
    Settings,
}

impl Default for CurrentView {
    fn default() -> Self {
        // First view in alphabetical order
        CurrentView::Window
    }
}
```

**Contract**:
- One variant per discovered view (from `ViewInfo::variant_name`)
- Sorted alphabetically by variant name (deterministic)
- Derives: `Clone`, `Debug`, `PartialEq`, `Eq`
- Implements `Default` using first variant
- Visibility: Same as annotated struct

### 2. App Struct Definition

```rust
struct MyApp {
    current_view: CurrentView,
    window_state: dampen_core::AppState<ui::window::Model>,
    settings_state: dampen_core::AppState<ui::settings::Model>,
    
    #[cfg(debug_assertions)]
    error_overlay: Option<dampen_dev::ErrorOverlay>,
}
```

**Contract**:
- `current_view: CurrentView` field (always present)
- One `{view_name}_state: AppState<ui::{module_path}::Model>` field per discovered view
- `error_overlay: Option<ErrorOverlay>` field (only if `dismiss_error_variant` specified, debug builds only)
- Visibility: Same as annotated struct
- Generics: Preserved from original struct (if any)

### 3. Initialization Methods

#### 3.1 `init()` Method

```rust
impl MyApp {
    pub fn init() -> (Self, iced::Task<Message>) {
        let window_state = {
            let document = ui::window::_window::document();
            let handlers = ui::window::create_handlers();
            dampen_core::AppState::with_handlers(document, handlers)
        };
        
        let settings_state = {
            let document = ui::settings::_settings::document();
            let handlers = ui::settings::create_handlers();
            dampen_core::AppState::with_handlers(document, handlers)
        };
        
        let app = Self {
            current_view: CurrentView::default(),
            window_state,
            settings_state,
            #[cfg(debug_assertions)]
            error_overlay: None,
        };
        
        (app, iced::Task::none())
    }
}
```

**Contract**:
- Initializes all `AppState` fields by calling `ui::{view_name}::_view_name::document()` (from `#[dampen_ui]` macro)
- Calls `ui::{view_name}::create_handlers()` if it exists (graceful fallback to empty registry)
- Sets `current_view` to `CurrentView::default()`
- Returns `(Self, Task<Message>)` tuple
- Visibility: `pub`

#### 3.2 `new()` Method (Alias)

```rust
impl MyApp {
    pub fn new() -> (Self, iced::Task<Message>) {
        Self::init()
    }
}
```

**Contract**:
- Simple alias to `init()` for Iced convention
- Visibility: `pub`

### 4. Update Method

```rust
impl MyApp {
    pub fn update(&mut self, message: Message) -> iced::Task<Message> {
        match message {
            Message::Handler(handler_msg) => {
                match self.current_view {
                    CurrentView::Window => {
                        self.window_state.dispatch_handler(
                            &handler_msg.handler_name,
                            handler_msg.value,
                        );
                    }
                    CurrentView::Settings => {
                        self.settings_state.dispatch_handler(
                            &handler_msg.handler_name,
                            handler_msg.value,
                        );
                    }
                }
                iced::Task::none()
            }
            
            #[cfg(debug_assertions)]
            Message::HotReload(event) => {
                use dampen_dev::HotReloadExt;
                match event.file_path.to_str() {
                    Some(path) if path.ends_with("window.dampen") => {
                        match self.window_state.reload() {
                            Ok(_) => self.error_overlay = None,
                            Err(e) => {
                                self.error_overlay = Some(dampen_dev::ErrorOverlay::new(
                                    path.to_string(),
                                    e.to_string(),
                                ));
                            }
                        }
                    }
                    Some(path) if path.ends_with("settings.dampen") => {
                        match self.settings_state.reload() {
                            Ok(_) => self.error_overlay = None,
                            Err(e) => {
                                self.error_overlay = Some(dampen_dev::ErrorOverlay::new(
                                    path.to_string(),
                                    e.to_string(),
                                ));
                            }
                        }
                    }
                    _ => {}
                }
                iced::Task::none()
            }
            
            #[cfg(debug_assertions)]
            Message::DismissError => {
                self.error_overlay = None;
                iced::Task::none()
            }
            
            _ => iced::Task::none(),
        }
    }
}
```

**Contract**:
- Routes `Message::Handler` to current view's `AppState::dispatch_handler()`
- Routes `Message::HotReload` to view-specific reload logic (debug builds only)
- Routes `Message::DismissError` to clear error overlay (debug builds only)
- Returns `Task<Message>`
- Visibility: `pub`
- Hot-reload logic only generated if `hot_reload_variant` specified
- Error dismissal logic only generated if `dismiss_error_variant` specified

### 5. View Method

```rust
impl MyApp {
    pub fn view(&self) -> iced::Element<'_, Message> {
        #[cfg(debug_assertions)]
        if let Some(ref overlay) = self.error_overlay {
            return overlay.view().map(|_| Message::DismissError);
        }
        
        match self.current_view {
            CurrentView::Window => {
                dampen_iced::build_ui(&self.window_state, |handler_msg| {
                    Message::Handler(handler_msg)
                })
            }
            CurrentView::Settings => {
                dampen_iced::build_ui(&self.settings_state, |handler_msg| {
                    Message::Handler(handler_msg)
                })
            }
        }
    }
}
```

**Contract**:
- Renders error overlay if present (debug builds, when `dismiss_error_variant` specified)
- Otherwise, matches on `current_view` and calls `dampen_iced::build_ui()` for current view
- Maps `HandlerMessage` to user's `Message::Handler` variant
- Returns `Element<'_, Message>`
- Visibility: `pub`

### 6. Subscription Method

```rust
#[cfg(debug_assertions)]
impl MyApp {
    pub fn subscription(&self) -> iced::Subscription<Message> {
        use dampen_dev::HotReloadExt;
        
        iced::Subscription::batch(vec![
            self.window_state.watch().map(Message::HotReload),
            self.settings_state.watch().map(Message::HotReload),
        ])
    }
}
```

**Contract**:
- ONLY generated if `hot_reload_variant` is specified
- ONLY compiled in debug builds (`#[cfg(debug_assertions)]`)
- Batches subscriptions from all views' `AppState::watch()`
- Maps `FileEvent` to user's `Message::HotReload` variant
- Returns `Subscription<Message>`
- Visibility: `pub`

### 7. Helper Methods (View Switching)

```rust
impl MyApp {
    pub fn switch_to_window(&mut self) {
        self.current_view = CurrentView::Window;
    }
    
    pub fn switch_to_settings(&mut self) {
        self.current_view = CurrentView::Settings;
    }
}
```

**Contract**:
- One `switch_to_{view_name}()` method per discovered view
- Sets `current_view` to corresponding variant
- Visibility: `pub`
- Can be called from handler functions

---

## Error Cases

### E1: Missing Required Attribute

**Trigger**: Omit `ui_dir`, `message_type`, or `handler_variant`

**Example**:
```rust
#[dampen_app(ui_dir = "src/ui")]
struct MyApp;
```

**Expected Error**:
```
error: missing required attribute 'message_type'
  --> src/main.rs:10:1
   |
10 | #[dampen_app(ui_dir = "src/ui")]
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = help: Add message_type = "Message" to the macro attributes
```

**Contract**:
- Error message MUST include missing attribute name
- MUST include suggestion with example value
- Error span MUST cover the entire attribute

### E2: Invalid UI Directory

**Trigger**: Specify `ui_dir` that doesn't exist

**Example**:
```rust
#[dampen_app(
    ui_dir = "src/nonexistent",
    message_type = "Message",
    handler_variant = "Handler"
)]
struct MyApp;
```

**Expected Error**:
```
error: UI directory not found: '/absolute/path/to/project/src/nonexistent'
  --> src/main.rs:10:14
   |
10 |     ui_dir = "src/nonexistent",
   |              ^^^^^^^^^^^^^^^^^
   |
   = help: Ensure the directory exists relative to Cargo.toml
```

**Contract**:
- Error message MUST include absolute path attempted
- MUST suggest checking path relative to crate root
- Error span MUST cover the string literal value

### E3: Missing .rs File

**Trigger**: Create `.dampen` file without corresponding `.rs` file

**Directory Structure**:
```
src/ui/
├── window.dampen    ✅
└── window.rs        ❌ MISSING
```

**Expected Error**:
```
error: No matching Rust module found for '/absolute/path/to/project/src/ui/window.dampen'
  --> src/main.rs:10:1
   |
10 | #[dampen_app(ui_dir = "src/ui", ...)]
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = help: Create a file at '/absolute/path/to/project/src/ui/window.rs' with a Model struct, or exclude this view with exclude = ["window"]
```

**Contract**:
- Error message MUST include path to `.dampen` file
- MUST suggest creating `.rs` file at specific path
- MUST suggest exclusion as alternative
- Error span MUST cover the entire attribute

### E4: View Naming Conflict

**Trigger**: Two `.dampen` files with same name in different directories

**Directory Structure**:
```
src/ui/
├── form/
│   └── input.dampen
└── dialog/
    └── input.dampen
```

**Expected Error**:
```
error: View naming conflict: 'Input' variant found in multiple locations:
         - /absolute/path/to/project/src/ui/form/input.dampen
         - /absolute/path/to/project/src/ui/dialog/input.dampen
  --> src/main.rs:10:1
   |
10 | #[dampen_app(ui_dir = "src/ui", ...)]
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = help: Rename one of the files to avoid conflicts, or exclude one with exclude = ["form/input"]
```

**Contract**:
- Error message MUST list ALL conflicting files
- MUST show the conflicting variant name
- MUST suggest renaming or exclusion
- Error span MUST cover the entire attribute

### E5: Invalid View Name

**Trigger**: `.dampen` file with invalid Rust identifier as name

**Example**: `123-invalid.dampen`, `my-view.dampen` (hyphens invalid)

**Expected Error**:
```
error: Invalid view name '123-invalid' in '/absolute/path/to/project/src/ui/123-invalid.dampen'
  --> src/main.rs:10:1
   |
10 | #[dampen_app(ui_dir = "src/ui", ...)]
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = help: View names must be valid Rust identifiers (alphanumeric and underscores only, start with letter)
```

**Contract**:
- Error message MUST show invalid name
- MUST include full file path
- MUST explain identifier rules
- Error span MUST cover the entire attribute

### E6: Invalid Glob Pattern

**Trigger**: Malformed glob pattern in `exclude`

**Example**:
```rust
#[dampen_app(
    ui_dir = "src/ui",
    message_type = "Message",
    handler_variant = "Handler",
    exclude = ["debug["]  // Unclosed bracket
)]
struct MyApp;
```

**Expected Error**:
```
error: Invalid glob pattern in exclude: 'debug['
  --> src/main.rs:14:16
   |
14 |     exclude = ["debug["]
   |                ^^^^^^^^
   |
   = help: Ensure patterns use valid glob syntax (*, ?, [abc], etc.)
         Pattern error: unclosed character class
```

**Contract**:
- Error message MUST show invalid pattern
- MUST include details from glob parsing error
- MUST suggest valid glob syntax
- Error span MUST cover the string literal

### E7: No Views Discovered (Warning)

**Trigger**: `ui_dir` exists but contains no `.dampen` files

**Expected Warning**:
```
warning: No .dampen files found in '/absolute/path/to/project/src/ui'
  --> src/main.rs:10:1
   |
10 | #[dampen_app(ui_dir = "src/ui", ...)]
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: The generated code will have an empty CurrentView enum
   = help: Add .dampen files to the directory, or check the ui_dir path
```

**Contract**:
- MUST be a warning (not an error - allow empty state for gradual adoption)
- MUST include absolute path searched
- MUST explain consequence (empty enum)
- Generated code MUST compile successfully

---

## Macro Invocation Location

The macro can be applied to a struct in:
- `src/main.rs` (typical for small apps)
- `src/app.rs` (typical for larger apps with module structure)
- Any module file (for library crates exposing UI components)

**Constraint**: The struct MUST be visible to the locations where `AppState` needs to be accessed.

---

## Dependencies Required

Users MUST have the following in their `Cargo.toml`:

```toml
[dependencies]
dampen-core = "0.1"      # For AppState
dampen-iced = "0.1"      # For build_ui
dampen-macros = "0.1"    # For #[dampen_app], #[dampen_ui], #[derive(UiModel)]
iced = "0.14"            # For Element, Task, Subscription

[dependencies.dampen-dev]
version = "0.1"
optional = true          # Only for debug builds

[features]
default = ["dev"]
dev = ["dampen-dev"]
```

**Contract**:
- Macro expansion assumes all dependencies are available
- No compile-time feature detection (user responsible for correct setup)
- Hot-reload code gated by `#[cfg(debug_assertions)]` (built-in Rust feature)

---

## Interaction with Other Macros

### With `#[dampen_ui]`

The generated code expects each view's `.rs` file to use `#[dampen_ui]`:

```rust
// src/ui/window.rs
use dampen_macros::{dampen_ui, UiModel};

#[derive(UiModel)]
pub struct Model {
    pub title: String,
}

#[dampen_ui("window.dampen")]
mod _window {}

pub fn create_handlers() -> dampen_core::HandlerRegistry {
    // Optional: register handlers
    dampen_core::HandlerRegistry::new()
}
```

**Contract**:
- `#[dampen_ui]` MUST be present for `document()` function generation
- Module name convention: `mod _{view_name} {}`
- `create_handlers()` function is optional (graceful fallback)

### With `#[derive(UiModel)]`

The generated code expects `Model` structs to derive `UiModel`:

```rust
#[derive(UiModel)]
pub struct Model {
    pub count: i32,
}
```

**Contract**:
- `Model` MUST implement `UiBindable` (either via derive or manually)
- Required for binding evaluation in `AppState`

---

## Backwards Compatibility

### Migration from Manual Code

Existing multi-view applications can migrate incrementally:

1. **Step 1**: Add `#[dampen_app]` macro to app struct
2. **Step 2**: Remove manually-written fields (macro will regenerate them)
3. **Step 3**: Remove manual `init()`, `update()`, `view()`, `subscription()` methods
4. **Step 4**: Update handler registration if needed

**Contract**:
- Generated code MUST be functionally equivalent to manual code
- No behavioral changes (validated by integration tests)
- Performance MUST be identical (zero-cost abstraction)

### Future-Proofing

**Reserved Attribute Names** (for future features):
- `base_path` - Custom module base path
- `view_trait` - Custom trait for views
- `router` - Custom routing logic

**Contract**:
- Using reserved names emits a warning (not an error)
- Future versions MAY add functionality for these attributes

---

## Summary

This API contract defines:

1. **Macro Signature**: Required and optional attributes with type constraints
2. **Input Contract**: File system requirements and struct constraints
3. **Output Contract**: Generated code structure (enum, struct, methods)
4. **Error Cases**: 7 error scenarios with specific message formats
5. **Dependencies**: Required crates and features
6. **Interactions**: Integration with `#[dampen_ui]` and `#[derive(UiModel)]`
7. **Migration**: Path from manual to macro-generated code

This contract serves as the authoritative specification for:
- Implementation (code generation logic)
- Testing (trybuild compile-fail tests)
- Documentation (user-facing API docs)
- Validation (requirements checklist)
