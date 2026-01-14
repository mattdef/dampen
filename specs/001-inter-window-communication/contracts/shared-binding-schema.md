# Contract: Shared Binding Schema

**Feature**: Inter-Window Communication  
**Date**: 2026-01-14  
**Status**: Draft

---

## Overview

This contract defines the XML binding syntax for accessing shared state in Dampen UI definitions.

---

## Binding Syntax

### Shared Field Access

**Pattern**: `{shared.<path>}`

**Grammar**:
```
shared_binding := "{shared." path "}"
path           := identifier ("." identifier)*
identifier     := [a-zA-Z_][a-zA-Z0-9_]*
```

**Examples**:

```xml
<!-- Simple field -->
<text value="{shared.theme}" />

<!-- Nested field -->
<text value="{shared.user.name}" />

<!-- Deeply nested -->
<text value="{shared.user.profile.avatar_url}" />
```

---

## Attribute Support

Shared bindings are supported in all attributes that accept bindings:

| Attribute | Type | Example |
|-----------|------|---------|
| `value` | String | `value="{shared.message}"` |
| `visible` | Boolean | `visible="{shared.is_logged_in}"` |
| `enabled` | Boolean | `enabled="{shared.can_edit}"` |
| `selected` | String | `selected="{shared.current_tab}"` |
| `label` | String | `label="Welcome, {shared.user.name}"` |

---

## Mixed Bindings

Shared and local bindings can be combined in the same attribute:

```xml
<!-- String interpolation with both -->
<text value="{model.greeting}, {shared.user.name}!" />

<!-- Conditional using shared -->
<button 
    label="{model.button_text}"
    visible="{shared.is_feature_enabled}"
/>
```

---

## Evaluation Rules

### Rule E-001: Read-Only Access

Shared bindings are **read-only** in XML. State can only be modified through handlers.

```xml
<!-- VALID: reading shared state -->
<text value="{shared.count}" />

<!-- NOT SUPPORTED: writing shared state (use handler) -->
<!-- <input bind="{shared.name}" /> -->
```

### Rule E-002: Missing Field Behavior

| Environment | Behavior |
|-------------|----------|
| Production | Returns empty string `""` |
| Development | Returns `""` + logs warning to console |

```xml
<!-- If shared.nonexistent doesn't exist -->
<text value="{shared.nonexistent}" />
<!-- Renders: "" (empty) -->
```

### Rule E-003: Type Coercion

Shared values are coerced to strings for display:

| Rust Type | String Representation |
|-----------|-----------------------|
| `String` | As-is |
| `&str` | As-is |
| `i32`, `i64`, etc. | Decimal format |
| `f32`, `f64` | Decimal format |
| `bool` | `"true"` or `"false"` |
| `Option<T>` | `""` if None, `T.to_string()` if Some |
| `Vec<T>` | `"[...]"` debug format |

### Rule E-004: Evaluation Timing

Bindings are evaluated:
1. On initial view render
2. On any state change (model or shared)
3. After handler execution completes

---

## Validation

### Compile-Time (Codegen Mode)

```rust
// Generated code validates field existence
let value = self.shared_context.read().user.name.clone();
//                                      ^^^^  ^^^^
// Compile error if 'user' or 'name' don't exist on SharedState
```

### Runtime (Interpreted Mode)

```rust
// Runtime binding resolution with graceful fallback
fn evaluate_shared_binding(&self, path: &[&str]) -> BindingValue {
    match self.shared_context.as_ref() {
        Some(ctx) => ctx.read().get_field(path).unwrap_or(BindingValue::Empty),
        None => {
            #[cfg(debug_assertions)]
            eprintln!("Warning: shared binding {path:?} but no shared context configured");
            BindingValue::Empty
        }
    }
}
```

---

## Contract Tests

### CT-SB-001: Simple shared binding renders value

**Given**: Shared state `{ theme: "dark" }`  
**When**: XML contains `<text value="{shared.theme}" />`  
**Then**: Widget displays "dark"

### CT-SB-002: Nested shared binding resolves correctly

**Given**: Shared state `{ user: { name: "Alice" } }`  
**When**: XML contains `<text value="{shared.user.name}" />`  
**Then**: Widget displays "Alice"

### CT-SB-003: Missing field returns empty string

**Given**: Shared state `{ }`  
**When**: XML contains `<text value="{shared.nonexistent}" />`  
**Then**: Widget displays ""

### CT-SB-004: Mixed bindings work together

**Given**: Model `{ greeting: "Hello" }`, Shared `{ name: "Bob" }`  
**When**: XML contains `<text value="{model.greeting}, {shared.name}" />`  
**Then**: Widget displays "Hello, Bob"

### CT-SB-005: Boolean binding controls visibility

**Given**: Shared state `{ is_visible: true }`  
**When**: XML contains `<button visible="{shared.is_visible}" />`  
**Then**: Button is visible

### CT-SB-006: Shared binding updates on state change

**Given**: Shared state `{ count: 0 }`, displayed via `{shared.count}`  
**When**: Handler modifies `shared.write().count = 42`  
**Then**: Widget updates to display "42"

### CT-SB-007: No shared context configured

**Given**: Application without `shared_model` attribute  
**When**: XML contains `{shared.anything}`  
**Then**: Binding evaluates to "" without error

---

## Parity Requirements

Shared bindings MUST produce identical output in:
- Interpreted mode (runtime XML parsing)
- Codegen mode (compile-time code generation)

Test: Run same application with same state in both modes, compare rendered output.
