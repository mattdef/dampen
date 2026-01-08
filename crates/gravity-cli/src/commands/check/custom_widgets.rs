use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;

use super::errors::CheckError;

/// Configuration for a custom widget's allowed attributes.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CustomWidgetConfig {
    #[serde(default)]
    pub allowed_attributes: HashSet<String>,
}

/// Registry of custom widget configurations.
#[derive(Debug, Clone, Default)]
pub struct CustomWidgetRegistry {
    widgets: HashMap<String, CustomWidgetConfig>,
}

impl CustomWidgetRegistry {
    /// Creates a new empty registry.
    pub fn new() -> Self {
        Self {
            widgets: HashMap::new(),
        }
    }

    /// Loads a custom widget registry from a JSON file.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the JSON file containing widget configurations
    ///
    /// # Returns
    ///
    /// A `Result` containing the registry or a `CheckError` if loading fails.
    ///
    /// # Example JSON format
    ///
    /// ```json
    /// {
    ///   "CustomWidget": {
    ///     "allowed_attributes": ["value", "mode", "format"]
    ///   },
    ///   "DataGrid": {
    ///     "allowed_attributes": ["columns", "rows", "sortable"]
    ///   }
    /// }
    /// ```
    pub fn load_from_json(path: &Path) -> Result<Self, CheckError> {
        let content = fs::read_to_string(path).map_err(|e| CheckError::Io(e))?;
        let widgets: HashMap<String, CustomWidgetConfig> =
            serde_json::from_str(&content).map_err(|e| {
                CheckError::CustomWidgetConfigLoadError {
                    path: path.to_path_buf(),
                    source: e,
                }
            })?;

        Ok(Self { widgets })
    }

    /// Checks if a widget is registered.
    pub fn has_widget(&self, widget_name: &str) -> bool {
        self.widgets.contains_key(widget_name)
    }

    /// Checks if an attribute is allowed for a custom widget.
    ///
    /// # Arguments
    ///
    /// * `widget_name` - Name of the custom widget
    /// * `attribute` - Name of the attribute to check
    ///
    /// # Returns
    ///
    /// `true` if the attribute is allowed, `false` otherwise.
    /// If the widget is not registered, returns `false`.
    pub fn is_attribute_allowed(&self, widget_name: &str, attribute: &str) -> bool {
        self.widgets
            .get(widget_name)
            .map(|config| config.allowed_attributes.contains(attribute))
            .unwrap_or(false)
    }

    /// Gets all allowed attributes for a custom widget.
    ///
    /// # Arguments
    ///
    /// * `widget_name` - Name of the custom widget
    ///
    /// # Returns
    ///
    /// A vector of attribute names, or an empty vector if the widget is not registered.
    pub fn get_allowed_attributes(&self, widget_name: &str) -> Vec<&str> {
        self.widgets
            .get(widget_name)
            .map(|config| {
                config
                    .allowed_attributes
                    .iter()
                    .map(|s| s.as_str())
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Adds a custom widget configuration.
    pub fn add_widget(&mut self, name: String, config: CustomWidgetConfig) {
        self.widgets.insert(name, config);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_registry() {
        let registry = CustomWidgetRegistry::new();
        assert!(!registry.has_widget("CustomWidget"));
        assert!(!registry.is_attribute_allowed("CustomWidget", "value"));
    }

    #[test]
    fn test_add_widget() {
        let mut registry = CustomWidgetRegistry::new();
        let mut config = CustomWidgetConfig::default();
        config.allowed_attributes.insert("value".to_string());
        config.allowed_attributes.insert("mode".to_string());

        registry.add_widget("CustomWidget".to_string(), config);

        assert!(registry.has_widget("CustomWidget"));
        assert!(registry.is_attribute_allowed("CustomWidget", "value"));
        assert!(registry.is_attribute_allowed("CustomWidget", "mode"));
        assert!(!registry.is_attribute_allowed("CustomWidget", "unknown"));
    }

    #[test]
    fn test_get_allowed_attributes() {
        let mut registry = CustomWidgetRegistry::new();
        let mut config = CustomWidgetConfig::default();
        config.allowed_attributes.insert("value".to_string());
        config.allowed_attributes.insert("mode".to_string());

        registry.add_widget("CustomWidget".to_string(), config);

        let attrs = registry.get_allowed_attributes("CustomWidget");
        assert_eq!(attrs.len(), 2);
        assert!(attrs.contains(&"value"));
        assert!(attrs.contains(&"mode"));
    }

    #[test]
    fn test_get_allowed_attributes_unknown_widget() {
        let registry = CustomWidgetRegistry::new();
        let attrs = registry.get_allowed_attributes("UnknownWidget");
        assert!(attrs.is_empty());
    }
}
