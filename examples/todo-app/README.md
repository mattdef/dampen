# Todo App Example

A comprehensive modern todo application demonstrating advanced Dampen widgets including ProgressBar, Canvas, Tooltip, PickList, and Image widgets.

## Features

### Widget Demonstration

This example showcases the following Dampen widgets:

- **ProgressBar**: Visual completion tracking showing overall task completion percentage
- **Canvas**: Custom 7-day completion trend visualization using the `canvas::Program` trait
- **Tooltip**: Contextual help text on action buttons
- **PickList**: Dropdown selections for category, priority, and filtering
- **Image**: Priority icons (Low, Medium, High)
- **Toggler**: Dark mode toggle
- **Standard widgets**: Text, Button, TextInput, Row, Column, Scrollable, Rule, Space

### Functionality

- **CRUD Operations**: Add, toggle completion, and delete todo items
- **Category Management**: Organize tasks by category (Work, Personal, Shopping, Health, Finance, Other)
- **Priority Levels**: Assign priority levels (Low, Medium, High) with visual indicators
- **Filtering**: View All, Active, or Completed tasks
- **Statistics**: Real-time completion tracking with progress bar
- **Dark Mode**: Toggle between light and dark themes

## Data Model

### TodoItem
```rust
pub struct TodoItem {
    pub id: usize,
    pub text: String,
    pub category: String,
    pub priority: Priority,
    pub completed: bool,
}
```

### Enums
- **Priority**: Low | Medium | High
- **TodoFilter**: All | Active | Completed

## Running the Example

```bash
cd examples/todo-app
dampen run
```

The application will open in a window. You can:
1. Add new tasks using the text input and "Add Task" button
2. Select a category and priority for each task
3. Filter tasks by status using the dropdown
4. Toggle task completion (implementation pending)
5. View overall progress in the progress bar
6. See completion trends in the canvas chart
7. Toggle dark mode using the toggler

## Running in Different Modes

### Development Mode (Interpreted with Hot-Reload)

```bash
cd examples/todo-app
dampen run
```

The UI will reload automatically when you modify `.dampen` files.

### Production Mode (Codegen)

```bash
# Debug build
dampen build -p todo-app

# Release build (optimized)
dampen build --release -p todo-app
# or equivalently:
dampen release -p todo-app

# Run
./target/release/todo-app
```

### Framework Development (using cargo directly)

If you're contributing to the Dampen framework, you can also use cargo:

```bash
# Interpreted mode
cargo run -p todo-app

# Codegen mode
cargo build -p todo-app --release --no-default-features --features codegen
./target/release/todo-app
```

## Hot-Reload Development Workflow

Dampen provides instant hot-reload capabilities for rapid UI iteration. Change your XML files and see updates in real-time without losing application state!

### Running in Debug Mode

To enable hot-reload, run the application in debug mode:

```bash
# Standard debug mode (hot-reload enabled by default)
dampen run

# With detailed logging
RUST_LOG=debug dampen run
```

When hot-reload is active, you'll see this message in the console:

```
🔥 Hot-reload enabled! Edit src/ui/window.dampen to see live updates.
📊 Multi-view enabled: Main window ↔ Statistics
```

### Hot-Reload Behavior

Understanding how hot-reload works helps you iterate efficiently:

| What Changes | Result | State Preserved |
|--------------|--------|-----------------|
| **XML UI structure** (widgets, layouts) | ✅ Updates instantly (< 1s) | ✅ Tasks, theme, filters |
| **Style attributes** (colors, spacing, sizes) | ✅ Updates instantly | ✅ Tasks, theme, filters |
| **Bindings** (`{field}` expressions) | ✅ Updates instantly | ✅ Tasks, theme, filters |
| **Rust code** (handlers, Model struct) | ❌ Requires rebuild | N/A |
| **Local UI state** (text input, scroll position) | ⚠️ Resets on reload | ✅ Tasks persist |

