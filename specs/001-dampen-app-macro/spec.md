# Feature Specification: Auto-Discovery Multi-View Application with #[dampen_app] Macro

**Feature Branch**: `001-dampen-app-macro`  
**Created**: 2026-01-12  
**Status**: Draft  
**Input**: User description: "Implement a procedural macro `#[dampen_app]` that automatically discovers `.dampen` UI files in the `src/ui` directory (recursively) and generates all boilerplate code for multi-view Dampen applications."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Automatic View Discovery and Initialization (Priority: P1)

As a Dampen application developer, I want the framework to automatically discover all my `.dampen` UI files and generate the necessary initialization code, so that I don't have to manually register each view in my application struct.

**Why this priority**: This is the core value proposition - eliminating boilerplate. Without this, developers gain no benefit from the macro.

**Independent Test**: Can be fully tested by creating a multi-view application with 3-5 `.dampen` files in `src/ui/`, applying the `#[dampen_app]` macro, compiling the project, and verifying that all views are discoverable and the application initializes successfully without manual view registration code.

**Acceptance Scenarios**:

1. **Given** a Dampen project with 3 `.dampen` files in `src/ui/` (e.g., `window.dampen`, `button.dampen`, `text.dampen`), **When** developer applies `#[dampen_app]` macro to the app struct and compiles, **Then** the macro generates a `CurrentView` enum with 3 variants and the app struct contains typed `AppState` fields for each view
2. **Given** a project with nested UI files in `src/ui/widgets/button/button.dampen`, **When** macro runs, **Then** it discovers the nested file and generates appropriate module path handling (preserving directory structure)
3. **Given** a project with 20 `.dampen` files, **When** developer compiles, **Then** compilation completes in under 5 seconds (file discovery overhead < 200ms as per spec requirement)

---

### User Story 2 - View Switching Without Manual Routing (Priority: P1)

As a Dampen application developer, I want view switching to work automatically when users trigger navigation actions, so that I don't have to write manual routing logic in my `update()` method.

**Why this priority**: Routing logic is the most error-prone and repetitive part (80 lines per spec). This is critical for the macro's value proposition.

**Independent Test**: Can be fully tested by creating a multi-view app with `switch_to_*` handlers, triggering view switches via button clicks, and verifying that the current view changes correctly without any manual `match` statements in user code.

**Acceptance Scenarios**:

1. **Given** an app with views `window` and `settings`, **When** user clicks a button that triggers `switch_to_settings` handler, **Then** the current view changes to `settings` and the settings view renders
2. **Given** the current view is `settings`, **When** user triggers `switch_to_window`, **Then** the app switches back to the window view
3. **Given** an app with 20 views, **When** switching between any two views, **Then** view transition completes in under 100ms

---

### User Story 3 - Hot-Reload for Multi-View Apps (Priority: P2)

As a Dampen application developer, I want hot-reload to work seamlessly across all discovered views, so that I can iterate quickly without manually configuring file watchers for each view.

**Why this priority**: Enhances developer experience significantly but the app is still functional without it (works in production mode).

**Independent Test**: Can be fully tested by running the app in dev mode, editing a `.dampen` file, and verifying that the view updates in the running application without restart.

**Acceptance Scenarios**:

1. **Given** an app running in dev mode with 5 views, **When** developer edits one `.dampen` file and saves, **Then** only that view reloads (not the entire app) and the change is visible within 500ms
2. **Given** a running app, **When** developer adds a new `.dampen` file to `src/ui/`, **Then** the system detects that a recompilation is needed and provides clear feedback
3. **Given** an app with an error in a `.dampen` file, **When** hot-reload triggers, **Then** an error overlay displays the parse error with file path and line number, dismissible via the `DismissError` message variant

---

### User Story 4 - Selective View Exclusion (Priority: P3)

As a Dampen application developer, I want to exclude certain `.dampen` files from auto-discovery (e.g., experimental or debug views), so that I have fine-grained control over which views are included in my application.

**Why this priority**: Nice-to-have customization feature. Most apps won't need this immediately.

**Independent Test**: Can be fully tested by adding `exclude = ["debug_view"]` to the macro attributes, verifying that the excluded view doesn't appear in the generated `CurrentView` enum, and confirming the app compiles successfully.

**Acceptance Scenarios**:

1. **Given** a project with `debug_view.dampen` in `src/ui/`, **When** developer adds `exclude = ["debug_view"]` to macro attributes, **Then** the `CurrentView` enum does not contain a `DebugView` variant
2. **Given** `exclude = ["experimental/*"]` pattern, **When** developer creates `experimental/new_feature.dampen`, **Then** it is excluded from discovery
3. **Given** an excluded view, **When** developer removes it from the exclusion list, **Then** recompilation generates the appropriate variant and fields

---

### User Story 5 - Clear Compile-Time Error Messages (Priority: P2)

As a Dampen application developer, I want the macro to provide clear, actionable error messages when I violate conventions (e.g., missing `.rs` file for a `.dampen` file), so that I can quickly fix configuration issues.

**Why this priority**: Critical for usability, but not part of the "happy path" functionality. Prevents developer frustration.

**Independent Test**: Can be fully tested by intentionally creating violations (e.g., `.dampen` file without matching `.rs` file), attempting to compile, and verifying that error messages include file paths, problem descriptions, and suggested fixes.

**Acceptance Scenarios**:

1. **Given** a `.dampen` file without a matching `.rs` file, **When** developer compiles, **Then** the compiler error message shows the file path, states "No matching Rust module found", and suggests creating the `.rs` file or excluding it
2. **Given** `ui_dir = "src/nonexistent"` in macro attributes, **When** compilation runs, **Then** error message states "UI directory 'src/nonexistent' does not exist" with suggestion to check the path
3. **Given** two `.dampen` files with the same name in different directories (`ui/form/input.dampen` and `ui/dialog/input.dampen`), **When** macro runs, **Then** compilation fails with error stating "View naming conflict: 'input' found in multiple locations" and lists both paths

---

### Edge Cases

- What happens when `src/ui/` directory exists but contains no `.dampen` files?
  - Macro generates a warning but compiles successfully with empty `CurrentView` enum (allowing gradual adoption)
- What happens when a `.dampen` file has a corresponding `.rs` file but no `Model` struct?
  - Compilation fails with type error when attempting to use `AppState<ui::view_name::Model>` (caught by Rust's type system)
- What happens when a view name uses invalid Rust identifier characters?
  - Discovery skips files with invalid names and emits a warning listing the problematic file
- What happens when the `ui_dir` path is relative vs absolute?
  - Macro resolves paths relative to the crate root (standard Rust proc macro behavior)
- What happens in nested modules when two directories have files with the same name?
  - Compilation fails with naming conflict error (as per User Story 5, scenario 3)
- What happens when a `.rs` file exists without a corresponding `.dampen` file?
  - Macro emits a compile-time warning about "orphaned .rs file" but continues (non-breaking)

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: Macro MUST recursively scan the directory specified by `ui_dir` attribute to discover all files with `.dampen` extension
- **FR-002**: Macro MUST verify that each discovered `.dampen` file has a corresponding `.rs` file with the same base name (e.g., `button.dampen` requires `button.rs`)
- **FR-003**: Macro MUST generate a `CurrentView` enum with one variant per discovered view, using PascalCase conversion from the file name (e.g., `text_input.dampen` → `TextInput` variant)
- **FR-004**: Macro MUST generate app struct fields in the format `{view_name}_state: AppState<ui::{module_path}::Model>` for each discovered view
- **FR-005**: Macro MUST generate an `init()` function that initializes all `AppState` fields and returns the app struct with initial view selection
- **FR-006**: Macro MUST generate an `update()` function that handles `switch_to_*` handlers and dispatches to the appropriate view's `AppState`
- **FR-007**: Macro MUST generate a `view()` function that renders the current view based on `current_view` field value
- **FR-008**: Macro MUST generate a `subscription()` function that sets up file watching for all discovered `.dampen` files (when `hot_reload_variant` is specified)
- **FR-009**: Macro MUST generate a `dispatch_handler()` function that routes handler calls to the correct view's `AppState`
- **FR-010**: Macro MUST preserve nested directory structure as nested modules (e.g., `ui/widgets/button/button.dampen` → `ui::widgets::button::Model`)
- **FR-011**: Macro MUST support exclusion of views via `exclude` attribute with glob pattern matching (e.g., `exclude = ["debug_view", "experimental/*"]`)
- **FR-012**: Macro MUST emit a compile error with file path and suggested fix when a `.dampen` file has no matching `.rs` file
- **FR-013**: Macro MUST emit a compile error when `ui_dir` path does not exist or is not a directory
- **FR-014**: Macro MUST emit a compile error when multiple `.dampen` files resolve to the same view name (naming conflict)
- **FR-015**: Macro MUST emit a warning when `ui_dir` contains no `.dampen` files and generate empty implementations
- **FR-016**: Macro MUST sort discovered views alphabetically to ensure deterministic code generation
- **FR-017**: Macro MUST accept required attributes: `ui_dir`, `message_type`, `handler_variant`
- **FR-018**: Macro MUST accept optional attributes: `hot_reload_variant`, `dismiss_error_variant`, `exclude`
- **FR-019**: Generated code MUST integrate with `dampen-dev` crate's `FileEvent` type when `hot_reload_variant` is specified
- **FR-020**: Generated code MUST handle hot-reload errors by displaying an error overlay (when `dismiss_error_variant` is specified)

### Key Entities

- **View**: Represents a UI screen defined by a `.dampen` file and corresponding `.rs` file with a `Model` struct
  - Attributes: view name (snake_case from filename), variant name (PascalCase), module path, file paths
  - Relationships: Each view has one Model struct, one AppState instance, one entry in CurrentView enum
- **CurrentView Enum**: Generated enum containing all discovered views as variants
  - Purpose: Type-safe view switching
  - Relationships: Used by app struct to track active view
- **App Struct**: Generated struct containing state for all views
  - Attributes: `current_view: CurrentView`, one `{name}_state: AppState<Model>` field per view, optional `error_overlay`
  - Relationships: Contains AppState instances for each discovered View

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Developers can reduce a 500-line main.rs file to under 100 lines when migrating a 20-view application to use the `#[dampen_app]` macro (85% boilerplate reduction)
- **SC-002**: Compilation time overhead for view discovery remains under 200ms for applications with 20 views
- **SC-003**: Adding a new view requires only creating 2 files (`view_name.dampen` + `view_name.rs`) with zero additional wiring code
- **SC-004**: 100% of manual routing logic is eliminated - developers write zero lines of `match` statements for view switching
- **SC-005**: Hot-reload continues to work for all discovered views with reload latency under 500ms per file change
- **SC-006**: Compile errors for convention violations include file paths, problem descriptions, and actionable suggestions in 100% of cases
- **SC-007**: Generated code produces identical runtime performance to manually-written equivalents (zero overhead abstraction validated via benchmarks)
- **SC-008**: Developers can successfully migrate existing multi-view applications (e.g., `widget-showcase` example) without behavior changes
