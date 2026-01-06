# API Contract: Auto-Loading and AppState

**Feature**: 006-auto-ui-loading
**Date**: 2026-01-06

## Macro API

### `#[gravity_ui]` Attribute

**Location**: `gravity-macros/src/ui_loader.rs`

**Usage**:
```rust
#[gravity_ui]
mod app_ui;

#[gravity_ui(path = "custom/path.gravity")]
mod custom_ui;
```

**Behavior**:
- `#[gravity_ui]` - Loads `<filename>.gravity` where `<filename>.gravity.rs` is the file containing the attribute
- `#[gravity_ui(path = "...")]` - Loads the specified `.gravity` file

**Output**:
The macro generates a module with:
- `pub static document: GravityDocument` - The parsed UI document

### Generated Module Structure

```rust
// Generated code in ui/app.gravity.rs
mod app_ui {
    include!("../ui/app.gravity");
}
```

## File Loading Convention

### Standard Convention

| File | Loads |
|------|-------|
| `ui/app.gravity.rs` | `ui/app.gravity` |
| `ui/settings.gravity.rs` | `ui/settings.gravity` |
| `ui/views/about.gravity.rs` | `ui/views/about.gravity` |

### Custom Path

```rust
#[gravity_ui(path = "shared/header.gravity")]
mod header_ui;
```

## Error Codes

| Code | Condition | Severity | Message | Help Message |
|------|-----------|----------|---------|--------------|
| G001 | File not found | Compile Error | "Gravity UI file not found: '{path}'" | "Check the path is correct relative to CARGO_MANIFEST_DIR" |
| G002 | Invalid XML | Compile Error | "Invalid XML in Gravity UI file '{path}': {error}" | "Check for malformed tags or unclosed elements" |
| G003 | Unknown handler | Warning/Compile | "Handler '{name}' not registered" | "Add `#[ui_handler]` to a function or check for typos" |
| G004 | Parse error | Compile Error | "Gravity parsing error: {message}" | See span location for details |
| G005 | Schema version mismatch | Compile Error | "Schema version {found} not supported (expected {expected})" | "Update the .gravity file or use a compatible version" |

## Error Handling Flow

```text
Compile-Time                    Runtime
    │                              │
    ├─ Check .gravity file exists ─┤
    │          │                   │
    │          ▼                   │
    ├─ Parse XML                   │
    │          │                   │
    │          ▼                   │
    ├─ Validate handlers ──────────┤
    │          │                   │
    │          ▼                   │
    ├─ Generate AppState ◄─────────┤
    │                              │
    └──────────────────────────────┘
                  │
                  ▼
          ┌──────────────────┐
          │ Valid Application│
          └──────────────────┘
```

## AppState API Contract

### Constructor Signatures

```rust
impl<M: UiBindable> AppState<M> {
    pub fn new(document: GravityDocument) -> Self
    where
        M: Default;

    pub fn with_model(document: GravityDocument, model: M) -> Self;

    pub fn with_handlers(document: GravityDocument, handler_registry: HandlerRegistry) -> Self
    where
        M: Default;
}
```

### Field Access

```rust
pub struct AppState<M: UiBindable = ()> {
    pub document: GravityDocument,
    pub model: M,
    pub handler_registry: HandlerRegistry,
}
```

### Compatibility Requirements

1. **GravityWidgetBuilder**: `AppState` must work with:
   ```rust
   GravityWidgetBuilder::new(
       &state.document,
       &state.model,
       Some(&state.handler_registry),
   )
   ```

2. **UiBindable**: Model type `M` must implement:
   ```rust
   pub trait UiBindable {
       fn get_field(&self, path: &[&str]) -> Option<BindingValue>;
       fn available_fields() -> Vec<String>;
   }
   ```

3. **HandlerRegistry**: Must be the standard registry type from `gravity_core::HandlerRegistry`

## Directory Structure Contract

### Valid Project Structure

```
project/
├── Cargo.toml
├── src/
│   └── main.rs
└── ui/
    ├── mod.rs
    ├── app.gravity.rs      # Loads app.gravity
    ├── app.gravity         # UI definition
    ├── settings.gravity.rs # Loads settings.gravity
    └── settings.gravity    # UI definition
```

### Module Export (ui/mod.rs)

```rust
pub mod app_gravity;
pub use app_gravity::create_app_state as create_default_app_state;

pub mod settings_gravity;
pub use settings_gravity::create_app_state as create_settings_state;
```

### Main.rs Usage

```rust
use crate::ui::create_default_app_state;

fn main() -> iced::Result {
    let app_state = create_default_app_state();
    iced::application(app_state, update, view).run()
}
```

## Version Compatibility

| Feature | Minimum Version | Notes |
|---------|-----------------|-------|
| AppState struct | 0.1.0 | New in this feature |
| #[gravity_ui] macro | 0.1.0 | New in this feature |
| Generic AppState<M> | 0.1.0 | Uses PhantomData |

## Backward Compatibility

- Existing `include_str!` patterns continue to work
- `GravityDocument::parse()` unchanged
- `HandlerRegistry` API unchanged
- No breaking changes to existing types
