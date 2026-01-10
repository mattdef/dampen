//! Configuration for code generation behavior
//!
//! This module provides configuration structures for controlling how
//! Dampen generates Rust code from XML UI definitions.

use std::path::PathBuf;

/// Configuration for code generation behavior
#[derive(Debug, Clone)]
pub struct CodegenConfig {
    /// Output directory for generated code
    pub output_dir: PathBuf,

    /// Whether to format generated code with prettyplease
    pub format_output: bool,

    /// Whether to validate generated code syntax
    pub validate_syntax: bool,

    /// Model type name (e.g., "MyModel")
    pub model_type: String,

    /// Message enum name (e.g., "Message")
    pub message_type: String,
}

impl CodegenConfig {
    /// Create a new CodegenConfig with the given output directory
    ///
    /// # Arguments
    /// * `output_dir` - Directory where generated code will be written
    ///
    /// # Returns
    /// A new CodegenConfig with default settings
    pub fn new(output_dir: PathBuf) -> Self {
        Self {
            output_dir,
            format_output: true,
            validate_syntax: true,
            model_type: "Model".to_string(),
            message_type: "Message".to_string(),
        }
    }

    /// Set the model type name
    pub fn with_model_type(mut self, model_type: impl Into<String>) -> Self {
        self.model_type = model_type.into();
        self
    }

    /// Set the message type name
    pub fn with_message_type(mut self, message_type: impl Into<String>) -> Self {
        self.message_type = message_type.into();
        self
    }

    /// Enable or disable code formatting
    pub fn with_formatting(mut self, format_output: bool) -> Self {
        self.format_output = format_output;
        self
    }

    /// Enable or disable syntax validation
    pub fn with_validation(mut self, validate_syntax: bool) -> Self {
        self.validate_syntax = validate_syntax;
        self
    }

    /// Validate the configuration
    ///
    /// # Returns
    /// Ok if configuration is valid, Err with message otherwise
    pub fn validate(&self) -> Result<(), String> {
        // Check if output_dir path is valid (we can't check writability at compile time)
        if self.output_dir.as_os_str().is_empty() {
            return Err("Output directory cannot be empty".to_string());
        }

        // Validate model type is a valid Rust identifier
        if !is_valid_identifier(&self.model_type) {
            return Err(format!(
                "Model type '{}' is not a valid Rust identifier",
                self.model_type
            ));
        }

        // Validate message type is a valid Rust identifier
        if !is_valid_identifier(&self.message_type) {
            return Err(format!(
                "Message type '{}' is not a valid Rust identifier",
                self.message_type
            ));
        }

        Ok(())
    }
}

impl Default for CodegenConfig {
    fn default() -> Self {
        Self::new(PathBuf::from("target/generated"))
    }
}

/// Check if a string is a valid Rust identifier
///
/// Valid identifiers must:
/// - Start with uppercase letter (for types)
/// - Contain only alphanumeric characters and underscores
fn is_valid_identifier(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }

    let mut chars = s.chars();

    // First character must be uppercase letter
    if let Some(first) = chars.next() {
        if !first.is_uppercase() {
            return false;
        }
    } else {
        return false;
    }

    // Remaining characters must be alphanumeric or underscore
    chars.all(|c| c.is_alphanumeric() || c == '_')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_identifiers() {
        assert!(is_valid_identifier("Model"));
        assert!(is_valid_identifier("MyModel"));
        assert!(is_valid_identifier("Model123"));
        assert!(is_valid_identifier("My_Model"));
        assert!(is_valid_identifier("M"));
    }

    #[test]
    fn test_invalid_identifiers() {
        assert!(!is_valid_identifier(""));
        assert!(!is_valid_identifier("model")); // lowercase
        assert!(!is_valid_identifier("123Model")); // starts with number
        assert!(!is_valid_identifier("My-Model")); // contains hyphen
        assert!(!is_valid_identifier("My Model")); // contains space
        assert!(!is_valid_identifier("_Model")); // starts with underscore
    }

    #[test]
    fn test_config_validation() {
        let config = CodegenConfig::default();
        assert!(config.validate().is_ok());

        let config = CodegenConfig::new(PathBuf::from("")).with_model_type("Model");
        assert!(config.validate().is_err());

        let config = CodegenConfig::new(PathBuf::from("target")).with_model_type("invalid");
        assert!(config.validate().is_err());

        let config = CodegenConfig::new(PathBuf::from("target")).with_message_type("invalid");
        assert!(config.validate().is_err());
    }
}
