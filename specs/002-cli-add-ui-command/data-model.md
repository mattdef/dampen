# Data Model: CLI Add UI Command

**Feature**: 002-cli-add-ui-command  
**Date**: 2026-01-13  
**Phase**: 1 (Design & Contracts)

## Overview

This document defines the data structures, types, and relationships for the `dampen add --ui` command implementation.

## Core Types

### 1. AddArgs (CLI Arguments)

**Purpose**: Command-line arguments parsed by clap.

```rust
/// Arguments for the `dampen add` command
#[derive(Debug, clap::Args)]
pub struct AddArgs {
    /// Type of component to add (currently only 'ui' supported)
    #[arg(long)]
    pub ui: Option<String>,
    
    /// Custom output directory path (relative to project root)
    /// 
    /// Examples:
    /// - "src/ui/admin/"
    /// - "ui/orders/"
    /// 
    /// If not provided, defaults to "src/ui/"
    #[arg(long)]
    pub path: Option<String>,
}
```

**Validation**:
- Exactly one of `--ui` must be provided (enforced by clap)
- Future extensibility: `--widget`, `--handler`, etc.

**Example usage**:
```bash
dampen add --ui settings
dampen add --ui new_order --path "src/ui/orders/"
```

---

### 2. WindowName (Validated Name)

**Purpose**: Validated and normalized window name in various cases.

```rust
/// A validated window name with multiple case representations
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WindowName {
    /// snake_case representation (used for filenames)
    /// Example: "user_profile"
    pub snake: String,
    
    /// PascalCase representation (used in Rust struct names)
    /// Example: "UserProfile"
    pub pascal: String,
    
    /// Title Case representation (used in UI text)
    /// Example: "User Profile"
    pub title: String,
    
    /// Original input (for error messages)
    pub original: String,
}

impl WindowName {
    /// Create and validate a window name
    /// 
    /// # Errors
    /// 
    /// Returns `Err` if name is invalid:
    /// - Empty string
    /// - Starts with non-letter or non-underscore
    /// - Contains invalid characters (not alphanumeric or underscore after conversion)
    /// - Is a reserved name
    pub fn new(name: &str) -> Result<Self, ValidationError> {
        // 1. Basic validation
        if name.is_empty() {
            return Err(ValidationError::EmptyName);
        }
        
        // 2. Convert to snake_case
        let snake = name.to_snake_case();
        
        // 3. Validate snake_case version
        let first = snake.chars().next().unwrap();
        if !first.is_alphabetic() && first != '_' {
            return Err(ValidationError::InvalidFirstChar(first));
        }
        
        if !snake.chars().all(|c| c.is_alphanumeric() || c == '_') {
            return Err(ValidationError::InvalidCharacters);
        }
        
        // 4. Check reserved names
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
}
```

**Case Conversion Examples**:
```rust
"settings"     → snake: "settings",     pascal: "Settings",     title: "Settings"
"UserProfile"  → snake: "user_profile", pascal: "UserProfile",  title: "User Profile"
"my-window"    → snake: "my_window",    pascal: "MyWindow",     title: "My Window"
"HTTPRequest"  → snake: "http_request", pascal: "HttpRequest",  title: "Http Request"
```

---

### 3. TargetPath (Resolved Path)

**Purpose**: Validated and normalized filesystem path within project.

