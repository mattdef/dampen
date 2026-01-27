//! Dampen Widget Builder - Automatic interpretation of Dampen markup
//!
//! This module provides the DampenWidgetBuilder which automatically converts
//! parsed Dampen UI definitions into Iced widgets with full support for
//! bindings, events, styles, and layouts.
//!
//! # Overview
//!
//! The builder eliminates manual widget rendering by automatically:
//! - Evaluating bindings like `{count}` or `{user.name}`
//! - Connecting event handlers via `on_click`, `on_input`, etc.
//! - Applying styles and layouts from attributes
//! - Recursively processing nested widget trees
//!
//! # Basic Usage
//!
//! ```rust,ignore
//! use dampen_core::{parse, HandlerRegistry};
//! use dampen_iced::DampenWidgetBuilder;
//! use dampen_macros::UiModel;
//! use serde::{Deserialize, Serialize};
//!
//! #[derive(UiModel, Serialize, Deserialize, Clone)]
//! struct Model { count: i32 }
//!
//! let xml = r#"<text value="{count}" />"#;
//! let document = parse(xml).unwrap();
//! let model = Model { count: 42 };
//!
//! let element = DampenWidgetBuilder::new(
//!     &document,
//!     &model,
//!     None,
//! ).build();
//! ```
//!
//! # Features
//!
//! - **Automatic binding evaluation**: Supports field access, method calls, conditionals
//! - **Event handling**: Connects handlers from `HandlerRegistry`
//! - **Style application**: Applies padding, spacing, colors, borders
//! - **Layout constraints**: Handles width, height, alignment
//! - **Verbose logging**: Debug mode for development
//! - **Error handling**: Graceful degradation with logging
//!
//! # Performance
//!
//! - 100 widgets: ~0.027ms
//! - 1000 widgets: ~0.284ms
//! - Binding evaluation: ~713ns per widget
//!
//! See the [README](../README.md) for complete documentation.

// Allow eprintln! for development debugging (gated by verbose flag)
// Allow type complexity for backend abstraction (message_factory closures)
// Allow collapsible if/match for better code clarity
#![allow(
    clippy::print_stderr,
    clippy::type_complexity,
    clippy::collapsible_match,
    clippy::collapsible_else_if
)]

pub mod helpers;
mod widgets;

use crate::HandlerMessage;
use dampen_core::binding::{BindingValue, UiBindable};
use dampen_core::handler::HandlerRegistry;
use dampen_core::ir::WidgetKind;
use dampen_core::ir::node::WidgetNode;
use dampen_core::ir::theme::StyleClass;
use dampen_core::state::AppState;
use dampen_core::state::ThemeContext;
use iced::{Element, Renderer, Theme};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

/// Builder for creating Iced widgets from Dampen markup
///
/// # Construction
///
/// Use one of these constructors:
/// - [`DampenWidgetBuilder::new()`] - Standard constructor with HandlerMessage
/// - [`DampenWidgetBuilder::from_document()`] - From complete DampenDocument
/// - [`DampenWidgetBuilder::new_with_factory()`] - Custom message factory
///
/// # Configuration
///
/// After construction, chain configuration methods:
/// - [`with_style_classes()`](Self::with_style_classes) - Add theme classes
///
/// # Execution
///
/// Call [`build()`](Self::build) to render the widget tree.
///
/// # Example
///
/// ```rust,ignore
/// use dampen_core::{parse, HandlerRegistry};
/// use dampen_iced::{DampenWidgetBuilder, HandlerMessage};
/// use dampen_macros::UiModel;
/// use serde::{Deserialize, Serialize};
/// use std::any::Any;
///
/// #[derive(UiModel, Serialize, Deserialize, Clone)]
/// struct Model { count: i32 }
///
/// fn increment(model: &mut Model) { model.count += 1; }
///
/// let xml = r#"<button label="+" on_click="increment" />"#;
/// let document = parse(xml).unwrap();
/// let model = Model { count: 0 };
///
/// let registry = HandlerRegistry::new();
/// registry.register_simple("increment", |m: &mut dyn Any| {
///     let model = m.downcast_mut::<Model>().unwrap();
///     increment(model);
/// });
///
/// let element = DampenWidgetBuilder::new(
///     &document,
///     &model,
///     Some(&registry),
/// ).build();
/// ```
#[allow(dead_code)]
#[derive(Clone)]
pub struct DampenWidgetBuilder<'a> {
    /// The root widget node from parsed XML
    pub(super) node: &'a WidgetNode,

    /// Application state for binding evaluation
    pub(super) model: &'a dyn UiBindable,

    /// Optional shared context for inter-window state
    pub(super) shared_context: Option<&'a dyn UiBindable>,

    /// Optional registry for event handler lookup
    pub(super) handler_registry: Option<&'a HandlerRegistry>,

    /// Optional style classes for theme support
    pub(super) style_classes: Option<&'a HashMap<String, StyleClass>>,

    /// Optional theme context for theming support
    pub(super) theme_context: Option<&'a ThemeContext>,

    /// Factory function to create messages from handler names
    pub(super) message_factory: Rc<dyn Fn(&str, Option<String>) -> HandlerMessage + 'a>,

    /// Binding context stack for `<for>` loop variables
    /// Each context maps variable names to their BindingValues
    pub(super) binding_context: RefCell<Vec<HashMap<String, BindingValue>>>,
}

