# Implementation Plan: Gravity Framework Technical Specifications

**Branch**: `001-framework-technical-specs` | **Date**: 2025-12-30 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/001-framework-technical-specs/spec.md`

## Summary

Build a declarative UI framework for Rust that compiles XML markup to Iced applications. The framework supports dual-mode operation: development mode with hot-reload interpretation, and production mode with static code generation. Implementation follows 8 phases from POC to public alpha, prioritizing short feedback loops and incremental validation.

## Technical Context

**Language/Version**: Rust Edition 2024, MSRV stable (no nightly features in public API)  
**Primary Dependencies**: `iced` 0.14+, `quick-xml` or `roxmltree`, `notify` (file watcher), `serde` (state serialization), `syn`/`quote`/`proc-macro2` (macros)  
**Storage**: N/A (framework library, no persistent storage)  
**Testing**: `cargo test`, property-based testing with `proptest`, snapshot testing with `insta`  
**Target Platform**: Windows, Linux, macOS (tier 1 support)  
**Project Type**: Rust workspace with 5 crates  
**Performance Goals**: XML parse <10ms/1000 widgets, hot-reload <500ms, codegen <5s  
**Constraints**: Zero unsafe in generated code, <50MB runtime memory baseline  
**Scale/Scope**: Framework supporting applications with 1000+ widgets, targeting solo/small team developers

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle | Requirement | Status | Notes |
|-----------|-------------|--------|-------|
| I. Declarative-First | XML as source of truth for UI | PASS | Core design premise |
| II. Type Safety | No runtime type erasure for messages/state | PASS | Compile-time handler verification in prod mode |
| III. Dual-Mode | Dev (hot-reload) + Prod (codegen) | PASS | Phases 4 + 5 explicitly address this |
| IV. Backend Abstraction | Core crate backend-agnostic | PASS | `gravity-core` has no Iced dependency |
| V. Test-First | TDD for all features | PASS | Each phase includes test criteria |

**Technical Constraints Check**:
- [x] Rust Edition 2024+
- [x] Stable Rust only (no nightly)
- [x] Workspace structure matches constitution
- [x] Performance budgets defined

## Project Structure

### Documentation (this feature)

```text
specs/001-framework-technical-specs/
├── plan.md              # This file
├── research.md          # Phase 0 research decisions
├── data-model.md        # IR and core type definitions
├── quickstart.md        # Developer getting-started guide
├── checklists/
│   └── requirements.md  # Spec quality checklist
└── contracts/
    └── xml-schema.md    # XML element and attribute specifications
```

### Source Code (repository root)

```text
# Rust workspace structure (per constitution)
Cargo.toml                    # Workspace manifest

crates/
├── gravity-core/             # Parser, AST, IR, trait definitions
│   ├── src/
│   │   ├── lib.rs
│   │   ├── parser/           # XML parsing
│   │   │   ├── mod.rs
│   │   │   ├── lexer.rs
│   │   │   └── error.rs
│   │   ├── ir/               # Intermediate representation
│   │   │   ├── mod.rs
│   │   │   ├── node.rs
│   │   │   ├── expr.rs
│   │   │   └── span.rs
│   │   ├── expr/             # Expression AST and evaluation
│   │   │   ├── mod.rs
│   │   │   ├── ast.rs
│   │   │   └── eval.rs
│   │   └── traits/           # Backend abstraction traits
│   │       ├── mod.rs
│   │       └── backend.rs
│   └── tests/
│       ├── parser_tests.rs
│       └── ir_tests.rs
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
│   │   ├── state.rs          # Model serialization/restoration
│   │   └── overlay.rs        # Error overlay UI
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
    │   ├── commands/
    │   │   ├── mod.rs
    │   │   ├── dev.rs
    │   │   ├── build.rs
    │   │   ├── check.rs
    │   │   └── inspect.rs
    │   └── config.rs
    └── tests/

