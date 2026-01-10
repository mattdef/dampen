# Dual-Mode Architecture Developer Guide

This guide explains the internals of Dampen's dual-mode architecture for framework contributors and advanced users.

## Table of Contents

- [Overview](#overview)
- [Architecture Diagram](#architecture-diagram)
- [Interpreted Mode Implementation](#interpreted-mode-implementation)
- [Codegen Mode Implementation](#codegen-mode-implementation)
- [Feature Flags](#feature-flags)
- [Hot-Reload System](#hot-reload-system)
- [Performance Optimizations](#performance-optimizations)
- [Testing Strategy](#testing-strategy)
- [Debugging](#debugging)

## Overview

The dual-mode architecture provides two distinct compilation paths:

1. **Interpreted Mode**: Runtime XML parsing with hot-reload capabilities
2. **Codegen Mode**: Build-time code generation for zero runtime overhead

Both modes produce **identical UI behavior** (mode parity) but with different performance characteristics and developer experience.

## Architecture Diagram

```
┌─────────────────────────────────────────────────────────┐
│                     User Application                     │
│                                                          │
│  ┌────────────────┐                  ┌────────────────┐ │
│  │  main.rs       │                  │  ui/window.rs  │ │
│  │                │                  │                │ │
│  │  - init()      │──────calls───────│  #[dampen_ui]  │ │
│  │  - update()    │                  │  - document()  │ │
│  │  - view()      │                  │  - handlers    │ │
│  └────────────────┘                  └────────────────┘ │
│         │                                      │         │
└─────────┼──────────────────────────────────────┼─────────┘
          │                                      │
          │              ┌───────────────────────┤
          │              │                       │
          v              v                       v
  ┌──────────────┬──────────────┐       ┌──────────────┐
  │ Interpreted  │   Codegen    │       │ dampen-core  │
  │    Mode      │     Mode     │       │              │
  ├──────────────┼──────────────┤       │ - parser     │
  │              │              │       │ - IR         │
  │ Runtime:     │ Build-time:  │       │ - traits     │
  │              │              │       └──────────────┘
  │ - Parse XML  │ - build.rs   │
  │ - LazyLock   │ - gen code   │
  │ - Hot-reload │ - Static IR  │
  └──────────────┴──────────────┘
          │              │
          └──────┬───────┘
                 │
                 v
        ┌────────────────┐
        │  dampen-iced   │
        │                │
        │  - Builder     │
        │  - Widgets     │
        │  - Theme       │
        └────────────────┘
                 │
                 v
        ┌────────────────┐
        │      Iced      │
        │   Framework    │
        └────────────────┘
```

## Interpreted Mode Implementation

### Core Components

**1. dampen-macros/src/ui_loader.rs**

The `#[dampen_ui]` macro generates different code based on feature flags:

```rust
#[cfg(any(feature = "interpreted", not(feature = "codegen")))]
{
    // Interpreted mode: runtime parsing with LazyLock
    quote! {
        mod #mod_name {
            use std::sync::LazyLock;
            use dampen_core::ir::DampenDocument;
            
            static DOCUMENT: LazyLock<DampenDocument> = LazyLock::new(|| {
                let xml_content = include_str!(#xml_path);
                dampen_core::parser::parse(xml_content)
                    .expect("Failed to parse XML")
            });
            
            pub fn document() -> DampenDocument {
                DOCUMENT.clone()
            }
        }
    }
}
```

**2. dampen-dev/src/reload.rs**

Hot-reload implementation with state preservation:

```rust
pub fn attempt_hot_reload<M, F>(
    xml_source: &str,
    current_state: &AppState<M>,
    context: &mut HotReloadContext<M>,
    create_handlers: F,
) -> ReloadResult<M>
where
    M: UiBindable + Serialize + DeserializeOwned + Default,
    F: FnOnce() -> HandlerRegistry,
{
    // 1. Parse new XML
    let new_doc = match parser::parse(xml_source) {
        Ok(doc) => doc,
        Err(e) => return ReloadResult::ParseError(e),
    };
    
    // 2. Preserve model state
    let preserved_model = current_state.model.clone();
    
    // 3. Create new AppState
    let new_state = AppState {
        document: new_doc,
        model: preserved_model,
        handler_registry: Some(create_handlers()),
    };
    
    ReloadResult::Success(new_state)
}
```

**3. dampen-dev/src/watcher.rs**

File system watching with debouncing:

```rust
pub fn watch_files<P: AsRef<Path>>(
    paths: Vec<P>,
    extension_filter: &str,
) -> Subscription<FileEvent> {
    // Uses notify crate with 100ms debouncing
    // Emits FileEvent::Modified on changes
}
```

### Initialization Flow (Interpreted)

```
1. User calls #[dampen_ui("app.dampen")]
   ↓
2. Macro expands to LazyLock initialization
   ↓
3. First call to document() triggers:
   - include_str!("app.dampen")
   - parser::parse(xml_content)
   - Store in static DOCUMENT
   ↓
4. Subsequent calls clone cached Document
```

### Performance Characteristics

- **First load**: ~50-100ms (XML parsing)
- **Subsequent loads**: ~0.5ms (clone cached Document)
- **Hot-reload**: <300ms (parse + rebuild AppState)
- **Memory**: +5-10MB (parser + LazyLock storage)

## Codegen Mode Implementation

### Core Components

**1. build.rs**

Processes `.dampen` files at compile time:

```rust
#[cfg(feature = "codegen")]
fn main() {
    println!("cargo:rerun-if-changed=src/ui");
    
    // Find all .dampen files
    let dampen_files = find_dampen_files("src/ui");
    
    for file in dampen_files {
        // Parse XML
        let xml = std::fs::read_to_string(&file)?;
        let document = dampen_core::parser::parse(&xml)?;
        
        // Generate Rust code
        let code = dampen_core::codegen::generate(&document);
        
        // Write to OUT_DIR
        let out_path = format!("{}/{}.rs", env::var("OUT_DIR")?, file.stem());
        std::fs::write(out_path, code)?;
    }
}
```

**2. dampen-core/src/codegen/mod.rs**

Code generation from IR:

```rust
pub fn generate(document: &DampenDocument) -> String {
    let mut code = String::new();
    
    code.push_str("pub fn document() -> DampenDocument {\n");
    code.push_str("    DampenDocument {\n");
    
    // Generate widget tree
    for widget in &document.root_widgets {
        code.push_str(&generate_widget(widget, 2));
    }
    
    code.push_str("    }\n");
    code.push_str("}\n");
    
    code
}
```

**3. Macro expansion (Codegen)**

```rust
#[cfg(all(feature = "codegen", not(feature = "interpreted")))]
{
    // Codegen mode: include generated code
    let generated_path = format!("{}/{}.rs", env!("OUT_DIR"), stem);
    
    quote! {
        mod #mod_name {
            include!(concat!(env!("OUT_DIR"), "/", #generated_file));
        }
    }
}
```

### Initialization Flow (Codegen)

```
1. Cargo invokes build.rs
   ↓
2. build.rs finds all .dampen files
   ↓
3. For each file:
   - Parse XML to IR
   - Generate Rust code
   - Write to OUT_DIR
   ↓
4. Macro includes generated file
   ↓
5. document() function returns static IR
   (no runtime parsing)
```

### Performance Characteristics

- **Build time**: +2-5s (code generation)
- **Runtime init**: <1ms (static data)
- **Memory**: -5MB (no parser needed)
- **Binary size**: +10-20KB (generated code)

## Feature Flags

### Flag Hierarchy

```toml
[features]
default = []
interpreted = ["dampen-dev", "dampen-core/interpreted"]
codegen = ["dampen-core/codegen"]

# Profile-based auto-selection
[profile.dev]
features = ["interpreted"]

[profile.release]
features = ["codegen"]
```

### Resolution Logic

```rust
// In dampen-macros/src/ui_loader.rs
#[cfg(any(
    feature = "interpreted",
    not(feature = "codegen")
))]
{
    // Use interpreted mode
}

#[cfg(all(
    feature = "codegen",
    not(feature = "interpreted")
))]
{
    // Use codegen mode
}
```

**Priority**: `interpreted` > `codegen` > default (interpreted)

## Hot-Reload System

### Architecture

```
File System          Watcher           Application
    │                   │                    │
    │  .dampen changed  │                    │
    ├──────notify──────>│                    │
    │                   │                    │
    │              ┌────┴────┐               │
    │              │ Debounce│               │
    │              │ (100ms) │               │
    │              └────┬────┘               │
    │                   │  FileEvent         │
    │                   ├────emit───────────>│
    │                   │                    │
    │                   │                ┌───┴───┐
    │                   │                │ Reload│
    │                   │                │Context│
    │                   │                └───┬───┘
    │                   │                    │
    │                   │                Parse XML
    │                   │                    │
    │                   │             Preserve State
    │                   │                    │
    │                   │             Update AppState
    │                   │                    │
    │                   │<───result──────────┤
```

### State Preservation

```rust
pub struct HotReloadContext<M> {
    last_successful_reload: Option<Instant>,
    reload_count: usize,
    error_count: usize,
    parse_cache: HashMap<u64, ParsedDocumentCache>,
}

// Model state is preserved across reloads
let preserved_model = current_state.model.clone();
let new_state = AppState {
    document: new_doc,
    model: preserved_model,  // ← State preserved
    handler_registry: Some(create_handlers()),
};
```

### Error Handling

```rust
pub enum ReloadResult<M: UiBindable> {
    Success(AppState<M>),
    ParseError(ParseError),
    NoChanges,
}

// On error, keep current state
match attempt_hot_reload(...) {
    ReloadResult::Success(new_state) => app.state = new_state,
    ReloadResult::ParseError(e) => {
        eprintln!("Hot-reload failed: {}", e);
        // Keep current state, show error overlay
    }
    ReloadResult::NoChanges => { /* No-op */ }
}
```

## Performance Optimizations

### 1. AST Caching

```rust
struct ParsedDocumentCache {
    document: DampenDocument,
    cached_at: Instant,
}

// Cache parsed documents by content hash
let hash = calculate_hash(&xml_source);
if let Some(cached) = context.parse_cache.get(&hash) {
    return ReloadResult::NoChanges;  // ← Skip re-parse
}
```

### 2. Async Parsing

```rust
pub async fn attempt_hot_reload_async<M, F>(
    xml_source: String,
    current_state: &AppState<M>,
    context: &mut HotReloadContext<M>,
    create_handlers: F,
) -> ReloadResult<M>
{
    // Parse XML on background thread
    let parse_result = tokio::task::spawn_blocking(move || {
        dampen_core::parser::parse(&xml_source)
    }).await;
    
    // UI thread not blocked during parsing
}
```

### 3. Debouncing

```rust
// In watcher.rs
let (tx, rx) = mpsc::channel();
let debouncer = new_debouncer(
    Duration::from_millis(100),  // ← 100ms window
    None,
    move |result| {
        // Only emit after 100ms of no changes
    }
)?;
```

## Testing Strategy

### Mode Parity Tests

Ensure both modes produce identical UI:

```rust
#[test]
fn test_mode_parity() {
    let xml = r#"<dampen><text value="test" /></dampen>"#;
    
    // Parse in interpreted mode
    #[cfg(feature = "interpreted")]
    let doc_interpreted = dampen_core::parser::parse(xml).unwrap();
    
    // Parse in codegen mode
    #[cfg(feature = "codegen")]
    let doc_codegen = generated::document();
    
    // Both should be identical
    assert_eq!(doc_interpreted, doc_codegen);
}
```

### Contract Tests

```rust
// tests/integration/mode_parity_tests.rs
#[test]
fn test_parse_parity() {
    // Load same XML in both modes
    // Verify IR is identical
}

#[test]
fn test_binding_parity() {
    // Evaluate bindings in both modes
    // Verify results match
}
```

### Performance Tests

```rust
// benchmarks/benches/dual_mode_bench.rs
fn bench_interpreted_init(c: &mut Criterion) {
    c.bench_function("interpreted_1000_widgets", |b| {
        b.iter(|| {
            let xml = generate_xml_with_widgets(1000);
            parser::parse(&xml)
        });
    });
}

fn bench_codegen_init(c: &mut Criterion) {
    c.bench_function("codegen_1000_widgets", |b| {
        b.iter(|| {
            generated::document()  // Static data, should be <1ms
        });
    });
}
```

## Debugging

### Interpreted Mode

**Enable verbose logging:**

```bash
RUST_LOG=dampen_dev=debug cargo run
```

**Check parse errors:**

```rust
match parser::parse(xml) {
    Ok(doc) => println!("Parsed successfully: {:?}", doc),
    Err(e) => {
        eprintln!("Parse error at {}:{}", e.span.line, e.span.col);
        eprintln!("  {}", e.message);
        eprintln!("  Suggestion: {}", e.suggestion.unwrap_or_default());
    }
}
```

### Codegen Mode

**Inspect generated code:**

```bash
# Generate code
cargo build --features codegen

# View generated files
ls -la target/debug/build/*/out/

# Read generated code
cat target/debug/build/my-app-*/out/window.rs
```

**Debug build script:**

```rust
// build.rs
fn main() {
    eprintln!("Running build.rs in codegen mode");
    eprintln!("OUT_DIR: {}", env::var("OUT_DIR").unwrap());
    
    // ... generation logic
}
```

### Hot-Reload Issues

**Check file watcher:**

```bash
# Verify notify crate is working
RUST_LOG=notify=debug cargo run
```

**Test reload manually:**

```rust
// Force reload on key press
match message {
    Message::KeyPressed(Key::F5) => {
        let xml = std::fs::read_to_string("src/ui/window.dampen")?;
        attempt_hot_reload(&xml, &app.state, &mut app.context, create_handlers)
    }
}
```

## Contributing

When adding new features to dual-mode:

1. **Implement in both modes** - ensure parity
2. **Add contract tests** - verify identical behavior
3. **Benchmark performance** - measure overhead
4. **Update documentation** - explain mode differences
5. **Test migration** - verify upgrade path works

See [CONTRIBUTING.md](../../CONTRIBUTING.md) for full guidelines.

## References

- [Migration Guide](../migration/dual-mode.md) - User migration instructions
- [Performance Guide](../performance.md) - Optimization tips
- [Architecture Specification](../../specs/001-dual-mode-architecture/spec.md) - Technical specification
