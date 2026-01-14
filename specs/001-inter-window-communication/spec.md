# Feature Specification: Inter-Window Communication

**Feature Branch**: `001-inter-window-communication`  
**Created**: 2026-01-14  
**Status**: Draft  
**Input**: User description: "Inter-window communication for Dampen multi-view applications. Enable shared state and messaging between views."

---

## Overview

Currently, Dampen multi-view applications have **completely isolated views**. Each view maintains its own independent state (Model) and event handlers (HandlerRegistry), with no mechanism to share data or communicate between views. This limitation forces developers to either duplicate state across views or implement workarounds that break the declarative paradigm.

This feature introduces **inter-view communication** through:
1. **Shared State** - A global state container accessible from all views
2. **Shared Bindings** - XML binding syntax to display shared data (`{shared.field}`)
3. **Shared Handlers** - Event handlers that can read and modify shared state

---

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Share User Preferences Across Views (Priority: P1)

As an application developer, I want to define a shared state that persists user preferences (theme, language, settings) so that when a user changes a preference in one view, all other views immediately reflect that change without requiring manual synchronization.

**Why this priority**: This is the most common use case for multi-view applications. Without shared preferences, developers must duplicate state and manually synchronize it, leading to bugs and inconsistent UX.

**Independent Test**: Can be fully tested by creating a two-view application where changing a preference in View A immediately updates the display in View B, delivering immediate value for preference-driven applications.

**Acceptance Scenarios**:

1. **Given** a multi-view application with shared state containing a "theme" preference, **When** a user changes the theme in the Settings view, **Then** the Main view immediately displays widgets styled according to the new theme without navigation or refresh.

2. **Given** a shared state with a "language" preference, **When** the language is updated from any view, **Then** all views display text in the selected language on their next render.

3. **Given** a view using `{shared.theme}` binding, **When** the shared state is not yet configured (opt-in feature), **Then** the binding evaluates to empty/default without causing errors.

---

### User Story 2 - Display Shared Data in XML Bindings (Priority: P1)

As an application developer, I want to use a `{shared.field}` syntax in my XML UI definitions so that I can declaratively display shared state values in any view without writing custom binding code.

**Why this priority**: The declarative XML binding is core to Dampen's value proposition. Without shared bindings, developers must fall back to imperative code, breaking the framework's declarative philosophy.

**Independent Test**: Can be fully tested by creating an XML file with `{shared.user.name}` binding and verifying it displays the shared state value correctly.

**Acceptance Scenarios**:

1. **Given** an XML widget with `value="{shared.user.name}"`, **When** the shared state contains `user.name = "Alice"`, **Then** the widget displays "Alice".

2. **Given** an XML widget with `visible="{shared.is_logged_in}"`, **When** the shared state contains `is_logged_in = true`, **Then** the widget is visible.

3. **Given** an XML widget with `{shared.nonexistent}` binding, **When** the field does not exist in shared state, **Then** the binding evaluates to empty string without errors.

4. **Given** nested shared state fields `{shared.user.profile.avatar}`, **When** the path exists in shared state, **Then** the nested value is correctly resolved.

---

### User Story 3 - Modify Shared State from Event Handlers (Priority: P1)

As an application developer, I want my event handlers to access and modify the shared state so that user actions in one view can update global application data.

**Why this priority**: Without the ability to modify shared state from handlers, the shared state would be read-only and useless for interactive applications.

**Independent Test**: Can be fully tested by creating a handler that increments a shared counter and verifying the counter value updates globally.

**Acceptance Scenarios**:

1. **Given** a button with `on_click="update_theme"` handler, **When** the handler modifies `shared.theme = "dark"`, **Then** the shared state is updated and all views using `{shared.theme}` reflect the change.

2. **Given** a text input with `on_change="update_username"` handler, **When** the handler sets `shared.user.name` to the input value, **Then** all views display the new username.

3. **Given** a handler that only reads shared state without modifying it, **When** the handler executes, **Then** no errors occur and the shared state remains unchanged.

---

### User Story 4 - Preserve Shared State During Hot-Reload (Priority: P2)

As an application developer using development mode, I want the shared state to be preserved when I edit XML files and hot-reload the UI so that I don't lose my application state during development.

**Why this priority**: Essential for developer experience but not required for production applications. Hot-reload is already a core feature; this ensures shared state integrates seamlessly.

**Independent Test**: Can be fully tested by modifying an XML file while the app is running with shared state and verifying the state persists after reload.

**Acceptance Scenarios**:

1. **Given** a running application with `shared.count = 42`, **When** an XML file is modified and hot-reloaded, **Then** `shared.count` remains 42 after the reload.

2. **Given** a running application with shared state, **When** the shared state definition file is modified (adding a new field), **Then** existing shared state values are preserved and the new field is initialized to default.

---

### User Story 5 - Opt-In Shared State Configuration (Priority: P2)

As an application developer, I want shared state to be an opt-in feature so that existing applications continue to work without modification and I only pay the complexity cost when I need it.

**Why this priority**: Backward compatibility is critical. The feature must not break existing applications or force migration.

**Independent Test**: Can be fully tested by running existing example applications (hello-world, counter, todo-app) without any modifications and verifying they work identically.

