# API Contract: Gravity Widget Builder

**Date**: 2026-01-03  
**Feature**: 003-widget-builder  
**Type**: Rust Library API

---

## Public API Surface

### 1. GravityWidgetBuilder Struct

```rust
pub struct GravityWidgetBuilder<'a, Message> {
    // Private fields
    node: &'a WidgetNode,
    model: &'a dyn UiBindable,
    handler_registry: Option<&'a HandlerRegistry<Message>>,
    verbose: bool,
}
```

**Generic Parameters**:
- `'a`: Lifetime of borrowed node and model data
- `Message`: Application message type (must implement `Clone + 'static`)

**Traits Required**:
- `Message: Clone + 'static` - For event dispatch

---

### 2. Constructor

```rust
impl<'a, Message> GravityWidgetBuilder<'a, Message> {
    /// Create a new widget builder
    ///
    /// # Arguments
    /// * `node` - Root widget node from parsed Gravity XML
    /// * `model` - Application state for binding evaluation
    /// * `handler_registry` - Optional registry for event handlers
    ///
    /// # Example
    /// ```rust
    /// let builder = GravityWidgetBuilder::new(
    ///     &document.root,
    ///     &app_state,
    ///     Some(&handler_registry)
    /// );
    /// ```
    pub fn new(
        node: &'a WidgetNode,
        model: &'a dyn UiBindable,
        handler_registry: Option<&'a HandlerRegistry<Message>>
    ) -> Self
}
```

**Parameters**:
- `node`: Reference to parsed widget tree root
- `model`: Any type implementing `UiBindable` trait
- `handler_registry`: Optional, enables event handling

**Returns**: Configured builder instance

**Errors**: None (failures occur during `build()`)

---

### 3. Configuration Method

```rust
impl<'a, Message> GravityWidgetBuilder<'a, Message> {
    /// Enable or disable verbose logging
    ///
    /// When enabled, the builder will:
    /// - Log binding evaluation results
    /// - Report missing handlers
    /// - Report unsupported widgets
    /// - Show conversion details
    ///
    /// Default: false
    ///
    /// # Example
    /// ```rust
    /// let builder = GravityWidgetBuilder::new(...)
    ///     .with_verbose(true);
    /// ```
    pub fn with_verbose(self, verbose: bool) -> Self
}
```

**Parameters**:
- `verbose`: `true` to enable debug logging, `false` to disable

**Returns**: Modified builder instance (consumes and returns self)

---

### 4. Build Method

```rust
impl<'a, Message> GravityWidgetBuilder<'a, Message> 
where
    Message: Clone + 'static
{
    /// Build the widget tree from the parsed document
    ///
    /// Recursively processes all widgets, evaluates bindings,
    /// connects events, and applies styles.
    ///
    /// # Returns
    /// An Iced Element ready for rendering
    ///
    /// # Example
    /// ```rust
    /// fn view(&self) -> Element<'_, Message> {
    ///     GravityWidgetBuilder::new(
    ///         &self.document.root,
    ///         &self.state,
    ///         Some(&self.handlers)
    ///     ).build()
    /// }
    /// ```
    ///
    /// # Performance
    /// - O(n) where n = number of widgets
    /// - Target: <50ms for 1000 widgets
    pub fn build(self) -> Element<'a, Message>
}
```

**Returns**: `iced::Element<'a, Message>` - Renderable widget tree

**Errors**: 
- Returns empty element on critical errors
- Logs errors to stderr if verbose mode enabled

---

## Required Traits

### UiBindable (from gravity-core)

```rust
pub trait UiBindable {
    /// Get a field value by path
    fn get_field(&self, path: &[&str]) -> Option<BindingValue>;
    
    /// List all available field paths
    fn available_fields() -> Vec<String>;
}
```

**Usage**: The builder calls `model.get_field(&["user", "name"])` to resolve `{user.name}` bindings.

---

### HandlerRegistry (from gravity-runtime)

```rust
pub struct HandlerRegistry<Message> {
    // Implementation detail
}

impl<Message> HandlerRegistry<Message> {
    /// Get handler by name
    pub fn get(&self, name: &str) -> Option<Handler<Message>>;
    
    /// Register a new handler
    pub fn register(&mut self, name: String, handler: Handler<Message>);
}
```

**Usage**: The builder looks up handlers by name from `on_click="handler_name"` attributes.

---

## Conversion API (Internal)

These are implementation details but documented for extensibility:

```rust
// Color conversions
impl From<&str> for iced::Color { ... }

// Length conversions  
impl From<&str> for iced::Length { ... }

// Padding conversions
impl From<&str> for iced::Padding { ... }

// Style property conversions
impl From<&StyleProperties> for ContainerStyle { ... }
impl From<&StyleProperties> for ButtonStyle { ... }

// Binding evaluation
fn evaluate_binding(expr: &str, model: &dyn UiBindable) -> Result<String, BindingError>
```

---

## Usage Patterns

### Pattern 1: Basic Usage

```rust
use gravity_iced::GravityWidgetBuilder;

fn view(state: &AppState) -> Element<'_, Message> {
    GravityWidgetBuilder::new(
        &state.document.root,
        &state.model,
        Some(&state.handlers)
    ).build()
}
```

### Pattern 2: With Verbose Logging

```rust
fn view(state: &AppState) -> Element<'_, Message> {
    GravityWidgetBuilder::new(
        &state.document.root,
        &state.model,
        Some(&state.handlers)
    )
    .with_verbose(cfg!(debug_assertions)) // Only in dev
    .build()
}
```