impl<'a> DampenWidgetBuilder<'a> {
    /// Create a new widget builder using the standard HandlerMessage type
    ///
    /// # Arguments
    ///
    /// * `node` - Root widget node from parsed XML
    /// * `model` - Application state implementing `UiBindable`
    /// * `handler_registry` - Optional registry for event handlers
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use dampen_core::{parse, HandlerRegistry};
    /// use dampen_iced::{DampenWidgetBuilder, HandlerMessage};
    /// use dampen_macros::UiModel;
    /// use serde::{Deserialize, Serialize};
    ///
    /// #[derive(UiModel, Serialize, Deserialize, Clone)]
    /// struct Model { count: i32 }
    ///
    /// let xml = r#"<text value="Hello" />"#;
    /// let document = parse(xml).unwrap();
    /// let model = Model { count: 0 };
    /// let registry = HandlerRegistry::new();
    ///
    /// let builder = DampenWidgetBuilder::new(
    ///     &document,
    ///     &model,
    ///     Some(&registry),
    /// );
    /// ```
    pub fn new(
        document: &'a dampen_core::DampenDocument,
        model: &'a dyn UiBindable,
        handler_registry: Option<&'a HandlerRegistry>,
    ) -> Self {
        Self {
            node: &document.root,
            model,
            shared_context: None,
            handler_registry,
            style_classes: Some(&document.style_classes),
            theme_context: None,
            message_factory: Rc::new(|name, value| {
                HandlerMessage::Handler(name.to_string(), value)
            }),
            binding_context: RefCell::new(Vec::new()),
        }
    }

    /// Create a new widget builder from a complete DampenDocument
    ///
    /// This constructor automatically extracts the root node and style classes
    /// from the document, providing a convenient way to work with parsed documents.
    ///
    /// # Arguments
    ///
    /// * `document` - Complete DampenDocument from parser
    /// * `model` - Application state implementing `UiBindable`
    /// * `handler_registry` - Optional registry for event handlers
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use dampen_core::parse;
    /// use dampen_iced::DampenWidgetBuilder;
    /// use dampen_macros::UiModel;
    /// use serde::{Deserialize, Serialize};
    ///
    /// #[derive(UiModel, Serialize, Deserialize, Clone)]
    /// struct Model { count: i32 }
    ///
    /// let xml = r#"<dampen><themes>...</themes><column>...</column></dampen>"#;
    /// let document = parse(xml).unwrap();
    /// let model = Model { count: 0 };
    ///
    /// // Builder automatically uses document.root and document.style_classes
    /// let builder = DampenWidgetBuilder::from_document(
    ///     &document,
    ///     &model,
    ///     None,
    /// );
    /// ```
    pub fn from_document(
        document: &'a dampen_core::DampenDocument,
        model: &'a dyn UiBindable,
        handler_registry: Option<&'a HandlerRegistry>,
    ) -> Self {
        Self::new(document, model, handler_registry)
    }

