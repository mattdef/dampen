# Tasks: Gravity Framework Technical Specifications

**Input**: Design documents from `/specs/001-framework-technical-specs/`  
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/xml-schema.md

**Tests**: Constitution mandates Test-First development (Principle V). Tests are included for each user story.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Path Conventions

Rust workspace with 5 crates:

```text
crates/gravity-core/src/     # Parser, IR, traits
crates/gravity-macros/src/   # Proc macros
crates/gravity-runtime/src/  # Hot-reload, file watcher
crates/gravity-iced/src/     # Iced backend
crates/gravity-cli/src/      # CLI commands
examples/                    # Example applications
```

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Initialize Cargo workspace and core project structure

- [X] T001 Create Cargo workspace manifest at Cargo.toml with all 5 crates
- [X] T002 [P] Initialize gravity-core crate at crates/gravity-core/Cargo.toml
- [X] T003 [P] Initialize gravity-macros crate at crates/gravity-macros/Cargo.toml (proc-macro = true)
- [X] T004 [P] Initialize gravity-runtime crate at crates/gravity-runtime/Cargo.toml
- [X] T005 [P] Initialize gravity-iced crate at crates/gravity-iced/Cargo.toml
- [X] T006 [P] Initialize gravity-cli crate at crates/gravity-cli/Cargo.toml
- [X] T007 [P] Configure rustfmt.toml with project formatting rules
- [X] T008 [P] Configure clippy.toml with lint rules
- [X] T009 Create .github/workflows/ci.yml for CI pipeline (test, clippy, fmt)
- [X] T010 Create examples/ directory structure with placeholder README

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core IR types and traits that ALL user stories depend on

**CRITICAL**: No user story work can begin until this phase is complete

### Core Types (gravity-core)

- [X] T011 [P] Implement Span type in crates/gravity-core/src/ir/span.rs
- [X] T012 [P] Implement WidgetKind enum in crates/gravity-core/src/ir/node.rs
- [X] T013 Implement WidgetNode struct in crates/gravity-core/src/ir/node.rs (depends on T011, T012)
- [X] T014 [P] Implement AttributeValue enum in crates/gravity-core/src/ir/node.rs
- [X] T015 [P] Implement EventBinding and EventKind in crates/gravity-core/src/ir/node.rs
- [X] T016 Implement GravityDocument struct in crates/gravity-core/src/ir/mod.rs
- [X] T017 Add serde derives to all IR types for serialization
- [X] T018 Create crates/gravity-core/src/ir/mod.rs exporting all IR types

### Expression AST (gravity-core)

- [X] T019 [P] Implement Expr enum in crates/gravity-core/src/expr/ast.rs
- [X] T020 [P] Implement FieldAccessExpr in crates/gravity-core/src/expr/ast.rs
- [X] T021 [P] Implement MethodCallExpr in crates/gravity-core/src/expr/ast.rs
- [X] T022 [P] Implement BinaryOpExpr and BinaryOp in crates/gravity-core/src/expr/ast.rs
- [X] T023 [P] Implement UnaryOpExpr and UnaryOp in crates/gravity-core/src/expr/ast.rs
- [X] T024 [P] Implement ConditionalExpr in crates/gravity-core/src/expr/ast.rs
- [X] T025 [P] Implement LiteralExpr in crates/gravity-core/src/expr/ast.rs
- [X] T026 Implement BindingExpr wrapper in crates/gravity-core/src/expr/ast.rs
- [X] T027 Create crates/gravity-core/src/expr/mod.rs exporting AST types

### Error Types (gravity-core)

- [X] T028 [P] Implement ParseError and ParseErrorKind in crates/gravity-core/src/parser/error.rs
- [X] T029 [P] Implement BindingError and BindingErrorKind in crates/gravity-core/src/expr/error.rs
- [X] T030 Implement Display trait for ParseError with span formatting

### Backend Trait (gravity-core)

- [X] T031 Define Backend trait in crates/gravity-core/src/traits/backend.rs
- [X] T032 Create crates/gravity-core/src/traits/mod.rs exporting Backend trait
- [X] T033 Create crates/gravity-core/src/lib.rs with public exports

**Checkpoint**: Foundation ready - gravity-core compiles with `cargo check -p gravity-core`

---

## Phase 3: User Story 1 - Define UI Structure Declaratively (Priority: P1) MVP

**Goal**: Parse XML markup into IR and render static UI through Iced

