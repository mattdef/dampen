# Research & Technical Decisions: CLI Add UI Command

**Feature**: 002-cli-add-ui-command  
**Date**: 2026-01-13  
**Phase**: 0 (Research & Decisions)

## Overview

This document records research findings and technical decisions for implementing the `dampen add --ui` command. All "NEEDS CLARIFICATION" items from the plan are resolved here.

## Decision 1: Window Name Validation Patterns

### Research Summary

Rust identifier rules (from Rust Reference):
- Must start with: letter (a-z, A-Z) or underscore (_)
- Can contain: letters, digits (0-9), underscores
- Cannot be: Rust keywords (fn, let, struct, etc.)
- Convention: snake_case for variables, functions, modules

Existing validation in `dampen new`:
```rust
// From crates/dampen-cli/src/commands/new.rs:96-126
fn validate_project_name(name: &str) -> Result<(), String> {
    if name.is_empty() { return Err(...); }
    if !first.is_alphabetic() && first != '_' { return Err(...); }
    if !name.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_') {
        return Err(...);
    }
    // Reserved names check
}
```

### Decision: Reuse and Extend Existing Pattern

**Chosen Approach**: Adapt `validate_project_name` logic for window names.

**Rationale**:
1. Proven pattern already in codebase
2. Consistent validation across CLI commands
3. Handles edge cases (empty, special chars, reserved words)

**Differences from project validation**:
- Allow hyphens in input but convert to underscores for filenames
- More permissive reserved word list (only filesystem-sensitive: `mod`, `lib`, `main`)
- Apply automatic case conversion (see Decision 2)

**Implementation**:
```rust
fn validate_window_name(name: &str) -> Result<String, String> {
    // 1. Basic validation (empty, first char, allowed chars)
    // 2. Check reserved names: ["mod", "lib", "main", "test"]
    // 3. Apply snake_case conversion (Decision 2)
    // 4. Return converted name
}
```

---

## Decision 2: Case Conversion Strategy

### Research Summary

Case conversion strategies:
- **Manual**: Split on uppercase, join with underscore
- **Crate**: Use `heck` crate (0.5+) for `to_snake_case()`
- **Regex**: Pattern-based splitting (overkill for this use case)

The `heck` crate is already used in Dampen macros:
```toml
# From crates/dampen-macros/Cargo.toml
heck = "0.5"
```

### Decision: Use `heck` Crate

**Chosen Approach**: Add `heck` as dependency to `dampen-cli`, use `to_snake_case()`.

**Rationale**:
1. Already in workspace (zero new dependencies at workspace level)
2. Handles edge cases correctly (acronyms, numbers, consecutive capitals)
3. Battle-tested library (100k+ downloads/month)
4. Minimal code: `use heck::ToSnakeCase; name.to_snake_case()`

**Examples**:
```rust
"MyWindow".to_snake_case()      // => "my_window"
"UserProfile".to_snake_case()   // => "user_profile"
"HTTPRequest".to_snake_case()   // => "http_request"
"user_profile".to_snake_case()  // => "user_profile" (idempotent)
```

**Alternative Rejected**: Manual conversion with `.chars()` iteration - more code, more edge cases, less tested.

---

## Decision 3: Template Engine Choice

### Research Summary

Options evaluated:
1. **Simple replacement** (existing pattern in `dampen new`)
   - Uses `include_str!` + `.replace()`
   - ~5 lines of code per template
   - Example: `template.replace("{{PROJECT_NAME}}", project_name)`

2. **Handlebars** (0.5+ MB, 50+ dependencies)
   - Full templating language with conditionals, loops
   - Overkill for simple placeholder replacement

3. **tera** (similar to Handlebars, Django-inspired)
   - Same complexity concerns

### Decision: Simple String Replacement

**Chosen Approach**: Reuse the pattern from `dampen new` command.

**Rationale**:
1. Zero new dependencies (Constitution alignment)
2. Sufficient for our needs (3-4 placeholders max)
3. Compile-time template inclusion (`include_str!`)
4. Existing pattern in codebase (consistency)
5. Easy to test and maintain

