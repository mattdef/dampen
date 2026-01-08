use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs;
use std::path::Path;

/// Definition of a handler function.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct HandlerDefinition {
    /// Handler function name as referenced in XML `on_*` attributes
    pub name: String,

    /// Expected message type; null for unit/no params
    #[serde(default)]
    pub param_type: Option<String>,

    /// Whether handler returns Command for async operations
    #[serde(default)]
    pub returns_command: bool,
}

/// Registry of event handlers for validation.
#[derive(Debug, Clone, Default)]
pub struct HandlerRegistry {
    handlers: HashSet<HandlerDefinition>,
}

impl HandlerRegistry {
    /// Creates a new empty registry.
    pub fn new() -> Self {
        Self {
            handlers: HashSet::new(),
        }
    }

    /// Loads a handler registry from a JSON file.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the JSON file containing handler definitions
    ///
    /// # Returns
    ///
    /// A `Result` containing the registry or an error if loading fails.
    ///
    /// # Example JSON format
    ///
    /// ```json
    /// [
    ///   {
    ///     "name": "increment",
    ///     "param_type": null,
    ///     "returns_command": false
    ///   },
    ///   {
    ///     "name": "setValue",
    ///     "param_type": "i32",
    ///     "returns_command": true
    ///   }
    /// ]
    /// ```
    pub fn load_from_json(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let handlers: Vec<HandlerDefinition> = serde_json::from_str(&content)?;

        let mut registry = Self::new();
        for handler in handlers {
            registry.add_handler(handler);
        }

        Ok(registry)
    }

    /// Checks if a handler is registered.
    ///
    /// # Arguments
    ///
    /// * `name` - Handler name to check
    ///
    /// # Returns
    ///
    /// `true` if the handler is registered, `false` otherwise.
    pub fn contains(&self, name: &str) -> bool {
        self.handlers.iter().any(|h| h.name == name)
    }

    /// Gets all handler names.
    ///
    /// # Returns
    ///
    /// A vector of all registered handler names.
    pub fn all_names(&self) -> Vec<String> {
        self.handlers.iter().map(|h| h.name.clone()).collect()
    }

    /// Adds a handler to the registry.
    pub fn add_handler(&mut self, handler: HandlerDefinition) {
        self.handlers.insert(handler);
    }

    /// Gets the number of registered handlers.
    pub fn len(&self) -> usize {
        self.handlers.len()
    }

    /// Checks if the registry is empty.
    pub fn is_empty(&self) -> bool {
        self.handlers.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_registry() {
        let registry = HandlerRegistry::new();
        assert!(registry.is_empty());
        assert_eq!(registry.len(), 0);
        assert!(!registry.contains("increment"));
    }

    #[test]
    fn test_add_handler() {
        let mut registry = HandlerRegistry::new();

        let handler = HandlerDefinition {
            name: "increment".to_string(),
            param_type: None,
            returns_command: false,
        };

        registry.add_handler(handler);

        assert!(!registry.is_empty());
        assert_eq!(registry.len(), 1);
        assert!(registry.contains("increment"));
        assert!(!registry.contains("decrement"));
    }

    #[test]
    fn test_all_names() {
        let mut registry = HandlerRegistry::new();

        registry.add_handler(HandlerDefinition {
            name: "increment".to_string(),
            param_type: None,
            returns_command: false,
        });

        registry.add_handler(HandlerDefinition {
            name: "decrement".to_string(),
            param_type: None,
            returns_command: false,
        });

        let names = registry.all_names();
        assert_eq!(names.len(), 2);
        assert!(names.contains(&"increment".to_string()));
        assert!(names.contains(&"decrement".to_string()));
    }

    #[test]
    fn test_duplicate_handlers() {
        let mut registry = HandlerRegistry::new();

        let handler = HandlerDefinition {
            name: "increment".to_string(),
            param_type: None,
            returns_command: false,
        };

        registry.add_handler(handler.clone());
        registry.add_handler(handler.clone());

        // HashSet should prevent duplicates
        assert_eq!(registry.len(), 1);
    }
}
