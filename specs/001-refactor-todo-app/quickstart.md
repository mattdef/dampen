# Quickstart: Refactor Todo-App to Match Iced Example

**Feature**: 001-refactor-todo-app
**Date**: 2026-01-22

## Overview

This quickstart guide helps you get started with the refactored todo-app example. The application demonstrates Dampen's dual-mode architecture (interpreted dev mode with hot-reload, codegen production mode) and matches the visual design of the Iced "todos" example.

## Prerequisites

- Rust 1.85 or higher (MSRV for Rust 2024 edition)
- Dampen CLI tool (if developing Dampen framework)
- Iced 0.14+ (already in dependencies)

## Project Structure

```
examples/todo-app/
├── Cargo.toml                 # Dependencies and metadata
├── src/
│   ├── main.rs                 # Entry point with #[dampen_app] macro
│   ├── shared.rs               # SharedState struct (simplified)
│   └── ui/
│       ├── window.rs           # Model, handlers, app logic
│       └── window.dampen      # UI definition (single view)
```

## Quick Start (3 Minutes)

### 1. Build and Run in Development Mode

```bash
cd examples/todo-app

# Development mode with hot-reload
dampen run
```

**What happens**:
- Dampen parses `window.dampen` XML at runtime
- Builds widget tree dynamically
- Enables hot-reload (XML changes appear in 2 seconds)
- Application starts and shows todo list

### 2. Try the Features

**Create a task**:
1. Type "Buy groceries" in the input field
2. Press Enter
3. Task appears in the list

**Mark task complete**:
1. Click the checkbox next to "Buy groceries"
2. Task is marked complete (visual indicator)
3. Counter updates: "0 tasks left"

**Filter tasks**:
1. Click "Active" - shows only incomplete tasks
2. Click "Completed" - shows only completed tasks
3. Click "All" - shows all tasks

**Edit a task**:
1. Click the ✏️ button next to a task
2. Task description becomes a text input
3. Modify the text and press Enter
4. Task is updated

**Delete a task**:
1. Click the 🗑️ button next to a task
2. Task is removed from the list

### 3. Try Hot-Reload

1. Keep the application running (`dampen run`)
2. Open `examples/todo-app/src/ui/window.dampen` in your editor
3. Find the title text: `<text value="todos" ... />`
4. Change it to: `<text value="MY TASKS" ... />`
5. Save the file
6. **Result**: Title changes in the running app within 2 seconds!

### 4. Build and Run in Production Mode

```bash
# Production build with codegen
dampen build --release

# Or use the alias
dampen release

# Run the production binary
./target/release/todo-app
```

**What happens**:
- Dampen reads `window.dampen` at build time
- Converts XML to Rust code (TokenStream)
- Compiles generated code with application
- No XML parsing at runtime
- Zero runtime overhead

**Performance**:
- Startup: <1 second
- Task operations: <10ms (instant UI updates)
- Memory: ~30-40MB baseline

## Development Workflow

### Adding New Features

#### Example: Add "Clear All Completed" Button

**Step 1**: Add handler to `window.rs`:

```rust
#[ui_handler]
pub fn clear_completed(model: &mut Model) {
    model.tasks.retain(|t| !t.completed);
    update_computed_fields(model);
}
```

**Step 2**: Register handler in `create_handler_registry()`:

```rust
registry.register_simple("clear_completed", |model: &mut dyn Any| {
    if let Some(m) = model.downcast_mut::<Model>() {
        clear_completed(m);
    }
});
```

**Step 3**: Add button to `window.dampen`:

```xml
<row spacing="20">
    <text value="{tasks_left_text}" width="fill" />

    <row spacing="10">
        <button label="All" on_click="filter:All" />
        <button label="Active" on_click="filter:Active" />
        <button label="Completed" on_click="filter:Completed" />
        <button label="Clear Completed" on_click="clear_completed" />
    </row>
</row>
```

**Step 4**: Run `dampen run` and see the new button!

#### Example: Change Window Size

**Step 1**: Open `examples/todo-app/src/main.rs`

**Step 2**: Find the window_size configuration:

```rust
iced::application(TodosApp::init, TodosApp::update, TodosApp::view)
    .window_size(iced::Size::new(500.0, 800.0))  // Change these values
    .centered()
    .subscription(TodosApp::subscription)
    .run()
```

**Step 3**: Update to desired size:

```rust
.window_size(iced::Size::new(600.0, 900.0))
```

**Step 4**: Run `dampen run` - window is now larger!

### Testing Changes