```rust
/// A validated target directory path
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TargetPath {
    /// Absolute path to target directory
    pub absolute: PathBuf,
    
    /// Relative path from project root (for display)
    pub relative: PathBuf,
    
    /// Project root directory
    pub project_root: PathBuf,
}

impl TargetPath {
    /// Resolve and validate a target path
    /// 
    /// # Arguments
    /// 
    /// * `project_root` - Absolute path to project root (detected via Cargo.toml)
    /// * `custom_path` - Optional custom path (from --path flag)
    /// 
    /// # Returns
    /// 
    /// Resolved path defaulting to `src/ui/` if no custom path provided.
    /// 
    /// # Errors
    /// 
    /// Returns `Err` if:
    /// - Path is absolute and outside project
    /// - Resolved path escapes project directory (via `..`)
    pub fn resolve(
        project_root: PathBuf,
        custom_path: Option<&str>,
    ) -> Result<Self, PathError> {
        let relative = if let Some(custom) = custom_path {
            let path = Path::new(custom);
            
            // Reject absolute paths
            if path.is_absolute() {
                return Err(PathError::AbsolutePath(path.to_path_buf()));
            }
            
            // Normalize (handle ., .., trailing slashes)
            normalize_path(path)
        } else {
            // Default: src/ui/
            PathBuf::from("src/ui")
        };
        
        // Build absolute path
        let absolute = project_root.join(&relative);
        
        // Verify path is within project
        if !absolute.starts_with(&project_root) {
            return Err(PathError::OutsideProject {
                path: relative.clone(),
                project_root: project_root.clone(),
            });
        }
        
        Ok(Self {
            absolute,
            relative,
            project_root,
        })
    }
    
    /// Get the full path for a window file
    pub fn file_path(&self, window_name: &str, extension: &str) -> PathBuf {
        self.absolute.join(format!("{}.{}", window_name, extension))
    }
}
```

**Path Resolution Examples**:
```rust
// Project: /home/user/my-app
// No custom path → src/ui/
resolve(project_root, None) 
// → TargetPath { absolute: "/home/user/my-app/src/ui", relative: "src/ui" }

// Custom: ui/orders/
resolve(project_root, Some("ui/orders/"))
// → TargetPath { absolute: "/home/user/my-app/ui/orders", relative: "ui/orders" }

// Invalid: ../outside/
resolve(project_root, Some("../outside/"))
// → Err(PathError::OutsideProject)
```

---

### 4. WindowTemplate (Template Content)

**Purpose**: Loaded template content with placeholder replacement.

```rust
/// A window file template
#[derive(Debug, Clone)]
pub struct WindowTemplate {
    /// Template content with placeholders
    pub content: String,
    
    /// Template type
    pub kind: TemplateKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TemplateKind {
    /// Rust module (.rs file)
    RustModule,
    
    /// Dampen XML (.dampen file)
    DampenXml,
}

impl WindowTemplate {
    /// Load a template from embedded resources
    pub fn load(kind: TemplateKind) -> Self {
        let content = match kind {
            TemplateKind::RustModule => {
                include_str!("../../../templates/add/window.rs.template")
            }
            TemplateKind::DampenXml => {
                include_str!("../../../templates/add/window.dampen.template")
            }
        };
        
        Self {
            content: content.to_string(),
            kind,
        }
    }
    
    /// Replace placeholders with window name variants
    pub fn render(&self, window_name: &WindowName) -> String {
        self.content
            .replace("{{WINDOW_NAME}}", &window_name.snake)
            .replace("{{WINDOW_NAME_PASCAL}}", &window_name.pascal)
            .replace("{{WINDOW_NAME_TITLE}}", &window_name.title)
    }
}
```

**Template Placeholders**:
- `{{WINDOW_NAME}}` → snake_case (e.g., "user_profile")
- `{{WINDOW_NAME_PASCAL}}` → PascalCase (e.g., "UserProfile")
- `{{WINDOW_NAME_TITLE}}` → Title Case (e.g., "User Profile")

---

### 5. ProjectInfo (Project Detection)

**Purpose**: Information about the detected Dampen project.

```rust
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
        // 1. Find Cargo.toml (walk up from current dir)
        let current = std::env::current_dir()?;
        let root = Self::find_cargo_toml(&current)
            .ok_or(ProjectError::CargoTomlNotFound)?;
        
        // 2. Parse Cargo.toml
        let cargo_path = root.join("Cargo.toml");
        let content = std::fs::read_to_string(&cargo_path)?;
        let parsed: toml::Value = toml::from_str(&content)?;
        
        // 3. Extract project name
        let name = parsed
            .get("package")
            .and_then(|p| p.get("name"))
            .and_then(|n| n.as_str())
            .map(|s| s.to_string());
        
        // 4. Check for dampen-core dependency
        let is_dampen = Self::has_dampen_core(&parsed);
        
        Ok(Self { root, name, is_dampen })
    }
    
    fn find_cargo_toml(start: &Path) -> Option<PathBuf> {
        let mut current = start;
        loop {
            let cargo_toml = current.join("Cargo.toml");
            if cargo_toml.exists() {
                return Some(current.to_path_buf());
            }
            
            current = current.parent()?;
        }
    }
    
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
```

