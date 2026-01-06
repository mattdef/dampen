# Implementation Plan: Automatic UI File Loading with AppState Structure

**Branch**: `006-auto-ui-loading` | **Date**: 2026-01-06 | **Spec**: [link](spec.md)
**Input**: Feature specification from `/specs/006-auto-ui-loading/spec.md`

## Summary

Implement automatic loading of `.gravity` XML files when corresponding `.gravity.rs` files are compiled, eliminating the need for manual `include_str!` paths. Define a global `AppState` structure in `gravity_core` containing `GravityDocument` (mandatory), `Model` (optional), and `HandlerRegistry` (optional). This enables developers to create UI views by simply adding `.gravity` and `.gravity.rs` file pairs in the `ui/` directory.

## Technical Context

**Language/Version**: Rust Edition 2024, MSRV 1.75 (per constitution)
**Primary Dependencies**: `gravity-core`, `gravity-macros`, `gravity-runtime`, `gravity-iced`, `iced` 0.14+
**Storage**: N/A (compile-time XML loading, no runtime persistence required for this feature)
**Testing**: cargo test, proptest for parser edge cases, insta for snapshot tests
**Target Platform**: Desktop applications (Windows, Linux, macOS) via Iced
**Project Type**: Rust framework/library (workspace structure)
**Performance Goals**: XML parse time < 10ms for 1000 widgets (per constitution)
**Constraints**: Must NOT break existing applications using manual `include_str!` loading; must support dev and prod modes identically
**Scale/Scope**: Applies to all Gravity projects; affects new project creation workflow

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle | Status | Notes |
|-----------|--------|-------|
| I. Declarative-First | ✅ PASS | XML remains source of truth; auto-loading preserves separation of structure (XML) and behavior (Rust) |
| II. Type Safety Preservation | ✅ PASS | AppState uses concrete types; no `Any`-based dispatch for core message/state |
| III. Dual-Mode Architecture | ✅ PASS | Auto-loading works in both dev (runtime interpretation) and prod (static code gen) modes |
| IV. Backend Abstraction | ✅ PASS | AppState is defined in gravity-core (backend-agnostic); no Iced types in core API |
| V. Test-First Development | ✅ PASS | Contract tests for auto-loading mechanism and AppState compatibility will be implemented |

**Post-Design Verification**: All principles remain satisfied after Phase 1 design. The AppState structure maintains backend abstraction by being defined in gravity-core. The auto-loading mechanism is transparent to the backend and doesn't introduce Iced dependencies in core crates.

## Project Structure

### Documentation (this feature)

```text
specs/006-auto-ui-loading/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output (complete)
├── data-model.md        # Phase 1 output (to be generated)
├── quickstart.md        # Phase 1 output (to be generated)
├── contracts/           # Phase 1 output (to be generated)
│   └── xml-schema.md    # AppState structure contract
└── tasks.md             # Phase 2 output (/speckit.tasks command - NOT created by /speckit.plan)
```

### Source Code (repository root)

```text
crates/
├── gravity-core/             # Parser, AST, IR, trait definitions
│   ├── src/
│   │   ├── lib.rs
│   │   ├── state/            # NEW: AppState struct definition
│   │   │   └── mod.rs
│   │   ├── parser/
│   │   ├── ir/
│   │   ├── handler/
│   │   └── binding/
│   └── tests/
│
├── gravity-macros/           # Proc macros for production mode
│   ├── src/
│   │   ├── lib.rs
│   │   ├── ui_model.rs       # Existing: #[derive(UiModel)]
│   │   └── ui_loader.rs      # NEW: #[gravity_ui] macro
│   ├── build.rs              # NEW: auto-discovery of .gravity files
│   └── tests/
│
├── gravity-runtime/          # Hot-reload interpreter, file watcher
│   ├── src/
│   │   ├── lib.rs
│   │   ├── interpreter.rs
│   │   ├── watcher.rs
│   │   └── state.rs
│   └── tests/
│
├── gravity-iced/             # Iced backend implementation
│   ├── src/
│   │   ├── lib.rs
│   │   ├── widgets/
│   │   └── builder.rs        # Existing: GravityWidgetBuilder
│   └── tests/
│
└── gravity-cli/              # Developer CLI (dev/build commands)
    ├── src/
    │   ├── main.rs
    │   ├── commands/
    │   │   ├── dev.rs
    │   │   └── check.rs
    └── tests/

examples/
├── hello-world/              # To be migrated to new pattern
├── counter/                  # To be migrated to new pattern
└── todo-app/                 # To be migrated to new pattern
```

