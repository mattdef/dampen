//! Application state container for Dampen UI views.
//!
//! This module provides the [`AppState`] struct that combines a parsed UI document
//! with application state and event handlers into a cohesive structure.
//!
//! # Overview
//!
//! `AppState<M, S>` is a generic container where:
//! - `document`: The parsed [`DampenDocument`](crate::ir::DampenDocument) (mandatory)
//! - `model`: Application state model implementing [`UiBindable`](crate::binding::UiBindable) (optional, defaults to `()`)
//! - `handler_registry`: Event handler registry (optional, defaults to empty)
//! - `shared_context`: Optional reference to shared state across views (defaults to `None`)
//!
//! # Examples
//!
//! Basic usage with document only:
//!
//! ```rust,ignore
//! use dampen_core::{parse, AppState};
//!
//! let xml = r#"<column><text value="Hello!" /></column>"#;
//! let document = parse(xml).unwrap();
//! let state = AppState::new(document);
//! ```
//!
//! With a custom model:
//!
//! ```rust,ignore
//! use dampen_core::{parse, AppState};
//! use dampen_macros::UiModel;
//!
//! #[derive(UiModel, Default)]
//! struct MyModel {
//!     count: i32,
//! }
//!
//! let xml = r#"<column><text value="Hello!" /></column>"#;
//! let document = parse(xml).unwrap();
//! let state = AppState::with_model(document, MyModel { count: 0 });
//! ```
//!
//! With shared state for inter-window communication:
//!
//! ```rust,ignore
//! use dampen_core::{parse, AppState, SharedContext};
//! use dampen_macros::UiModel;
//!
//! #[derive(UiModel, Default)]
//! struct MyModel { count: i32 }
//!
//! #[derive(UiModel, Default, Clone)]
//! struct SharedState { theme: String }
//!
//! let xml = r#"<column><text value="Hello!" /></column>"#;
//! let document = parse(xml).unwrap();
//! let shared = SharedContext::new(SharedState::default());
//! let state = AppState::with_shared(document, MyModel::default(), HandlerRegistry::new(), shared);
//! ```
//!
//! # See Also
//!
//! - [`DampenDocument`](crate::ir::DampenDocument) - The parsed UI document
//! - [`HandlerRegistry`](crate::handler::HandlerRegistry) - Event handler registry
//! - [`UiBindable`](crate::binding::UiBindable) - Trait for bindable models
//! - [`SharedContext`](crate::shared::SharedContext) - Shared state container

use std::marker::PhantomData;
use std::sync::{RwLockReadGuard, RwLockWriteGuard};

use crate::shared::SharedContext;
use crate::{binding::UiBindable, handler::HandlerRegistry, ir::DampenDocument};

/// Application state container for a Dampen UI view.
///
/// This struct combines the parsed UI document with application state and event handlers.
/// It is the central state structure used throughout Dampen applications.
///
/// # Type Parameters
///
/// * `M` - The local model type implementing [`UiBindable`](crate::binding::UiBindable). Defaults to unit type `()`.
/// * `S` - The shared state type implementing [`UiBindable`](crate::binding::UiBindable) + `Send + Sync`. Defaults to unit type `()`.
///
/// # Backward Compatibility
///
/// For applications not using shared state, `S` defaults to `()` and
/// `shared_context` is `None`. All existing code continues to work unchanged.
///
/// # Fields
///
/// * `document` - The parsed UI document containing widget tree and themes
/// * `model` - Application state model for data bindings
/// * `handler_registry` - Registry of event handlers for UI interactions
/// * `shared_context` - Optional reference to shared state across views
#[derive(Debug, Clone)]
pub struct AppState<M: UiBindable = (), S: UiBindable + Send + Sync + 'static = ()> {
    /// The parsed UI document containing widget tree and themes.
    pub document: DampenDocument,

    /// Application state model for data bindings.
    /// Generic over `UiBindable` for type-safe field access.
    pub model: M,

    /// Registry of event handlers for UI interactions.
    pub handler_registry: HandlerRegistry,

    /// Optional reference to shared context for inter-window communication.
    pub shared_context: Option<SharedContext<S>>,

    /// Type marker to capture the generic parameters.
    _marker: PhantomData<(M, S)>,
}

// ============================================
// Constructors for backward compatibility (M only, S = ())
// ============================================

impl<M: UiBindable> AppState<M, ()> {
    /// Creates a new AppState with default model and empty handler registry.
    ///
    /// This is the simplest constructor for static UIs that don't use data binding
    /// or shared state.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use dampen_core::{parse, AppState};
    ///
    /// let xml = r#"<column><text value="Hello!" /></column>"#;
    /// let document = parse(xml).unwrap();
    /// let state = AppState::new(document);
    /// ```
    pub fn new(document: DampenDocument) -> Self
    where
        M: Default,
    {
        Self {
            document,
            model: M::default(),
            handler_registry: HandlerRegistry::default(),
            shared_context: None,
            _marker: PhantomData,
        }
    }

    /// Creates an AppState with a custom model and default handler registry.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use dampen_core::{parse, AppState};
    /// use dampen_macros::UiModel;
    ///
    /// #[derive(UiModel, Default)]
    /// struct MyModel {
    ///     count: i32,
    /// }
    ///
    /// let xml = r#"<column><text value="Hello!" /></column>"#;
    /// let document = parse(xml).unwrap();
    /// let model = MyModel { count: 42 };
    /// let state = AppState::with_model(document, model);
    /// ```
    pub fn with_model(document: DampenDocument, model: M) -> Self {
        Self {
            document,
            model,
            handler_registry: HandlerRegistry::default(),
            shared_context: None,
            _marker: PhantomData,
        }
    }

