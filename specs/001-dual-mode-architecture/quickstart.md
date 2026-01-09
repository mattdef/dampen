# Quickstart: Dual-Mode Architecture

**Feature**: 001-dual-mode-architecture  
**Audience**: Dampen developers implementing this feature  
**Last Updated**: 2026-01-09

## Overview

This guide provides a quick-start reference for implementing and using Dampen's dual-mode architecture:
- **Interpreted Mode**: Development with hot-reload (<300ms)
- **Codegen Mode**: Production with zero runtime overhead

## For Implementers

### Development Setup

```bash
# Clone and build
git checkout 001-dual-mode-architecture
cargo build --workspace

# Run tests
cargo test --workspace

# Run specific crate tests
cargo test -p dampen-dev      # Hot-reload functionality
cargo test -p dampen-core      # Code generation
```

### Phase 1: File Watching (Week 1-2)

**Create `dampen-dev` crate**:
```bash
cargo new --lib crates/dampen-dev
```

**Add dependencies** (`crates/dampen-dev/Cargo.toml`):
```toml
[dependencies]
dampen-core = { path = "../dampen-core" }
iced = { workspace = true }
notify = "6.1"
notify-debouncer-full = "0.3"
crossbeam-channel = "0.5"
futures = "0.3"
serde_json = "1.0"
```

**Key files to implement**:
1. `crates/dampen-dev/src/watcher.rs` - File watcher with notify
2. `crates/dampen-dev/src/subscription.rs` - Iced subscription
3. `crates/dampen-dev/src/reload.rs` - Hot-reload coordination
4. `crates/dampen-dev/src/overlay.rs` - Error overlay UI

**Testing hot-reload**:
```bash
# Terminal 1: Run example in interpreted mode
cd examples/counter
cargo run --no-default-features --features interpreted

# Terminal 2: Edit UI file
vim src/ui/app.dampen  # Make changes and save

# Observe: UI updates in <300ms without restart
```

### Phase 2: Full Codegen (Week 3-5)

**Enhance code generation** (`crates/dampen-core/src/codegen/bindings.rs` - NEW):

```rust
// Target: Generate this
self.count.to_string()

// Instead of this (current)
self.count.to_binding_value().to_display_string()
```

**Key files to modify**:
1. `crates/dampen-core/src/codegen/view.rs` - Widget code generation
2. `crates/dampen-core/src/codegen/bindings.rs` (NEW) - Expression inlining
3. `crates/dampen-core/src/codegen/handlers.rs` (NEW) - Handler dispatch

**Testing codegen**:
```bash
# Build in codegen mode
cd examples/counter
cargo build --release --features codegen

# Verify no runtime dependencies
nm target/release/counter | grep -i binding  # Should be empty

# Benchmark
cargo bench --features codegen
```

### Phase 3: CLI Integration (Week 6)

**Add CLI commands**:
1. `crates/dampen-cli/src/commands/run.rs` (NEW)
2. `crates/dampen-cli/src/commands/build.rs` (NEW)

**Testing CLI**:
```bash
# Development mode
dampen run examples/counter

# Production build
dampen build examples/counter --release

# Verify mode selection
cat examples/counter/Cargo.toml  # Check features
```

---

## For Users

### Using Interpreted Mode (Development)

**1. Enable hot-reload in Cargo.toml**:
```toml
[features]
default = ["interpreted"]
interpreted = ["dampen-dev"]

[dependencies]
dampen-dev = { version = "0.2", optional = true }
```

**2. Add file watcher subscription** (`src/main.rs`):
```rust
#[cfg(feature = "interpreted")]
fn subscription(_app: &App) -> Subscription<Message> {
    dampen_dev::watch_files(
        vec!["src/ui/app.dampen"],
        100  // debounce ms
    )
    .map(Message::HotReload)
}

#[cfg(feature = "codegen")]
fn subscription(_app: &App) -> Subscription<Message> {
    Subscription::none()
}
```

**3. Handle reload message**:
```rust
fn update(app: &mut App, message: Message) -> Command<Message> {
    match message {
        #[cfg(feature = "interpreted")]
        Message::HotReload(event) => {
            match event {
                FileEvent::Success { document, .. } => {
                    app.state.hot_reload(document);
                }
                FileEvent::ParseError { error, .. } => {
                    app.show_error(error);
                }
                _ => {}
            }
            Command::none()
        }
        // ... other messages
    }
}
```

**4. Run in development mode**:
```bash
cargo run  # Uses default features = ["interpreted"]

# Or explicitly
dampen run
```

### Using Codegen Mode (Production)

**1. Create build.rs**:
```rust
fn main() {
    #[cfg(feature = "codegen")]
    {
        use dampen_core::codegen;
        
        // Parse UI files
        let doc = dampen_core::parse_file("src/ui/app.dampen").unwrap();
        
        // Generate code
        let code = codegen::generate_application(
            &doc,
            "Model",
            "Message",
            &handlers,
        ).unwrap();
        
        // Write to OUT_DIR
        let out_dir = std::env::var("OUT_DIR").unwrap();
        std::fs::write(
            format!("{}/ui_app.rs", out_dir),
            code.code,
        ).unwrap();
        
        println!("cargo:rerun-if-changed=src/ui/app.dampen");
    }
}
```

