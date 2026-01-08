# Gravity Development Guidelines

Auto-generated from all feature plans. Last updated: 2025-12-30

## Active Technologies
- File-based (XML UI definitions, optional separate style files), serialized state in `.gravity-state.json` (002-layout-theming-styling)
- Rust Edition 2024, Stable Rust (no nightly features) (003-widget-builder)
- N/A (runtime interpretation, no persistence required) (003-widget-builder)
- Rust Edition 2024, MSRV 1.75 + Iced 0.14+ (already in workspace) (004-advanced-widgets-todo)
- JSON state files via serde_json (existing pattern) (004-advanced-widgets-todo)
- Rust Edition 2021, MSRV 1.75 + Iced 0.14 (with `image` feature enabled), gravity-core (005-implement-real-widgets)
- N/A (UI widgets only) (005-implement-real-widgets)
- Rust Edition 2024, MSRV 1.75 (per constitution) + `gravity-core`, `gravity-macros`, `gravity-runtime`, `gravity-iced`, `iced` 0.14+ (006-auto-ui-loading)
- N/A (compile-time XML loading, no runtime persistence required for this feature) (006-auto-ui-loading)

- **Language**: Rust Edition 2024, MSRV stable (no nightly features in public API)
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
├── gravity-core/             # Parser, AST, IR, trait definitions
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
├── gravity-macros/           # Proc macros (#[derive(UiModel)], #[gravity_ui])
│   ├── src/
│   │   ├── lib.rs
│   │   ├── ui_model.rs
│   │   └── ui_loader.rs
│   └── tests/
│
├── gravity-runtime/          # Interpreter, state management, error handling
│   ├── src/
│   │   ├── lib.rs
│   │   ├── interpreter.rs
│   │   ├── state.rs
│   │   └── overlay.rs
│   └── tests/
│
├── gravity-iced/             # Iced backend implementation
│   ├── src/
│   │   ├── lib.rs
│   │   ├── widgets/          # IR-to-Iced widget mapping
│   │   ├── theme.rs
│   │   └── commands.rs
│   └── tests/
│
└── gravity-cli/              # Developer CLI
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

```bash
# Build all crates
cargo build --workspace

# Run all tests
cargo test --workspace

# Run specific crate tests
cargo test -p gravity-core
cargo test -p gravity-macros
cargo test -p gravity-runtime
cargo test -p gravity-iced
cargo test -p gravity-cli

# Linting
cargo clippy --workspace -- -D warnings

# Formatting
cargo fmt --all
cargo fmt --all -- --check  # CI check

# Run examples
cargo run -p hello-world
cargo run -p counter
cargo run -p todo-app

# Documentation
cargo doc --workspace --no-deps --open

# Benchmarks (when implemented)
cargo bench -p gravity-core
```

## Code Style

### Rust Conventions

- **Edition**: 2024 (or 2021 until 2024 stabilizes)
- **MSRV**: Stable Rust only, no nightly features in public API
- **Formatting**: Default rustfmt configuration
- **Linting**: `cargo clippy` with `-D warnings`
- **Documentation**: All public items must have rustdoc comments
- **Unsafe**: Zero unsafe in generated code unless explicitly justified

### Naming Conventions

- **Crates**: `gravity-{module}` (kebab-case)
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
- Target: >90% coverage for gravity-core

## Architecture Principles

### Constitution (v1.0.0)

1. **Declarative-First**: XML is the source of truth for UI structure
2. **Type Safety Preservation**: No runtime type erasure for messages/state
3. **Production Mode**: Static code generation for deployments
4. **Backend Abstraction**: Core crate has no Iced dependency
5. **Test-First Development**: Tests define contracts before implementation

### Crate Dependencies

```text
gravity-core (no backend deps)
    ↑
    ├── gravity-macros (proc-macro, depends on core)
    ├── gravity-runtime (depends on core)
    └── gravity-iced (depends on core + iced)
            ↑
            └── gravity-cli (depends on all above)
```

### Key Traits

