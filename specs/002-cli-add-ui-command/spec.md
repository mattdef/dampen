# Feature Specification: CLI Add UI Command

**Feature Branch**: `002-cli-add-ui-command`  
**Created**: 2026-01-13  
**Status**: ✅ COMPLETE (Phase 8)  
**Completed**: 2026-01-13  
**Input**: User description: "Add CLI command to generate new UI windows with templates based on hello-world example"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Generate Basic UI Window (Priority: P1)

A developer working on a Dampen application wants to add a new UI window without manually creating files. They run a single command and get a working window with all necessary boilerplate code.

**Why this priority**: This is the core feature that delivers immediate value - developers can scaffold new windows quickly and consistently without manual file creation or copy-pasting.

**Independent Test**: Can be fully tested by running `dampen add --ui settings` in a Dampen project, verifying that `src/ui/settings.rs` and `src/ui/settings.dampen` are created with valid templates, and the files compile successfully.

**Acceptance Scenarios**:

1. **Given** a Dampen project with a `src/ui/` directory, **When** developer runs `dampen add --ui settings`, **Then** the system creates `src/ui/settings.rs` and `src/ui/settings.dampen` with template code based on hello-world example
2. **Given** a Dampen project, **When** developer runs `dampen add --ui user_profile`, **Then** the system creates files with underscore-compatible naming (`user_profile.rs`, `user_profile.dampen`)
3. **Given** newly generated UI window files, **When** developer runs `cargo build`, **Then** the project compiles successfully without errors
4. **Given** a Dampen project, **When** developer runs `dampen add --ui MyWindow`, **Then** the system creates files with lowercase naming (`my_window.rs`, `my_window.dampen`) to follow Rust conventions

---

### User Story 2 - Custom Directory Paths (Priority: P2)

A developer organizing their UI into subdirectories wants to create a new window in a specific location. They use the `--path` option to specify the exact directory.

**Why this priority**: This enables better project organization for larger applications with multiple feature areas, but basic functionality works without it.

**Independent Test**: Can be tested independently by running `dampen add --ui new_order --path "src/ui/orders/"` and verifying files are created in the correct subdirectory with proper structure.

**Acceptance Scenarios**:

1. **Given** a Dampen project, **When** developer runs `dampen add --ui new_order --path "src/ui/orders/"`, **Then** the system creates `src/ui/orders/new_order.rs` and `src/ui/orders/new_order.dampen`
2. **Given** a path that doesn't exist, **When** developer runs `dampen add --ui settings --path "src/ui/admin/"`, **Then** the system creates the missing directories and generates the files
3. **Given** a custom path, **When** developer provides relative path `ui/orders/`, **Then** the system normalizes it to `src/ui/orders/` following Dampen conventions
4. **Given** an absolute path outside the project, **When** developer runs the command, **Then** the system shows an error message requiring the path to be within the project

---

### User Story 3 - Prevent Duplicate Files (Priority: P2)

A developer accidentally runs the add command for a window that already exists. The system prevents accidental overwrites and suggests alternatives.

**Why this priority**: Prevents data loss and improves user experience, but doesn't block core functionality.

**Independent Test**: Can be tested by creating a window, attempting to create it again, and verifying the error message and suggestion to use a different name.

**Acceptance Scenarios**:

1. **Given** a Dampen project with existing `src/ui/settings.rs`, **When** developer runs `dampen add --ui settings`, **Then** the system shows an error "Window 'settings' already exists at src/ui/settings.rs" and exits without creating files
2. **Given** an existing window file, **When** developer receives the duplicate error, **Then** the error message suggests using a different name or removing the existing file first
3. **Given** a partial conflict (only `.rs` exists, not `.dampen`), **When** developer runs the command, **Then** the system treats this as a conflict and prevents creation of either file

---

### User Story 4 - Integration with Project Structure (Priority: P3)

A developer creates a new window and wants to use it immediately in their application. The generated files follow Dampen conventions and integrate seamlessly.

**Why this priority**: Improves developer experience by providing ready-to-use templates, but manual integration is straightforward.

**Independent Test**: Can be tested by generating a window, adding it to the UI module exports, and verifying it works with the application's view and update functions.

**Acceptance Scenarios**:

1. **Given** newly generated window files, **When** developer inspects the `.rs` file, **Then** it contains a complete `Model` struct with `#[derive(UiModel)]`, `create_app_state()`, and `create_handler_registry()` functions
2. **Given** newly generated window files, **When** developer inspects the `.dampen` file, **Then** it contains a simple working XML UI with column layout, text, button, and data binding
3. **Given** a generated window, **When** developer adds `pub mod window_name;` to `src/ui/mod.rs`, **Then** the module is accessible and compiles without errors
4. **Given** generated template code, **When** developer runs `dampen check`, **Then** the XML validates successfully

---

### User Story 5 - Project Validation (Priority: P1)

A developer accidentally runs the `dampen add` command outside of a Dampen project directory. The system detects this and provides clear feedback to prevent confusion.

