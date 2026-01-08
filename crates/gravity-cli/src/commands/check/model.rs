use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs;
use std::path::Path;

/// Definition of a model field.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ModelField {
    /// Field name (last part of binding path)
    pub name: String,

    /// Type hint for display purposes
    #[serde(default)]
    pub type_name: String,

    /// If true, field has nested children
    #[serde(default)]
    pub is_nested: bool,

    /// Nested field definitions for structs
    #[serde(default)]
    pub children: Vec<ModelField>,
}

/// Registry of model fields for validation.
#[derive(Debug, Clone, Default)]
pub struct ModelInfo {
    fields: HashSet<ModelField>,
}

impl ModelInfo {
    /// Creates a new empty model info.
    pub fn new() -> Self {
        Self {
            fields: HashSet::new(),
        }
    }

    /// Loads model info from a JSON file.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the JSON file containing model field definitions
    ///
    /// # Returns
    ///
    /// A `Result` containing the model info or an error if loading fails.
    ///
    /// # Example JSON format
    ///
    /// ```json
    /// [
    ///   {
    ///     "name": "count",
    ///     "type_name": "i32",
    ///     "is_nested": false,
    ///     "children": []
    ///   },
    ///   {
    ///     "name": "user",
    ///     "type_name": "User",
    ///     "is_nested": true,
    ///     "children": [
    ///       {"name": "name", "type_name": "String", "is_nested": false, "children": []},
    ///       {"name": "email", "type_name": "String", "is_nested": false, "children": []}
    ///     ]
    ///   }
    /// ]
    /// ```
    pub fn load_from_json(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let fields: Vec<ModelField> = serde_json::from_str(&content)?;

        let mut model = Self::new();
        for field in fields {
            model.add_field(field);
        }

        Ok(model)
    }

    /// Checks if a field path exists in the model.
    ///
    /// # Arguments
    ///
    /// * `path` - Slice of field names representing the path (e.g., &["user", "name"])
    ///
    /// # Returns
    ///
    /// `true` if the field path exists, `false` otherwise.
    pub fn contains_field(&self, path: &[&str]) -> bool {
        if path.is_empty() {
            return false;
        }

        // Find the top-level field
        let top_level_name = path[0];
        let top_level_field = self.fields.iter().find(|f| f.name == top_level_name);

        match top_level_field {
            None => false,
            Some(field) => {
                if path.len() == 1 {
                    // Just checking the top-level field
                    true
                } else {
                    // Need to check nested fields
                    Self::contains_nested_field(field, &path[1..])
                }
            }
        }
    }

    /// Helper function to check nested fields recursively.
    fn contains_nested_field(field: &ModelField, path: &[&str]) -> bool {
        if path.is_empty() {
            return true;
        }

        // If field is not nested, it can't have children
        if !field.is_nested {
            return false;
        }

        // Find the next field in the path
        let next_name = path[0];
        let next_field = field.children.iter().find(|f| f.name == next_name);

        match next_field {
            None => false,
            Some(child) => {
                if path.len() == 1 {
                    true
                } else {
                    Self::contains_nested_field(child, &path[1..])
                }
            }
        }
    }

    /// Gets all top-level field names.
    ///
    /// # Returns
    ///
    /// A vector of all top-level field names.
    pub fn top_level_fields(&self) -> Vec<&str> {
        self.fields.iter().map(|f| f.name.as_str()).collect()
    }

    /// Gets all available field paths as strings.
    ///
    /// # Returns
    ///
    /// A vector of all field paths (e.g., "count", "user.name").
    pub fn all_field_paths(&self) -> Vec<String> {
        let mut paths = Vec::new();

        for field in &self.fields {
            paths.push(field.name.clone());
            Self::collect_nested_paths(field, &field.name, &mut paths);
        }

        paths
    }

    /// Helper function to collect nested field paths recursively.
    fn collect_nested_paths(field: &ModelField, prefix: &str, paths: &mut Vec<String>) {
        if field.is_nested {
            for child in &field.children {
                let path = format!("{}.{}", prefix, child.name);
                paths.push(path.clone());
                Self::collect_nested_paths(child, &path, paths);
            }
        }
    }

    /// Adds a field to the model.
    pub fn add_field(&mut self, field: ModelField) {
        self.fields.insert(field);
    }

    /// Gets the number of top-level fields.
    pub fn len(&self) -> usize {
        self.fields.len()
    }

    /// Checks if the model is empty.
    pub fn is_empty(&self) -> bool {
        self.fields.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_model() {
        let model = ModelInfo::new();
        assert!(model.is_empty());
        assert_eq!(model.len(), 0);
        assert!(!model.contains_field(&["count"]));
    }

    #[test]
    fn test_add_simple_field() {
        let mut model = ModelInfo::new();

        let field = ModelField {
            name: "count".to_string(),
            type_name: "i32".to_string(),
            is_nested: false,
            children: vec![],
        };

        model.add_field(field);

        assert!(!model.is_empty());
        assert_eq!(model.len(), 1);
        assert!(model.contains_field(&["count"]));
        assert!(!model.contains_field(&["unknown"]));
    }

    #[test]
    fn test_nested_field() {
        let mut model = ModelInfo::new();

        let field = ModelField {
            name: "user".to_string(),
            type_name: "User".to_string(),
            is_nested: true,
            children: vec![ModelField {
                name: "name".to_string(),
                type_name: "String".to_string(),
                is_nested: false,
                children: vec![],
            }],
        };

        model.add_field(field);

        assert!(model.contains_field(&["user"]));
        assert!(model.contains_field(&["user", "name"]));
        assert!(!model.contains_field(&["user", "email"]));
    }

    #[test]
    fn test_top_level_fields() {
        let mut model = ModelInfo::new();

        model.add_field(ModelField {
            name: "count".to_string(),
            type_name: "i32".to_string(),
            is_nested: false,
            children: vec![],
        });

        model.add_field(ModelField {
            name: "enabled".to_string(),
            type_name: "bool".to_string(),
            is_nested: false,
            children: vec![],
        });

        let fields = model.top_level_fields();
        assert_eq!(fields.len(), 2);
        assert!(fields.contains(&"count"));
        assert!(fields.contains(&"enabled"));
    }

    #[test]
    fn test_all_field_paths() {
        let mut model = ModelInfo::new();

        model.add_field(ModelField {
            name: "count".to_string(),
            type_name: "i32".to_string(),
            is_nested: false,
            children: vec![],
        });

        model.add_field(ModelField {
            name: "user".to_string(),
            type_name: "User".to_string(),
            is_nested: true,
            children: vec![ModelField {
                name: "name".to_string(),
                type_name: "String".to_string(),
                is_nested: false,
                children: vec![],
            }],
        });

        let paths = model.all_field_paths();
        assert!(paths.contains(&"count".to_string()));
        assert!(paths.contains(&"user".to_string()));
        assert!(paths.contains(&"user.name".to_string()));
    }
}
