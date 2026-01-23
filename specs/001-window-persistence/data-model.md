# Data Model: Window State Persistence

**Feature**: 001-window-persistence
**Date**: 2026-01-24

## Entities

### WindowState

The core entity representing persisted window configuration.

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `width` | `u32` | Yes | Window width in logical pixels |
| `height` | `u32` | Yes | Window height in logical pixels |
| `x` | `Option<i32>` | No | Window X position (None if platform doesn't support positioning) |
| `y` | `Option<i32>` | No | Window Y position (None if platform doesn't support positioning) |
| `maximized` | `bool` | Yes | Whether window was maximized |

**Validation Rules**:
- `width` must be >= 100 (minimum usable window width)
- `height` must be >= 100 (minimum usable window height)
- `width` must be <= 16384 (reasonable upper bound)
- `height` must be <= 16384 (reasonable upper bound)
- If `x`/`y` are provided, they should place at least part of the window on a visible monitor

**State Transitions**:
- `Default` → `Loaded`: When valid config file exists and is parsed
- `Default` → `Default`: When config file is missing, corrupted, or invalid
- `Loaded` → `Saved`: When application closes and state is written to disk
- `Saved` → `Loaded`: Next application start reads the saved state

### WindowStateConfig

Builder/configuration for loading window state with defaults.

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `app_name` | `String` | Yes | Application identifier for namespacing config files |
| `default_width` | `u32` | Yes | Default width if no saved state exists |
| `default_height` | `u32` | Yes | Default height if no saved state exists |
| `default_maximized` | `bool` | No | Default maximized state (defaults to `false`) |

## Relationships

```
WindowStateConfig (input)
        │
        ▼
   ┌─────────────────┐
   │  load_or_default │
   └─────────────────┘
        │
        ▼
   WindowState (output)
        │
        ├─── Applied to iced::application()
        │
        └─── Saved on CloseRequested event
```

## Storage Format

### File Location

Platform-specific paths using `directories` crate:

```
Linux:   ~/.config/{app_name}/window.json
Windows: C:\Users\{user}\AppData\Roaming\{app_name}\window.json
macOS:   ~/Library/Application Support/{app_name}/window.json
```

### JSON Schema

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "title": "WindowState",
  "type": "object",
  "required": ["width", "height", "maximized"],
  "properties": {
    "width": {
      "type": "integer",
      "minimum": 100,
      "maximum": 16384
    },
    "height": {
      "type": "integer",
      "minimum": 100,
      "maximum": 16384
    },
    "x": {
      "type": ["integer", "null"]
    },
    "y": {
      "type": ["integer", "null"]
    },
    "maximized": {
      "type": "boolean"
    }
  }
}
```

### Example JSON File

```json
{
  "width": 1200,
  "height": 800,
  "x": 100,
  "y": 50,
  "maximized": false
}
```

## Rust Type Definitions

```rust
/// Persisted window state for Dampen applications.
///
/// This struct is serialized to JSON and stored in the platform-specific
/// configuration directory. It captures the window's geometry and state
/// at the time the application was closed.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WindowState {
    /// Window width in logical pixels
    pub width: u32,

    /// Window height in logical pixels
    pub height: u32,

    /// Window X position (None if platform doesn't support absolute positioning)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<i32>,

    /// Window Y position (None if platform doesn't support absolute positioning)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y: Option<i32>,

    /// Whether the window was maximized
    pub maximized: bool,
}

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

## Conversion Helpers

```rust
impl WindowState {
    /// Convert to Iced Size for window_size()
    pub fn size(&self) -> iced::Size {
        iced::Size::new(self.width as f32, self.height as f32)
    }

    /// Convert to Iced Point for position() (returns None if position unavailable)
    pub fn position(&self) -> Option<iced::Point> {
        match (self.x, self.y) {
            (Some(x), Some(y)) => Some(iced::Point::new(x as f32, y as f32)),
            _ => None,
        }
    }

    /// Create with default values
    pub fn with_defaults(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            x: None,
            y: None,
            maximized: false,
        }
    }

    /// Validate the state against reasonable bounds
    pub fn validate(&self) -> Result<(), PersistenceError> {
        if self.width < 100 || self.height < 100 {
            return Err(PersistenceError::InvalidState {
                reason: format!(
                    "Window dimensions {}x{} are below minimum 100x100",
                    self.width, self.height
                ),
            });
        }
        if self.width > 16384 || self.height > 16384 {
            return Err(PersistenceError::InvalidState {
                reason: format!(
                    "Window dimensions {}x{} exceed maximum 16384x16384",
                    self.width, self.height
                ),
            });
        }
        Ok(())
    }
}
```
