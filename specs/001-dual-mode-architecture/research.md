# Research: Dual-Mode Architecture Technical Decisions

**Feature**: 001-dual-mode-architecture  
**Date**: 2026-01-09  
**Status**: Complete

## Executive Summary

This document consolidates research findings from four key technical areas required for implementing Dampen's dual-mode architecture (interpreted development mode + codegen production mode):

1. **File Watching**: Using `notify` crate for hot-reload
2. **Iced Subscriptions**: Custom subscription patterns for file change events
3. **Code Generation**: Strategies for eliminating runtime interpretation
4. **State Preservation**: Hot-reload without losing application state

---

## 1. File Watching with `notify` Crate

### Decision: Use `notify-debouncer-full` with 50-100ms Window

**Rationale**:
- `notify` v6.1 is battle-tested (used by cargo-watch, bacon, trunk)
- Built-in `notify-debouncer-full` handles duplicate events automatically
- 50-100ms debounce window balances responsiveness vs. excessive reloads
- Cross-platform support (inotify, FSEvents, ReadDirectoryChangesW) via `recommended_watcher()`

### Implementation Pattern

**Architecture**:
```
DampenWatcher (notify wrapper)
    ↓ Filters by .dampen extension
    ↓ Debounces events (100ms)
Crossbeam Channel
    ↓ Non-blocking send
Iced Subscription
    ↓ Polls channel via async stream
Application Message (ReloadUI)
```

**Key Components**:

1. **Watcher Module** (`dampen-dev/src/watcher.rs`):
   - Wraps `notify::recommended_watcher()`
   - Exposes `crossbeam_channel::Receiver<PathBuf>`
   - Runs on background thread

2. **Subscription Module** (`dampen-dev/src/subscription.rs`):
   - Implements `iced::Subscription<Message>`
   - Wraps receiver in `futures::stream::unfold`
   - Maps `PathBuf` → `Message::HotReload(path)`

### Error Handling Strategy

| Error Type | Response | Rationale |
|------------|----------|-----------|
| File deleted | Ignore event, wait for Create | Editors delete-then-recreate on save |
| Permission denied | Log warning, continue watching | Don't crash watcher thread |
| Parse failure | Show error overlay, keep old UI | Preserve working state |
| Network drive timeout | Retry with exponential backoff | Common on WSL/network paths |

### Cross-Platform Considerations

- **Linux (inotify)**: Watch parent directory to avoid FD limits
- **macOS (FSEvents)**: Filter by `.dampen` extension (coarse events)
- **Windows**: Use 100ms+ debounce (may miss rapid events)

### Performance Expectations

| Metric | Target | Notes |
|--------|--------|-------|
| Event detection | <100ms | Platform-dependent, spec requirement |
| Debounce overhead | +50-100ms | Configurable |
| File read (1KB XML) | 1-5ms | Async I/O |
| Total detection time | <150ms | Well within 300ms hot-reload budget |

### Dependencies

```toml
[dependencies]
notify = "6.1"
notify-debouncer-full = "0.3"
crossbeam-channel = "0.5"
futures = "0.3"
```

---

## 2. Iced Subscription Patterns

### Decision: Recipe-Based Subscription with Channel Bridge

**Rationale**:
- Iced 0.14 uses Recipe pattern for custom subscriptions
- Need to bridge `notify`'s sync channels to Iced's async streams
- Existing codebase already uses Iced subscriptions (`window::resize_events`)

### Custom Subscription Implementation

**Pattern**:
```rust
pub fn watch_files<P: AsRef<Path>>(
    paths: Vec<P>,
    debounce_ms: u64,
) -> Subscription<FileEvent> {
    Subscription::from_recipe(FileWatcherRecipe {
        paths: paths.iter().map(|p| p.as_ref().to_path_buf()).collect(),
        debounce_ms,
    })
}

struct FileWatcherRecipe {
    paths: Vec<PathBuf>,
    debounce_ms: u64,
}

impl Recipe for FileWatcherRecipe {
    type Output = FileEvent;
    
    fn hash(&self, state: &mut Hasher) {
        self.paths.hash(state);
        self.debounce_ms.hash(state);
    }
    
    fn stream(self: Box<Self>, _input: EventStream) -> BoxStream<Self::Output> {
        // Bridge notify sync channel → async stream via spawn_blocking
        // Debounce events
        // Parse XML and yield FileEvent::Success or FileEvent::Error
    }
}
```

