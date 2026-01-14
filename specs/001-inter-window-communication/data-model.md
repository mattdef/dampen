# Data Model: Inter-Window Communication

**Branch**: `001-inter-window-communication` | **Date**: 2026-01-14
**Purpose**: Define the core types, traits, and data structures for shared state.

---

## Entity Overview

```
┌─────────────────────────────────────────────────────────────────────────┐
│                           Application Layer                              │
│  ┌────────────────────────────────────────────────────────────────────┐ │
│  │                    SharedContext<S>                                 │ │
│  │  ┌──────────────────────────────────────────────────────────────┐  │ │
│  │  │  state: Arc<RwLock<S>>                                        │  │ │
│  │  │  where S: UiBindable + Send + Sync + 'static                  │  │ │
│  │  └──────────────────────────────────────────────────────────────┘  │ │
│  └────────────────────────────────────────────────────────────────────┘ │
│                              ▲                                           │
│                    (cloned reference)                                    │
│              ┌───────────────┴───────────────┐                          │
│              ▼                               ▼                          │
│  ┌────────────────────┐         ┌────────────────────┐                 │
│  │ AppState<M, S>     │         │ AppState<N, S>     │                 │
│  │ ├── document       │         │ ├── document       │                 │
│  │ ├── model: M       │         │ ├── model: N       │                 │
│  │ ├── handlers       │         │ ├── handlers       │                 │
│  │ └── shared_ctx ────┼─────────┼─► shared_ctx       │                 │
│  └────────────────────┘         └────────────────────┘                 │
│         View 1                         View 2                           │
└─────────────────────────────────────────────────────────────────────────┘
```

---

## Core Types

### SharedContext<S>

**Location**: `crates/dampen-core/src/shared/mod.rs`

**Purpose**: Thread-safe container for application-wide shared state. Cloneable reference that can be passed to all views.

```rust
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};
use crate::binding::UiBindable;

/// Thread-safe shared state container.
///
/// `SharedContext` wraps user-defined shared state in an `Arc<RwLock<S>>`,
/// enabling safe concurrent access from multiple views. Each view receives
/// a cloned reference to the same underlying state.
///
/// # Type Parameters
///
/// * `S` - The shared state type. Must implement:
///   - `UiBindable` for XML binding access
///   - `Send + Sync` for thread safety
///   - `'static` for Arc storage
///
/// # Example
///
/// ```rust
/// use dampen_core::SharedContext;
/// use dampen_macros::UiModel;
///
/// #[derive(Default, Clone, UiModel)]
/// struct SharedState {
///     theme: String,
///     user_name: Option<String>,
/// }
///
/// let ctx = SharedContext::new(SharedState::default());
/// let ctx2 = ctx.clone(); // Same underlying state
///
/// ctx.write().theme = "dark".to_string();
/// assert_eq!(ctx2.read().theme, "dark");
/// ```
#[derive(Debug)]
pub struct SharedContext<S>
where
    S: UiBindable + Send + Sync + 'static,
{
    state: Arc<RwLock<S>>,
}

impl<S> SharedContext<S>
where
    S: UiBindable + Send + Sync + 'static,
{
    /// Create a new SharedContext with initial state.
    pub fn new(initial: S) -> Self {
        Self {
            state: Arc::new(RwLock::new(initial)),
        }
    }

    /// Acquire read access to shared state.
    ///
    /// # Panics
    ///
    /// Panics if the lock is poisoned (a thread panicked while holding write lock).
    pub fn read(&self) -> RwLockReadGuard<'_, S> {
        self.state.read().expect("SharedContext lock poisoned")
    }

    /// Acquire write access to shared state.
    ///
    /// # Panics
    ///
    /// Panics if the lock is poisoned.
    pub fn write(&self) -> RwLockWriteGuard<'_, S> {
        self.state.write().expect("SharedContext lock poisoned")
    }

    /// Try to acquire read access without blocking.
    ///
    /// Returns `None` if the lock is currently held for writing.
    pub fn try_read(&self) -> Option<RwLockReadGuard<'_, S>> {
        self.state.try_read().ok()
    }

    /// Try to acquire write access without blocking.
    ///
    /// Returns `None` if the lock is currently held.
    pub fn try_write(&self) -> Option<RwLockWriteGuard<'_, S>> {
        self.state.try_write().ok()
    }
}

