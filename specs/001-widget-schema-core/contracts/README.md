# Contracts: Widget Schema Migration to Core

**Feature**: 001-widget-schema-core  
**Date**: 2026-01-23

## Overview

This feature does not expose external APIs (REST, GraphQL, etc.). The "contract" is the Rust public API in `dampen_core::schema`.

## Public API Contract

### Module: `dampen_core::schema`

**Exports**:
- `WidgetSchema` - Struct containing attribute categories
- `get_widget_schema(kind: &WidgetKind) -> WidgetSchema` - Standalone function
- `COMMON_STYLE_ATTRIBUTES` - Static slice of style attribute names
- `COMMON_LAYOUT_ATTRIBUTES` - Static slice of layout attribute names
- `COMMON_EVENTS` - Static slice of event attribute names

**Stability Guarantee**:
- `WidgetSchema` fields are public and stable
- Adding new attributes to constants is backward compatible
- Removing attributes is a breaking change (semver MAJOR)

### Extension: `impl WidgetKind`

**New Method**:
```rust
impl WidgetKind {
    pub fn schema(&self) -> WidgetSchema;
}
```

This method is added to the existing `WidgetKind` enum in `dampen_core::ir::node`.

## No External APIs

This feature is purely internal refactoring. No HTTP endpoints, no GraphQL schemas, no external protocols are affected.
