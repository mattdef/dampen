# Research: Window State Persistence

**Feature**: 001-window-persistence
**Date**: 2026-01-24

## Research Questions Resolved

### 1. Where should WindowState struct live?

**Decision**: `dampen-dev` crate in a new `persistence` module

**Rationale**:
- Follows the existing pattern established by `theme_loader.rs`
- Maintains Constitution Principle IV (Backend Abstraction) - `dampen-core` stays Iced-free
- Persistence is a development/runtime concern, not core IR functionality
- The `dampen-dev` crate already has `directories` as a dependency for theme loading

**Alternatives considered**:
- `dampen-core`: Rejected - would violate backend abstraction principle
- `dampen-iced`: Rejected - persistence is not widget-related, wrong abstraction level
- New crate `dampen-persistence`: Rejected - over-engineering for a single feature

### 2. How to detect available monitors for validation?

**Decision**: Use Iced's window/monitor APIs at application startup

**Rationale**:
- Iced 0.14+ provides `iced::window::fetch_id()` and monitor information
- However, monitor enumeration happens after application starts
- For initial positioning, we validate against reasonable bounds (e.g., screen dimensions)
- If position is off-screen, center on primary monitor

**Alternatives considered**:
- Platform-specific APIs (winit, x11rb, windows-rs): Rejected - adds complexity and dependencies
- Skip validation entirely: Rejected - creates poor UX when monitors change

### 3. How to handle Wayland position limitations?

**Decision**: Save position on all platforms, gracefully ignore on restore if platform doesn't support it

**Rationale**:
- Wayland (via XDG shell) does not allow clients to set absolute window position
- Iced abstracts this - `.position()` is silently ignored on Wayland
- Users get size/maximized restoration, position is handled by compositor
- No special code paths needed - the Iced abstraction handles the platform difference

**Alternatives considered**:
- Detect Wayland and skip position saving: Rejected - unnecessary complexity
- Warn users on Wayland: Rejected - Iced handles this transparently

### 4. When to save window state?

**Decision**: Save only on `CloseRequested` window event, not on every resize/move

**Rationale**:
- Avoids disk thrashing (resize events can fire at 60+ fps during drag)
- Matches user expectation: "save when I close the app"
- Aligns with FR-006 from spec: "save state only on application close event"

**Alternatives considered**:
- Debounced saves (1 sec after last change): Rejected - adds complexity, CloseRequested is sufficient
- Periodic saves (every 30 sec): Rejected - unnecessary I/O, risk of stale data on crash

### 5. Config file format and location?

**Decision**: JSON file at `{config_dir}/{app_name}/window.json`

**Rationale**:
- `directories` crate provides `ProjectDirs::from("", "", app_name)` for cross-platform paths
  - Linux: `~/.config/{app_name}/window.json`
  - Windows: `C:\Users\{user}\AppData\Roaming\{app_name}\window.json`
  - macOS: `~/Library/Application Support/{app_name}/window.json`
- JSON is human-readable, debuggable, and uses existing `serde_json` dependency
- Single file per app keeps things simple

**Alternatives considered**:
- TOML: Rejected - would add new dependency, JSON already available
- Binary format: Rejected - not human-debuggable, no significant benefit
- SQLite: Rejected - massive overkill for storing 5 values

### 6. How to integrate with #[dampen_app] macro?

**Decision**: Add optional `persistence` attribute to macro, expand to save-on-close logic

**Rationale**:
- `persistence = true` enables automatic state saving when window closes
- Macro generates code to intercept `Window(CloseRequested)` events
- Requires user to have a `Window(iced::window::Event)` variant in their Message enum
- Macro can generate the variant if needed (similar to existing `HotReload` handling)

**Alternatives considered**:
- No macro integration, manual-only: Rejected - violates SC-002 (single-line enablement)
- Attribute in dampen XML: Rejected - persistence is runtime config, not UI structure

### 7. How to handle first-run (no saved state)?

**Decision**: `load_or_default(app_name, default_width, default_height)` helper function

**Rationale**:
- Developer specifies defaults when calling the load function
- If config file doesn't exist or is invalid, returns WindowState with defaults
- Simple API: one function call handles both loading and fallback

**Alternatives considered**:
- Separate `load()` and `default()` functions: Rejected - more boilerplate for users
- Read defaults from dampen XML: Rejected - adds complexity, XML is for UI structure

### 8. Error handling strategy?

**Decision**: Log warnings, never panic, return defaults on failure

**Rationale**:
- Aligns with FR-010/FR-011: "not block startup/shutdown when loading/saving fails"
- Uses `tracing` or `log` crate for warnings (already in workspace dependencies)
- File permission errors, JSON parse errors, missing directories all handled gracefully

**Alternatives considered**:
- Return Result to caller: Rejected - shifts burden to user, most will ignore anyway
- Silent failures: Rejected - users should know if persistence isn't working

## Dependencies Confirmed

All required dependencies already exist in workspace:

| Dependency | Version | Location | Purpose |
|------------|---------|----------|---------|
| `serde` | 1.0 | workspace | Serialization derives |
| `serde_json` | 1.0 | workspace | JSON file format |
| `directories` | 5.0 | workspace | Cross-platform config paths |
| `tokio` | 1.0 | workspace (with `fs` feature) | Async file I/O |
| `tracing` | 0.1 | workspace | Warning logging |

No new dependencies required.

## Iced API Usage

### Window Size and Position

```rust
// Setting initial window configuration
iced::application(...)
    .window_size(iced::Size::new(state.width as f32, state.height as f32))
    .position(iced::Point::new(state.x.unwrap_or(0) as f32, state.y.unwrap_or(0) as f32))
    .centered()  // Used when position is None or invalid
    // ...
```

### Window Events (for save-on-close)

```rust
// In Message enum
enum Message {
    // ... other variants
    Window(iced::window::Event),
}

// In subscription
fn subscription(&self) -> Subscription<Message> {
    iced::window::events().map(|(_, event)| Message::Window(event))
}

// In update
fn update(&mut self, message: Message) -> Task<Message> {
    match message {
        Message::Window(iced::window::Event::CloseRequested) => {
            // Save window state here
            save_window_state("my-app", &current_state);
            iced::window::close(iced::window::Id::MAIN)
        }
        // ...
    }
}
```

### Getting Current Window Geometry

```rust
// Iced 0.14 provides window information via tasks
// Use iced::window::fetch_size() and similar APIs
// For simplicity, track size/position in application state via Resized/Moved events
```

## Platform-Specific Behavior

| Platform | Position | Size | Maximized | Notes |
|----------|----------|------|-----------|-------|
| Linux X11 | ✅ Supported | ✅ Supported | ✅ Supported | Full functionality |
| Linux Wayland | ⚠️ Ignored | ✅ Supported | ✅ Supported | Position set by compositor |
| Windows | ✅ Supported | ✅ Supported | ✅ Supported | Full functionality |
| macOS | ✅ Supported | ✅ Supported | ✅ Supported | Full functionality |

## Open Questions (None)

All technical questions have been resolved. Ready for Phase 1 design.