### Lifecycle Management

- **Activation**: Stream created when subscription first appears (based on hash)
- **Persistence**: Subscription persists while hash remains unchanged
- **Teardown**: Automatic cleanup when subscription removed or hash changes
- **Conditional**: Only active in development mode (`#[cfg(debug_assertions)]`)

### Event Flow

```
File System Change
    ↓
notify::Event (sync)
    ↓
std::sync::mpsc::channel
    ↓
tokio::task::spawn_blocking (bridge)
    ↓
tokio::sync::mpsc::channel (async)
    ↓
futures::stream::unfold
    ↓
Iced Subscription Output
    ↓
Application Message
```

### Error Propagation

**Three-Layer Model**:

```rust
enum FileEvent {
    Success { path: PathBuf, document: DampenDocument },
    ParseError { path: PathBuf, error: ParseError },
    WatcherError { path: PathBuf, error: String },
}
```

**Handling**:
- **Success**: Apply hot-reload normally
- **ParseError**: Show error overlay, preserve old UI state
- **WatcherError**: Log to console, continue watching

---

## 3. Code Generation Strategies

### Decision: build.rs-Based Generation with Optional Proc Macro Wrapper

**Rationale**:
- Dampen has multiple `.dampen` files requiring coordinated generation
- Need to scan Rust source for `#[ui_handler]` attributes
- Existing pattern already uses build.rs template
- Can reuse `dampen-core` codegen logic without proc_macro limitations

### Architecture Comparison

| Aspect | build.rs | proc_macro | Hybrid (Selected) |
|--------|----------|------------|-------------------|
| File I/O | ✅ Full access | ⚠️ Limited | ✅ Full access |
| Error reporting | ⚠️ Warnings only | ✅ Precise spans | ✅ Best of both |
| IDE support | ❌ No inline errors | ✅ rust-analyzer | ✅ Good |
| Multi-file coordination | ✅ Excellent | ❌ Isolated | ✅ Excellent |
| Code reuse | ✅ Uses core directly | ⚠️ Constraints | ✅ Shared logic |

### Binding Expression Inlining Strategy

**Current Issue**: Generated code still uses runtime traits:
```rust
// Current (hybrid):
#model_ident::count.to_binding_value().to_display_string()
```

**Goal**: Pure Rust code generation:
```rust
// Target (full codegen):
self.count.to_string()                    // Field access
(self.count + 1).to_string()              // Binary op
self.items.len().to_string()              // Method call
if self.done { "✓" } else { "○" }        // Conditional
format!("Count: {}", self.count)          // Interpolation
```

### Expression Generation Approach

**AST-to-TokenStream Translation**:

```rust
fn generate_expr(expr: &Expr, model: &syn::Ident) -> TokenStream {
    match expr {
        Expr::FieldAccess(f) => {
            let field = syn::Ident::new(&f.path[0], Span::call_site());
            quote! { #model.#field.to_string() }
        }
        Expr::BinaryOp(b) => {
            let left = generate_expr(&b.left, model);
            let right = generate_expr(&b.right, model);
            let op = match b.op {
                BinaryOp::Add => quote!(+),
                BinaryOp::Sub => quote!(-),
                // ...
            };
            quote! { (#left #op #right).to_string() }
        }
        Expr::MethodCall(m) => {
            let receiver = generate_expr(&m.receiver, model);
            let method = syn::Ident::new(&m.method, Span::call_site());
            quote! { #receiver.#method() }
        }
        Expr::Conditional(c) => {
            let cond = generate_expr(&c.condition, model);
            let then = generate_expr(&c.then_branch, model);
            let else_ = generate_expr(&c.else_branch, model);
            quote! { if #cond { #then } else { #else_ } }
        }
        // ... other cases
    }
}
```

### Type Preservation Strategy

**Challenge**: Build-time doesn't have runtime type information

**Solution**: Conservative generation + compiler verification

- Always use `.to_string()` for display values (compiler checks if valid)
- Leverage trait bounds in generated code (`M: UiBindable`)
- Trust Rust's type checker to catch errors
- Accept minor clippy warnings (`useless_conversion`) with targeted suppressions

### Conditional Compilation Pattern

