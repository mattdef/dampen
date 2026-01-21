# Contract: Boolean Attribute Resolver Helper

**Function**: `resolve_boolean_attribute`  
**Location**: `crates/dampen-iced/src/builder/helpers.rs`  
**Purpose**: Parse XML boolean attributes with support for multiple string formats

---

## Function Signature

```rust
/// Resolves a boolean attribute from a widget node with support for multiple formats.
///
/// This helper eliminates ~15-20 lines of duplicated boolean parsing logic per widget
/// by providing a single, well-tested function that handles:
/// - Multiple truthy formats: "true", "1", "yes", "on"
/// - Multiple falsy formats: "false", "0", "no", "off", "" (empty)
/// - Case-insensitive matching ("True", "TRUE", "tRuE" all work)
/// - Whitespace trimming ("  true  " → true)
/// - Graceful defaults for invalid values
///
/// # Arguments
///
/// * `node` - Widget XML node containing attributes
/// * `attr_name` - Name of the attribute to resolve (e.g., "disabled", "enabled")
/// * `default` - Value to return if attribute is missing or invalid
///
/// # Returns
///
/// `true` or `false` based on attribute value, or `default` if attribute doesn't exist.
///
/// # Example
///
/// ```rust
/// use crate::builder::helpers::resolve_boolean_attribute;
///
/// // Check if button is disabled
/// let is_disabled = resolve_boolean_attribute(node, "disabled", false);
/// if !is_disabled {
///     button = button.on_press(message);
/// }
///
/// // Check if checkbox is initially checked
/// let is_checked = resolve_boolean_attribute(node, "checked", false);
/// ```
pub fn resolve_boolean_attribute(
    node: &WidgetNode,
    attr_name: &str,
    default: bool,
) -> bool {
    match node.attributes.get(attr_name) {
        None => default,
        Some(AttributeValue::Static(s)) => parse_boolean_string(s, default),
        Some(AttributeValue::Binding(_)) => {
            // TODO: Evaluate binding expression (requires model context)
            // For now, fallback to default
            default
        }
    }
}

