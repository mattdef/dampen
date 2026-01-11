# Feature Specification: XML Schema Version Parsing and Validation

**Feature Branch**: `001-schema-version-validation`  
**Created**: 2026-01-11  
**Status**: Draft  
**Input**: User description: "Implement comprehensive version parsing and validation for Dampen XML files to ensure schema compatibility, prevent future breaking changes, and provide clear error messages when incompatible versions are used."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Parse Version from XML File (Priority: P1)

As a Dampen developer, I want my `.dampen` XML files to declare which schema version they use, so that the framework can ensure compatibility and prevent unexpected behavior.

**Why this priority**: This is the foundation of version support. Without parsing the version attribute, no validation can occur. This enables all subsequent version-related features.

**Independent Test**: Can be fully tested by parsing XML files with various `version` attribute values and verifying the resulting document contains the correct version information.

**Acceptance Scenarios**:

1. **Given** a `.dampen` file with `<dampen version="1.0">`, **When** the parser processes the file, **Then** the document reports version 1.0
2. **Given** a `.dampen` file without a `version` attribute, **When** the parser processes the file, **Then** the document defaults to version 1.0 (backward compatibility)
3. **Given** a `.dampen` file with version attribute format "major.minor", **When** the parser processes it, **Then** both major and minor components are correctly extracted

---

### User Story 2 - Reject Unsupported Future Versions (Priority: P1)

As a Dampen developer, I want the parser to reject XML files that declare a version newer than my framework supports, so that I don't encounter silent failures or undefined behavior from unsupported features.

**Why this priority**: Critical for preventing runtime errors and confusion. Users must know immediately when their files are incompatible with their framework version.

**Independent Test**: Can be fully tested by parsing XML files with future version declarations (e.g., "2.0") and verifying appropriate error messages are returned.

**Acceptance Scenarios**:

1. **Given** a `.dampen` file with `version="2.0"` and a framework that supports max v1.0, **When** the parser processes the file, **Then** an error is returned with message indicating version 2.0 is not supported
2. **Given** a `.dampen` file with `version="1.1"` and a framework that supports max v1.0, **When** the parser processes the file, **Then** an error is returned suggesting to upgrade the framework
3. **Given** an unsupported version error, **When** displayed to the user, **Then** the error includes actionable suggestions (upgrade framework or downgrade file)

---

### User Story 3 - Reject Invalid Version Formats (Priority: P2)

As a Dampen developer, I want clear error messages when my version attribute is malformed, so that I can quickly fix typos or formatting mistakes.

**Why this priority**: Provides a good developer experience with clear, actionable error messages rather than cryptic failures.

**Independent Test**: Can be fully tested by parsing XML files with various invalid version formats and verifying appropriate error messages.

**Acceptance Scenarios**:

1. **Given** a `.dampen` file with `version="1"` (missing minor), **When** parsed, **Then** an error indicates expected format is "major.minor"
2. **Given** a `.dampen` file with `version="v1.0"` (invalid prefix), **When** parsed, **Then** an error indicates expected format is "major.minor"
3. **Given** a `.dampen` file with `version="1.0.5"` (patch version), **When** parsed, **Then** an error indicates only major.minor format is supported
4. **Given** a `.dampen` file with `version=""` (empty string), **When** parsed, **Then** an error indicates version cannot be empty

---

### User Story 4 - Consistent Version Declaration Across Project (Priority: P2)

As a Dampen framework maintainer, I want all example files and templates to explicitly declare their schema version, so that users have consistent, best-practice examples to follow.

**Why this priority**: Ensures documentation by example and prevents confusion when users compare their files to examples.

**Independent Test**: Can be verified by checking all `.dampen` files in the project have explicit version declarations.

**Acceptance Scenarios**:

1. **Given** the `dampen new` command, **When** a new project is created, **Then** the generated `.dampen` files include `version="1.0"`
2. **Given** all example projects in the repository, **When** inspected, **Then** every `.dampen` file declares `version="1.0"` explicitly
3. **Given** test fixture files, **When** inspected, **Then** they also declare explicit versions for consistency

