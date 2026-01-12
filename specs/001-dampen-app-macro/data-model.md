# Data Model: Auto-Discovery Multi-View Application with #[dampen_app] Macro

**Feature Branch**: `001-dampen-app-macro`  
**Date**: 2026-01-12  
**Status**: Complete

## Overview

This document defines the internal data structures used by the `#[dampen_app]` procedural macro during code generation. These structures represent the intermediate state between file discovery and code emission.

**Key Entities**:
1. **ViewInfo**: Represents a discovered `.dampen` file with all metadata needed for code generation
2. **MacroAttributes**: Parsed attributes from `#[dampen_app(...)]` annotation

---

## Entity Definitions

### 1. ViewInfo Struct

Represents a single discovered view during the file scanning phase.

#### Purpose

Encapsulates all information about a discovered `.dampen` file needed to generate:
- `CurrentView` enum variants
- App struct fields (`{view_name}_state: AppState<Model>`)
- Module paths for imports
- View switching logic
- Hot-reload subscriptions

#### Fields

```rust
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ViewInfo {
    /// Snake_case identifier derived from filename
    /// Example: "text_input" for "text_input.dampen"
    pub view_name: String,
    
    /// PascalCase enum variant name
    /// Example: "TextInput" for "text_input.dampen"
    pub variant_name: String,
    
    /// Struct field name for AppState instance
    /// Example: "text_input_state" for "text_input.dampen"
    pub field_name: String,
    
    /// Rust module path from ui_dir root to the view
    /// Example: "ui::widgets::text_input" for "src/ui/widgets/text_input.dampen"
    pub module_path: String,
    
    /// Absolute path to the .dampen file
    /// Example: "/path/to/project/src/ui/widgets/text_input.dampen"
    pub dampen_file: PathBuf,
    
    /// Absolute path to the corresponding .rs file
    /// Example: "/path/to/project/src/ui/widgets/text_input.rs"
    pub rs_file: PathBuf,
}
```

#### Field Derivation Rules

| Field | Derivation | Example Input | Example Output |
|-------|------------|---------------|----------------|
| `view_name` | Filename without extension, as-is (must be snake_case) | `text_input.dampen` | `"text_input"` |
| `variant_name` | Snake_case to PascalCase conversion | `text_input` | `"TextInput"` |
| `field_name` | `view_name` + `"_state"` suffix | `text_input` | `"text_input_state"` |
| `module_path` | Relative path from `ui_dir` with `::` separators | `src/ui/widgets/text_input.dampen` (ui_dir=`src/ui`) | `"ui::widgets::text_input"` |
| `dampen_file` | Discovered file path (absolute) | `.../text_input.dampen` | `PathBuf("/abs/path/...")` |
| `rs_file` | Replace `.dampen` extension with `.rs` | `text_input.dampen` | `PathBuf(".../text_input.rs")` |

**Snake_case to PascalCase Conversion**:
```rust
fn to_pascal_case(snake: &str) -> String {
    snake
        .split('_')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().chain(chars).collect(),
                None => String::new(),
            }
        })
        .collect()
}
// Example: "text_input" → "TextInput"
// Example: "main_window" → "MainWindow"
```

#### Validation Rules

**VR-001: Valid Rust Identifier**
- `view_name` MUST be a valid Rust identifier (alphanumeric + underscores, starts with letter or underscore)
- Regex: `^[a-zA-Z_][a-zA-Z0-9_]*$`
- **Violation**: Compile error with message:
  ```
  error: Invalid view name '{name}' in {file_path}
         help: View names must be valid Rust identifiers (alphanumeric and underscores only)
  ```

**VR-002: Unique Variant Names**
- `variant_name` MUST be unique across all discovered views
- Check: No two `ViewInfo` instances have the same `variant_name`
- **Violation**: Compile error with message:
  ```
  error: View naming conflict: '{variant_name}' found in multiple locations:
         - {file_path_1}
         - {file_path_2}
         help: Rename one of the files or exclude one via the 'exclude' attribute
  ```