**Placeholders needed**:
```rust
// For window.rs.template
{{WINDOW_NAME}}         // snake_case: "settings"
{{WINDOW_NAME_PASCAL}}  // PascalCase: "Settings"

// For window.dampen.template
{{WINDOW_NAME_TITLE}}   // Title Case: "Settings"
```

**Implementation pattern**:
```rust
let template = include_str!("../../templates/add/window.rs.template");
let content = template
    .replace("{{WINDOW_NAME}}", &window_name)
    .replace("{{WINDOW_NAME_PASCAL}}", &to_pascal_case(&window_name))
    .replace("{{WINDOW_NAME_TITLE}}", &to_title_case(&window_name));
```

**Alternative Rejected**: Template engine - adds unnecessary complexity and dependencies for simple placeholder substitution.

---

## Decision 4: Cargo.toml Parsing

### Research Summary

Options for detecting Dampen projects:
1. **cargo_metadata crate**
   - Parses full Cargo.toml + workspace structure
   - Heavyweight: JSON output parsing, subprocess invocation
   - 500 KB crate + dependencies

2. **toml crate**
   - Parse Cargo.toml manually
   - 100 KB crate
   - Requires handling workspace inheritance

3. **String matching**
   - Simple: check if "dampen-core" or "dampen_core" in file content
   - Fast: no parsing overhead
   - Fragile: could match comments or false positives

### Decision: Hybrid Approach (toml crate)

**Chosen Approach**: Use `toml` crate (0.8+) for lightweight parsing.

**Rationale**:
1. Reliable: Proper TOML parsing, handles edge cases
2. Lightweight: ~100 KB, pure Rust, no subprocess
3. Targeted: Only parse `[dependencies]` and `[dev-dependencies]` sections
4. Maintainable: Less fragile than string matching

**Implementation**:
```rust
fn is_dampen_project(path: &Path) -> Result<bool, String> {
    let cargo_toml = path.join("Cargo.toml");
    if !cargo_toml.exists() {
        return Ok(false);
    }
    
    let content = fs::read_to_string(&cargo_toml)?;
    let parsed: toml::Value = toml::from_str(&content)?;
    
    let has_dep = parsed
        .get("dependencies")
        .and_then(|d| d.get("dampen-core"))
        .is_some();
    
    let has_dev_dep = parsed
        .get("dev-dependencies")
        .and_then(|d| d.get("dampen-core"))
        .is_some();
    
    Ok(has_dep || has_dev_dep)
}
```

**Alternative Rejected**: 
- `cargo_metadata`: Too heavyweight for simple check
- String matching: Too fragile, could miss workspace setups

**Note**: `toml` crate is already in dampen-cli dependencies for other purposes.

---

## Decision 5: Path Normalization

### Research Summary

Path handling requirements:
1. Resolve relative paths (e.g., `ui/orders/` → `src/ui/orders/`)
2. Detect absolute paths outside project
3. Cross-platform compatibility (Windows backslash, Unix forward slash)
4. Handle trailing slashes, `.`, `..`

Rust stdlib approach:
```rust
use std::path::{Path, PathBuf};

// Canonicalize resolves symlinks, requires path to exist
path.canonicalize()?;

// Components-based normalization (no IO)
fn normalize_path(path: &Path) -> PathBuf {
    let mut components = Vec::new();
    for component in path.components() {
        match component {
            Component::ParentDir => { components.pop(); }
            Component::Normal(c) => components.push(c),
            _ => {}
        }
    }
    components.iter().collect()
}
```

### Decision: Custom Normalization (No External Crate)

**Chosen Approach**: Use `std::path::PathBuf` + custom normalization logic.

**Rationale**:
1. No new dependencies (stdlib is sufficient)
2. Works with non-existent paths (no IO required)
3. Cross-platform via `std::path` abstractions
4. Handles `.`, `..`, trailing slashes

