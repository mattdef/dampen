# Dampen Usage Guide

**For Application Developers**

This guide covers everything you need to know to build applications **with** Dampen. If you're contributing **to** the Dampen framework itself, see [CONTRIBUTING.md](CONTRIBUTING.md).

---

## Table of Contents

1. [Installation](#installation)
2. [Creating a New Project](#creating-a-new-project)
3. [Development Workflow](#development-workflow)
4. [CLI Commands Reference](#cli-commands-reference)
   - [`dampen new`](#dampen-new-name)
   - [`dampen add`](#dampen-add---ui-window_name) *(NEW!)*
   - [`dampen run`](#dampen-run)
   - [`dampen build`](#dampen-build)
   - [`dampen release`](#dampen-release)
   - [`dampen test`](#dampen-test)
   - [`dampen check`](#dampen-check)
   - [`dampen inspect`](#dampen-inspect-file)
5. [Common Tasks](#common-tasks)
   - [Adding a New Widget](#adding-a-new-widget)
   - [Adding a New Field to Your Model](#adding-a-new-field-to-your-model)
   - [Creating a New View](#creating-a-new-view)
   - [Building Multi-View Applications](#building-multi-view-applications-with-dampen_app)
   - [Debugging Build Issues](#debugging-build-issues)
   - [Testing Your Application](#testing-your-application)
6. [Working with Workspaces](#working-with-workspaces)
7. [Troubleshooting](#troubleshooting)

---

## Installation

Install the Dampen CLI tool using cargo:

```bash
cargo install dampen-cli
```

**That's the only time you need cargo!** After installation, use `dampen` commands for all development tasks.

### Verify Installation

```bash
dampen --version
```

### Updating the CLI

```bash
cargo install dampen-cli --force
```

---

## Creating a New Project

Create a new Dampen project with a single command:

```bash
dampen new my-app
cd my-app
```

This creates a complete project structure:

```
my-app/
├── Cargo.toml              # Project dependencies
├── build.rs                # Code generation (XML → Rust)
├── README.md               # Getting started guide
├── src/
│   ├── main.rs             # Application entry point
│   └── ui/
│       ├── mod.rs          # UI module exports
│       ├── window.rs       # UI model and handlers
│       └── window.dampen   # Declarative UI definition (XML)
└── tests/
    └── integration.rs      # Integration tests
```

### Run Your New Project

```bash
dampen run
```

Your application window will open with a working UI and interactive button!

---

## Development Workflow

### Typical Development Cycle

```bash
# 1. Create or edit UI files
vim src/ui/window.dampen

# 2. Validate XML syntax
dampen check

# 3. Run with hot-reload
dampen run

# 4. Make changes to XML - UI updates automatically!

# 5. Run tests
dampen test

# 6. Build for production
dampen release
```

### Hot-Reload Development

The `dampen run` command provides automatic hot-reload:

1. Start your application: `dampen run`
2. Edit `.dampen` XML files
3. Save the file
4. UI updates automatically in the running application!

**No restart needed** - changes appear instantly.

---

## CLI Commands Reference

### `dampen new <name>`

Create a new Dampen project.

```bash
dampen new my-app
```

**Options:**
- `<name>` - Project name (must be valid Rust package name)

**Output:**
- Creates directory with project structure
- Generates sample UI and code
- Ready to run immediately

---

### `dampen run`

Run your application in development mode with hot-reload.

```bash
# Basic run
dampen run

# Pass arguments to your application
dampen run -- --my-arg value

# Run specific package in workspace
dampen run -p my-app

# Verbose output
dampen run -v
```

**Options:**
- `-p, --package <PACKAGE>` - Package to run (workspace support)
- `-v, --verbose` - Show detailed output
- `-- <args>` - Pass arguments to the application

**Features:**
- Hot-reload enabled (XML changes applied automatically)
- Interpreted mode (fast startup, no rebuild)
- Development-optimized performance

---

### `dampen build`

Build your application in debug mode with codegen.

```bash
# Basic build
dampen build

# Build specific package
dampen build -p my-app

# Enable additional features
dampen build --features tokio

# Verbose output
dampen build -v
```

**Options:**
- `-p, --package <PACKAGE>` - Package to build
- `--features <FEATURES>` - Additional features (comma-separated)
- `-v, --verbose` - Show detailed output

**Output:**
- Debug binary in `target/debug/`
- Includes codegen (compile-time XML processing)
- No optimizations (fast compilation)

**Use Case:** Testing production codegen behavior without optimization overhead.

---

### `dampen release`

Build optimized production binary.

```bash
# Basic release build
dampen release

# Release build for specific package
dampen release -p my-app

# Enable additional features
dampen release --features tokio,logging

# Verbose output
dampen release -v
```

**Options:**
- `-p, --package <PACKAGE>` - Package to build
- `--features <FEATURES>` - Additional features (comma-separated)
- `-v, --verbose` - Show detailed output
- `--target-dir <DIR>` - Custom target directory

**Output:**
- Optimized binary in `target/release/`
- Full compiler optimizations
- Codegen enabled
- Ready for deployment

**Use Case:** Production builds, performance testing, deployment.

---

### `dampen test`

Run your test suite.

```bash
# Run all tests
dampen test

# Run tests matching a name
dampen test my_test

# Run tests for specific package
dampen test -p my-app

# Run in release mode
dampen test --release

# Quiet mode (show dots)
dampen test --quiet

# Verbose output
dampen test -v

# Pass arguments to test binary
dampen test -- --nocapture

# Run ignored tests
dampen test --ignored

# Run only ignored tests
dampen test --only-ignored
```

**Options:**
- `<TESTNAME>` - Filter tests by name
- `-p, --package <PACKAGE>` - Package to test
- `--release` - Run tests in release mode
- `--quiet` - Display one character per test
- `-v, --verbose` - Show detailed output
- `--features <FEATURES>` - Additional features
- `--ignored` - Run ignored tests
- `--only-ignored` - Run only ignored tests
- `-- <args>` - Arguments for test binary

**Use Case:** Running unit tests, integration tests, CI/CD pipelines.

---

### `dampen check`

Validate `.dampen` XML files without building.

```bash
# Check current directory
dampen check

# Check specific directory
dampen check --dir src/ui

# Verbose output
dampen check -v
```

**Options:**
- `--dir <DIR>` - Directory to check (default: current)
- `-v, --verbose` - Show detailed output

**Validates:**
- XML syntax correctness
- Widget names and attributes
- Binding expressions
- Handler references

**Output:**
- Success message if valid
- Detailed error messages with line/column numbers if invalid
- Exit code 0 for success, 1 for errors

---

### `dampen add --ui <window_name>`

**NEW!** Scaffold a new UI window with templates.

```bash
# Create a window in default location (src/ui/)
dampen add --ui settings

# Create a window in custom directory
dampen add --ui order_form --path "src/ui/orders"

# Window names are auto-converted to snake_case
dampen add --ui UserProfile
# → Creates: user_profile.rs, user_profile.dampen
```

**Options:**
- `--ui <NAME>` - Name of the window to create
- `--path <PATH>` - Custom output directory (default: `src/ui/`)

**What it creates:**
- `.rs` file with Model, handlers, and AppState setup
- `.dampen` file with basic UI layout
- Ready-to-use template based on best practices

**Generated structure:**

The Rust module includes:
```rust
#[derive(Default, Clone, UiModel)]
pub struct Model {
    pub message: String,
}

#[dampen_ui("settings.dampen")]
mod _settings {}

pub fn create_app_state() -> AppState<Model> { ... }
pub fn create_handler_registry() -> HandlerRegistry { ... }
```

The XML file includes:
```xml
<dampen>
    <column padding="40" spacing="20">
        <text value="Welcome to Settings!" size="32" weight="bold" />
        <button label="Click me!" on_click="on_action" />
        <text value="{message}" size="24" />
    </column>
</dampen>
```

**After generation:**

1. Add the module to `src/ui/mod.rs`:
   ```rust
   pub mod settings;
   ```

2. Validate the XML:
   ```bash
   dampen check
   ```

3. Use in your application:
   ```rust
   use ui::settings;
   let state = settings::create_app_state();
   ```

**Validation:**
- Ensures you're in a Dampen project
- Validates window name (must be valid Rust identifier)
- Prevents overwriting existing files
- Validates custom paths (must be relative, within project)

**Benefits:**
- ✅ Creates production-ready code in < 1 second
- ✅ Consistent structure across windows
- ✅ Includes all necessary boilerplate
- ✅ Safe (prevents accidental overwrites)

**Use case:** Quickly scaffold new windows without manual file creation or copy-pasting. Reduces window creation time from ~5 minutes to < 1 second.

---

### `dampen inspect <file>`

Inspect intermediate representation (IR) and generated code.

```bash
# Inspect a .dampen file
dampen inspect src/ui/window.dampen

# Show generated Rust code
dampen inspect src/ui/window.dampen --mode codegen
```

**Options:**
- `<file>` - Path to `.dampen` file
- `--mode <MODE>` - Output mode (ir, codegen)

**Use Case:** Debugging, learning, understanding code generation.

---

## Common Tasks

### Adding a New Widget

1. Edit your `.dampen` file:

```xml
<dampen>
    <column spacing="10">
        <text value="Hello" />
        <!-- Add new button -->
        <button label="Click me!" on_click="handle_click" />
    </column>
</dampen>
```

2. Add handler in your Rust code:

```rust
registry.register_simple("handle_click", |model: &mut dyn std::any::Any| {
    if let Some(m) = model.downcast_mut::<Model>() {
        m.message = "Button clicked!".to_string();
    }
});
```

3. Run and test:

```bash
dampen run
```

---

### Adding a New Field to Your Model

1. Update your model struct:

```rust
#[derive(Default, UiModel, Serialize, Deserialize, Clone, Debug)]
pub struct Model {
    pub message: String,
    pub counter: i32,  // Add new field
}
```

2. Use the field in your UI:

```xml
<text value="{counter}" />
```

3. Update handlers to modify the field:

```rust
registry.register_simple("increment", |model: &mut dyn std::any::Any| {
    if let Some(m) = model.downcast_mut::<Model>() {
        m.counter += 1;
    }
});
```

---

### Creating a New View

**Quick Method (Recommended):**

Use the `dampen add` command to scaffold a new view automatically:

```bash
# Create a settings view
dampen add --ui settings

# Create in custom directory
dampen add --ui admin_panel --path "src/ui/admin"
```

This creates both `.rs` and `.dampen` files with all necessary boilerplate. Then just:

1. Add to `src/ui/mod.rs`:
   ```rust
   pub mod settings;
   ```

2. Run `dampen check` to validate

3. Use in your app!

**Manual Method:**

If you prefer manual creation:

1. Create new files in `src/ui/`:

```bash
touch src/ui/settings.rs
touch src/ui/settings.dampen
```

2. Define the view in `settings.rs`:

```rust
use dampen_macros::{dampen_ui, UiModel};
use dampen_core::AppState;

#[derive(Default, UiModel, Serialize, Deserialize, Clone, Debug)]
pub struct SettingsModel {
    pub theme: String,
}

#[dampen_ui("settings.dampen")]
mod _settings {}

pub fn create_app_state() -> AppState<SettingsModel> {
    let document = _settings::document();
    AppState::new(document)
}
```

3. Create the UI in `settings.dampen`:

```xml
<dampen>
    <column padding="20">
        <text value="Settings" size="24" weight="bold" />
        <text value="{theme}" />
    </column>
</dampen>
```

4. Export from `src/ui/mod.rs`:

```rust
pub mod window;
pub mod settings;  // Add this
```

---

### Building Multi-View Applications with `#[dampen_app]`

For applications with multiple views (e.g., window, settings, about), use the `#[dampen_app]` macro to automatically generate view management boilerplate.

#### What the Macro Does

The `#[dampen_app]` macro:
- **Discovers** all `.dampen` files in your UI directory
- **Generates** a `CurrentView` enum with one variant per view
- **Creates** AppState fields for each view
- **Implements** `init()`, `update()`, `view()`, and `subscription()` methods
- **Provides** `switch_to_*()` convenience methods for navigation
- **Enables** hot-reload for all views (debug builds only)

#### Quick Start Example

1. **Create your view files** in `src/ui/`:

```
src/ui/
├── mod.rs
├── window.rs          → Main view
├── window.dampen
├── settings.rs        → Settings view
├── settings.dampen
├── about.rs           → About view
└── about.dampen
```

2. **Annotate your app struct** in `src/main.rs`:

```rust
use dampen_macros::dampen_app;

// Define your message enum
#[derive(Debug, Clone)]
pub enum Message {
    Handler(dampen_core::HandlerMessage),
    #[cfg(debug_assertions)]
    HotReload(std::path::PathBuf),
}

// Apply the macro
#[dampen_app(
    ui_dir = "src/ui",
    message_type = "Message",
    handler_variant = "Handler",
    hot_reload_variant = "HotReload",
    default_view = "window"
)]
struct MyApp;

fn main() -> iced::Result {
    iced::application(
        MyApp::init,
        MyApp::update,
        MyApp::view,
    )
    .subscription(MyApp::subscription)
    .run()
}
```

3. **The macro generates** all this code for you:

```rust
// CurrentView enum
pub enum CurrentView {
    Window,
    Settings,
    About,
}

// App struct with fields
pub struct MyApp {
    window_state: AppState<window::Model>,
    settings_state: AppState<settings::Model>,
    about_state: AppState<about::Model>,
    current_view: CurrentView,
}

// View switching helpers
impl MyApp {
    pub fn switch_to_window(&mut self) { /* ... */ }
    pub fn switch_to_settings(&mut self) { /* ... */ }
    pub fn switch_to_about(&mut self) { /* ... */ }
}

// Full init(), update(), view(), subscription() implementations
```

#### Configuration Options

**Required attributes:**

- `ui_dir` - Directory containing `.dampen` files (e.g., `"src/ui"`)
- `message_type` - Your Message enum name (e.g., `"Message"`)
- `handler_variant` - Variant for handler messages (e.g., `"Handler"`)

**Optional attributes:**

- `hot_reload_variant` - Variant for hot-reload events (enables file watching)
- `dismiss_error_variant` - Variant for error overlay dismissal
- `exclude` - Glob patterns to exclude (e.g., `["debug", "experimental/*"]`)
- `default_view` - View to show on startup (defaults to first alphabetically)

#### View Switching

Call the generated `switch_to_*()` methods in your handlers:

```rust
// In your view's handler
pub fn create_handlers() -> HandlerRegistry {
    let mut registry = HandlerRegistry::new();
    
    registry.register_command("go_to_settings", |model, app| {
        app.switch_to_settings();
        iced::Task::none()
    });
    
    registry
}
```

#### Excluding Views from Discovery

Exclude debug or experimental views:

```rust
#[dampen_app(
    ui_dir = "src/ui",
    message_type = "Message",
    handler_variant = "Handler",
    exclude = ["debug", "experimental/*", "*.backup"]
)]
struct MyApp;
```

Patterns support:
- Exact match: `"debug"` excludes `src/ui/debug.dampen`
- Wildcards: `"experimental/*"` excludes all files in `src/ui/experimental/`
- Extensions: `.dampen` is automatically added if not present

#### File Organization

**Flat structure** (recommended for small apps):

```
src/ui/
├── mod.rs
├── window.rs
├── window.dampen
├── settings.rs
└── settings.dampen
```

**Nested structure** (for larger apps):

```
src/ui/
├── mod.rs
├── window/
│   ├── mod.rs
│   └── window.dampen
└── settings/
    ├── mod.rs
    └── settings.dampen
```

The macro handles both automatically!

#### Best Practices

1. **Name your main view "window"** and use `default_view = "window"` for clarity
2. **Keep view files focused** - one responsibility per view
3. **Use exclude patterns** to hide debug/experimental views in production
4. **Leverage hot-reload** during development for instant feedback
5. **Test view switching** to ensure proper state isolation

#### Troubleshooting

**Error: "No views found"**
- Check `ui_dir` path is correct relative to crate root
- Ensure `.dampen` files exist in the directory
- Check exclude patterns aren't filtering everything

**Error: "View name must be a valid Rust identifier"**
- Rename files to use `snake_case` (e.g., `my_view.dampen`)
- Avoid special characters and spaces

**Error: "Corresponding .rs file not found"**
- Create `view_name.rs` next to `view_name.dampen`
- Export the module in `mod.rs`

**Error: "Default view 'xyz' not found"**
- Check spelling matches the filename (without `.dampen`)
- View must exist and not be excluded

See [docs/migration/multi-view-macro.md](migration/multi-view-macro.md) for migrating existing multi-view applications.

---

### Shared State for Multi-View Applications

**NEW in v0.2.4!** Share state across multiple views in your application using `SharedContext`.

#### When to Use Shared State

Use shared state when you need:
- **User preferences** (theme, language) accessible from all views
- **Session data** (logged-in user, auth tokens) shared across windows
- **Application-wide settings** that multiple views can read and modify
- **Cross-view communication** where one view's action affects another view's display

#### Using the `#[dampen_app]` Macro (Recommended)

The easiest way to add shared state is using the `shared_model` attribute:

1. **Define your shared state model** in `src/shared.rs`:

```rust
use dampen_macros::UiModel;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, UiModel, Serialize, Deserialize)]
pub struct SharedState {
    pub theme: String,
    pub username: String,
    pub language: String,
}
```

2. **Add `shared_model` to your `#[dampen_app]` macro**:

```rust
mod shared;  // Import the shared module

#[dampen_app(
    ui_dir = "src/ui",
    message_type = "Message",
    handler_variant = "Handler",
    shared_model = "SharedState"  // ← Add this line!
)]
struct MyApp;
```

**That's it!** The macro automatically:
- ✅ Creates `SharedContext<shared::SharedState>`
- ✅ Initializes it with `SharedState::default()`
- ✅ Passes it to all views via `create_app_state_with_shared()`
- ✅ Configures handlers to use `dispatch_with_shared()`

3. **Update your view modules** to support shared context:

```rust
// In src/ui/window.rs
pub fn create_app_state_with_shared(
    shared: SharedContext<SharedState>
) -> AppState<Model, SharedState> {
    let document = _window::document();
    let handlers = create_handler_registry();
    AppState::with_handlers(document, handlers)
        .with_shared_context(shared)
}

// Keep the old function for backward compatibility
pub fn create_app_state() -> AppState<Model> {
    let document = _window::document();
    let handlers = create_handler_registry();
    AppState::with_handlers(document, handlers)
}
```

4. **Use `{shared.field}` bindings** in your XML:

```xml
<dampen>
    <column padding="40" spacing="20">
        <text value="Welcome, {shared.username}!" size="24" />
        <text value="Theme: {shared.theme}" size="16" />
        <text value="Language: {shared.language}" size="16" />
    </column>
</dampen>
```

5. **Create handlers that modify shared state** as described in the handler variants section below.

**See the `macro-shared-state` example for a complete working application:**

```bash
dampen run -p macro-shared-state
```

---

#### Manual Setup (Advanced)

1. **Define your shared state model** in `src/shared.rs`:

```rust
use dampen_macros::UiModel;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, UiModel, Serialize, Deserialize)]
pub struct SharedState {
    pub theme: String,
    pub username: String,
    pub language: String,
}
```

2. **Create SharedContext in main.rs**:

```rust
use dampen_core::SharedContext;
use crate::shared::SharedState;

fn main() -> iced::Result {
    // Create shared context
    let shared = SharedContext::new(SharedState {
        theme: "dark".to_string(),
        username: "Guest".to_string(),
        language: "en".to_string(),
    });

    // Initialize views with shared context
    let window_state = ui::window::create_app_state_with_shared(shared.clone());
    let settings_state = ui::settings::create_app_state_with_shared(shared.clone());

    // ... rest of application setup
}
```

3. **Update AppState constructors** to accept shared context:

```rust
// In src/ui/window.rs
pub fn create_app_state_with_shared(
    shared: SharedContext<SharedState>
) -> AppState<Model, SharedState> {
    let document = _window::document();
    let handlers = create_handler_registry();
    AppState::with_handlers(document, handlers)
        .with_shared_context(shared)
}
```

4. **Use `{shared.field}` bindings** in your XML:

```xml
<!-- src/ui/window.dampen -->
<dampen>
    <column padding="40" spacing="20">
        <text value="Welcome, {shared.username}!" size="24" />
        <text value="Theme: {shared.theme}" size="16" />
        <text value="Language: {shared.language}" size="16" />
    </column>
</dampen>
```

5. **Create handlers that modify shared state**:

```rust
// In src/ui/settings.rs
pub fn create_handler_registry() -> HandlerRegistry {
    let mut registry = HandlerRegistry::new();

    registry.register_with_value_and_shared(
        "change_theme",
        |_model: &mut dyn Any, shared: &dyn Any, theme: String| {
            if let Some(s) = shared.downcast_ref::<SharedContext<SharedState>>() {
                s.write(|state| {
                    state.theme = theme;
                });
            }
        }
    );

    registry
}
```

#### Binding Syntax

**Local model bindings** (view-specific state):
```xml
<text value="{message}" />          <!-- Local to this view -->
<text value="{user.email}" />       <!-- Nested local field -->
```

**Shared state bindings** (cross-view state):
```xml
<text value="{shared.theme}" />     <!-- Shared across all views -->
<text value="{shared.username}" />  <!-- Shared user data -->
```

You can mix both in the same view:
```xml
<column>
    <text value="Hello, {shared.username}!" />  <!-- Shared -->
    <text value="{local_message}" />            <!-- Local -->
</column>
```

#### Handler Variants for Shared State

**Simple handler with shared access** (read-only):
```rust
registry.register_with_shared(
    "display_theme",
    |model: &mut dyn Any, shared: &dyn Any| {
        if let Some(s) = shared.downcast_ref::<SharedContext<SharedState>>() {
            let theme = s.read(|state| state.theme.clone());
            // Use theme value...
        }
    }
);
```

**Value handler with shared access** (receives input + shared state):
```rust
registry.register_with_value_and_shared(
    "update_preference",
    |model: &mut dyn Any, shared: &dyn Any, value: String| {
        if let Some(s) = shared.downcast_ref::<SharedContext<SharedState>>() {
            s.write(|state| {
                state.theme = value;
            });
        }
    }
);
```

**Command handler with shared access** (async operations):
```rust
registry.register_with_command_and_shared(
    "save_settings",
    |model: &mut dyn Any, shared: &dyn Any| -> Box<dyn Any> {
        if let Some(s) = shared.downcast_ref::<SharedContext<SharedState>>() {
            let state = s.read(|s| s.clone());
            // Return async task to save settings...
        }
        Box::new(iced::Task::none())
    }
);
```

#### Passing Parameters to Handlers

Handlers can receive parameters from XML using the colon syntax `handler_name:parameter`.

**String literal parameters** (for constant values):
```xml
<!-- Use single or double quotes for string literals -->
<button label="Light Theme" on_click="change_theme:'Light'" />
<button label="Dark Theme" on_click="change_theme:'Dark'" />
<button label="Alice" on_click="set_username:'Alice'" />
```

**Field reference parameters** (from model or loop context):
```xml
<!-- No quotes - references a field from the model -->
<button label="Save" on_click="save_value:{current_id}" />

<!-- In a for loop, reference loop item fields -->
<for expr="items" var="item">
    <button label="Edit" on_click="edit_item:{item.id}" />
    <button label="Delete" on_click="delete_item:{item.id}" />
</for>
```

**Important:**
- ✅ Use **quotes** for string literals: `on_click="handler:'value'"`
- ✅ Use **braces** for field references: `on_click="handler:{field}"`
- ❌ Without quotes, `light` is treated as a field name (and will be empty if not found)
- ❌ Without braces, `{item.id}` won't be resolved from loop context

**Example handler implementation:**
```rust
registry.register_with_value_and_shared(
    "change_theme",
    |model: &mut dyn Any, value: Box<dyn Any>, shared: &dyn Any| {
        if let (Some(s), Ok(theme)) = (
            shared.downcast_ref::<SharedContext<SharedState>>(),
            value.downcast::<String>(),
        ) {
            s.write(|state| {
                state.theme = *theme;  // theme is a Box<String>
            });
        }
    },
);
```

#### Thread Safety

`SharedContext<S>` is **thread-safe** and uses `Arc<RwLock<S>>` internally:
- **Multiple readers** can access shared state simultaneously
- **Single writer** blocks until all readers finish
- **Clone-friendly** - cloning creates a new reference to the same data
- **Sub-microsecond** access time (no performance concerns)

#### Hot-Reload Preservation

Shared state **survives hot-reload** automatically:
- Local view state (AppState<M>) resets on XML changes
- Shared state (SharedContext<S>) persists across reloads
- User preferences remain intact during development

```rust
// Hot-reload keeps shared state alive
s.write(|state| state.theme = "dark".to_string());
// Edit .dampen file and save...
// theme is still "dark" after reload! ✅
```

#### Complete Example

See the `macro-shared-state` example for a full working application:

```bash
# Run the macro-based shared state example (recommended)
dampen run -p macro-shared-state
```

**Project structure:**
```
examples/macro-shared-state/
├── src/
│   ├── main.rs           # App with #[dampen_app(shared_model = "SharedState")]
│   ├── shared.rs         # SharedState model
│   └── ui/
│       ├── window.rs     # Main view (displays shared state)
│       ├── window.dampen
│       ├── settings.rs   # Settings view (modifies shared state)
│       └── settings.dampen
```

**Key features demonstrated:**
- Using `shared_model` attribute for automatic setup (zero boilerplate!)
- Reading shared state with `{shared.field}` bindings
- Modifying shared state from handlers
- View switching with persistent shared state
- Hot-reload preservation of shared state

#### Best Practices

1. **Keep shared state minimal** - Only share data truly needed across views
2. **Use local state for view-specific data** - Don't over-share
3. **Clone on read for expensive operations** - Minimize lock duration
4. **Batch writes when possible** - Multiple writes in one `write()` call
5. **Document shared fields** - Make cross-view dependencies clear

#### Backward Compatibility

Shared state is **100% opt-in**:
- Existing apps work unchanged
- AppState<M> defaults to AppState<M, ()>
- No breaking changes to API
- {field} bindings still work as before

#### Troubleshooting

**Error: "{shared.field} renders empty"**
- Ensure AppState has shared_context set via `with_shared_context()`
- Check SharedContext is cloned correctly when passing to views
- Verify field name matches SharedState struct

**Error: "Handlers not modifying shared state"**
- Use `s.write(|state| ...)` not `s.read(|state| ...)`
- Ensure downcast succeeds: `if let Some(s) = ...`
- Check handler is registered with `_and_shared` variant

**Issue: "Views don't update when shared state changes"**

This is an Iced framework limitation: **modifying state doesn't automatically trigger re-renders**.

**Solution**: Add a dummy trigger field to force re-renders:

```rust
#[derive(Default, UiModel, Clone)]
pub struct Model {
    #[ui_skip]  // Don't expose to UI bindings
    _refresh_trigger: usize,
}

// In handlers that modify shared state:
registry.register_with_value_and_shared(
    "update_theme",
    |model: &mut dyn Any, value: Box<dyn Any>, shared: &dyn Any| {
        if let (Some(m), Some(s), Ok(theme)) = (
            model.downcast_mut::<Model>(),
            shared.downcast_ref::<SharedContext<SharedState>>(),
            value.downcast::<String>(),
        ) {
            // Modify shared state
            let mut guard = s.write();
            guard.theme = *theme;
            
            // Trigger re-render by modifying local model
            m._refresh_trigger = m._refresh_trigger.wrapping_add(1);
        }
    }
);
```

**Why this works**:
- Iced checks if the local model changed (via PartialEq/Clone)
- Incrementing `_refresh_trigger` makes the model "different"
- Iced re-renders the view, picking up the new shared state values

**See**: `examples/macro-shared-state/src/ui/settings.rs` for a complete example

**Error: "Shared state resets on hot-reload"**
- This should not happen! File a bug report if it does
- Verify you're using `dampen run` (dev mode)

---

### Interactive State Styling

**NEW in v0.3.0!** Add visual feedback to widgets based on user interaction (hover, pressed, focus, disabled).

#### What is State Styling?

State styling allows you to define different visual appearances for widgets based on their interaction state:

- **Hover**: Mouse cursor is over the widget
- **Active**: Widget is being clicked/pressed
- **Focus**: Widget has keyboard focus (text inputs)
- **Disabled**: Widget is disabled and non-interactive

#### Quick Example

```xml
<dampen>
    <styles>
        <style name="primary_button">
            <!-- Default appearance -->
            <base 
                background="#3498db"
                color="#ffffff"
                padding="12 24"
                border_radius="6" />
            
            <!-- Hover state: lighter blue -->
            <hover background="#5dade2" />
            
            <!-- Active state: darker blue -->
            <active background="#2874a6" />
            
            <!-- Disabled state: semi-transparent -->
            <disabled opacity="0.5" />
        </style>
    </styles>
    
    <button label="Click Me" class="primary_button" on_click="handle_click" />
</dampen>
```

#### Supported Widgets

| Widget | Supported States | Example Use Case |
|--------|------------------|------------------|
| **Button** | Hover, Active, Disabled | Interactive buttons with press feedback |
| **TextInput** | Hover, Focus, Disabled | Highlight focused input field |
| **Checkbox** | Hover, Disabled | Show hover feedback on checkboxes |
| **Radio** | Hover | Highlight radio button on hover |
| **Toggler** | Hover, Disabled | Toggle switch with hover state |

#### Creating a Style Class

1. **Define the style class** in your `.dampen` file:

```xml
<styles>
    <style name="success_button">
        <base 
            background="#27ae60"
            color="#ffffff"
            padding="10 20"
            border_radius="4" />
        <hover background="#52be80" />
        <active background="#1e8449" />
    </style>
</styles>
```

2. **Apply the class** to widgets:

```xml
<button label="Save" class="success_button" on_click="save" />
<button label="Submit" class="success_button" on_click="submit" />
```

3. **Test the interaction**:
   ```bash
   dampen run
   ```
   
   Move your mouse over the button to see the hover effect, then click to see the active state.

#### Style Precedence

Styles are applied in this order (highest to lowest priority):

1. **Inline styles** (attributes on the widget itself)
2. **State styles** (from `<hover>`, `<active>`, etc.)
3. **Base styles** (from `<base>`)
4. **Theme defaults**

**Example**:

```xml
<style name="btn">
    <base background="#3498db" color="#ffffff" />
    <hover background="#5dade2" />
</style>

<!-- Inline background overrides hover state -->
<button class="btn" background="#e74c3c" />
<!-- Result on hover: background is RED (#e74c3c), not blue -->
```

#### Common Patterns

**Pattern 1: Subtle Hover Feedback**

```xml
<style name="card">
    <base 
        background="#ffffff"
        padding="20"
        border_radius="12"
        border_width="1"
        border_color="#e0e0e0" />
    <hover 
        border_color="#3498db"
        shadow="0 4 12 #00000020" />
</style>

<container class="card">
    <text value="Hover over me!" />
</container>
```

**Pattern 2: Button Press Feedback**

```xml
<style name="action_button">
    <base background="#3498db" padding="12 24" />
    <hover background="#5dade2" />
    <active background="#2874a6" />
</style>

<button label="Click" class="action_button" on_click="action" />
```

**Pattern 3: Disabled State**

```xml
<style name="form_button">
    <base background="#2ecc71" color="#ffffff" />
    <hover background="#52be80" />
    <disabled opacity="0.5" />
</style>

<button 
    label="Submit" 
    class="form_button" 
    enabled="{form_valid}" 
    on_click="submit" />
```

**Pattern 4: Focus Highlight for Inputs**

```xml
<style name="input_field">
    <base 
        background="#f8f9fa"
        border_width="2"
        border_color="#dee2e6"
        border_radius="4" />
    <hover border_color="#adb5bd" />
    <focus 
        border_color="#3498db"
        background="#ffffff" />
</style>

<text_input 
    value="{username}" 
    class="input_field"
    placeholder="Enter username..." 
    on_input="update_username" />
```

#### Advanced: Multiple Buttons

You can reuse the same style class across multiple widgets:

```xml
<styles>
    <style name="btn_primary">
        <base background="#3498db" color="#fff" padding="10 20" />
        <hover background="#5dade2" />
        <active background="#2874a6" />
    </style>
    
    <style name="btn_danger">
        <base background="#e74c3c" color="#fff" padding="10 20" />
        <hover background="#ec7063" />
        <active background="#c0392b" />
    </style>
</styles>

<row spacing="10">
    <button label="Save" class="btn_primary" on_click="save" />
    <button label="Delete" class="btn_danger" on_click="delete" />
    <button label="Cancel" class="btn_primary" on_click="cancel" />
</row>
```

#### Troubleshooting

**Problem:** Hover state doesn't show

**Solutions:**
- Verify the style class name matches exactly (case-sensitive)
- Check that the `<styles>` section is before the widget definitions
- Ensure the widget type supports hover (see table above)
- Use `dampen check` to validate XML syntax

**Problem:** Active state appears instantly

**Solution:**
- This is correct! Active state shows when mouse button is pressed down
- Release the mouse to return to hover state

**Problem:** Inline styles override state styles

**Solution:**
- This is by design (see Style Precedence above)
- Remove inline styles from the widget if you want class states to apply
- Or define the inline style in the style class instead

#### Performance

State styling is highly optimized:
- **Resolution time**: < 1ms per widget (imperceptible)
- **Memory overhead**: ~50 bytes per widget with states
- **Render performance**: No impact (< 16ms frame time maintained)

State resolution happens only when the widget enters a new state, not on every frame.

#### Examples

Study these examples to see state styling in action:

```bash
# Complete styling showcase with state variants
dampen run -p styling

# Interactive counter with button states
dampen run -p counter

# Todo app with focus/hover states on inputs
dampen run -p todo-app
```

#### Learn More

- **Complete Guide**: [STYLING.md](STYLING.md) - All styling features
- **Implementation Details**: [WIDGETS_STATE_IMPLEMENTATION.md](WIDGETS_STATE_IMPLEMENTATION.md) - For contributors
- **XML Schema**: [XML_SCHEMA.md](XML_SCHEMA.md) - Full syntax reference

---

### Debugging Build Issues

If your build fails:

1. **Validate XML first:**
   ```bash
   dampen check
   ```

2. **Check for common issues:**
   - Handler names in XML match registered handlers
   - Field names in `{bindings}` match model fields
   - Model derives `UiModel`, `Serialize`, `Deserialize`
   - XML file path in `#[dampen_ui("...")]` is correct

3. **Inspect the IR:**
   ```bash
   dampen inspect src/ui/window.dampen
   ```

4. **Build with verbose output:**
   ```bash
   dampen build -v
   ```

---

### Testing Your Application

Create tests in `tests/`:

```rust
#[test]
fn test_model_initialization() {
    let model = Model::default();
    assert_eq!(model.message, "");
}
```

Run tests:

```bash
# All tests
dampen test

# Specific test
dampen test test_model_initialization

# With output
dampen test -- --nocapture
```

---

## Working with Workspaces

If your project has multiple packages:

### Project Structure

```
my-workspace/
├── Cargo.toml          # Workspace manifest
├── app-ui/
│   ├── Cargo.toml
│   └── src/
└── app-backend/
    ├── Cargo.toml
    └── src/
```

### Running Specific Packages

```bash
# Run specific package
dampen run -p app-ui

# Build specific package
dampen build -p app-backend

# Test specific package
dampen test -p app-ui

# Release build for specific package
dampen release -p app-ui
```

### Building Multiple Packages

```bash
# Build all packages (from workspace root)
dampen build

# Release all packages
dampen release
```

---

## Troubleshooting

### CLI Not Found

**Problem:** `dampen: command not found`

**Solution:**
```bash
# Ensure cargo bin is in PATH
export PATH="$HOME/.cargo/bin:$PATH"

# Reinstall if needed
cargo install dampen-cli --force
```

---

### Hot-Reload Not Working

**Problem:** Changes to `.dampen` files don't appear in running app

**Solutions:**
1. Ensure you're using `dampen run` (not `dampen build`)
2. Check file watcher permissions
3. Try restarting the application
4. Check console for error messages

---

### Build Fails

**Problem:** Build fails with errors

**Solutions:**

1. **Validate XML:**
   ```bash
   dampen check
   ```

2. **Check handler registration:**
   - Every handler in XML must be registered in Rust
   - Handler names are case-sensitive

3. **Check model derivations:**
   ```rust
   #[derive(UiModel, Serialize, Deserialize, Clone, Debug)]
   pub struct Model { ... }
   ```

4. **Check XML file path:**
   ```rust
   #[dampen_ui("window.dampen")]  // Relative to .rs file
   ```

---

### Tests Fail

**Problem:** `dampen test` fails

**Solutions:**

1. **Run with verbose output:**
   ```bash
   dampen test -v
   ```

2. **Run specific test:**
   ```bash
   dampen test my_failing_test -- --nocapture
   ```

3. **Check test dependencies:**
   - Ensure test features are enabled in `Cargo.toml`

---

### Binding Errors

**Problem:** UI doesn't display bound values

**Solutions:**

1. **Check field names:**
   - `{field}` in XML must exactly match `pub field` in struct

2. **Check model derivation:**
   ```rust
   #[derive(UiModel)]  // Required for bindings
   ```

3. **Check for typos:**
   - Field names are case-sensitive
   - No extra spaces in `{field}` syntax

---

### Performance Issues

**Problem:** Application is slow

**Solutions:**

1. **Use release build for testing performance:**
   ```bash
   dampen release
   ./target/release/my-app
   ```

2. **Profile the application:**
   - Development mode (`dampen run`) is not optimized
   - Always benchmark with release builds

3. **Check for excessive re-renders:**
   - Review your update logic
   - Minimize state changes

---

## Getting Help

### Documentation

- **Dampen Core:** [docs.rs/dampen-core](https://docs.rs/dampen-core)
- **Iced Framework:** [docs.rs/iced](https://docs.rs/iced)
- **Examples:** [examples/](../examples/) directory
- **XML Schema:** [XML_SCHEMA.md](XML_SCHEMA.md)

### Examples

Study the included examples:

```bash
# Simple hello world
dampen run -p hello-world

# Interactive counter
dampen run -p counter

# Complex todo app
dampen run -p todo-app

# Widget showcase
dampen run -p widget-showcase
```

### Community

- **Issues:** [GitHub Issues](https://github.com/dampen-ui/dampen/issues)
- **Discussions:** [GitHub Discussions](https://github.com/dampen-ui/dampen/discussions)

---

## Quick Reference

### Essential Commands

| Task | Command |
|------|---------|
| Create project | `dampen new my-app` |
| Add UI window | `dampen add --ui <name>` |
| Run with hot-reload | `dampen run` |
| Validate XML | `dampen check` |
| Build debug | `dampen build` |
| Build release | `dampen release` |
| Run tests | `dampen test` |
| Inspect IR | `dampen inspect <file>` |

### Common Flags

| Flag | Purpose | Works With |
|------|---------|------------|
| `-p <pkg>` | Specify package | run, build, release, test |
| `-v` | Verbose output | All commands |
| `--features` | Enable features | build, release, test |
| `--` | Pass args through | run, test |

---

## Next Steps

Now that you understand the Dampen CLI:

1. **Create your first app:** `dampen new my-app`
2. **Read the examples:** Study `examples/` directory
3. **Learn XML syntax:** Read [XML_SCHEMA.md](XML_SCHEMA.md)
4. **Build something awesome!**

---

**Happy coding with Dampen!** 🚀

*For framework contributors, see [CONTRIBUTING.md](CONTRIBUTING.md) for workspace development workflows.*