**Independent Test**: `examples/hello-world` displays UI defined entirely in XML

### Tests for User Story 1

- [X] T034 [P] [US1] Create test fixtures at crates/gravity-core/tests/fixtures/valid_simple.gravity
- [X] T035 [P] [US1] Create test fixtures at crates/gravity-core/tests/fixtures/valid_nested.gravity
- [X] T036 [P] [US1] Create test fixtures at crates/gravity-core/tests/fixtures/invalid_syntax.gravity
- [X] T037 [US1] Implement parser unit tests in crates/gravity-core/tests/parser_tests.rs
- [X] T038 [US1] Implement IR serialization tests in crates/gravity-core/tests/ir_tests.rs

### XML Parser Implementation

- [X] T039 [US1] Add roxmltree dependency to crates/gravity-core/Cargo.toml
- [X] T040 [US1] Implement XML tokenizer in crates/gravity-core/src/parser/lexer.rs
- [X] T041 [US1] Implement widget element parsing in crates/gravity-core/src/parser/mod.rs
- [X] T042 [US1] Implement attribute parsing with binding detection in crates/gravity-core/src/parser/mod.rs
- [X] T043 [US1] Implement event attribute parsing (on_click, etc.) in crates/gravity-core/src/parser/mod.rs
- [X] T044 [US1] Implement error recovery with span information in crates/gravity-core/src/parser/error.rs
- [X] T045 [US1] Implement parse() public function returning Result<GravityDocument, ParseError>

### Expression Tokenizer (no evaluation yet)

- [X] T046 [US1] Implement expression tokenizer in crates/gravity-core/src/expr/tokenizer.rs
- [X] T047 [US1] Parse `{field}` syntax into FieldAccessExpr
- [X] T048 [US1] Parse `{obj.field}` nested syntax into FieldAccessExpr with path
- [X] T049 [US1] Store raw expression in BindingExpr for later evaluation

### Iced Backend (minimal)

- [X] T050 [US1] Add iced dependency to crates/gravity-iced/Cargo.toml
- [X] T051 [US1] Implement IcedBackend struct in crates/gravity-iced/src/lib.rs
- [X] T052 [P] [US1] Implement text widget mapping in crates/gravity-iced/src/widgets/text.rs
- [X] T053 [P] [US1] Implement button widget mapping in crates/gravity-iced/src/widgets/button.rs
- [X] T054 [P] [US1] Implement column layout mapping in crates/gravity-iced/src/widgets/column.rs
- [X] T055 [P] [US1] Implement row layout mapping in crates/gravity-iced/src/widgets/row.rs
- [X] T056 [US1] Implement IR-to-Element conversion in crates/gravity-iced/src/lib.rs
- [X] T057 [US1] Create crates/gravity-iced/src/widgets/mod.rs exporting widget builders

### Hello World Example

- [X] T058 [US1] Create examples/hello-world/Cargo.toml
- [X] T059 [US1] Create examples/hello-world/ui/main.gravity with static content
- [X] T060 [US1] Create examples/hello-world/src/main.rs loading and rendering XML

**Checkpoint**: `cargo run -p hello-world` displays UI from XML file

---

## Phase 4: User Story 3 - Connect UI Events to Typed Handlers (Priority: P1)

**Goal**: Event handlers in Rust respond to UI events declared in XML

**Independent Test**: `examples/counter` increments/decrements via button clicks

### Tests for User Story 3

- [X] T061 [P] [US3] Create handler test fixtures at crates/gravity-macros/tests/ui_handler_tests.rs
- [X] T062 [US3] Implement trybuild tests for macro expansion in crates/gravity-macros/tests/

### Handler Macro Implementation

- [X] T063 [US3] Add syn, quote, proc-macro2 dependencies to crates/gravity-macros/Cargo.toml
- [X] T064 [US3] Implement #[ui_handler] attribute macro in crates/gravity-macros/src/ui_handler.rs
- [X] T065 [US3] Generate handler registration code in macro expansion
- [X] T066 [US3] Validate handler signature (simple, with value, with Command)
- [X] T067 [US3] Create crates/gravity-macros/src/lib.rs exporting ui_handler macro

### Handler Registry (gravity-core)

- [X] T068 [US3] Define HandlerRegistry struct in crates/gravity-core/src/handler/registry.rs
- [X] T069 [US3] Define HandlerEntry enum (Simple, WithValue, WithCommand)
- [X] T070 [US3] Implement handler lookup by name
- [X] T071 [US3] Create crates/gravity-core/src/handler/mod.rs

