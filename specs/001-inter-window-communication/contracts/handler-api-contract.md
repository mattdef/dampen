# Contract: Handler API for Shared State

**Feature**: Inter-Window Communication  
**Date**: 2026-01-14  
**Status**: Draft

---

## Overview

This contract defines the API for registering and dispatching handlers that access shared state.

---

## Handler Registration API

### Method: `register_with_shared`

**Signature**:
```rust
pub fn register_with_shared<F>(&self, name: &str, handler: F)
where
    F: Fn(&mut dyn Any, &dyn Any) + Send + Sync + 'static
```

**Parameters**:
- `name`: Handler name matching XML `on_click`, `on_change`, etc.
- `handler`: Function receiving mutable model and shared context reference

**Usage**:
```rust
registry.register_with_shared("update_theme", |model, shared| {
    let model = model.downcast_mut::<Model>().unwrap();
    let shared = shared.downcast_ref::<SharedContext<SharedState>>().unwrap();
    
    // Read from shared
    let current = shared.read().theme.clone();
    
    // Write to shared
    shared.write().theme = model.selected_theme.clone();
});
```

---

### Method: `register_with_value_and_shared`

**Signature**:
```rust
pub fn register_with_value_and_shared<F>(&self, name: &str, handler: F)
where
    F: Fn(&mut dyn Any, Box<dyn Any>, &dyn Any) + Send + Sync + 'static
```

**Usage**:
```rust
registry.register_with_value_and_shared("set_username", |model, value, shared| {
    let model = model.downcast_mut::<Model>().unwrap();
    let value = value.downcast_ref::<String>().unwrap();
    let shared = shared.downcast_ref::<SharedContext<SharedState>>().unwrap();
    
    shared.write().username = value.clone();
});
```

---

### Method: `register_with_command_and_shared`

**Signature**:
```rust
pub fn register_with_command_and_shared<F>(&self, name: &str, handler: F)
where
    F: Fn(&mut dyn Any, &dyn Any) -> Box<dyn Any> + Send + Sync + 'static
```

**Usage**:
```rust
registry.register_with_command_and_shared("fetch_user_data", |model, shared| {
    let shared = shared.downcast_ref::<SharedContext<SharedState>>().unwrap();
    let user_id = shared.read().current_user_id;
    
    // Return async command
    Box::new(Command::perform(fetch_user(user_id), Message::UserFetched))
});
```

---

## Handler Dispatch API

### Method: `dispatch_with_shared`

**Signature**:
```rust
pub fn dispatch_with_shared(
    &self,
    handler_name: &str,
    model: &mut dyn Any,
    shared: &dyn Any,
    value: Option<String>,
) -> Option<Box<dyn Any>>
```

**Behavior**:

| Handler Type | Receives Model | Receives Value | Receives Shared | Returns |
|--------------|----------------|----------------|-----------------|---------|
| `Simple` | ✓ | ✗ | ✗ | None |
| `WithValue` | ✓ | ✓ | ✗ | None |
| `WithCommand` | ✓ | ✗ | ✗ | Some(cmd) |
| `WithShared` | ✓ | ✗ | ✓ | None |
| `WithValueAndShared` | ✓ | ✓ | ✓ | None |
| `WithCommandAndShared` | ✓ | ✗ | ✓ | Some(cmd) |

**Backward Compatibility**:
- Existing handlers (`Simple`, `WithValue`, `WithCommand`) work unchanged
- `shared` parameter is ignored for non-shared handler variants
- Applications not using shared state pass `&()` as shared context

---

## Thread Safety Contract

### TS-001: Read Lock Duration

Handler code SHOULD minimize read lock duration:

```rust
// GOOD: Short lock duration
let theme = shared.read().theme.clone();
drop(shared); // Explicit release (optional, auto on scope exit)
// ... expensive processing with theme ...

// AVOID: Long lock duration
let guard = shared.read();
// ... expensive processing while holding lock ...
```

### TS-002: No Nested Locks

Handler code MUST NOT acquire nested locks:

```rust
// BAD: Deadlock risk
let read_guard = shared.read();
let write_guard = shared.write(); // DEADLOCK!

// GOOD: Sequential access
let value = shared.read().field.clone();
shared.write().other_field = value;
```

### TS-003: Panic Safety

