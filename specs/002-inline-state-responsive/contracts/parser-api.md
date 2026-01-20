# Parser API Contract: State-Prefixed Attributes

**Component**: `dampen-core::parser`  
**File**: `crates/dampen-core/src/parser/mod.rs`

## Function: parse_node (Modified)

### Current Behavior

Parses XML node into `WidgetNode`, handling:
- Base attributes
- Style attributes (parsed into `StyleProperties`)
- Layout attributes (parsed into `LayoutConstraints`)
- Breakpoint-prefixed attributes (stored in `breakpoint_attributes`)

### New Behavior

Additionally detects and processes state-prefixed attributes:

```rust
// Pseudocode for attribute processing in parse_node()
for (name, value) in node.attributes() {
    // Check for breakpoint prefix (existing)
    if let Some((prefix, attr_name)) = name.split_once('-') {
        if Breakpoint::parse(prefix).is_ok() {
            breakpoint_attributes.entry(bp).or_default().insert(attr_name, value);
            continue;
        }
    }
    
    // Check for state prefix (NEW)
    if let Some((state_prefix, attr_name)) = name.split_once(':') {
        if let Some(state) = WidgetState::from_prefix(state_prefix) {
            let style_value = parse_style_attribute(attr_name, value)?;
            inline_state_variants
                .entry(state)
                .or_insert_with(StyleProperties::default)
                .apply_attribute(attr_name, style_value);
            continue;
        }
    }
    
    // Handle as regular attribute (existing)
    // ...
}
```

### Input/Output Contract

**Input**:
```xml
<button 
    label="Click"
    background="#blue"
    hover:background="#red"
    hover:border="2px solid white"
    focus:background="#green"
    active:opacity="0.8"
    disabled:opacity="0.5"
/>
```

**Output** (`WidgetNode`):
```rust
WidgetNode {
    kind: WidgetKind::Button,
    attributes: { "label": Static("Click") },
    style: Some(StyleProperties { background: Some(#blue), .. }),
    inline_state_variants: {
        Hover: StyleProperties { background: Some(#red), border: Some(...) },
        Focus: StyleProperties { background: Some(#green) },
        Active: StyleProperties { opacity: Some(0.8) },
        Disabled: StyleProperties { opacity: Some(0.5) },
    },
    // ...
}
```

### Error Cases

| Condition | Behavior |
|-----------|----------|
| Invalid state prefix (`unknown:background`) | Log warning, treat as regular attribute |
| Invalid style attribute (`hover:invalid`) | Return `ParseError` with suggestion |
| Malformed value (`hover:background="not-a-color"`) | Return `ParseError` with span |

### Error Format

```rust
ParseError {
    kind: ParseErrorKind::InvalidAttribute,
    message: "Unknown style attribute 'invalid' in state prefix",
    span: Span { line: 5, column: 12 },
    suggestion: Some("Did you mean 'background'?"),
}
```

---

## Function: WidgetState::from_prefix (New)

**Location**: `crates/dampen-core/src/ir/theme.rs`

### Signature

```rust
impl WidgetState {
    pub fn from_prefix(s: &str) -> Option<Self>;
}
```

### Contract

| Input | Output |
|-------|--------|
| `"hover"` | `Some(WidgetState::Hover)` |
| `"HOVER"` | `Some(WidgetState::Hover)` |
| `"focus"` | `Some(WidgetState::Focus)` |
| `"active"` | `Some(WidgetState::Active)` |
| `"disabled"` | `Some(WidgetState::Disabled)` |
| `"pressed"` | `None` (not a valid state) |
| `""` | `None` |
| `"hover "` | `None` (trailing space) |

### Notes

- Case-insensitive matching
- No whitespace trimming (caller's responsibility)
- Returns `None` for unknown prefixes (not an error)
