# Gravity Development Guidelines

Auto-generated from all feature plans. Last updated: 2025-12-30

## Active Technologies
- File-based (XML UI definitions, optional separate style files), serialized state in `.gravity-state.json` (002-layout-theming-styling)
- Rust Edition 2024, Stable Rust (no nightly features) (003-widget-builder)
- N/A (runtime interpretation, no persistence required) (003-widget-builder)

- **Language**: Rust Edition 2024, MSRV stable (no nightly features in public API)
- **UI Framework**: `iced` 0.14+
- **XML Parsing**: `roxmltree` 0.19+
- **File Watching**: `notify` 6.0+
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
├── gravity-macros/           # Proc macros (#[derive(UiModel)], #[ui_handler])
│   ├── src/
│   │   ├── lib.rs
│   │   ├── ui_model.rs
│   │   └── ui_handler.rs
│   └── tests/
│
├── gravity-runtime/          # Hot-reload interpreter, file watcher
│   ├── src/
│   │   ├── lib.rs
│   │   ├── interpreter.rs
│   │   ├── watcher.rs
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
    │   ├── commands/         # dev.rs, build.rs, check.rs, inspect.rs
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

## Gravity Dev Command (Hot-Reload Mode)

The `gravity dev` command provides a development environment with automatic hot-reload of UI files.

### Basic Usage

```bash
# From your project directory
gravity dev --ui <ui_directory> --file <main_file> [options]

# Example
gravity dev --ui ui --file main.gravity --verbose
```

### Options

| Option | Description | Default |
|--------|-------------|---------|
| `--ui, -u <dir>` | UI directory containing `.gravity` files | `ui` |
| `--file <name>` | Main `.gravity` file (relative to ui dir) | `main.gravity` |
| `--state <file>` | State file for persistence | `.gravity-state.json` |
| `--verbose, -v` | Enable verbose output | `false` |

### How It Works

1. **Initial Load**: Reads and parses the main `.gravity` file
2. **UI Rendering**: Displays the UI in an Iced window
3. **File Watching**: Monitors the UI directory for changes
4. **Hot-Reload**: Automatically reloads and updates the UI when files change
5. **State Persistence**: Maintains application state across reloads

### Example Project Structure

```
my-project/
├── Cargo.toml
├── src/
│   └── main.rs          # Optional: Rust code for handlers
├── ui/
│   └── main.gravity     # Main UI file
└── .gravity-state.json  # Auto-generated state file
```

### UI File Example (`ui/main.gravity`)

```xml
<column padding="40" spacing="20">
    <text value="My App" size="32" weight="bold" />
    <text value="Count: {count}" size="18" />
    <row spacing="10">
        <button label="Increment" on_click="increment" />
        <button label="Decrement" on_click="decrement" />
    </row>
    <button label="Reset" on_click="reset" />
</column>
```

### Development Workflow

1. **Start Dev Server**:
   ```bash
   gravity dev --ui ui --file main.gravity --verbose
   ```

2. **Edit UI Files**: Modify `.gravity` files in your `ui/` directory

3. **See Changes**: The UI updates automatically within ~200ms of saving

4. **Check Logs**: Use `--verbose` to see reload events and errors

### Error Handling

- **Parse Errors**: Displayed in a red overlay in the UI
- **Runtime Errors**: Shown with location and suggestions
- **File Not Found**: Clear error message with path

### State Management

- **Automatic Persistence**: State is saved to `.gravity-state.json`
- **Cross-Reload**: State survives hot-reloads
- **Clean Start**: Delete state file to reset

### Performance

- **Hot-Reload Latency**: < 500ms from save to UI update
- **File Polling**: Every 200ms
- **Debouncing**: Built-in to prevent multiple reloads

### Troubleshooting

**UI doesn't update:**
- Check `--verbose` output for errors
- Verify file paths are correct
- Ensure `.gravity` files have valid XML syntax

**State not persisting:**
- Check file permissions on `.gravity-state.json`
- Verify state file is in working directory

**Slow reloads:**
- Check for very large UI files
- Verify disk I/O performance
- Consider reducing file size or complexity

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
3. **Dual-Mode Architecture**: Dev (hot-reload) + Prod (static codegen)
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

## Performance Budgets

| Metric | Target |
|--------|--------|
| XML parse time | < 10ms for 1000 widgets |
| Hot-reload latency | < 500ms from save to UI update |
| Code generation | < 5s for typical application |
| Runtime memory (dev) | < 50MB baseline |

## Recent Changes
- 003-widget-builder: Added Rust Edition 2024, Stable Rust (no nightly features)
- 002-layout-theming-styling: Added Rust Edition 2024, MSRV stable (no nightly features in public API)

- **Phase 5 Complete**: User Story 5 - Derive Bindable Model from Rust Struct
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

### Current Status: Phase 8 Complete ✓

**Implemented Components:**
- `gravity-core/src/binding/`: UiBindable trait, BindingValue enum
- `gravity-core/src/expr/eval.rs`: Expression evaluator
- `gravity-macros/src/ui_model.rs`: #[derive(UiModel)] macro
- `gravity-macros/tests/ui_model_tests.rs`: 10 comprehensive tests
- `gravity-runtime/src/watcher.rs`: File watcher with notify
- `gravity-runtime/src/interpreter.rs`: Hot-reload interpreter
- `gravity-runtime/src/overlay.rs`: Error overlay UI
- `gravity-cli/src/commands/dev.rs`: Dev mode with hot-reload
- `gravity-cli/src/commands/check.rs`: UI validation command
- `examples/todo-app/`: Working bindings example
- `examples/counter/`: Working handlers example
- `examples/hello-world/`: Working static example

**Hot-Reload Features:**
- ✅ File watching with `notify` 6.0+
- ✅ Automatic reload on `.gravity` file changes
- ✅ State persistence across reloads
- ✅ Error overlay UI for parse/runtime errors
- ✅ Verbose logging mode
- ✅ ~200ms hot-reload latency

**Validation Features (Phase 8):**
- ✅ `gravity check` validates XML syntax and widget names
- ✅ Clear error messages with span information
- ✅ Exit code 0 for success, 1 for failure
- ✅ File walking with `.gravity` extension filtering
- ✅ Comprehensive test coverage for validation

**Next Steps:**
- Phase 9: User Story 7 - Support All Core Iced Widgets
  - Implement remaining widgets (container, scrollable, stack, etc.)
  - Add attribute parsing for width/height, padding, spacing
  - Create comprehensive todo-app example

<!-- MANUAL ADDITIONS END -->