### Pattern 3: Static UI (No Handlers)

```rust
fn view(state: &AppState) -> Element<'_, Message> {
    GravityWidgetBuilder::new(
        &state.document.root,
        &state.model,
        None  // No event handling
    ).build()
}
```

### Pattern 4: Custom Model

```rust
#[derive(UiModel)]
struct MyState {
    count: i32,
    user: User,
}

impl UiBindable for MyState {
    fn get_field(&self, path: &[&str]) -> Option<BindingValue> {
        // Generated by #[derive(UiModel)] or manual
        match path {
            ["count"] => Some(BindingValue::Int(self.count)),
            ["user", "name"] => Some(BindingValue::String(self.user.name.clone())),
            _ => None
        }
    }
}

fn view(state: &MyState) -> Element<'_, Message> {
    GravityWidgetBuilder::new(
        &state.document.root,
        state,  // Implements UiBindable
        Some(&state.handlers)
    ).build()
}
```

---

## Error Handling Contract

### Runtime Errors (Handled Gracefully)

| Error Type | Detection | Behavior |
|------------|-----------|----------|
| Missing handler | Registry lookup fails | Event ignored, warning logged (if verbose) |
| Unsupported widget | Match fallback | Empty placeholder, error logged |
| Binding error | evaluate_binding fails | Empty string, error logged |
| Missing attribute | From conversion | Default value applied |

### Compile-Time Guarantees

- All widget types handled (exhaustive match)
- Message type must be Clone + 'static
- Model must implement UiBindable
- All conversions are infallible (IR pre-validated)

---

## Performance Contract

### Time Complexity

- **Build time**: O(n) where n = widget count
- **Per widget**: < 0.05ms average
- **Target**: 50ms for 1000 widgets

### Space Complexity

- **Memory**: O(d) where d = tree depth (recursion stack)
- **Allocations**: Minimal, only for evaluated bindings

### Benchmarks

```rust
#[bench]
fn bench_build_1000_widgets(b: &mut Bencher) {
    let doc = create_large_document(1000);
    let model = create_model();
    let registry = create_handlers();
    
    b.iter(|| {
        GravityWidgetBuilder::new(&doc.root, &model, Some(®istry))
            .build()
    });
}
```

**Target**: < 50ms per iteration

---

## Backward Compatibility

### Version 1.0.0 (Initial Release)

- All API methods are `pub`
- No `#[deprecated]` items
- No breaking changes planned

### Future Extensions

Possible additions (not in v1.0):
- `with_theme(theme: Theme)` - Custom theme support
- `with_cache(cache: Cache)` - Memoization
- `build_async()` - For async models

All additions will be non-breaking (new methods only).

---

## Migration Guide

### From Manual Rendering

**Before** (410 lines):
```rust
fn render_text(node: &WidgetNode) -> Element<'_, Message> {
    let text = match &node.text {
        Some(t) => evaluate_binding(t, &model).unwrap_or_default(),
        None => String::new(),
    };
    // ... 30 lines of style conversions
    Text::new(text).size(16).into()
}

fn render_button(node: &WidgetNode) -> Element<'_, Message> {
    // ... 40 lines of conversions
}

fn view(state: &AppState) -> Element<'_, Message> {
    Column::new()
        .push(render_text(&doc.children[0]))
        .push(render_button(&doc.children[1]))
        .into()
}
```

**After** (10 lines):
```rust
fn view(state: &AppState) -> Element<'_, Message> {
    GravityWidgetBuilder::new(
        &state.document.root,
        &state.model,
        Some(&state.handlers)
    ).build()
}
```

---

## Testing Contract

### Required Test Coverage

1. **Unit Tests** (80% coverage)
   - All From implementations
   - Binding evaluation edge cases
   - Default value logic

2. **Integration Tests** (100% coverage)
   - Full tree building
   - Event connection
   - Error paths

3. **Performance Tests**
   - 1000 widget benchmark
   - Memory allocation count

4. **Snapshot Tests**
   - Generated widget trees match expected output

### Test Entry Points

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_color_conversion() { ... }
    
    #[test]
    fn test_binding_evaluation() { ... }
    
    #[test]
    fn test_full_widget_tree() { ... }
}
```

---

## Compliance Matrix

| Requirement | API Support | Test Coverage |
|-------------|-------------|---------------|
| FR-001: Builder constructor | ✅ `new()` | Unit |
| FR-002: Build method | ✅ `build()` | Integration |
| FR-003: Recursive processing | ✅ Internal | Integration |
| FR-004: Binding evaluation | ✅ Via model | Unit + Integration |
| FR-005: Event connection | ✅ Via registry | Integration |
| FR-006: Style application | ✅ From trait | Unit |
| FR-007: Layout application | ✅ From trait | Unit |
| FR-008: Type conversions | ✅ From trait | Unit |
| FR-009: All properties | ✅ Complete | Unit |
| FR-010: Backend agnostic | ✅ Core separate | Architecture |
| FR-011: <50 lines | ✅ Single call | Examples |
| FR-012: All widget types | ✅ Complete match | Integration |
| FR-013: Graceful defaults | ✅ From trait | Unit |
| FR-014: Verbose logging | ✅ `with_verbose()` | Unit |
| FR-015: Graceful degradation | ✅ Option registry | Integration |

---

## Examples

See `specs/003-widget-builder/quickstart.md` for complete usage examples.
