# Contract: Handler Resolution Helper

**Function**: `resolve_handler_param`  
**Location**: `crates/dampen-iced/src/builder/helpers.rs`  
**Purpose**: Resolve event handler parameters from context or model bindings with comprehensive error reporting

---

## Function Signature

```rust
/// Resolves an event handler parameter expression to a concrete value.
///
/// This helper eliminates ~25-30 lines of duplicated binding resolution logic per widget
/// by providing a single function that:
/// 1. Attempts resolution from loop context (for items, indices, etc.)
/// 2. Falls back to model field access (shared application state)
/// 3. Returns detailed error with handler/widget context on failure
///
/// # Type Parameters
///
/// * `M` - Model type implementing `UiBindable` trait
///
/// # Arguments
///
/// * `builder` - DampenWidgetBuilder containing context and model state
/// * `event_param_expr` - Binding expression (e.g., "item.value", "model.count")
///
/// # Returns
///
/// `Ok(BindingValue)` if resolution succeeds, or `Err(HandlerResolutionError)` with
/// detailed diagnostic information for debugging.
///
/// # Errors
///
/// Returns `HandlerResolutionError` when:
/// - Expression not found in context (e.g., "item" outside of for loop)
/// - Field doesn't exist in model (e.g., "model.nonexistent_field")
/// - Type mismatch in binding evaluation
///
/// # Example
///
/// ```rust
/// use crate::builder::helpers::resolve_handler_param;
///
/// // In button widget builder
/// if let Some(event_param) = &on_click_event.param {
///     match resolve_handler_param(&self, event_param) {
///         Ok(value) => {
///             let handler_msg = create_handler_message("on_click", Some(value));
///             button = button.on_press(handler_msg);
///         }
///         Err(e) => {
///             eprintln!("{}", e);  // Print detailed error
///             // Continue without handler attachment
///         }
///     }
/// }
/// ```
pub fn resolve_handler_param<M>(
    builder: &DampenWidgetBuilder<M>,
    event_param_expr: &str,
) -> Result<BindingValue, HandlerResolutionError>
where
    M: UiBindable + Clone + 'static,
{
    // Attempt 1: Resolve from context (for loop items, local bindings)
    if let Some(value) = builder.resolve_from_context(event_param_expr) {
        return Ok(value);
    }

    // Attempt 2: Evaluate from model (shared application state)
    match evaluate_binding_expr_with_shared(
        event_param_expr,
        builder.model.clone(),
        builder.shared_state.clone(),
    ) {
        Ok(value) => Ok(value),
        Err(binding_error) => Err(HandlerResolutionError {
            handler_name: String::from("unknown"), // Caller should wrap this
            widget_kind: String::from("unknown"),   // Caller should wrap this
            widget_id: None,
            param_expr: event_param_expr.to_string(),
            binding_error,
            span: Span::default(), // Caller should wrap with actual span
            context_note: Some(String::from(
                "Tried context resolution first, then model field access"
            )),
        }),
    }
}
```

---

## Error Type Definition

```rust
use crate::expr::error::BindingError;
use crate::ir::span::Span;

/// Error during handler parameter resolution with rich diagnostic context.
///
/// This error wraps `BindingError` with additional information about which
/// handler and widget failed, making debugging much easier.
#[derive(Debug, Clone, PartialEq)]
pub struct HandlerResolutionError {
    /// The handler name that failed (e.g., "on_click", "on_change")
    pub handler_name: String,
    
    /// The widget type where the error occurred (e.g., "Button", "TextInput")
    pub widget_kind: String,
    
    /// Optional widget ID for disambiguation (from `id` attribute)
    pub widget_id: Option<String>,
    
    /// The parameter expression that failed to resolve
    pub param_expr: String,
    
    /// The underlying binding evaluation error
    pub binding_error: BindingError,
    
    /// Location in the XML file
    pub span: Span,
    
    /// Additional context about resolution attempts
    pub context_note: Option<String>,
}

impl std::fmt::Display for HandlerResolutionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Main error line with handler and widget context
        write!(
            f,
            "error[{}]: Handler parameter resolution failed for '{}' on {}",
            self.binding_error.kind as u8,
            self.handler_name,
            self.widget_kind
        )?;
        
        // Add widget ID if available
        if let Some(id) = &self.widget_id {
            write!(f, " (id=\"{}\")", id)?;
        }
        
        // Add location
        write!(f, " at line {}, column {}", self.span.line, self.span.column)?;
        
        // Parameter expression
        write!(f, "\n  param: {}", self.param_expr)?;
        
        // Reason from binding error
        write!(f, "\n  reason: {}", self.binding_error.message)?;
        
        // Suggestion from binding error if present
        if let Some(suggestion) = &self.binding_error.suggestion {
            write!(f, "\n  help: {}", suggestion)?;
        }
        
        // Additional context notes
        if let Some(note) = &self.context_note {
            write!(f, "\n  note: {}", note)?;
        }
        
        Ok(())
    }
}

