# Feature Specification: Gravity Framework Technical Specifications

**Feature Branch**: `001-framework-technical-specs`  
**Created**: 2025-12-30  
**Status**: Draft  
**Input**: Complete technical specifications for 9 core modules of the Gravity declarative UI framework

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Define UI Structure Declaratively (Priority: P1)

A developer wants to create a GUI application by writing XML markup instead of imperative Rust code. They define their UI structure, widget hierarchy, and data bindings in a `.gravity` XML file, then connect it to their Rust application state.

**Why this priority**: This is the foundational capability - without declarative UI definition, the framework has no purpose. All other features depend on this core functionality.

**Independent Test**: A developer can write a complete UI layout in XML with static content, compile it, and see the rendered UI without writing any view logic in Rust.

**Acceptance Scenarios**:

1. **Given** a valid XML file with widget definitions, **When** the framework parses it, **Then** an intermediate representation (IR) is produced containing the complete widget tree
2. **Given** an XML file with binding expressions like `{counter}`, **When** parsed, **Then** the IR contains typed expression nodes that can be evaluated against a Model
3. **Given** an XML file with syntax errors, **When** parsed, **Then** error messages include line/column positions and clear explanations

---

### User Story 2 - Hot-Reload UI During Development (Priority: P1)

A developer iterating on their UI wants to see changes immediately without recompiling. They modify the XML file and the running application updates within 500ms.

**Why this priority**: Fast iteration is critical for UI development. This is a core differentiator from static Rust UI code.

**Independent Test**: Developer can start the app, modify an XML attribute (e.g., button text), save, and see the change reflected in the running app without restart.

**Acceptance Scenarios**:

1. **Given** a running application in dev mode, **When** the XML file is modified and saved, **Then** the UI updates within 500ms
2. **Given** application state (Model) with values, **When** hot-reload occurs, **Then** the Model state is preserved after reload
3. **Given** an XML modification that introduces a parsing error, **When** hot-reload triggers, **Then** an error overlay displays the issue without crashing the app

---

### User Story 3 - Connect UI Events to Typed Handlers (Priority: P1)

A developer wants to define event handlers in Rust that respond to UI events declared in XML. The handler receives typed parameters and the connection is verified at compile time.

**Why this priority**: Without event handling, the UI cannot be interactive. Type safety is a core promise of the framework.

**Independent Test**: Developer can define a handler in Rust, reference it in XML via `on_click="handler_name"`, and verify at compile time that the handler exists and has the correct signature.

**Acceptance Scenarios**:

1. **Given** an XML element with `on_click="increment"`, **When** the code is compiled, **Then** the framework verifies a handler named `increment` exists
2. **Given** a handler `fn on_input(model: &mut Model, value: String)`, **When** bound to a text input's `on_change`, **Then** the value parameter receives the input text
3. **Given** a handler returning `Command<Message>`, **When** executed, **Then** the command is dispatched through Iced's command system

---

### User Story 4 - Generate Production Code Without Runtime Overhead (Priority: P2)

A developer wants to ship their application without XML parsing overhead. The build process generates static Rust code equivalent to hand-written Iced code.

**Why this priority**: Production performance is essential, but developers can work with dev mode initially. This enables shipping optimized binaries.

**Independent Test**: Developer can run `gravity build` and verify the generated code compiles without the runtime interpreter and produces the same UI behavior.

**Acceptance Scenarios**:

1. **Given** a valid XML UI definition, **When** `gravity build` runs, **Then** Rust source code is generated implementing the Iced Application trait
2. **Given** binding expressions in XML, **When** code is generated, **Then** expressions are inlined as Rust code with no runtime evaluation
3. **Given** the generated code, **When** compiled with `--release`, **Then** binary size is comparable to hand-written Iced code

---

### User Story 5 - Derive Bindable Model from Rust Struct (Priority: P2)

A developer wants their Rust struct to automatically expose fields for XML binding. They add `#[derive(UiModel)]` and the framework generates the necessary accessors and traits.

**Why this priority**: Reduces boilerplate and ensures type safety between Model and bindings. Can be deferred after core parsing works.

**Independent Test**: Developer adds `#[derive(UiModel)]` to a struct and can immediately use `{field_name}` in XML to bind to that field.

**Acceptance Scenarios**:

1. **Given** a struct with `#[derive(UiModel)]`, **When** compiled, **Then** getter methods are generated for each field
2. **Given** a field marked `#[ui_skip]`, **When** the macro runs, **Then** no binding accessor is generated for that field
3. **Given** nested structs, **When** using `{parent.child.field}` in XML, **Then** the binding resolves through the accessor chain

---

### User Story 6 - Validate UI Definitions Before Runtime (Priority: P2)

