# Gravity Examples

This directory contains progressive examples demonstrating the Gravity UI framework.

## Examples

### 1. Hello World (`hello-world/`)
**Phase**: MVP (Phase 3)  
**Concepts**: Static XML, basic widgets, rendering

Minimal example showing how to define a UI in XML and render it through Iced.

**Run**: `cargo run -p hello-world`

---

### 2. Counter (`counter/`)
**Phase**: Interactive (Phase 4)  
**Concepts**: Event handlers, state changes

Interactive counter demonstrating button clicks and state updates.

**Run**: `cargo run -p counter`

---

### 3. Todo App (`todo-app/`)
**Phase**: Bindings (Phase 5)  
**Concepts**: Data binding, lists, form inputs

Full CRUD application showing model bindings and complex state.

**Run**: `cargo run -p todo-app`

---

### 4. Full Demo (`full-demo/`)
**Phase**: Complete (Phase 9+)  
**Concepts**: All features combined

Comprehensive showcase of all framework capabilities.

**Run**: `cargo run -p full-demo`

## Running Examples

From the workspace root:

```bash
# Run specific example
cargo run -p hello-world

# Run in dev mode (if supported)
cargo run -p counter --features dev

# Build all examples
cargo build --workspace --examples
```

## Example Structure

Each example follows this pattern:

```
example-name/
├── Cargo.toml          # Example-specific dependencies
├── ui/
│   └── main.gravity    # XML UI definition
└── src/
    └── main.rs         # Model, handlers, and main function
```

## Learning Path

1. Start with `hello-world` to understand basic structure
2. Move to `counter` for event handling
3. Try `todo-app` for data binding
4. Explore `full-demo` for advanced features
