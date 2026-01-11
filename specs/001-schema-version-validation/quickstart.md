# Quickstart: XML Schema Version Validation

**Feature**: 001-schema-version-validation  
**Date**: 2026-01-11

## Overview

This guide explains how to use schema versioning in Dampen XML files and how the framework validates version compatibility.

## Declaring a Version

Add the `version` attribute to your `<dampen>` root element:

```xml
<?xml version="1.0" encoding="UTF-8"?>
<dampen version="1.0">
    <column padding="20">
        <text value="Hello, Dampen!" />
    </column>
</dampen>
```

### Version Format

- **Format**: `major.minor` (e.g., "1.0", "1.1", "2.0")
- **Current Supported**: 1.0
- **Default**: If omitted, version 1.0 is assumed

## Validation Behavior

### Supported Versions

Files declaring version 1.0 or earlier are accepted:

```xml
<!-- Accepted -->
<dampen version="1.0">
    ...
</dampen>
```

### Unsupported Future Versions

Files declaring versions newer than supported are rejected:

```xml
<!-- Rejected with error -->
<dampen version="2.0">
    ...
</dampen>
```

**Error message**:
```
Error: Schema version 2.0 is not supported. Maximum supported version: 1.0
Suggestion: Upgrade dampen-core to support v2.0, or use version="1.0"
```

### Invalid Version Formats

Malformed version strings are rejected:

```xml
<!-- Rejected: missing minor version -->
<dampen version="1">
    ...
</dampen>

<!-- Rejected: invalid prefix -->
<dampen version="v1.0">
    ...
</dampen>

<!-- Rejected: patch version not supported -->
<dampen version="1.0.5">
    ...
</dampen>
```

**Error message**:
```
Error: Invalid version format '1'. Expected 'major.minor' (e.g., '1.0')
Suggestion: Use format: version="1.0"
```

## Backward Compatibility

Files without a version attribute continue to work:

```xml
<!-- Accepted, defaults to version 1.0 -->
<dampen>
    <column>
        <text value="Still works!" />
    </column>
</dampen>
```

**Note**: While omitting the version is currently allowed, explicit versioning is recommended for clarity and future compatibility.

## Validating Your Files

Use the `dampen check` command to validate all `.dampen` files:

```bash
# Validate all files in current project
dampen check

# Validate a specific file
dampen check src/ui/window.dampen
```

## Creating New Projects

New projects created with `dampen new` automatically include version declarations:

```bash
dampen new my-app
cd my-app
cat src/ui/window.dampen
```

Output:
```xml
<?xml version="1.0" encoding="UTF-8"?>
<dampen version="1.0">
    <column padding="40" spacing="20">
        ...
    </column>
</dampen>
```

## Best Practices

1. **Always declare version explicitly**: Helps ensure compatibility and makes intent clear
2. **Use the latest supported version**: Currently 1.0
3. **Run `dampen check` before commits**: Catches version and syntax errors early
4. **Don't mix versions in a project**: All `.dampen` files should use the same version

## Future Versions

When Dampen 1.1 is released:
- Files declaring `version="1.0"` will continue to work
- New widgets may require `version="1.1"` or higher
- The framework will provide clear upgrade guidance

## Troubleshooting

### "Schema version X.Y is not supported"

Your file declares a version newer than your installed framework supports.

**Solutions**:
1. Upgrade dampen-core to the latest version
2. Or change the version attribute to a supported version (currently "1.0")

### "Invalid version format"

Your version attribute is not in the correct format.

**Solutions**:
1. Use `major.minor` format: `version="1.0"`
2. Don't use prefixes like "v" or suffixes like "-beta"
3. Don't include patch numbers like "1.0.0"

### File works but has no version

While this works today, consider adding `version="1.0"` for:
- Clarity about which schema features you're using
- Easier migration when new versions are released
- Consistency with best practices

## Version History

| Version | Status | Widgets Added |
|---------|--------|---------------|
| 1.0 | Current | All core widgets (column, row, text, button, etc.), Grid, Tooltip, ComboBox, Canvas (experimental) |
| 1.1 | Planned | To be determined |
