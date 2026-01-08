# Data Model: Check Validation Enhancements

**Feature**: 001-check-validation-enhancements  
**Date**: 2026-01-08  
**Status**: Design Complete

## Overview

This document defines the data structures used by the enhanced `gravity check` command for validation. All structures are designed for:
- Efficient lookup (HashSet/HashMap based)
- Human-readable JSON serialization
- Forward compatibility (optional fields use defaults)

---

## HandlerRegistry

**Purpose**: Validates that event handlers referenced in XML exist in the registered set.

### JSON Schema

```json
[
  {
    "name": "increment",
    "param_type": null,
    "returns_command": false
  },
  {
    "name": "setValue",
    "param_type": "i32",
    "returns_command": true
  }
]
```

### Field Definitions

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | Yes | Handler function name as referenced in XML `on_*` attributes |
| `param_type` | String or null | No | Expected message type; null for unit/no params |
| `returns_command` | Boolean | No | Whether handler returns Command for async operations |

### Rust Structure

```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct HandlerDefinition {
    pub name: String,
    #[serde(default)]
    pub param_type: Option<String>,
    #[serde(default)]
    pub returns_command: bool,
}

#[derive(Debug, Clone, Default)]
pub struct HandlerRegistry {
    handlers: HashSet<HandlerDefinition>,
}

impl HandlerRegistry {
    pub fn load_from_json(path: &Path) -> Result<Self, serde_json::Error> { ... }
    pub fn contains(&self, name: &str) -> bool { ... }
    pub fn all_names(&self) -> Vec<&str> { ... }
}
```

### Validation Rules

1. Handler names must be non-empty
2. Registry lookup is case-sensitive
3. Suggestions provided for names with Levenshtein distance <= 3

---

## ModelInfo

**Purpose**: Validates that binding paths reference existing fields in the data model.

### JSON Schema

```json
[
  {
    "name": "count",
    "type_name": "i32",
    "is_nested": false,
    "children": []
  },
  {
    "name": "user",
    "type_name": "User",
    "is_nested": true,
    "children": [
      {"name": "name", "type_name": "String", "is_nested": false, "children": []},
      {"name": "email", "type_name": "String", "is_nested": false, "children": []}
    ]
  }
]
```

### Field Definitions

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | Yes | Field name (last part of binding path) |
| `type_name` | String | No | Type hint for display purposes |
| `is_nested` | Boolean | No | If true, field has nested children |
| `children` | Array | No | Nested field definitions for structs |

### Rust Structure

```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ModelField {
    pub name: String,
    #[serde(default)]
    pub type_name: String,
    #[serde(default)]
    pub is_nested: bool,
    #[serde(default)]
    pub children: Vec<ModelField>,
}

#[derive(Debug, Clone, Default)]
pub struct ModelInfo {
    fields: HashSet<ModelField>,
}

impl ModelInfo {
    pub fn load_from_json(path: &Path) -> Result<Self, serde_json::Error> { ... }
    pub fn contains_field(&self, path: &[&str]) -> bool { ... }
    pub fn top_level_fields(&self) -> Vec<&str> { ... }
}
```

### Validation Rules

1. Binding paths are validated segment by segment
2. If `is_nested` is false, path cannot extend beyond that field
3. Available fields are listed in error messages

---

## WidgetAttributeSchema

**Purpose**: Defines valid attributes for each widget type and enables unknown attribute detection.

### Structure

```rust
pub struct WidgetAttributeSchema {
    pub required: HashSet<&'static str>,
    pub optional: HashSet<&'static str>,
    pub events: HashSet<&'static str>,
    pub style_attributes: HashSet<&'static str>,
    pub layout_attributes: HashSet<&'static str>,
}

impl WidgetAttributeSchema {
    pub fn for_widget(kind: &WidgetKind) -> Self { ... }
    pub fn all_valid(&self) -> HashSet<&'static str> { ... }
}
```

### Common Attribute Sets

```rust
pub const STYLE_COMMON: HashSet<&'static str> = hashset![
    "background", "color", "border_color", "border_width",
    "border_radius", "border_style", "shadow", "opacity", "transform"
];

pub const LAYOUT_COMMON: HashSet<&'static str> = hashset![
    "width", "height", "min_width", "max_width", "min_height", "max_height",
    "padding", "spacing", "align_items", "justify_content",
    "direction", "position", "top", "right", "bottom", "left", "z_index"
];

pub const EVENTS_COMMON: HashSet<&'static str> = hashset![
    "on_click", "on_press", "on_release", "on_change",
    "on_input", "on_submit", "on_select", "on_toggle", "on_scroll"
];
```

### Widget-Specific Attributes

| Widget | Required | Optional | Events |
|--------|----------|----------|--------|
| Text | value | size, weight, family | common |
| Image | src | width, height, fit | common |
| Button | - | label | on_click, on_press |
| TextInput | - | placeholder, value | on_input, on_submit, on_change |
| Checkbox | - | checked, label | on_toggle |
| Radio | label, value | selected, disabled | on_select |
| Slider | - | min, max, value, step | on_change |
| Column | - | - | common |
| Row | - | - | common |
| Container | - | - | common |
| Scrollable | - | - | on_scroll |
| Stack | - | - | common |

