//! Binding system types

/// Trait for types that expose bindable fields
pub trait UiBindable: serde::Serialize + for<'de> serde::Deserialize<'de> {
    /// Get a field value by path
    fn get_field(&self, path: &[&str]) -> Option<BindingValue>;

    /// List available field paths for error suggestions
    fn available_fields() -> Vec<String>;
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
}
