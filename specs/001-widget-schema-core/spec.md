# Feature Specification: Widget Schema Migration to Core

**Feature Branch**: `001-widget-schema-core`  
**Created**: 2026-01-23  
**Status**: Draft  
**Input**: User description: "Migrer le schema de validation des widgets de dampen-cli vers dampen-core pour creer une source unique de verite"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Framework Developer Adds New Widget Attribute (Priority: P1)

As a framework developer, when I add a new attribute to a widget in the parser/builder (dampen-core or dampen-iced), I want the `dampen check` command to automatically recognize this attribute as valid, so that I don't have to manually update the CLI validation logic separately.

**Why this priority**: This is the core problem being solved. Currently, adding an attribute requires updates in multiple places, leading to validation errors for valid attributes.

**Independent Test**: Can be fully tested by adding a mock attribute to a widget's schema in dampen-core and verifying that dampen-cli recognizes it without any CLI code changes.

**Acceptance Scenarios**:

1. **Given** a widget schema defined in dampen-core with attribute "new_attr", **When** I run `dampen check` on a file using "new_attr", **Then** no "Unknown attribute" error is reported.
2. **Given** a widget schema in dampen-core without attribute "invalid_attr", **When** I run `dampen check` on a file using "invalid_attr", **Then** an "Unknown attribute" error is reported with suggestions.
3. **Given** I add a new attribute to a widget's schema in dampen-core, **When** I rebuild the CLI without modifying CLI code, **Then** the new attribute is automatically recognized as valid.

---

### User Story 2 - CLI Tool Queries Core for Valid Attributes (Priority: P1)

As the dampen-cli tool, I need to query dampen-core to get the list of valid attributes for each widget type, so that validation logic stays synchronized with the actual parser capabilities.

**Why this priority**: This is the mechanism that enables User Story 1. Without this, the CLI cannot access the central schema.

**Independent Test**: Can be tested by calling the schema API from dampen-core and verifying it returns the expected attributes for each widget type.

**Acceptance Scenarios**:

1. **Given** I call `get_widget_schema(WidgetKind::Button)` from dampen-core, **When** the schema is returned, **Then** it contains "label", "enabled" in optional attributes and "on_press", "on_click" in events.
2. **Given** I call `get_widget_schema(WidgetKind::Container)`, **When** the schema is returned, **Then** it includes common layout attributes like "align_x", "align_y", "padding", "width", "height".
3. **Given** I call `get_widget_schema(WidgetKind::TextInput)`, **When** the schema is returned, **Then** it includes "size" in optional attributes (recently added fix).

---

### User Story 3 - Third-Party Tool Accesses Schema Information (Priority: P2)

As a developer building IDE plugins or other tools that work with Dampen files, I want to access the widget schema from dampen-core, so that my tools can provide accurate autocompletion and validation.

**Why this priority**: This extends the value beyond just the CLI, enabling ecosystem growth. However, it's secondary to fixing the immediate CLI synchronization issue.

**Independent Test**: Can be tested by importing dampen-core as a library dependency and accessing the public schema API.

**Acceptance Scenarios**:

1. **Given** I import `dampen_core::schema`, **When** I call `WidgetSchema::for_widget(&WidgetKind::Text)`, **Then** I receive a schema with "value" as required and "size", "weight", "color" as optional.
2. **Given** the schema module is public in dampen-core, **When** I compile a project depending on dampen-core, **Then** the schema types and functions are accessible without feature flags.

---

### User Story 4 - Backward Compatibility for Existing CLI Validation (Priority: P2)

As a user of `dampen check`, I want the validation behavior to remain identical after the migration, so that my existing workflows and CI pipelines continue to work without changes.

**Why this priority**: Ensuring no regression is critical for user trust, but it's a constraint rather than a new capability.

**Independent Test**: Can be tested by running `dampen check` on the todo-app example before and after migration and comparing results.

**Acceptance Scenarios**:

1. **Given** the todo-app example passes `dampen check` before migration, **When** I run `dampen check` after migration, **Then** it still passes with identical output.
2. **Given** a file with an invalid attribute "align_xx", **When** I run `dampen check` before and after migration, **Then** both produce the same error message with the same suggestion ("align_x").

---

### Edge Cases

- What happens when a Custom widget kind is queried for its schema? The schema should return empty/permissive attributes since custom widgets are user-defined.
- How does the system handle deprecated attributes? The schema should continue to support deprecated attribute warnings separately from the validation schema.
- What happens if dampen-core is updated but dampen-cli is not recompiled? Since they share the same workspace, this is handled by Cargo's dependency management, but the schema API should be stable.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: dampen-core MUST expose a public `schema` module that defines valid attributes for each widget type.
- **FR-002**: dampen-core MUST provide a `WidgetSchema` struct containing: required attributes, optional attributes, event attributes, style attributes, and layout attributes.
- **FR-003**: dampen-core MUST provide a function `get_widget_schema(kind: &WidgetKind) -> WidgetSchema` that returns the schema for any widget type.
- **FR-004**: dampen-core MUST define common attribute sets as public constants: `COMMON_STYLE_ATTRIBUTES`, `COMMON_LAYOUT_ATTRIBUTES`, `COMMON_EVENTS`.
- **FR-005**: dampen-cli MUST remove its local attribute definitions and use dampen-core's schema module instead.
- **FR-006**: dampen-cli MUST retain its suggestion logic (Levenshtein distance) for user-friendly error messages.
- **FR-007**: dampen-cli MUST retain its error reporting format (CheckError types) unchanged.
- **FR-008**: The schema MUST include all recently fixed attributes: `align_x`, `align_y`, `align_self`, `style` (common), `size` (for TextInput, Checkbox).
- **FR-009**: The `WidgetSchema` MUST provide a method to get all valid attributes combined (similar to current `all_valid()` method).
- **FR-010**: Custom widgets (WidgetKind::Custom) MUST return a permissive schema that allows any attributes.

### Key Entities

- **WidgetSchema**: Represents the validation contract for a widget type. Contains sets of attribute names organized by category (required, optional, events, style, layout).
- **WidgetKind**: Existing enum in dampen-core representing all supported widget types. Will gain a method or associated function to retrieve its schema.
- **Common Attribute Sets**: Static collections of attribute names shared across multiple widget types (e.g., all widgets support "width", "height", "padding").

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: After migration, adding a new attribute to dampen-core requires changes in exactly 1 location (dampen-core schema), not 2 locations (core + CLI).
- **SC-002**: The `dampen check` command produces identical validation results on the todo-app example before and after migration (0 errors in both cases).
- **SC-003**: All existing unit tests in dampen-cli pass without modification to test logic (only import paths may change).
- **SC-004**: The schema module is documented and accessible as public API in dampen-core's generated documentation.
- **SC-005**: Build time impact is negligible (less than 5% increase in workspace compilation time).
- **SC-006**: The migration introduces no new runtime allocations for schema lookups (using static slices instead of HashSet where possible).

## Assumptions

- The schema definitions currently in `dampen-cli/src/commands/check/attributes.rs` are accurate and complete for the current state of the framework.
- The `lazy_static` crate dependency can be removed from dampen-cli if no longer needed after migration.
- The WidgetKind enum in dampen-core already contains all widget variants and is the source of truth for widget types.
- Cargo workspace dependency resolution ensures dampen-cli always uses the same version of dampen-core it was compiled with.

## Out of Scope

- Runtime schema modification or extension (schemas are compile-time constants).
- Validation of attribute values (only attribute name validity is in scope).
- Migration of the suggestion/Levenshtein logic to dampen-core (stays in CLI as UX concern).
- Changes to the CheckError types or error message format in dampen-cli.
- Support for loading custom widget schemas from external configuration files.
