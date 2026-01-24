use std::path::PathBuf;

/// Error type for window persistence operations.
#[derive(Debug, thiserror::Error)]
pub enum PersistenceError {
    /// Failed to determine the configuration directory for the application.
    #[error("Failed to determine config directory for app '{app_name}'")]
    NoConfigDir {
        /// The application name that was used.
        app_name: String,
    },

    /// Failed to create the configuration directory.
    #[error("Failed to create config directory '{path}': {source}")]
    CreateDirFailed {
        /// Path to the directory.
        path: PathBuf,
        /// Underlying I/O error.
        #[source]
        source: std::io::Error,
    },

    /// Failed to read the configuration file.
    #[error("Failed to read config file '{path}': {source}")]
    ReadFailed {
        /// Path to the file.
        path: PathBuf,
        /// Underlying I/O error.
        #[source]
        source: std::io::Error,
    },

    /// Failed to write the configuration file.
    #[error("Failed to write config file '{path}': {source}")]
    WriteFailed {
        /// Path to the file.
        path: PathBuf,
        /// Underlying I/O error.
        #[source]
        source: std::io::Error,
    },

    /// Failed to parse the configuration file (JSON error).
    #[error("Failed to parse config file '{path}': {source}")]
    ParseFailed {
        /// Path to the file.
        path: PathBuf,
        /// Underlying JSON parse error.
        #[source]
        source: serde_json::Error,
    },

    /// The window state contains invalid values (e.g. dimensions too small).
    #[error("Invalid window state: {reason}")]
    InvalidState {
        /// Description of why the state is invalid.
        reason: String,
    },
}