impl std::error::Error for HandlerResolutionError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.binding_error)
    }
}

impl HandlerResolutionError {
    /// Create error from binding error with handler/widget context
    pub fn from_binding_error(
        binding_error: BindingError,
        handler_name: String,
        widget_kind: String,
        widget_id: Option<String>,
        param_expr: String,
        span: Span,
    ) -> Self {
        Self {
            handler_name,
            widget_kind,
            widget_id,
            param_expr,
            binding_error,
            span,
            context_note: None,
        }
    }
    
    /// Add a context note about resolution attempts
    pub fn with_context_note(mut self, note: String) -> Self {
        self.context_note = Some(note);
        self
    }
}
```

---

## Behavior Specification

### Input Validation

| Parameter | Validation | Behavior if Invalid |
|-----------|------------|---------------------|
| `builder` | Must be valid DampenWidgetBuilder reference | Compile error |
| `event_param_expr` | Any string | If empty, context lookup fails → model evaluation |

### Processing Flow

1. **Context Resolution** (first attempt):
   - Call `builder.resolve_from_context(event_param_expr)`
   - Check if expression exists in context stack (for loop items, local bindings)
   - If found → return `Ok(value)`

2. **Model Evaluation** (second attempt):
   - Call `evaluate_binding_expr_with_shared(expr, model, shared_state)`
   - Evaluate expression as model field access
   - If succeeds → return `Ok(value)`
   - If fails → proceed to error construction

3. **Error Construction**:
   - Wrap `BindingError` in `HandlerResolutionError`
   - Add context note: "Tried context resolution first, then model field access"
   - Return `Err(error)`

### Error Handling

Returns `Result<BindingValue, HandlerResolutionError>`:
- **Ok(value)**: Resolution succeeded (from context or model)
- **Err(error)**: Both resolution attempts failed, detailed diagnostic included

---

## Contract Guarantees

### Preconditions

- `builder` contains valid model and optional context stack
- `event_param_expr` is a valid binding expression string

### Postconditions

- If expression exists in context OR model, returns `Ok(BindingValue)`
- If both fail, returns `Err` with diagnostic information
- Never panics (all errors handled gracefully)

### Invariants

- Context resolution is attempted before model evaluation (performance optimization)
- Error messages always include handler name, widget type, and expression
- Span information is preserved for XML location tracking

---

## Test Cases

### TC1: Context-Based Resolution Succeeds

**Setup**:
```rust
// For loop: <for items="{model.items}">
//   <button on_click="delete:{item.id}"/>
// </for>
let context_stack = vec![("item", Item { id: 42 })];
let builder = builder.with_context(context_stack);
```

**Input**: `event_param_expr = "item.id"`

**Expected**: `Ok(BindingValue::Int(42))`

**Rationale**: Expression found in context, no model evaluation needed

---

### TC2: Model-Based Resolution Succeeds

**Setup**:
```rust
struct AppModel {
    counter: i32,
}
let builder = builder_with_model(AppModel { counter: 10 });
```

**Input**: `event_param_expr = "model.counter"`

**Expected**: `Ok(BindingValue::Int(10))`

**Rationale**: Context lookup fails, model evaluation succeeds

---

### TC3: Both Resolutions Fail - Unknown Field

**Setup**:
```rust
struct AppModel {
    counter: i32,
}
let builder = builder_with_model(AppModel { counter: 10 });
```

**Input**: `event_param_expr = "model.nonexistent_field"`

**Expected**:
```
Err(HandlerResolutionError {
    handler_name: "on_click",
    widget_kind: "Button",
    param_expr: "model.nonexistent_field",
    binding_error: BindingError { kind: UnknownField, message: "Field 'nonexistent_field' not found", ... },
    context_note: Some("Tried context resolution first, then model field access"),
    ...
})
```

**Error Output**:
```
error[0]: Handler parameter resolution failed for 'on_click' on Button at line 45, column 12
  param: model.nonexistent_field
  reason: Field 'nonexistent_field' not found
  help: Available fields in AppModel: counter
  note: Tried context resolution first, then model field access
```

---

### TC4: Context Item Outside For Loop

**Setup**:
```rust
// Button NOT inside a for loop
let builder = builder_without_context();
```

**Input**: `event_param_expr = "item.id"`

**Expected**:
```
Err(HandlerResolutionError {
    param_expr: "item.id",
    binding_error: BindingError { kind: UnknownField, message: "Field 'item' not found", ... },
    context_note: Some("Tried context resolution first, then model field access"),
    ...
})
```

**Error Output**:
```
error[0]: Handler parameter resolution failed for 'on_click' on Button at line 23, column 8
  param: item.id
  reason: Field 'item' not found
  help: 'item' bindings only available inside <for> loops
  note: Tried context resolution first, then model field access