**Development Testing**:
```bash
# Run with hot-reload
dampen run

# Make changes to window.dampen or window.rs
# Observe changes immediately (hot-reload for XML)
# Or restart app for Rust code changes
```

**Production Testing**:
```bash
# Build production version
dampen build --release

# Test all features:
# - Create, edit, delete, complete tasks
# - Filter by All/Active/Completed
# - Verify empty states
# - Check performance (startup, operations)
```

**Unit Testing**:
```bash
# Run unit tests (if added)
cargo test -p todo-app

# Run with output
cargo test -p todo-app -- --nocapture
```

### Debugging

**View Parsed IR** (for debugging XML parsing):
```bash
dampen inspect src/ui/window.dampen
```

**Check XML Syntax**:
```bash
dampen check
```

**Enable Verbose Logging**:
```bash
RUST_LOG=debug dampen run
```

**Common Issues**:

| Issue | Solution |
|-------|----------|
| XML syntax error | Run `dampen check` to validate syntax |
| Handler not found | Check handler name in XML matches registry |
| Binding not working | Ensure field exists in Model and is not marked `#[ui_skip]` |
| Hot-reload not working | Check you're running `dampen run` (not `dampen build`) |
| Codegen fails | Check for unsupported features in XML (e.g., dynamic attributes) |

## Code Architecture

### Model (`window.rs`)

```rust
#[derive(Default, UiModel, Serialize, Deserialize, Clone, Debug)]
pub struct Model {
    pub input_value: String,      // Text in input field
    pub filter: Filter,           // Current filter
    pub tasks: Vec<Task>,          // All tasks
    #[ui_skip]
    pub editing_id: Option<Uuid>, // Task being edited
    pub edit_text: String,        // Text in edit input
    #[ui_skip]
    pub filtered_tasks: Vec<Task>, // Filtered tasks
    pub tasks_left: i64,          // Count of incomplete
    pub tasks_left_text: String,   // "X tasks left"
    pub empty_message: String,     // Empty state message
    pub filtered_tasks_len: i64,   // Count of filtered
}
```

### Handlers (`window.rs`)

```rust
#[ui_handler]
pub fn input_changed(model: &mut Model, value: String) {
    model.input_value = value;
}

#[ui_handler]
pub fn create_task(model: &mut Model) {
    if !model.input_value.trim().is_empty() {
        model.tasks.push(Task::new(model.input_value.clone()));
        model.input_value.clear();
        update_computed_fields(model);
    }
}

#[ui_handler]
pub fn toggle_task(model: &mut Model, id: String) {
    if let Ok(uuid) = Uuid::parse_str(&id) {
        if let Some(task) = model.tasks.iter_mut().find(|t| t.id == uuid) {
            task.completed = !task.completed;
            update_computed_fields(model);
        }
    }
}

// ... more handlers (see full implementation)
```

### UI Definition (`window.dampen`)

```xml
<column width="fill" height="fill" padding="40">
    <scrollable>
        <container max_width="800" align_x="center">
            <column spacing="20">
                <!-- Title -->
                <text value="todos" size="100" width="fill" align_x="center" />

                <!-- Input for new tasks -->
                <text_input
                    id="new-task"
                    value="{input_value}"
                    on_input="input_changed"
                    on_submit="create_task"
                    placeholder="What needs to be done?"
                    padding="15"
                    size="30"
                />

                <!-- Controls row -->
                <row spacing="20">
                    <text value="{tasks_left_text}" width="fill" />
                    <row spacing="10">
                        <button label="All" on_click="filter:All" />
                        <button label="Active" on_click="filter:Active" />
                        <button label="Completed" on_click="filter:Completed" />
                    </row>
                </row>

                <!-- Tasks list -->
                <for each="task" in="{filtered_tasks}">
                    <if test="{task.state == 'Editing'}">
                        <!-- Editing mode -->
                        <row spacing="20">
                            <text_input
                                id="task-{task.id}"
                                value="{edit_text}"
                                on_input="update_edit_text"
                                on_submit="save_edit"
                                padding="10"
                            />
                            <button label="🗑️" on_click="delete_task:{task.id}" />
                        </row>
                    </if>

                    <if test="{task.state == 'Idle'}">
                        <!-- Idle mode -->
                        <row spacing="20" align_y="center">
                            <checkbox
                                checked="{task.completed}"
                                on_change="toggle_task:{task.id}"
                                width="fill"
                                label="{task.description}"
                            />
                            <button label="✏️" on_click="edit_task:{task.id}" />
                        </row>
                    </if>
                </for>

                <!-- Empty state -->
                <if test="{filtered_tasks_len == 0}">
                    <container height="200" width="fill" align_x="center" align_y="center">
                        <text value="{empty_message}" size="25" align_x="center" />
                    </container>
                </if>
            </column>
        </container>
    </scrollable>
</column>
```

