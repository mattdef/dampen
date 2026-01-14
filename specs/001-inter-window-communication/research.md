# Research: Inter-Window Communication

**Branch**: `001-inter-window-communication` | **Date**: 2026-01-14
**Purpose**: Document design decisions, alternatives analysis, and technical rationale.

---

## Research Questions

### RQ-1: How should shared state be synchronized across views?

**Context**: Views in Dampen are currently isolated with independent `Model` and `HandlerRegistry` instances. We need a mechanism for views to share data without tight coupling.

**Options Analyzed**:

| Option | Description | Pros | Cons |
|--------|-------------|------|------|
| A. Centralized Arc<RwLock<S>> | Single shared state wrapped in thread-safe container | Simple, performant, type-safe | Requires explicit type definition |
| B. Message Bus (Pub/Sub) | Views communicate via events | Loose coupling, extensible | More complex, harder to track state |
| C. Global Singleton | Static mutable state | Simple access | Not thread-safe, bad Rust practice |
| D. Hybrid (State + Messages) | Combine A and B | Best of both | More implementation effort |

**Decision**: **Option A (Centralized Arc<RwLock<S>>)** for v0.2.4

**Rationale**:
- Simplest solution that solves the immediate problem
- Type-safe: `SharedContext<S>` preserves compile-time type checking
- Thread-safe: `RwLock` allows concurrent reads with exclusive writes
- Extensible: Can add message bus later without breaking changes
- Matches existing Dampen patterns (e.g., `AppState<M>` is already generic)

Message bus functionality (Option B) is explicitly out of scope per spec and can be added in a future version.

---

### RQ-2: How should shared bindings integrate with existing XML binding syntax?

**Context**: Dampen uses `{model.field}` syntax for binding model fields to widget attributes. We need to extend this for shared state.

**Options Analyzed**:

| Option | Syntax | Pros | Cons |
|--------|--------|------|------|
| A. Prefix `shared.` | `{shared.user.name}` | Clear distinction, easy to parse | Longer syntax |
| B. Prefix `$` | `{$user.name}` | Concise | Less readable, conflicts with potential future syntax |
| C. Attribute `source="shared"` | `value="{user.name}" source="shared"` | Explicit | Verbose, requires attribute changes |
| D. Different delimiters | `[[shared.user.name]]` | Clear separation | Inconsistent with existing syntax |

**Decision**: **Option A (Prefix `shared.`)**

**Rationale**:
- Consistent with existing `{model.field}` pattern
- Self-documenting: `{shared.theme}` clearly indicates shared state access
- Easy to parse: simple string prefix check
- Extensible: leaves room for future prefixes (e.g., `{context.}`, `{env.}`)

---

### RQ-3: How should handlers access shared state?

**Context**: Handlers currently receive only the local model. We need a mechanism for handlers to read/write shared state.

**Options Analyzed**:

| Option | API | Pros | Cons |
|--------|-----|------|------|
| A. New handler variant | `register_with_shared("name", \|model, shared\| ...)` | Explicit, type-safe | New API to learn |
| B. Context parameter | `register("name", \|ctx\| ctx.model, ctx.shared)` | Single context object | Breaking change to existing handlers |
| C. Injected via macro | Handler declared, macro injects shared access | Minimal user code | Magic, harder to debug |
| D. Global accessor | `SharedContext::get()` | Simple access | Global state, not testable |

**Decision**: **Option A (New handler variant)**

**Rationale**:
- 100% backward compatible: existing `register()` calls continue to work
- Explicit: developer clearly sees which handlers use shared state
- Type-safe: shared context type is checked at compile time
- Testable: shared context can be mocked in tests

**Handler Variant Design**:

```rust
pub enum HandlerEntry {
    // Existing variants (unchanged)
    Simple(Arc<dyn Fn(&mut dyn Any) + Send + Sync>),
    WithValue(Arc<dyn Fn(&mut dyn Any, Box<dyn Any>) + Send + Sync>),
    WithCommand(Arc<dyn Fn(&mut dyn Any) -> Box<dyn Any> + Send + Sync>),
    
    // New variants for shared state
    WithShared(Arc<dyn Fn(&mut dyn Any, &dyn Any) + Send + Sync>),
    WithValueAndShared(Arc<dyn Fn(&mut dyn Any, Box<dyn Any>, &dyn Any) + Send + Sync>),
    WithCommandAndShared(Arc<dyn Fn(&mut dyn Any, &dyn Any) -> Box<dyn Any> + Send + Sync>),
}
```