**Structure Decision**: The auto-loading mechanism will be implemented as a new procedural macro `#[gravity_ui]` in `gravity-macros` with build.rs support for file discovery. The `AppState` struct will be defined in `gravity-core/src/state/` to maintain backend abstraction. This structure follows existing crate architecture and keeps backend-specific code isolated in `gravity-iced`.

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| N/A | No constitution violations | N/A |

## Phase 0: Research Complete ✓

All technical decisions resolved through research (see `research.md`):

| Decision | Choice | Research Section |
|----------|--------|------------------|
| Auto-loading mechanism | Hybrid (build.rs + proc macro) | Q1 |
| AppState API | Generic struct + constructors | Q2 |
| Error handling | syn::Error + proc-macro-error2 | Q3 |

## Phase 1: Design Outputs ✓

**Status**: Complete - all deliverables generated

### Phase 1 Checklist

- [x] Generate `data-model.md` with AppState struct definition
- [x] Generate `contracts/xml-schema.md` with macro API and error codes
- [x] Generate `quickstart.md` with developer guide
- [x] Update agent context for opencode

### 1.1 Data Model (data-model.md)

**AppState struct** - defined in `gravity-core/src/state/mod.rs`:

```rust
use std::marker::PhantomData;
use crate::{GravityDocument, HandlerRegistry, binding::UiBindable};

pub struct AppState<M: UiBindable = ()> {
    pub document: GravityDocument,
    pub model: M,
    pub handler_registry: HandlerRegistry,
    _marker: PhantomData<M>,
}

impl<M: UiBindable> AppState<M> {
    pub fn new(document: GravityDocument) -> Self
    where
        M: Default,
    {
        Self {
            document,
            model: M::default(),
            handler_registry: HandlerRegistry::default(),
            _marker: PhantomData,
        }
    }

    pub fn with_model(document: GravityDocument, model: M) -> Self {
        Self {
            document,
            model,
            handler_registry: HandlerRegistry::default(),
            _marker: PhantomData,
        }
    }

    pub fn with_handlers(document: GravityDocument, handler_registry: HandlerRegistry) -> Self
    where
        M: Default,
    {
        Self {
            document,
            model: M::default(),
            handler_registry,
            _marker: PhantomData,
        }
    }
}
```

**Relationships**:
- `AppState` owns `GravityDocument` (parsed UI tree)
- `AppState<M>` is generic over the model type implementing `UiBindable`
- `AppState` owns `HandlerRegistry` for event handlers
- Compatible with `GravityWidgetBuilder::new()`

### 1.2 API Contracts (contracts/xml-schema.md)

**Macro API**:

```rust
// In ui/app.gravity.rs
#[gravity_ui("ui/app.gravity")]
mod app_ui {}

pub fn create_app_state() -> AppState {
    AppState::new(app_ui::document)
}
```

**File loading convention**:
- `<filename>.gravity.rs` automatically loads `<filename>.gravity`
- Custom path: `#[gravity_ui(path = "custom/path.gravity")]`

**Error codes**:
| Code | Condition | Message |
|------|-----------|---------|
| G001 | File not found | "Gravity UI file not found: '{path}'" |
| G002 | Invalid XML | "Invalid XML in Gravity UI file: {error}" |
| G003 | Unknown handler | "Handler '{name}' not registered" |
| G004 | Parse error | GravityDocument parse failed |

### 1.3 Quickstart Guide (quickstart.md)

**New project structure**:
```
my-project/
├── Cargo.toml
├── src/
│   └── main.rs
└── ui/
    ├── mod.rs
    ├── app.gravity.rs
    └── app.gravity
```

**Step-by-step guide**:
1. Create `ui/` directory
2. Add `app.gravity` XML file
3. Create `app.gravity.rs` with `#[gravity_ui]` macro
4. Export AppState from `ui/mod.rs`
5. Import and use in `main.rs`

---

## Phase 1 Checklist

- [ ] Generate `data-model.md` with AppState struct definition
- [ ] Generate `contracts/xml-schema.md` with macro API and error codes
- [ ] Generate `quickstart.md` with developer guide
- [ ] Update agent context for opencode

## Phase 2: Tasks (pending /speckit.tasks)

After Phase 1 complete, generate `tasks.md` with:
- Implementation tasks for each component
- Test requirements
- Migration steps for existing examples
