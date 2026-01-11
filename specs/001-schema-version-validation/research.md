# Research: XML Schema Version Parsing and Validation

**Feature**: 001-schema-version-validation  
**Date**: 2026-01-11

## Research Questions

### 1. Version String Parsing Strategy

**Decision**: Use simple string split on "." with u16 parsing for major and minor components.

**Rationale**:
- Version format is strictly "major.minor" per specification
- No need for complex semver libraries (no patch, prerelease, or build metadata)
- u16 provides sufficient range (0-65535) for version numbers
- Simple implementation minimizes dependencies and complexity

**Alternatives Considered**:
- `semver` crate: Overkill for major.minor format, adds dependency
- Regex parsing: More complex, slower, unnecessary for simple format
- Manual character iteration: Error-prone, less readable

**Implementation**:
```rust
fn parse_version_string(version_str: &str, span: Span) -> Result<SchemaVersion, ParseError> {
    let trimmed = version_str.trim();
    let parts: Vec<&str> = trimmed.split('.').collect();
    
    if parts.len() != 2 {
        return Err(/* format error */);
    }
    
    let major = parts[0].parse::<u16>().map_err(/* parse error */)?;
    let minor = parts[1].parse::<u16>().map_err(/* parse error */)?;
    
    Ok(SchemaVersion { major, minor })
}
```

### 2. Version Validation Strategy

**Decision**: Define a constant `MAX_SUPPORTED_VERSION` and compare using tuple ordering.

**Rationale**:
- Simple comparison: `(declared.major, declared.minor) <= (max.major, max.minor)`
- Single constant to update when framework version increases
- No complex version range logic needed for current requirements

**Alternatives Considered**:
- Version ranges (e.g., "1.0-1.5"): Deferred to future per spec
- Minimum version check: Not needed for v1.0 (only version)
- Feature flags per version: Premature optimization

**Implementation**:
```rust
const MAX_SUPPORTED_VERSION: SchemaVersion = SchemaVersion { major: 1, minor: 0 };

fn validate_version_supported(version: &SchemaVersion, span: Span) -> Result<(), ParseError> {
    if (version.major, version.minor) > (MAX_SUPPORTED_VERSION.major, MAX_SUPPORTED_VERSION.minor) {
        return Err(/* unsupported version error */);
    }
    Ok(())
}
```

### 3. Error Message Design

**Decision**: Follow existing ParseError pattern with specific error kind, clear message, and actionable suggestion.

**Rationale**:
- Consistency with existing error handling in parser
- Users need to know: what's wrong, where it is, how to fix it
- Span information enables IDE integration and clear error positioning

**Error Patterns**:

| Scenario | Error Kind | Message | Suggestion |
|----------|------------|---------|------------|
| Invalid format | InvalidVersion | "Invalid version format 'X'. Expected 'major.minor' (e.g., '1.0')" | "Use format: version=\"1.0\"" |
| Future version | UnsupportedVersion | "Schema version X.Y is not supported. Maximum supported: 1.0" | "Upgrade dampen-core or use version=\"1.0\"" |
| Empty string | InvalidVersion | "Version attribute cannot be empty" | "Use format: version=\"1.0\"" |

### 4. Backward Compatibility Strategy

**Decision**: Default to v1.0 when version attribute is absent; no warnings in this implementation.

**Rationale**:
- Existing files without version must continue to work (SC-004)
- Warnings planned for v1.1 per specification
- Silent default maintains smooth upgrade path

**Implementation**:
```rust
let version = if let Some(version_attr) = root.attribute("version") {
    parse_version_string(version_attr, span)?
} else {
    SchemaVersion { major: 1, minor: 0 }  // Default silently
};
validate_version_supported(&version, span)?;
```

### 5. Widget Version Infrastructure

**Decision**: Add `minimum_version()` method to `WidgetKind` enum, all returning v1.0 for now.

**Rationale**:
- Infrastructure ready for v1.1 when new widgets are version-gated
- No enforcement now (per spec: "infrastructure only")
- Pattern established for future development

**Implementation**:
```rust
impl WidgetKind {
    /// Returns minimum schema version required for this widget.
    /// All widgets currently require v1.0.
    /// TODO: Update in v1.1 when Grid, Tooltip, ComboBox are version-gated
    pub fn minimum_version(&self) -> SchemaVersion {
        SchemaVersion { major: 1, minor: 0 }
    }
}
```

### 6. File Update Strategy

**Decision**: Automated batch update using glob pattern matching and simple text replacement.

**Rationale**:
- 30+ files need updating - manual is error-prone
- Consistent format across all files
- Easy to verify with grep

**Pattern**:
- Find: `<dampen>` or `<dampen ` (without version)
- Replace with: `<dampen version="1.0">` or `<dampen version="1.0" `
- Preserve existing attributes (xmlns, etc.)
- Skip files that already have version attribute

## Edge Case Handling

| Edge Case | Handling | Test Priority |
|-----------|----------|---------------|
| Whitespace: `" 1.0 "` | Trim before parsing | High |
| Leading zeros: `"01.00"` | Parse as 1.0 (u16 handles) | Medium |
| Negative: `"-1.0"` | Reject (u16 parse fails) | High |
| Non-numeric: `"1.x"` | Reject with format error | High |
| Patch version: `"1.0.5"` | Reject (parts.len() != 2) | High |
| Empty: `""` | Reject with specific error | High |
| Very large: `"999.999"` | Parse OK, validate rejects if unsupported | Low |

## Dependencies

No new dependencies required. All functionality uses:
- `roxmltree` (existing): XML attribute access
- `thiserror` (existing): Error derive macro
- Standard library: String parsing, comparison

## Performance Considerations

- Version parsing: O(1) string split + 2 integer parses = negligible
- Version validation: Single tuple comparison = negligible
- No impact on XML parse performance budget (<10ms for 1000 widgets)

## Risks and Mitigations

| Risk | Probability | Impact | Mitigation |
|------|-------------|--------|------------|
| Missed files in update | Medium | Low | Use glob, verify with grep |
| Breaking existing tests | Low | Medium | Run full test suite after each change |
| Inconsistent error messages | Low | Low | Use consistent error pattern template |

## Conclusion

All research questions resolved. No NEEDS CLARIFICATION items remain. Ready for Phase 1 design.
