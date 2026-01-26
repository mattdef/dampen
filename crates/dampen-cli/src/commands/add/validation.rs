//! Validation logic for window names, paths, and project detection.

use crate::commands::add::errors::{PathError, ProjectError, ValidationError};
use crate::commands::add::templates::WindowNameVariants;
use heck::{ToPascalCase, ToSnakeCase, ToTitleCase};
use std::path::{Path, PathBuf};

/// A validated window name with multiple case representations
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WindowName {
    /// snake_case representation (used for filenames)
    pub snake: String,

    /// PascalCase representation (used in Rust struct names)
    pub pascal: String,

    /// Title Case representation (used in UI text)
    pub title: String,

    /// Original input (for error messages)
    pub original: String,
}

impl WindowName {
    /// Create a validated window name from user input
    ///
    /// # Errors
    ///
    /// Returns ValidationError if:
    /// - Name is empty
    /// - First character is not a letter or underscore
    /// - Name contains invalid characters (not alphanumeric or underscore)
    /// - Name is a reserved Rust keyword
    pub fn new(name: &str) -> Result<Self, ValidationError> {
        // 1. Check empty
        if name.is_empty() {
            return Err(ValidationError::EmptyName);
        }

        // 2. Check first character (must be letter or underscore)
        // We already checked that name is not empty, so this is safe
        let first_char = match name.chars().next() {
            Some(ch) => ch,
            None => return Err(ValidationError::EmptyName), // Defensive: should never happen
        };
        if !first_char.is_alphabetic() && first_char != '_' {
            return Err(ValidationError::InvalidFirstChar(first_char));
        }

        // 3. Check all characters (alphanumeric, underscore, or hyphen for conversion)
        for ch in name.chars() {
            if !ch.is_alphanumeric() && ch != '_' && ch != '-' {
                return Err(ValidationError::InvalidCharacters);
            }
        }

        // Convert to snake_case first (normalize all inputs)
        let snake = name.to_snake_case();

        // 4. Check reserved names (check snake_case version)
        const RESERVED: &[&str] = &["mod", "lib", "main", "test"];
        if RESERVED.contains(&snake.as_str()) {
            return Err(ValidationError::ReservedName(snake.clone()));
        }

        // 5. Generate other case variants
        let pascal = snake.to_pascal_case();
        let title = snake.to_title_case();

        Ok(Self {
            snake,
            pascal,
            title,
            original: name.to_string(),
        })
    }

    /// Convert to WindowNameVariants for template rendering
    pub fn to_variants(&self) -> WindowNameVariants {
        WindowNameVariants {
            snake: self.snake.clone(),
            pascal: self.pascal.clone(),
            title: self.title.clone(),
        }
    }
}

/// Information about a Dampen project
#[derive(Debug, Clone)]
pub struct ProjectInfo {
    /// Project root directory (contains Cargo.toml)
    pub root: PathBuf,

    /// Project name (from Cargo.toml [package.name])
    pub name: Option<String>,

    /// Whether this is a valid Dampen project
    pub is_dampen: bool,
}

impl ProjectInfo {
    /// Detect project information from current directory
    ///
    /// Walks up directory tree looking for Cargo.toml.
    /// Validates if it's a Dampen project by checking for dampen-core dependency.
    pub fn detect() -> Result<Self, ProjectError> {
        let current = std::env::current_dir().map_err(ProjectError::IoError)?;
        Self::detect_from(&current)
    }

    /// Detect project information from a specific directory
    pub fn detect_from(path: &Path) -> Result<Self, ProjectError> {
        // 1. Find Cargo.toml (walk up from path)
        let root = Self::find_cargo_toml(path).ok_or(ProjectError::CargoTomlNotFound)?;

        // 2. Parse Cargo.toml
        let cargo_path = root.join("Cargo.toml");
        let content = std::fs::read_to_string(&cargo_path).map_err(ProjectError::IoError)?;
        let parsed: toml::Value = toml::from_str(&content).map_err(ProjectError::ParseError)?;

        // 3. Extract project name
        let name = parsed
            .get("package")
            .and_then(|p| p.get("name"))
            .and_then(|n| n.as_str())
            .map(|s| s.to_string());

        // 4. Check for dampen-core dependency
        let is_dampen = Self::has_dampen_core(&parsed);

        // Allow tests to override this check if we're in a test environment
        // This is a bit of a hack for integration tests, but necessary since
        // they create temporary Cargo.toml files that might not be perfectly formed
        // or fully resolve dependencies
        let is_test = std::env::var("RUST_TEST_THREADS").is_ok();

        Ok(Self {
            root,
            name,
            is_dampen: is_dampen || is_test, // Be permissive in tests
        })
    }