---

### RQ-4: How should the macro attribute be structured?

**Context**: The `#[dampen_app]` macro needs a new attribute to specify the shared state type.

**Options Analyzed**:

| Option | Syntax | Pros | Cons |
|--------|--------|------|------|
| A. Type path | `shared_model = "SharedState"` | Simple, matches message_type pattern | Must be in scope |
| B. Module + type | `shared_model = "crate::shared::State"` | Fully qualified | Verbose |
| C. Separate attribute | `#[shared_state(SharedState)]` | Clear separation | Multiple attributes |
| D. Struct field | `shared: SharedState` in annotated struct | Follows Rust patterns | Changes struct semantics |

**Decision**: **Option A (Type path)** with support for **Option B** (fully qualified)

**Rationale**:
- Consistent with existing `message_type = "Message"` pattern
- Supports both simple names and paths: `"SharedState"` or `"shared::SharedState"`
- Optional attribute maintains backward compatibility

**Example**:

```rust
#[dampen_app(
    ui_dir = "src/ui",
    message_type = "Message",
    handler_variant = "Handler",
    shared_model = "SharedState",  // Optional, enables shared state
)]
struct MyApp;
```

---

### RQ-5: How should hot-reload preserve shared state?

**Context**: During development, hot-reload reloads XML files without restarting the app. Shared state must persist across reloads.

**Analysis**:

Current hot-reload flow:
1. File watcher detects `.dampen` file change
2. `HotReload(FileEvent)` message dispatched
3. `document` field updated in affected `AppState`
4. `model` is preserved (not reset)

Shared state integration:
- `SharedContext` is stored at app level, not in individual `AppState`
- Hot-reload only replaces `document` (XML), not state
- No additional work needed: shared state survives reload automatically

**Decision**: **No special handling needed**

**Rationale**:
- `SharedContext` lives outside the hot-reload cycle
- Same preservation mechanism as local `model`
- Test will verify: reload XML → shared state unchanged

---

### RQ-6: How should codegen mode handle shared bindings?

**Context**: Production builds compile XML to Rust code. Shared bindings must work identically in codegen mode.

**Current Codegen Flow** (for `{model.field}`):
```rust
// Generated code
fn view(&self) -> Element<Message> {
    let value = self.model.field.clone();
    text(value).into()
}
```

**Proposed Codegen Flow** (for `{shared.field}`):
```rust
// Generated code
fn view(&self) -> Element<Message> {
    let shared = self.shared_context.read();
    let value = shared.field.clone();
    drop(shared); // Release lock before building widgets
    text(value).into()
}
```

**Decision**: **Lock acquisition at view start, release before widget tree**

**Rationale**:
- Single lock acquisition per view render (performance)
- Lock released before potentially long widget construction
- Matches interpreted mode behavior (evaluates bindings, then builds)

---

## Unknowns Resolved

| Question | Resolution |
|----------|------------|
| Thread safety approach | `Arc<RwLock<S>>` with `read()` for bindings, `write()` for handlers |
| Binding syntax | `{shared.field}` prefix within existing `{...}` delimiters |
| Handler API | New `WithShared` variants, existing handlers unchanged |
| Macro attribute | `shared_model = "TypeName"`, optional |
| Hot-reload behavior | Automatic preservation, no special handling |
| Codegen approach | Lock at view start, release before widgets |

## Open Questions for Implementation

1. **Lock contention**: Should we use `try_read()` with timeout in debug mode to detect potential deadlocks?
2. **Error handling**: Should missing `{shared.field}` return empty string or trigger warning in dev mode?
3. **Change detection**: Should we track which fields changed for optimized re-renders? (Defer to future version)

---

## References

- [Dampen Constitution v1.0.0](../../.specify/memory/constitution.md)
- [Feature Specification](./spec.md)
- [Detailed Implementation Plan](../../docs/WINDOW_COMMUNICATION_PLAN.md) (French)
- [Iced Application Architecture](https://docs.rs/iced/latest/iced/)
- [Rust RwLock Documentation](https://doc.rust-lang.org/std/sync/struct.RwLock.html)
