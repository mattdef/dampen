# Dual-Mode Architecture Migration Guide

This guide helps you migrate existing Dampen projects to the new dual-mode architecture, which supports both **interpreted mode** (hot-reload for development) and **codegen mode** (zero-overhead for production).

## Overview

The dual-mode architecture introduces:

- **Interpreted Mode**: Runtime XML parsing with hot-reload support for rapid development
- **Codegen Mode**: Build-time code generation for zero runtime overhead in production
- **Automatic Mode Selection**: Development builds use interpreted mode, release builds use codegen mode

## Quick Migration (New Projects)

If you're starting a new project, use the CLI:

```bash
dampen new my-app
cd my-app
cargo run  # Automatically uses interpreted mode in dev
cargo build --release  # Automatically uses codegen mode
```

The generated project is already configured for dual-mode.

## Migrating Existing Projects

### Step 1: Update Cargo.toml

Add the dual-mode feature configuration to your `Cargo.toml`:

```toml
[dependencies]
dampen-core = { version = "0.1", features = [] }
dampen-macros = { version = "0.1" }
dampen-iced = { version = "0.1" }

# For interpreted mode hot-reload support
[dependencies.dampen-dev]
version = "0.1"
optional = true

[features]
default = []
interpreted = ["dampen-dev", "dampen-core/interpreted"]
codegen = ["dampen-core/codegen"]

# Automatic mode selection based on build profile
[profile.dev]
features = ["interpreted"]

[profile.release]
features = ["codegen"]
```

### Step 2: Add build.rs for Codegen Mode

Create `build.rs` in your project root:

```rust
//! Build script for codegen mode
//!
//! This script generates Rust code from .dampen XML files at compile time.

fn main() {
    #[cfg(feature = "codegen")]
    {
        println!("cargo:rerun-if-changed=src/ui");
        
        // Add code generation logic here when implementing codegen mode
        // For now, this is a placeholder for future implementation
    }
}
```

### Step 3: Update Your UI Module

#### Before (Manual XML Loading)

```rust
// src/ui/window.rs
use dampen_core::parser;

pub fn create_app_state() -> dampen_core::AppState<Model> {
    let xml = include_str!("window.dampen");
    let document = parser::parse(xml).expect("Failed to parse XML");
    dampen_core::AppState::with_handlers(document, create_handlers())
}
```

#### After (Auto-Loading with Dual-Mode)

```rust
// src/ui/window.rs
use dampen_macros::{dampen_ui, UiModel};

#[derive(UiModel)]
pub struct Model {
    // Your model fields
}

#[dampen_ui("window.dampen")]
mod _window {}

pub fn create_app_state() -> dampen_core::AppState<Model> {
    let document = _window::document();
    dampen_core::AppState::with_handlers(document, create_handlers())
}
```

### Step 4: Enable Hot-Reload (Interpreted Mode Only)

In your `main.rs`, add hot-reload support:

```rust
use iced::{Element, Task, Subscription};

fn subscription(app: &App) -> Subscription<Message> {
    #[cfg(feature = "interpreted")]
    {
        use dampen_dev::watch_files;
        use std::path::PathBuf;
        
        watch_files(vec![PathBuf::from("src/ui/window.dampen")], "xml")
            .map(|_| Message::ReloadUI)
    }
    
    #[cfg(not(feature = "interpreted"))]
    {
        Subscription::none()
    }
}

fn update(app: &mut App, message: Message) -> Task<Message> {
    match message {
        #[cfg(feature = "interpreted")]
        Message::ReloadUI => {
            use dampen_dev::attempt_hot_reload;
            
            let xml = std::fs::read_to_string("src/ui/window.dampen")
                .unwrap_or_default();
            
            match attempt_hot_reload(&xml, &app.state, &mut app.reload_context, || create_handlers()) {
                dampen_dev::ReloadResult::Success(new_state) => {
                    app.state = new_state;
                    println!("✓ Hot-reload succeeded");
                }
                dampen_dev::ReloadResult::ParseError(e) => {
                    eprintln!("✗ Hot-reload failed: {}", e);
                }
                _ => {}
            }
            Task::none()
        }
        // ... other messages
    }
}
```

### Step 5: Add Reload Context (For Hot-Reload)

Update your app struct:

```rust
pub struct App {
    state: dampen_core::AppState<Model>,
    
    #[cfg(feature = "interpreted")]
    reload_context: dampen_dev::HotReloadContext<Model>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            state: create_app_state(),
            
            #[cfg(feature = "interpreted")]
            reload_context: dampen_dev::HotReloadContext::new(),
        }
    }
}
```

## Testing Both Modes

### Test Interpreted Mode (Development)

```bash
cargo run
# or explicitly:
cargo run --features interpreted
```

**Expected behavior:**
- Fast compile times
- Hot-reload on XML file changes
- Runtime XML parsing

### Test Codegen Mode (Production)

```bash
cargo build --release
cargo run --release
# or explicitly:
cargo build --features codegen
```

**Expected behavior:**
- Longer initial compile time (code generation)
- Zero runtime XML parsing
- Optimal performance

## Mode Parity Verification

Both modes should produce identical UI behavior. Test with:

```bash
# Run mode parity tests
cd tests/integration
cargo test mode_parity
```

## Common Issues

### Issue: Hot-reload not working

**Solution:** Ensure you have:
1. `interpreted` feature enabled
2. File watcher subscription active
3. Reload context in your app struct
4. Message handler for `ReloadUI`

### Issue: Codegen build fails

**Solution:** Verify:
1. `build.rs` is present and configured
2. `codegen` feature is enabled in release profile
3. All `.dampen` files are valid XML

### Issue: Mode-specific code not compiling

**Solution:** Use feature gates:

```rust
#[cfg(feature = "interpreted")]
use dampen_dev::*;

#[cfg(feature = "codegen")]
const GENERATED: bool = true;
```

## Performance Comparison

| Metric | Interpreted Mode | Codegen Mode |
|--------|------------------|--------------|
| Initial compile time | ~5s | ~8s (includes codegen) |
| Hot-reload latency | <300ms | N/A |
| Runtime XML parsing | Yes | No |
| Binary size | Smaller | Slightly larger |
| Production performance | Good | Optimal |

## Rollback Plan

If you need to rollback to manual XML loading:

1. Remove `build.rs`
2. Remove feature configuration from `Cargo.toml`
3. Replace `#[dampen_ui]` with manual `parser::parse()` calls
4. Remove hot-reload code

Your `.dampen` XML files remain unchanged and compatible.

## Next Steps

- Read [Developer Guide](../development/dual-mode.md) for implementation details
- Check [Performance Guide](../performance.md) for optimization tips
- See `examples/` directory for reference implementations

## Getting Help

If you encounter issues during migration:

1. Check the [FAQ](../FAQ.md)
2. Review example projects in `examples/`
3. Open an issue on GitHub with your `Cargo.toml` and error output