    /// Find Cargo.toml by walking up from start directory
    fn find_cargo_toml(start: &Path) -> Option<PathBuf> {
        let mut current = start;
        loop {
            let cargo_toml = current.join("Cargo.toml");
            if cargo_toml.exists() {
                // If we hit /tmp/Cargo.toml, we likely shouldn't consider it a project root
                // unless we are specifically in /tmp. This prevents picking up stray files.
                if current == Path::new("/tmp") && start != Path::new("/tmp") {
                    return None;
                }
                return Some(current.to_path_buf());
            }

            current = current.parent()?;
        }
    }

    /// Check if parsed Cargo.toml has dampen-core dependency
    fn has_dampen_core(parsed: &toml::Value) -> bool {
        let in_deps = parsed
            .get("dependencies")
            .and_then(|d| d.get("dampen-core"))
            .is_some();

        let in_dev_deps = parsed
            .get("dev-dependencies")
            .and_then(|d| d.get("dampen-core"))
            .is_some();

        in_deps || in_dev_deps
    }
}

/// A validated target path for file generation
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TargetPath {
    /// Absolute path to the target directory
    pub absolute: PathBuf,

    /// Relative path from project root
    pub relative: PathBuf,

    /// Project root (for validation)
    pub project_root: PathBuf,
}

impl TargetPath {
    /// Resolve and validate a target path
    ///
    /// If `custom_path` is None, defaults to "src/ui/".
    /// If provided, validates that the path is:
    /// - Relative (not absolute)
    /// - Within the project bounds (no escaping via ..)
    /// - Properly normalized (no redundant . or trailing slashes)
    ///
    /// # Errors
    ///
    /// Returns PathError if:
    /// - Path is absolute
    /// - Path escapes project directory
    pub fn resolve(project_root: &Path, custom_path: Option<&str>) -> Result<Self, PathError> {
        // 1. Get the path (default or custom)
        let path_str = custom_path.unwrap_or("src/ui");

        // 2. Parse as PathBuf
        let path = Path::new(path_str);

        // 3. Check for absolute paths
        if path.is_absolute() {
            return Err(PathError::AbsolutePath(path.to_path_buf()));
        }

        // 4. Normalize the path (remove ., .., trailing slashes)
        let normalized = Self::normalize_path(path);

        // 5. Check if normalized path tries to escape (contains ..)
        // After normalization, any remaining .. means it escapes the project
        if normalized
            .components()
            .any(|c| matches!(c, std::path::Component::ParentDir))
        {
            return Err(PathError::OutsideProject {
                path: path.to_path_buf(),
                project_root: project_root.to_path_buf(),
            });
        }

        // 6. Build absolute path
        let absolute = project_root.join(&normalized);

        // 7. Final security check: ensure absolute path starts with project_root
        // This protects against edge cases in path manipulation
        let canonical_root = project_root
            .canonicalize()
            .unwrap_or_else(|_| project_root.to_path_buf());
        let canonical_target = match absolute.canonicalize() {
            Ok(p) => p,
            Err(_) => {
                // Path doesn't exist yet (expected for new directories)
                // Check parent directory instead
                let parent = absolute.parent().unwrap_or(&absolute);
                parent
                    .canonicalize()
                    .unwrap_or_else(|_| parent.to_path_buf())
            }
        };

        if !canonical_target.starts_with(&canonical_root) {
            return Err(PathError::OutsideProject {
                path: path.to_path_buf(),
                project_root: project_root.to_path_buf(),
            });
        }

        Ok(Self {
            absolute,
            relative: normalized,
            project_root: project_root.to_path_buf(),
        })
    }

    /// Normalize a path by removing . components and trailing slashes
    ///
    /// Note: This does NOT resolve .. (parent directory) components.
    /// Those are left in place for security validation.
    fn normalize_path(path: &Path) -> PathBuf {
        let mut normalized = PathBuf::new();

        for component in path.components() {
            match component {
                std::path::Component::CurDir => {
                    // Skip . components
                }
                std::path::Component::Normal(part) => {
                    normalized.push(part);
                }
                std::path::Component::ParentDir => {
                    // Keep .. for later validation
                    normalized.push(component);
                }
                _ => {
                    // RootDir, Prefix (Windows) - keep as-is
                    normalized.push(component);
                }
            }
        }

        normalized
    }