```toml
# User's Cargo.toml
[features]
default = ["codegen"]
codegen = []

[profile.dev]
features = ["interpreted"]  # Hot-reload enabled

[profile.release]
features = ["codegen"]      # Zero runtime overhead
```

```rust
// User's src/ui/window.rs
#[cfg(feature = "codegen")]
include!(concat!(env!("OUT_DIR"), "/ui_window.rs"));  // Generated

#[cfg(feature = "interpreted")]
mod window_interpreted {
    // Runtime interpreter with hot-reload
    use dampen_iced::DampenWidgetBuilder;
    // ...
}
```

### Code Quality Assurance

1. **Formatting**: Use `prettyplease` crate for rustfmt-compatible output
2. **Validation**: Verify generated code parses as valid Rust
3. **Clippy**: Targeted suppressions only (`#![allow(clippy::useless_conversion)]`)
4. **Testing**: Snapshot tests with `insta` crate

---

## 4. Hot-Reload State Preservation

### Decision: Serialize Model, Reset UI Structure and Handlers

**Rationale**:
- Flutter and React both preserve application state during hot-reload
- Dampen's `AppState<M>` has clear separation of concerns
- Models already implement `Serialize + Deserialize` via `serde`

### State Preservation Matrix

| Component | Preserve? | Rationale |
|-----------|-----------|-----------|
| `model: M` (user data) | ✅ YES | Core application state, expensive to recreate |
| `document: DampenDocument` (UI structure) | ❌ NO | Source of changes, must reload |
| `handler_registry: HandlerRegistry` | ❌ NO | Function pointers invalidated by code changes |
| Scroll positions, UI state | ✅ YES (future) | Preserves developer context |
| Computed/derived state | ❌ NO | Can be recomputed from model |

### Serialization Pattern

**Snapshot-Restore with JSON**:

```rust
// 1. Snapshot current state
let model_snapshot = serde_json::to_string(&app_state.model)?;

// 2. Parse new XML
let new_document = parse_xml(xml_source)?;

// 3. Rebuild handlers (user function, may have changed)
let new_handlers = create_handlers();

// 4. Restore model from snapshot
let restored_model: M = serde_json::from_str(&model_snapshot)
    .unwrap_or_else(|_| M::default());  // Graceful degradation

// 5. Create new AppState
let new_state = AppState {
    document: new_document,
    model: restored_model,
    handler_registry: new_handlers,
    _marker: PhantomData,
};
```

**Why JSON**:
- Human-readable for debugging
- Forward/backward compatible with schema changes
- Already in dependency tree (`serde_json`)
- No file I/O needed (in-memory snapshot)

### Handler Preservation Strategy

**Decision: Full Registry Replacement**

**Pattern**:
1. User provides `create_handler_registry()` function
2. On hot-reload, call function again (new code version)
3. Handler names (strings) remain stable
4. XML references handlers by name: `on_click="increment"`

**Why This Works**:
- No function pointer stability required
- User code is recompiled with new implementations
- Aligns with "ephemeral code, persistent data" principle

### Error Recovery Mechanisms

| Error Type | Action | User Experience |
|------------|--------|-----------------|
| XML syntax error | Reject reload, keep old UI | Error overlay with file/line/message, fix and retry |
| Widget not found | Reject reload | Error overlay with suggestions |
| Model deserialization error | Accept reload, use `M::default()` | Warning banner, state reset |
| Handler not found | Accept reload | Runtime error on interaction |
| Parse validation error | Reject reload | Error overlay with details |

### Reload Flow

```
File Change Detection
    ↓
Snapshot Model (JSON, in-memory)
    ↓
Parse New XML
    ↓
Validate Schema ━━ Fail ━━> Error Overlay, Keep Old UI
    ↓ Success
Rebuild Handler Registry
    ↓
Deserialize Model ━━ Fail ━━> Use M::default(), Show Warning
    ↓ Success
Create New AppState
    ↓
Trigger Iced Re-render
    ↓
Hot-Reload Complete (<300ms)
```

### Performance Budget

