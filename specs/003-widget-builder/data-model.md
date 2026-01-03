# Data Model: Gravity Widget Builder

**Date**: 2026-01-03  
**Feature**: 003-widget-builder  
**Phase**: 1 - Design

---

## Core Entities

### 1. GravityWidgetBuilder

**Purpose**: Central orchestrator for converting parsed UI definitions to rendered widgets

**Fields**:
- `node: &'a WidgetNode` - Root widget node from parsed XML
- `model: &'a dyn UiBindable` - Application state for binding evaluation
- `handler_registry: Option<&'a HandlerRegistry<Message>>` - Event handler registry
- `verbose: bool` - Enable debug logging

**Lifecycle**:
1. Created via `GravityWidgetBuilder::new(node, model, handler_registry)`
2. Configured with `.with_verbose(true)` (optional)
3. Executed via `.build()` → returns `Element<'a, Message>`

**State Transitions**:
- `New` → `Configured` → `Built` (single-use, non-reusable)

---

### 2. Conversion System (convert.rs)

**Purpose**: Type conversions between IR and Iced types

**Conversion Map**:

| IR Type | Iced Type | Conversion Logic |
|---------|-----------|------------------|
| `StyleProperties.color` | `iced::Color` | Parse hex (#RRGGBB) or named colors |
| `StyleProperties.background` | `iced::Background` | Color or gradient from hex |
| `StyleProperties.padding` | `iced::Padding` | Parse "top right bottom left" or single value |
| `StyleProperties.spacing` | `iced::Spacing` | Parse single value for both axes |
| `StyleProperties.width/height` | `iced::Length` | Parse "px N", "% N", "auto", "fill" |
| `StyleProperties.border` | `iced::Border` | Parse "width radius color" |
| `StyleProperties.shadow` | `iced::Shadow` | Parse "x y blur color" |
| `WidgetNode.text` | `String` | Evaluate bindings {expr}, return String |

**Validation Rules**:
- All conversions must be infallible (IR is pre-validated)
- Invalid values use defaults (FR-013)
- Malformed bindings return empty string

---

### 3. Widget Building Flow

**Input**: `WidgetNode` tree from parser

**Process**:
```
1. Parse node.widget_type
2. Extract style properties
3. Convert to Iced styles (From trait)
4. Evaluate text/bindings
5. Connect events (if handler exists)
6. Recursively build children
7. Wrap in container/layout
8. Return Element
```

**Recursion**:
- Depth-first traversal
- Each child creates new builder instance
- Shared model and registry across all levels
- No cycle detection needed (tree structure)

---

## Data Flow

### Binding Evaluation Flow

```
XML: <text value="{user.name}" />
    ↓
WidgetNode.text = "{user.name}"
    ↓
evaluate_binding_expr("{user.name}", model)
    ↓
model.get_field(["user", "name"])
    ↓
BindingValue::String("Alice")
    ↓
String::from("Alice")
    ↓
Iced Text widget
```

### Event Connection Flow

```
XML: <button label="Click" on_click="increment" />
    ↓
WidgetNode.on_click = "increment"
    ↓
handler_registry.get("increment")
    ↓
Some(Handler { signature, message })
    ↓
Button.on_press(Some(message))
    ↓
Iced Button widget
```

### Style Application Flow

```
XML: <column background="#3498db" padding="20" />
    ↓
StyleProperties { background: "#3498db", padding: "20" }
    ↓
From<StyleProperties> for ContainerStyle
    ↓
ContainerStyle {
    background: Color::from_hex("#3498db"),
    padding: Padding::from(20)
}
    ↓
Container::new(...).style(style)
```

---

## Relationships

### Entity Relationships

```
GravityWidgetBuilder (1) ──► (1) WidgetNode (root)
                              │
                              ├─► (0..n) WidgetNode (children)
                              │
GravityWidgetBuilder (1) ──► (1) UiBindable (model)
                              │
                              ├─► (0..n) fields (via get_field)
                              │
GravityWidgetBuilder (1) ──► (0..1) HandlerRegistry
                              │
                              ├─► (0..n) handlers (by name)
                              │
WidgetNode (1) ──► (0..1) StyleProperties
WidgetNode (1) ──► (0..1) LayoutConstraints
WidgetNode (1) ──► (0..1) EventHandlers
```

### Dependency Graph

```
gravity-core (IR types)
    ↓ (used by)
gravity-iced (builder + convert)
    ↓ (uses)
gravity-runtime (HandlerRegistry, evaluate_binding_expr)
    ↓ (produces)
Iced widgets (Element)
```

---

## Validation Rules

### Builder Creation
- ✅ `node` must be non-null
- ✅ `model` must implement `UiBindable`
- ✅ `handler_registry` can be None (graceful degradation)

### Widget Processing
- ✅ All widget types must be handled (exhaustive match)
- ✅ Missing attributes use defaults
- ✅ Invalid bindings return empty string
- ✅ Missing handlers log warning (if verbose)

### Performance Constraints
- ✅ No unnecessary allocations in hot path
- ✅ O(n) complexity where n = widget count
- ✅ Target: 50ms for 1000 widgets

---

## Error States

| Error Condition | Detection | Response | User Impact |
|-----------------|-----------|----------|-------------|
| Unsupported widget type | Match exhaustiveness | Empty placeholder + error log | Widget not rendered |
| Missing handler | Registry lookup fails | Event ignored + warning | Button click does nothing |
| Binding parse error | evaluate_binding_expr fails | Empty string + error log | Text shows empty |
| Malformed attribute | From conversion fails | Default value + warning | Visual default applied |
| No handler registry | Option is None | Events ignored silently | Static UI only |

---

## State Transitions

### Builder Lifecycle

```
[Uninitialized]
    ↓ new(node, model, registry)
[Initialized]
    ↓ build()
    ├─► [Success] Element<'a, Message>
    └─► [Error] Empty element + logs
```

### Widget Node Processing

```
[Node Received]
    ↓ match widget_type
[Type Identified]
    ↓ extract properties
[Properties Extracted]
    ↓ convert styles (From)
[Styles Converted]
    ↓ evaluate bindings
[Bindings Evaluated]
    ↓ connect events
[Events Connected]
    ↓ build children (recursive)
[Children Built]
    ↓ wrap in layout
[Layout Wrapped]
    ↓ return Element
```

---

## API Contracts

### Public API

```rust
// Constructor
impl<'a, Message> GravityWidgetBuilder<'a, Message> {
    pub fn new(
        node: &'a WidgetNode,
        model: &'a dyn UiBindable,
        handler_registry: Option<&'a HandlerRegistry<Message>>
    ) -> Self
}

// Configuration
impl<'a, Message> GravityWidgetBuilder<'a, Message> {
    pub fn with_verbose(self, verbose: bool) -> Self
}

// Execution
impl<'a, Message> GravityWidgetBuilder<'a, Message> 
where
    Message: Clone + 'static
{
    pub fn build(self) -> Element<'a, Message>
}
```

### Conversion API (Internal)

```rust
// From implementations
impl From<StyleProperties> for ContainerStyle { ... }
impl From<StyleProperties> for ButtonStyle { ... }
impl From<Length> for iced::Length { ... }
impl From<Color> for iced::Color { ... }
// ... etc
```

---

## Testing Requirements

### Unit Tests
- Each From implementation
- Binding evaluation edge cases
- Missing attribute defaults
- Error handling paths

### Integration Tests
- Full widget tree building
- Binding updates (model changes)
- Event handler connection
- Verbose logging output

### Performance Tests
- 1000 widget render time
- Binding evaluation overhead
- Memory allocation count

---

## Implementation Notes

### File Structure
```
crates/gravity-iced/src/
├── lib.rs              # Export builder
├── builder.rs          # GravityWidgetBuilder struct
├── convert.rs          # From implementations
└── widgets/            # Existing (unchanged)
```

### Key Functions
1. `GravityWidgetBuilder::build()` - Entry point
2. `GravityWidgetBuilder::build_widget()` - Recursive dispatcher
3. `GravityWidgetBuilder::build_text()` - Text widget handler
4. `GravityWidgetBuilder::build_button()` - Button widget handler
5. `GravityWidgetBuilder::build_column()` - Column layout handler
6. `GravityWidgetBuilder::build_row()` - Row layout handler
7. `GravityWidgetBuilder::build_container()` - Container handler

### Dependencies
- `gravity-core` (IR types)
- `gravity-runtime` (HandlerRegistry, evaluate_binding_expr)
- `iced` (rendering)
- `serde` (for model binding, if needed)

---

## Next Steps

1. Implement `convert.rs` with all From implementations
2. Implement `builder.rs` with recursive building logic
3. Update `lib.rs` to export builder
4. Add unit tests for conversions
5. Add integration tests for full pipeline
6. Update examples to use builder
7. Performance benchmarking