**VR-003: Corresponding .rs File Exists**
- `rs_file` path MUST exist in the file system (enforces FR-002)
- Check: `rs_file.exists() && rs_file.is_file()`
- **Violation**: Compile error with message:
  ```
  error: No matching Rust module found for '{dampen_file}'
         help: Create a file at '{rs_file}' with a Model struct, or exclude this view
  ```

**VR-004: Valid Module Path**
- `module_path` MUST not conflict with Rust keywords
- Check: Each segment is not a keyword (`mod`, `use`, `self`, `super`, `crate`, etc.)
- **Violation**: Compile error with message:
  ```
  error: Module path '{module_path}' contains reserved keyword
         help: Rename the directory or file to avoid Rust keywords
  ```

#### Relationships

- **One ViewInfo per discovered .dampen file**: Each `.dampen` file in the `ui_dir` tree generates exactly one `ViewInfo` instance
- **Collection used in code generation**: A `Vec<ViewInfo>` is passed to the code generator to produce:
  - `CurrentView` enum with variants from `variant_name` fields
  - App struct fields from `field_name` and `module_path` fields
  - `switch_to_*` handler logic using `view_name` fields
  - Hot-reload subscriptions using `dampen_file` paths

#### Example Instances

**Example 1: Top-level view**
```rust
ViewInfo {
    view_name: "window".to_string(),
    variant_name: "Window".to_string(),
    field_name: "window_state".to_string(),
    module_path: "ui::window".to_string(),
    dampen_file: PathBuf::from("/project/src/ui/window.dampen"),
    rs_file: PathBuf::from("/project/src/ui/window.rs"),
}
```

**Example 2: Nested view**
```rust
ViewInfo {
    view_name: "button".to_string(),
    variant_name: "Button".to_string(),
    field_name: "button_state".to_string(),
    module_path: "ui::widgets::button".to_string(),
    dampen_file: PathBuf::from("/project/src/ui/widgets/button.dampen"),
    rs_file: PathBuf::from("/project/src/ui/widgets/button.rs"),
}
```

**Example 3: Multi-word view**
```rust
ViewInfo {
    view_name: "text_input".to_string(),
    variant_name: "TextInput".to_string(),
    field_name: "text_input_state".to_string(),
    module_path: "ui::text_input".to_string(),
    dampen_file: PathBuf::from("/project/src/ui/text_input.dampen"),
    rs_file: PathBuf::from("/project/src/ui/text_input.rs"),
}
```

#### Implementation Notes

**Sorting**: `ViewInfo` implements `Ord` to enable deterministic ordering (FR-016). Views are sorted by `view_name` alphabetically.

**Display**: Implement `Display` for debugging:
```rust
impl Display for ViewInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({}) -> {}", 
            self.view_name, 
            self.variant_name, 
            self.dampen_file.display()
        )
    }
}
```

---

### 2. MacroAttributes Struct

Represents parsed attributes from the `#[dampen_app(...)]` annotation.

#### Purpose

Encapsulates user configuration provided to the macro, controlling:
- Where to scan for `.dampen` files
- What message types to use for integration
- Optional hot-reload and error handling
- Exclusion patterns for selective view discovery

#### Fields

```rust
#[derive(Debug, Clone)]
pub struct MacroAttributes {
    /// Required: Directory to scan for .dampen files (relative to crate root)
    /// Example: "src/ui"
    pub ui_dir: String,
    
    /// Required: Name of the user's Message enum
    /// Example: "Message"
    pub message_type: Ident,
    
    /// Required: Message variant for HandlerMessage dispatch
    /// Example: "Handler" (becomes Message::Handler(HandlerMessage))
    pub handler_variant: Ident,
    
    /// Optional: Message variant for hot-reload file events
    /// Example: Some("HotReload") (becomes Message::HotReload(FileEvent))
    pub hot_reload_variant: Option<Ident>,
    
    /// Optional: Message variant for error overlay dismissal
    /// Example: Some("DismissError") (becomes Message::DismissError)
    pub dismiss_error_variant: Option<Ident>,
    
    /// Optional: Glob patterns to exclude from discovery
    /// Example: vec!["debug_view".to_string(), "experimental/*".to_string()]
    pub exclude: Vec<String>,
}
```