impl<S> Clone for SharedContext<S>
where
    S: UiBindable + Send + Sync + 'static,
{
    fn clone(&self) -> Self {
        Self {
            state: Arc::clone(&self.state),
        }
    }
}

// Unit struct for when shared state is not used
impl SharedContext<()> {
    /// Create an empty shared context (no-op).
    pub fn empty() -> Self {
        Self::new(())
    }
}
```

**Field Descriptions**:

| Field | Type | Description |
|-------|------|-------------|
| `state` | `Arc<RwLock<S>>` | Thread-safe reference-counted pointer to the shared state with read-write locking |

---

### HandlerEntry (Extended)

**Location**: `crates/dampen-core/src/handler/mod.rs`

**Purpose**: Enum representing different handler signatures. Extended with variants that receive shared context.

```rust
use std::any::Any;
use std::sync::Arc;

/// Handler function entry types.
///
/// Each variant represents a different handler signature, enabling
/// type-safe dispatch while maintaining runtime flexibility.
pub enum HandlerEntry {
    // ============================================
    // Existing variants (unchanged for compatibility)
    // ============================================
    
    /// Simple handler: `fn(&mut Model)`
    Simple(Arc<dyn Fn(&mut dyn Any) + Send + Sync>),
    
    /// Handler with value: `fn(&mut Model, String)`
    WithValue(Arc<dyn Fn(&mut dyn Any, Box<dyn Any>) + Send + Sync>),
    
    /// Handler returning command: `fn(&mut Model) -> Command`
    WithCommand(Arc<dyn Fn(&mut dyn Any) -> Box<dyn Any> + Send + Sync>),
    
    // ============================================
    // New variants for shared state access
    // ============================================
    
    /// Handler with shared context: `fn(&mut Model, &SharedContext<S>)`
    ///
    /// Use when the handler needs to read or write shared state.
    WithShared(Arc<dyn Fn(&mut dyn Any, &dyn Any) + Send + Sync>),
    
    /// Handler with value and shared: `fn(&mut Model, String, &SharedContext<S>)`
    ///
    /// Use when the handler receives input value and needs shared state.
    WithValueAndShared(Arc<dyn Fn(&mut dyn Any, Box<dyn Any>, &dyn Any) + Send + Sync>),
    
    /// Handler with command and shared: `fn(&mut Model, &SharedContext<S>) -> Command`
    ///
    /// Use when the handler needs shared state and returns a command.
    WithCommandAndShared(Arc<dyn Fn(&mut dyn Any, &dyn Any) -> Box<dyn Any> + Send + Sync>),
}
```

**Variant Summary**:

| Variant | Signature | Use Case |
|---------|-----------|----------|
| `Simple` | `fn(&mut M)` | Button click, simple action |
| `WithValue` | `fn(&mut M, String)` | Text input, slider value |
| `WithCommand` | `fn(&mut M) -> Cmd` | Async operations |
| `WithShared` | `fn(&mut M, &Shared)` | Update shared preferences |
| `WithValueAndShared` | `fn(&mut M, String, &Shared)` | Input that updates shared state |
| `WithCommandAndShared` | `fn(&mut M, &Shared) -> Cmd` | Async with shared state access |

---

### HandlerRegistry (Extended Methods)

**Location**: `crates/dampen-core/src/handler/mod.rs`

**Purpose**: Registry for storing and dispatching handlers. Extended with methods for shared-aware handlers.

```rust
impl HandlerRegistry {
    // ============================================
    // Existing methods (unchanged)
    // ============================================
    
    pub fn new() -> Self { /* ... */ }
    pub fn register<F>(&self, name: &str, handler: F) { /* ... */ }
    pub fn register_with_value<F>(&self, name: &str, handler: F) { /* ... */ }
    pub fn register_with_command<F>(&self, name: &str, handler: F) { /* ... */ }
    pub fn get(&self, name: &str) -> Option<HandlerEntry> { /* ... */ }
    
    // ============================================
    // New methods for shared state handlers
    // ============================================
    