A developer wants to catch errors in their XML and bindings before running the application. The CLI provides validation commands.

**Why this priority**: Early error detection improves developer experience. Can work without this initially by seeing errors at runtime.

**Independent Test**: Developer runs `gravity check` and receives a report of all XML errors and binding mismatches without starting the app.

**Acceptance Scenarios**:

1. **Given** an XML file with invalid widget names, **When** `gravity check` runs, **Then** errors list the invalid elements with suggestions
2. **Given** a binding `{nonexistent_field}` referencing a field not in the Model, **When** validated, **Then** the error identifies the binding location and available fields
3. **Given** valid XML and Model, **When** `gravity check` runs, **Then** exit code is 0 with no output

---

### User Story 7 - Support All Core Iced Widgets (Priority: P3)

A developer needs to use the full range of Iced widgets (button, text, column, row, container, scrollable, text_input, checkbox, slider, pick_list, etc.) from XML.

**Why this priority**: Complete widget coverage is needed for real applications, but initial version can launch with a subset.

**Independent Test**: Developer can write XML using any supported Iced widget and it renders correctly with all attributes functional.

**Acceptance Scenarios**:

1. **Given** a `<slider min="0" max="100" value="{progress}" on_change="set_progress" />`, **When** rendered, **Then** the slider reflects the model value and updates on drag
2. **Given** a `<pick_list options="{items}" selected="{current}" on_select="choose" />`, **When** rendered, **Then** the dropdown shows all options and fires events on selection
3. **Given** layout widgets `<column>`, `<row>`, `<container>`, **When** nested, **Then** child widgets are laid out according to Iced's layout system

---

### User Story 8 - Debug IR and Generated Code (Priority: P3)

A developer troubleshooting issues wants to inspect the intermediate representation and generated code to understand what the framework produces.

**Why this priority**: Debugging tools are important for adoption but not required for basic functionality.

**Independent Test**: Developer runs `gravity inspect` and can see the parsed IR tree and optionally the generated Rust code.

**Acceptance Scenarios**:

1. **Given** an XML file, **When** `gravity inspect` runs, **Then** the IR tree is printed in a readable format showing widget hierarchy and bindings
2. **Given** the `--codegen` flag, **When** `gravity inspect --codegen` runs, **Then** the generated Rust code is printed without writing to files
3. **Given** `--format json`, **When** running inspect, **Then** output is machine-readable JSON for tooling integration

---

### Edge Cases

- What happens when XML references a handler that doesn't exist? Compile-time error with handler name and XML location.
- How does the system handle circular or self-referential bindings? Binding expressions are evaluated once per render; cycles are not possible in unidirectional flow.
- What happens when Model struct changes but serialized state exists from hot-reload? Migration strategy attempts field-by-field restoration; unmatched fields use defaults.
- How are binding type mismatches handled? Compile-time verification in prod mode; runtime error overlay in dev mode.
- What happens with deeply nested XML (>100 levels)? Parser handles arbitrary depth; performance may degrade but no hard limit.

## Requirements *(mandatory)*

### Functional Requirements

#### Module 1: MVU-D Architecture (Model-View-Update Declarative)

- **FR-001**: System MUST implement unidirectional data flow: Model -> View -> User Action -> Message -> Update -> Model
- **FR-002**: System MUST generate a ViewModel layer that exposes bindable accessors and maps events to Messages
- **FR-003**: System MUST support binding expression syntax: `{field}`, `{object.field}`, `{expr}` in XML attributes
- **FR-004**: System MUST support binding types: simple read, formatted read, conditional, and dynamic style bindings
- **FR-005**: System MUST serialize Model state before hot-reload and restore after XML reconstruction
- **FR-006**: System MUST provide migration strategy when Model structure changes between reloads

#### Module 2: XML Format and UI Schema

- **FR-007**: System MUST define XML elements for all core Iced widgets: button, text, column, row, container, scrollable, text_input, checkbox, slider, pick_list, image, svg, space, rule
- **FR-008**: System MUST support widget attributes: id, style, layout constraints (width, height, padding, spacing), event bindings (on_click, on_change, on_input, on_submit)
- **FR-009**: System MUST use `on_<event>="handler_name"` syntax for event handler references
- **FR-010**: System MUST support namespace and schema versioning for forward compatibility
- **FR-011**: System MUST support dynamic expressions in any attribute value using `{expr}` syntax

#### Module 3: Intermediate Representation (IR/AST)

- **FR-012**: System MUST produce an IR structure representing: widget tree hierarchy, event bindings, style references, and parsed binding expressions
- **FR-013**: System MUST include source location (line/column) in all IR nodes for error reporting
- **FR-014**: System MUST support IR serialization for caching between builds
- **FR-015**: System MUST produce actionable error messages with context when parsing fails

