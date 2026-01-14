# Contract: Rust Module Template (window.rs.template)

**Feature**: 002-cli-add-ui-command  
**Template**: `crates/dampen-cli/templates/add/window.rs.template`  
**Purpose**: Generate Rust module file for new UI window

## Template Contract

### Placeholders

| Placeholder | Description | Example Value |
|-------------|-------------|---------------|
| `{{WINDOW_NAME}}` | snake_case window name (for filenames, variables) | `settings` |
| `{{WINDOW_NAME_PASCAL}}` | PascalCase window name (for struct names) | `Settings` |

**Note**: `{{WINDOW_NAME_TITLE}}` is not used in Rust files (only in XML).

### Required Sections

The template MUST contain:

1. **File documentation comment**
   - Explains auto-loading pattern
   - References corresponding `.dampen` file

2. **Import statements**
   - `dampen_core::{AppState, HandlerRegistry}`
   - `dampen_macros::{UiModel, dampen_ui}`
   - `serde::{Deserialize, Serialize}`

3. **`#[dampen_ui]` macro invocation**
   - Loads corresponding XML file
   - Module name: `_app` (convention)
   - Path: `"{{WINDOW_NAME}}.dampen"` (relative to file)

4. **Model struct**
   - Name: `Model` (not parameterized)
   - Derives: `Default, UiModel, Serialize, Deserialize, Clone, Debug`
   - Fields: At least one example field (e.g., `message: String`)

5. **`create_app_state()` function**
   - Returns: `AppState<Model>`
   - Implementation: Loads document + handlers

6. **`create_handler_registry()` function**
   - Returns: `HandlerRegistry`
   - Includes: At least one example handler registration

### Template Content

```rust
// Auto-loaded UI module for {{WINDOW_NAME}}.
//
// This file is automatically compiled and loads the corresponding {{WINDOW_NAME}}.dampen XML file.

use dampen_core::{AppState, HandlerRegistry};
use dampen_macros::{UiModel, dampen_ui};
use serde::{Deserialize, Serialize};

/// Auto-load the {{WINDOW_NAME}}.dampen XML file.
/// Path is relative to this file.
#[dampen_ui("{{WINDOW_NAME}}.dampen")]
mod _app {}

/// The application model for {{WINDOW_NAME}}.
#[derive(Default, UiModel, Serialize, Deserialize, Clone, Debug)]
pub struct Model {
    /// Example field - replace with your own model fields
    pub message: String,
}

/// Create the AppState for the {{WINDOW_NAME}} view.
pub fn create_app_state() -> AppState<Model> {
    let document = _app::document();
    let handler_registry = create_handler_registry();
    AppState::with_handlers(document, handler_registry)
}

/// Create and configure the handler registry.
pub fn create_handler_registry() -> HandlerRegistry {
    let registry = HandlerRegistry::new();

    // Example handler - replace with your own handlers
    registry.register_simple("example_handler", |model: &mut dyn std::any::Any| {
        if let Some(m) = model.downcast_mut::<Model>() {
            m.message = "Handler triggered!".to_string();
        }
    });

    registry
}
```

### Validation Rules

**After rendering**, the generated file MUST:

1. ✅ Compile without errors in a Dampen project
2. ✅ Pass `cargo clippy` with no warnings
3. ✅ Pass `cargo fmt --check`
4. ✅ Reference correct `.dampen` file name
5. ✅ Include at least one example handler
6. ✅ Have valid rustdoc comments

### Contract Tests

```rust
#[test]
fn test_rust_template_structure() {
    let window_name = WindowName::new("settings").unwrap();
    let template = WindowTemplate::load(TemplateKind::RustModule);
    let rendered = template.render(&window_name);
    
    // Must contain required imports
    assert!(rendered.contains("use dampen_core::{AppState, HandlerRegistry}"));
    assert!(rendered.contains("use dampen_macros::{UiModel, dampen_ui}"));
    
    // Must have #[dampen_ui] with correct file reference
    assert!(rendered.contains(r#"#[dampen_ui("settings.dampen")]"#));
    
    // Must have Model struct with derives
    assert!(rendered.contains("pub struct Model"));
    assert!(rendered.contains("#[derive(Default, UiModel, Serialize, Deserialize, Clone, Debug)]"));
    
    // Must have create functions
    assert!(rendered.contains("pub fn create_app_state() -> AppState<Model>"));
    assert!(rendered.contains("pub fn create_handler_registry() -> HandlerRegistry"));
    
    // Must have example handler
    assert!(rendered.contains("registry.register_simple"));
}

#[test]
fn test_rust_template_placeholder_replacement() {
    let window_name = WindowName::new("UserProfile").unwrap();
    let template = WindowTemplate::load(TemplateKind::RustModule);
    let rendered = template.render(&window_name);
    
    // snake_case in file references
    assert!(rendered.contains(r#"#[dampen_ui("user_profile.dampen")]"#));
    assert!(rendered.contains("// Auto-loaded UI module for user_profile"));
    
    // No unreplaced placeholders
    assert!(!rendered.contains("{{"));
    assert!(!rendered.contains("}}"));
}

#[test]
fn test_rust_template_compiles() {
    // Integration test: write generated file to temp dir and compile
    let temp_dir = tempfile::tempdir().unwrap();
    let window_name = WindowName::new("test_window").unwrap();
    
    // Generate files
    generate_window_files(&temp_dir.path(), &window_name).unwrap();
    
    // Should compile without errors
    let output = std::process::Command::new("rustc")
        .arg("--crate-type=lib")
        .arg(temp_dir.path().join("test_window.rs"))
        .output()
        .unwrap();
    
    assert!(output.status.success(), "Generated Rust file failed to compile");
}
```

### Change History

| Version | Date | Changes |
|---------|------|---------|
| 1.0 | 2026-01-13 | Initial template contract |

### Related Contracts

- [window.dampen.template](./window_dampen_template.md) - XML UI template
- [data-model.md](../data-model.md) - WindowName structure
