# Data Model: XML Schema Version Parsing and Validation

**Feature**: 001-schema-version-validation  
**Date**: 2026-01-11

## Entities

### SchemaVersion (Existing - No Changes)

Represents a schema version with major and minor components.

**Location**: `crates/dampen-core/src/ir/mod.rs`

**Fields**:
| Field | Type | Description |
|-------|------|-------------|
| major | u16 | Major version number (breaking changes) |
| minor | u16 | Minor version number (backward-compatible additions) |

**Traits**: Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default

**Default Value**: `{ major: 1, minor: 0 }`

**Validation Rules**:
- Both fields are non-negative (enforced by u16 type)
- No maximum value constraint (validated by `validate_version_supported`)

### DampenDocument (Existing - No Changes)

The root parsed structure containing version and widget tree.

**Location**: `crates/dampen-core/src/ir/mod.rs`

**Relevant Field**:
| Field | Type | Description |
|-------|------|-------------|
| version | SchemaVersion | Schema version of the document |

**Usage**: Version is populated during parsing from the `version` attribute or defaults to 1.0.

### ParseErrorKind (Extended)

Enumeration of parse error types.

**Location**: `crates/dampen-core/src/parser/error.rs`

**New Variant**:
| Variant | Description |
|---------|-------------|
| UnsupportedVersion | Schema version is newer than framework supports |

**Existing Relevant Variants**:
| Variant | Description |
|---------|-------------|
| InvalidValue | Invalid attribute value (used for malformed version strings) |

## Constants

### MAX_SUPPORTED_VERSION

Maximum schema version supported by this framework release.

**Location**: `crates/dampen-core/src/parser/mod.rs`

**Value**: `SchemaVersion { major: 1, minor: 0 }`

**Usage**: Compared against parsed version to reject future versions.

**Update Policy**: Increment when framework adds support for new version.

## Functions

### parse_version_string

Parses a version string into a SchemaVersion struct.

**Location**: `crates/dampen-core/src/parser/mod.rs`

**Signature**:
```
fn parse_version_string(version_str: &str, span: Span) -> Result<SchemaVersion, ParseError>
```

**Inputs**:
| Parameter | Type | Description |
|-----------|------|-------------|
| version_str | &str | Raw version string from XML attribute |
| span | Span | Source location for error reporting |

**Outputs**:
| Result | Description |
|--------|-------------|
| Ok(SchemaVersion) | Successfully parsed version |
| Err(ParseError) | Invalid format error with span |

**Validation Rules**:
1. Trim whitespace from input
2. Split on "." character
3. Must have exactly 2 parts
4. Each part must parse as u16
5. Reject empty string

### validate_version_supported

Validates that a parsed version is supported by this framework.

**Location**: `crates/dampen-core/src/parser/mod.rs`

**Signature**:
```
fn validate_version_supported(version: &SchemaVersion, span: Span) -> Result<(), ParseError>
```

**Inputs**:
| Parameter | Type | Description |
|-----------|------|-------------|
| version | &SchemaVersion | Parsed version to validate |
| span | Span | Source location for error reporting |

**Outputs**:
| Result | Description |
|--------|-------------|
| Ok(()) | Version is supported |
| Err(ParseError) | Unsupported version error with suggestion |

**Validation Rule**: `(version.major, version.minor) <= (MAX.major, MAX.minor)`

### WidgetKind::minimum_version (New Method)

Returns the minimum schema version required for a widget type.

**Location**: `crates/dampen-core/src/ir/node.rs`

**Signature**:
```
impl WidgetKind {
    pub fn minimum_version(&self) -> SchemaVersion
}
```

**Current Behavior**: All widgets return `SchemaVersion { major: 1, minor: 0 }`

**Future Behavior (v1.1+)**: Match on widget kind to return appropriate minimum version.

## State Transitions

### Version Parsing Flow

```
XML Input                    Parse Phase                  Validation Phase
─────────────────────────────────────────────────────────────────────────────
<dampen version="1.0">  ──>  parse_version_string()  ──>  validate_version_supported()
       │                            │                            │
       │                            ▼                            ▼
       │                     SchemaVersion              Check against MAX
       │                     { major: 1,               { major: 1, minor: 0 }
       │                       minor: 0 }                        │
       │                            │                            ▼
       │                            └──────────────────>  DampenDocument.version
       │
<dampen>  ─────────────>  Default: { major: 1, minor: 0 }  ──────┘
(no version attr)
```

### Error Handling Flow

```
Input                           Error Condition              Result
─────────────────────────────────────────────────────────────────────────
version="1"              ──>    parts.len() != 2     ──>    InvalidValue
version="v1.0"           ──>    parse::<u16> fails   ──>    InvalidValue
version=""               ──>    empty after trim     ──>    InvalidValue
version="2.0"            ──>    > MAX_SUPPORTED      ──>    UnsupportedVersion
```

## Relationships

```
┌─────────────────────┐
│   DampenDocument    │
├─────────────────────┤
│ version: Schema...  │◄───── Populated by parser
│ root: WidgetNode    │
│ themes: HashMap     │
│ style_classes: ...  │
└─────────────────────┘
          │
          │ contains
          ▼
┌─────────────────────┐
│    SchemaVersion    │
├─────────────────────┤
│ major: u16          │
│ minor: u16          │
└─────────────────────┘
          │
          │ compared against
          ▼
┌─────────────────────┐
│ MAX_SUPPORTED_...   │
│ (constant)          │
└─────────────────────┘
```

## Compatibility Notes

- SchemaVersion struct is unchanged (no migration needed)
- DampenDocument struct is unchanged (version field already exists)
- Parser behavior changes are backward-compatible (default to v1.0)
- New ParseErrorKind variant is additive (no breaking change)