### Event Dispatch (gravity-runtime)

- [X] T072 [US3] Implement event dispatch in crates/gravity-runtime/src/interpreter.rs
- [X] T073 [US3] Map EventBinding from IR to handler lookup
- [X] T074 [US3] Generate Iced Message from handler call
- [X] T075 [US3] Integrate Command return type with Iced runtime

### Counter Example

- [X] T076 [US3] Create examples/counter/Cargo.toml
- [X] T077 [US3] Create examples/counter/ui/main.gravity with increment/decrement buttons
- [X] T078 [US3] Create examples/counter/src/main.rs with handler implementations

**Checkpoint**: `cargo run -p counter` responds to button clicks ✓

---

## Phase 5: User Story 5 - Derive Bindable Model from Rust Struct (Priority: P2)

**Goal**: `#[derive(UiModel)]` generates binding accessors for XML expressions

**Independent Test**: `{field_name}` in XML displays current Model value

### Tests for User Story 5

- [X] T079 [P] [US5] Create UiModel derive test fixtures in crates/gravity-macros/tests/ui_model_tests.rs
- [X] T080 [US5] Test primitive field accessors
- [X] T081 [US5] Test nested struct accessors (single-level paths only, nested paths handled by evaluator)
- [X] T082 [US5] Test #[ui_skip] attribute

### UiBindable Trait (gravity-core)

- [X] T083 [US5] Define UiBindable trait in crates/gravity-core/src/binding/mod.rs
- [X] T084 [US5] Define BindingValue enum (String, Integer, Float, Bool, List, None)
- [X] T085 [US5] Implement BindingValue::to_display_string()
- [X] T086 [US5] Implement BindingValue::to_bool() for conditionals

### UiModel Derive Macro

- [X] T087 [US5] Implement #[derive(UiModel)] in crates/gravity-macros/src/ui_model.rs
- [X] T088 [US5] Generate get_field() implementation for each field
- [X] T089 [US5] Support primitive types (i32, i64, f32, f64, bool, String)
- [X] T090 [US5] Support Option<T> fields
- [X] T091 [US5] Support Vec<T> fields
- [X] T092 [US5] Implement #[ui_skip] attribute handling
- [X] T093 [US5] Implement #[ui_bind] attribute for explicit inclusion
- [X] T094 [US5] Generate available_fields() for error suggestions

### Expression Evaluator (gravity-core)

- [X] T095 [US5] Implement expression evaluator in crates/gravity-core/src/expr/eval.rs
- [X] T096 [US5] Evaluate FieldAccessExpr against UiBindable
- [X] T097 [US5] Evaluate MethodCallExpr (len, to_string, etc.)
- [X] T098 [US5] Evaluate BinaryOpExpr (comparisons, logical)
- [X] T099 [US5] Evaluate ConditionalExpr (if-then-else)
- [X] T100 [US5] Evaluate formatted bindings with interpolation

### Integration with Runtime

- [X] T101 [US5] Update interpreter to evaluate bindings before rendering
- [X] T102 [US5] Pass evaluated values to widget constructors
- [X] T103 [US5] Update parser to create AttributeValue::Binding and AttributeValue::Interpolated
- [X] T104 [US5] Add tests for binding parsing

**Checkpoint**: `{counter}` in XML displays current model value ✓

---

## Phase 6: User Story 2 - Hot-Reload UI During Development (Priority: P1)

**Goal**: XML changes reflect in running app within 500ms without restart

**Independent Test**: Modify button text in XML, see change in running app

### Tests for User Story 2

- [X] T103 [P] [US2] Create hot-reload test harness in crates/gravity-runtime/tests/reload_tests.rs
- [X] T104 [US2] Test file change detection
- [X] T105 [US2] Test state preservation across reload
- [X] T106 [US2] Test error overlay on parse failure

### File Watcher (gravity-runtime)

- [X] T107 [US2] Add notify dependency to crates/gravity-runtime/Cargo.toml
- [X] T108 [US2] Implement file watcher in crates/gravity-runtime/src/watcher.rs
- [X] T109 [US2] Configure debounce (100ms) to batch rapid saves
- [X] T110 [US2] Filter for .gravity file extensions

### State Serialization (gravity-runtime)

