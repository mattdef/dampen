use crate::persistence::error::PersistenceError;
use serde::{Deserialize, Serialize};

/// Persisted window state for Dampen applications.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WindowState {
    /// Window width in logical pixels
    pub width: u32,

    /// Window height in logical pixels
    pub height: u32,

    /// Window X position (None if platform doesn't support absolute positioning)
    ///
    /// On Linux with Wayland, absolute window positioning is generally not supported
    /// by compositors. In this case, `x` and `y` will be `None`, and the window
    /// position will be determined by the compositor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<i32>,

    /// Window Y position (None if platform doesn't support absolute positioning)
    ///
    /// On Linux with Wayland, absolute window positioning is generally not supported
    /// by compositors. In this case, `x` and `y` will be `None`, and the window
    /// position will be determined by the compositor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y: Option<i32>,

    /// Whether the window was maximized.
    ///
    /// If true, the application should initialize the window in a maximized state,
    /// regardless of the saved width/height/position.
    pub maximized: bool,
}

impl WindowState {
    /// Create a default WindowState with the given dimensions.
    pub fn with_defaults(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            x: None,
            y: None,
            maximized: false,
        }
    }

    /// Convert to Iced Size for use with `.window_size()`.
    pub fn size(&self) -> iced::Size {
        iced::Size::new(self.width as f32, self.height as f32)
    }

    /// Convert to Iced Point for use with `.position()`.
    pub fn position(&self) -> Option<iced::Point> {
        match (self.x, self.y) {
            (Some(x), Some(y)) => Some(iced::Point::new(x as f32, y as f32)),
            _ => None,
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
