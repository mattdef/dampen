# Research: Gravity Widget Builder Implementation

**Date**: 2026-01-03  
**Feature**: 003-widget-builder  
**Phase**: 0 - Research & Architecture

---

## Decision 1: From Trait Implementation Strategy

**Decision**: Implement `From<IR_Type>` for `Iced_Type` in `gravity-iced/src/convert.rs`

**Rationale**: 
- Rust's `From` trait provides zero-cost abstractions for type conversions
- Allows idiomatic `.into()` calls in the builder
- Centralizes all conversion logic in one file
- Enables compile-time verification of all conversions
- Supports the backend abstraction principle (conversions are backend-specific)

**Alternatives Considered**:
- ❌ Manual conversion functions: More verbose, harder to maintain
- ❌ Macro-based generation: Overly complex for this use case
- ❌ Trait objects: Would violate type safety principle

**Implementation Notes**:
- Color: Parse hex strings to `iced::Color`
- Length: Parse units (px, %, auto) to `iced::Length`
- Padding/Spacing: Parse to `iced::Padding` / `iced::Spacing`
- Border: Parse width, radius, color to `iced::Border`
- Background: Parse color or gradient to `iced::Background`
- Transform: Parse rotation, scale to `iced::Transform`

---

## Decision 2: Recursive Widget Building Pattern

**Decision**: Use match expression on `WidgetNode` enum with recursive calls to builder

**Rationale**:
- Exhaustive matching ensures all widget types are handled
- Recursive builder calls maintain context (model, handlers, depth)
- Clean separation: each widget type has dedicated conversion logic
- Easy to extend with new widget types

**Pattern**:
```rust
fn build_widget(&self, node: &WidgetNode) -> Element<'a, Message> {
    match node.widget_type {
        WidgetType::Text => self.build_text(node),
        WidgetType::Button => self.build_button(node),
        WidgetType::Column => self.build_column(node),
        WidgetType::Row => self.build_row(node),
        WidgetType::Container => self.build_container(node),
    }
}
```

**Alternatives Considered**:
- ❌ Visitor pattern: Over-engineered for this scope
- ❌ Trait-based dispatch: Would require changing IR types

---

## Decision 3: HandlerRegistry Integration

**Decision**: Pass `&HandlerRegistry<Message>` to builder, map event names to messages

**Rationale**:
- Registry already exists in gravity-runtime
- Type-safe message dispatch via generics
- Graceful degradation when registry is None (FR-015)
- Supports the clarification: "Graceful degradation with clear error messages"

**Implementation**:
```rust
pub struct GravityWidgetBuilder<'a, Message> {
    node: &'a WidgetNode,
    model: &'a dyn UiBindable,
    handler_registry: Option<&'a HandlerRegistry<Message>>,
    verbose: bool,
}
```

**Event Handling**:
- Parse `on_click="handler_name"` from XML
- Look up handler in registry
- If found: map to `Message::Handler(name, payload)`
- If not found: log warning (if verbose), ignore event
- If registry None: log error, no event connection

---

## Decision 4: Binding Evaluation Strategy

**Decision**: Use `evaluate_binding_expr` from gravity-runtime for all property values

**Rationale**:
- Already exists and tested
- Handles complex expressions like `{user.name}`, `{count + 1}`
- Returns `BindingValue` enum (String, Int, Bool, etc.)
- Integrates with model's `get_field()` method

**Implementation**:
```rust
fn evaluate_property(&self, raw_value: &str) -> String {
    if raw_value.contains('{') {
        match evaluate_binding_expr(raw_value, self.model) {
            Ok(value) => value.to_string(),
            Err(e) => {
                if self.verbose {
                    eprintln!("Binding error: {}", e);
                }
                String::new() // Graceful degradation
            }
        }
    } else {
        raw_value.to_string()
    }
}
```

**Alternatives Considered**:
- ❌ Manual string parsing: Error-prone, doesn't handle nested expressions
- ❌ Regex replacement: Performance issues, incomplete coverage

---

## Decision 5: Performance Optimization

**Decision**: Cache conversion results, minimize allocations, use borrowed data

**Rationale**:
- Target: 50ms for 1000 widgets (from spec)
- Builder uses references throughout (no owned data unless necessary)
- `From` implementations are zero-cost
- Recursive structure is O(n) where n = widget count

**Optimizations**:
1. All builder methods take `&self` and `&WidgetNode`
2. `From` implementations use borrowed data
3. String allocations only for evaluated bindings
4. No unnecessary clones in hot paths

**Performance Budget**:
- Per widget: < 0.05ms average
- Binding evaluation: < 0.01ms per property
- Type conversion: < 0.001ms per property

---

## Decision 6: Error Handling & Logging

**Decision**: Verbose mode with error overlay + console logging (from clarification)

**Rationale**:
- Matches existing gravity CLI patterns
- Non-verbose mode for production use
- Error overlay provides immediate visual feedback
- Console logging aids debugging

**Implementation**:
```rust
if self.verbose {
    eprintln!("[GravityWidgetBuilder] {}", message);
}
```

**Error Types**:
- Missing handler: Warning, event ignored
- Unsupported widget: Error, empty placeholder
- Binding error: Error, fallback to empty string
- Missing required attribute: Warning, use default

---

## Decision 7: Backend Abstraction Compliance

**Decision**: All Iced-specific code isolated in `gravity-iced`, core remains pure

**Rationale**:
- Satisfies Constitution Principle IV
- Enables future backends (e.g., web, mobile)
- `gravity-core` types are backend-agnostic
- Conversions happen at the boundary

**Architecture**:
```
gravity-core (IR types)
    ↓
gravity-iced (convert.rs + builder.rs)
    ↓
Iced widgets
```

**Future Backend Example**:
```rust
// gravity-web/src/convert.rs
impl From<StyleProperties> for WebCss {
    // Different implementation, same IR input
}
```

---

## Summary

All research questions resolved. The implementation strategy is clear:

1. **Structure**: `gravity-iced/src/builder.rs` + `gravity-iced/src/convert.rs`
2. **Pattern**: Recursive match on WidgetNode with From conversions
3. **Integration**: HandlerRegistry and evaluate_binding_expr from runtime
4. **Performance**: O(n) with minimal allocations, target 50ms/1000 widgets
5. **Error Handling**: Verbose mode with overlays + logging
6. **Compliance**: Backend-agnostic core, Iced-specific conversions isolated

**Ready for Phase 1: Design & Contracts**
