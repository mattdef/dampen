# Dampen Development Guidelines

Auto-generated from all feature plans. Last updated: 2025-12-30

## Active Technologies
- File-based (XML UI definitions, optional separate style files), serialized state in `.dampen-state.json`
- Rust Edition 2024, MSRV 1.85, Stable Rust (no nightly features) 
- Rust Edition 2024, MSRV 1.85 + Iced 0.14+ (already in workspace) 
- JSON state files via serde_json (existing pattern) 
- Rust Edition 2024, MSRV 1.85 + Iced 0.14 (with `image` feature enabled), dampen-core
- Rust Edition 2024, MSRV 1.85 (aligned with Edition 2024) + `dampen-core`, `dampen-macros`, `dampen-iced`, `iced` 0.14+ 
- Rust Edition 2024, MSRV 1.85 (aligned with Edition 2024) + `iced` 0.14+ (reference backend), `dampen-core`, `dampen-iced` 
- Rust Edition 2024, MSRV 1.85+ + roxmltree (XML parsing), proc-macro2/syn/quote (macro generation), Cargo build.rs mechanism 
- Rust Edition 2024, MSRV 1.85 (aligned with Edition 2024) + dampen-core (parser, IR), serde_json (JSON handling), clap (CLI) 
- JSON files for handler registry (`--handlers`) and model info (`--model`) 
- File-based (`.dampen` XML UI definitions, optional `.dampen-state.json` for state persistence) 
- Rust Edition 2024, MSRV 1.85 (per constitution) + roxmltree 0.19+ (XML parsing), thiserror (errors) 
- Rust Edition 2024, MSRV 1.85 (aligned with Dampen constitution) 
- Rust Edition 2024, MSRV 1.85 (aligned with Dampen constitution) + `clap` 4.0+ (CLI parsing), `std::fs` (file operations), existing dampen-cli infrastructure 
- File-based templates (`.rs.template` and `.dampen.template` files) stored in `crates/dampen-cli/templates/add/` 

- **Language**: Rust Edition 2024, MSRV 1.85 (minimum for Edition 2024, enables LazyLock from std)
- **UI Framework**: `iced` 0.14+
- **XML Parsing**: `roxmltree` 0.19+
- **Serialization**: `serde`, `serde_json` 1.0+
- **Proc Macros**: `syn` 2.0+, `quote` 2.0+, `proc-macro2` 2.0+
- **CLI**: `clap` 4.0+
- **Testing**: `proptest` 1.0+, `insta` 1.0+ (snapshots)

## Project Structure

```text
Cargo.toml                    # Workspace manifest

crates/
├── dampen-core/              # Parser, AST, IR, trait definitions
│   ├── src/
│   │   ├── lib.rs
│   │   ├── parser/           # XML parsing (mod.rs, lexer.rs, error.rs)
│   │   ├── ir/               # Intermediate representation (mod.rs, node.rs, expr.rs, span.rs)
│   │   ├── expr/             # Expression AST and evaluation (mod.rs, ast.rs, eval.rs)
│   │   ├── binding/          # UiBindable trait
│   │   ├── handler/          # Handler registry
│   │   ├── codegen/          # Code generation
│   │   └── traits/           # Backend abstraction (mod.rs, backend.rs)
│   └── tests/
│
├── dampen-macros/            # Proc macros (#[derive(UiModel)], #[dampen_ui])
│   ├── src/
│   │   ├── lib.rs
│   │   ├── ui_model.rs
│   │   └── ui_loader.rs
│   └── tests/
│
├── dampen-iced/              # Iced backend implementation
│   ├── src/
│   │   ├── lib.rs
│   │   ├── widgets/          # IR-to-Iced widget mapping
│   │   ├── theme.rs
│   │   └── commands.rs
│   └── tests/
│
└── dampen-cli/               # Developer CLI
    ├── src/
    │   ├── main.rs
    │   ├── commands/         # build.rs, check.rs, inspect.rs
    │   └── config.rs
    └── tests/

examples/
├── hello-world/              # Minimal static XML example
├── counter/                  # Interactive handlers example
├── todo-app/                 # Full bindings example
└── full-demo/                # Complete showcase

specs/
└── 001-framework-technical-specs/
    ├── spec.md               # Feature specification
    ├── plan.md               # Implementation plan
    ├── tasks.md              # Task breakdown
    ├── research.md           # Technology decisions
    ├── data-model.md         # IR type definitions
    ├── quickstart.md         # Developer guide
    └── contracts/
        └── xml-schema.md     # XML element specification
```