## Key Concepts

### Dual-Mode Architecture

Dampen supports two execution modes:

| Aspect | Interpreted Mode | Codegen Mode |
|--------|-----------------|--------------|
| **Command** | `dampen run` | `dampen build --release` |
| **When XML is parsed** | Runtime | Build time |
| **Hot-reload** | ✅ Yes | ❌ No |
| **Runtime overhead** | ~1-5ms parsing | 0ms |
| **Compilation** | ❌ No | ✅ Yes |
| **Use case** | Development | Production |

**Workflow**:
1. Use **interpreted mode** while actively developing UI
2. Test in **codegen mode** before deployment
3. Ship **codegen mode** builds to production

### Data Binding

Dampen uses `{variable}` syntax to bind UI elements to model fields:

```xml
<text value="{tasks_left_text}" />
```

This binds the text element to `model.tasks_left_text`. When the field changes, the UI updates automatically.

### Handler Registration

Handlers are Rust functions that respond to UI events:

```rust
#[ui_handler]
pub fn create_task(model: &mut Model) {
    // implementation
}
```

Handlers are automatically registered with the `#[ui_handler]` macro.

### Conditional Rendering

Use `<if test="{condition}">` for conditional rendering:

```xml
<if test="{task.state == 'Editing'}">
    <text_input />
</if>
```

### List Rendering

Use `<for each="item" in="{list}">` for lists:

```xml
<for each="task" in="{filtered_tasks}">
    <checkbox label="{task.description}" />
</for>
```

## Best Practices

### Performance

- Keep model fields minimal (use `#[ui_skip]` for internal state)
- Use computed fields for derived data (recompute only when needed)
- Test with large task counts (1000+) to verify performance

### Code Organization

- Keep handlers in `window.rs` with the Model
- Put shared state in `shared.rs` (even if not used yet)
- Keep UI definition in `window.dampen` (single view for this example)

### Error Handling

- Use `Result<T, E>` for fallible operations
- Return early on validation errors (don't create invalid tasks)
- Include helpful error messages for syntax errors

### Testing

- Write unit tests for model logic
- Test manually in both interpreted and codegen modes
- Verify performance (startup, operations) before committing

## Troubleshooting

### Hot-Reload Not Working

**Symptom**: Changes to `window.dampen` don't appear in running app

**Solutions**:
1. Ensure you're running `dampen run` (not `dampen build`)
2. Check file watcher is active (look for "Watching files..." message)
3. Verify XML syntax is valid (run `dampen check`)

### Codegen Build Fails

**Symptom**: `dampen build --release` fails with compilation errors

**Solutions**:
1. Check generated code: `dampen inspect src/ui/window.dampen`
2. Verify all handlers are registered
3. Check for unsupported XML features (e.g., dynamic attributes)

### Handler Not Found

**Symptom**: Clicking a button does nothing, error in console

**Solutions**:
1. Check handler name in XML matches registry
2. Verify handler is registered in `create_handler_registry()`
3. Ensure handler has correct signature (`&mut Model` + optional args)

### Binding Not Working

**Symptom**: UI element doesn't update when model changes

**Solutions**:
1. Check field exists in Model
2. Ensure field is not marked `#[ui_skip]`
3. Verify `update_computed_fields()` is called after model changes

## Next Steps

- **Learn more**: Read `AGENTS.md` for Dampen development guidelines
- **Explore examples**: Check `examples/hello-world` for simpler demo
- **Extend the app**: Add features like task priorities, due dates, or tags
- **Contribute**: Report issues or submit PRs to improve Dampen

## Resources

- **Dampen Documentation**: See `README.md` in repo root
- **Iced Examples**: https://github.com/iced-rs/iced/tree/master/examples/todos
- **AGENTS.md**: Development guidelines and conventions
- **Feature Spec**: `/specs/001-refactor-todo-app/spec.md`
- **Implementation Plan**: `/specs/001-refactor-todo-app/plan.md`

## Support

- **Issues**: Report bugs or feature requests at https://github.com/anomalyco/dampen/issues
- **Discussions**: Ask questions at https://github.com/anomalyco/dampen/discussions
- **Documentation**: Check `docs/` directory for detailed guides

---

**Happy coding! 🚀**
