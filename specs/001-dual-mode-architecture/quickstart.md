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

## Common Workflows for End Users

### Workflow 1: Creating a New Project

```bash
# Create project with dual-mode support
dampen new my-todo-app
cd my-todo-app

# Project structure created:
# - Cargo.toml (with dual-mode features)
# - build.rs (for codegen mode)
# - src/ui/window.dampen (UI definition)
# - src/ui/window.rs (model + handlers)

# Run in development mode (hot-reload enabled)
cargo run

# Edit UI and see changes live
# No restart needed!
```

### Workflow 2: Adding Hot-Reload to Existing Project

```bash
# 1. Update Cargo.toml
cat >> Cargo.toml << 'EOF'
[dependencies.dampen-dev]
version = "0.1"
optional = true

[features]
interpreted = ["dampen-dev"]
codegen = []

[profile.dev]
features = ["interpreted"]

[profile.release]
features = ["codegen"]
EOF

# 2. Add subscription in main.rs
# (See migration guide for full code)

# 3. Test hot-reload
cargo run
# Edit .dampen files and save to see instant updates
```

### Workflow 3: Development Iteration Cycle

```bash
# Day-to-day development with hot-reload:

# 1. Start app in interpreted mode
cargo run

# 2. Make UI changes
vim src/ui/window.dampen
# - Change text values
# - Adjust layouts
# - Add new widgets
# - Modify styling

# 3. Save file (Ctrl+S)
# App auto-reloads in <300ms

# 4. Test interactivity
# Click buttons, type in fields, etc.

# 5. If you see parse errors:
# - Check terminal for error message
# - Fix XML syntax
# - Save again to retry
```

### Workflow 4: Preparing for Production Release

```bash
# 1. Test in both modes for parity
cargo run                        # Interpreted
cargo run --release             # Codegen

# 2. Run validation
dampen check src/ui/*.dampen

# 3. Run tests
cargo test --workspace

# 4. Build release binary
cargo build --release

# 5. Binary is optimized:
# - No runtime XML parser
# - Static widget tree
# - Minimal size
# - Maximum performance

# 6. Deploy
./target/release/my-todo-app
```

### Workflow 5: Debugging Parse Errors

```bash
# Enable detailed logging
RUST_LOG=dampen_dev=debug cargo run

# You'll see:
# - File watch events
# - Parse attempts
# - Error details with line/column
# - Suggested fixes

# Example error output:
# ✗ Hot-reload failed: Parse error at line 15, column 8
#   Unexpected closing tag '</button>'
#   Suggestion: Check that all tags are properly balanced
```

### Workflow 6: Performance Profiling

```bash
# 1. Run benchmarks
cd benchmarks
cargo bench

# 2. Compare modes
cargo bench --features interpreted  # ~50ms init
cargo bench --features codegen      # <1ms init

# 3. Profile hot-reload
cargo run --features interpreted
# Edit file and watch terminal for:
# ✓ Hot-reload succeeded in 247ms
#   - Parse: 156ms
#   - Rebuild: 91ms

# 4. Check memory usage
cargo run --release
# Monitor with: htop or Activity Monitor
```

### Workflow 7: Switching Between Modes

```bash
# Force interpreted in release (for debugging)
cargo build --release --no-default-features --features interpreted

# Force codegen in dev (to test production build)
cargo build --no-default-features --features codegen

# Test mode parity
cargo test mode_parity

# Should output:
# test mode_parity_tests::test_parse_parity ... ok
# test mode_parity_tests::test_binding_parity ... ok
# test mode_parity_tests::test_handler_parity ... ok
```

### Workflow 8: Migrating to Dual-Mode

For existing Dampen projects:

```bash
# 1. Backup project
git commit -am "Before dual-mode migration"

# 2. Follow migration guide
cat docs/migration/dual-mode.md

# 3. Update dependencies (Cargo.toml)
# 4. Add build.rs
# 5. Replace manual parse calls with #[dampen_ui]
# 6. Add hot-reload subscription (optional)

# 7. Test both modes
cargo run                    # Interpreted
cargo build --release        # Codegen

# 8. Verify parity
cargo test

# 9. Commit changes
git commit -am "Migrate to dual-mode architecture"
```

### Workflow 9: Contributing to Dampen

```bash
# 1. Clone repository
git clone https://github.com/dampen/dampen.git
cd dampen

# 2. Create feature branch
git checkout -b fix/hot-reload-issue

# 3. Make changes
vim crates/dampen-dev/src/reload.rs

# 4. Run tests
cargo test --workspace
cargo clippy --workspace

# 5. Run benchmarks
cd benchmarks
cargo bench

# 6. Test examples
cd examples/counter
cargo run                    # Test interpreted
cargo run --release          # Test codegen

# 7. Submit PR
git commit -am "Fix: Improve hot-reload error messages"
git push origin fix/hot-reload-issue
```

### Workflow 10: Inspecting Generated Code

```bash
# In codegen mode, view what Rust code is generated:

# 1. Build with codegen
cargo build --features codegen

# 2. Find generated files
ls target/debug/build/my-app-*/out/

# 3. Read generated code
cat target/debug/build/my-app-*/out/window.rs

# You'll see:
# - Static DampenDocument construction
# - Inline widget tree
# - No runtime parsing

# 4. Compare with source
diff src/ui/window.dampen target/.../window.rs
```

---

**Document Status**: ✅ Complete  
**Ready for Implementation**: Yes
