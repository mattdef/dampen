//! Binding system types

/// Trait for types that expose bindable fields
pub trait UiBindable {
    /// Get a field value by path
    fn get_field(&self, path: &[&str]) -> Option<BindingValue>;

    /// List available field paths for error suggestions
    fn available_fields() -> Vec<String>
    where
        Self: Sized;
}

/// Value returned from a binding evaluation
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum BindingValue {
    String(String),
    Integer(i64),
    Float(f64),
    Bool(bool),
    List(Vec<BindingValue>),
    None,
}

impl BindingValue {
    /// Convert to display string
    pub fn to_display_string(&self) -> String {
        match self {
            BindingValue::String(s) => s.clone(),
            BindingValue::Integer(i) => i.to_string(),
            BindingValue::Float(f) => f.to_string(),
            BindingValue::Bool(b) => b.to_string(),
            BindingValue::List(l) => format!("[{} items]", l.len()),
            BindingValue::None => String::new(),
        }
    }

    /// Convert to boolean for conditionals
    pub fn to_bool(&self) -> bool {
        match self {
            BindingValue::Bool(b) => *b,
            BindingValue::String(s) => !s.is_empty(),
            BindingValue::Integer(i) => *i != 0,
            BindingValue::Float(f) => *f != 0.0,
            BindingValue::List(l) => !l.is_empty(),
            BindingValue::None => false,
        }
    }

    /// Create BindingValue from a value
    pub fn from_value<T: ToBindingValue>(value: &T) -> Self {
        value.to_binding_value()
    }
}

/// Trait for converting types to BindingValue
pub trait ToBindingValue {
    fn to_binding_value(&self) -> BindingValue;
}

impl ToBindingValue for String {
    fn to_binding_value(&self) -> BindingValue {
        BindingValue::String(self.clone())
    }
}

impl ToBindingValue for &str {
    fn to_binding_value(&self) -> BindingValue {
        BindingValue::String(self.to_string())
    }
}

impl ToBindingValue for i32 {
    fn to_binding_value(&self) -> BindingValue {
        BindingValue::Integer(*self as i64)
    }
}

impl ToBindingValue for i64 {
    fn to_binding_value(&self) -> BindingValue {
        BindingValue::Integer(*self)
    }
}

impl ToBindingValue for f32 {
    fn to_binding_value(&self) -> BindingValue {
        BindingValue::Float(*self as f64)
    }
}

impl ToBindingValue for f64 {
    fn to_binding_value(&self) -> BindingValue {
        BindingValue::Float(*self)
    }
}

impl ToBindingValue for bool {
    fn to_binding_value(&self) -> BindingValue {
        BindingValue::Bool(*self)
    }
}

impl<T: ToBindingValue> ToBindingValue for Vec<T> {
    fn to_binding_value(&self) -> BindingValue {
        BindingValue::List(self.iter().map(|v| v.to_binding_value()).collect())
    }
}

impl<T: ToBindingValue> ToBindingValue for Option<T> {
    fn to_binding_value(&self) -> BindingValue {
        match self {
            Some(v) => v.to_binding_value(),
            None => BindingValue::None,
        }
    }
}
