// Theme property validation and circular dependency detection
use crate::commands::check::errors::CheckError;
use std::collections::HashMap;
use std::path::PathBuf;

/// Information about a style class for validation
#[derive(Debug, Clone)]
struct StyleClassInfo {
    _name: String,
    extends: Vec<String>,
    file: PathBuf,
    line: u32,
    col: u32,
}

/// Information about a theme property error
#[derive(Debug, Clone)]
struct ThemePropertyError {
    theme_name: String,
    property: String,
    message: String,
    file: PathBuf,
    line: u32,
    col: u32,
}

/// Validator for theme properties and style class dependencies
#[derive(Debug, Default)]
pub struct ThemeValidator {
    style_classes: HashMap<String, StyleClassInfo>,
    property_errors: Vec<ThemePropertyError>,
}

impl ThemeValidator {
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a style class for validation
    pub fn add_style_class(
        &mut self,
        name: &str,
        extends: Vec<String>,
        file: &str,
        line: u32,
        col: u32,
    ) {
        let info = StyleClassInfo {
            _name: name.to_string(),
            extends,
            file: PathBuf::from(file),
            line,
            col,
        };
        self.style_classes.insert(name.to_string(), info);
    }

    /// Add an invalid theme property error
    pub fn add_invalid_theme_property(
        &mut self,
        theme_name: &str,
        property: &str,
        value: &str,
        file: &str,
        line: u32,
        col: u32,
    ) -> Result<(), String> {
        // For now, just record the error
        let error = ThemePropertyError {
            theme_name: theme_name.to_string(),
            property: property.to_string(),
            message: format!("Invalid value '{}' for property '{}'", value, property),
            file: PathBuf::from(file),
            line,
            col,
        };
        self.property_errors.push(error);
        Err(format!("Invalid property: {}", property))
    }

    /// Validate all themes and style classes
    pub fn validate(&self) -> Vec<CheckError> {
        let mut errors = Vec::new();

        // Add property errors
        for prop_error in &self.property_errors {
            errors.push(CheckError::InvalidThemeProperty {
                property: prop_error.property.clone(),
                theme: prop_error.theme_name.clone(),
                file: prop_error.file.clone(),
                line: prop_error.line,
                col: prop_error.col,
                message: prop_error.message.clone(),
                valid_properties: String::new(),
            });
        }

        // Check for circular dependencies
        for class_name in self.style_classes.keys() {
            let mut path = Vec::new();
            if let Err(cycle_error) = self.check_circular_dependency(class_name, &mut path) {
                errors.push(cycle_error);
            }
        }

        // Check for missing parent classes
        for (class_name, class_info) in &self.style_classes {
            for parent in &class_info.extends {
                if !self.style_classes.contains_key(parent) {
                    errors.push(CheckError::InvalidThemeProperty {
                        property: "extends".to_string(),
                        theme: class_name.clone(),
                        file: class_info.file.clone(),
                        line: class_info.line,
                        col: class_info.col,
                        message: format!("Parent class '{}' not found", parent),
                        valid_properties: self
                            .style_classes
                            .keys()
                            .cloned()
                            .collect::<Vec<_>>()
                            .join(", "),
                    });
                }
            }
        }

        errors
    }

    /// Check for circular dependencies in style class inheritance
    #[allow(clippy::result_large_err)]
    fn check_circular_dependency(
        &self,
        class_name: &str,
        path: &mut Vec<String>,
    ) -> Result<(), CheckError> {
        if path.contains(&class_name.to_string()) {
            // Found a cycle
            let cycle = format!("{} → {}", path.join(" → "), class_name);
            return Err(CheckError::ThemeCircularDependency {
                theme: class_name.to_string(),
                cycle,
            });
        }

        if let Some(class_info) = self.style_classes.get(class_name) {
            path.push(class_name.to_string());

            for parent in &class_info.extends {
                self.check_circular_dependency(parent, path)?;
            }

            path.pop();
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_circular_dependency() {
        let mut validator = ThemeValidator::new();

        // a -> b -> a
        validator.add_style_class("a", vec!["b".to_string()], "test.dampen", 1, 1);
        validator.add_style_class("b", vec!["a".to_string()], "test.dampen", 2, 1);

        let errors = validator.validate();
        assert!(!errors.is_empty());

        let has_circular = errors
            .iter()
            .any(|e| matches!(e, CheckError::ThemeCircularDependency { .. }));
        assert!(has_circular);
    }

    #[test]
    fn test_no_circular_dependency() {
        let mut validator = ThemeValidator::new();

        // a -> b -> c (no cycle)
        validator.add_style_class("a", vec!["b".to_string()], "test.dampen", 1, 1);
        validator.add_style_class("b", vec!["c".to_string()], "test.dampen", 2, 1);
        validator.add_style_class("c", vec![], "test.dampen", 3, 1);

        let errors = validator.validate();

        let has_circular = errors
            .iter()
            .any(|e| matches!(e, CheckError::ThemeCircularDependency { .. }));
        assert!(!has_circular);
    }

    #[test]
    fn test_missing_parent_class() {
        let mut validator = ThemeValidator::new();

        validator.add_style_class("a", vec!["nonexistent".to_string()], "test.dampen", 1, 1);

        let errors = validator.validate();
        assert!(!errors.is_empty());

        let has_invalid_property = errors
            .iter()
            .any(|e| matches!(e, CheckError::InvalidThemeProperty { .. }));
        assert!(has_invalid_property);
    }
}