```rust
// Backend abstraction (gravity-core/src/traits/backend.rs)
pub trait Backend {
    type Widget<'a>;
    type Message: Clone + 'static;
    fn text(&self, content: &str) -> Self::Widget<'_>;
    fn button(&self, label: Self::Widget<'_>, on_press: Option<Self::Message>) -> Self::Widget<'_>;
    // ... other widgets
}

// Binding abstraction (gravity-core/src/binding/mod.rs)
pub trait UiBindable: Serialize + for<'de> Deserialize<'de> {
    fn get_field(&self, path: &[&str]) -> Option<BindingValue>;
    fn available_fields() -> Vec<String>;
}
```

### AppState Pattern (006-auto-ui-loading)

The `AppState<M>` struct provides a unified way to manage UI state:

```rust
use gravity_core::AppState;

// Simple usage (no model)
let state = AppState::<()>::new(document);

// With model
let state = AppState::with_model(document, my_model);

// With handlers
let state = AppState::with_handlers(document, handler_registry);
```

### Auto-Loading UI Files

Use the `#[gravity_ui]` macro to automatically load XML files:

```rust
// src/ui/app.rs
use gravity_macros::{gravity_ui, UiModel};
use gravity_core::AppState;

#[derive(UiModel)]
pub struct Model { count: i32 }

#[gravity_ui("app.gravity")]
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
    ├── app.rs          # UI code with #[gravity_ui] macro
    └── app.gravity     # XML UI definition
```

## Performance Budgets

| Metric | Target |
|--------|--------|
| XML parse time | < 10ms for 1000 widgets |
| Code generation | < 5s for typical application |
| Runtime memory | < 50MB baseline |

## Recent Changes
- 006-auto-ui-loading: Added Rust Edition 2024, MSRV 1.75 (per constitution) + `gravity-core`, `gravity-macros`, `gravity-runtime`, `gravity-iced`, `iced` 0.14+
- 005-implement-real-widgets: Added Rust Edition 2021, MSRV 1.75 + Iced 0.14 (with `image` feature enabled), gravity-core
- 004-advanced-widgets-todo: Added Rust Edition 2024, MSRV 1.75 + Iced 0.14+ (already in workspace)

**Phase 7 Complete (006-auto-ui-loading):**
- ✅ Contract tests for auto-loading mechanism (gravity-macros/tests/auto_loading_tests.rs)
- ✅ Contract tests for AppState struct (gravity-core/tests/appstate_tests.rs)
- ✅ Integration test for hello-world (examples/hello-world/tests/integration.rs)
- ✅ Migrated examples/counter to new auto-loading pattern
- ✅ Migrated examples/todo-app to new auto-loading pattern
- ✅ Updated AGENTS.md with AppState usage patterns

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

### Current Status: Phase 7 Complete ✓

**Implemented Components:**
- `gravity-core/src/state/mod.rs`: AppState struct with constructors
- `gravity-core/src/binding/`: UiBindable trait, BindingValue enum
- `gravity-macros/src/ui_loader.rs`: #[gravity_ui] macro for auto-loading
- `gravity-macros/tests/auto_loading_tests.rs`: Contract tests
- `gravity-core/tests/appstate_tests.rs`: Contract tests
- `examples/hello-world/`: Minimal auto-loading example
- `examples/counter/`: Migrated to auto-loading pattern
- `examples/todo-app/`: Migrated to auto-loading pattern
- `examples/settings/`: New example demonstrating multiple views

**Auto-Loading Features:**
- ✅ #[gravity_ui] macro for automatic XML loading
- ✅ LazyLock for thread-safe lazy initialization
- ✅ Multiple views support (app.gravity, settings.gravity)
- ✅ AppState with Model and HandlerRegistry support

**Validation Features (Phase 8):**
- ✅ `gravity check` validates XML syntax and widget names
- ✅ Clear error messages with span information
- ✅ Exit code 0 for success, 1 for failure
- ✅ File walking with `.gravity` extension filtering
- ✅ Comprehensive test coverage for validation

**Next Steps:**
- Phase 8: Documentation & Final Polish
  - Update README.md with auto-loading documentation
  - Run final verification tests
  - Prepare for feature release

<!-- MANUAL ADDITIONS END -->