**2. Include generated code** (`src/ui/mod.rs`):
```rust
#[cfg(feature = "codegen")]
include!(concat!(env!("OUT_DIR"), "/ui_app.rs"));

#[cfg(feature = "interpreted")]
pub use interpreted::*;
```

**3. Build for production**:
```bash
cargo build --release --features codegen

# Or use CLI
dampen build --release
```

---

## Common Workflows

### Workflow 1: Rapid UI Iteration

```bash
# Terminal 1: Run with hot-reload
cargo run

# Terminal 2: Edit UI
vim src/ui/app.dampen
# Save file → UI updates automatically in <300ms
```

### Workflow 2: Testing Both Modes

```bash
# Test interpreted mode
cargo test --features interpreted

# Test codegen mode
cargo test --features codegen

# Ensure parity
cargo run --example mode-parity-test
```

### Workflow 3: Production Deployment

```bash
# Build optimized binary
cargo build --release --features codegen

# Verify performance
cargo bench --features codegen

# Deploy
./target/release/my-app
```

---

## Troubleshooting

### Hot-Reload Not Working

**Symptoms**: File changes don't trigger UI updates

**Solutions**:
1. Check feature flag: `cargo run --features interpreted`
2. Verify subscription is active: Add debug print in `subscription()`
3. Check file watcher: Look for permission errors in console
4. Verify file path: Must match watched paths exactly

### Parse Errors Not Showing

**Symptoms**: Invalid XML crashes app instead of showing error overlay

**Solutions**:
1. Check error handling in `update()` function
2. Verify `ErrorOverlay` component is rendered
3. Check `FileEvent::ParseError` variant is handled
4. Enable error logging: `RUST_LOG=dampen_dev=debug cargo run`

### Codegen Mode Build Failures

**Symptoms**: `cargo build --features codegen` fails with syntax errors

**Solutions**:
1. Check generated code: `cat $OUT_DIR/ui_app.rs`
2. Verify XML is valid: `dampen check src/ui/app.dampen`
3. Run codegen tests: `cargo test -p dampen-core --test codegen_tests`
4. Check build.rs output: `cargo build --features codegen --verbose`

### Performance Issues

**Symptoms**: Hot-reload takes >300ms

**Solutions**:
1. Check debounce settings (should be 50-100ms)
2. Profile parse time: Add timing logs in `parse()`
3. Verify file size (<1000 widgets recommended)
4. Check for slow disk I/O (network drives, encryption)

---

## Performance Targets

| Metric | Target | Measurement |
|--------|--------|-------------|
| File change detection | <100ms | `notify` latency |
| XML parsing | <10ms | `parse()` duration |
| Model serialization | <5ms | `serde_json::to_string()` |
| Hot-reload total | <300ms | End-to-end latency |
| Production frame rendering | Within 5% of baseline | Benchmark comparison |

---

## Example Code Snippets

### Complete Hot-Reload Setup

```rust
// Cargo.toml
[features]
default = ["interpreted"]
codegen = []
interpreted = ["dampen-dev"]

// src/main.rs
use dampen_core::AppState;
#[cfg(feature = "interpreted")]
use dampen_dev::{FileEvent, watch_files};

struct App {
    state: AppState<Model>,
}

#[cfg(feature = "interpreted")]
fn subscription(_app: &App) -> Subscription<Message> {
    watch_files(vec!["src/ui/app.dampen"], 100)
        .map(Message::HotReload)
}

fn update(app: &mut App, msg: Message) -> Command<Message> {
    match msg {
        #[cfg(feature = "interpreted")]
        Message::HotReload(event) => {
            match event {
                FileEvent::Success { document, .. } => {
                    app.state.hot_reload(document);
                }
                FileEvent::ParseError { error, .. } => {
                    eprintln!("Parse error: {}", error);
                }
                _ => {}
            }
            Command::none()
        }
        // ... other messages
    }
}
```

### Complete Codegen Setup

```rust
// build.rs
fn main() {
    #[cfg(feature = "codegen")]
    {
        let doc = dampen_core::parse_file("src/ui/app.dampen").unwrap();
        let handlers = create_handler_metadata();
        let code = dampen_core::codegen::generate_application(
            &doc, "Model", "Message", &handlers
        ).unwrap();
        
        let out = std::env::var("OUT_DIR").unwrap();
        std::fs::write(format!("{}/ui.rs", out), code.code).unwrap();
        println!("cargo:rerun-if-changed=src/ui/app.dampen");
    }
}

// src/ui/mod.rs
#[cfg(feature = "codegen")]
include!(concat!(env!("OUT_DIR"), "/ui.rs"));
```

---

## Next Steps

After completing this feature:

1. **Update Examples**: Migrate all examples to dual-mode
2. **Write Migration Guide**: Help existing projects adopt dual-mode
3. **Performance Benchmarks**: Publish comparison data
4. **Documentation**: Update main README and developer guide

---

## References

- **Specification**: [spec.md](./spec.md)
- **Implementation Plan**: [plan.md](./plan.md)
- **Research**: [research.md](./research.md)
- **Data Model**: [data-model.md](./data-model.md)
- **Constitution**: `/.specify/memory/constitution.md` (Principle III)

---

**Document Status**: ✅ Complete  
**Ready for Implementation**: Yes