**Acceptance Scenarios**:

1. **Given** an existing application without shared state configuration, **When** upgrading to the new version, **Then** the application compiles and runs identically without changes.

2. **Given** an application using the new `shared_model` configuration, **When** the application starts, **Then** the shared context is initialized and available to all views.

3. **Given** an application with `{shared.x}` binding but no shared state configured, **When** the binding is evaluated, **Then** it fails gracefully with a clear error message during development.

---

### User Story 6 - Parity Between Interpreted and Codegen Modes (Priority: P2)

As an application developer, I want shared state and bindings to work identically in both interpreted (development) and codegen (production) modes so that my application behaves consistently across environments.

**Why this priority**: Dampen's dual-mode architecture is a key differentiator. Shared state must maintain parity to preserve this value.

**Independent Test**: Can be fully tested by running the same application in both modes and comparing output for identical inputs.

**Acceptance Scenarios**:

1. **Given** an application with `{shared.user.name}` binding, **When** run in interpreted mode, **Then** the binding evaluates correctly.

2. **Given** the same application with the same binding, **When** run in codegen mode, **Then** the binding produces identical output.

3. **Given** a shared handler in interpreted mode, **When** the same handler runs in codegen mode, **Then** the shared state modifications are identical.

---

### Edge Cases

- What happens when a view tries to write to shared state while another view is reading? (Concurrent access handled by thread-safe locking)
- How does the system handle circular dependencies where shared state update triggers another shared state update? (Updates are atomic; no recursive triggers within single update cycle)
- What happens when shared state serialization fails during hot-reload persistence? (Graceful degradation with warning; state reset to defaults)
- How does the system behave when shared state exceeds reasonable memory limits? (Standard memory pressure handling; no special limits)

---

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST provide a mechanism to define application-wide shared state accessible from all views
- **FR-002**: System MUST support `{shared.field}` binding syntax in XML to display shared state values
- **FR-003**: System MUST support nested field access in shared bindings (e.g., `{shared.user.profile.name}`)
- **FR-004**: System MUST allow event handlers to read shared state values
- **FR-005**: System MUST allow event handlers to modify shared state values
- **FR-006**: System MUST automatically propagate shared state changes to all views using those bindings
- **FR-007**: System MUST preserve shared state during hot-reload in development mode
- **FR-008**: System MUST maintain 100% backward compatibility with applications not using shared state
- **FR-009**: System MUST produce identical behavior in interpreted and codegen modes
- **FR-010**: System MUST provide clear error messages when shared bindings reference non-existent fields
- **FR-011**: System MUST handle concurrent read/write access to shared state safely
- **FR-012**: System MUST provide an opt-in configuration to enable shared state

### Key Entities

- **SharedState**: User-defined structure containing application-wide data. Must implement bindable trait. Contains fields accessible via `{shared.field}` syntax.

- **SharedContext**: Internal container that wraps SharedState. Provides thread-safe read/write access. Cloneable reference shared across all views.

- **SharedHandler**: Event handler variant that receives both local model and shared context. Can read and modify shared state.

- **SharedBinding**: Expression in XML that references shared state (prefix `shared.`). Evaluated at render time to produce display values.

---

## Assumptions

The following assumptions have been made based on analysis and industry standards:

1. **Thread Safety**: Shared state access will use read-write locks for safe concurrent access
2. **Binding Evaluation**: Shared bindings are evaluated at render time, not continuously observed
3. **Default Values**: Missing shared fields return empty/default values rather than errors in production
4. **State Initialization**: Shared state is initialized once at application startup with default values
5. **Handler Signature**: Existing handlers continue to work; shared handlers are a new variant

---

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Developers can share state between views with less than 10 lines of configuration code
- **SC-002**: Shared state updates propagate to all views in under 16ms (one frame at 60fps)
- **SC-003**: Existing applications (hello-world, counter, todo-app, settings) pass all tests without modification
- **SC-004**: 100% of shared binding evaluations in interpreted mode match codegen mode output
- **SC-005**: Hot-reload preserves 100% of shared state values across XML file changes
- **SC-006**: Applications using shared state add less than 5% memory overhead compared to baseline
- **SC-007**: Developer documentation enables first shared state usage within 5 minutes of reading
- **SC-008**: Zero breaking changes for applications not opting into shared state

---

## Dependencies

- Existing `AppState<M>` structure (will be extended)
- Existing `HandlerRegistry` and `HandlerEntry` enum (will be extended)
- Existing `UiBindable` trait (shared state must implement this)
- Existing `DampenWidgetBuilder` (will be extended for shared binding resolution)
- Existing `#[dampen_app]` macro (will be extended with new attributes)

---

## Out of Scope

The following are explicitly **not** part of this feature:

1. **Multiple OS Windows**: This feature addresses inter-view communication within a single window, not true multi-window OS applications
2. **Persistent Storage**: Shared state is in-memory only; persistence to disk/database is a separate feature
3. **Remote State Sync**: No network synchronization of shared state between application instances
4. **Undo/Redo**: No built-in history tracking for shared state changes
5. **Pub/Sub Messaging**: While the architecture supports it, explicit event bus messaging is a future enhancement