    /// Register a handler that receives shared context.
    ///
    /// # Example
    ///
    /// ```rust
    /// registry.register_with_shared("update_theme", |model, shared| {
    ///     let model = model.downcast_mut::<Model>().unwrap();
    ///     let shared = shared.downcast_ref::<SharedContext<SharedState>>().unwrap();
    ///     shared.write().theme = model.selected_theme.clone();
    /// });
    /// ```
    pub fn register_with_shared<F>(&self, name: &str, handler: F)
    where
        F: Fn(&mut dyn Any, &dyn Any) + Send + Sync + 'static,
    {
        if let Ok(mut handlers) = self.handlers.write() {
            handlers.insert(
                name.to_string(),
                HandlerEntry::WithShared(Arc::new(handler)),
            );
        }
    }
    
    /// Register a handler with value and shared context.
    pub fn register_with_value_and_shared<F>(&self, name: &str, handler: F)
    where
        F: Fn(&mut dyn Any, Box<dyn Any>, &dyn Any) + Send + Sync + 'static,
    {
        if let Ok(mut handlers) = self.handlers.write() {
            handlers.insert(
                name.to_string(),
                HandlerEntry::WithValueAndShared(Arc::new(handler)),
            );
        }
    }
    
    /// Register a command handler with shared context.
    pub fn register_with_command_and_shared<F>(&self, name: &str, handler: F)
    where
        F: Fn(&mut dyn Any, &dyn Any) -> Box<dyn Any> + Send + Sync + 'static,
    {
        if let Ok(mut handlers) = self.handlers.write() {
            handlers.insert(
                name.to_string(),
                HandlerEntry::WithCommandAndShared(Arc::new(handler)),
            );
        }
    }
    
    /// Dispatch a handler with shared context.
    ///
    /// Handles all handler variants, passing shared context only to
    /// variants that expect it. Returns command output if applicable.
    pub fn dispatch_with_shared(
        &self,
        handler_name: &str,
        model: &mut dyn Any,
        shared: &dyn Any,
        value: Option<String>,
    ) -> Option<Box<dyn Any>> {
        let entry = self.get(handler_name)?;
        
        match entry {
            // Existing variants (backward compatible)
            HandlerEntry::Simple(h) => {
                h(model);
                None
            }
            HandlerEntry::WithValue(h) => {
                h(model, Box::new(value.unwrap_or_default()));
                None
            }
            HandlerEntry::WithCommand(h) => {
                Some(h(model))
            }
            
            // New shared variants
            HandlerEntry::WithShared(h) => {
                h(model, shared);
                None
            }
            HandlerEntry::WithValueAndShared(h) => {
                h(model, Box::new(value.unwrap_or_default()), shared);
                None
            }
            HandlerEntry::WithCommandAndShared(h) => {
                Some(h(model, shared))
            }
        }
    }
}
```

---

### AppState<M, S> (Extended)

**Location**: `crates/dampen-core/src/state/mod.rs`

**Purpose**: Per-view state container. Extended with optional reference to shared context.

```rust
use std::marker::PhantomData;
use crate::binding::UiBindable;
use crate::handler::HandlerRegistry;
use crate::ir::DampenDocument;
use crate::shared::SharedContext;

/// Application state for a single view.
///
/// # Type Parameters
///
/// * `M` - Local model type for this view
/// * `S` - Shared state type (use `()` when not using shared state)
///
/// # Backward Compatibility
///
/// For applications not using shared state, `S` defaults to `()` and
/// `shared_context` is `None`. All existing code continues to work.
pub struct AppState<M: UiBindable = (), S: UiBindable + Send + Sync + 'static = ()> {
    /// Parsed XML document (IR)
    pub document: DampenDocument,
    
    /// Local model for this view
    pub model: M,
    
    /// Handler registry for this view
    pub handler_registry: HandlerRegistry,
    
    /// Optional reference to shared context
    pub shared_context: Option<SharedContext<S>>,
    
    _marker: PhantomData<S>,
}

impl<M: UiBindable> AppState<M, ()> {
    // Existing constructors (unchanged, backward compatible)
    
