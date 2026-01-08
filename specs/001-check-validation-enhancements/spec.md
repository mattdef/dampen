# Feature Specification: Check Validation Enhancements

**Feature Branch**: `001-check-validation-enhancements`
**Created**: 2026-01-08
**Status**: Draft
**Input**: User description: "Improve XML validation in gravity check command with rigorous validation including unknown attributes, handler registry validation, binding validation, cross-widget validation (radio groups), theme validation, and Levenshtein distance suggestions"

## Clarifications

### Session 2026-01-08

- Q: How are handler registry and model info provided to gravity check? → A: Require explicit CLI flags: `--handlers FILE` and `--model FILE`
- Q: How should gravity check behave when handler registry or model files are missing or malformed? → A: Fail validation with a clear error message indicating which file is missing/malformed
- Q: How should gravity check handle performance for very large files? → A: Validate all files but warn if processing time exceeds threshold
- Q: How should gravity check handle custom widgets with user-defined attributes? → A: Allow custom widget attribute definitions via a config file
- Q: Should gravity check collect all errors or fail fast on first error? → A: Collect all errors and report at end

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Unknown Attribute Detection (Priority: P1)

As a developer writing Gravity UI XML files, I want to be warned when I use an incorrect attribute name on a widget, so that I can catch typos and mistakes early before runtime.

**Why this priority**: This is the most common and impactful validation gap. Currently, unknown attributes are silently ignored, leading to bugs that are hard to diagnose. Every developer makes typos in attribute names, and this validation will catch these errors immediately.

**Independent Test**: Can be fully tested by running `gravity check` on XML files with misspelled attributes (e.g., `on_clik` instead of `on_click`) and verifying that errors are reported with helpful suggestions.

**Acceptance Scenarios**:

1. **Given** a Gravity XML file with a button widget, **When** the attribute `on_clik` is used, **Then** the check command reports an error: "Unknown attribute 'on_clik' for button" with suggestion "Did you mean 'on_click'?"

2. **Given** a Gravity XML file with valid attributes only, **When** running `gravity check`, **Then** no attribute-related errors are reported.

3. **Given** a Gravity XML file with an unknown attribute, **When** running `gravity check` in strict mode, **Then** the command exits with code 1.

---

### User Story 2 - Handler Registry Validation (Priority: P1)

As a developer using event handlers in my UI, I want to be warned when I reference a handler that doesn't exist, so that I can fix broken event connections before running the application.

**Why this priority**: Event handlers are critical for interactive UIs. If a handler is misspelled or doesn't exist, the UI becomes unresponsive. This validation catches these issues at build time.

**Independent Test**: Can be fully tested by creating XML files with handler references that don't match registered handlers and verifying that errors are reported with available handler names.

**Acceptance Scenarios**:

1. **Given** a handler named `increment` is registered in the application, **When** XML references `on_click="incremnt"`, **Then** check reports: "Unknown handler 'incremnt'" with suggestion "Did you mean 'increment'?"

2. **Given** no handlers are registered, **When** XML references any handler, **Then** check reports all available handlers (or indicates none are registered).

3. **Given** a valid handler reference, **When** running `gravity check`, **Then** no handler-related errors are reported.

---

### User Story 3 - Binding Validation Against Model (Priority: P1)

As a developer using data bindings, I want to be warned when I reference a field that doesn't exist in my model, so that I can fix binding errors that would otherwise cause blank or broken UI elements.

**Why this priority**: Data bindings are essential for dynamic UIs. Invalid bindings result in silent failures at runtime, showing blank values or broken displays. Early detection prevents confusing user experiences.

**Independent Test**: Can be fully tested by creating XML files with bindings to non-existent model fields and verifying that errors report available fields.

**Acceptance Scenarios**:

1. **Given** a model with field `user.name`, **When** XML uses binding `{user.nme}`, **Then** check reports: "Invalid binding field 'user.nme'" with available fields.

2. **Given** a valid binding path, **When** running `gravity check`, **Then** no binding-related errors are reported.

3. **Given** a nested binding path, **When** an intermediate field is not marked as nested in the model, **Then** check reports the invalid path.

---

### User Story 4 - Cross-Widget Radio Group Validation (Priority: P2)

As a developer using radio button groups, I want validation to ensure all radio buttons in a group have unique values and consistent handlers, so that my radio selection works correctly at runtime.

**Why this priority**: Radio groups require specific constraints (unique values, consistent handlers). Without validation, duplicate values cause undefined behavior and inconsistent handlers break event handling. This prevents runtime bugs in selection logic.

**Independent Test**: Can be fully tested by creating radio groups with duplicate values or inconsistent handlers and verifying that errors are reported.

**Acceptance Scenarios**:

1. **Given** a radio group with two buttons having value "option1", **When** running `gravity check`, **Then** check reports: "Duplicate radio value 'option1' in group 'size'".

2. **Given** a radio group where buttons have different `on_select` handlers, **When** running `gravity check`, **Then** check reports: "Radio group has inconsistent on_select handlers".

3. **Given** a valid radio group with unique values and consistent handlers, **When** running `gravity check`, **Then** no radio group errors are reported.

---

### User Story 5 - Theme Property Validation (Priority: P2)

As a developer using custom themes, I want validation to ensure theme properties are valid, so that I can catch configuration errors before the UI renders with broken styles.

**Why this priority**: Invalid theme properties cause silent style failures or runtime errors. Developers need immediate feedback on theme configuration issues.

**Independent Test**: Can be fully tested by creating theme definitions with invalid property names and verifying that errors are reported.

**Acceptance Scenarios**:

1. **Given** a theme with property `font_siz` (misspelled), **When** running `gravity check`, **Then** check reports: "Invalid theme property 'font_siz'" with valid properties list.

2. **Given** a theme with circular dependency (Theme A extends B, B extends A), **When** running `gravity check`, **Then** check reports: "Theme has circular dependency: A -> B -> A".

3. **Given** a valid theme, **When** running `gravity check`, **Then** no theme errors are reported.

---

### User Story 6 - Strict Mode for Warnings as Errors (Priority: P3)

As a developer who wants to enforce high code quality, I want the option to treat all warnings as errors, so that I can ensure my XML files meet all quality standards before deployment.

**Why this priority**: Strict mode is a quality enforcement tool for production builds and CI/CD pipelines. It allows teams to choose their validation strictness level.

**Independent Test**: Can be fully tested by running `gravity check` with and without `--strict` flag on files with warnings.

**Acceptance Scenarios**:

1. **Given** an XML file with a warning (e.g., unknown attribute), **When** running `gravity check` without `--strict`, **Then** command exits with code 0 and displays warning.

2. **Given** an XML file with a warning (e.g., unknown attribute), **When** running `gravity check --strict`, **Then** command exits with code 1 and reports the warning as error.

3. **Given** an XML file with no warnings, **When** running `gravity check --strict`, **Then** command exits with code 0.

---

### User Story 7 - Required Attribute Validation (Priority: P2)

As a developer, I want to be warned when required attributes are missing from widgets, so that I can provide complete widget configurations that render correctly.

**Why this priority**: Some widgets require specific attributes to function (e.g., Text needs `value`, Image needs `src`). Missing required attributes cause runtime errors or broken UI.

**Independent Test**: Can be fully tested by creating XML files with widgets missing required attributes and verifying that errors are reported.

**Acceptance Scenarios**:

1. **Given** a Text widget without `value` attribute, **When** running `gravity check`, **Then** check reports: "Missing required attribute 'value' for Text".

2. **Given** an Image widget without `src` attribute, **When** running `gravity check`, **Then** check reports: "Missing required attribute 'src' for Image".

3. **Given** a widget with all required attributes present, **When** running `gravity check`, **Then** no missing attribute errors are reported.

---

### Edge Cases

- Handler registry or model files are missing or malformed: Validation fails with a clear error message indicating which file is problematic
- How does the system handle very long attribute names (over 100 characters)?
- What happens with empty XML files or empty widget definitions?
- How does validation handle deeply nested widget hierarchies (100+ levels)?
- What happens when multiple errors are found: All errors are collected and reported at end
- How are bindings with complex expressions (method calls, binary operations) validated?
- What happens when a widget has both unknown AND missing required attributes?
- Custom widgets with user-defined attributes: Supported via config file definitions

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST detect and report unknown attributes on all widgets with helpful suggestions based on Levenshtein distance (threshold: distance <= 3)
- **FR-002**: System MUST validate that all event handlers referenced in XML exist in the handler registry provided via `--handlers FILE` CLI flag
- **FR-003**: System MUST validate that all binding paths reference existing fields in the model definition provided via `--model FILE` CLI flag
- **FR-004**: System MUST validate radio button groups for duplicate values within the same group
- **FR-005**: System MUST validate radio button groups for consistent on_select handlers
- **FR-006**: System MUST validate theme properties against a defined list of valid properties
- **FR-007**: System MUST detect circular dependencies in theme inheritance chains
- **FR-008**: System MUST report missing required attributes for widgets
- **FR-009**: System MUST provide a `--strict` flag that causes warnings to exit with code 1
- **FR-010**: System MUST include file path, line number, and column in all error messages
- **FR-011**: System MUST continue validation across all widgets and collect all errors before reporting at end
- **FR-012**: System SHOULD suggest similar valid attributes when an unknown attribute is detected
- **FR-013**: System SHOULD suggest similar handler names when an unknown handler is detected
- **FR-014**: System SHOULD list available fields when a binding path is invalid
- **FR-015**: System MUST fail validation with a clear error message when handler registry or model files are missing or malformed
- **FR-016**: System SHOULD warn users if validation processing time exceeds 1 second threshold for large files
- **FR-017**: System SHOULD support custom widget attribute definitions via a config file to allow user-defined attributes

### Key Entities

- **HandlerRegistry**: A collection of registered handler definitions used to validate handler references in XML. Contains handler names and optional metadata (parameter types, return types).
- **ModelInfo**: A description of the data model fields used to validate binding expressions. Contains field names, types, and nesting information.
- **WidgetAttributeSchema**: A definition of valid attributes (required, optional, events, styles, layout) for each widget type.
- **RadioGroup**: A logical grouping of radio buttons identified by the `group` attribute or anonymous grouping by proximity.
- **ValidationError**: Structured error information including error type, location (file, line, column), and context for display.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Users can identify and fix all attribute typos in their XML files before runtime, reducing attribute-related bugs by 100%
- **SC-002**: Users receive actionable error messages with line/column locations for all validation errors, enabling direct navigation to issues
- **SC-003**: Users see helpful suggestions (Levenshtein-based) for 90% of unknown attribute and handler errors
- **SC-004**: Radio button groups with validation errors are identified before runtime, preventing selection bugs
- **SC-005**: Theme configuration errors are caught at validation time rather than causing runtime style failures
- **SC-006**: `gravity check` completes validation of a typical UI file (100-500 widgets) in under 1 second, and warns users if processing exceeds this threshold for larger files
- **SC-007**: Strict mode enables CI/CD pipelines to fail builds on validation warnings, ensuring code quality gates
