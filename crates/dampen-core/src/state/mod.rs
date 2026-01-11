//! Application state container for Dampen UI views.
//!
//! This module provides the [`AppState`] struct that combines a parsed UI document
//! with application state and event handlers into a cohesive structure.
//!
//! # Overview
//!
//! `AppState<M>` is a generic container where:
//! - `document`: The parsed [`DampenDocument`](crate::ir::DampenDocument) (mandatory)
//! - `model`: Application state model implementing [`UiBindable`](crate::binding::UiBindable) (optional, defaults to `()`)
//! - `handler_registry`: Event handler registry (optional, defaults to empty)
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
//! # See Also
//!
//! - [`DampenDocument`](crate::ir::DampenDocument) - The parsed UI document
//! - [`HandlerRegistry`](crate::handler::HandlerRegistry) - Event handler registry
//! - [`UiBindable`](crate::binding::UiBindable) - Trait for bindable models

use std::marker::PhantomData;

use crate::{binding::UiBindable, handler::HandlerRegistry, ir::DampenDocument};

/// Application state container for a Dampen UI view.
///
/// This struct combines the parsed UI document with application state and event handlers.
/// It is the central state structure used throughout Dampen applications.
///
/// # Type Parameters
///
/// * `M` - The model type implementing [`UiBindable`](crate::binding::UiBindable). Defaults to unit type `()`.
///
/// # Fields
///
/// * `document` - The parsed UI document containing widget tree and themes
/// * `model` - Application state model for data bindings
/// * `handler_registry` - Registry of event handlers for UI interactions
/// * `_marker` - Type marker to capture the generic parameter
#[derive(Debug, Clone)]
pub struct AppState<M: UiBindable = ()> {
    /// The parsed UI document containing widget tree and themes.
    pub document: DampenDocument,

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
            _marker: PhantomData,
        }
    }

    /// Creates an AppState with custom model and handler registry.
    ///
    /// This is the most flexible constructor, allowing you to specify all components
    /// of the application state. Useful for hot-reload scenarios where both model
    /// and handlers need to be specified.
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
            _marker: PhantomData,
        }
    }

    /// Hot-reload: updates the UI document while preserving the model and handlers.
    ///
    /// This method is designed for development mode hot-reload scenarios where the UI
    /// definition (XML) changes but the application state (model) should be preserved.
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