examples/
├── hello-world/              # Minimal example (Phase 0 target)
├── counter/                  # Interactive example (Phase 2 target)
├── todo-app/                 # Bindings example (Phase 3 target)
└── full-demo/                # Complete showcase (Phase 6 target)
```

**Structure Decision**: Rust workspace with 5 crates per constitution. The `crates/` directory isolates framework code; `examples/` provides progressive learning materials.

## Complexity Tracking

No constitution violations to justify.

---

## Phase 0: Foundations (POC)

**Objective**: Validate end-to-end technical feasibility by rendering a minimal XML-defined UI through Iced.

**Prerequisites**: None (starting point)

**Milestone**: M1 - First XML-to-Iced render

**Estimation**: M (Medium) - 2-3 weeks

### Tasks

1. Initialize Cargo workspace with `gravity-core` and `gravity-iced` crates
2. Implement minimal XML parser (hardcoded for `<column>`, `<row>`, `<text>`, `<button>`)
3. Define minimal IR: `WidgetNode` enum with variant per widget type
4. Implement Iced backend trait with hardcoded widget constructors
5. Create `hello-world` example: static XML rendered via Iced
6. Set up CI: `cargo test`, `cargo clippy`, `cargo fmt --check`

### Definition of Done

- [ ] `hello-world` example compiles and displays UI defined in XML
- [ ] XML parse errors include line/column information
- [ ] All tests pass (`cargo test --workspace`)
- [ ] CI pipeline green on all platforms

### Livrables

- `gravity-core` crate (minimal)
- `gravity-iced` crate (minimal)
- `examples/hello-world/`
- CI configuration (GitHub Actions)

### Risks

| Risk | Mitigation |
|------|------------|
| Iced 0.14 API breaking changes | Pin exact version, monitor changelog |
| XML parser library limitations | Evaluate roxmltree vs quick-xml early |
| Widget lifetime/ownership complexity | Start with owned data, optimize later |

---

## Phase 1: Core Parser and IR

**Objective**: Production-quality XML parser with complete IR for all base Iced widgets.

**Prerequisites**: Phase 0 complete

**Milestone**: (internal) Parser foundation solid

**Estimation**: L (Large) - 3-4 weeks

### Tasks

1. Design complete IR structures: `WidgetNode`, `Attribute`, `EventBinding`, `Span`
2. Implement full XML parser with error recovery
3. Add expression parsing for `{expr}` syntax (tokenize only, no evaluation)
4. Define XML schema (namespace, versioning)
5. Implement IR serialization (serde) for caching
6. Create comprehensive parser test suite (>90% coverage)
7. Add property-based tests for edge cases (deeply nested, malformed XML)
8. Document XML schema with examples

### Definition of Done

- [ ] All core Iced widgets have IR representation
- [ ] Parser handles malformed XML gracefully with clear errors
- [ ] Expression syntax `{field}`, `{obj.field}` is tokenized correctly
- [ ] IR round-trips through serialization
- [ ] Parser tests achieve >90% code coverage
- [ ] XML schema documented in `contracts/xml-schema.md`

### Livrables

- Complete `gravity-core/src/parser/`
- Complete `gravity-core/src/ir/`
- `contracts/xml-schema.md`
- Test fixtures in `gravity-core/tests/fixtures/`

### Risks

| Risk | Mitigation |
|------|------------|
| Expression grammar ambiguity | Define formal grammar upfront, use recursive descent |
| Performance on large files | Benchmark early, consider streaming parser |

---

## Phase 2: Handler System

**Objective**: Connect XML events to typed Rust handlers with compile-time verification.

**Prerequisites**: Phase 1 complete

**Milestone**: M2 - Interactive application with handlers

**Estimation**: M (Medium) - 2-3 weeks

### Tasks

1. Design `#[ui_handler]` proc macro attribute
2. Implement handler registry (runtime HashMap for dev mode)
3. Generate `Message` enum variants from handlers
4. Implement event dispatch: XML event -> Message -> handler
5. Add handler signature validation (simple, with value, with Command)
6. Create `counter` example demonstrating click handlers
7. Add compile-time validation for handler existence (prod mode preparation)

### Definition of Done

- [ ] `#[ui_handler]` macro compiles and registers handlers
- [ ] Event bindings in XML dispatch to correct handlers
- [ ] Handler with `value: T` parameter receives correct typed value
- [ ] Handler returning `Command<Message>` integrates with Iced
- [ ] `counter` example is interactive
- [ ] Referencing non-existent handler produces clear error