    /// Create a new widget builder directly from an AppState
    ///
    /// This is the most convenient constructor when working with AppState,
    /// as it automatically extracts document, model, and handler_registry.
    ///
    /// # Arguments
    ///
    /// * `app_state` - Complete AppState containing document, model, and handler registry
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use dampen_core::parse;
    /// use dampen_iced::DampenWidgetBuilder;
    /// use dampen_macros::UiModel;
    /// use serde::{Deserialize, Serialize};
    ///
    /// #[derive(UiModel, Serialize, Deserialize, Clone)]
    /// struct Model { count: i32 }
    ///
    /// let xml = r#"<column><text value="Hello!" /></column>"#;
    /// let document = parse(xml).unwrap();
    /// let app_state = AppState::with_model(document, Model { count: 0 });
    ///
    /// let builder = DampenWidgetBuilder::from_app_state(&app_state);
    /// ```
    pub fn from_app_state<M: UiBindable, S: UiBindable + Send + Sync + 'static>(
        app_state: &'a AppState<M, S>,
    ) -> Self {
        let mut builder = Self::new(
            &app_state.document,
            &app_state.model,
            Some(&app_state.handler_registry),
        );

        // Include shared context if present
        if let Some(ref shared_ctx) = app_state.shared_context {
            builder = builder.with_shared(shared_ctx as &dyn UiBindable);
        }

        // Include theme context if present
        if let Some(ref theme_ctx) = app_state.theme_context {
            builder = builder.with_theme_context(theme_ctx);
        }

        builder
    }
}

#[allow(dead_code)]
impl<'a> DampenWidgetBuilder<'a> {
    /// Create a new widget builder with a custom message factory
    ///
    /// This is useful when you need custom message types instead of HandlerMessage.
    ///
    /// # Arguments
    ///
    /// * `node` - Root widget node
    /// * `model` - Application state
    /// * `handler_registry` - Optional event handler registry
    /// * `message_factory` - Function that converts handler names to messages
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use dampen_core::{parse, HandlerRegistry};
    /// use dampen_iced::DampenWidgetBuilder;
    /// use dampen_macros::UiModel;
    /// use serde::{Deserialize, Serialize};
    ///
    /// #[derive(UiModel, Serialize, Deserialize, Clone)]
    /// struct Model { count: i32 }
    ///
    /// #[derive(Clone, Debug)]
    /// enum MyHandlerMessage {
    ///     Increment,
    ///     Decrement,
    /// }
    ///
    /// let xml = r#"<button label="+" on_click="increment" />"#;
    /// let document = parse(xml).unwrap();
    /// let model = Model { count: 0 };
    ///
    /// let builder = DampenWidgetBuilder::new_with_factory(
    ///     &document,
    ///     &model,
    ///     None,
    ///     |name| match name {
    ///         "increment" => MyHandlerMessage::Increment,
    ///         _ => MyHandlerMessage::Decrement,
    ///     },
    /// );
    /// ```
    pub fn new_with_factory<F>(
        node: &'a WidgetNode,
        model: &'a dyn UiBindable,
        handler_registry: Option<&'a HandlerRegistry>,
        message_factory: F,
    ) -> Self
    where
        F: Fn(&str, Option<String>) -> HandlerMessage + 'a,
    {
        Self {
            node,
            model,
            shared_context: None,
            handler_registry,
            style_classes: None,
            theme_context: None,
            message_factory: Rc::new(message_factory),
            binding_context: RefCell::new(Vec::new()),
        }
    }

    /// Add style classes to the builder for theme support
    ///
    /// Style classes allow reusable styling definitions that can be applied
    /// to widgets via the `class` attribute in XML.
    ///
    /// # Arguments
    ///
    /// * `style_classes` - HashMap of style class names to definitions
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use dampen_core::ir::theme::StyleClass;
    /// use std::collections::HashMap;
    ///
    /// let mut classes = HashMap::new();
    /// classes.insert("primary".to_string(), StyleClass { /* ... */ });
    ///
    /// let builder = DampenWidgetBuilder::new(/* ... */)
    ///     .with_style_classes(&classes);
    /// ```
    pub fn with_style_classes(mut self, style_classes: &'a HashMap<String, StyleClass>) -> Self {
        self.style_classes = Some(style_classes);
        self
    }

    /// Set the shared context for inter-window state bindings
    ///
    /// When a shared context is provided, bindings like `{shared.theme}`
    /// will be resolved against this context instead of the local model.
    ///
    /// # Arguments
    ///
    /// * `shared` - Reference to a shared state implementing `UiBindable`
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use dampen_core::SharedContext;
    ///
    /// struct SharedState { theme: String }
    /// // ... implement UiBindable for SharedState
    ///
    /// let shared_ctx = SharedContext::new(SharedState { theme: "dark".to_string() });
    /// let guard = shared_ctx.read();
    ///
    /// let builder = DampenWidgetBuilder::new(/* ... */)
    ///     .with_shared(&*guard);
    /// ```
    pub fn with_shared(mut self, shared: &'a dyn UiBindable) -> Self {
        self.shared_context = Some(shared);
        self
    }