## Commands

### For Application Developers (User Workflow)

Application developers using Dampen should use the `dampen` CLI exclusively:

```bash
# Create a new project
dampen new my-app

# Run in development mode (with hot-reload)
dampen run

# Build for development (debug mode)
dampen build

# Build for production (optimized)
dampen release

# Run tests
dampen test

# Validate XML syntax
dampen check

# Inspect generated IR
dampen inspect src/ui/window.dampen

# Run examples
dampen run -p hello-world
dampen run -p counter
dampen run -p todo-app
```

See [docs/USAGE.md](docs/USAGE.md) for complete user documentation.

### For Framework Contributors (Developer Workflow)

Contributors working on the Dampen framework itself use `cargo` directly:

```bash
# Build all crates
cargo build --workspace

# Run all tests
cargo test --workspace

# Run specific crate tests
cargo test -p dampen-core
cargo test -p dampen-macros
cargo test -p dampen-iced
cargo test -p dampen-cli

# Linting
cargo clippy --workspace -- -D warnings

# Formatting
cargo fmt --all
cargo fmt --all -- --check  # CI check

# Documentation
cargo doc --workspace --no-deps --open

# Benchmarks (when implemented)
cargo bench -p dampen-core
```

See [docs/CONTRIBUTING.md](docs/CONTRIBUTING.md) for contributor documentation.

## Code Style

### Rust Conventions

- **Edition**: 2024
- **MSRV**: 1.85 (minimum for Edition 2024, enables LazyLock from std)
- **Formatting**: Default rustfmt configuration
- **Linting**: `cargo clippy` with `-D warnings`
- **Documentation**: All public items must have rustdoc comments
- **Unsafe**: Zero unsafe in generated code unless explicitly justified

### Naming Conventions

- **Crates**: `dampen-{module}` (kebab-case)
- **Types**: PascalCase (`WidgetNode`, `BindingExpr`)
- **Functions**: snake_case (`parse_xml`, `evaluate_binding`)
- **Constants**: SCREAMING_SNAKE_CASE
- **Modules**: snake_case matching file names

### Error Handling

- Use `Result<T, E>` for fallible operations
- Custom error types with `thiserror` derive
- Include `Span` (line/column) in all parse errors
- Error messages must be actionable with fix suggestions

### Testing Requirements

- TDD mandatory (Constitution Principle V)
- Contract tests for parser: XML input → expected IR
- Integration tests for full pipeline
- Property-based tests for parser edge cases
- Snapshot tests for code generation
- Release-mode compilation: Test `cargo build --release` for macro-generated code
- Target: >90% coverage for dampen-core

## Architecture Principles

### Constitution (v1.0.0)

1. **Declarative-First**: XML is the source of truth for UI structure
2. **Type Safety Preservation**: No runtime type erasure for messages/state
3. **Production Mode**: Static code generation for deployments
4. **Backend Abstraction**: Core crate has no Iced dependency
5. **Test-First Development**: Tests define contracts before implementation

### Crate Dependencies

```text
dampen-core (no backend deps)
    ↑
    ├── dampen-macros (proc-macro, depends on core)
    └── dampen-iced (depends on core + iced)
            ↑
            └── dampen-cli (depends on all above)
```

### Key Traits

```rust
// Backend abstraction (dampen-core/src/traits/backend.rs)
pub trait Backend {
    type Widget<'a>;
    type Message: Clone + 'static;
    fn text(&self, content: &str) -> Self::Widget<'_>;
    fn button(&self, label: Self::Widget<'_>, on_press: Option<Self::Message>) -> Self::Widget<'_>;
    // ... other widgets
}

// Binding abstraction (dampen-core/src/binding/mod.rs)
pub trait UiBindable: Serialize + for<'de> Deserialize<'de> {
    fn get_field(&self, path: &[&str]) -> Option<BindingValue>;
    fn available_fields() -> Vec<String>;
}
```

### AppState Pattern (006-auto-ui-loading)

The `AppState<M>` struct provides a unified way to manage UI state:

```rust
use dampen_core::AppState;

// Simple usage (no model)
let state = AppState::<()>::new(document);

// With model
let state = AppState::with_model(document, my_model);

// With handlers
let state = AppState::with_handlers(document, handler_registry);
```

### Auto-Loading UI Files

Use the `#[dampen_ui]` macro to automatically load XML files:

```rust
// src/ui/app.rs
use dampen_macros::{dampen_ui, UiModel};
use dampen_core::AppState;

#[derive(UiModel)]
pub struct Model { count: i32 }

#[dampen_ui("app.dampen")]
mod _app {}

pub fn create_app_state() -> AppState<Model> {
    let document = _app::document();
    let handlers = create_handlers();
    AppState::with_handlers(document, handlers)
}
```

File structure:
```
src/
└── ui/
    ├── mod.rs          # Export the app module
    ├── app.rs          # UI code with #[dampen_ui] macro
    └── app.dampen     # XML UI definition
```

## Performance Budgets

| Metric | Target |
|--------|--------|
| XML parse time | < 10ms for 1000 widgets |
| Code generation | < 5s for typical application |
| Runtime memory | < 50MB baseline |

## Recent Changes

**002-cli-add-ui-command: ✅ COMPLETE (2026-01-13)**
- Implemented `dampen add --ui` command for scaffolding UI windows
- 253 tests passing (94 unit + 16 integration)
- Zero clippy warnings, comprehensive documentation
- Reduces window creation time from ~5 minutes to < 1 second
- See `specs/002-cli-add-ui-command/COMPLETION.md` for full details

- 001-dampen-app-macro: Added Rust Edition 2024, MSRV 1.85 (aligned with Dampen constitution)
- 001-schema-version-validation: Added Rust Edition 2024, MSRV 1.85 (per constitution) + roxmltree 0.19+ (XML parsing), thiserror (errors)

**Phase 7 Complete (006-auto-ui-loading):**

  - Implemented `#[derive(UiModel)]` macro with field accessors
  - Created `UiBindable` trait and `BindingValue` enum
  - Implemented expression evaluator for field access, method calls, binary ops, conditionals
  - Added support for primitives, Option<T>, Vec<T>, #[ui_skip], #[ui_bind]
  - Working `todo-app` example demonstrating bindings
  - All tests passing (14 tests total), clippy clean

  - Implemented `#[ui_handler]` attribute macro with signature validation
  - Created `HandlerRegistry` with support for simple, value, and command handlers
  - Added Iced backend integration for event dispatch
  - Working `counter` example demonstrating interactive handlers
  - All tests passing, clippy clean

**Phase 8 Complete (007-add-radio-widget):**

  - Added Radio widget to WidgetKind enum
  - Implemented radio button parsing with label, value, selected, disabled attributes
  - Added full Iced radio widget rendering in DampenWidgetBuilder
  - Implemented single-selection behavior (inherent to Iced radio API)
  - Added selection change event dispatch via on_select handler
  - Implemented default selection support via selected attribute binding
  - Added disabled state support with static/dynamic bindings
  - Custom value types supported via UiBindable (enums, Option<String>, etc.)
  - 52 radio tests passing across all crates (parsing, rendering, selection, events, default, disabled, value types)
  - All tests passing, radio code clippy clean


<!-- MANUAL ADDITIONS START -->

## Development Workflow

### Starting a New Feature

1. Create specification in `specs/{NNN}-{feature-name}/`
2. Run `/speckit.plan` to generate implementation plan
3. Run `/speckit.tasks` to generate task breakdown
4. Follow phases in order (Setup → Foundational → User Stories → Polish)

### Commit Guidelines

- Atomic commits per task or logical group
- Format: `feat(crate): description` or `fix(crate): description`
- Reference task IDs in commits: `T001`, `T002`, etc.

### PR Requirements

- All tests pass (`cargo test --workspace`)
- Clippy clean (`cargo clippy --workspace -- -D warnings`)
- Formatted (`cargo fmt --all -- --check`)
- Documentation updated if public API changed

### Current Status: Phase 8 Complete ✓

**Implemented Components:**
- `dampen-core/src/state/mod.rs`: AppState struct with constructors
- `dampen-core/src/binding/`: UiBindable trait, BindingValue enum
- `dampen-macros/src/ui_loader.rs`: #[dampen_ui] macro for auto-loading
- `dampen-macros/tests/auto_loading_tests.rs`: Contract tests
- `dampen-core/tests/appstate_tests.rs`: Contract tests
- `examples/hello-world/`: Minimal auto-loading example
- `examples/counter/`: Migrated to auto-loading pattern
- `examples/todo-app/`: Migrated to auto-loading pattern
- `examples/settings/`: New example demonstrating multiple views