/// Internal helper: Parse a string into a boolean.
fn parse_boolean_string(s: &str, default: bool) -> bool {
    match s.trim().to_lowercase().as_str() {
        "true" | "1" | "yes" | "on" => true,
        "false" | "0" | "no" | "off" | "" => false,
        _ => default, // Unknown value → use default
    }
}
```

---

## Behavior Specification

### Input Validation

| Parameter | Validation | Behavior if Invalid |
|-----------|------------|---------------------|
| `node` | Must be valid WidgetNode | Compile error if wrong type |
| `attr_name` | Any string | Returns `default` if attribute not found |
| `default` | Any bool value | Returned for missing/invalid attributes |

### Processing Flow

1. **Attribute Lookup**:
   - Get attribute from `node.attributes` HashMap
   - If not found → return `default`

2. **Value Type Check**:
   - If `AttributeValue::Static(s)` → proceed to parsing
   - If `AttributeValue::Binding(_)` → return `default` (future: evaluate binding)

3. **String Parsing**:
   - Trim whitespace: `"  true  "` → `"true"`
   - Convert to lowercase: `"True"` → `"true"`
   - Match against known formats:
     - Truthy: `"true" | "1" | "yes" | "on"` → `true`
     - Falsy: `"false" | "0" | "no" | "off" | ""` → `false`
     - Unknown: `_` → `default`

4. **Return**: Final boolean value

### Error Handling

**No errors** - Function is designed for total safety:
- Missing attribute → `default`
- Invalid format → `default`
- Binding expression (not yet implemented) → `default`
- Never panics, never returns Result

---

## Contract Guarantees

### Preconditions

- `node` is a valid WidgetNode reference
- `attr_name` is a valid &str

### Postconditions

- Returns `true`, `false`, or `default` (never panics)
- Same input always produces same output (pure function)
- Whitespace and case variations handled consistently

### Invariants

- Truthy formats always return `true`
- Falsy formats always return `false`
- Unknown formats always return `default`
- Case-insensitive matching preserved

---

## Test Cases

### TC1: Truthy Values (Case Variations)

```rust
assert_eq!(resolve_boolean_attribute(&node_with("disabled", "true"), "disabled", false), true);
assert_eq!(resolve_boolean_attribute(&node_with("disabled", "TRUE"), "disabled", false), true);
assert_eq!(resolve_boolean_attribute(&node_with("disabled", "True"), "disabled", false), true);
assert_eq!(resolve_boolean_attribute(&node_with("disabled", "tRuE"), "disabled", false), true);
```

**Expected**: All return `true`

---

### TC2: Numeric Truthy Values

```rust
assert_eq!(resolve_boolean_attribute(&node_with("enabled", "1"), "enabled", false), true);
```

**Expected**: Returns `true`

---

### TC3: Natural Language Truthy Values

```rust
assert_eq!(resolve_boolean_attribute(&node_with("visible", "yes"), "visible", false), true);
assert_eq!(resolve_boolean_attribute(&node_with("visible", "YES"), "visible", false), true);
assert_eq!(resolve_boolean_attribute(&node_with("active", "on"), "active", false), true);
assert_eq!(resolve_boolean_attribute(&node_with("active", "ON"), "active", false), true);
```

**Expected**: All return `true`

---

### TC4: Falsy Values (Case Variations)

```rust
assert_eq!(resolve_boolean_attribute(&node_with("disabled", "false"), "disabled", true), false);
assert_eq!(resolve_boolean_attribute(&node_with("disabled", "FALSE"), "disabled", true), false);
assert_eq!(resolve_boolean_attribute(&node_with("disabled", "False"), "disabled", true), false);
```

**Expected**: All return `false`

---

### TC5: Numeric Falsy Values

```rust
assert_eq!(resolve_boolean_attribute(&node_with("enabled", "0"), "enabled", true), false);
```

**Expected**: Returns `false`

---

### TC6: Natural Language Falsy Values

```rust
assert_eq!(resolve_boolean_attribute(&node_with("visible", "no"), "visible", true), false);
assert_eq!(resolve_boolean_attribute(&node_with("visible", "NO"), "visible", true), false);
assert_eq!(resolve_boolean_attribute(&node_with("active", "off"), "active", true), false);
assert_eq!(resolve_boolean_attribute(&node_with("active", "OFF"), "active", true), false);
```

**Expected**: All return `false`

---

### TC7: Empty String

```rust
assert_eq!(resolve_boolean_attribute(&node_with("disabled", ""), "disabled", true), false);
```

**Expected**: Returns `false` (empty string is falsy in Dampen, unlike HTML5)

**Rationale**: Dampen is XML-based, not HTML. Empty string should represent "no value" rather than "attribute present".

---

### TC8: Whitespace Handling

```rust
assert_eq!(resolve_boolean_attribute(&node_with("enabled", "  true  "), "enabled", false), true);
assert_eq!(resolve_boolean_attribute(&node_with("enabled", "\ttrue\n"), "enabled", false), true);
assert_eq!(resolve_boolean_attribute(&node_with("disabled", "  "), "disabled", true), false);
```

**Expected**: 
- Leading/trailing whitespace trimmed before parsing
- Whitespace-only string is falsy

---

### TC9: Missing Attribute

```rust
let node = WidgetNode::new_without_attr("disabled");
assert_eq!(resolve_boolean_attribute(&node, "disabled", false), false);
assert_eq!(resolve_boolean_attribute(&node, "disabled", true), true);
```

**Expected**: Returns `default` value

---

### TC10: Invalid Values (Default Fallback)

```rust
assert_eq!(resolve_boolean_attribute(&node_with("disabled", "enabled"), "disabled", false), false);
assert_eq!(resolve_boolean_attribute(&node_with("disabled", "enabled"), "disabled", true), true);
assert_eq!(resolve_boolean_attribute(&node_with("count", "2"), "count", false), false);
assert_eq!(resolve_boolean_attribute(&node_with("flag", "-1"), "flag", false), false);
assert_eq!(resolve_boolean_attribute(&node_with("emoji", "👍"), "emoji", false), false);
assert_eq!(resolve_boolean_attribute(&node_with("abbrev", "t"), "abbrev", false), false);
assert_eq!(resolve_boolean_attribute(&node_with("abbrev", "y"), "abbrev", false), false);
```

**Expected**: All return `default` (invalid strings are not accepted)

---

### TC11: Case Mixing

```rust
assert_eq!(resolve_boolean_attribute(&node_with("flag", "YeS"), "flag", false), true);
assert_eq!(resolve_boolean_attribute(&node_with("flag", "oFf"), "flag", true), false);
```

**Expected**: Case-insensitive matching works

---

### TC12: Attribute Name Case Sensitivity

```rust
let node = node_with("Disabled", "true");  // Uppercase attribute name
assert_eq!(resolve_boolean_attribute(&node, "disabled", false), false);  // Lowercase lookup
```

**Expected**: Returns `false` (attribute names are case-sensitive in XML)

**Note**: Attribute name lookup is case-sensitive (standard XML behavior), but attribute VALUE parsing is case-insensitive.

---

## Supported Formats Table

| Format Category | Values | Result |
|----------------|--------|--------|
| **Boolean (standard)** | `"true"`, `"false"` | `true`, `false` |
| **Numeric** | `"1"`, `"0"` | `true`, `false` |
| **Natural language** | `"yes"`, `"no"` | `true`, `false` |
| **Toggle/switch** | `"on"`, `"off"` | `true`, `false` |
| **Empty** | `""` | `false` |
| **Case variations** | `"TRUE"`, `"False"`, `"YeS"` | Converted to lowercase first |
| **Whitespace** | `"  true  "`, `"\ttrue\n"` | Trimmed before parsing |
| **Invalid/Unknown** | `"enabled"`, `"2"`, `"-1"`, `"t"`, `"👍"` | Returns `default` |

---

## Migration Guide

### Before (15-20 lines per widget)

```rust
// button.rs - OLD APPROACH
let is_disabled = match node.attributes.get("disabled") {
    None => false,
    Some(AttributeValue::Static(s)) => match s.to_lowercase().as_str() {
        "true" | "1" | "yes" | "on" => true,
        "false" | "0" | "no" | "off" => false,
        _ => false,
    },
    Some(AttributeValue::Binding(_)) => {
        // Would need to evaluate binding
        false
    }
};

