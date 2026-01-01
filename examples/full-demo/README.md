# Full Demo Example

This example showcases **all features** of the Gravity UI framework.

## Features Demonstrated

### 1. Basic Bindings
- Counter with increment/decrement/reset
- Conditional button enabling

### 2. Form Inputs
- Text input with `on_input` handlers
- Password masking
- Real-time value display

### 3. Conditional Rendering
- Loading state display
- Error state display
- Conditional text based on state

### 4. Lists and Collections
- Dynamic list management
- Add/remove items
- Sort functionality
- Collection statistics (length, total)

### 5. Layout Examples
- Container widgets with alignment
- Row/column layouts
- Spacing and padding

### 6. Status Tracking
- Interaction counting
- Action history
- Session timing

## Running the Demo

```bash
# Development mode with hot-reload
cargo run --features dev

# Production build
cargo run --release
```

## File Structure

```
full-demo/
├── Cargo.toml
├── src/
│   └── main.rs          # Model and handlers
└── ui/
    └── main.gravity     # UI definition
```

## What to Try

1. **Edit the UI**: Modify `ui/main.gravity` and see changes instantly
2. **Test bindings**: Change counter, add items, toggle states
3. **Experiment**: Try adding new widgets or modifying layouts

This example is designed to be a reference for building real applications.