**Why this priority**: Critical for user experience - prevents developers from creating files in wrong locations and provides immediate feedback about the error.

**Independent Test**: Can be fully tested by running `dampen add --ui settings` in a non-Dampen directory (e.g., empty directory or non-Rust project) and verifying the appropriate error message is displayed.

**Acceptance Scenarios**:

1. **Given** a directory without a `Cargo.toml` file, **When** developer runs `dampen add --ui settings`, **Then** the system shows error "Not a Dampen project: Cargo.toml not found" and exits with non-zero code
2. **Given** a Rust project without Dampen dependencies, **When** developer runs `dampen add --ui settings`, **Then** the system shows error "Not a Dampen project: dampen-core dependency not found in Cargo.toml" and exits with non-zero code
3. **Given** a valid Dampen project, **When** developer runs `dampen add --ui settings`, **Then** the system proceeds with file generation without validation errors
4. **Given** a non-Dampen directory error, **When** developer receives the error message, **Then** the message includes helpful suggestion "Run 'dampen new <project_name>' to create a new Dampen project"

---

### Edge Cases

- What happens when command is run outside a Dampen project?
  - System validates presence of `Cargo.toml` and Dampen dependencies before proceeding
  - Clear error message with suggestion to create a new project using `dampen new`
  
- What happens when window name contains special characters or spaces?
  - System should reject invalid names and show clear error message with valid naming examples
  - Valid: alphanumeric, underscores, hyphens; Invalid: spaces, special chars
  
- How does system handle very long window names?
  - System should accept names up to 255 characters (filesystem limit) but warn if over 50 characters
  
- What happens when `src/ui/` directory doesn't exist in a valid Dampen project?
  - System creates the directory structure automatically
  
- How does system handle insufficient filesystem permissions?
  - System shows clear error message indicating permission denied with the specific path
  
- What happens when disk is full during file creation?
  - System shows error and ensures no partial files are left behind (atomic creation or cleanup)
  
- How does system handle concurrent invocations?
  - Each invocation is independent; race conditions between two commands creating the same window would be caught by file existence check

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST provide a `dampen add` command with `--ui <window_name>` argument that generates UI window files
- **FR-002**: System MUST validate that command is run within a Dampen project by checking for `Cargo.toml` and dampen-core dependency
- **FR-003**: System MUST create two files per window: `<window_name>.rs` (Rust module) and `<window_name>.dampen` (XML UI definition)
- **FR-004**: System MUST generate the Rust file with: Model struct with `#[derive(UiModel)]`, `#[dampen_ui]` macro, `create_app_state()` function, and `create_handler_registry()` function
- **FR-005**: System MUST generate the XML file with a basic working UI containing: column layout, heading text, interactive button, and data binding example
- **FR-006**: System MUST validate window names to ensure they are valid Rust identifiers (alphanumeric and underscores, starting with letter or underscore)
- **FR-007**: System MUST convert window names to snake_case for file names (e.g., "MyWindow" → "my_window.rs")
- **FR-008**: System MUST default to placing files in `src/ui/` directory if no `--path` is specified
- **FR-009**: System MUST support `--path <directory>` option to specify custom output directory
- **FR-010**: System MUST create missing directory paths when custom `--path` is provided
- **FR-011**: System MUST prevent overwriting existing window files and show clear error messages
- **FR-012**: System MUST generate template code that compiles successfully with the project's Dampen dependencies
- **FR-013**: System MUST generate XML that validates with `dampen check` command
- **FR-014**: System MUST normalize paths to ensure they are within the project directory (relative paths starting from project root)
- **FR-015**: System MUST replace `{{WINDOW_NAME}}` placeholders in templates with the actual window name
- **FR-016**: System MUST show success message with created file paths after successful generation
- **FR-017**: System MUST exit with code 0 on success and non-zero on failure
- **FR-018**: System MUST display helpful error message when run outside a Dampen project, suggesting `dampen new` command

### Key Entities

- **Window Template**: A set of two files (`.rs` and `.dampen`) that follow the hello-world example pattern with placeholders for customization
- **Window Name**: User-provided identifier that becomes the base name for generated files (validated as Rust identifier)
- **Target Directory**: The filesystem location where window files will be created (default `src/ui/` or custom via `--path`)
- **Generated Files**: The concrete `.rs` and `.dampen` files created from templates with placeholders replaced

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Developers can generate a new working UI window in under 5 seconds using a single command
- **SC-002**: Generated window files compile successfully without any modifications in 100% of cases
- **SC-003**: Generated window XML validates with `dampen check` in 100% of cases
- **SC-004**: Command handles invalid inputs gracefully with clear error messages in 100% of error cases
- **SC-005**: Developers can create windows in custom directories without errors in 100% of attempts
- **SC-006**: System prevents accidental file overwrites in 100% of duplicate name scenarios
- **SC-007**: System detects non-Dampen project directories in 100% of cases before attempting file creation
- **SC-008**: Reduce time to create new UI windows from ~5 minutes (manual) to <10 seconds (automated)