| Phase | Estimated Time | Cumulative |
|-------|----------------|------------|
| File change detection | ~10ms | 10ms |
| Model serialization | ~5ms | 15ms |
| XML parsing | <10ms | 25ms |
| Schema validation | ~5ms | 30ms |
| Handler registry rebuild | ~2ms | 32ms |
| Model deserialization | ~5ms | 37ms |
| Iced widget tree rebuild | ~100ms | 137ms |
| Render | ~50ms | 187ms |
| **Total** | **187ms** | **✅ Within 300ms target** |

---

## 5. Integration Decisions

### New Crate: `dampen-dev`

**Purpose**: Development-mode-specific functionality (hot-reload, file watching)

**Structure**:
```
crates/dampen-dev/
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── watcher.rs       # FileWatcher (notify wrapper)
│   ├── subscription.rs  # Iced subscription implementation
│   ├── reload.rs        # Hot-reload coordination
│   └── overlay.rs       # Error overlay UI components
└── tests/
    ├── watcher_tests.rs
    └── reload_tests.rs
```

**Dependencies**:
```toml
[dependencies]
dampen-core = { path = "../dampen-core" }
iced = { workspace = true }
notify = "6.1"
notify-debouncer-full = "0.3"
crossbeam-channel = "0.5"
futures = "0.3"
serde_json = "1.0"
```

### CLI Integration

**New Commands**:

1. **`dampen run`**: Development mode launcher
   ```bash
   cargo run --no-default-features --features interpreted
   ```

2. **`dampen build`**: Production build wrapper
   ```bash
   cargo build --release --features codegen
   ```

**Implementation** (`dampen-cli/src/commands/run.rs`, `build.rs`):
- Set appropriate feature flags
- Configure cargo invocation
- Stream output to console

### Example Migration

**Update `examples/counter/Cargo.toml`**:
```toml
[features]
default = ["interpreted"]
codegen = []
interpreted = ["dampen-dev"]

[dependencies]
dampen-dev = { path = "../../crates/dampen-dev", optional = true }
```

**Update `examples/counter/build.rs`** (NEW):
```rust
fn main() {
    #[cfg(feature = "codegen")]
    {
        // Generate production code
        let doc = parse_dampen_file("src/ui/app.dampen");
        let code = dampen_core::codegen::generate_application(&doc);
        write_generated_code(&out_dir(), "ui_app.rs", &code);
    }
}
```

**Update `examples/counter/src/main.rs`**:
```rust
#[cfg(feature = "codegen")]
include!(concat!(env!("OUT_DIR"), "/ui_app.rs"));

#[cfg(feature = "interpreted")]
fn view(app: &App) -> Element<Message> {
    dampen_iced::DampenWidgetBuilder::from_app_state(&app.state).build()
}

#[cfg(feature = "interpreted")]
fn subscription(_app: &App) -> Subscription<Message> {
    dampen_dev::watch_files(vec!["src/ui/app.dampen"], 100)
        .map(Message::HotReload)
}
```

---

## 6. Open Questions & Resolutions

### Q1: Handler Signature Discovery in build.rs

**Question**: How does build.rs know handler signatures when they're in Rust source files?

**Resolution**: **Option A (MVP)** - Explicit handler manifest (handlers.toml):
```toml
[increment]
params = []

[set_value]
params = ["String"]
```

**Future**: Parse Rust source in build.rs using `syn` (Option B)

### Q2: Hot-Reload Without Interpretation

**Question**: Can hot-reload work with generated code?

**Resolution**: **No, by design**. Hot-reload requires runtime UI construction. This is exactly why dual-mode is essential:
- Development (`dampen run`): Interpreted mode → hot-reload works
- Production (`dampen build --release`): Codegen mode → zero overhead

Not a problem—just a documented trade-off.

### Q3: Incremental Build Performance

**Question**: Will build.rs slow down development builds?

**Mitigation**:
- Use `cargo:rerun-if-changed` per file
- Cache parsed ASTs in `OUT_DIR/cache/`
- Only regenerate changed files
- Target: <5s incremental builds with 10 .dampen files

### Q4: Type Information at Build Time

**Question**: How do we generate type-correct code without runtime type info?

**Resolution**: **Conservative generation + compiler verification**:
- Generate `.to_string()` for all display values
- Compiler verifies field access is valid
- Accept minor clippy warnings with targeted suppressions
- Trust Rust's type checker

---

## 7. Implementation Roadmap

### Phase 1: File Watching & Hot-Reload (Week 1-2)