### Livrables

- `gravity-macros` crate
- Updated `gravity-runtime` with dispatch
- `examples/counter/`

### Risks

| Risk | Mitigation |
|------|------------|
| Proc macro debugging difficulty | Extensive trybuild tests |
| Handler signature flexibility | Support fixed set of signatures first |

---

## Phase 3: Binding System (MVU-D)

**Objective**: Implement expression evaluation and Model-to-View data binding.

**Prerequisites**: Phase 2 complete

**Milestone**: M3 - Realistic applications possible with bindings

**Estimation**: L (Large) - 3-4 weeks

### Tasks

1. Design expression AST: field access, method calls, conditionals
2. Implement expression evaluator against runtime Model
3. Design `#[derive(UiModel)]` macro
4. Implement binding accessors generation
5. Support field types: primitives, String, Vec<T>, Option<T>
6. Implement `#[ui_skip]` and `#[ui_bind]` attributes
7. Add formatted bindings: `"Total: {items.len()}"`
8. Add conditional bindings: `enabled="{count > 0}"`
9. Create `todo-app` example with full bindings
10. Add binding error detection (missing field, type mismatch)

### Definition of Done

- [ ] Simple bindings `{field}` display current Model value
- [ ] Nested bindings `{obj.field}` work correctly
- [ ] Formatted bindings interpolate correctly
- [ ] Conditional bindings evaluate boolean expressions
- [ ] `#[derive(UiModel)]` generates correct accessors
- [ ] `#[ui_skip]` excludes fields from binding
- [ ] `todo-app` example demonstrates CRUD operations
- [ ] Binding errors show field name and available alternatives

### Livrables

- `gravity-core/src/expr/` (evaluator)
- Enhanced `gravity-macros` (UiModel)
- `examples/todo-app/`

### Risks

| Risk | Mitigation |
|------|------------|
| Expression grammar complexity | Limit to subset (no arbitrary Rust expressions) |
| Type inference across XML boundary | Require explicit types or infer from Model |

---

## Phase 4: Hot-Reload (Dev Mode)

**Objective**: Enable live UI updates on XML file changes without application restart.

**Prerequisites**: Phase 3 complete

**Milestone**: M4 - Hot-reload operational

**Estimation**: M (Medium) - 2-3 weeks

### Tasks

1. Integrate `notify` crate for file watching
2. Implement Model serialization before reload (via serde)
3. Implement XML re-parse and IR reconstruction on change
4. Implement Model restoration after reload
5. Design migration strategy for Model structure changes
6. Implement error overlay UI for parse/binding errors
7. Create `gravity dev` CLI command
8. Measure and optimize reload latency (<500ms target)

### Definition of Done

- [ ] File changes trigger UI rebuild within 500ms
- [ ] Model state preserved across reload (counter value persists)
- [ ] Parse errors display in overlay without crash
- [ ] Binding errors display in overlay without crash
- [ ] `gravity dev` starts application in hot-reload mode
- [ ] Model migration handles added/removed fields gracefully

### Livrables

- `gravity-runtime` crate (complete)
- `gravity-cli/src/commands/dev.rs`
- Hot-reload test harness

### Risks

| Risk | Mitigation |
|------|------------|
| File watcher platform differences | Use `notify` with recommended backend per OS |
| Serialization format incompatibility | Use JSON with lenient deserialization |
| Overlay UI complexity | Simple text overlay initially, enhance later |

---

## Phase 5: Code Generator (Production Mode)

**Objective**: Generate static Rust code from XML for zero-runtime-overhead production builds.

**Prerequisites**: Phase 4 complete (for parity testing)

**Milestone**: M5 - Production build optimized

**Estimation**: L (Large) - 3-4 weeks

### Tasks

1. Evaluate proc-macro vs build.rs approach (see research.md)
2. Design code generation templates using `quote`
3. Implement `impl Application` generation for user Model
4. Implement `view()` generation with inlined widget tree
5. Implement `update()` generation with handler dispatch
6. Inline binding expressions as Rust code
7. Implement compile-time handler validation
8. Add optimizations: dead code elimination, constant folding
9. Create `gravity build` CLI command
10. Benchmark generated code vs hand-written Iced