#### Attribute Syntax

```rust
#[dampen_app(
    ui_dir = "src/ui",                    // Required: String literal
    message_type = "Message",             // Required: Identifier
    handler_variant = "Handler",          // Required: Identifier
    hot_reload_variant = "HotReload",     // Optional: Identifier
    dismiss_error_variant = "DismissError", // Optional: Identifier
    exclude = ["debug_view", "experimental/*"] // Optional: Array of string literals
)]
```

#### Parsing Strategy

Use `syn::parse_nested_meta()` (modern syn 2.x API) with clear error messages:

```rust
use syn::{Ident, LitStr, parse::Parser};
use syn::punctuated::Punctuated;

impl MacroAttributes {
    pub fn parse(attr: &syn::Attribute) -> syn::Result<Self> {
        let mut ui_dir = None;
        let mut message_type = None;
        let mut handler_variant = None;
        let mut hot_reload_variant = None;
        let mut dismiss_error_variant = None;
        let mut exclude = Vec::new();

        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("ui_dir") {
                let value: LitStr = meta.value()?.parse()?;
                ui_dir = Some(value.value());
            } else if meta.path.is_ident("message_type") {
                let value: Ident = meta.value()?.parse()?;
                message_type = Some(value);
            } else if meta.path.is_ident("handler_variant") {
                let value: Ident = meta.value()?.parse()?;
                handler_variant = Some(value);
            } else if meta.path.is_ident("hot_reload_variant") {
                let value: Ident = meta.value()?.parse()?;
                hot_reload_variant = Some(value);
            } else if meta.path.is_ident("dismiss_error_variant") {
                let value: Ident = meta.value()?.parse()?;
                dismiss_error_variant = Some(value);
            } else if meta.path.is_ident("exclude") {
                let content;
                syn::bracketed!(content in meta.input);
                let patterns: Punctuated<LitStr, syn::Token![,]> = 
                    content.parse_terminated(LitStr::parse, syn::Token![,])?;
                exclude = patterns.iter().map(|lit| lit.value()).collect();
            } else {
                return Err(meta.error("Unsupported attribute"));
            }
            Ok(())
        })?;

        // Validate required attributes
        let ui_dir = ui_dir.ok_or_else(|| {
            syn::Error::new(
                attr.span(),
                "missing required attribute 'ui_dir'\n\
                 help: Add ui_dir = \"src/ui\" to the macro attributes"
            )
        })?;

        let message_type = message_type.ok_or_else(|| {
            syn::Error::new(
                attr.span(),
                "missing required attribute 'message_type'\n\
                 help: Add message_type = \"Message\" to the macro attributes"
            )
        })?;

        let handler_variant = handler_variant.ok_or_else(|| {
            syn::Error::new(
                attr.span(),
                "missing required attribute 'handler_variant'\n\
                 help: Add handler_variant = \"Handler\" to the macro attributes"
            )
        })?;

        Ok(MacroAttributes {
            ui_dir,
            message_type,
            handler_variant,
            hot_reload_variant,
            dismiss_error_variant,
            exclude,
        })
    }
}
```

#### Validation Rules

**VAR-001: UI Directory Exists**
- `ui_dir` path MUST exist relative to `CARGO_MANIFEST_DIR`
- Check: Resolve path and call `Path::exists()` and `Path::is_dir()`
- **Violation**: Compile error (FR-013):
  ```
  error: UI directory not found: 'src/nonexistent'
         help: Ensure the directory exists relative to Cargo.toml
  ```

**VAR-002: Valid Identifiers**
- `message_type`, `handler_variant`, `hot_reload_variant`, `dismiss_error_variant` MUST be valid Rust identifiers
- **Enforced by**: `syn::Ident` parsing (automatic validation)
- **Violation**: Compile error from syn:
  ```
  error: expected identifier, found keyword `mod`
  ```

**VAR-003: Valid Glob Patterns**
- Each string in `exclude` MUST be a valid glob pattern
- Check: `glob::Pattern::new(&pattern).is_ok()`
- **Violation**: Compile error:
  ```
  error: Invalid glob pattern in exclude: '{pattern}'
         help: Ensure patterns use valid glob syntax (*, ?, [abc], etc.)
  ```

