# Research: Widget Schema Migration to Core

**Feature**: 001-widget-schema-core  
**Date**: 2026-01-23  
**Status**: Complete

## Research Questions

### Q1: What is the best data structure for compile-time schema constants?

**Decision**: Use `&'static [&'static str]` arrays instead of `HashSet<&'static str>`

**Rationale**:
- `HashSet` requires runtime allocation via `lazy_static!` macro
- Static slices are compile-time constants with zero allocation
- Constitution mandates no new runtime allocations for schema lookups (SC-006)
- For small sets (< 50 elements), linear search is comparable to hash lookup

**Alternatives Considered**:
1. `HashSet<&'static str>` with `lazy_static!` - Rejected: requires runtime allocation
2. `phf::Set` (perfect hash function) - Rejected: adds external dependency
3. `const fn` with arrays - Chosen: zero dependencies, compile-time constant

**Implementation Note**: The `WidgetSchema` struct will store static slices, but the `all_valid()` method will return a `HashSet` for API compatibility with existing CLI code.

### Q2: Should we use `impl WidgetKind` or standalone function for schema retrieval?

**Decision**: Use both - `impl WidgetKind` method AND standalone `get_widget_schema()` function

**Rationale**:
- Method syntax (`kind.schema()`) is idiomatic Rust
- Standalone function matches existing patterns in dampen-core (e.g., `parse()`)
- Provides flexibility for different call sites

**Implementation**:
```rust
impl WidgetKind {
    pub fn schema(&self) -> WidgetSchema { ... }
}

pub fn get_widget_schema(kind: &WidgetKind) -> WidgetSchema {
    kind.schema()
}
```

### Q3: How to handle the `lazy_static!` dependency in dampen-cli?

**Decision**: Remove `lazy_static` from dampen-cli after migration

**Rationale**:
- `lazy_static!` was only used for `STYLE_COMMON`, `LAYOUT_COMMON`, `EVENTS_COMMON`
- After migration, CLI imports these from dampen-core
- dampen-core will use `const` arrays or `std::sync::LazyLock` (stable in Rust 1.80+)

**Migration Steps**:
1. Move definitions to dampen-core as `const` arrays
2. Update dampen-cli to import from dampen-core
3. Remove `lazy_static` from dampen-cli's Cargo.toml
4. Verify no other usages of `lazy_static` in dampen-cli

### Q4: How to ensure backward compatibility with existing CLI tests?

**Decision**: Preserve the `WidgetAttributeSchema` API surface in dampen-cli as a thin wrapper

**Rationale**:
- Existing tests use `WidgetAttributeSchema::for_widget()` and `all_valid()`
- Changing test code violates SC-003 ("tests pass without modification to test logic")
- Thin wrapper delegates to dampen-core while preserving interface

**Implementation**:
```rust
// In dampen-cli/src/commands/check/attributes.rs
pub struct WidgetAttributeSchema {
    inner: dampen_core::schema::WidgetSchema,
}

impl WidgetAttributeSchema {
    pub fn for_widget(kind: &WidgetKind) -> Self {
        Self { inner: kind.schema() }
    }
    
    pub fn all_valid(&self) -> HashSet<&'static str> {
        self.inner.all_valid()
    }
    // ... other methods delegate to inner
}
```

### Q5: What attributes need to be included in the schema?

**Decision**: Migrate ALL attributes currently defined in dampen-cli, including recent fixes

**Inventory from `dampen-cli/src/commands/check/attributes.rs`**:

**STYLE_COMMON** (14 attributes):
- background, color, border_color, border_width, border_radius, border_style
- shadow, opacity, transform, style, text_color, shadow_color, shadow_offset, shadow_blur_radius

**LAYOUT_COMMON** (22 attributes):
- width, height, min_width, max_width, min_height, max_height
- padding, spacing, align_items, justify_content, align, align_x, align_y, align_self
- direction, position, top, right, bottom, left, z_index
- class, theme, theme_ref

**EVENTS_COMMON** (9 attributes):
- on_click, on_press, on_release, on_change, on_input, on_submit, on_select, on_toggle, on_scroll

**Widget-specific attributes**: See data-model.md for complete per-widget breakdown

## Dependencies Analysis

### dampen-core additions:
- No new external dependencies
- Uses `std::sync::LazyLock` if needed (stable since Rust 1.80, MSRV 1.85 satisfies this)

### dampen-cli changes:
- Remove: `lazy_static` dependency
- Add: None (already depends on dampen-core)

## Risk Assessment

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| API compatibility break | Low | Medium | Wrapper preserves interface |
| Performance regression | Low | Low | Static slices are faster than HashSet |
| Missing attributes | Medium | High | Comprehensive test coverage |
| Build time increase | Low | Low | No new dependencies |

## Conclusion

All research questions resolved. No blockers identified. Ready for Phase 1 design.