**Detection Logic**:
1. Start from current directory
2. Walk up until finding `Cargo.toml`
3. Parse TOML to extract project name
4. Check for `dampen-core` in dependencies or dev-dependencies
5. Return project info with validation result

---

### 6. GeneratedFiles (Output Result)

**Purpose**: Information about successfully generated files.

```rust
/// Result of file generation
#[derive(Debug, Clone)]
pub struct GeneratedFiles {
    /// Path to generated .rs file
    pub rust_file: PathBuf,
    
    /// Path to generated .dampen file
    pub dampen_file: PathBuf,
    
    /// Window name used
    pub window_name: WindowName,
    
    /// Target directory (relative to project)
    pub target_dir: PathBuf,
}

impl GeneratedFiles {
    /// Create a success message for display
    pub fn success_message(&self) -> String {
        format!(
            "Created new UI window: {}\n  \
             - {}\n  \
             - {}\n\n\
             Next steps:\n  \
             1. Add 'pub mod {};' to src/ui/mod.rs\n  \
             2. Use in your application's view function\n  \
             3. Run 'cargo build' to compile",
            self.window_name.snake,
            self.rust_file.display(),
            self.dampen_file.display(),
            self.window_name.snake
        )
    }
}
```

**Example Output**:
```
Created new UI window: settings
  - src/ui/settings.rs
  - src/ui/settings.dampen

Next steps:
  1. Add 'pub mod settings;' to src/ui/mod.rs
  2. Use in your application's view function
  3. Run 'cargo build' to compile
```

---

## Error Types

### ValidationError

**Purpose**: Window name validation errors.

```rust
#[derive(Debug, thiserror::Error)]
pub enum ValidationError {
    #[error("Window name cannot be empty")]
    EmptyName,
    
    #[error("Window name must start with a letter or underscore, found '{0}'")]
    InvalidFirstChar(char),
    
    #[error("Window name contains invalid characters (only letters, numbers, and underscores allowed)")]
    InvalidCharacters,
    
    #[error("'{0}' is a reserved name")]
    ReservedName(String),
}
```

### PathError

**Purpose**: Path resolution and validation errors.

```rust
#[derive(Debug, thiserror::Error)]
pub enum PathError {
    #[error("Absolute paths are not allowed: {0}")]
    AbsolutePath(PathBuf),
    
    #[error("Path '{path}' is outside project directory '{project_root}'")]
    OutsideProject {
        path: PathBuf,
        project_root: PathBuf,
    },
    
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}
```

### ProjectError

**Purpose**: Project detection and validation errors.

```rust
#[derive(Debug, thiserror::Error)]
pub enum ProjectError {
    #[error("Cargo.toml not found in current directory or any parent directory")]
    CargoTomlNotFound,
    
    #[error("Not a Dampen project: dampen-core dependency not found in Cargo.toml")]
    NotDampenProject,
    
    #[error("Failed to read Cargo.toml: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Failed to parse Cargo.toml: {0}")]
    ParseError(#[from] toml::de::Error),
}
```

### GenerationError

**Purpose**: File generation errors.

```rust
#[derive(Debug, thiserror::Error)]
pub enum GenerationError {
    #[error("Window '{window_name}' already exists at {path}")]
    FileExists {
        window_name: String,
        path: PathBuf,
    },
    
    #[error("Failed to create directory {path}: {source}")]
    DirectoryCreation {
        path: PathBuf,
        source: std::io::Error,
    },
    
    #[error("Failed to write file {path}: {source}")]
    FileWrite {
        path: PathBuf,
        source: std::io::Error,
    },
}
```

