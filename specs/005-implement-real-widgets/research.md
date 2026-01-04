# Research: Implement Real Iced Widgets

**Feature Branch**: `005-implement-real-widgets`  
**Date**: 2026-01-04

## Research Questions

### Q1: How do Iced 0.14 widgets handle event callbacks?

**Decision**: Iced 0.14 widgets use closure-based event handlers that produce messages.

**Rationale**: Each interactive widget takes a closure that maps the event value to a message type. For some widgets (Button), the handler is optional via `.on_press()`. For others (PickList, Slider), the handler is a required constructor parameter.

**Key Findings**:
- `TextInput::on_input(Fn(String) -> Message)` - optional method
- `Checkbox::on_toggle(Fn(bool) -> Message)` - optional method
- `Toggler::on_toggle(Fn(bool) -> Message)` - optional method  
- `PickList::new(options, selected, Fn(T) -> Message)` - required in constructor
- `Slider::new(range, value, Fn(T) -> Message)` - required in constructor
- `Image` - no events (display only)

**Alternatives Considered**:
- Custom message handling system - rejected, Iced's system is well-designed
- Direct model mutation in callbacks - rejected, violates Elm architecture

### Q2: How should HandlerMessage be extended to carry event values?

**Decision**: Use the existing `Handler(String, Option<String>)` variant with serialized values.

**Rationale**: The current HandlerMessage already supports an optional String payload. Rather than adding new variants and breaking the existing API, we can serialize values (bool, f32) to strings and let handlers deserialize them. This maintains backward compatibility.

**Key Findings**:
- Current: `HandlerMessage::Handler(String, Option<String>)`
- Bool values: `"true"` / `"false"` strings
- Float values: `value.to_string()`
- Selection values: Already strings

**Alternatives Considered**:
- New enum variants (`Toggle(String, bool)`, etc.) - rejected, breaks existing handler dispatching
- Generic value type with `Box<dyn Any>` - rejected, loses type safety
- Separate message types per widget - rejected, complicates builder

### Q3: How to handle lifetime requirements for widget values?

**Decision**: Use owned `String` values stored in the widget creation scope.

**Rationale**: Iced widgets store references to placeholder/value strings. For Gravity's builder pattern where values come from evaluated bindings, we need to ensure the strings live long enough. By creating owned strings at widget construction time, we avoid lifetime issues.

**Key Findings**:
- `text_input(placeholder: &str, value: &str)` - references needed
- Builder evaluates bindings to `String`, which can be borrowed within the build scope
- Closures can move owned handler names

**Implementation Pattern**:
```rust
let placeholder = self.evaluate_attribute(...);  // Returns String
let value = self.evaluate_attribute(...);        // Returns String
text_input(&placeholder, &value)                 // Borrow for 'a
```

### Q4: How to handle pick_list options from comma-separated string?

**Decision**: Parse options attribute into `Vec<String>`, use owned strings as options.

**Rationale**: The XML format uses `options="A,B,C"` for simplicity. Splitting and trimming this string produces owned values that PickList can use directly.

**Key Findings**:
- PickList requires `L: Borrow<[T]>` where `T: ToString + PartialEq + Clone`
- `Vec<String>` satisfies these requirements
- Selected value must match one of the options (or be `None`)

**Implementation Pattern**:
```rust
let options_str = evaluate_attribute(...);
let options: Vec<String> = options_str.split(',')
    .map(|s| s.trim().to_string())
    .collect();
let selected = evaluate_attribute(...);
let selected_opt = options.iter()
    .find(|o| *o == &selected)
    .cloned();
```

### Q5: What about Image widget and feature flags?

**Decision**: Image feature is already enabled in workspace Cargo.toml.

**Rationale**: The workspace already specifies `iced = { version = "0.14", features = ["tokio", "canvas", "image"] }`, so image support is available.

**Key Findings**:
- `image("path/to/file.png")` creates widget from path
- Handle::from_path(path) for explicit construction
- Widget dimensions via `.width()` and `.height()` methods
- Content fit via `.content_fit()` method

**Error Handling**: When image file doesn't exist, Iced displays a broken image indicator. We'll log a warning in verbose mode.

### Q6: How to handle slider step attribute?

**Decision**: Iced 0.14 Slider doesn't have a built-in step feature; use continuous values.

**Rationale**: The Iced slider widget provides continuous value selection. Step quantization would need to be handled in the application's update function if required.

**Key Findings**:
- `slider(range, value, on_change)` is the full API
- No `.step()` method available
- Applications can quantize in their message handler

**Implementation**: Document that step is not supported by the underlying Iced widget. Values are continuous.

## Summary

All technical questions have been resolved. The implementation approach is:

1. **Keep HandlerMessage unchanged** - use `Option<String>` payload for all value-carrying events
2. **Use owned strings** - evaluate bindings return `String`, borrow within build scope
3. **Parse options inline** - split comma-separated options attribute
4. **Image already supported** - feature flag enabled in workspace
5. **Slider is continuous** - no step support at widget level
