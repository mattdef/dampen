<div align="center">

<img src="assets/logo_dampen.webp" height="100" alt="Dampen Logo" />  

# Dampen

[![Crates.io](https://img.shields.io/crates/v/dampen-cli.svg)](https://crates.io/crates/dampen-cli)
[![Documentation](https://docs.rs/dampen-core/badge.svg)](https://docs.rs/dampen-core)
[![License: MIT/Apache-2.0](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE-MIT)
[![Rust Version](https://img.shields.io/badge/rustc-1.88+-lightgray.svg)](https://rust-lang.org)

**Declarative UI framework for Rust with Iced backend, hot reloading and advanced styling.**

Dampen allows you to define your user interface in XML and render it via Iced.

<img src="assets/hot-reload-demo.gif" width="850" alt="Exemple" />  

</div>

---

> **⚠️ DEVELOPMENT STATUS**
> 
> **Dampen is currently under active development and is NOT ready for production use.**
> 
> The framework is functional and can be tested for experimentation, learning, and contributing to its development. However, the API is unstable and subject to breaking changes. Features may be incomplete, and there may be bugs or performance issues.
> 
> **Use Dampen for:**
> - ✅ Experimentation and learning
> - ✅ Contributing to development
> - ✅ Testing and providing feedback
> - ✅ Prototype applications
> 
> **Do NOT use Dampen for:**
> - ❌ Production applications
> - ❌ Mission-critical systems
> - ❌ Applications requiring API stability
> 
> We welcome your feedback and contributions! See [CONTRIBUTING.md](docs/CONTRIBUTING.md) to get involved.

---

## Features

- ✅ **Declarative XML definitions**
- ✅ **Advanced styling system** (themes, classes, state styles)
- ✅ **Responsive design** with breakpoints (mobile, tablet, desktop)
- ✅ **Type-safe event handlers**
- ✅ **Expression evaluation** in XML attributes
- ✅ **Full Iced widget support** (text, buttons, inputs, layouts, etc.)
- ✅ **Radio button groups** with single-selection behavior
- ✅ **Data binding** with `#[derive(UiModel)]`
- ✅ **CLI validation** tools for syntax checking
- ✅ **Dual-mode architecture**: Hot-reload for development, codegen for production
- ✅ **Hot-reload support**: See UI changes instantly without recompiling

## Installation

```bash
cargo install dampen-cli
```

## Quick Start

### Create a New Project

Use the CLI command to scaffold a new Dampen project:

```bash
# Create a new project
dampen new my-app

# Navigate to the project
cd my-app

# Run the application
dampen run
```

### Add New UI Windows

**NEW!** Quickly scaffold new UI windows with the `dampen add` command:

```bash
# Add a settings window
dampen add --ui settings

# Add a window in custom directory
dampen add --ui order_form --path "src/ui/orders"
```

This creates:
- `settings.rs` - Model, handlers, and AppState setup
- `settings.dampen` - Basic UI layout with data binding example

Then add to `src/ui/mod.rs`:
```rust
pub mod settings;
```

**Benefits:**
- ✅ Production-ready code in < 1 second
- ✅ Consistent structure across windows
- ✅ Prevents accidental overwrites
- ✅ Reduces manual boilerplate

See `dampen add --help` for more options.

### Project Validation

```bash
# Validate XML syntax and widget names
dampen check

# Build the project
dampen build

# Inspect generated IR
dampen inspect src/ui/window.dampen
```

## Advanced Features

### Data Binding

```rust
#[derive(UiModel, Default, Serialize, Deserialize, Clone)]
struct Model {
    count: i32,
    name: String,
    items: Vec<TodoItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TodoItem {
    id: usize,
    text: String,
    completed: bool,
}
```

### Advanced Theming System

Define themes in `src/ui/theme/theme.dampen` for complete application theming:

```xml
<?xml version="1.0" encoding="UTF-8" ?>
<dampen version="1.0">
    <themes>
        <theme name="light">
            <palette
                primary="#3498db"
                secondary="#2ecc71"
                success="#27ae60"
                warning="#f39c12"
                danger="#e74c3c"
                background="#ecf0f1"
                surface="#ffffff"
                text="#2c3e50"
                text_secondary="#7f8c8d" />
            <typography font_family="Inter, sans-serif" font_size_base="16" />
            <spacing unit="8" />
        </theme>

        <theme name="dark" extends="light">
            <palette
                background="#1a1a2e"
                surface="#16213e"
                text="#eaeaea"
                text_secondary="#a0a0a0" />
        </theme>
    </themes>

    <default_theme name="light" />
    <follow_system enabled="true" />
</dampen>
```

**Runtime Theme Switching:**
```xml
<button label="Dark Mode" on_click="set_theme('dark')" />
<button label="Light Mode" on_click="set_theme('light')" />
```

**Theme Features:**
- **Theme Inheritance** - Extend themes with `extends="base_theme"`
- **System Detection** - Auto-detect dark/light mode with `<follow_system enabled="true" />`
- **Hot-Reload** - Edit `theme.dampen` and see changes instantly in development
- **Runtime Switching** - Switch themes without restarting

See [STYLING.md](docs/STYLING.md) for complete documentation.

### Reusable Style Classes

```xml
<styles>
    <style name="btn_primary">
        <base
            background="#3498db"
            color="#ffffff"
            padding="8 16"
            border_radius="6"
            border_width="0" />
        <hover background="#5dade2" />
        <active background="#2874a6" />
        <disabled opacity="0.5" />
    </style>
    
    <style name="btn_danger">
        <base
            background="#e74c3c"
            color="#ffffff"
            padding="8 16"
            border_radius="6" />
        <hover background="#ec7063" />
    </style>
</styles>

<button class="btn_primary" label="Submit" on_click="submit" />
<button class="btn_danger" label="Delete" on_click="delete" />
```

### Available Widgets

| Widget | Description |
|--------|-------------|
| `text` | Text display |
| `button` | Interactive button |
| `text_input` | Text input field |
| `checkbox` | Checkbox |
| `toggler` | Toggle switch |
| `pick_list` | Dropdown list |
| `radio` | Radio button |
| `column` | Vertical layout |
| `row` | Horizontal layout |
| `scrollable` | Scrollable area |
| `container` | Container |
| `for` | Dynamic loop |
| `grid` | Grid layout |
| `progress_bar` | Progress bar |
| `svg` | SVG image |
| `tooltip` | Tooltip |

## Dual-Mode Architecture

Dampen supports two compilation modes optimized for different use cases:

### Interpreted Mode (Development)

**Enabled by default in development builds**

- ✅ **Fast iteration**: Hot-reload UI changes without recompiling
- ✅ **Runtime parsing**: XML loaded and parsed at application startup
- ✅ **Instant feedback**: See changes in <300ms
- ✅ **Debugging friendly**: Error overlays with detailed messages

```bash
# Development mode (automatic)
dampen run

# Debug build (interpreted)
dampen build
```

**Hot-reload example:**

```rust
use dampen_dev::watch_files;

fn subscription(app: &App) -> Subscription<Message> {
    watch_files(vec![PathBuf::from("src/ui/window.dampen")], "xml")
        .map(|_| Message::ReloadUI)
}
```

### Codegen Mode (Production)

**Enabled with --release flag**

- ✅ **Zero runtime overhead**: All XML parsed at compile time
- ✅ **Optimal performance**: Direct widget construction
- ✅ **Smaller binaries**: No runtime parser included
- ✅ **Build-time validation**: Catch errors before deployment

```bash
# Release run (codegen)
dampen run --release

# Release build (codegen)
dampen build --release

# Alternative: release command (alias for build --release)
dampen release
```

**How it works:**

1. `build.rs` processes `.dampen` files at compile time
2. Generated Rust code embedded via macros
3. No runtime XML parsing required

### Mode Selection

Mode selection is **automatic** based on `--release` flag:

| CLI Command | Mode | Use Case |
|-------------|------|----------|
| `dampen new` | - | Create new project |
| `dampen add` | - | Scaffold new UI window |
| `dampen run` | Interpreted | Development with hot-reload |
| `dampen run --release` | Codegen | Production testing |
| `dampen build` | Interpreted | Debug builds |
| `dampen build --release` | Codegen | Production builds (optimized) |
| `dampen release` | Codegen | Alias for `build --release` |
| `dampen test` | Interpreted | Fast test iteration |
| `dampen check` | - | Validate XML syntax |

**Advanced usage:**

```bash
# Enable additional features
dampen release --features tokio,logging

# Run tests in release mode
dampen test --release

# Verbose output
dampen build -v
```

> **Note**: By default, `dampen run` and `dampen build` use interpreted mode for fast development.
> Use `--release` flag to enable codegen mode for production builds.

## Architecture

### Crate Structure

```
crates/
├── dampen-core/           # XML parser, IR, traits (no Iced dependency)
├── dampen-macros/         # Macros #[derive(UiModel)], #[dampen_ui]
├── dampen-iced/           # Iced backend implementation
├── dampen-dev/            # Development mode tooling for Dampen
└── dampen-cli/            # Developer CLI (build, check, inspect)

```

### Core Principles

1. **Declarative-First**: XML is the source of truth for UI structure
2. **Type Safety**: No type erasure for messages/state
3. **Production Mode**: Static code generation for deployments
4. **Backend-Agnostic**: Core crate has no Iced dependency
5. **Test-First**: TDD for all features


## Examples

See the [examples/](examples/) directory for progressive demonstrations:

| Example | Features |
|---------|----------|
| **hello-world** | Minimal static UI rendering |
| **counter** | Interactive event handlers |
| **todo-app** | Complete data binding with lists |
| **styling** | Themes, classes, state styles |
| **responsive** | Responsive design with breakpoints |
| **settings** | Multiple views and navigation |
| **widget-showcase** | Demonstration of all widgets |

## CLI Commands

```bash
# Generate production code
dampen build --ui ui --output src/ui_generated.rs

# Validate UI files without running
dampen check --ui ui

# Inspect IR or generated code
dampen inspect --file ui/main.dampen
dampen inspect --file ui/main.dampen --codegen --handlers increment,decrement
```

## Documentation

- **[API Documentation](https://docs.rs/dampen-core)** - Complete Rustdoc
- **[XML Schema Reference](docs/XML_SCHEMA.md)** - Widgets and attributes
- **[Styling Guide](docs/STYLING.md)** - Themes, classes, state styles
- **[Examples](examples/README.md)** - Progressive example projects

## Contributing

We welcome contributions from the community! Whether you're fixing bugs, adding features, improving documentation, or reporting issues, your help is appreciated.

**Before contributing, please read our [Contributing Guide](docs/CONTRIBUTING.md)** which covers:

- Code of conduct and community standards
- Setting up your development environment
- Coding standards and style guidelines
- Testing requirements (TDD is mandatory)
- Pull request process and commit message format
- How to report issues and request features

**Quick start for contributors:**

```bash
# Fork and clone the repository
git clone https://github.com/YOUR_USERNAME/dampen.git
cd dampen

# Build and test
cargo build --workspace
cargo test --workspace
cargo clippy --workspace -- -D warnings

# Try the examples
dampen run -p hello-world
```

All contributions must:
- ✅ Pass all tests (`cargo test --workspace`)
- ✅ Pass clippy lints (`cargo clippy --workspace -- -D warnings`)
- ✅ Be properly formatted (`cargo fmt --all`)
- ✅ Include tests for new functionality
- ✅ Update documentation as needed

See [docs/CONTRIBUTING.md](docs/CONTRIBUTING.md) for complete details.

## License

This project is dual-licensed under Apache 2.0 or MIT, at your option.

---

**Built with ❤️ using Rust and Iced**
