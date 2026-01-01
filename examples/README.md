# Gravity Examples

This directory contains example applications demonstrating Gravity's features.

## Available Examples

### 1. Hello World (`hello-world/`)
**Purpose**: Minimal static example  
**Features**: Basic XML parsing, simple rendering  
**UI**: Embedded in Rust code

### 2. Styling (`styling/`)
**Purpose**: Layout and styling demonstration  
**Features**: 
- External `.gravity` files
- Layout attributes (padding, spacing, width, height)
- Style attributes (background, border, shadow)
- Binding expressions
- Theme support

**Run**: `cargo run -p styling`

### 3. Responsive (`responsive/`)
**Purpose**: Responsive layout demonstration  
**Features**:
- External `.gravity` files
- Fixed/fill width containers
- Min/max constraints
- Alignment
- Nested layouts

**Run**: `cargo run -p responsive`

### 4. Counter (`counter/`)
**Purpose**: Interactive handlers demonstration  
**Features**:
- Event handlers
- State management
- Dynamic updates

### 5. Todo App (`todo-app/`)
**Purpose**: Full bindings demonstration  
**Features**:
- Model bindings
- List rendering
- CRUD operations

## Key Differences

| Example | UI Location | Features | Complexity |
|---------|-------------|----------|------------|
| hello-world | Embedded | Basic | Low |
| styling | External file | Layout + Style | Medium |
| responsive | External file | Constraints | Medium |
| counter | Embedded | Handlers | Medium |
| todo-app | Embedded | Bindings | High |

## Running Examples

```bash
# Run any example
cargo run -p <example-name>

# Examples:
cargo run -p styling
cargo run -p responsive
cargo run -p hello-world
```

## Why External .gravity Files?

### Benefits
1. **Separation of Concerns**: UI designers work on XML, developers on Rust
2. **Hot-Reload**: Modify UI without recompiling
3. **Readability**: XML is more readable for UI structure
4. **Tooling**: Can use XML editors, validators
5. **Collaboration**: Designers and developers can work independently

### Trade-offs
- Requires file I/O at runtime
- Need to manage file paths
- Slightly more complex setup

## Getting Started

### 1. Explore Styling Example
```bash
cd examples/styling
cat ui/main.gravity
cargo run
```

### 2. Modify the UI
Edit `ui/main.gravity` and change:
- `padding="40"` → `padding="60"`
- Colors
- Text content

### 3. Run Again
```bash
cargo run
```

See your changes immediately!