**VAR-004: No Duplicate Attributes**
- Each attribute key can only appear once
- **Enforced by**: Parsing logic (overwrites previous value)
- **Best practice**: Emit warning if duplicate detected (optional enhancement)

#### Relationships

- **One MacroAttributes per macro invocation**: Each `#[dampen_app]` annotation produces one `MacroAttributes` instance
- **Used to configure discovery**: The `ui_dir` and `exclude` fields control which `.dampen` files are discovered
- **Used in code generation**: The message type fields control generated method signatures and subscriptions

#### Example Instances

**Example 1: Minimal configuration**
```rust
MacroAttributes {
    ui_dir: "src/ui".to_string(),
    message_type: Ident::new("Message", Span::call_site()),
    handler_variant: Ident::new("Handler", Span::call_site()),
    hot_reload_variant: None,
    dismiss_error_variant: None,
    exclude: vec![],
}
```

**Example 2: Full configuration with hot-reload**
```rust
MacroAttributes {
    ui_dir: "src/ui".to_string(),
    message_type: Ident::new("Message", Span::call_site()),
    handler_variant: Ident::new("Handler", Span::call_site()),
    hot_reload_variant: Some(Ident::new("HotReload", Span::call_site())),
    dismiss_error_variant: Some(Ident::new("DismissError", Span::call_site())),
    exclude: vec![
        "debug_view".to_string(),
        "experimental/*".to_string(),
    ],
}
```

**Example 3: Production build (no hot-reload)**
```rust
MacroAttributes {
    ui_dir: "src/views".to_string(),
    message_type: Ident::new("AppMessage", Span::call_site()),
    handler_variant: Ident::new("HandleUi", Span::call_site()),
    hot_reload_variant: None,  // No hot-reload in production
    dismiss_error_variant: None,
    exclude: vec![
        "test/*".to_string(),  // Exclude test views
    ],
}
```

#### Implementation Notes

**Optional Attributes Handling**:
- When `hot_reload_variant` is `None`, do not generate `subscription()` method
- When `dismiss_error_variant` is `None`, do not generate error overlay dismissal logic
- Empty `exclude` list means discover all `.dampen` files

**Error Message Quality**:
- All validation errors MUST include file paths and suggestions (per FR-012, FR-013, FR-014)
- Use multi-line error messages with `\n` separators
- Include `help:` prefix for actionable suggestions

---

## Data Flow

### Discovery Phase

```
User Code:                  MacroAttributes          Discovery Process
#[dampen_app(...)]    →    (parse attributes)   →   scan ui_dir
struct MyApp;                                    →   filter by .dampen extension
                                                  →   filter by exclude patterns
                                                  →   validate .rs files exist
                                                  →   build ViewInfo instances
                                                  →   sort alphabetically
                                                  →   Vec<ViewInfo>
```

### Code Generation Phase

```
Vec<ViewInfo>          →    Generate CurrentView enum (from variant_name)
MacroAttributes        →    Generate app struct fields (from field_name, module_path)
                       →    Generate init() method (initialize all AppState fields)
                       →    Generate update() method (route switch_to_* handlers)
                       →    Generate view() method (render current view)
                       →    Generate subscription() method (if hot_reload_variant)
                       →    Generate dispatch_handler() method (route to views)
                       →    quote! { ... } → TokenStream
```

### Error Handling Flow

```
Invalid Attribute      →    MacroAttributes::parse() error
Missing UI Directory   →    resolve_ui_dir() error
Missing .rs File       →    ViewInfo validation error (VR-003)
Naming Conflict        →    ViewInfo validation error (VR-002)
Invalid View Name      →    ViewInfo validation error (VR-001)
                       →    syn::Error with Span
                       →    Compile error displayed to user
```

---

## Type Dependencies

### Rust Standard Library
- `std::path::PathBuf` - File paths
- `std::collections::HashMap` - Conflict detection
- `std::env` - `CARGO_MANIFEST_DIR` access

### Syn Crate
- `syn::Ident` - Rust identifiers for message types
- `syn::LitStr` - String literals for paths and patterns
- `syn::Error` - Error reporting with spans
- `syn::parse::Parser` - Attribute parsing

