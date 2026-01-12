# #[dampen_app] Macro - Usage Guide

**Status**: ✅ MVP Complete  
**Version**: 0.2.1  
**Last Updated**: 2026-01-12

## Overview

The `#[dampen_app]` procedural macro automatically generates multi-view application boilerplate for Dampen GUI applications. It eliminates manual routing, view management, and initialization code.

## Features

✅ **Automatic View Discovery** - Scans `ui_dir` for `.dampen` files  
✅ **Generated CurrentView Enum** - Type-safe view enumeration  
✅ **View Switching Helpers** - `switch_to_*()` methods for each view  
✅ **Automatic Document Loading** - Integrates with `#[dampen_ui]` macro  
✅ **Update/View Methods** - Routing logic generated automatically  
✅ **Compile-Time Validation** - Catches missing files, naming conflicts, etc.

## Quick Start

### 1. Project Structure

```
my-app/
├── Cargo.toml
├── src/
│   ├── main.rs
│   └── ui/
│       ├── mod.rs
│       ├── home.rs
│       ├── home.dampen
│       ├── settings.rs
│       └── settings.dampen
```

### 2. Define UI Modules

Each view needs:
- A `.dampen` XML file with UI definition
- A `.rs` file with Model struct and `#[dampen_ui]` macro

**src/ui/home.rs**:
```rust
use dampen_macros::{dampen_ui, UiModel};
use serde::{Deserialize, Serialize};

#[dampen_ui("home.dampen")]
pub mod _home {}

#[derive(Default, UiModel, Serialize, Deserialize, Clone, Debug)]
pub struct Model {
    pub title: String,
}
```

**src/ui/home.dampen**:
```xml
<dampen>
    <column padding="20" spacing="10">
        <text value="Home View" size="24" />
    </column>
</dampen>
```

**src/ui/mod.rs**:
```rust
pub mod home;
pub mod settings;
```

### 3. Use #[dampen_app] Macro

**src/main.rs**:
```rust
mod ui;

use dampen_macros::dampen_app;

// Define Message enum
#[derive(Clone, Debug)]
pub enum Message {
    Handler(()),  // Placeholder for handler messages
}

// Apply #[dampen_app] macro
#[dampen_app(
    ui_dir = "src/ui",
    message_type = "Message",
    handler_variant = "Handler"
)]
struct App;

fn main() {
    // Initialize app
    let app = App::init();
    
    // The generated App struct has:
    // - CurrentView enum
    // - home_state: AppState<ui::home::Model>
    // - settings_state: AppState<ui::settings::Model>
    // - current_view: CurrentView
    // - Methods: init(), new(), switch_to_*(), update(), view()
}
```

## Generated Code

Given views `home` and `settings`, the macro generates:

### CurrentView Enum
```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CurrentView {
    Home,
    Settings,
}
```

### App Struct
```rust
pub struct App {
    home_state: dampen_core::AppState<ui::home::Model>,
    settings_state: dampen_core::AppState<ui::settings::Model>,
    current_view: CurrentView,
}
```

### Methods
```rust
impl App {
    pub fn init() -> Self { /* ... */ }
    pub fn new() -> Self { Self::init() }
    
    pub fn switch_to_home(&mut self) { /* ... */ }
    pub fn switch_to_settings(&mut self) { /* ... */ }
    
    pub fn update(&mut self, message: Message) -> iced::Task<Message> { /* ... */ }
    pub fn view(&self) -> iced::Element<'_, Message> { /* ... */ }
}
```

## Macro Attributes

### Required

| Attribute | Type | Description | Example |
|-----------|------|-------------|---------|
| `ui_dir` | String | Directory containing `.dampen` files | `"src/ui"` |
| `message_type` | Ident | Name of Message enum | `"Message"` |
| `handler_variant` | Ident | Variant for handler dispatch | `"Handler"` |

### Optional

| Attribute | Type | Description | Example |
|-----------|------|-------------|---------|
| `hot_reload_variant` | Ident | Variant for hot-reload events | `"HotReload"` |
| `dismiss_error_variant` | Ident | Variant for error dismissal | `"DismissError"` |
| `exclude` | Array | Glob patterns to exclude | `["debug_*", "test/*"]` |

## Validation Rules

The macro performs compile-time validation:

**VR-001: Valid Rust Identifiers**  
View names must be valid Rust identifiers (alphanumeric + underscores, start with letter).

**VR-002: Unique Variant Names**  
PascalCase variant names must be unique across all views.

**VR-003: Corresponding .rs Files**  
Every `.dampen` file must have a matching `.rs` file with Model struct.

## Error Messages

### E1: Missing Required Attribute
```
error: missing required attribute 'ui_dir'
help: Add ui_dir = "src/ui" to the macro attributes
```

### E2: Invalid UI Directory
```
error: UI directory not found: 'src/nonexistent'
help: Ensure the directory exists relative to Cargo.toml
```

### E3: Missing .rs File
```
error: No matching Rust module found for '/path/to/view.dampen'
help: Create a file at '/path/to/view.rs' with a Model struct
```

### E4: View Naming Conflict
```
error: View naming conflict: 'Input' variant found in multiple locations:
       - /path/to/form/input.dampen
       - /path/to/dialog/input.dampen
help: Rename one file or use exclude pattern
```

### E5: Invalid View Name
```
error: Invalid view name '123-invalid' in '/path/to/123-invalid.dampen'
help: View names must be valid Rust identifiers
```

## Limitations (Current MVP)

⚠️ **Handler Dispatch**: Not yet implemented (requires `HandlerMessage` API)  
⚠️ **View Rendering**: Simplified (requires `dampen_iced::build_ui` API)  
⚠️ **Hot-Reload**: Not yet implemented (Phase 5)  
⚠️ **Error Overlay**: Not yet implemented (Phase 5)

## Next Steps

1. **Phase 5**: Hot-reload support (optional `hot_reload_variant`)
2. **Phase 6**: Error overlay integration (`dismiss_error_variant`)
3. **Phase 7**: Handler dispatch integration with existing API
4. **Phase 8**: Full view rendering with `dampen_iced::build_ui`

## Testing

Run tests:
```bash
cargo test -p dampen-macros --test dampen_app_tests
```

Test categories:
- **us1_discovery_tests**: View discovery and validation
- **us1_codegen_tests**: Code generation (snapshots)
- **us2_view_switching_tests**: View switching logic
- **error_cases**: Error handling

## Examples

See `/tmp/dampen_test_app` for a minimal working example.

## Contributing

See `specs/001-dampen-app-macro/` for detailed specification and contracts.

---

**Status**: MVP Complete ✅  
**Tests**: 17 passing, 3 ignored  
**Coverage**: Core functionality implemented