**Implementation strategy**:
```rust
fn resolve_target_path(
    project_root: &Path,
    custom_path: Option<&str>,
    window_name: &str,
) -> Result<PathBuf, String> {
    let base = if let Some(custom) = custom_path {
        // Custom path provided
        let path = Path::new(custom);
        
        // Reject absolute paths outside project
        if path.is_absolute() {
            return Err("Path must be relative to project root".to_string());
        }
        
        // Normalize and prepend project root
        project_root.join(normalize_path(path))
    } else {
        // Default: src/ui/
        project_root.join("src").join("ui")
    };
    
    // Verify resolved path is within project
    if !base.starts_with(project_root) {
        return Err("Path escapes project directory".to_string());
    }
    
    Ok(base)
}
```

**Edge cases handled**:
- `ui/orders/` → normalized to `src/ui/orders/`
- `./ui/` → same as `ui/`
- `../outside/` → rejected (escapes project)
- `C:\absolute\path` (Windows) → rejected
- Trailing slashes → automatically handled by PathBuf

**Alternative Rejected**: `pathdiff` crate - adds dependency for functionality we can implement with stdlib.

---

## Decision 6: Atomic File Creation

### Research Summary

Strategies to prevent partial writes:
1. **Write-to-temp + rename**
   - Write to `{filename}.tmp`
   - Rename to final name (atomic on POSIX)
   - Cleanup on error

2. **Transactional wrapper**
   - Create both files
   - Delete both if either fails
   - Requires rollback logic

3. **Pre-check + write**
   - Validate everything first
   - Write files
   - Rely on filesystem atomicity

### Decision: Pre-Check + Cleanup on Error

**Chosen Approach**: Validate upfront, write files, cleanup on partial failure.

**Rationale**:
1. Simplest implementation (no temp files)
2. Sufficient for CLI tool (not high-concurrency scenario)
3. Clear error messages show which file failed
4. Matches pattern in `dampen new` command

**Implementation**:
```rust
fn generate_window_files(
    target_dir: &Path,
    window_name: &str,
) -> Result<(PathBuf, PathBuf), String> {
    let rs_path = target_dir.join(format!("{}.rs", window_name));
    let dampen_path = target_dir.join(format!("{}.dampen", window_name));
    
    // Pre-check: both files must not exist
    if rs_path.exists() {
        return Err(format!("File already exists: {}", rs_path.display()));
    }
    if dampen_path.exists() {
        return Err(format!("File already exists: {}", dampen_path.display()));
    }
    
    // Create directory if needed
    fs::create_dir_all(target_dir)
        .map_err(|e| format!("Failed to create directory: {}", e))?;
    
    // Generate content
    let rs_content = generate_rs_template(window_name);
    let dampen_content = generate_dampen_template(window_name);
    
    // Write .rs file
    fs::write(&rs_path, rs_content)
        .map_err(|e| format!("Failed to write {}: {}", rs_path.display(), e))?;
    
    // Write .dampen file (cleanup .rs if this fails)
    if let Err(e) = fs::write(&dampen_path, dampen_content) {
        let _ = fs::remove_file(&rs_path); // Cleanup
        return Err(format!("Failed to write {}: {}", dampen_path.display(), e));
    }
    
    Ok((rs_path, dampen_path))
}
```

**Error scenarios handled**:
- Disk full: Returns error after .rs write, cleans up .rs
- Permission denied: Returns error before any writes
- .rs succeeds, .dampen fails: Cleans up .rs file

**Alternative Rejected**: Write-to-temp approach - more complex, unnecessary for single-user CLI tool.

---

## Decision 7: Error Message Design

### Research Summary

CLI UX best practices (from Cargo, clap examples):
1. **Structure**: `Error: {what went wrong}`
2. **Context**: Include relevant details (file paths, values)
3. **Suggestions**: Offer actionable next steps
4. **Color**: Use ANSI colors for terminals (optional)

