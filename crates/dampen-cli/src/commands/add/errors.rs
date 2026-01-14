//! Error types for the add command.

use std::path::PathBuf;
use thiserror::Error;

/// Errors that can occur during window name validation.
#[derive(Debug, Error)]
pub enum ValidationError {
    /// Window name is empty
    #[error("Window name cannot be empty")]
    EmptyName,

    /// Window name starts with an invalid character
    #[error("Window name must start with a letter or underscore, found '{0}'")]
    InvalidFirstChar(char),

    /// Window name contains invalid characters
    #[error(
        "Window name contains invalid characters (only letters, numbers, and underscores allowed)"
    )]
    InvalidCharacters,

    /// Window name is a reserved keyword
    #[error("'{0}' is a reserved name")]
    ReservedName(String),
}

/// Errors that can occur during path resolution and validation.
#[derive(Debug, Error)]
pub enum PathError {
    /// Path is absolute (not allowed)
    #[error(
        "Absolute paths are not allowed: {0}\nhelp: Use a relative path within the project, e.g., 'src/ui/orders/'"
    )]
    AbsolutePath(PathBuf),

    /// Path escapes project directory
    #[error(
        "Path '{path}' is outside the project directory\nhelp: Use a relative path within the project, e.g., 'src/ui/orders/'"
    )]
    OutsideProject {
        /// The problematic path
        path: PathBuf,
        /// The project root for reference
        project_root: PathBuf,
    },

    /// I/O error during path operations
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}

/// Errors that can occur during project detection and validation.
#[derive(Debug, Error)]
pub enum ProjectError {
    /// Cargo.toml not found in current directory or parents
    #[error(
        "Not a Dampen project: Cargo.toml not found in current directory or any parent directory\nhelp: Run 'dampen new <project_name>' to create a new Dampen project"
    )]
    CargoTomlNotFound,

    /// Project doesn't have dampen-core dependency
    #[error(
        "Not a Dampen project: dampen-core dependency not found in Cargo.toml\nhelp: Add dampen-core to [dependencies] or run 'dampen new' to create a new project"
    )]
    NotDampenProject,

    /// I/O error reading Cargo.toml
    #[error("Failed to read Cargo.toml: {0}")]
    IoError(#[from] std::io::Error),

    /// Error parsing Cargo.toml
    #[error("Failed to parse Cargo.toml: {0}")]
    ParseError(#[from] toml::de::Error),
}

/// Errors that can occur during file generation.
#[derive(Debug, Error)]
pub enum GenerationError {
    /// Window file already exists
    #[error(
        "Window '{window_name}' already exists at {path}\nhelp: Choose a different name or remove the existing file first"
    )]
    FileExists {
        /// The window name that conflicts
        window_name: String,
        /// The path to the conflicting file
        path: PathBuf,
    },

    /// Failed to create directory
    #[error("Failed to create directory {path}: {source}")]
    DirectoryCreation {
        /// The directory path
        path: PathBuf,
        /// The underlying I/O error
        #[source]
        source: std::io::Error,
    },

    /// Failed to write file
    #[error("Failed to write file {path}: {source}")]
    FileWrite {
        /// The file path
        path: PathBuf,
        /// The underlying I/O error
        #[source]
        source: std::io::Error,
    },
}

/// Errors that can occur during automatic integration of modules.
#[derive(Debug, Error)]
pub enum IntegrationError {
    /// Failed to read a mod.rs file
    #[error("Failed to read {path}: {source}")]
    ModFileRead {
        /// The mod.rs file path
        path: PathBuf,
        /// The underlying I/O error
        #[source]
        source: std::io::Error,
    },

    /// Failed to write a mod.rs file
    #[error("Failed to write {path}: {source}")]
    ModFileWrite {
        /// The mod.rs file path
        path: PathBuf,
        /// The underlying I/O error
        #[source]
        source: std::io::Error,
    },

    /// Failed to create directory
    #[error("Failed to create directory {path}: {source}")]
    DirectoryCreation {
        /// The directory path
        path: PathBuf,
        /// The underlying I/O error
        #[source]
        source: std::io::Error,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validation_error_empty_name() {
        let err = ValidationError::EmptyName;
        assert_eq!(err.to_string(), "Window name cannot be empty");
    }

    #[test]
    fn test_validation_error_invalid_first_char() {
        let err = ValidationError::InvalidFirstChar('9');
        assert_eq!(
            err.to_string(),
            "Window name must start with a letter or underscore, found '9'"
        );
    }

    #[test]
    fn test_validation_error_invalid_characters() {
        let err = ValidationError::InvalidCharacters;
        assert_eq!(
            err.to_string(),
            "Window name contains invalid characters (only letters, numbers, and underscores allowed)"
        );
    }

    #[test]
    fn test_validation_error_reserved_name() {
        let err = ValidationError::ReservedName("mod".to_string());
        assert_eq!(err.to_string(), "'mod' is a reserved name");
    }

    #[test]
    fn test_path_error_absolute_path() {
        let err = PathError::AbsolutePath(PathBuf::from("/absolute/path"));
        let msg = err.to_string();
        assert!(msg.contains("Absolute paths are not allowed"));
        assert!(msg.contains("/absolute/path"));
        assert!(msg.contains("help:"));
    }

    #[test]
    fn test_path_error_outside_project() {
        let err = PathError::OutsideProject {
            path: PathBuf::from("../outside"),
            project_root: PathBuf::from("/project"),
        };
        let msg = err.to_string();
        assert!(msg.contains("outside the project directory"));
        assert!(msg.contains("help:"));
    }

    #[test]
    fn test_project_error_cargo_toml_not_found() {
        let err = ProjectError::CargoTomlNotFound;
        let msg = err.to_string();
        assert!(msg.contains("Cargo.toml not found"));
        assert!(msg.contains("help:"));
        assert!(msg.contains("dampen new"));
    }

    #[test]
    fn test_project_error_not_dampen_project() {
        let err = ProjectError::NotDampenProject;
        let msg = err.to_string();
        assert!(msg.contains("dampen-core dependency not found"));
        assert!(msg.contains("help:"));
    }

    #[test]
    fn test_generation_error_file_exists() {
        let err = GenerationError::FileExists {
            window_name: "settings".to_string(),
            path: PathBuf::from("src/ui/settings.rs"),
        };
        let msg = err.to_string();
        assert!(msg.contains("already exists"));
        assert!(msg.contains("settings"));
        assert!(msg.contains("help:"));
    }

    #[test]
    fn test_generation_error_directory_creation() {
        let io_err = std::io::Error::new(std::io::ErrorKind::PermissionDenied, "permission denied");
        let err = GenerationError::DirectoryCreation {
            path: PathBuf::from("src/ui/admin"),
            source: io_err,
        };
        let msg = err.to_string();
        assert!(msg.contains("Failed to create directory"));
        assert!(msg.contains("src/ui/admin"));
    }

    #[test]
    fn test_generation_error_file_write() {
        let io_err = std::io::Error::new(std::io::ErrorKind::PermissionDenied, "permission denied");
        let err = GenerationError::FileWrite {
            path: PathBuf::from("src/ui/settings.rs"),
            source: io_err,
        };
        let msg = err.to_string();
        assert!(msg.contains("Failed to write file"));
        assert!(msg.contains("src/ui/settings.rs"));
    }
}
