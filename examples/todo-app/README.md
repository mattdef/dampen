# Todo App Example

A simple, elegant todo application demonstrating Dampen's declarative UI framework with dual-mode architecture (interpreted dev mode with hot-reload, codegen production mode).

This example matches the visual design of the Iced "todos" example while showcasing Dampen's unique features.

## Features

### Core Functionality

- **Task Management**: Create, edit, delete, and complete tasks
- **Inline Editing**: Edit task descriptions directly in the list
- **Task Filtering**: View All, Active, or Completed tasks
- **Keyboard Navigation**: Full Tab/Shift+Tab support (native Iced)
- **Hot-Reload**: Modify UI definition and see changes in <2 seconds (dev mode)
- **Production Mode**: Optimized codegen with zero runtime overhead

### Technical Highlights

- **Declarative UI**: All UI defined in `window.dampen` XML file
- **Type-Safe Bindings**: Model fields bound to UI with compile-time validation (codegen mode)
- **State Management**: Clean separation between model state and computed fields
- **Dual-Mode Support**: Same UI definition works in interpreted and codegen modes

## Quick Start

### Development Mode (with hot-reload)

```bash
cd examples/todo-app
dampen run
```

**What happens:**
- XML parsed at runtime for instant hot-reload
- Modify `src/ui/window.dampen` and see changes in <2s
- Fast iteration for UI development

### Production Mode (optimized)

```bash
cd examples/todo-app
dampen build --release
./target/release/todo-app
```

**What happens:**
- XML compiled to Rust code at build time
- Zero runtime parsing overhead
- Type-safe bindings validated at compile time
- Optimized for deployment

## Usage

Once running, you can:

1. **Create a task**: Type in the input field and press Enter
2. **Mark complete**: Click the checkbox next to a task
3. **Edit a task**: Click the ✏️ button, modify text, press Enter to save
4. **Delete a task**: Click the 🗑️ button
5. **Filter tasks**: Click "All", "Active", or "Completed" buttons
6. **Navigate with keyboard**: Use Tab/Shift+Tab to move between elements

## Data Model

### Task
```rust
pub struct Task {
    pub id: Uuid,
    pub description: String,
    pub completed: bool,
    pub state: TaskState,  // Idle | Editing
}
```

### Model (Application State)
```rust
pub struct Model {
    // User input
    pub input_value: String,
    pub filter: Filter,  // All | Active | Completed
    pub tasks: Vec<Task>,
    pub editing_id: Option<Uuid>,
    pub edit_text: String,

    // Computed fields (updated automatically)
    pub filtered_tasks: Vec<Task>,
    pub tasks_left: i64,
    pub tasks_left_text: String,
    pub empty_message: String,
    pub filtered_tasks_len: i64,
}
```

### Enums
- **Filter**: `All` | `Active` | `Completed`
- **TaskState**: `Idle` | `Editing`

## Hot-Reload Development Workflow

Dampen provides instant hot-reload for rapid UI iteration:

```bash
# Start the app in development mode
dampen run

# In another terminal or editor, modify src/ui/window.dampen
# For example, change the title:
#   <text value="todos" ... />
# to:
#   <text value="MY TASKS" ... />

# Save the file → See changes instantly! ⚡
```

Hot-reload works for:
- Widget structure changes (add/remove widgets)
- Attribute modifications (text, colors, sizes)
- Style updates (padding, spacing, alignment)
- Binding changes (data connections)

## File Structure

```
examples/todo-app/
├── Cargo.toml                 # Dependencies and build configuration
├── src/
│   ├── main.rs                # Entry point with #[dampen_app] macro
│   ├── shared.rs              # SharedState struct (future use)
│   └── ui/
│       ├── mod.rs             # UI module exports
│       ├── window.rs          # Model, handlers, business logic
│       └── window.dampen      # UI definition (declarative XML)
```

## Architecture

### Dual-Mode System

Dampen supports two execution modes:

| Aspect | Interpreted Mode | Codegen Mode |
|--------|-----------------|--------------|
| **Command** | `dampen run` | `dampen build --release` |
| **XML Processing** | Runtime | Build time |
| **Hot-Reload** | ✅ Yes | ❌ No |
| **Startup Time** | ~500ms | ~300ms |
| **Use Case** | Development | Production |

### Data Flow

```
User Action (click, type)
    ↓
UI Widget (button, text_input)
    ↓
Handler (edit_task, create_task)
    ↓
Model Update + update_computed_fields()
    ↓
UI Re-render (automatic)
```

### Handler System

Handlers are registered in `window.rs` and connected to UI events in `window.dampen`:

**Rust (window.rs):**
```rust
#[ui_handler]
pub fn create_task(model: &mut Model) {
    if !model.input_value.trim().is_empty() {
        model.tasks.push(Task::new(model.input_value.clone()));
        model.input_value.clear();
        update_computed_fields(model);
    }
}
```

**XML (window.dampen):**
```xml
<text_input
    value="{input_value}"
    on_input="input_changed"
    on_submit="create_task"
    placeholder="What needs to be done?" />
```

## Testing

### Manual Testing

```bash
# Test interpreted mode
dampen run
# Verify: create, edit, delete, complete, filter tasks

# Test hot-reload
# 1. Keep app running
# 2. Edit window.dampen
# 3. Verify changes appear within 2 seconds

# Test production mode
dampen build --release
./target/release/todo-app
# Verify: all features work, fast startup
```

### Unit Tests

Unit tests are implemented for core model logic:

```bash
cargo test -p todo-app
```

## Performance

- **Startup**: <1 second (production mode)
- **Task Operations**: <10ms (instant UI updates)
- **Hot-Reload**: <2 seconds
- **Memory**: ~30-40MB baseline
- **Scale**: Tested with 1000+ tasks

## Debugging

**View Parsed IR:**
```bash
dampen inspect src/ui/window.dampen
```

**Check XML Syntax:**
```bash
dampen check
```

**Enable Verbose Logging:**
```bash
RUST_LOG=debug dampen run
```

## Common Issues

| Issue | Solution |
|-------|----------|
| Hot-reload not working | Ensure you're running `dampen run` (not `dampen build`) |
| Handler not found | Check handler name in XML matches registry in window.rs |
| Binding not working | Ensure field exists in Model and isn't marked `#[ui_skip]` |
| Codegen fails | Run `dampen check` to validate XML syntax |

## Learn More

- **Full Guide**: See `/specs/001-refactor-todo-app/quickstart.md` for detailed walkthrough
- **Implementation Plan**: `/specs/001-refactor-todo-app/plan.md`
- **Data Model**: `/specs/001-refactor-todo-app/data-model.md`
- **Dampen Documentation**: Root `README.md` and `CLAUDE.md`

## Contributing

Found a bug or want to improve the example? Contributions are welcome!

1. Check existing issues at https://github.com/anomalyco/dampen/issues
2. Submit a PR with clear description of changes
3. Ensure `cargo clippy` and `cargo fmt` pass
4. Test in both interpreted and codegen modes

## License

This example is part of the Dampen project. See root LICENSE file for details.