If handler panics while holding write lock, the lock is poisoned:
- Subsequent `read()` and `write()` calls will panic
- Application should be restarted

---

## XML Integration

### Attribute Mapping

| XML Attribute | Handler Variant | Value Parameter |
|---------------|-----------------|-----------------|
| `on_click="name"` | `Simple` or `WithShared` | None |
| `on_change="name"` | `WithValue` or `WithValueAndShared` | Input value |
| `on_select="name"` | `WithValue` or `WithValueAndShared` | Selected value |
| `on_submit="name"` | `WithCommand` or `WithCommandAndShared` | None |

### Example XML with Handlers

```xml
<!-- Simple handler (no shared) -->
<button label="Cancel" on_click="close_dialog" />

<!-- Handler with shared state -->
<button label="Apply Theme" on_click="apply_theme" />

<!-- Value handler with shared -->
<input value="{model.name}" on_change="update_shared_name" />
```

```rust
// Corresponding handler registration
registry.register("close_dialog", |model| {
    model.dialog_open = false;
});

registry.register_with_shared("apply_theme", |model, shared| {
    let model = model.downcast_mut::<Model>().unwrap();
    let shared = shared.downcast_ref::<SharedContext<SharedState>>().unwrap();
    shared.write().theme = model.selected_theme.clone();
});

registry.register_with_value_and_shared("update_shared_name", |model, value, shared| {
    let value = value.downcast_ref::<String>().unwrap();
    let shared = shared.downcast_ref::<SharedContext<SharedState>>().unwrap();
    shared.write().user_name = value.clone();
});
```

---

## Contract Tests

### CT-HA-001: Simple handler still works

**Given**: Handler registered with `register("click", ...)`  
**When**: Dispatched via `dispatch_with_shared("click", model, shared, None)`  
**Then**: Handler executes, model is modified, shared is ignored

### CT-HA-002: WithShared handler receives shared context

**Given**: Handler registered with `register_with_shared("update", ...)`  
**When**: Dispatched with shared context  
**Then**: Handler can read and write shared state

### CT-HA-003: WithValueAndShared receives all parameters

**Given**: Handler registered with `register_with_value_and_shared("input", ...)`  
**When**: Dispatched with value "hello" and shared context  
**Then**: Handler receives model, "hello", and shared context

### CT-HA-004: Unknown handler returns None

**Given**: No handler registered for "unknown"  
**When**: `dispatch_with_shared("unknown", ...)`  
**Then**: Returns `None`, no panic

### CT-HA-005: Shared state changes persist across views

**Given**: View A handler modifies `shared.write().count = 42`  
**When**: View B reads `shared.read().count`  
**Then**: Value is 42

### CT-HA-006: Command handler with shared returns command

**Given**: Handler registered with `register_with_command_and_shared(...)`  
**When**: Dispatched with shared context  
**Then**: Returns `Some(command)`

---

## Error Handling

### Downcast Failures

```rust
// Handler implementation MUST handle downcast correctly
registry.register_with_shared("handler", |model, shared| {
    // Recommended: panic with clear message
    let model = model.downcast_mut::<Model>()
        .expect("Handler 'handler' received wrong model type");
    let shared = shared.downcast_ref::<SharedContext<SharedState>>()
        .expect("Handler 'handler' received wrong shared context type");
    
    // ... handler logic ...
});
```

### Missing Handler

```rust
// dispatch_with_shared returns None for unknown handlers
if registry.dispatch_with_shared("unknown", ...).is_none() {
    #[cfg(debug_assertions)]
    eprintln!("Warning: handler 'unknown' not found");
}
```

---

## Migration Guide

### Existing Handlers (No Changes Required)

```rust
// Before (still works)
registry.register("click", |model| {
    let model = model.downcast_mut::<Model>().unwrap();
    model.clicked = true;
});

// After (unchanged)
// Same code works with dispatch_with_shared
```

### Adding Shared State Access

```rust
// Before: Local state only
registry.register("set_theme", |model| {
    let model = model.downcast_mut::<Model>().unwrap();
    model.theme = "dark".to_string();
});

// After: Update shared state
registry.register_with_shared("set_theme", |model, shared| {
    let model = model.downcast_mut::<Model>().unwrap();
    let shared = shared.downcast_ref::<SharedContext<SharedState>>().unwrap();
    shared.write().theme = model.selected_theme.clone();
});
```
