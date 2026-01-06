# Feature Specification: Automatic UI File Loading with AppState Structure

**Feature Branch**: `006-auto-ui-loading`  
**Created**: 2026-01-06  
**Status**: Draft  
**Input**: User description: "Automatic loading of .gravity UI files based on co-located .gravity.rs files, with a global AppState structure containing GravityDocument (mandatory), Model (optional), and HandlerRegistry (optional)"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Create a Simple Gravity UI Application (Priority: P1)

As a Gravity developer, I want to create a new UI view by placing a `.gravity` XML file alongside a `.gravity.rs` file in the `ui/` directory, so that I don't have to manually configure file paths or create AppState boilerplate.

**Why this priority**: This is the core workflow improvement that reduces friction for new and existing Gravity projects. Every developer will use this pattern daily.

**Independent Test**: Can be tested by creating a new project with `ui/app.gravity` and `ui/app.gravity.rs`, then running `gravity check` to verify the XML is valid and the AppState is properly generated. No other user stories depend on this being complete.

**Acceptance Scenarios**:

1. **Given** a new project with `ui/app.gravity` containing valid XML, **When** the developer creates `ui/app.gravity.rs` with an empty module, **Then** the XML file is automatically loaded at compile time without explicit `include_str!` macros.

2. **Given** a project with `ui/settings.gravity` and `ui/settings.gravity.rs`, **When** the Rust compiler processes the module, **Then** a GravityDocument is automatically created from the XML content.

3. **Given** a project with the standard structure `src/main.rs` and `ui/mod.rs` importing `ui/app.gravity.rs`, **When** `main.rs` calls the AppState initialization, **Then** the complete UI state including document, model, and handlers is available.

---

### User Story 2 - Define AppState with Optional Components (Priority: P1)

As a Gravity developer, I want to define an AppState structure that can include a Model and HandlerRegistry optionally, so that simple UIs don't require complex state management boilerplate.

**Why this priority**: Enables simple static UIs (no handlers, no model) while supporting full interactive applications. This is essential for backward compatibility and supporting different complexity levels.

**Independent Test**: Can be tested by creating an AppState with only GravityDocument, another with Model only, and another with all three components. Each should compile and function correctly with GravityWidgetBuilder.

**Acceptance Scenarios**:

1. **Given** an AppState containing only GravityDocument, **When** GravityWidgetBuilder::new is called, **Then** the UI renders correctly with empty bindings and no handler support.

2. **Given** an AppState containing GravityDocument and Model, **When** bindings are evaluated in the UI, **Then** field values from the Model are correctly displayed.

3. **Given** an AppState containing all three components, **When** button clicks trigger handlers, **Then** the registered handlers execute with access to the Model.

4. **Given** an existing Gravity application using manual AppState creation, **When** the application is migrated to the new pattern, **Then** all existing functionality continues to work without modification.

---

### User Story 3 - Reduce Main.rs Boilerplate (Priority: P2)

As a Gravity developer, I want my main.rs to simply import and use the AppState from ui/mod.rs, so that the entry point is minimal and focuses on application configuration rather than UI loading.

**Why this priority**: Improves developer experience by reducing boilerplate and making the main entry point clean and readable. This is a quality-of-life improvement for all Gravity projects.

**Independent Test**: Can be tested by creating a main.rs that imports AppState from ui/mod.rs and runs the application. The resulting binary should function identically to a manually configured version.

**Acceptance Scenarios**:

1. **Given** a main.rs that imports AppState from ui/mod.rs, **When** the application runs, **Then** the UI displays correctly with all handlers functioning.

2. **Given** a developer reading main.rs, **When** they look for UI configuration, **Then** they find it clearly organized in ui/ directory with minimal code in main.rs.

---

### User Story 4 - Support Multiple UI Views (Priority: P3)

As a Gravity developer, I want to define multiple UI views (app.gravity, settings.gravity, about.gravity) in the same project, so that I can build applications with multiple screens or dialogs.

**Why this priority**: Enables complex applications with multiple views. Lower priority because single-view applications are more common and this builds on the core auto-loading feature.

**Independent Test**: Can be tested by creating a project with three different .gravity files and their corresponding .gravity.rs files, then switching between them at runtime.

**Acceptance Scenarios**:

1. **Given** a project with `ui/app.gravity.rs` and `ui/settings.gravity.rs`, **When** ui/mod.rs exports both AppStates, **Then** main.rs can choose which UI to load.

2. **Given** a running application with multiple views, **When** the user navigates between views, **Then** each view correctly loads its associated GravityDocument and handlers.

---

### Edge Cases

