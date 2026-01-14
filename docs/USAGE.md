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
