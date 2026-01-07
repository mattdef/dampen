//! Application state container for Gravity UI views.
//!
//! This module provides the [`AppState`] struct that combines a parsed UI document
//! with application state and event handlers into a cohesive structure.
//!
//! # Overview
//!
//! `AppState<M>` is a generic container where:
//! - `document`: The parsed [`GravityDocument`](crate::ir::GravityDocument) (mandatory)
//! - `model`: Application state model implementing [`UiBindable`](crate::binding::UiBindable) (optional, defaults to `()`)
//! - `handler_registry`: Event handler registry (optional, defaults to empty)
//!
//! # Examples
//!
//! Basic usage with document only:
//!
//! ```rust,ignore
//! use gravity_core::{parse, AppState};
//!
//! let xml = r#"<column><text value="Hello!" /></column>"#;
//! let document = parse(xml).unwrap();
//! let state = AppState::new(document);
//! ```
//!
//! With a custom model:
//!
//! ```rust,ignore
//! use gravity_core::{parse, AppState};
//! use gravity_macros::UiModel;
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
//! - [`GravityDocument`](crate::ir::GravityDocument) - The parsed UI document
//! - [`HandlerRegistry`](crate::handler::HandlerRegistry) - Event handler registry
//! - [`UiBindable`](crate::binding::UiBindable) - Trait for bindable models

use std::marker::PhantomData;

use crate::{binding::UiBindable, handler::HandlerRegistry, ir::GravityDocument};

/// Application state container for a Gravity UI view.
///
/// This struct combines the parsed UI document with application state and event handlers.
/// It is the central state structure used throughout Gravity applications.
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
    /// # Examples
    ///
    /// ```rust,ignore
    /// use gravity_core::{parse, AppState};
    ///
    /// let xml = r#"<column><text value="Hello!" /></column>"#;
    /// let document = parse(xml).unwrap();
    /// let state = AppState::new(document);
    /// ```
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
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use gravity_core::{parse, AppState};
    /// use gravity_macros::UiModel;
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
    pub fn with_model(document: GravityDocument, model: M) -> Self {
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
    /// use gravity_core::{parse, AppState, HandlerRegistry};
    ///
    /// let xml = r#"<column><text value="Hello!" /></column>"#;
    /// let document = parse(xml).unwrap();
    /// let mut registry = HandlerRegistry::new();
    /// registry.register_simple("greet", |_model| {
    ///     println!("Button clicked!");
    /// });
    /// let state = AppState::with_handlers(document, registry);
    /// ```
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