- [X] T111 [US2] Implement RuntimeState wrapper in crates/gravity-runtime/src/state.rs
- [X] T112 [US2] Serialize model to JSON before reload
- [X] T113 [US2] Deserialize model after reload with lenient parsing
- [X] T114 [US2] Implement StateRestoration enum (Restored, Partial, Default)
- [X] T115 [US2] Handle added/removed fields gracefully

### Error Overlay (gravity-runtime)

- [X] T116 [US2] Implement error overlay widget in crates/gravity-runtime/src/overlay.rs
- [X] T117 [US2] Display ParseError with span information
- [X] T118 [US2] Display BindingError with field suggestions
- [X] T119 [US2] Add dismiss button to overlay

### Hot-Reload Integration

- [X] T120 [US2] Implement reload loop in crates/gravity-runtime/src/interpreter.rs
- [X] T121 [US2] Re-parse XML on file change
- [X] T122 [US2] Rebuild widget tree from new IR
- [X] T123 [US2] Measure and log reload latency

**Checkpoint**: Changes to XML files update running app within 500ms ✓

---

## Phase 7: User Story 4 - Generate Production Code (Priority: P2)

**Goal**: `gravity build` generates static Rust code with zero runtime overhead

**Independent Test**: Generated code compiles and produces identical UI to dev mode

### Tests for User Story 4

- [X] T124 [P] [US4] Create codegen snapshot tests in crates/gravity-core/tests/codegen_tests.rs
- [X] T125 [US4] Test generated view() function
- [X] T126 [US4] Test generated update() function
- [X] T127 [US4] Test inlined binding expressions

### Code Generation (gravity-core)

- [X] T128 [US4] Add quote dependency to crates/gravity-core/Cargo.toml
- [X] T129 [US4] Create codegen module at crates/gravity-core/src/codegen/mod.rs
- [X] T130 [US4] Implement Application trait generation in crates/gravity-core/src/codegen/application.rs
- [X] T131 [US4] Implement view() generation in crates/gravity-core/src/codegen/view.rs
- [X] T132 [US4] Implement update() generation in crates/gravity-core/src/codegen/update.rs
- [X] T133 [US4] Inline binding expressions as Rust code
- [X] T134 [US4] Generate Message enum from handlers
- [X] T135 [US4] Apply constant folding optimization

### Build Script Integration

- [X] T136 [US4] Create build.rs template for gravity projects
- [X] T137 [US4] Scan for .gravity files in ui/ directory
- [X] T138 [US4] Generate ui_generated.rs in OUT_DIR

### CLI Build Command (gravity-cli)

- [X] T139 [US4] Add clap dependency to crates/gravity-cli/Cargo.toml
- [X] T140 [US4] Implement build command in crates/gravity-cli/src/commands/build.rs
- [X] T141 [US4] Parse XML and generate code to specified output
- [X] T142 [US4] Add compile-time handler validation

**Checkpoint**: `gravity build` generates compilable Rust code

---

## Phase 8: User Story 6 - Validate UI Definitions Before Runtime (Priority: P2)

**Goal**: `gravity check` validates XML and bindings without running the app

**Independent Test**: Invalid XML produces clear error report with exit code 1

### Tests for User Story 6

- [X] T143 [P] [US6] Create validation test fixtures in crates/gravity-cli/tests/check_tests.rs
- [X] T144 [US6] Test invalid widget detection
- [X] T145 [US6] Test unknown handler detection
- [X] T146 [US6] Test binding field validation

### Check Command (gravity-cli)

- [X] T147 [US6] Implement check command in crates/gravity-cli/src/commands/check.rs
- [X] T148 [US6] Parse all .gravity files in project
- [X] T149 [US6] Validate widget names against WidgetKind enum
- [X] T150 [US6] Validate handler references exist
- [X] T151 [US6] Validate binding fields against Model
- [X] T152 [US6] Output errors with spans and suggestions
- [X] T153 [US6] Return exit code 0 on success, 1 on failure

**Checkpoint**: `gravity check` validates project with clear output ✓

---

## Phase 9: User Story 7 - Support All Core Iced Widgets (Priority: P3)

**Note**: The `todo-app` example demonstrates bindings with `{items.len()}`, `{if ...}` working correctly.

**Goal**: Complete widget coverage for all core Iced widgets

**Independent Test**: XML can use any supported widget with all attributes

### Remaining Widget Implementations