---

### User Story 5 - Widget Version Infrastructure (Priority: P3)

As a Dampen framework maintainer, I want infrastructure in place to associate widgets with minimum required versions, so that future versions can enforce widget compatibility without major refactoring.

**Why this priority**: Lower priority as this is forward-looking infrastructure. No enforcement occurs now, but the foundation is needed for v1.1 and beyond.

**Independent Test**: Can be verified by checking that all widgets report a minimum version and the validation function exists (even if currently a no-op).

**Acceptance Scenarios**:

1. **Given** any widget type, **When** queried for minimum version, **Then** it returns version 1.0 (all current widgets)
2. **Given** the validation infrastructure, **When** reviewed, **Then** it contains clear documentation for when to activate enforcement
3. **Given** the codebase, **When** searching for widget version methods, **Then** the pattern is consistent and extensible

---

### Edge Cases

- What happens when `version` attribute contains whitespace (e.g., `version=" 1.0 "`)? Trim whitespace and parse normally
- What happens when `version` contains non-numeric characters (e.g., `version="1.0-beta"`)? Reject with format error
- What happens when major or minor version exceeds reasonable bounds (e.g., `version="999.999"`)? Parse successfully if format is valid; validation will reject if unsupported
- What happens when version uses negative numbers (e.g., `version="-1.0"`)? Reject with format error
- What happens when version uses leading zeros (e.g., `version="01.00"`)? Accept and parse as 1.0

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: Parser MUST read the `version` attribute from the `<dampen>` root element
- **FR-002**: Parser MUST parse version strings in "major.minor" format into separate major and minor components
- **FR-003**: Parser MUST default to version 1.0 when the `version` attribute is absent (backward compatibility)
- **FR-004**: Parser MUST reject version strings that don't match the "major.minor" format with a clear error
- **FR-005**: Parser MUST reject version declarations newer than the maximum supported version (currently 1.0) with an actionable error
- **FR-006**: Error messages MUST include the invalid/unsupported version value and the expected format or maximum supported version
- **FR-007**: Error messages MUST include actionable suggestions (e.g., "Upgrade dampen-core" or "Use version 1.0")
- **FR-008**: All example `.dampen` files MUST explicitly declare `version="1.0"`
- **FR-009**: The `dampen new` command template MUST generate files with `version="1.0"`
- **FR-010**: Each widget type MUST be able to report its minimum required schema version
- **FR-011**: Parser MUST support versions where major or minor is 0 (e.g., "0.9" should be parseable, though may be rejected as unsupported)
- **FR-012**: Documentation MUST clearly describe version format, default behavior, and error scenarios

### Key Entities

- **SchemaVersion**: Represents a schema version with major and minor components. Used to compare compatibility between declared file versions and framework-supported versions.
- **DampenDocument**: The root parsed structure that contains the schema version along with the widget tree, themes, and style classes.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: 100% of `.dampen` files in the repository explicitly declare a version attribute
- **SC-002**: Parser correctly accepts version="1.0" files and rejects version="1.1" or higher with error
- **SC-003**: Invalid version formats produce error messages that include the expected format within 1 second
- **SC-004**: Files without version attribute continue to parse successfully (0 breaking changes to existing projects)
- **SC-005**: New projects created with `dampen new` include version="1.0" in all generated `.dampen` files
- **SC-006**: Error messages for unsupported versions include both the declared version and the maximum supported version
- **SC-007**: All existing tests continue to pass after implementation (backward compatibility verified)
- **SC-008**: Documentation accurately describes the version format, default behavior, and all error scenarios

## Assumptions

- The maximum supported schema version is 1.0 for this implementation
- Version format is strictly "major.minor" with no patch component
- All current widgets (including experimental Canvas) are considered v1.0 widgets
- Namespace validation (xmlns attribute) is deferred to a future implementation
- Deprecation warnings for missing version attributes are planned for v1.1, not this implementation
- Widget-level version enforcement will be activated in a future version when v1.1 widgets are introduced
