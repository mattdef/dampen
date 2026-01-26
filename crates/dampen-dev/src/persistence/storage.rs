use crate::persistence::{PersistenceError, WindowState};
use directories::ProjectDirs;
use std::fs;
use std::path::PathBuf;

/// Builder for window settings with persistence support.
///
/// This builder allows configuring default window settings that will be used
/// when no persisted state exists (e.g., on first launch).
///
/// # Example
///
/// ```ignore
/// iced::application(...)
///     .window(DampenApp::window_settings()
///         .default_size(1024, 768)
///         .default_maximized(true)
///         .min_size(400, 300)
///         .resizable(true)
///         .build())
///     .run()
/// ```
pub struct WindowSettingsBuilder {
    app_name: String,
    default_width: u32,
    default_height: u32,
    default_maximized: bool,
    min_size: Option<(u32, u32)>,
    max_size: Option<(u32, u32)>,
    resizable: bool,
}

impl WindowSettingsBuilder {
    /// Create a new builder for the given application.
    pub fn new(app_name: &str) -> Self {
        Self {
            app_name: app_name.to_string(),
            default_width: 800,
            default_height: 600,
            default_maximized: false,
            min_size: None,
            max_size: None,
            resizable: true,
        }
    }

    /// Set the default window size for first launch.
    ///
    /// This size is used when no persisted state exists.
    pub fn default_size(mut self, width: u32, height: u32) -> Self {
        self.default_width = width;
        self.default_height = height;
        self
    }

    /// Set whether the window should be maximized on first launch.
    ///
    /// This is used when no persisted state exists.
    pub fn default_maximized(mut self, maximized: bool) -> Self {
        self.default_maximized = maximized;
        self
    }

    /// Set the minimum window size.
    pub fn min_size(mut self, width: u32, height: u32) -> Self {
        self.min_size = Some((width, height));
        self
    }

    /// Set the maximum window size.
    pub fn max_size(mut self, width: u32, height: u32) -> Self {
        self.max_size = Some((width, height));
        self
    }

    /// Set whether the window is resizable.
    pub fn resizable(mut self, resizable: bool) -> Self {
        self.resizable = resizable;
        self
    }

    /// Build the final `iced::window::Settings`.
    ///
    /// This loads any persisted state and merges it with the defaults.
    pub fn build(self) -> iced::window::Settings {
        let mut state = load_or_default(&self.app_name, self.default_width, self.default_height);

        // Apply default maximized only if no persisted state was loaded
        // (we detect this by checking if the size matches our defaults exactly
        // and there's no position - indicating fresh defaults were used)
        let is_fresh = state.width == self.default_width
            && state.height == self.default_height
            && state.x.is_none()
            && state.y.is_none()
            && !state.maximized;

        if is_fresh && self.default_maximized {
            state.maximized = true;
        }

        iced::window::Settings {
            size: state.size(),
            position: state
                .position()
                .map(iced::window::Position::Specific)
                .unwrap_or(iced::window::Position::Centered),
            min_size: self
                .min_size
                .map(|(w, h)| iced::Size::new(w as f32, h as f32)),
            max_size: self
                .max_size
                .map(|(w, h)| iced::Size::new(w as f32, h as f32)),
            resizable: self.resizable,
            ..Default::default()
        }
    }
}

impl From<WindowSettingsBuilder> for iced::window::Settings {
    fn from(builder: WindowSettingsBuilder) -> Self {
        builder.build()
    }
}

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
pub fn get_config_path(app_name: &str) -> Option<PathBuf> {
    ProjectDirs::from("", "", app_name).map(|dirs| dirs.config_dir().join("window.json"))
}

fn load_window_state(app_name: &str) -> Result<WindowState, PersistenceError> {
    let path = get_config_path(app_name).ok_or_else(|| PersistenceError::NoConfigDir {
        app_name: app_name.to_string(),
    })?;

    if !path.exists() {
        return Err(PersistenceError::ReadFailed {
            path: path.clone(),
            source: std::io::Error::new(std::io::ErrorKind::NotFound, "File not found"),
        });
    }

    let content = fs::read_to_string(&path).map_err(|e| PersistenceError::ReadFailed {
        path: path.clone(),
        source: e,
    })?;

    let state: WindowState =
        serde_json::from_str(&content).map_err(|e| PersistenceError::ParseFailed {
            path: path.clone(),
            source: e,
        })?;

    state.validate()?;

    Ok(state)
}

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
pub fn load_or_default(app_name: &str, default_width: u32, default_height: u32) -> WindowState {
    #[cfg(debug_assertions)]
    println!("DEBUG: Loading window state for '{}'", app_name);

    match load_window_state(app_name) {
        Ok(mut state) => {
            #[cfg(debug_assertions)]
            println!("DEBUG: Loaded state: {:?}", state);

            // Validate position
            #[allow(clippy::collapsible_if)]
            if let (Some(x), Some(y)) = (state.x, state.y) {
                if !crate::persistence::monitor::position_is_reasonable(x, y) {
                    tracing::warn!("Ignoring unreasonable window position: {}, {}", x, y);
                    state.x = None;
                    state.y = None;
                }
            }
            state
        }
        Err(e) => {
            #[cfg(debug_assertions)]
            println!("DEBUG: Failed to load state: {}", e);

            // Log warning if it's not just "file not found"
            let is_not_found = matches!(
                &e,
                PersistenceError::ReadFailed { source, .. }
                if source.kind() == std::io::ErrorKind::NotFound
            );

            if !is_not_found {
                tracing::warn!("Failed to load window state for '{}': {}", app_name, e);
            }

            WindowState::with_defaults(default_width, default_height)
        }
    }
}

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
pub fn save_window_state(app_name: &str, state: &WindowState) -> Result<(), PersistenceError> {
    #[cfg(debug_assertions)]
    println!("DEBUG: Saving window state for '{}': {:?}", app_name, state);

    let path = get_config_path(app_name).ok_or_else(|| {
        let e = PersistenceError::NoConfigDir {
            app_name: app_name.to_string(),
        };
        tracing::warn!("Failed to save window state: {}", e);
        e
    })?;

    let parent = path.parent().ok_or_else(|| {
        let e = PersistenceError::WriteFailed {
            path: path.clone(),
            source: std::io::Error::other("Invalid path"),
        };
        tracing::warn!("Failed to save window state: {}", e);
        e
    })?;

    #[allow(clippy::collapsible_if)]
    if !parent.exists() {
        if let Err(e) = fs::create_dir_all(parent) {
            let err = PersistenceError::CreateDirFailed {
                path: parent.to_path_buf(),
                source: e,
            };
            tracing::warn!("Failed to save window state: {}", err);
            return Err(err);
        }
    }

    let temp_path = path.with_extension("tmp");
    let json = match serde_json::to_string_pretty(state) {
        Ok(j) => j,
        Err(e) => {
            let err = PersistenceError::WriteFailed {
                path: path.clone(),
                source: std::io::Error::other(e),
            };
            tracing::warn!("Failed to save window state: {}", err);
            return Err(err);
        }
    };

    if let Err(e) = fs::write(&temp_path, json) {
        let err = PersistenceError::WriteFailed {
            path: temp_path.clone(),
            source: e,
        };
        tracing::warn!("Failed to save window state: {}", err);
        return Err(err);
    }

    if let Err(e) = fs::rename(&temp_path, &path) {
        let err = PersistenceError::WriteFailed {
            path: path.clone(),
            source: e,
        };
        tracing::warn!("Failed to save window state: {}", err);
        return Err(err);
    }

    Ok(())
}
