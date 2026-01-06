# Data Model: AppState Structure

**Feature**: 006-auto-ui-loading
**Date**: 2026-01-06

## Overview

The `AppState` struct is the central state container for Gravity UI views. It combines the parsed UI document with application state and event handlers into a single, cohesive structure.

## AppState Definition

### Location

`gravity-core/src/state/mod.rs`

### Struct Definition

```rust
use std::marker::PhantomData;
use crate::{GravityDocument, HandlerRegistry, binding::UiBindable};

/// Application state container for a Gravity UI view.
///
/// # Type Parameters
///
/// * `M` - The model type implementing `UiBindable`. Defaults to unit type `()`.
///
/// # Examples
///
/// Basic usage with document only:
/// ```ignore
/// let state = AppState::new(document);
/// ```
///
/// With custom model:
/// ```ignore
/// #[derive(UiModel, Default)]
/// struct MyModel {
///     count: i32,
/// }
///
/// let state = AppState::with_model(document, MyModel { count: 0 });
/// ```
///
/// Full configuration:
/// ```ignore
/// let state = AppState {
///     document,
///     model: my_model,
///     handler_registry: handlers,
///     _marker: PhantomData,
/// };
/// ```
pub struct AppState<M: UiBindable = ()> {
    /// The parsed UI document containing widget tree and themes.
    pub document: GravityDocument,

    /// Application state model for data bindings.
    /// Generic over `UiBindable` for type-safe field access.
    pub model: M,

    /// Registry of event handlers for UI interactions.
    pub handler_registry: HandlerRegistry,

    /// Type marker to capture the generic parameter.
    _marker: PhantomData<M>,
}

impl<M: UiBindable> AppState<M> {
    /// Creates a new AppState with default model and empty handler registry.
    ///
    /// # Panics
    ///
    /// This constructor does not set the `document` field. Caller must initialize it.
    /// For full initialization, use struct initialization syntax or `with_*` methods.
    pub fn new(document: GravityDocument) -> Self
    where
        M: Default,
    {
        Self {
            document,
            model: M::default(),
            handler_registry: HandlerRegistry::default(),
            _marker: PhantomData,
        }
    }

    /// Creates an AppState with a custom model and default handler registry.
    pub fn with_model(document: GravityDocument, model: M) -> Self {
        Self {
            document,
            model,
            handler_registry: HandlerRegistry::default(),
            _marker: PhantomData,
        }
    }

    /// Creates an AppState with a custom handler registry and default model.
    pub fn with_handlers(document: GravityDocument, handler_registry: HandlerRegistry) -> Self
    where
        M: Default,
    {
        Self {
            document,
            model: M::default(),
            handler_registry,
            _marker: PhantomData,
        }
    }
}
```

## Entity Relationships

```
AppState<M>
    │
    ├── document: GravityDocument
    │       ├── version: SchemaVersion
    │       ├── root: WidgetNode
    │       ├── themes: HashMap<String, Theme>
    │       └── style_classes: HashMap<String, StyleClass>
    │
    ├── model: M (implements UiBindable)
    │       ├── get_field(path: &[&str]) -> Option<BindingValue>
    │       └── available_fields() -> Vec<String>
    │
    └── handler_registry: HandlerRegistry
            └── handlers: HashMap<String, HandlerEntry>
                    ├── Simple(Arc<Fn(&mut dyn Any)>)
                    ├── WithValue(Arc<Fn(&mut dyn Any, Box<dyn Any>)>)
                    └── WithCommand(Arc<Fn(&mut dyn Any) -> Box<dyn Any>>)
```

## Compatibility

### GravityWidgetBuilder

`AppState` is fully compatible with `GravityWidgetBuilder`:

```rust
fn view(state: &AppState) -> Element<'_, HandlerMessage> {
    GravityWidgetBuilder::new(
        &state.document,
        &state.model,
        Some(&state.handler_registry),
    )
    .build()
}
```

### Default Trait

`AppState<M>` implements `Default` when `M: Default`:

```rust
impl<M: UiBindable> Default for AppState<M>
where
    M: Default,
{
    fn default() -> Self {
        Self {
            document: panic!("document is required"),
            model: M::default(),
            handler_registry: HandlerRegistry::default(),
            _marker: PhantomData,
        }
    }
}
```

**Note**: The `document` field will panic if accessed before initialization. This is intentional to enforce that a valid `GravityDocument` must always be provided.

## Validation Rules

1. `document` field MUST be a valid, parsed `GravityDocument`
2. `model` MUST implement `UiBindable` trait for binding evaluation
3. `handler_registry` MAY be empty but MUST be initialized

## Migration from Manual AppState

Existing manual AppState:

```rust
struct AppState {
    model: Model,
    document: GravityDocument,
    handler_registry: HandlerRegistry,
}
```

New pattern:

```rust
type AppState = gravity_core::AppState<Model>;
```

Or with full control:

```rust
let state = AppState::<Model>::with_model(document, Model::default());
```