    pub fn new(document: DampenDocument) -> Self
    where
        M: Default,
    {
        Self {
            document,
            model: M::default(),
            handler_registry: HandlerRegistry::new(),
            shared_context: None,
            _marker: PhantomData,
        }
    }
    
    pub fn with_model(document: DampenDocument, model: M) -> Self {
        Self {
            document,
            model,
            handler_registry: HandlerRegistry::new(),
            shared_context: None,
            _marker: PhantomData,
        }
    }
    
    pub fn with_handlers(document: DampenDocument, handler_registry: HandlerRegistry) -> Self
    where
        M: Default,
    {
        Self {
            document,
            model: M::default(),
            handler_registry,
            shared_context: None,
            _marker: PhantomData,
        }
    }
}

impl<M: UiBindable, S: UiBindable + Send + Sync + 'static> AppState<M, S> {
    // New constructors for shared state
    
    /// Create AppState with shared context.
    pub fn with_shared(
        document: DampenDocument,
        model: M,
        handler_registry: HandlerRegistry,
        shared_context: SharedContext<S>,
    ) -> Self {
        Self {
            document,
            model,
            handler_registry,
            shared_context: Some(shared_context),
            _marker: PhantomData,
        }
    }
    
    /// Get read access to shared state (if configured).
    pub fn shared(&self) -> Option<std::sync::RwLockReadGuard<'_, S>> {
        self.shared_context.as_ref().map(|ctx| ctx.read())
    }
    
    /// Get write access to shared state (if configured).
    pub fn shared_mut(&self) -> Option<std::sync::RwLockWriteGuard<'_, S>> {
        self.shared_context.as_ref().map(|ctx| ctx.write())
    }
}
```

---

### SharedBinding (Expression Type)

**Location**: `crates/dampen-core/src/expr/ast.rs`

**Purpose**: AST node representing a shared state binding in XML.

```rust
/// Binding expression variants.
#[derive(Debug, Clone, PartialEq)]
pub enum BindingExpr {
    // Existing variants
    Literal(String),
    FieldAccess(Vec<String>),          // {model.field.subfield}
    MethodCall(Box<Self>, String),     // {model.items.len()}
    Binary(Box<Self>, BinaryOp, Box<Self>),
    Conditional(Box<Self>, Box<Self>, Box<Self>),
    
    // New variant for shared state
    SharedFieldAccess(Vec<String>),    // {shared.field.subfield}
}

impl BindingExpr {
    /// Check if this expression accesses shared state.
    pub fn uses_shared(&self) -> bool {
        match self {
            Self::SharedFieldAccess(_) => true,
            Self::MethodCall(inner, _) => inner.uses_shared(),
            Self::Binary(left, _, right) => left.uses_shared() || right.uses_shared(),
            Self::Conditional(cond, then, else_) => {
                cond.uses_shared() || then.uses_shared() || else_.uses_shared()
            }
            _ => false,
        }
    }
}
```

---

## Type Relationships

```
UiBindable (trait)
    │
    ├── implemented by user's Model types
    │
    └── implemented by user's SharedState type
            │
            └── wrapped by SharedContext<S>
                    │
                    ├── cloned into each AppState<M, S>
                    │
                    └── passed to WithShared handler variants
                            │
                            └── dispatched by HandlerRegistry
```

---

## Validation Rules

| Rule | Description | Error |
|------|-------------|-------|
| V-001 | `shared_model` type must implement `UiBindable` | Compile error: trait bound not satisfied |
| V-002 | `shared_model` type must implement `Send + Sync` | Compile error: trait bound not satisfied |
| V-003 | `{shared.field}` must reference existing field | Runtime: empty value (dev: warning) |
| V-004 | Handler downcasts must succeed | Runtime panic with clear message |

---

## Default Values

| Type | Default |
|------|---------|
| `SharedContext<()>` | Empty context (no-op) |
| `AppState.shared_context` | `None` when not configured |
| Missing `{shared.field}` | Empty string |

---

## Serialization (Future)

Currently out of scope, but the design supports future serialization:

```rust
// Future: Serialize shared state for persistence
impl<S: Serialize> SharedContext<S> {
    pub fn to_json(&self) -> serde_json::Result<String> {
        serde_json::to_string(&*self.read())
    }
}
```