### Definition of Done

- [ ] `gravity build` generates compilable Rust code
- [ ] Generated code produces identical UI to dev mode
- [ ] No XML parsing occurs at runtime in production
- [ ] Handler mismatches are compile errors
- [ ] Generated code is human-readable
- [ ] Performance within 5% of hand-written Iced

### Livrables

- Code generation module in `gravity-core` or separate crate
- `gravity-cli/src/commands/build.rs`
- Benchmark suite

### Risks

| Risk | Mitigation |
|------|------------|
| Codegen complexity explosion | Generate idiomatic code, not optimized code |
| Macro hygiene issues | Extensive testing with diverse Model types |
| Build time regression | Measure incrementally, optimize hot paths |

---

## Phase 6: Polish and Developer Experience

**Objective**: Production-ready CLI, comprehensive error messages, and documentation.

**Prerequisites**: Phase 5 complete

**Milestone**: M6 - Public alpha release

**Estimation**: M (Medium) - 2-3 weeks

### Tasks

1. Implement `gravity check` command (validation without running)
2. Implement `gravity inspect` command (IR visualization)
3. Add `--format json` output for tooling integration
4. Enhance error messages with spans, suggestions, and fix hints
5. Create user documentation (README, guides)
6. Create example project templates
7. Package for crates.io publication
8. Set up documentation site (mdbook or similar)

### Definition of Done

- [ ] `gravity check` validates XML and bindings with clear output
- [ ] `gravity inspect` displays IR tree and optionally generated code
- [ ] Error messages include source spans and suggestions
- [ ] README covers installation, quick start, concepts
- [ ] At least 4 example projects cover different use cases
- [ ] Crates publish successfully to crates.io
- [ ] Documentation site is live

### Livrables

- Complete `gravity-cli`
- `docs/` directory or external site
- `examples/` with 4+ projects
- Published crates

### Risks

| Risk | Mitigation |
|------|------------|
| Documentation drift | Generate from code where possible |
| Example maintenance | Include examples in CI test suite |

---

## Phase 7: Advanced Features

**Objective**: Extend framework with advanced Iced features and tooling preparation.

**Prerequisites**: Phase 6 complete

**Milestone**: (future) Feature completeness

**Estimation**: XL (Extra Large) - 4-6 weeks

### Tasks

1. Add remaining Iced widgets (canvas, responsive layouts)
2. Implement declarative theming/styling system
3. Add Iced subscription support (timers, system events)
4. Prepare LSP foundation (parser API for real-time feedback)
5. Add widget attribute autocompletion data
6. Implement go-to-definition for handler references
7. Performance optimization pass

### Definition of Done

- [ ] All Iced widgets available in XML
- [ ] Themes definable in XML or separate style file
- [ ] Subscriptions expressible declaratively
- [ ] LSP server prototype provides completions
- [ ] Framework handles 1000+ widget applications smoothly

### Livrables

- Extended widget support
- Theming system
- LSP crate (prototype)

### Risks

| Risk | Mitigation |
|------|------------|
| Scope creep | Strict phase gates, defer to future versions |
| LSP complexity | Start with completions only, iterate |

---

## Milestones Summary

| Milestone | Phase | Deliverable | Success Criteria |
|-----------|-------|-------------|------------------|
| M1 | 0 | First XML render | `hello-world` displays from XML |
| M2 | 2 | Interactive app | `counter` responds to clicks |
| M3 | 3 | Realistic apps | `todo-app` with full CRUD |
| M4 | 4 | Hot-reload | <500ms refresh, state preserved |
| M5 | 5 | Production build | Generated code matches hand-written perf |
| M6 | 6 | Alpha release | Published to crates.io with docs |

## Global Success Criteria

- 80%+ of UI definable in XML for typical applications
- Hot-reload latency <500ms consistently
- Generated production code within 5% of hand-written performance
- Clear, actionable error messages for all failure modes
- Framework usable by developers with Iced familiarity