    /// Creates an AppState with a custom handler registry and default model.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use dampen_core::{parse, AppState, HandlerRegistry};
    ///
    /// let xml = r#"<column><text value="Hello!" /></column>"#;
    /// let document = parse(xml).unwrap();
    /// let mut registry = HandlerRegistry::new();
    /// registry.register_simple("greet", |_model| {
    ///     println!("Button clicked!");
    /// });
    /// let state = AppState::with_handlers(document, registry);
    /// ```
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

    /// Creates an AppState with custom model and handler registry.
    ///
    /// This is the most flexible constructor for apps without shared state,
    /// allowing you to specify all components of the application state.
    /// Useful for hot-reload scenarios where both model and handlers need to be specified.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use dampen_core::{parse, AppState, HandlerRegistry};
    /// use dampen_macros::UiModel;
    ///
    /// #[derive(UiModel, Default)]
    /// struct MyModel {
    ///     count: i32,
    /// }
    ///
    /// let xml = r#"<column><text value="Hello!" /></column>"#;
    /// let document = parse(xml).unwrap();
    /// let model = MyModel { count: 42 };
    /// let mut registry = HandlerRegistry::new();
    /// registry.register_simple("increment", |model| {
    ///     let model = model.downcast_mut::<MyModel>().unwrap();
    ///     model.count += 1;
    /// });
    /// let state = AppState::with_all(document, model, registry);
    /// ```
    pub fn with_all(document: DampenDocument, model: M, handler_registry: HandlerRegistry) -> Self {
        Self {
            document,
            model,
            handler_registry,
            shared_context: None,
            _marker: PhantomData,
        }
    }
}

// ============================================
// Constructors and methods for shared state (M and S)
// ============================================

impl<M: UiBindable, S: UiBindable + Send + Sync + 'static> AppState<M, S> {
    /// Creates an AppState with shared context for inter-window communication.
    ///
    /// This constructor is used when your application needs to share state
    /// across multiple views.
    ///
    /// # Arguments
    ///
    /// * `document` - Parsed UI document
    /// * `model` - Local model for this view
    /// * `handler_registry` - Event handlers for this view
    /// * `shared_context` - Shared state accessible from all views
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use dampen_core::{parse, AppState, SharedContext, HandlerRegistry};
    /// use dampen_macros::UiModel;
    ///
    /// #[derive(UiModel, Default)]
    /// struct MyModel { count: i32 }
    ///
    /// #[derive(UiModel, Default, Clone)]
    /// struct SharedState { theme: String }
    ///
    /// let xml = r#"<column><text value="{shared.theme}" /></column>"#;
    /// let document = parse(xml).unwrap();
    /// let shared = SharedContext::new(SharedState { theme: "dark".to_string() });
    ///
    /// let state = AppState::with_shared(
    ///     document,
    ///     MyModel::default(),
    ///     HandlerRegistry::new(),
    ///     shared,
    /// );
    /// ```
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
    ///
    /// Returns `None` if this AppState was created without shared context.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// if let Some(guard) = state.shared() {
    ///     println!("Current theme: {}", guard.theme);
    /// }
    /// ```
    pub fn shared(&self) -> Option<RwLockReadGuard<'_, S>> {
        self.shared_context.as_ref().map(|ctx| ctx.read())
    }

    /// Get write access to shared state (if configured).
    ///
    /// Returns `None` if this AppState was created without shared context.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// if let Some(mut guard) = state.shared_mut() {
    ///     guard.theme = "light".to_string();
    /// }
    /// ```
    pub fn shared_mut(&self) -> Option<RwLockWriteGuard<'_, S>> {
        self.shared_context.as_ref().map(|ctx| ctx.write())
    }

    /// Check if this AppState has shared context configured.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// if state.has_shared() {
    ///     // Use shared state features
    /// }
    /// ```
    pub fn has_shared(&self) -> bool {
        self.shared_context.is_some()
    }

    /// Hot-reload: updates the UI document while preserving the model, handlers, and shared context.
    ///
    /// This method is designed for development mode hot-reload scenarios where the UI
    /// definition (XML) changes but the application state (model and shared state)
    /// should be preserved.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use dampen_core::{parse, AppState};
    /// use dampen_macros::UiModel;
    ///
    /// #[derive(UiModel, Default)]
    /// struct MyModel {
    ///     count: i32,
    /// }
    ///
    /// let xml_v1 = r#"<column><text value="Old UI" /></column>"#;
    /// let document_v1 = parse(xml_v1).unwrap();
    /// let mut state = AppState::with_model(document_v1, MyModel { count: 42 });
    ///
    /// // Later, the UI file changes...
    /// let xml_v2 = r#"<column><text value="New UI" /></column>"#;
    /// let document_v2 = parse(xml_v2).unwrap();
    /// state.hot_reload(document_v2);
    ///
    /// // Model state (count: 42) is preserved
    /// assert_eq!(state.model.count, 42);
    /// ```
    pub fn hot_reload(&mut self, new_document: DampenDocument) {
        self.document = new_document;
    }
}
