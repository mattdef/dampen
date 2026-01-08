# Feature Specification: Production Mode with Static Code Generation

**Feature Branch**: `008-prod-codegen`  
**Created**: 2026-01-08  
**Status**: Draft  
**Input**: User description: "Implement production mode with static code generation, replacing runtime interpretation pattern with build-time code generation to eliminate XML parsing at runtime"

## User Scenarios & Testing *(mandatory)*

<!--
  IMPORTANT: User stories should be PRIORITIZED as user journeys ordered by importance.
  Each user story/journey must be INDEPENDENTLY TESTABLE - meaning if you implement just ONE of them,
  you should still have a viable MVP (Minimum Viable Product) that delivers value.
  
  Assign priorities (P1, P2, P3, etc.) to each story, where P1 is the most critical.
  Think of each story as a standalone slice of functionality that can be:
  - Developed independently
  - Tested independently
  - Deployed independently
  - Demonstrated to users independently
-->

### User Story 1 - Production Build Command (Priority: P1)

As a developer, I want to build my Gravity application for production with a single command, so that I can deploy optimized binaries without XML parsing overhead.

**Why this priority**: This is the core value proposition of the feature - enabling production deployments with performance benefits. Without this, developers cannot use Gravity in production environments.

**Independent Test**: Can be fully tested by running `gravity build --prod` on a sample project and verifying the output binary is generated and runs without XML parsing at runtime.

**Acceptance Scenarios**:

1. **Given** a Gravity project with `.gravity` UI files, **When** the developer runs `gravity build --prod`, **Then** the command generates a production-ready binary with statically compiled UI code.

2. **Given** a Gravity project, **When** the developer runs `gravity build --prod --release`, **Then** the command generates an optimized release binary.

3. **Given** a Gravity project with no `.gravity` files, **When** the developer runs `gravity build --prod`, **Then** the command completes successfully with a warning.

---

### User Story 2 - New Project Setup (Priority: P1)

As a developer starting a new Gravity project, I want the project to be automatically configured for production builds, so that I can deploy my application immediately after creation.

**Why this priority**: Ensures new projects have the correct build configuration from day one, reducing setup friction and ensuring consistent behavior across projects.

**Independent Test**: Can be fully tested by creating a new project with `gravity new` and verifying `build.rs` exists and `Cargo.toml` references it.

**Acceptance Scenarios**:

1. **Given** the Gravity CLI, **When** a developer runs `gravity new myproject`, **Then** the generated project includes a `build.rs` file configured for code generation.

2. **Given** a newly created Gravity project, **When** the developer examines `Cargo.toml`, **Then** it contains `build = "build.rs"` in the package section.

3. **Given** a newly created Gravity project, **When** the developer runs `cargo build`, **Then** the build process generates `ui_generated.rs` in the output directory.

---

### User Story 3 - Handler Registration (Priority: P2)

As a developer, I want my UI handlers to be automatically discovered and registered during the build process, so that I don't need to manually maintain handler registries.

**Why this priority**: Automating handler registration reduces developer overhead and eliminates a common source of bugs where handlers are not properly connected to UI events.

**Independent Test**: Can be fully tested by defining handlers with `#[ui_handler]` attribute and verifying they are called when corresponding UI events are triggered in the running application.

**Acceptance Scenarios**:

1. **Given** a handler function marked with `#[ui_handler]`, **When** the application is built in production mode, **Then** the handler is automatically registered and callable from UI events.

2. **Given** handlers defined in the application, **When** the build script runs, **Then** it extracts handler metadata and includes them in the generated code.

3. **Given** a handler with a specific signature, **When** the corresponding UI element triggers an event, **Then** the handler receives the expected parameters.

---

### User Story 4 - Development Mode Compatibility (Priority: P2)

As a developer, I want to continue using the development mode with `#[gravity_ui]` for rapid iteration, so that I can quickly test changes without full rebuilds.

**Why this priority**: Maintains developer productivity during development while enabling optimized production builds. Developers should not be forced to use production mode during iteration.

**Independent Test**: Can be fully tested by keeping a project on the development pattern, running `cargo run`, and verifying the application loads and responds correctly.

**Acceptance Scenarios**:

1. **Given** a Gravity project using `#[gravity_ui]`, **When** the developer runs `cargo run`, **Then** the application loads UI from XML at runtime as before.

2. **Given** a project configured for development mode, **When** the developer switches to production mode by adding `build.rs`, **Then** the same handlers work in both modes.

---

### User Story 5 - Example Migration (Priority: P3)

As a Gravity maintainer, I want all example projects to demonstrate production mode, so that new users see the recommended pattern from the start.

**Why this priority**: Ensures examples serve as proper templates for production use and validates the feature works across different application types.

**Independent Test**: Can be fully tested by building each migrated example with `cargo build --release` and verifying the application runs correctly.

**Acceptance Scenarios**:

1. **Given** the example projects, **When** they are migrated to production mode, **Then** each example builds successfully with `cargo build --release`.

2. **Given** migrated example projects, **When** a user runs the built application, **Then** all UI elements and interactions work as expected.

---

### Edge Cases

- What happens when a handler referenced in XML does not exist in the codebase?
- How does the system handle circular dependencies in handler signatures?
- What is the behavior when multiple handlers have the same name?
- How does the build process handle large numbers of `.gravity` files (100+)?
- What happens when the build script encounters invalid XML?

## Clarifications

### Session 2026-01-08

