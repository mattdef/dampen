# Quickstart: Widget Schema Migration to Core

**Feature**: 001-widget-schema-core  
**Date**: 2026-01-23

## Overview

This guide explains how to use the new centralized widget schema in dampen-core after the migration.

## For Framework Developers

### Adding a New Attribute to a Widget

**Before (old way - two files to modify):**
1. Add attribute parsing in `dampen-core/src/parser/mod.rs`
2. Add attribute to schema in `dampen-cli/src/commands/check/attributes.rs`

**After (new way - single source of truth):**
1. Add attribute parsing in `dampen-core/src/parser/mod.rs`
2. Add attribute to schema in `dampen-core/src/schema/mod.rs` (same crate!)

### Example: Adding "tooltip" attribute to Button

```rust
// In dampen-core/src/schema/mod.rs

impl WidgetKind {
    pub fn schema(&self) -> WidgetSchema {
        match self {
            WidgetKind::Button => WidgetSchema {
                required: &[],
                optional: &["label", "enabled", "tooltip"],  // Added "tooltip"
                events: &["on_click", "on_press", "on_release"],
                style_attributes: COMMON_STYLE_ATTRIBUTES,
                layout_attributes: COMMON_LAYOUT_ATTRIBUTES,
            },
            // ...
        }
    }
}
```

That's it! The CLI will automatically recognize "tooltip" as valid.

## For Tool Developers (IDE Plugins, etc.)

### Accessing Widget Schema from dampen-core

```rust
use dampen_core::schema::{WidgetSchema, get_widget_schema, COMMON_LAYOUT_ATTRIBUTES};
use dampen_core::WidgetKind;

// Get schema for a specific widget
let button_schema = get_widget_schema(&WidgetKind::Button);

// Or use method syntax
let text_schema = WidgetKind::Text.schema();

// Check if an attribute is valid
let all_valid = button_schema.all_valid();
if all_valid.contains("on_click") {
    println!("on_click is valid for Button!");
}

// Get all valid attribute names for autocompletion
let suggestions: Vec<&str> = button_schema.all_valid_names();
```

### Available Constants

```rust
use dampen_core::schema::{
    COMMON_STYLE_ATTRIBUTES,   // Background, color, border, etc.
    COMMON_LAYOUT_ATTRIBUTES,  // Width, height, padding, align_x, etc.
    COMMON_EVENTS,             // on_click, on_change, on_input, etc.
};

// These are &'static [&'static str] arrays
for attr in COMMON_LAYOUT_ATTRIBUTES {
    println!("Layout attribute: {}", attr);
}
```

## For CLI Users

No changes! The `dampen check` command works exactly the same way:

```bash
# Validate Dampen XML files
dampen check --input src/ui/

# Verbose output
dampen check --input src/ui/ --verbose
```

Error messages remain identical:
```
[ERROR] Unknown attribute 'on_clik' for widget 'button' in src/ui/main.dampen:42:5
        Did you mean 'on_click'? (distance: 1)
```

## API Reference

### WidgetSchema struct

```rust
pub struct WidgetSchema {
    pub required: &'static [&'static str],
    pub optional: &'static [&'static str],
    pub events: &'static [&'static str],
    pub style_attributes: &'static [&'static str],
    pub layout_attributes: &'static [&'static str],
}

impl WidgetSchema {
    /// Returns all valid attributes as a HashSet for O(1) lookup
    pub fn all_valid(&self) -> HashSet<&'static str>;
    
    /// Returns all valid attributes as a Vec for iteration
    pub fn all_valid_names(&self) -> Vec<&'static str>;
}
```

### Functions

```rust
/// Get schema for a widget kind (standalone function)
pub fn get_widget_schema(kind: &WidgetKind) -> WidgetSchema;
```

### Method on WidgetKind

```rust
impl WidgetKind {
    /// Get schema for this widget kind (method syntax)
    pub fn schema(&self) -> WidgetSchema;
}
```

## Migration Checklist

For teams migrating existing tools:

- [ ] Update imports: `use dampen_core::schema::WidgetSchema` instead of local definition
- [ ] Remove local `lazy_static!` definitions if copied from CLI
- [ ] Update any `HashSet<&'static str>` storage to accept `all_valid()` return type
- [ ] Test with all widget types to ensure coverage