**Deliverables**:
- [x] Create `dampen-dev` crate
- [ ] Implement `DampenWatcher` with `notify`
- [ ] Create Iced subscription for file events
- [ ] Build error overlay UI component
- [ ] Implement snapshot/restore for models
- [ ] Integration tests for hot-reload

**Success Criteria**:
- File changes detected in <100ms
- Hot-reload completes in <300ms
- State preserved across reloads
- Parse errors show overlay without crash

### Phase 2: Full Codegen Pipeline (Week 3-5)

**Deliverables**:
- [ ] Enhance `generate_binding_expr()` for direct field access
- [ ] Implement all expression types (field, binary, method, conditional)
- [ ] Add `prettyplease` formatting to build.rs
- [ ] Remove all runtime dependencies from generated code
- [ ] Add validation for invalid expressions
- [ ] Snapshot tests for all widget types

**Success Criteria**:
- All binding expressions inline correctly
- Code is clippy-clean with minimal suppressions
- Performance within 5% of hand-written baseline

### Phase 3: Mode Selection & CLI (Week 6)

**Deliverables**:
- [ ] Add feature flags to `dampen-core`
- [ ] Update `#[dampen_ui]` macro for both modes
- [ ] Implement `dampen run` command
- [ ] Implement `dampen build` command
- [ ] Update project template with dual-mode setup
- [ ] Migration guide for existing projects

**Success Criteria**:
- `cargo run` uses interpreted mode
- `cargo build --release` uses codegen mode
- No manual feature flag configuration needed
- All examples work in both modes

### Phase 4: Quality & Documentation (Week 7-8)

**Deliverables**:
- [ ] Comprehensive benchmarks (dev vs prod mode)
- [ ] Performance optimization (if needed)
- [ ] Developer documentation
- [ ] Migration guide
- [ ] Example showcases

**Success Criteria**:
- Benchmarks show 5-10x speedup in prod mode
- Documentation covers all use cases
- All examples migrated successfully

---

## 8. Risks & Mitigations

| Risk | Impact | Probability | Mitigation |
|------|--------|-------------|------------|
| Hot-reload state loss | High | Medium | Comprehensive serialization tests, fallback to default model |
| Codegen breaks bindings | High | Medium | Extensive contract tests, snapshot testing, validate against hand-written baseline |
| File watcher platform issues | Medium | Low | Use battle-tested `notify` crate, fallback to polling mode |
| Performance doesn't meet targets | Medium | Low | Early benchmarking, optimization budget in schedule |
| Incomplete widget codegen | Medium | Medium | Incremental approach, start with core widgets |

---

## 9. Dependencies Summary

### New Dependencies

```toml
# dampen-dev (NEW CRATE)
[dependencies]
notify = "6.1"
notify-debouncer-full = "0.3"
crossbeam-channel = "0.5"
futures = "0.3"
serde_json = "1.0"

# dampen-core (EXTEND)
[dependencies]
prettyplease = "0.2"  # For formatted code generation

# dampen-cli (EXTEND)
# No new dependencies
```

### Version Constraints

- Rust Edition: 2024
- MSRV: Stable (no nightly features)
- Iced: 0.14+ (already in workspace)
- All crates use workspace dependencies where applicable

---

## 10. References

### External Research

1. **notify crate**: https://docs.rs/notify/6.1 - File system watcher
2. **Iced subscriptions**: https://docs.rs/iced/0.14 - Custom subscription patterns
3. **Flutter hot reload**: https://docs.flutter.dev/tools/hot-reload - State preservation model
4. **React Fast Refresh**: https://github.com/facebook/react/tree/main/packages/react-refresh - Component identity

### Codebase Analysis

1. **dampen-core/src/codegen/**: Existing code generation infrastructure
2. **dampen-core/src/state/**: AppState structure and UiBindable trait
3. **dampen-macros/src/ui_loader.rs**: Current `#[dampen_ui]` macro
4. **examples/**: Existing patterns for model and handler definitions

### Specification Documents

1. **specs/001-dual-mode-architecture/spec.md**: Feature requirements
2. **specs/001-dual-mode-architecture/plan.md**: Implementation plan
3. **.specify/memory/constitution.md**: Constitutional principles

---

**Document Status**: ✅ Complete  
**All Research Questions Resolved**: Yes  
**Ready for Phase 1 (Design)**: Yes
