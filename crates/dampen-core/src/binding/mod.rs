//! Binding system types
//!
//! This module provides the core abstraction for data binding in Dampen.
//!
//! # Overview
//!
//! The binding system allows XML expressions like `{counter}` or `{user.name}`
//! to access fields from Rust structs at runtime.
//!
//! # Key Types
//!
//! - [`UiBindable`] - Trait implemented by models to expose fields
//! - [`BindingValue`] - Runtime value representation
//! - [`ToBindingValue`] - Trait for converting Rust types to BindingValue
//!
//! # Example
//!
//! ```rust
//! use dampen_core::{UiBindable, BindingValue};
//!
//! #[derive(Default)]
//! struct Model {
//!     count: i32,
//!     name: String,
//! }
//!
//! impl UiBindable for Model {
//!     fn get_field(&self, path: &[&str]) -> Option<BindingValue> {
//!         match path {
//!             ["count"] => Some(BindingValue::Integer(self.count as i64)),
//!             ["name"] => Some(BindingValue::String(self.name.clone())),
//!             _ => None,
//!         }
//!     }
//!
//!     fn available_fields() -> Vec<String> {
//!         vec!["count".to_string(), "name".to_string()]
//!     }
//! }
//! ```

/// Trait for types that expose bindable fields
///
/// This trait is typically derived using `#[derive(UiModel)]` from the
/// `dampen-macros` crate, but can be implemented manually for custom logic.
///
/// # Example
///
/// ```rust
/// use dampen_core::{UiBindable, BindingValue};
///
/// struct MyModel { value: i32 }
///
/// impl UiBindable for MyModel {
///     fn get_field(&self, path: &[&str]) -> Option<BindingValue> {
///         if path == ["value"] {
///             Some(BindingValue::Integer(self.value as i64))
///         } else {
///             None
///         }
///     }
///
///     fn available_fields() -> Vec<String> {
///         vec!["value".to_string()]
///     }
/// }
/// ```
pub trait UiBindable {
    /// Get a field value by path
    ///
    /// # Arguments
    ///
    /// * `path` - Array of path segments, e.g., `["user", "name"]`
    ///
    /// # Returns
    ///
    /// `Some(BindingValue)` if the field exists, `None` otherwise
    fn get_field(&self, path: &[&str]) -> Option<BindingValue>;

    /// List available field paths for error suggestions
    ///
    /// Used to provide helpful error messages when a binding references
    /// a non-existent field.
    fn available_fields() -> Vec<String>
    where
        Self: Sized;
}

/// Value returned from a binding evaluation
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum BindingValue {
    /// String value
    String(String),
    /// Integer value
    Integer(i64),
    /// Floating-point value
    Float(f64),
    /// Boolean value
    Bool(bool),
    /// List of values
    List(Vec<BindingValue>),
    /// Object/record with named fields
    Object(std::collections::HashMap<String, BindingValue>),
    /// Custom opaque value (not serializable)
    #[serde(skip)]
    Custom(std::sync::Arc<dyn std::any::Any + Send + Sync>),
    /// No value (null/none)
    None,
}

impl PartialEq for BindingValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::String(l0), Self::String(r0)) => l0 == r0,
            (Self::Integer(l0), Self::Integer(r0)) => l0 == r0,
            (Self::Float(l0), Self::Float(r0)) => l0 == r0,
            (Self::Bool(l0), Self::Bool(r0)) => l0 == r0,
            (Self::List(l0), Self::List(r0)) => l0 == r0,
            (Self::Object(l0), Self::Object(r0)) => l0 == r0,
            (Self::Custom(l0), Self::Custom(r0)) => std::sync::Arc::ptr_eq(l0, r0),
            (Self::None, Self::None) => true,
            _ => false,
        }
    }
}

impl BindingValue {
    /// Convert to display string for rendering
    ///
    /// Used when a binding value needs to be displayed as text.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dampen_core::BindingValue;
    ///
    /// let val = BindingValue::Integer(42);
    /// assert_eq!(val.to_display_string(), "42");
    /// ```
    pub fn to_display_string(&self) -> String {
        match self {
            BindingValue::String(s) => s.clone(),
            BindingValue::Integer(i) => i.to_string(),
            BindingValue::Float(f) => f.to_string(),
            BindingValue::Bool(b) => b.to_string(),
            BindingValue::List(l) => format!("[{} items]", l.len()),
            BindingValue::Object(map) => format!("{{Object with {} fields}}", map.len()),
            BindingValue::Custom(_) => "[Custom Value]".to_string(),
            BindingValue::None => String::new(),
        }
    }