- [ ] T154 [P] [US7] Implement container widget in crates/gravity-iced/src/widgets/container.rs
- [ ] T155 [P] [US7] Implement scrollable widget in crates/gravity-iced/src/widgets/scrollable.rs
- [ ] T156 [P] [US7] Implement stack widget in crates/gravity-iced/src/widgets/stack.rs
- [ ] T157 [P] [US7] Implement text_input widget in crates/gravity-iced/src/widgets/text_input.rs
- [ ] T158 [P] [US7] Implement checkbox widget in crates/gravity-iced/src/widgets/checkbox.rs
- [ ] T159 [P] [US7] Implement slider widget in crates/gravity-iced/src/widgets/slider.rs
- [ ] T160 [P] [US7] Implement pick_list widget in crates/gravity-iced/src/widgets/pick_list.rs
- [ ] T161 [P] [US7] Implement toggler widget in crates/gravity-iced/src/widgets/toggler.rs
- [ ] T162 [P] [US7] Implement image widget in crates/gravity-iced/src/widgets/image.rs
- [ ] T163 [P] [US7] Implement svg widget in crates/gravity-iced/src/widgets/svg.rs
- [ ] T164 [P] [US7] Implement space widget in crates/gravity-iced/src/widgets/space.rs
- [ ] T165 [P] [US7] Implement rule widget in crates/gravity-iced/src/widgets/rule.rs

### Widget Attribute Support

- [ ] T166 [US7] Implement width/height constraints parsing
- [ ] T167 [US7] Implement padding attribute parsing (single value and box)
- [ ] T168 [US7] Implement spacing attribute for layouts
- [ ] T169 [US7] Implement align attribute parsing

### Todo App Example

- [ ] T170 [US7] Create examples/todo-app/Cargo.toml
- [ ] T171 [US7] Create examples/todo-app/ui/main.gravity with full widget variety
- [ ] T172 [US7] Create examples/todo-app/src/main.rs demonstrating CRUD operations

**Checkpoint**: All core Iced widgets available in XML

---

## Phase 10: User Story 8 - Debug IR and Generated Code (Priority: P3)

**Goal**: `gravity inspect` displays IR tree and optionally generated code

**Independent Test**: IR tree output shows widget hierarchy and bindings

### Inspect Command (gravity-cli)

- [ ] T173 [US8] Implement inspect command in crates/gravity-cli/src/commands/inspect.rs
- [ ] T174 [US8] Display IR tree in human-readable format
- [ ] T175 [US8] Add --codegen flag to show generated Rust code
- [ ] T176 [US8] Add --format json flag for machine-readable output
- [ ] T177 [US8] Display binding expressions with resolved types

### CLI Integration

- [ ] T178 [US8] Create crates/gravity-cli/src/commands/mod.rs with subcommand routing
- [ ] T179 [US8] Implement dev command in crates/gravity-cli/src/commands/dev.rs
- [ ] T180 [US8] Implement config loading in crates/gravity-cli/src/config.rs
- [ ] T181 [US8] Add gravity.toml parser

**Checkpoint**: `gravity inspect` displays useful debug information

---

## Phase 11: Polish & Cross-Cutting Concerns

**Purpose**: Documentation, examples, and release preparation

- [ ] T182 [P] Create README.md with installation and quick start
- [ ] T183 [P] Create docs/QUICKSTART.md from specs/001-framework-technical-specs/quickstart.md
- [ ] T184 [P] Create docs/XML_SCHEMA.md from specs/001-framework-technical-specs/contracts/xml-schema.md
- [ ] T185 Create examples/full-demo/ showcasing all features
- [ ] T186 [P] Add rustdoc comments to all public types in gravity-core
- [ ] T187 [P] Add rustdoc comments to all public macros in gravity-macros
- [ ] T188 Run cargo doc --workspace and verify no warnings
- [ ] T189 [P] Add proptest dependency for property-based testing
- [ ] T190 [P] Add insta dependency for snapshot testing
- [ ] T191 Create property-based parser tests in crates/gravity-core/tests/proptest_parser.rs
- [ ] T192 Create codegen snapshot tests in crates/gravity-core/tests/snapshots/
- [ ] T193 Benchmark XML parse time for 1000-widget file
- [ ] T194 Benchmark hot-reload latency
- [ ] T195 Prepare crates for crates.io publication (metadata, license)

---

## Dependencies & Execution Order

### Phase Dependencies