#### Module 4: Typed Handler System

- **FR-016**: System MUST provide `#[ui_handler]` macro attribute to mark event handler functions
- **FR-017**: System MUST support handler signatures: `fn(model: &mut Model)`, `fn(model: &mut Model, value: T)`, `fn(model: &mut Model) -> Command<Message>`
- **FR-018**: System MUST generate Message enum variants corresponding to declared handlers
- **FR-019**: System MUST verify at compile-time that handlers referenced in XML exist and have compatible signatures

#### Module 5: UiModel Derive Macro

- **FR-020**: System MUST provide `#[derive(UiModel)]` macro generating binding accessors for struct fields
- **FR-021**: System MUST support types: primitives, String, Vec<T>, Option<T>, nested UiModel structs
- **FR-022**: System MUST support control attributes: `#[ui_bind]`, `#[ui_skip]`, `#[ui_format="..."]`
- **FR-023**: System MUST define and implement `UiBindable` trait on derived types

#### Module 6: Runtime Interpreter (Dev Mode)

- **FR-024**: System MUST parse XML files at application startup in dev mode
- **FR-025**: System MUST evaluate binding expressions against the current Model state
- **FR-026**: System MUST integrate file watcher to detect XML modifications
- **FR-027**: System MUST reconstruct UI tree on file change while preserving Model state
- **FR-028**: System MUST display parsing errors in an overlay without crashing the application
- **FR-029**: System MUST display binding errors (missing field, type mismatch) in the error overlay

#### Module 7: Code Generator (Production Mode)

- **FR-030**: System MUST generate Rust code implementing Iced's Application trait
- **FR-031**: System MUST inline binding expressions as Rust code (no runtime evaluation)
- **FR-032**: System MUST generate `view()` function with statically-compiled widget tree
- **FR-033**: System MUST generate `update()` function dispatching to user handlers
- **FR-034**: System MUST apply optimizations: dead code elimination, constant expression pre-evaluation

#### Module 8: Iced Backend Abstraction

- **FR-035**: System MUST define a backend trait separating IR from rendering implementation
- **FR-036**: System MUST map IR widget nodes to Iced 0.14+ widget constructors
- **FR-037**: System MUST support Iced theming (Theme, custom styles)
- **FR-038**: System MUST support Iced subscriptions for system events and timers
- **FR-039**: System MUST integrate with Iced's Command system for side effects

#### Module 9: CLI Developer Experience

- **FR-040**: System MUST provide `gravity dev` command starting hot-reload mode
- **FR-041**: System MUST provide `gravity build` command generating production code
- **FR-042**: System MUST provide `gravity check` command validating XML and bindings
- **FR-043**: System MUST provide `gravity inspect` command displaying IR (debug)
- **FR-044**: System MUST support configuration via `gravity.toml` or `Cargo.toml` metadata

### Key Entities

- **Model**: User-defined Rust struct representing application state; must be serializable for hot-reload
- **Message**: Enum type generated partially by framework (standard events) and extended by user handlers
- **Widget IR Node**: Intermediate representation of a widget with type, attributes, children, bindings, and source location
- **Binding Expression**: Parsed AST representing a `{expr}` with type information and referenced fields
- **Handler Registration**: Mapping from handler name to function signature, used for validation and code generation
- **Backend Trait**: Interface defining widget construction, layout, and event mapping for a specific renderer

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Developers can create a complete UI application using only XML for layout and Rust for state/handlers
- **SC-002**: Hot-reload reflects XML changes in the running application within 500ms
- **SC-003**: Application state is preserved across 95% of hot-reload cycles (excluding Model structure changes)
- **SC-004**: Compile-time validation catches 100% of handler name mismatches between XML and Rust
- **SC-005**: Generated production code performs equivalently to hand-written Iced code (within 5% on benchmarks)
- **SC-006**: Error messages include source location and actionable fix suggestions in 100% of parsing failures
- **SC-007**: XML parsing completes within 10ms for files with up to 1000 widgets
- **SC-008**: Production code generation completes within 5 seconds for typical applications
- **SC-009**: Developers can implement any Iced example application using the declarative XML format
- **SC-010**: Framework supports Windows, Linux, and macOS without platform-specific XML syntax

### Assumptions

- Iced 0.14+ API remains stable for Application trait and widget constructors
- Rust Edition 2024 provides required proc-macro capabilities
- Developers are familiar with Iced concepts (Message, Command, Subscription)
- XML is an acceptable format for the target developer audience (vs. custom DSL)
- Hot-reload state preservation via serde is sufficient (no persistent disk state required)