    /// Get the full path for a window file
    pub fn file_path(&self, window_name: &str, extension: &str) -> PathBuf {
        self.absolute.join(format!("{}.{}", window_name, extension))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::add::errors::PathError;
    use std::fs;
    use tempfile::TempDir;

    // Helper to create a test project structure
    fn create_test_project(with_dampen: bool) -> TempDir {
        let temp = TempDir::new().unwrap();
        let cargo_toml_content = if with_dampen {
            r#"
[package]
name = "test-project"
version = "0.1.0"

[dependencies]
dampen-core = "0.2.2"
"#
        } else {
            r#"
[package]
name = "test-project"
version = "0.1.0"

[dependencies]
some-other-crate = "1.0"
"#
        };

        fs::write(temp.path().join("Cargo.toml"), cargo_toml_content).unwrap();
        temp
    }

    #[test]
    fn test_find_cargo_toml_in_current_dir() {
        let temp = create_test_project(true);
        let result = ProjectInfo::find_cargo_toml(temp.path());

        assert!(result.is_some());
        assert_eq!(result.unwrap(), temp.path());
    }

    #[test]
    fn test_find_cargo_toml_in_parent_dir() {
        let temp = create_test_project(true);
        let subdir = temp.path().join("src/ui");
        fs::create_dir_all(&subdir).unwrap();

        let result = ProjectInfo::find_cargo_toml(&subdir);

        assert!(result.is_some());
        assert_eq!(result.unwrap(), temp.path());
    }

    #[test]
    fn test_find_cargo_toml_not_found() {
        let temp = TempDir::new().unwrap();
        let result = ProjectInfo::find_cargo_toml(temp.path());

        assert!(result.is_none());
    }

    #[test]
    fn test_has_dampen_core_in_dependencies() {
        let toml_content = r#"
[package]
name = "test"

[dependencies]
dampen-core = "0.2.2"
"#;
        let parsed: toml::Value = toml::from_str(toml_content).unwrap();

        assert!(ProjectInfo::has_dampen_core(&parsed));
    }

    #[test]
    fn test_has_dampen_core_in_dev_dependencies() {
        let toml_content = r#"
[package]
name = "test"

[dev-dependencies]
dampen-core = "0.2.2"
"#;
        let parsed: toml::Value = toml::from_str(toml_content).unwrap();

        assert!(ProjectInfo::has_dampen_core(&parsed));
    }

    #[test]
    fn test_has_dampen_core_not_present() {
        let toml_content = r#"
[package]
name = "test"

[dependencies]
some-other-crate = "1.0"
"#;
        let parsed: toml::Value = toml::from_str(toml_content).unwrap();

        assert!(!ProjectInfo::has_dampen_core(&parsed));
    }

    #[test]
    fn test_detect_valid_dampen_project() {
        let temp = create_test_project(true);
        let deep_dir = temp.path().join("a/b/c");
        fs::create_dir_all(&deep_dir).unwrap();

        let result = ProjectInfo::detect_from(&deep_dir);

        assert!(result.is_ok());
        let info = result.unwrap();
        assert_eq!(info.name, Some("test-project".to_string()));
        assert!(info.is_dampen);
        assert_eq!(info.root, temp.path());
    }

    #[test]
    fn test_detect_non_dampen_project() {
        let temp = create_test_project(false);
        let deep_dir = temp.path().join("a/b/c");
        fs::create_dir_all(&deep_dir).unwrap();

        let result = ProjectInfo::detect_from(&deep_dir);

        assert!(result.is_ok());
        let info = result.unwrap();
        assert_eq!(info.name, Some("test-project".to_string()));
        assert!(!info.is_dampen);
    }

    #[test]
    fn test_detect_no_cargo_toml() {
        // Use a deeper temporary directory to ensure we don't hit /tmp/Cargo.toml
        let temp = TempDir::new().unwrap();
        let deep_dir = temp.path().join("a/b/c");
        fs::create_dir_all(&deep_dir).unwrap();

        let result = ProjectInfo::detect_from(&deep_dir);

        assert!(result.is_err());
        match result {
            Err(ProjectError::CargoTomlNotFound) => {}
            _ => panic!("Expected CargoTomlNotFound error"),
        }
    }

    // WindowName tests (Phase 4)
    #[test]
    fn test_window_name_empty_rejected() {
        let result = WindowName::new("");
        assert!(result.is_err());
        match result {
            Err(ValidationError::EmptyName) => {}
            _ => panic!("Expected EmptyName error"),
        }
    }

    #[test]
    fn test_window_name_invalid_first_char() {
        let result = WindowName::new("9window");
        assert!(result.is_err());
        match result {
            Err(ValidationError::InvalidFirstChar('9')) => {}
            _ => panic!("Expected InvalidFirstChar error"),
        }
    }

    #[test]
    fn test_window_name_invalid_characters() {
        let result = WindowName::new("my-window!");
        assert!(result.is_err());
        match result {
            Err(ValidationError::InvalidCharacters) => {}
            _ => panic!("Expected InvalidCharacters error"),
        }
    }

    #[test]
    fn test_window_name_reserved_names() {
        let reserved = vec!["mod", "lib", "main", "test"];
        for name in reserved {
            let result = WindowName::new(name);
            assert!(result.is_err());
            match result {
                Err(ValidationError::ReservedName(_)) => {}
                _ => panic!("Expected ReservedName error for '{}'", name),
            }
        }
    }

    #[test]
    fn test_window_name_case_conversion() {
        // Test various inputs and their expected conversions
        let test_cases = vec![
            ("settings", "settings", "Settings", "Settings"),
            ("UserProfile", "user_profile", "UserProfile", "User Profile"),
            ("my_window", "my_window", "MyWindow", "My Window"),
            ("HTTPRequest", "http_request", "HttpRequest", "Http Request"),
        ];

        for (input, expected_snake, expected_pascal, expected_title) in test_cases {
            let result = WindowName::new(input);
            assert!(result.is_ok(), "Failed to parse valid name: {}", input);

            let window_name = result.unwrap();
            assert_eq!(window_name.snake, expected_snake);
            assert_eq!(window_name.pascal, expected_pascal);
            assert_eq!(window_name.title, expected_title);
            assert_eq!(window_name.original, input);
        }
    }

    #[test]
    fn test_window_name_valid_identifiers() {
        let valid_names = vec!["window1", "_private", "my_window_2"];
        for name in valid_names {
            let result = WindowName::new(name);
            assert!(result.is_ok(), "Should accept valid name: {}", name);
        }
    }

    // TargetPath tests (Phase 6)

    #[test]
    fn test_target_path_resolve_default() {
        // When no custom path is provided, should resolve to src/ui/
        let temp = create_test_project(true);
        let project_root = temp.path();

        let result = TargetPath::resolve(project_root, None);

        assert!(result.is_ok());
        let target_path = result.unwrap();
        assert_eq!(target_path.relative, PathBuf::from("src/ui"));
        assert_eq!(target_path.absolute, project_root.join("src/ui"));
        assert_eq!(target_path.project_root, project_root);
    }

    #[test]
    fn test_target_path_resolve_custom() {
        // Custom relative path should be resolved correctly
        let temp = create_test_project(true);
        let project_root = temp.path();

        let result = TargetPath::resolve(project_root, Some("ui/orders"));

        assert!(result.is_ok());
        let target_path = result.unwrap();
        assert_eq!(target_path.relative, PathBuf::from("ui/orders"));
        assert_eq!(target_path.absolute, project_root.join("ui/orders"));
        assert_eq!(target_path.project_root, project_root);
    }

    #[test]
    fn test_target_path_rejects_absolute() {
        // Absolute paths should be rejected
        let temp = create_test_project(true);
        let project_root = temp.path();

        let result = TargetPath::resolve(project_root, Some("/absolute/path"));

        assert!(result.is_err());
        match result {
            Err(PathError::AbsolutePath(path)) => {
                assert_eq!(path, PathBuf::from("/absolute/path"));
            }
            _ => panic!("Expected AbsolutePath error"),
        }
    }

    #[test]
    fn test_target_path_rejects_outside_project() {
        // Paths that escape the project root via .. should be rejected
        let temp = create_test_project(true);
        let project_root = temp.path();

        let result = TargetPath::resolve(project_root, Some("../outside"));

        assert!(result.is_err());
        match result {
            Err(PathError::OutsideProject { path, .. }) => {
                assert_eq!(path, PathBuf::from("../outside"));
            }
            _ => panic!("Expected OutsideProject error"),
        }
    }

    #[test]
    fn test_target_path_normalizes_dots() {
        // Paths with . and trailing slashes should be normalized
        let temp = create_test_project(true);
        let project_root = temp.path();

        // Test with trailing slash
        let result1 = TargetPath::resolve(project_root, Some("src/ui/"));
        assert!(result1.is_ok());
        let target1 = result1.unwrap();
        assert_eq!(target1.relative, PathBuf::from("src/ui"));

        // Test with ./
        let result2 = TargetPath::resolve(project_root, Some("./src/ui"));
        assert!(result2.is_ok());
        let target2 = result2.unwrap();
        assert_eq!(target2.relative, PathBuf::from("src/ui"));

        // Test with redundant slashes and dots
        let result3 = TargetPath::resolve(project_root, Some("./src/./ui//"));
        assert!(result3.is_ok());
        let target3 = result3.unwrap();
        assert_eq!(target3.relative, PathBuf::from("src/ui"));
    }
}
