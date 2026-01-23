# Feature Specification: Window State Persistence

**Feature Branch**: `001-window-persistence`
**Created**: 2026-01-23
**Status**: Draft
**Input**: User description: "Persistence des fenêtres - Opt-in window state persistence via macro or configuration, supporting size, position, and maximized state with cross-platform compatibility and performance-conscious I/O"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Restore Window Position on App Restart (Priority: P1)

As an application user, I want my application window to open at the same size and position where I last closed it, so that I don't have to manually resize and reposition it every time I start the app.

**Why this priority**: This is the core value proposition of window persistence - eliminating repetitive manual window arrangement. It directly addresses user frustration and improves daily workflow efficiency.

**Independent Test**: Can be fully tested by resizing/moving an app window, closing it, reopening it, and verifying the window appears at the saved dimensions and position.

**Acceptance Scenarios**:

1. **Given** a user has resized and repositioned the application window, **When** the user closes the application and reopens it, **Then** the window opens at the previously saved size and position
2. **Given** the user is running the application for the first time, **When** the application opens, **Then** the window uses the default size and position defined by the developer
3. **Given** window state data exists from a previous session, **When** the application loads the state, **Then** the application starts without noticeable delay compared to fresh start

---

### User Story 2 - Restore Maximized State (Priority: P1)

As an application user, I want the application to remember if I had maximized the window, so that I can quickly return to my preferred full-screen working state.

**Why this priority**: Maximized state is a critical part of window arrangement that users expect to persist alongside size and position. It's part of the same core experience.

**Independent Test**: Can be tested by maximizing the window, closing the app, reopening, and verifying the window opens maximized.

**Acceptance Scenarios**:

1. **Given** the user has maximized the application window, **When** the application is closed and reopened, **Then** the window opens in maximized state
2. **Given** the user has restored a maximized window to normal size, **When** the application is closed and reopened, **Then** the window opens at the normal (non-maximized) saved size and position

---

### User Story 3 - Developer Enables Persistence (Priority: P1)

As a Dampen application developer, I want to enable window persistence for my application with minimal code changes, so that I can provide this feature to my users without complex implementation.

**Why this priority**: The developer experience determines adoption. If enabling persistence is difficult, developers won't use it. This is foundational to the feature's success.

**Independent Test**: Can be tested by adding persistence configuration to an existing Dampen app and verifying window state is saved and restored.

**Acceptance Scenarios**:

1. **Given** a developer wants to enable persistence, **When** they add the persistence configuration to their application, **Then** window state is automatically saved and restored without additional code
2. **Given** a developer has not explicitly enabled persistence, **When** the application runs, **Then** window state is not persisted (opt-in behavior)
3. **Given** a developer enables persistence, **When** they specify a custom app name/identifier, **Then** the state is saved using that identifier (allowing multiple apps to have separate states)

---

### User Story 4 - Graceful Handling of Disconnected Monitors (Priority: P2)

As a user with multiple monitors, I want the application to handle situations where my saved position is on a monitor that is no longer connected, so that I can still see and use the application.

**Why this priority**: Multi-monitor setups are common, and monitors being disconnected (laptops undocked, external monitors turned off) is a frequent scenario. Poor handling makes the app unusable.

**Independent Test**: Can be tested by saving window position on a secondary monitor, disconnecting that monitor, and reopening the app to verify it appears on an available monitor.

**Acceptance Scenarios**:

1. **Given** the saved window position is on a monitor that is no longer available, **When** the application opens, **Then** the window is repositioned to be visible on the primary monitor
2. **Given** the saved window position partially overlaps with an available monitor, **When** the application opens, **Then** the window is adjusted to be fully visible
3. **Given** all monitors from the saved configuration are still available, **When** the application opens, **Then** the window appears at the exact saved position

---

### User Story 5 - Cross-Platform Position Handling (Priority: P2)

As a user on Linux with Wayland, I want window persistence to work as well as possible despite platform limitations, so that I get the best experience available on my system.

**Why this priority**: Wayland has known limitations around absolute window positioning. Users should understand what works and what doesn't on their platform.

**Independent Test**: Can be tested on Wayland by verifying size and maximized state persist while position gracefully degrades.

**Acceptance Scenarios**:

1. **Given** the platform supports absolute window positioning (X11, Windows, macOS), **When** the application opens, **Then** both size and position are restored
2. **Given** the platform does not support absolute positioning (Wayland), **When** the application opens, **Then** size and maximized state are restored, and position is handled by the window manager
3. **Given** position data cannot be retrieved on the current platform, **When** the application closes, **Then** only size and maximized state are saved without error

---

### Edge Cases

- What happens when the config file is corrupted or contains invalid JSON?
  - The application uses default window settings and logs a warning (non-blocking)
- What happens when the user has no write permission to the config directory?
  - State is not saved; the application runs normally with a logged warning
- What happens when the saved window size exceeds all available monitor dimensions?
  - The window is resized to fit within the primary monitor
- What happens when multiple instances of the same application are running?
  - The last instance to close overwrites the saved state (documented behavior)

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST persist window state only when explicitly enabled by the developer (opt-in)
- **FR-002**: System MUST save window width and height
- **FR-003**: System MUST save window position (x, y coordinates) when the platform supports it
- **FR-004**: System MUST save maximized state (boolean)
- **FR-005**: System MUST store state data in the platform-appropriate configuration directory (XDG_CONFIG_HOME on Linux, AppData/Roaming on Windows, Application Support on macOS)
- **FR-006**: System MUST save state only on application close event (not on every resize/move)
- **FR-007**: System MUST validate loaded window position against available monitors before restoring
- **FR-008**: System MUST fall back to default window settings when saved state is invalid or unavailable
- **FR-009**: System MUST allow developers to specify default window dimensions used when no saved state exists
- **FR-010**: System MUST not block application startup when loading persisted state fails
- **FR-011**: System MUST not block application shutdown when saving state fails

### Key Entities

- **WindowState**: Represents the persisted window configuration containing width, height, x position (optional), y position (optional), and maximized state. This is the core data structure serialized to disk.
- **AppIdentifier**: A unique string identifying the application, used to namespace the saved configuration file. Allows multiple Dampen apps to coexist without conflicts.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Users can close and reopen an application with window state restored in under 100ms additional startup time
- **SC-002**: Window persistence feature can be enabled with a single configuration change (one line or attribute)
- **SC-003**: 100% of valid window states are correctly restored when monitors are unchanged
- **SC-004**: Applications gracefully handle all edge cases (corrupted files, missing permissions, disconnected monitors) without crashing
- **SC-005**: Window persistence works correctly on all three major desktop platforms (Linux, Windows, macOS) with documented limitations

## Assumptions

- Applications have sufficient filesystem permissions to read/write to standard config directories in typical deployments
- The `directories` crate provides reliable cross-platform config directory detection
- Iced framework provides necessary window event subscriptions (close, resize) to capture state changes
- The JSON format is sufficient for state storage (no need for binary or encrypted formats)
- Single-instance behavior for state saving is acceptable (last-close-wins for multiple instances)