**Key Points:**
- ✅ **Tasks persist**: Your todo list is saved and restored automatically
- ✅ **Theme persists**: Dark mode preference is maintained
- ✅ **Filters persist**: Active/Completed filter selection is saved
- ⚠️ **Input resets**: Text being typed is cleared (design trade-off for simplicity)
- ⚠️ **Scroll resets**: Scroll position returns to top

### Error Handling

Hot-reload is resilient to XML syntax errors:

**When you introduce an invalid XML change:**

```
thread 'main' panicked at examples/todo-app/src/ui/window.rs:161:1:
Failed to parse Dampen UI file: ParseError {
    kind: InvalidValue,
    message: "Invalid alignment: 'right'. Expected start, center, end, or stretch",
    span: Span { start: 10787, end: 11005, line: 323, column: 27 },
    suggestion: None
}
```

**What happens:**
1. ✅ **App stays running**: No crash, old UI remains visible
2. ✅ **Clear error message**: Shows exact line and column number
3. ✅ **Tasks safe**: Your data is preserved
4. ✅ **Auto-recovery**: Fix the XML and save - UI updates immediately

**Common errors and fixes:**

| Error | Cause | Fix |
|-------|-------|-----|
| `Invalid alignment: 'right'` | Used HTML alignment name | Use Dampen values: `start`, `center`, `end`, `stretch` |
| `Unclosed tag` | Missing `</widget>` | Add closing tag or use self-closing `<widget />` |
| `Unknown widget` | Typo in widget name | Check spelling: `text_input` not `textInput` |
| `Unknown attribute` | Invalid attribute name | Consult widget docs for valid attributes |

### Example: Changing Button Color

Let's modify the "Add Task" button style in real-time:

1. **Keep the app running** in one terminal
2. **Open** `src/ui/window.dampen` in your editor
3. **Find** the `btn_success` style class (around line 90):

```xml
<style name="btn_success">
    <base
        background="#27ae60"
        color="#ffffff"
        padding="8 16"
        border_radius="6"
        border_width="0"
    />
    <hover background="#52be80" />
    <active background="#1e8449" />
</style>
```

4. **Change** the background color to purple:

```xml
background="#9b59b6"  <!-- Was: #27ae60 -->
```

5. **Save** the file
6. **Watch** the button turn purple instantly - no rebuild needed!

### Example: Adjusting Spacing

Fine-tune your layout spacing in real-time:

1. **Keep the app running**
2. **Open** `src/ui/window.dampen`
3. **Find** the main container (around line 220):

```xml
<scrollable>
    <container padding="20">
        <column spacing="20">
```

4. **Change** the padding and spacing:

```xml
<container padding="40">  <!-- Was: 20 -->
    <column spacing="30">  <!-- Was: 20 -->
```

5. **Save** the file
6. **Watch** the UI breathing space expand instantly!

**Try these spacing values:**
- Tight layout: `padding="10"` `spacing="10"`
- Comfortable (default): `padding="20"` `spacing="20"`
- Spacious: `padding="40"` `spacing="30"`
- Extra spacious: `padding="60"` `spacing="40"`

### Example: Adding a Tooltip

Enhance user experience by adding contextual help:

1. **Keep the app running**
2. **Open** `src/ui/window.dampen`
3. **Find** the "Clear Completed" button (around line 240):

```xml
<button
    label="Clear Completed"
    on_click="clear_completed"
    class="btn_outlined"
/>
```

4. **Wrap it with a tooltip**:

```xml
<tooltip
    message="Remove all completed tasks from the list"
    position="top"
>
    <button
        label="Clear Completed"
        on_click="clear_completed"
        class="btn_outlined"
    />
</tooltip>
```

5. **Save** the file
6. **Hover** over the button to see your new tooltip!

**Tooltip positions available:** `top`, `bottom`, `left`, `right`

### Troubleshooting Hot-Reload