Cargo example:
```
error: package `my-app` already exists
  --> Cargo.toml:2:1
help: try using a different name or remove the existing directory
```

### Decision: Three-Part Error Format

**Chosen Approach**: Error message + context + suggestion.

**Rationale**:
1. Clear: User knows what failed
2. Actionable: User knows how to fix
3. Consistent: Matches Rust/Cargo conventions
4. No colors: Keep output simple (clap handles color via `--color` flag)

**Error message patterns**:

```rust
// File exists
format!(
    "Window '{}' already exists at {}\n\
     help: Choose a different name or remove the existing file first",
    window_name, path.display()
)

// Not a Dampen project
format!(
    "Not a Dampen project: Cargo.toml not found in {}\n\
     help: Run 'dampen new <project_name>' to create a new Dampen project",
    path.display()
)

// Missing dependency
format!(
    "Not a Dampen project: dampen-core dependency not found in Cargo.toml\n\
     help: Add dampen-core to [dependencies] or run 'dampen new' to create a new project"
)

// Invalid window name
format!(
    "Invalid window name '{}': {}\n\
     help: Use only letters, numbers, and underscores. Examples: settings, user_profile, my_window",
    name, reason
)

// Path outside project
format!(
    "Path '{}' is outside the project directory\n\
     help: Use a relative path within the project, e.g., 'src/ui/orders/'",
    path.display()
)
```

**Implementation**:
```rust
// Custom error type
#[derive(Debug)]
pub enum AddError {
    InvalidName { name: String, reason: String },
    NotDampenProject { path: PathBuf },
    FileExists { path: PathBuf, window_name: String },
    PathOutsideProject { path: PathBuf },
    IoError { context: String, source: std::io::Error },
}

impl fmt::Display for AddError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::InvalidName { name, reason } => write!(
                f,
                "Invalid window name '{}': {}\n\
                 help: Use only letters, numbers, and underscores. Examples: settings, user_profile",
                name, reason
            ),
            // ... other variants
        }
    }
}
```

**Alternative Rejected**: Single-line errors without suggestions - less helpful for users.

---

## Technology Stack Summary

| Component | Technology | Justification |
|-----------|-----------|---------------|
| CLI parsing | clap 4.0+ | Existing dependency, standard in Rust CLI |
| Case conversion | heck 0.5+ | Already in workspace (dampen-macros) |
| Path handling | std::path | Stdlib sufficient, cross-platform |
| File I/O | std::fs | Stdlib sufficient, simple needs |
| TOML parsing | toml 0.8+ | Existing dependency in dampen-cli |
| Templates | include_str! + replace | Existing pattern in dampen new |
| Error handling | Custom enum + Display | Follows Rust conventions |

**Zero new major dependencies** - All crates either in stdlib or already in workspace.

---

## Integration Points

### Reuse from `dampen new`

1. **Template pattern**:
   - `include_str!("../../templates/...")`
   - `.replace("{{...}}", value)`
   - File: `crates/dampen-cli/src/commands/new.rs:180-290`

2. **Validation pattern**:
   - Character-by-character validation
   - Reserved names check
   - File: `crates/dampen-cli/src/commands/new.rs:96-126`

3. **Directory creation**:
   - `fs::create_dir_all()`
   - Error context wrapping
   - File: `crates/dampen-cli/src/commands/new.rs:146-178`

4. **Cleanup on error**:
   - Manual file removal on partial failure
   - File: `crates/dampen-cli/src/commands/new.rs:292-297`

### New Patterns

1. **Project detection**: Check Cargo.toml for dampen-core
2. **Path normalization**: Resolve custom paths within project bounds
3. **Duplicate detection**: Check for existing .rs and .dampen files

---

## Open Questions

None - all research items resolved.

---

## Next Steps

1. ✅ Research complete
2. → Proceed to Phase 1: Generate data-model.md, contracts/, quickstart.md
3. → Update agent context (AGENTS.md)
4. → Run `/speckit.tasks` for implementation breakdown