    /// Set the theme context for theming support
    ///
    /// When a theme context is provided, widgets will use the active theme
    /// colors and styling from the theme.dampen file.
    ///
    /// # Arguments
    ///
    /// * `theme_context` - Reference to the theme context
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use dampen_core::ThemeContext;
    ///
    /// let theme_ctx = /* ... */;
    /// let builder = DampenWidgetBuilder::new(/* ... */)
    ///     .with_theme_context(&theme_ctx);
    /// ```
    pub fn with_theme_context(mut self, theme_context: &'a ThemeContext) -> Self {
        self.theme_context = Some(theme_context);
        self
    }

    /// Build the widget tree and return an Iced Element
    ///
    /// This is the main entry point that processes the entire widget tree,
    /// evaluates all bindings, connects events, and applies styles.
    ///
    /// # Returns
    ///
    /// An Iced `Element` ready to be used in your application's view
    ///
    /// # Type Requirements
    ///
    /// `HandlerMessage` must implement `Clone` and be `'static`
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use iced::Element;
    ///
    /// let builder = DampenWidgetBuilder::new(/* ... */);
    /// let element: Element<'_, HandlerMessage> = builder.build();
    /// ```
    pub fn build(self) -> Element<'a, HandlerMessage, Theme, Renderer>
    where
        HandlerMessage: Clone + 'static,
    {
        self.build_widget(self.node)
    }

    /// Recursively build a widget from a node
    ///
    /// This is the core dispatcher that routes to widget-specific builders
    /// based on the node's `WidgetKind`.
    ///
    /// # Arguments
    ///
    /// * `node` - Widget node to build
    ///
    /// # Returns
    ///
    /// An Iced Element representing the widget
    pub(super) fn build_widget(
        &self,
        node: &WidgetNode,
    ) -> Element<'a, HandlerMessage, Theme, Renderer>
    where
        HandlerMessage: Clone + 'static,
    {
        #[cfg(debug_assertions)]
        eprintln!("[DampenWidgetBuilder] Building widget: {:?}", node.kind);
        match node.kind {
            WidgetKind::Text => self.build_text(node),
            WidgetKind::Button => self.build_button(node),
            WidgetKind::Column => self.build_column(node),
            WidgetKind::Row => self.build_row(node),
            WidgetKind::Container => self.build_container(node),
            WidgetKind::TextInput => self.build_text_input(node),
            WidgetKind::Checkbox => self.build_checkbox(node),
            WidgetKind::Slider => self.build_slider(node),
            WidgetKind::PickList => self.build_pick_list(node),
            WidgetKind::Toggler => self.build_toggler(node),
            WidgetKind::Image => self.build_image(node),
            WidgetKind::Scrollable => self.build_scrollable(node),
            WidgetKind::Stack => self.build_stack(node),
            WidgetKind::Space => self.build_space(node),
            WidgetKind::Rule => self.build_rule(node),
            WidgetKind::Svg => self.build_svg(node),
            WidgetKind::Custom(_) => self.build_custom(node),
            WidgetKind::ComboBox => self.build_combo_box(node),
            WidgetKind::ProgressBar => self.build_progress_bar(node),
            WidgetKind::Tooltip => self.build_tooltip(node),
            WidgetKind::Grid => self.build_grid(node),
            WidgetKind::Canvas => self.build_canvas(node),
            WidgetKind::Float => self.build_float(node),
            WidgetKind::For => self.build_for(node),
            WidgetKind::If => self.build_if(node),
            WidgetKind::DatePicker => self.build_date_picker(node),
            WidgetKind::TimePicker => self.build_time_picker(node),
            WidgetKind::Menu => self.build_menu(node),
            WidgetKind::MenuItem => self.build_menu_item(node),
            WidgetKind::MenuSeparator => self.build_menu_separator(node),
            WidgetKind::ContextMenu => self.build_context_menu(node),
            WidgetKind::Radio => self.build_radio(node),
            WidgetKind::DataTable => self.build_data_table(node),
            WidgetKind::DataColumn
            | WidgetKind::CanvasRect
            | WidgetKind::CanvasCircle
            | WidgetKind::CanvasLine
            | WidgetKind::CanvasText
            | WidgetKind::CanvasGroup => iced::widget::column(Vec::new()).into(),
        }
    }
}