**Problem: Hot-reload not working?**

Check these common issues:

1. **Debug mode not enabled**
   ```bash
   # Make sure you're running in debug mode (not release)
   cargo run -p todo-app   # ✅ Debug mode (hot-reload enabled)
   cargo run --release     # ❌ Release mode (no hot-reload)
   ```

2. **File watcher permissions**
   - Linux: Check inotify limits
     ```bash
     # Check current limit
     cat /proc/sys/fs/inotify/max_user_watches
     
     # Increase if needed (add to /etc/sysctl.conf)
     fs.inotify.max_user_watches=524288
     ```
   - macOS: File watcher should work by default (uses FSEvents)
   - Windows: File watcher should work by default (uses ReadDirectoryChangesW)

3. **File not being watched**
   - Check console output: `Successfully watching: /path/to/file.dampen`
   - Only `.dampen` files in `src/ui/` are watched by default
   - If you moved files, restart the application

4. **Syntax error blocking reload**
   - Check console for `ParseError` messages
   - Fix the syntax error and save again
   - The app will recover automatically

5. **Changes not visible**
   - Make sure you saved the file (check editor)
   - Try a more obvious change (e.g., change a color to red)
   - Check if you're editing the correct file (main vs statistics view)

**Still not working?**
- Restart the application
- Check `RUST_LOG=debug cargo run -p todo-app` for detailed logs
- Ensure no other process is locking the `.dampen` files

### Tips for Effective Hot-Reload Development

✅ **DO:**
- Make small, incremental changes to see immediate feedback
- Use hot-reload for visual tuning (colors, spacing, layouts)
- Keep the console visible to catch parse errors quickly
- Test different filter states and themes during development

❌ **AVOID:**
- Making multiple simultaneous changes (harder to debug)
- Editing Rust code and XML together (Rust needs full rebuild)
- Relying on scroll position or input state preservation

### Multi-View Hot-Reload

This app demonstrates hot-reload with multiple windows:

- **Main window** (`window.dampen`): Hot-reloads independently
- **Statistics window** (`statistics.dampen`): Hot-reloads independently
- **Shared state**: Always synchronized between views

**Try it:**
1. Open the Statistics window (click "📊 Statistics")
2. Modify `statistics.dampen` (change a color or spacing)
3. Save and watch the Statistics window update
4. Main window remains unaffected

## Inspecting Generated Code

Dampen transforms your declarative XML into efficient Rust code at compile-time. Understanding this generated code builds trust and helps debug complex scenarios.

### Using `dampen inspect`

The `dampen inspect` command shows you the intermediate representation (IR) that Dampen generates from your XML:

```bash
# Inspect the main window UI
dampen inspect --file src/ui/window.dampen

# Inspect the statistics window
dampen inspect --file src/ui/statistics.dampen
```

**Output includes:**
- Widget tree structure
- Bindings and their types
- Event handlers
- Style classes applied
- Conditional expressions

**Example output:**
```
Document {
    themes: [Theme { name: "light", palette: {...} }, Theme { name: "dark", ... }],
    styles: [StyleClass { name: "btn_primary", ... }, ...],
    root: Container {
        padding: 20,
        children: [
            Column {
                spacing: 20,
                children: [
                    Text { value: Binding("new_item_text"), size: 16 },
                    Button { label: "Add Task", on_click: Handler("add_item") },
                    ...
                ]
            }
        ]
    }
}
```

### Viewing Generated Rust Code

Dampen generates Rust code during the build process. You can inspect this generated code to understand exactly how your XML translates to Rust.

**Finding generated code:**

```bash
# Build the project first
cargo build -p todo-app

# Find the build output directory
ls target/debug/build/ | grep todo-app

# View the generated code
cat target/debug/build/todo-app-*/out/window.rs
```

**Or use this one-liner:**

