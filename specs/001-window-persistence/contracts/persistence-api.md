# Persistence API Contracts

**Feature**: 001-window-persistence
**Date**: 2026-01-24

This document defines the public API surface for window state persistence in Dampen applications.

## Module: `dampen_dev::persistence`

### Core Functions

#### `load_or_default`

Loads window state from disk or returns defaults if loading fails.

```rust
/// Load persisted window state or return defaults.
///
/// This is the primary entry point for window persistence. It attempts to
/// load saved state from the platform-specific config directory. If loading
/// fails (file missing, corrupted, invalid), it returns a default WindowState
/// with the provided dimensions.
///
/// # Arguments
///
/// * `app_name` - Application identifier used to namespace the config file
/// * `default_width` - Default window width if no saved state exists
/// * `default_height` - Default window height if no saved state exists
///
/// # Returns
///
/// A `WindowState` struct ready to be applied to the Iced application builder.
///
/// # Example
///
/// ```rust
/// use dampen_dev::persistence::load_or_default;
///
/// let state = load_or_default("my-app", 800, 600);
/// iced::application(...)
///     .window_size(state.size())
///     .run()
/// ```
pub fn load_or_default(app_name: &str, default_width: u32, default_height: u32) -> WindowState;
```

**Contract**:
- MUST never panic
- MUST return a valid WindowState in all cases
- MUST log a warning (via `tracing`) if loading fails
- MUST use platform-appropriate config directory
- MUST validate loaded state before returning (fall back to defaults if invalid)

---

#### `save_window_state`

Saves window state to disk.

```rust
/// Save window state to the platform-specific config directory.
///
/// This function should be called when the application is closing (typically
/// in response to a `CloseRequested` window event). It creates the config
/// directory if it doesn't exist.
///
/// # Arguments
///
/// * `app_name` - Application identifier used to namespace the config file
/// * `state` - The WindowState to persist
///
/// # Returns
///
/// `Ok(())` on success, `Err(PersistenceError)` on failure.
///
/// # Example
///
/// ```rust
/// use dampen_dev::persistence::{save_window_state, WindowState};
///
/// fn update(&mut self, message: Message) -> Task<Message> {
///     match message {
///         Message::Window(iced::window::Event::CloseRequested) => {
///             let state = WindowState {
///                 width: self.window_width,
///                 height: self.window_height,
///                 x: self.window_x,
///                 y: self.window_y,
///                 maximized: self.is_maximized,
///             };
///             if let Err(e) = save_window_state("my-app", &state) {
///                 tracing::warn!("Failed to save window state: {e}");
///             }
///             iced::window::close(iced::window::Id::MAIN)
///         }
///         // ...
///     }
/// }
/// ```
pub fn save_window_state(app_name: &str, state: &WindowState) -> Result<(), PersistenceError>;
```

**Contract**:
- MUST never panic
- MUST create parent directories if they don't exist
- MUST write atomically (temp file + rename) to prevent corruption
- MUST return descriptive error on failure
- SHOULD complete in <10ms under normal conditions

---

#### `get_config_path`

Returns the config file path without loading.

```rust
/// Get the path where window state would be stored.
///
/// Useful for debugging or displaying to users. Does not create the directory.
///
/// # Arguments
///
/// * `app_name` - Application identifier
///
/// # Returns
///
/// `Some(PathBuf)` with the config file path, or `None` if the config directory
/// cannot be determined on this platform.
///
/// # Example
///
/// ```rust
/// use dampen_dev::persistence::get_config_path;
///
/// if let Some(path) = get_config_path("my-app") {
///     println!("Window state stored at: {}", path.display());
/// }
/// ```
pub fn get_config_path(app_name: &str) -> Option<PathBuf>;
```

**Contract**:
- MUST never panic
- MUST return platform-appropriate path
- MUST NOT create any files or directories

---

### WindowState Methods

#### `size`

```rust
impl WindowState {
    /// Convert to Iced Size for use with `.window_size()`.
    ///
    /// # Example
    ///
    /// ```rust
    /// let state = load_or_default("my-app", 800, 600);
    /// iced::application(...)
    ///     .window_size(state.size())
    /// ```
    pub fn size(&self) -> iced::Size;
}
```

**Contract**:
- MUST return `iced::Size::new(self.width as f32, self.height as f32)`

---

#### `position`

```rust
impl WindowState {
    /// Convert to Iced Point for use with `.position()`.
    ///
    /// Returns `None` if position was not saved (e.g., on Wayland).
    ///
    /// # Example
    ///
    /// ```rust
    /// let state = load_or_default("my-app", 800, 600);
    /// let mut app = iced::application(...);
    /// if let Some(pos) = state.position() {
    ///     app = app.position(pos);
    /// } else {
    ///     app = app.centered();
    /// }
    /// ```
    pub fn position(&self) -> Option<iced::Point>;
}
```

**Contract**:
- MUST return `Some(Point)` only if both `x` and `y` are `Some`
- MUST return `None` if either `x` or `y` is `None`

---

#### `with_defaults`

```rust
impl WindowState {
    /// Create a default WindowState with the given dimensions.
    ///
    /// Position is `None` (use system default), maximized is `false`.
    ///
    /// # Example
    ///
    /// ```rust
    /// let state = WindowState::with_defaults(800, 600);
    /// assert_eq!(state.width, 800);
    /// assert_eq!(state.maximized, false);
    /// ```
    pub fn with_defaults(width: u32, height: u32) -> Self;
}
```

**Contract**:
- MUST set `x` and `y` to `None`
- MUST set `maximized` to `false`

---

## Macro Attribute: `persistence`

When the `#[dampen_app]` macro is used with `persistence = true`, the macro generates
additional code to automatically save window state on close.

```rust
#[dampen_app(
    ui_dir = "src/ui",
    message_type = "Message",
    handler_variant = "Handler",
    // ... other attributes ...
    persistence = true,           // Enable window persistence
    app_name = "my-app",          // Required when persistence = true
)]
struct MyApp;
```

**Generated Code Contract**:
- MUST add a `Window(iced::window::Event)` subscription
- MUST intercept `CloseRequested` events
- MUST save current window state before returning close task
- MUST track window size via `Resized` events (or via window API)
- MUST NOT interfere with existing message handling

---

## Error Types

```rust
/// Error type for window persistence operations.
#[derive(Debug, thiserror::Error)]
pub enum PersistenceError {
    #[error("Failed to determine config directory for app '{app_name}'")]
    NoConfigDir { app_name: String },

    #[error("Failed to create config directory '{path}': {source}")]
    CreateDirFailed {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("Failed to read config file '{path}': {source}")]
    ReadFailed {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("Failed to write config file '{path}': {source}")]
    WriteFailed {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("Failed to parse config file '{path}': {source}")]
    ParseFailed {
        path: PathBuf,
        #[source]
        source: serde_json::Error,
    },

    #[error("Invalid window state: {reason}")]
    InvalidState { reason: String },
}
```

**Contract**:
- All error variants MUST include actionable information
- File-related errors MUST include the full path
- Parse errors MUST include the underlying serde error

---

## Re-exports

The `dampen_dev` crate MUST re-export:

```rust
// In dampen_dev/src/lib.rs
pub mod persistence;
pub use persistence::{
    load_or_default,
    save_window_state,
    get_config_path,
    WindowState,
    PersistenceError,
};
```