```

---

### TC5: Type Mismatch

**Setup**:
```rust
struct AppModel {
    count: String,  // String, not Int
}
let builder = builder_with_model(AppModel { count: "hello".to_string() });
```

**Input**: `event_param_expr = "model.count"`  (expecting Int)

**Expected**:
```
Ok(BindingValue::String("hello"))
```

**Rationale**: Resolution succeeds, type checking happens later in handler creation

---

### TC6: Widget with ID for Disambiguation

**Setup**:
```xml
<button id="delete-btn" on_click="delete:{item.id}"/>
<button id="edit-btn" on_click="edit:{item.id}"/>
```

**Expected Error Output** (if first button fails):
```
error[0]: Handler parameter resolution failed for 'delete' on Button (id="delete-btn") at line 45, column 12
  param: item.id
  reason: Field 'item' not found
  ...
```

**Rationale**: Widget ID helps identify which button failed when multiple buttons exist

---

## Usage Pattern

### In Widget Builder (e.g., button.rs)

```rust
// Extract handler event from node
if let Some(on_click) = node.events.iter().find(|e| e.kind == EventKind::Click) {
    // Get handler name
    let handler_name = &on_click.handler;
    
    // Resolve parameter if present
    let param_value = if let Some(param_expr) = &on_click.param {
        match resolve_handler_param(self, param_expr) {
            Ok(value) => Some(value),
            Err(mut e) => {
                // Enrich error with widget context
                e.handler_name = handler_name.clone();
                e.widget_kind = "Button".to_string();
                e.widget_id = node.attributes.get("id").map(|v| v.to_string());
                e.span = node.span.clone();
                
                eprintln!("{}", e);  // Log detailed error
                None  // Continue without parameter
            }
        }
    } else {
        None
    };
    
    // Create handler message
    if let Some(registry) = &self.handler_registry {
        if let Some(handler) = registry.get(handler_name) {
            let msg = handler(param_value);
            button = button.on_press(msg);
        }
    }
}
```

---

## Migration Guide

### Before (25-30 lines per widget)

```rust
// button.rs - OLD APPROACH
if let Some(on_click) = node.events.iter().find(|e| e.kind == EventKind::Click) {
    let handler_name = &on_click.handler;
    
    let param_value = if let Some(param_expr) = &on_click.param {
        // Attempt 1: Context resolution
        if let Some(value) = self.resolve_from_context(param_expr) {
            Some(value)
        } else {
            // Attempt 2: Model evaluation
            match evaluate_binding_expr_with_shared(
                param_expr,
                self.model.clone(),
                self.shared_state.clone(),
            ) {
                Ok(value) => Some(value),
                Err(e) => {
                    eprintln!("Error evaluating handler param '{}': {}", param_expr, e);
                    None
                }
            }
        }
    } else {
        None
    };
    
    // ... handler creation logic
}
```

### After (8-12 lines per widget)

```rust
// button.rs - NEW APPROACH
use crate::builder::helpers::resolve_handler_param;

if let Some(on_click) = node.events.iter().find(|e| e.kind == EventKind::Click) {
    let handler_name = &on_click.handler;
    
    let param_value = if let Some(param_expr) = &on_click.param {
        match resolve_handler_param(self, param_expr) {
            Ok(value) => Some(value),
            Err(e) => {
                eprintln!("{}", e);  // Much better error message!
                None
            }
        }
    } else {
        None
    };
    
    // ... handler creation logic
}
```

**Lines Saved**: ~20 lines per widget × 5 widgets = **~100 lines total**

**Additional Benefit**: Error messages are now consistent and much more helpful for debugging

---

## Error Message Examples

### Good Error (with this helper)

```
error[0]: Handler parameter resolution failed for 'delete' on Button (id="delete-btn") at line 45, column 12
  param: item.id
  reason: Field 'item' not found in current context
  help: Available fields in Model: counter, items, selected_item
  note: For loop item bindings are resolved from loop context, not model
```

### Bad Error (without helper)

```
Error evaluating handler param 'item.id': Field 'item' not found
```

**Improvement**: The new error tells you:
- Which handler failed (`delete`)
- Which widget failed (`Button with id="delete-btn"`)
- Where in XML (`line 45, column 12`)
- What was attempted (`item.id`)
- Why it failed (`Field 'item' not found`)
- How to fix it (`For loop item bindings are resolved from loop context`)

---

## Dependencies

### Required Imports

```rust
use crate::binding::evaluate::evaluate_binding_expr_with_shared;
use crate::binding::value::BindingValue;
use crate::builder::DampenWidgetBuilder;
use crate::expr::error::BindingError;
use crate::ir::span::Span;
use crate::UiBindable;
```

---

## Version History

- **v0.1.0** (2026-01-21): Initial contract definition
