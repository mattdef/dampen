# Gravity UI Framework

[![Crates.io](https://img.shields.io/crates/v/gravity-core.svg)](https://crates.io/crates/gravity-core)
[![Documentation](https://docs.rs/gravity-core/badge.svg)](https://docs.rs/gravity-core)
[![CI](https://github.com/your-org/gravity/actions/workflows/ci.yml/badge.svg)](https://github.com/your-org/gravity/actions)
[![License: MIT/Apache-2.0](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE-MIT)
[![Rust Version](https://img.shields.io/badge/rustc-1.75+-lightgray.svg)](https://rust-lang.org)

**A declarative UI framework for Rust with Iced backend, featuring hot-reload development and production code generation.**

Gravity allows you to define your UI in XML and render it through Iced, with support for:
- ✅ **Declarative XML UI definitions**
- ✅ **Hot-reload development mode** (<500ms updates)
- ✅ **Production code generation** (zero runtime overhead)
- ✅ **Type-safe event handlers and bindings**
- ✅ **Expression evaluation** in UI attributes
- ✅ **Multiple Iced widgets** (text, buttons, inputs, layouts, etc.)

## Quick Start

### Installation

Add Gravity to your `Cargo.toml`:

```toml
[dependencies]
gravity = "0.1"
gravity-iced = "0.1"

# For development mode with hot-reload
gravity-runtime = { version = "0.1", optional = true }

[features]
default = []
dev = ["gravity-runtime"]
```

### Project Structure

```
my-app/
├── Cargo.toml
├── src/
│   └── main.rs
└── ui/
    └── main.gravity    # Your UI definition
```

### Hello World in 5 Minutes

**1. Create `ui/main.gravity`:**

```xml
<column padding="40" spacing="20" align="center">
    <text value="Hello, Gravity!" size="32" weight="bold" />
    <text value="Welcome to declarative Rust UI" />
    <button label="Click me!" on_click="greet" />
</column>
```

**2. Create `src/main.rs`:**

```rust
use gravity::prelude::*;
use gravity_iced::IcedBackend;

#[derive(Default, UiModel)]
struct Model {
    greeting: String,
}

#[derive(Clone)]
enum Message {
    Greet,
}

#[ui_handler]
fn greet(model: &mut Model) {
    model.greeting = "Hello from Gravity!".to_string();
}

fn main() {
    gravity::run::<Model, Message, IcedBackend>(
        "ui/main.gravity",
        Model::default(),
    );
}
```

**3. Run:**

```bash
# Development mode with hot-reload
cargo run --features dev

# Production build
cargo run --release
```

## Features

### Declarative UI

Define your entire UI in XML:

```xml
<column padding="40" spacing="20">
    <text value="Counter: {count}" size="48" />
    <row spacing="20">
        <button label="-" on_click="decrement" enabled="{count > 0}" />
        <button label="+" on_click="increment" />
    </row>
    <button label="Reset" on_click="reset" />
</column>
```

### Type-Safe Handlers

```rust
#[ui_handler]
fn increment(model: &mut Model) {
    model.count += 1;
}

#[ui_handler]
fn update_name(model: &mut Model, value: String) {
    model.name = value;
}
```

### Data Binding

```rust
#[derive(Default, UiModel, Serialize, Deserialize)]
struct Model {
    count: i32,
    name: String,
    items: Vec<String>,
}
```

### Hot-Reload Development

```bash
gravity dev --ui ui --file main.gravity --verbose
```

- Edit `.gravity` files and see changes instantly
- State is preserved across reloads
- Errors appear as overlays in the UI

### Production Code Generation

```bash
gravity build --ui ui --output src/ui_generated.rs
```

Generates static Rust code with zero runtime overhead.

### Validation

```bash
gravity check --ui ui
```

Validates XML syntax, widget names, handlers, and bindings without running the app.

### Debugging

```bash
gravity inspect --file ui/main.gravity
gravity inspect --file ui/main.gravity --codegen --handlers increment,decrement
```

View IR tree or generated code for debugging.

## Architecture

### Core Principles

1. **Declarative-First**: XML is the source of truth for UI structure
2. **Type Safety**: No runtime type erasure for messages/state
3. **Dual-Mode**: Dev (hot-reload) + Prod (codegen)
4. **Backend Agnostic**: Core crate has no Iced dependency
5. **Test-First**: TDD for all features

### Project Structure

```text
crates/
├── gravity-core/      # Parser, IR, traits (backend-agnostic)
├── gravity-macros/    # #[derive(UiModel)], #[ui_handler]
├── gravity-runtime/   # Hot-reload interpreter, file watcher
├── gravity-iced/      # Iced backend implementation
└── gravity-cli/       # Developer CLI (dev, build, check, inspect)

examples/
├── hello-world/       # Minimal static example
├── counter/           # Interactive handlers
├── todo-app/          # Full bindings example
└── full-demo/         # Complete showcase (coming soon)
```

## Documentation

- **[Quick Start Guide](docs/QUICKSTART.md)** - Detailed getting started
- **[XML Schema Reference](docs/XML_SCHEMA.md)** - All widgets and attributes
- **[API Documentation](https://docs.rs/gravity-core)** - Full rustdoc
- **[Examples](examples/)** - Progressive example projects

## CLI Commands

```bash
# Development server with hot-reload
gravity dev --ui ui --file main.gravity

# Generate production code
gravity build --ui ui --output src/ui_generated.rs

# Validate without running
gravity check --ui ui

# Inspect IR or generated code
gravity inspect --file ui/main.gravity
gravity inspect --file ui/main.gravity --codegen
```

## Performance

- **XML Parse**: <10ms for 1000 widgets
- **Hot-Reload**: <500ms from save to UI update
- **Code Generation**: <5s for typical application
- **Runtime Memory**: <50MB baseline

## Requirements

- Rust 1.75 or later
- Edition 2021 (2024 when stable)
- No nightly features in public API

## Examples

See the [examples directory](examples/) for progressive demonstrations:

1. **hello-world** - Static UI rendering
2. **counter** - Interactive state management
3. **todo-app** - Full data binding with lists
4. **full-demo** - All features showcase (coming soon)

## Contributing

Contributions are welcome! Please read [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## License

This project is licensed under either of:

- Apache License, Version 2.0
- MIT license

at your option.

## Status

**Current Version**: 0.1.0 (Alpha)

**Completed Phases**:
- ✅ Phase 1-3: MVP (Declarative UI parsing and rendering)
- ✅ Phase 4-5: Interactive (Handlers and bindings)
- ✅ Phase 6: Hot-reload development
- ✅ Phase 7-8: Production mode (Codegen and validation)
- ✅ Phase 9: Complete widget support
- ✅ Phase 10: Debug tools (Inspect command)
- 🚧 Phase 11: Polish and release preparation

## Roadmap

- [ ] Full documentation site
- [ ] LSP server for IDE integration
- [ ] More advanced widgets (canvas, charts)
- [ ] Theming system
- [ ] Subscription support
- [ ] Crates.io publication

## Support

- **Issues**: [GitHub Issues](https://github.com/your-org/gravity/issues)
- **Discussions**: [GitHub Discussions](https://github.com/your-org/gravity/discussions)
- **Documentation**: [docs.rs](https://docs.rs/gravity-core)

---

**Built with ❤️ using Rust and Iced**
