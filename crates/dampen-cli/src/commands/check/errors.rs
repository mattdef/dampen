use std::path::PathBuf;
use thiserror::Error;

/// Enhanced error types for the `dampen check` command.
#[derive(Error, Debug)]
pub enum CheckError {
    // Phase 1: Unknown Attribute Detection
    #[error("Unknown attribute '{attr}' for widget '{widget}' in {file}:{line}:{col}{suggestion}")]
    UnknownAttribute {
        attr: String,
        widget: String,
        file: PathBuf,
        line: u32,
        col: u32,
        suggestion: String,
    },

    // Phase 7: Required Attribute Validation
    #[error("Missing required attribute '{attr}' for widget '{widget}' in {file}:{line}:{col}")]
    MissingRequiredAttribute {
        attr: String,
        widget: String,
        file: PathBuf,
        line: u32,
        col: u32,
    },

    // Phase 2: Handler Registry Validation
    #[error("Unknown handler '{handler}' in {file}:{line}:{col}{suggestion}")]
    UnknownHandler {
        handler: String,
        file: PathBuf,
        line: u32,
        col: u32,
        suggestion: String,
    },

    // Phase 3: Binding Validation Against Model
    #[error(
        "Invalid binding field '{field}' in {file}:{line}:{col}. Available fields: {available}"
    )]
    InvalidBindingField {
        field: String,
        file: PathBuf,
        line: u32,
        col: u32,
        available: String,
    },

    // Phase 4: Radio Group Validation
    #[error("Duplicate radio value '{value}' in group '{group}' at {file}:{line}:{col}. First occurrence: {first_file}:{first_line}:{first_col}")]
    DuplicateRadioValue {
        value: String,
        group: String,
        file: PathBuf,
        line: u32,
        col: u32,
        first_file: PathBuf,
        first_line: u32,
        first_col: u32,
    },

    #[error("Radio group '{group}' has inconsistent on_select handlers in {file}:{line}:{col}. Found handlers: {handlers}")]
    InconsistentRadioHandlers {
        group: String,
        file: PathBuf,
        line: u32,
        col: u32,
        handlers: String,
    },

    // Phase 5: Theme Property Validation
    #[error("Invalid theme property '{property}' in theme '{theme}' at {file}:{line}:{col}: {message}. Valid properties: {valid_properties}")]
    InvalidThemeProperty {
        property: String,
        theme: String,
        file: PathBuf,
        line: u32,
        col: u32,
        message: String,
        valid_properties: String,
    },

    #[error("Theme '{theme}' has circular dependency: {cycle}")]
    ThemeCircularDependency { theme: String, cycle: String },

    // JSON loading errors
    #[error("Failed to load handler registry from {path}: {source}")]
    HandlerRegistryLoadError {
        path: PathBuf,
        source: serde_json::Error,
    },

    #[error("Failed to load model info from {path}: {source}")]
    ModelInfoLoadError {
        path: PathBuf,
        source: serde_json::Error,
    },

    #[error("Failed to load custom widget config from {path}: {source}")]
    CustomWidgetConfigLoadError {
        path: PathBuf,
        source: serde_json::Error,
    },

    // Generic errors
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}