**Auto-Loading Features:**
- ✅ #[dampen_ui] macro for automatic XML loading
- ✅ LazyLock for thread-safe lazy initialization
- ✅ Multiple views support (app.dampen, settings.dampen)
- ✅ AppState with Model and HandlerRegistry support

**Validation Features:**
- ✅ `dampen check` validates XML syntax and widget names
- ✅ Clear error messages with span information
- ✅ Exit code 0 for success, 1 for failure
- ✅ File walking with `.dampen` extension filtering
- ✅ Comprehensive test coverage for validation

**Radio Widget Features (Phase 8 - 007-add-radio-widget):**
- ✅ Radio widget XML parsing (label, value, selected, disabled attributes)
- ✅ Single-selection behavior (inherent to Iced radio API)
- ✅ Selection change events via on_select handler
- ✅ Default selection support via selected attribute binding
- ✅ Disabled state with static/dynamic bindings
- ✅ Custom value types via UiBindable (enums, Option<String>)
- ✅ 52 comprehensive tests (parsing, rendering, selection, events, default, disabled, value types)
- ✅ Full Iced backend integration
- ✅ Clippy clean, all tests passing

### Creating a New Project

Use the CLI to scaffold a new Dampen project:

```bash
# Create a new project
dampen new my-app

# Navigate to the project
cd my-app

# Run the application
dampen run
```

The `dampen new` command creates a complete project structure:

```
my-app/
├── Cargo.toml              # Project dependencies
├── README.md               # Getting started guide
├── build.rs                # Code generation (XML → Rust)
├── src/
│   ├── main.rs             # Application entry point
│   └── ui/
│       ├── mod.rs          # UI module exports
│       ├── window.rs       # UI model and handlers
│       └── window.dampen  # Declarative UI definition (XML)
└── tests/
    └── integration.rs      # Integration tests
```

**Key files:**

| File | Purpose |
|------|---------|
| `src/ui/window.dampen` | XML UI definition with widgets, bindings, and handlers |
| `src/ui/window.rs` | Model definition with `#[derive(UiModel)]`, handlers registry |
| `src/main.rs` | Application orchestration (view, update, subscriptions) |
| `build.rs` | Compiles `.dampen` XML files to Rust code at build time |

**Generated example UI:**

```xml
<dampen>
    <column padding="40" spacing="20">
        <text value="Hello, Dampen!" size="32" weight="bold" />
        <button label="Click me!" on_click="greet" />
        <text value="{message}" size="24" />
    </column>
</dampen>
```

**Project validation:**

```bash
# Validate XML syntax and widget names
dampen check

# Build the project
dampen build

# Inspect the generated IR
dampen inspect src/ui/window.dampen
```

### Adding New UI Windows

**NEW (Phase 8 Complete - 2026-01-13):** Use `dampen add` to scaffold additional UI windows:

```bash
# Add a settings window in default location (src/ui/)
dampen add --ui settings

# Add a window in custom directory
dampen add --ui admin_panel --path "src/ui/admin"

# Window names are auto-converted to snake_case
dampen add --ui UserProfile
# → Creates: user_profile.rs, user_profile.dampen
```

**What it creates:**

```
src/ui/
├── settings.rs         # Model with #[derive(UiModel)], handlers
└── settings.dampen    # XML UI with basic layout
```

**Generated Rust module:**

```rust
use dampen_core::{AppState, HandlerRegistry};
use dampen_macros::{dampen_ui, UiModel};

#[derive(Default, Clone, UiModel)]
pub struct Model {
    pub message: String,
}

#[dampen_ui("settings.dampen")]
mod _settings {}

pub fn create_app_state() -> AppState<Model> {
    let document = _settings::document();
    let handlers = create_handler_registry();
    AppState::with_handlers(document, handlers)
}

pub fn create_handler_registry() -> HandlerRegistry {
    // Handler registration
}
```

**After generation:**

1. Add module to `src/ui/mod.rs`:
   ```rust
   pub mod settings;
   ```

2. Validate:
   ```bash
   dampen check
   ```

**Validation & Security:**
- ✅ Ensures you're in a Dampen project
- ✅ Validates window name (Rust identifier, not reserved keyword)
- ✅ Prevents overwriting existing files
- ✅ Rejects absolute paths and directory escaping
- ✅ Path normalization (handles `./`, `//`, etc.)

**Performance:**
- Creates files in < 0.1 seconds
- Reduces window creation time from ~5 minutes to < 1 second

**See also:** `specs/002-cli-add-ui-command/COMPLETION.md` for full implementation details.

<!-- MANUAL ADDITIONS END -->