    /// Convert to boolean for conditionals
    ///
    /// Used when a binding is used in a boolean context like `enabled="{condition}"`.
    ///
    /// # Truthiness Rules
    ///
    /// * `Bool(true)` → `true`
    /// * Non-empty strings → `true`
    /// * Non-zero numbers → `true`
    /// * Non-empty lists → `true`
    /// * `None` → `false`
    /// * `Custom` → `true`
    pub fn to_bool(&self) -> bool {
        match self {
            BindingValue::Bool(b) => *b,
            BindingValue::String(s) => !s.is_empty(),
            BindingValue::Integer(i) => *i != 0,
            BindingValue::Float(f) => *f != 0.0,
            BindingValue::List(l) => !l.is_empty(),
            BindingValue::Object(map) => !map.is_empty(),
            BindingValue::Custom(_) => true,
            BindingValue::None => false,
        }
    }

    /// Create BindingValue from a value
    ///
    /// Convenience method for converting types that implement `ToBindingValue`.
    pub fn from_value<T: ToBindingValue>(value: &T) -> Self {
        value.to_binding_value()
    }

    /// Get a field from an Object binding value
    ///
    /// Returns `None` if this is not an Object or the field doesn't exist.
    pub fn get_field(&self, field_name: &str) -> Option<BindingValue> {
        match self {
            BindingValue::Object(map) => map.get(field_name).cloned(),
            _ => None,
        }
    }
}

/// Trait for converting types to BindingValue
///
/// This trait is implemented for common Rust types to allow them to be
/// used in binding expressions.
///
/// # Example
///
/// ```rust
/// use dampen_core::{ToBindingValue, BindingValue};
///
/// let val = 42i32.to_binding_value();
/// assert_eq!(val, BindingValue::Integer(42));
/// ```
pub trait ToBindingValue {
    /// Convert self to a BindingValue
    fn to_binding_value(&self) -> BindingValue;
}

/// Convert `String` to `BindingValue::String`
impl ToBindingValue for String {
    fn to_binding_value(&self) -> BindingValue {
        BindingValue::String(self.clone())
    }
}

/// Convert `&str` to `BindingValue::String`
impl ToBindingValue for &str {
    fn to_binding_value(&self) -> BindingValue {
        BindingValue::String(self.to_string())
    }
}

/// Convert `i32` to `BindingValue::Integer`
impl ToBindingValue for i32 {
    fn to_binding_value(&self) -> BindingValue {
        BindingValue::Integer(*self as i64)
    }
}

/// Convert `i64` to `BindingValue::Integer`
impl ToBindingValue for i64 {
    fn to_binding_value(&self) -> BindingValue {
        BindingValue::Integer(*self)
    }
}

/// Convert `f32` to `BindingValue::Float`
impl ToBindingValue for f32 {
    fn to_binding_value(&self) -> BindingValue {
        BindingValue::Float(*self as f64)
    }
}

/// Convert `f64` to `BindingValue::Float`
impl ToBindingValue for f64 {
    fn to_binding_value(&self) -> BindingValue {
        BindingValue::Float(*self)
    }
}

/// Convert `bool` to `BindingValue::Bool`
impl ToBindingValue for bool {
    fn to_binding_value(&self) -> BindingValue {
        BindingValue::Bool(*self)
    }
}

/// Convert `Vec<T>` to `BindingValue::List`
impl<T: ToBindingValue> ToBindingValue for Vec<T> {
    fn to_binding_value(&self) -> BindingValue {
        BindingValue::List(self.iter().map(|v| v.to_binding_value()).collect())
    }
}

/// Convert `Option<T>` to `BindingValue` or `BindingValue::None`
impl<T: ToBindingValue> ToBindingValue for Option<T> {
    fn to_binding_value(&self) -> BindingValue {
        match self {
            Some(v) => v.to_binding_value(),
            None => BindingValue::None,
        }
    }
}

/// Convert `HashMap<String, T>` to `BindingValue::Object`
impl<T: ToBindingValue> ToBindingValue for std::collections::HashMap<String, T> {
    fn to_binding_value(&self) -> BindingValue {
        BindingValue::Object(
            self.iter()
                .map(|(k, v)| (k.clone(), v.to_binding_value()))
                .collect(),
        )
    }
}

/// Convert `Arc<dyn Any + Send + Sync>` to `BindingValue::Custom`
impl ToBindingValue for std::sync::Arc<dyn std::any::Any + Send + Sync> {
    fn to_binding_value(&self) -> BindingValue {
        BindingValue::Custom(self.clone())
    }
}

/// Implement UiBindable for the unit type.
///
/// This allows `AppState<()>` to be used for static UIs without a model.
impl UiBindable for () {
    fn get_field(&self, _path: &[&str]) -> Option<BindingValue> {
        None
    }

    fn available_fields() -> Vec<String> {
        vec![]
    }
}