```text
Phase 1 (Setup) ─────────────────────────────────────────────────┐
                                                                  │
Phase 2 (Foundational) ──────────────────────────────────────────┤
        │                                                         │
        ├─► Phase 3 (US1: Declarative UI) ──────► MVP            │
        │           │                                             │
        │           ▼                                             │
        ├─► Phase 4 (US3: Handlers) ─────────────────────────────┤
        │           │                                             │
        │           ▼                                             │
        ├─► Phase 5 (US5: Bindings) ─────────────────────────────┤
        │           │                                             │
        │           ▼                                             │
        ├─► Phase 6 (US2: Hot-Reload) ───────────────────────────┤
        │           │                                             │
        │           ▼                                             │
        ├─► Phase 7 (US4: Codegen) ──────────────────────────────┤
        │           │                                             │
        │           ▼                                             │
        ├─► Phase 8 (US6: Validation) ───────────────────────────┤
        │           │                                             │
        │           ▼                                             │
        ├─► Phase 9 (US7: All Widgets) ──────────────────────────┤
        │           │                                             │
        │           ▼                                             │
        └─► Phase 10 (US8: Debug) ───────────────────────────────┤
                    │                                             │
                    ▼                                             │
            Phase 11 (Polish) ◄───────────────────────────────────┘
```

### User Story Dependencies

| Story | Depends On | Can Parallel With |
|-------|------------|-------------------|
| US1 (Declarative UI) | Foundational | None (MVP start) |
| US3 (Handlers) | US1 | - |
| US5 (Bindings) | US3 | - |
| US2 (Hot-Reload) | US5 | - |
| US4 (Codegen) | US5 | US2 |
| US6 (Validation) | US4 | - |
| US7 (Widgets) | US1 | US3-US6 |
| US8 (Debug) | US4 | US6 |

### Parallel Opportunities Within Phases

**Phase 2 (Foundational)**:
```bash
# All IR types can be created in parallel:
T011, T012, T014, T015  # Different files
T019-T025               # Different expression types
T028, T029              # Different error types
```

**Phase 3 (US1)**:
```bash
# Widget implementations in parallel:
T052, T053, T054, T055  # Different widget files
```

**Phase 9 (US7)**:
```bash
# All widgets in parallel:
T154-T165               # Each widget is independent
```

---

## Implementation Strategy

### MVP First (Phases 1-3)

1. Complete Phase 1: Setup workspace
2. Complete Phase 2: Core IR types
3. Complete Phase 3: US1 - Declarative UI parsing
4. **STOP and VALIDATE**: Run `examples/hello-world`
5. Demo: Static XML renders through Iced

### Interactive Demo (Phases 4-5)

1. Complete Phase 4: US3 - Handler system
2. Complete Phase 5: US5 - Binding system
3. **VALIDATE**: Run `examples/counter`
4. Demo: Interactive UI with state bindings

### Dev Experience (Phase 6)

1. Complete Phase 6: US2 - Hot-reload
2. **VALIDATE**: Modify XML, see live updates
3. Demo: Full development workflow

### Production Ready (Phases 7-11)

1. Complete Phase 7: US4 - Code generation
2. Complete remaining phases
3. **VALIDATE**: Benchmark performance
4. Publish to crates.io

---

## Task Summary

| Phase | User Story | Task Count | Parallel Tasks |
|-------|------------|------------|----------------|
| 1 | Setup | 10 | 6 |
| 2 | Foundational | 23 | 17 |
| 3 | US1 - Declarative UI | 27 | 8 |
| 4 | US3 - Handlers | 18 | 2 |
| 5 | US5 - Bindings | 24 | 5 |
| 6 | US2 - Hot-Reload | 21 | 1 |
| 7 | US4 - Codegen | 19 | 1 |
| 8 | US6 - Validation | 11 | 1 |
| 9 | US7 - All Widgets | 19 | 12 |
| 10 | US8 - Debug | 9 | 0 |
| 11 | Polish | 14 | 7 |
| **TOTAL** | | **195** | **60** |

---

## Notes

- [P] tasks = different files, no dependencies
- [Story] label maps task to specific user story for traceability
- Each user story should be independently completable and testable
- Verify tests fail before implementing (TDD per Constitution Principle V)
- Commit after each task or logical group
- Stop at any checkpoint to validate story independently
- Avoid: vague tasks, same file conflicts, cross-story dependencies that break independence