---

## State Transitions

### Command Execution Flow

```
[User Input]
    ↓
[Parse Args] → AddArgs
    ↓
[Detect Project] → ProjectInfo
    ↓ (if not Dampen project)
    ✗ ProjectError::NotDampenProject
    ↓ (if valid)
[Validate Name] → WindowName
    ↓ (if invalid)
    ✗ ValidationError
    ↓ (if valid)
[Resolve Path] → TargetPath
    ↓ (if invalid)
    ✗ PathError
    ↓ (if valid)
[Check Duplicates]
    ↓ (if exists)
    ✗ GenerationError::FileExists
    ↓ (if not exists)
[Load Templates] → Vec<WindowTemplate>
    ↓
[Render Templates]
    ↓
[Create Directory]
    ↓ (if fails)
    ✗ GenerationError::DirectoryCreation
    ↓ (if succeeds)
[Write Files]
    ↓ (if fails)
    ✗ GenerationError::FileWrite (+ cleanup)
    ↓ (if succeeds)
[Return Success] → GeneratedFiles
```

### Validation States

```rust
/// Window name validation state
enum NameState {
    Unparsed(String),                    // Raw input
    Validated(WindowName),                // Passed validation
    Invalid(String, ValidationError),     // Failed validation
}

/// Path resolution state
enum PathState {
    Unresolved(Option<String>),          // --path argument
    Resolved(TargetPath),                 // Within project bounds
    Invalid(String, PathError),           // Outside project or absolute
}

/// File existence state
enum FileState {
    Unchecked,                            // Not yet checked
    Available,                            // Files don't exist
    Conflict(PathBuf),                    // At least one file exists
}
```

---

## Relationships

```
AddArgs
  ├─ ui: Option<String> ──────→ WindowName
  └─ path: Option<String> ────→ TargetPath

ProjectInfo
  └─ root: PathBuf ───────────→ TargetPath (project_root)

WindowName
  ├─ snake: String ───────────→ filename generation
  ├─ pascal: String ──────────→ template placeholder
  └─ title: String ───────────→ template placeholder

TargetPath
  ├─ absolute: PathBuf ───────→ fs::write destination
  └─ relative: PathBuf ───────→ user-facing messages

WindowTemplate
  ├─ content: String ─────────→ render()
  └─ kind: TemplateKind ──────→ file extension

GeneratedFiles
  ├─ rust_file: PathBuf
  ├─ dampen_file: PathBuf
  ├─ window_name: WindowName
  └─ target_dir: PathBuf
```

---

## Size & Performance

**Memory Usage**:
- `AddArgs`: ~100 bytes (2 Option<String>)
- `WindowName`: ~200 bytes (4 String fields)
- `TargetPath`: ~300 bytes (3 PathBuf fields)
- `WindowTemplate`: ~10 KB (template content)
- `ProjectInfo`: ~200 bytes (PathBuf + Option<String>)
- `GeneratedFiles`: ~500 bytes (2 PathBuf + WindowName + PathBuf)

**Total per command**: <20 KB (dominated by template content)

**I/O Operations**:
1. Read current directory (path traversal)
2. Read Cargo.toml (~1-5 KB)
3. Check file existence (2 stat calls)
4. Create directory (mkdir -p, may be no-op)
5. Write .rs file (~1-2 KB)
6. Write .dampen file (~300 bytes)

**Total I/O**: <10 KB read, <5 KB write, ~10ms expected latency

---

## Thread Safety

All types are `Send + Sync` (no interior mutability):
- File operations use `std::fs` (thread-safe)
- No shared state between invocations
- Command execution is single-threaded (CLI context)

---

## Next Steps

1. ✅ Data model complete
2. → Generate contract templates (window.rs.template, window.dampen.template)
3. → Create quickstart.md with usage examples
4. → Update AGENTS.md with new patterns