- What happens when a `.gravity.rs` file exists but the corresponding `.gravity` file is missing? (Error with clear message indicating missing file)
- What happens when the XML in the `.gravity` file is invalid? (Parse error reported with line/column information)
- What happens when a handler references a non-existent handler name in the XML? (Warning at parse time, error at runtime)
- What happens with large .gravity files (1000+ widgets)? (Performance remains under 10ms parse time)
- What happens when two .gravity.rs files try to load the same .gravity file? (Each gets its own GravityDocument instance)

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST automatically load `.gravity` files when a corresponding `.gravity.rs` file is compiled, without requiring explicit `include_str!` or file path configuration.
- **FR-002**: System MUST locate `.gravity` files by replacing the `.rs` extension with `.gravity` in the same directory.
- **FR-003**: System MUST support UI files stored in the `ui/` directory convention, as specified by the project structure.
- **FR-004**: System MUST define an `AppState` structure in `gravity_core` containing:
  - GravityDocument (mandatory, always present)
  - Model (optional, defaults to unit type if not specified)
  - HandlerRegistry (optional, defaults to empty registry if not specified)
- **FR-005**: AppState MUST be compatible with `GravityWidgetBuilder::new()` and `GravityWidgetBuilder::from_document()` for UI rendering.
- **FR-006**: System MUST NOT break existing Gravity applications that use manual AppState creation with `include_str!`.
- **FR-007**: System MUST provide clear error messages when:
  - A `.gravity.rs` file references a missing `.gravity` file
  - The XML in a `.gravity` file is invalid
  - A handler is referenced but not registered
- **FR-008**: System MUST support auto-loading in both dev mode (hot-reload) and production builds without differences in behavior.
- **FR-009**: Existing `gravity dev` and `gravity check` commands MUST continue to work with projects using both old and new patterns.
- **FR-010**: AppState defined in each `.gravity.rs` file MUST be accessible from `ui/mod.rs` through standard Rust module exports.

### Key Entities

- **AppState**: A struct in gravity_core containing the core application state for a UI view. Contains three fields: `document: GravityDocument` (required), `model: impl UiBindable` (optional, defaults to `()`), and `handler_registry: HandlerRegistry` (optional, defaults to empty).
- **GravityDocument**: Existing structure containing the parsed UI tree, themes, and style classes. Remains unchanged.
- **UiModule**: A convention for co-located UI definition files where `<filename>.gravity.rs` automatically loads `<filename>.gravity` and provides an AppState.
- **HandlerRegistry**: Existing structure for registering and dispatching event handlers. Remains unchanged.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Developers can create a new Gravity UI view by adding two files (`.gravity` and `.gravity.rs`) without any file path configuration or boilerplate in main.rs.
- **SC-002**: Migration of existing projects to the new pattern requires zero code changes to existing AppState logic, only moving AppState to the appropriate `.gravity.rs` file.
- **SC-003**: Existing applications using manual `include_str!` loading continue to compile and run without modification.
- **SC-004**: The `gravity check` command validates `.gravity` files referenced by `.gravity.rs` files without additional configuration.
- **SC-005**: Parse time for auto-loaded `.gravity` files remains under 10ms for files containing up to 1000 widgets.
- **SC-006**: Error messages for missing or invalid `.gravity` files include the file path and line/column information, enabling developers to quickly locate issues.
- **SC-007**: Developer satisfaction with UI creation workflow improves, measured by reduced lines of boilerplate code in main.rs (target: 50% reduction in main.rs LOC for typical applications).

## Assumptions

- The AppState structure will be defined as a generic struct in gravity_core that can be instantiated in each `.gravity.rs` file.
- The auto-loading mechanism will use a Rust procedural macro or build script to inject the XML content at compile time.
- The `ui/mod.rs` file will serve as the central export point for all AppState instances in the project.
- Existing examples (hello-world, counter, todo-app) will be migrated to the new pattern to demonstrate the workflow.
- The gravity-cli commands will detect and handle both old and new project structures transparently.
- Handler registration will continue to use the existing HandlerRegistry API in each `.gravity.rs` file.

## Dependencies

- Existing GravityDocument structure (gravity-core/src/ir/mod.rs)
- Existing HandlerRegistry structure (gravity-core/src/handler/mod.rs)
- Existing UiBindable trait (gravity-core/src/binding/mod.rs)
- Existing GravityWidgetBuilder (gravity-iced/src/builder.rs)
- Existing gravity-runtime interpreter and file watcher
- Existing gravity-cli commands (dev, check)

## Out of Scope

- Runtime UI view switching (handled by application logic, not the loading mechanism)
- Custom file extensions or alternate directory structures (stick to `ui/` and `.gravity` extension)
- Pre-compiled UI bundles (focus on compile-time loading)
- Network-based UI loading (keep files local for now)
- UI hot-reload during development (existing watcher handles this, auto-loading is compile-time)

## Open Questions

None - requirements are clear from the feature description and codebase analysis.