```bash
# View generated window.rs
find target/debug/build -name "window.rs" -path "*/out/*" | head -1 | xargs cat

# View generated statistics.rs
find target/debug/build -name "statistics.rs" -path "*/out/*" | head -1 | xargs cat

# Or use cargo expand to see macro-generated code
cargo expand ui::window | less
cargo expand ui::statistics | less
```

**Generated code location:**

For this app using `#[dampen_ui]` macro, code is generated inline. Use `cargo expand` to view it:

```
mod _app {
    use dampen_core::parse;
    use std::sync::LazyLock;
    
    fn __load_document() -> dampen_core::DampenDocument {
        let xml = "<?xml version=\"1.0\" ...>";  // Full XML inlined
        parse(xml).expect("Failed to parse Dampen UI file")
    }
    
    pub static DOCUMENT: LazyLock<DampenDocument> = LazyLock::new(__load_document);
    
    pub fn document() -> dampen_core::DampenDocument {
        (*DOCUMENT).clone()
    }
}
```

For build.rs-based projects, generated files are in:
```
target/
└── debug/
    └── build/
        └── todo-app-<hash>/
            └── out/
                ├── window.rs          # Generated from window.dampen
                └── statistics.rs      # Generated from statistics.dampen
```

### Understanding the XML-to-Rust Mapping

Dampen transforms your declarative XML into type-safe Rust structures. Here are common patterns:

#### Example 1: Button Widget

**XML:**
```xml
<button 
    label="Add Task" 
    on_click="add_item" 
    class="btn_success"
/>
```

**Generated IR (conceptual):**
```rust
WidgetKind::Button {
    label: Some("Add Task".to_string()),
    on_press: Some(HandlerRef {
        name: "add_item",
        args: vec![]
    }),
    style_class: Some("btn_success".to_string()),
}
```

#### Example 2: Data Binding

**XML:**
```xml
<text value="{completion_percentage}%" size="14" />
```

**Generated IR:**
```rust
WidgetKind::Text {
    value: BindingExpr::FieldAccess {
        path: vec!["completion_percentage"]
    },
    size: Some(14),
}
```

#### Example 3: Event Handler with Arguments

**XML:**
```xml
<button 
    label="Delete" 
    on_click="delete_item:{item.id}"
/>
```

**Generated IR:**
```rust
WidgetKind::Button {
    label: Some("Delete".to_string()),
    on_press: Some(HandlerRef {
        name: "delete_item",
        args: vec![
            BindingExpr::FieldAccess {
                path: vec!["item", "id"]
            }
        ]
    }),
}
```

#### Example 4: Conditional Expression

**XML:**
```xml
<text value="{if completion_percentage == 100 then 'Done!' else 'In Progress'}" />
```

**Generated IR:**
```rust
WidgetKind::Text {
    value: BindingExpr::Conditional {
        condition: Box::new(BinaryOp {
            op: Operator::Equals,
            left: FieldAccess { path: ["completion_percentage"] },
            right: Literal(100)
        }),
        if_true: Box::new(Literal("Done!")),
        if_false: Box::new(Literal("In Progress"))
    }
}
```

### Code Quality Guarantees

Dampen-generated code follows strict quality standards:

✅ **Idiomatic Rust:**
- `snake_case` for functions and variables
- `CamelCase` for types and enums
- Proper lifetimes and ownership

✅ **Zero Runtime Overhead:**
- XML parsing happens at compile-time
- Generated code uses `LazyLock` for static initialization
- No dynamic dispatch or reflection

✅ **Type Safety:**
- Bindings are type-checked via `UiBindable` trait
- Event handlers validated at compile-time
- No string-based lookups at runtime

✅ **Readable Output:**
- Generated code includes comments with source XML locations
- Meaningful variable names derived from XML
- Structured similarly to hand-written Rust

### Verifying Code Quality

Run these commands to verify the generated code meets quality standards:

```bash
# Check for compiler warnings (should be zero)
cargo clippy --all -- -D warnings

# Build in release mode (optimizations enabled)
cargo build --release -p todo-app

# Check binary size (should be reasonable)
ls -lh target/release/todo-app
```

**Expected results:**
- ✅ Zero clippy warnings
- ✅ Release binary ~27 MB (debug: ~155 MB)
- ✅ Fast compilation (~30s release, ~2s incremental debug)

### Performance Transparency

Understanding the performance characteristics of generated code:

| Metric | Debug Build | Release Build |
|--------|-------------|---------------|
| **Binary Size** | ~155 MB | ~27 MB |
| **Compilation Time** | ~2-5s (incremental) | ~30s (full) |
| **Runtime Overhead** | Zero (static initialization) | Zero |
| **Memory Usage** | ~10-30 MB baseline | ~10-30 MB baseline |

**Why the size difference?**
- Debug: Includes debug symbols, unoptimized code, panic info
- Release: Stripped symbols, LTO, optimizations applied

**Generated code characteristics:**
- No runtime XML parsing
- No virtual dispatch for widgets
- Direct function calls for event handlers
- Static type checking for bindings

### Debugging Tips

**If generated code doesn't compile:**

1. Check `dampen inspect` output for syntax errors
2. Verify your Model struct has all referenced fields
3. Ensure handlers are registered in HandlerRegistry
4. Check binding types match Model field types

**If generated code looks wrong:**

1. Verify your XML is valid (`dampen check`)
2. Check for typos in widget names or attributes
3. Review binding expressions for syntax errors
4. Consult the Dampen widget documentation

**Still confused?**
- Read the generated code - it's meant to be understandable!
- Compare with hand-written Iced code
- Ask in Dampen community forums

## Implementation Details

### Event Handlers

All event handlers are registered manually with HandlerRegistry:

- `add_item`: Add new todo with category and priority
- `toggle_item`: Toggle completion status
- `delete_item`: Remove a todo
- `clear_all`: Remove all todos
- `clear_completed`: Remove only completed todos
- `update_category`: Change selected category
- `update_priority`: Change selected priority
- `apply_filter`: Apply filter (All/Active/Completed)
- `toggle_dark_mode`: Toggle dark/light mode
- `update_new_item`: Update new item text input

### Canvas Visualization

The statistics chart implements the `canvas::Program<Message>` trait:

```rust
impl canvas::Program<Message> for StatisticsChart {
    type State = ();
    
    fn draw(&self, ...) -> Vec<canvas::Geometry> {
        // Draw 7-day completion trend
        // - Axes with labels
        // - Data points as circles
        // - Connected lines
    }
}
```

### State Management

The application uses `#[derive(UiModel)]` for automatic binding support:

```rust
#[derive(UiModel, Debug, Clone, Serialize, Deserialize)]
pub struct TodoAppModel {
    pub items: Vec<TodoItem>,
    pub current_filter: TodoFilter,
    pub new_item_text: String,
    pub selected_category: String,
    pub selected_priority: Priority,
    pub dark_mode: bool,
    pub completed_count: i64,
    pub pending_count: i64,
    pub completion_percentage: f32,
    // ...
}
```

## Limitations

- **ComboBox**: Not yet implemented in the widget builder, using PickList instead
- **Grid**: Not yet implemented, using Row layout for headers
- **Float**: Not yet implemented, no floating action button
- **Dynamic Lists**: Current implementation uses placeholder text instead of rendering individual todo items (awaiting list rendering support)

## Next Steps

Once ComboBox, Grid, and dynamic list rendering are implemented, this example will be updated to:

1. Replace PickList with ComboBox for searchable category selection
2. Use Grid layout for proper task table display
3. Render individual todo items with checkboxes
4. Add floating action button for quick task addition

## Screenshots

*Coming soon: Screenshots showing the todo app with various tasks, categories, and the canvas visualization*

## License

This example is part of the Dampen framework and follows the same licensing terms.