### Validation Rules

1. Unknown attributes trigger error with suggestion
2. Required attributes missing trigger error
3. Custom widgets use config file for allowed attributes

---

## RadioGroup

**Purpose**: Validates consistency of radio button groups (duplicate values, handler consistency).

### Structure

```rust
#[derive(Debug)]
pub struct RadioGroup {
    pub id: String,
    pub values: HashMap<String, Span>,
    pub selected_binding: Option<String>,
    pub on_select_handler: Option<String>,
}
```

### Validation Rules

1. All radio buttons in same group must have unique values
2. All radio buttons should have same `on_select` handler
3. Duplicate values trigger error with location of both occurrences

---

## ValidationError

**Purpose**: Structured error information for all validation failures.

### Error Types

| Error Type | Trigger | Message Format |
|------------|---------|----------------|
| UnknownAttribute | Attribute not in schema | "Unknown attribute '{attr}' for widget '{widget}' in {file}:{line}:{col}. Did you mean '{suggestion}'?" |
| MissingRequiredAttribute | Required attr missing | "Missing required attribute '{attr}' for widget '{widget}' in {file}:{line}:{col}" |
| UnknownHandler | Handler not in registry | "Unknown handler '{handler}' in {file}:{line}:{col}. Available handlers: {list}" |
| InvalidBindingField | Field not in model | "Invalid binding field '{field}' in {file}:{line}:{col}. Available fields: {list}" |
| DuplicateRadioValue | Same value in group | "Duplicate radio value '{value}' in group '{group}' at {file}:{line}:{col}" |
| InconsistentRadioHandlers | Different handlers | "Radio group '{group}' has inconsistent on_select handlers" |
| InvalidThemeProperty | Unknown theme prop | "Invalid theme property '{property}' in theme '{theme}': {message}" |
| ThemeCircularDependency | Cycle in theme inheritance | "Theme '{theme}' has circular dependency: {cycle}" |

### Rust Structure

```rust
#[derive(Error, Debug)]
pub enum CheckError {
    #[error("Unknown attribute '{attr}' for widget '{widget}' in {file}:{line}:{col}.{suggestion}")]
    UnknownAttribute {
        attr: String,
        widget: String,
        file: PathBuf,
        line: u32,
        col: u32,
        suggestion: String,
    },

    #[error("Missing required attribute '{attr}' for widget '{widget}' in {file}:{line}:{col}")]
    MissingRequiredAttribute {
        attr: String,
        widget: String,
        file: PathBuf,
        line: u32,
        col: u32,
    },

    #[error("Unknown handler '{handler}' in {file}:{line}:{col}.{suggestion}")]
    UnknownHandler {
        handler: String,
        file: PathBuf,
        line: u32,
        col: u32,
        suggestion: String,
    },

    #[error("Invalid binding field '{field}' in {file}:{line}:{col}. Available fields: {available}")]
    InvalidBindingField {
        field: String,
        file: PathBuf,
        line: u32,
        col: u32,
        available: String,
    },

    #[error("Duplicate radio value '{value}' in group '{group}' at {file}:{line}:{col}")]
    DuplicateRadioValue {
        value: String,
        group: String,
        file: PathBuf,
        line: u32,
        col: u32,
    },

    #[error("Radio group '{group}' has inconsistent on_select handlers")]
    InconsistentRadioHandlers {
        group: String,
        file: PathBuf,
        line: u32,
        col: u32,
    },

    // ... existing error types preserved
}
```

---

## CustomWidgetConfig

**Purpose**: Configuration for custom widget attributes.

### JSON Schema

```json
{
  "CustomWidget": {
    "allowed_attributes": ["value", "mode", "format", "custom_prop"]
  },
  "AnotherCustom": {
    "allowed_attributes": ["id", "data"]
  }
}
```

### Structure

```rust
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CustomWidgetConfig {
    pub allowed_attributes: HashSet<String>,
}

#[derive(Debug, Clone, Default)]
pub struct CustomWidgetRegistry {
    widgets: HashMap<String, CustomWidgetConfig>,
}

impl CustomWidgetRegistry {
    pub fn load_from_json(path: &Path) -> Result<Self, serde_json::Error> { ... }
    pub fn is_attribute_allowed(&self, widget: &str, attr: &str) -> bool { ... }
}
```

---

## Relationships

```
Validator
├── HandlerRegistry (optional, via --handlers)
├── ModelInfo (optional, via --model)
├── WidgetAttributeSchema (built-in)
├── CustomWidgetRegistry (optional, via config)
└── ErrorCollector
```

---

## Validation Flow

```
XML File
    ↓
Parse (gravity_core::parser)
    ↓
WidgetNode Tree
    ↓
┌─────────────────────────────────────┐
│ For each widget:                    │
│ 1. Check required attributes        │
│ 2. Check unknown attributes         │
│ 3. Check handlers (if registry)     │
│ 4. Check bindings (if model)        │
│ 5. Check custom attrs (if config)   │
└─────────────────────────────────────┘
    ↓
Cross-Widget Validation (Radio Groups)
    ↓
Theme Validation (Properties, Cycles)
    ↓
Collect All Errors
    ↓
Report with Suggestions
```