if !is_disabled {
    button = button.on_press(message);
}
```

### After (1 line per widget)

```rust
// button.rs - NEW APPROACH
use crate::builder::helpers::resolve_boolean_attribute;

let is_disabled = resolve_boolean_attribute(node, "disabled", false);
if !is_disabled {
    button = button.on_press(message);
}
```

**Lines Saved**: ~15 lines per widget × 3 widgets = **~45 lines total**

---

## Usage Examples

### Button Disabled State

```rust
let is_disabled = resolve_boolean_attribute(node, "disabled", false);
let button = if !is_disabled {
    button.on_press(MyMessage::ButtonClicked)
} else {
    button
};
```

### Checkbox Initial State

```rust
let is_checked = resolve_boolean_attribute(node, "checked", false);
let checkbox = checkbox(is_checked, move |checked| MyMessage::CheckChanged(checked));
```

### Radio Button Selected State

```rust
let is_selected = resolve_boolean_attribute(node, "selected", false);
```

### TextInput Readonly State

```rust
let is_readonly = resolve_boolean_attribute(node, "readonly", false);
// (future: apply readonly styling or disable input)
```

---

## Future Enhancements

### Binding Expression Support

Currently, `AttributeValue::Binding(_)` returns `default`. Future versions should:

```rust
Some(AttributeValue::Binding(expr)) => {
    // Evaluate binding against model
    match evaluate_binding_expr(&expr, &model) {
        Ok(BindingValue::Bool(b)) => b,
        Ok(_) => default, // Wrong type
        Err(_) => default, // Evaluation error
    }
}
```

**Requires**: Model context parameter added to function signature

---

## Dependencies

### Required Imports

```rust
use dampen_core::AttributeValue;
use dampen_core::WidgetNode;
```

### No External Dependencies

This is a pure function with zero external crate dependencies beyond `dampen_core`.

---

## Version History

- **v0.1.0** (2026-01-21): Initial contract definition
- **v0.2.0** (future): Add binding expression support