### Proc-Macro2 Crate
- `proc_macro2::Span` - Source location tracking
- `proc_macro2::TokenStream` - Generated code

### External Dependencies
- `walkdir::WalkDir` - Directory traversal
- `glob::Pattern` - Exclusion pattern matching

---

## Testing Strategy

### Unit Tests for ViewInfo

**Test 1: Field derivation**
```rust
#[test]
fn test_view_info_field_derivation() {
    let info = ViewInfo::from_path(
        PathBuf::from("/project/src/ui/text_input.dampen"),
        PathBuf::from("/project/src/ui")
    );
    
    assert_eq!(info.view_name, "text_input");
    assert_eq!(info.variant_name, "TextInput");
    assert_eq!(info.field_name, "text_input_state");
    assert_eq!(info.module_path, "ui::text_input");
}
```

**Test 2: Nested path handling**
```rust
#[test]
fn test_nested_view_info() {
    let info = ViewInfo::from_path(
        PathBuf::from("/project/src/ui/widgets/button/button.dampen"),
        PathBuf::from("/project/src/ui")
    );
    
    assert_eq!(info.module_path, "ui::widgets::button");
}
```

**Test 3: Invalid view name**
```rust
#[test]
fn test_invalid_view_name_rejected() {
    let result = ViewInfo::validate_name("123-invalid");
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Invalid view name"));
}
```

### Unit Tests for MacroAttributes

**Test 4: Required attributes**
```rust
#[test]
fn test_missing_required_attribute() {
    let tokens = quote! {
        #[dampen_app(ui_dir = "src/ui")]
    };
    
    let result = MacroAttributes::parse(&parse_macro_input!(tokens));
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("missing required attribute 'message_type'"));
}
```

**Test 5: Optional attributes**
```rust
#[test]
fn test_optional_attributes_omitted() {
    let tokens = quote! {
        #[dampen_app(
            ui_dir = "src/ui",
            message_type = "Message",
            handler_variant = "Handler"
        )]
    };
    
    let attrs = MacroAttributes::parse(&parse_macro_input!(tokens)).unwrap();
    assert!(attrs.hot_reload_variant.is_none());
    assert!(attrs.dismiss_error_variant.is_none());
}
```

**Test 6: Exclusion patterns**
```rust
#[test]
fn test_exclude_patterns() {
    let tokens = quote! {
        #[dampen_app(
            ui_dir = "src/ui",
            message_type = "Message",
            handler_variant = "Handler",
            exclude = ["debug_*", "test/*"]
        )]
    };
    
    let attrs = MacroAttributes::parse(&parse_macro_input!(tokens)).unwrap();
    assert_eq!(attrs.exclude, vec!["debug_*", "test/*"]);
}
```

### Integration Tests

**Test 7: End-to-end discovery**
```rust
#[test]
fn test_discovery_with_exclusions() {
    let fixtures = setup_fixture_directory(); // Creates temp dir with .dampen files
    let attrs = MacroAttributes {
        ui_dir: fixtures.path().to_str().unwrap().to_string(),
        exclude: vec!["debug_view".to_string()],
        // ... other fields
    };
    
    let views = discover_views(&attrs).unwrap();
    assert_eq!(views.len(), 2); // 3 files, 1 excluded
    assert!(!views.iter().any(|v| v.view_name == "debug_view"));
}
```

---

## Summary

This data model defines two core types for the `#[dampen_app]` macro:

1. **ViewInfo**: Represents discovered views with all metadata needed for code generation
   - Derived fields for enum variants, struct fields, module paths
   - Validation rules for Rust identifiers, uniqueness, file existence
   - Implements `Ord` for deterministic sorting

2. **MacroAttributes**: Represents user configuration from macro attributes
   - Required fields: `ui_dir`, `message_type`, `handler_variant`
   - Optional fields: hot-reload, error dismissal, exclusions
   - Validation rules for paths, identifiers, glob patterns

These types form the foundation for the discovery and code generation phases, ensuring type-safe, validated, and deterministic macro expansion.