- Q: Handler reference error messages content → A: Include handler name, XML file path, line number, and suggested handlers
- Q: Invalid XML error handling → A: Build fails with error using `gravity check` on all .gravity files
- Q: Build observability level → A: Verbose progress including parsed files, handlers found, generated code location
- Q: Duplicate handler names handling → A: Build fails with error showing all duplicates and their source locations
- Q: Circular handler dependencies handling → A: Build fails with error - Detect and report circular dependencies at compile time

## Requirements *(mandatory)*

<!--
  ACTION REQUIRED: The content in this section represents placeholders.
  Fill them out with the right functional requirements.
-->

### Functional Requirements

- **FR-001**: The system MUST provide a `gravity build --prod` command that generates production-ready binaries.
- **FR-002**: The system MUST eliminate XML parsing at runtime in production builds by generating static Rust code.
- **FR-003**: The system MUST generate a `build.rs` file automatically for new projects created with `gravity new`.
- **FR-004**: The system MUST extract handler metadata from `#[ui_handler]` macro attributes for build-time registration.
- **FR-005**: The system MUST maintain backward compatibility with development mode (`#[gravity_ui]` pattern).
- **FR-006**: The system MUST support both debug and release production builds via `--release` flag.
- **FR-007**: The system MUST provide clear error messages when handler references in XML do not match defined handlers. Error messages MUST include handler name, XML file path, line number, and suggested handlers.
- **FR-008**: The system MUST rebuild generated code when `.gravity` files change.
- **FR-009**: The system MUST support all existing widget types in production mode.
- **FR-010**: The system MUST generate code that compiles without warnings in production builds.
- **FR-011**: The system MUST validate all `.gravity` files using `gravity check` during the build process, failing with specific error location and nature if XML is invalid.
- **FR-012**: The system MUST output verbose build progress including number of parsed files, handlers found, and generated code location.
- **FR-013**: The system MUST fail the build when duplicate handler names are detected, showing all duplicates and their source locations.
- **FR-014**: The system MUST detect and report circular handler dependencies at compile time, failing the build with error details.

### Key Entities *(include if feature involves data)*

- **HandlerInfo**: Metadata structure containing handler name, signature type, parameter types, and return type.
- **HandlerSignatureType**: Enumeration of handler signature patterns (Simple, WithValue, WithCommand).
- **build.rs Template**: Build script template that scans `.gravity` files and generates static UI code.
- **ui_generated.rs**: Auto-generated Rust file containing statically compiled UI definitions and handler registrations.
- **HandlerRegistry**: Runtime structure that maps handler names to their implementations.

## Success Criteria *(mandatory)*

<!--
  ACTION REQUIRED: Define measurable success criteria.
  These must be technology-agnostic and measurable.
-->

### Measurable Outcomes

- **SC-001**: Developers can complete a production build in under 5 minutes for a typical application (under 100 widgets).
- **SC-002**: Production binaries have zero XML parsing overhead at runtime, eliminating the LazyLock initialization cost.
- **SC-003**: All example projects compile successfully in production mode with zero compilation errors.
- **SC-004**: Handler registration is fully automated, requiring zero manual registry maintenance from developers.
- **SC-005**: The migration path from development to production mode takes less than 30 minutes for an existing project.
- **SC-006**: Zero regression in functionality - all existing tests pass in both development and production modes.

### Performance Targets

- **PT-001**: Production build time is within 200% of the development build time.
- **PT-002**: Generated code size is within 150% of the equivalent runtime-parsed code size.
- **PT-003**: Application startup time in production mode is at least 50% faster than development mode.

### Quality Gates

- **QG-001**: All workspace tests pass (`cargo test --workspace`).
- **QG-002**: No clippy warnings in production builds (`cargo clippy --workspace -- -D warnings`).
- **QG-003**: All code is formatted according to project standards (`cargo fmt --all -- --check`).

## Assumptions

- The Gravity framework continues to use Rust as the implementation language.
- The Iced framework remains the reference backend for UI rendering.
- Existing `.gravity` XML schema remains compatible with both development and production modes.
- Handler signature patterns remain limited to the three defined types (Simple, WithValue, WithCommand).
- New projects are created using `gravity new` CLI command.
- Examples serve as the primary validation test suite for the feature.

## Dependencies

- Existing `#[ui_handler]` attribute macro implementation.
- Existing code generation infrastructure in `gravity-core/src/codegen/`.
- Existing XML parser (`roxmltree`) for build-time parsing.
- Existing Iced backend integration in `gravity-iced`.
- Cargo build system and `build.rs` mechanism.

## Out of Scope

- Changes to the XML schema or widget capabilities.
- Support for non-Iced backends in production mode.
- Hot reload functionality in production mode.
- Custom handler signature types beyond the three defined patterns.
- Integration with third-party build systems other than Cargo.

## Risks

| Risk | Probability | Impact | Mitigation |
|------|-------------|--------|------------|
| Handler metadata extraction complexity | High | High | Implement progressively, start with simple version |
| Backward compatibility issues | Medium | High | Test all existing examples, migrate comprehensively |
| Build time increase | Medium | Low | Measure before/after, optimize if necessary |
| Incomplete handler validation | Medium | Medium | Add validation errors in build script |

## Success Metrics

- 100% of existing examples successfully migrated to production mode.
- Zero new test failures introduced by the feature.
- Developer feedback confirms improved production deployment experience.
- Production build command used by all new projects within 3 months of release.
