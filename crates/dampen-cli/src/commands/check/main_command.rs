#![allow(clippy::print_stderr, clippy::print_stdout)]

//! Check command - validates Dampen UI files

use clap::Args;
use dampen_core::ir::layout::{Direction, Position};
use dampen_core::{
    ir::{AttributeValue, EventKind, WidgetKind},
    parser,
    parser::style_parser,
};
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use thiserror::Error;
use walkdir::WalkDir;

#[derive(Error, Debug)]
pub enum CheckError {
    #[error("Directory not found: {0}")]
    DirectoryNotFound(PathBuf),

    #[error("Parse error in {file}:{line}:{col}: {message}")]
    ParseError {
        file: PathBuf,
        line: u32,
        col: u32,
        message: String,
    },

    #[error("XML validation error in {file}:{line}:{col}: {message}")]
    XmlValidationError {
        file: PathBuf,
        line: u32,
        col: u32,
        message: String,
    },

    #[error("Invalid widget '{widget}' in {file}:{line}:{col}")]
    InvalidWidget {
        widget: String,
        file: PathBuf,
        line: u32,
        col: u32,
    },

    #[error("Unknown attribute '{attr}' for widget '{widget}' in {file}:{line}:{col}{suggestion}")]
    UnknownAttribute {
        attr: String,
        widget: String,
        file: PathBuf,
        line: u32,
        col: u32,
        suggestion: String,
    },

    #[error("Unknown handler '{handler}' in {file}:{line}:{col}{suggestion}")]
    UnknownHandler {
        handler: String,
        file: PathBuf,
        line: u32,
        col: u32,
        suggestion: String,
    },

    #[error("Invalid binding field '{field}' in {file}:{line}:{col}")]
    InvalidBinding {
        field: String,
        file: PathBuf,
        line: u32,
        col: u32,
    },

    #[error("Invalid style attribute '{attr}' in {file}:{line}:{col}: {message}")]
    InvalidStyleAttribute {
        attr: String,
        file: PathBuf,
        line: u32,
        col: u32,
        message: String,
    },

    #[error("Invalid state prefix '{prefix}' in {file}:{line}:{col}")]
    InvalidStatePrefix {
        prefix: String,
        file: PathBuf,
        line: u32,
        col: u32,
    },

    #[error("Invalid style value for '{attr}' in {file}:{line}:{col}: {message}")]
    InvalidStyleValue {
        attr: String,
        file: PathBuf,
        line: u32,
        col: u32,
        message: String,
    },

    #[error("Invalid layout constraint in {file}:{line}:{col}: {message}")]
    InvalidLayoutConstraint {
        file: PathBuf,
        line: u32,
        col: u32,
        message: String,
    },

    #[error("Unknown theme '{theme}' referenced in {file}:{line}:{col}")]
    UnknownTheme {
        theme: String,
        file: PathBuf,
        line: u32,
        col: u32,
    },

    #[error("Unknown style class '{class}' referenced in {file}:{line}:{col}")]
    UnknownStyleClass {
        class: String,
        file: PathBuf,
        line: u32,
        col: u32,
    },

    #[error("Invalid breakpoint attribute '{attr}' in {file}:{line}:{col}")]
    InvalidBreakpoint {
        attr: String,
        file: PathBuf,
        line: u32,
        col: u32,
    },

    #[error("Invalid state attribute '{attr}' in {file}:{line}:{col}")]
    InvalidState {
        attr: String,
        file: PathBuf,
        line: u32,
        col: u32,
    },

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

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

#[derive(Args)]
pub struct CheckArgs {
    /// Directory containing .dampen files (default: auto-detect src/ui or ui)
    #[arg(short, long)]
    pub input: Option<String>,

    /// Enable verbose output
    #[arg(short, long)]
    pub verbose: bool,

    /// Path to handler registry JSON (default: auto-discover handlers.json)
    #[arg(long)]
    pub handlers: Option<String>,

    /// Path to model info JSON (default: auto-discover model.json)
    #[arg(long)]
    pub model: Option<String>,

    /// Path to custom widget configuration JSON file
    #[arg(long)]
    pub custom_widgets: Option<String>,

    /// Treat warnings as errors (strict mode for CI/CD)
    #[arg(long)]
    pub strict: bool,

    /// Show minimum required schema version for each widget type
    #[arg(long)]
    pub show_widget_versions: bool,
}

/// Resolves the UI directory path using smart detection
fn resolve_ui_directory(explicit_input: Option<&str>) -> Result<PathBuf, String> {
    // If explicitly provided, use it
    if let Some(path) = explicit_input {
        let path_buf = PathBuf::from(path);
        if path_buf.exists() {
            return Ok(path_buf);
        } else {
            return Err(format!("Specified UI directory does not exist: {}", path));
        }
    }

    // Try src/ui/ (Rust convention)
    let src_ui = PathBuf::from("src/ui");
    if src_ui.exists() && src_ui.is_dir() {
        return Ok(src_ui);
    }

    // Try ui/ (fallback)
    let ui = PathBuf::from("ui");
    if ui.exists() && ui.is_dir() {
        return Ok(ui);
    }

    // None found
    Err("No UI directory found. Please create one of:\n\
         - src/ui/ (recommended for Rust projects)\n\
         - ui/ (general purpose)\n\n\
         Or specify a custom path with --input:\n\
         dampen check --input path/to/ui"
        .to_string())
}

/// Resolves optional file paths with auto-discovery
fn resolve_optional_file(explicit_path: Option<&str>, filename: &str) -> Option<PathBuf> {
    // If explicitly provided, use it
    if let Some(path) = explicit_path {
        let path_buf = PathBuf::from(path);
        if path_buf.exists() {
            return Some(path_buf);
        }
        // Note: If explicit path doesn't exist, we'll let the caller handle the error
        return Some(path_buf);
    }

    // Try project root
    let root_file = PathBuf::from(filename);
    if root_file.exists() {
        return Some(root_file);
    }

    // Try src/ directory
    let src_file = PathBuf::from("src").join(filename);
    if src_file.exists() {
        return Some(src_file);
    }

    // Not found - this is OK for optional files
    None
}

/// Display a table of all widget types with their minimum schema version requirements
fn display_widget_version_table() {
    println!("Widget Version Requirements");
    println!("===========================\n");
    println!("{:<20} {:<10} Status", "Widget", "Min Version");
    println!("{:-<20} {:-<10} {:-<30}", "", "", "");

    let widgets = vec![
        ("column", WidgetKind::Column),
        ("row", WidgetKind::Row),
        ("container", WidgetKind::Container),
        ("scrollable", WidgetKind::Scrollable),
        ("stack", WidgetKind::Stack),
        ("text", WidgetKind::Text),
        ("image", WidgetKind::Image),
        ("svg", WidgetKind::Svg),
        ("button", WidgetKind::Button),
        ("text_input", WidgetKind::TextInput),
        ("checkbox", WidgetKind::Checkbox),
        ("slider", WidgetKind::Slider),
        ("pick_list", WidgetKind::PickList),
        ("toggler", WidgetKind::Toggler),
        ("radio", WidgetKind::Radio),
        ("space", WidgetKind::Space),
        ("rule", WidgetKind::Rule),
        ("progress_bar", WidgetKind::ProgressBar),
        ("combobox", WidgetKind::ComboBox),
        ("tooltip", WidgetKind::Tooltip),
        ("grid", WidgetKind::Grid),
        ("canvas", WidgetKind::Canvas),
        ("float", WidgetKind::Float),
    ];

    for (name, widget) in widgets {
        let min_version = widget.minimum_version();
        let version_str = format!("{}.{}", min_version.major, min_version.minor);
        let status = if min_version.minor > 0 {
            "Experimental (not fully functional)"
        } else {
            "Stable"
        };
        println!("{:<20} {:<10} {}", name, version_str, status);
    }

    println!("\nNote: Widgets requiring v1.1+ are experimental and may not be fully functional.");
    println!("Use 'dampen check' to validate your .dampen files for version compatibility.");
}

pub fn execute(args: &CheckArgs) -> Result<(), CheckError> {
    use crate::commands::check::handlers::HandlerRegistry;

    // If --show-widget-versions flag is set, display widget version table and exit
    if args.show_widget_versions {
        display_widget_version_table();
        return Ok(());
    }

    // Resolve UI directory
    let input_path = resolve_ui_directory(args.input.as_deref())
        .map_err(|msg| CheckError::Io(std::io::Error::new(std::io::ErrorKind::NotFound, msg)))?;

    if args.verbose {
        eprintln!("Using UI directory: {}", input_path.display());
    }

    // Resolve optional files
    let handlers_path = resolve_optional_file(args.handlers.as_deref(), "handlers.json");
    if args.verbose {
        if let Some(ref path) = handlers_path {
            eprintln!("Using handler registry: {}", path.display());
        }
    }

    let model_path = resolve_optional_file(args.model.as_deref(), "model.json");
    if args.verbose {
        if let Some(ref path) = model_path {
            eprintln!("Using model info: {}", path.display());
        }
    }

    // Load handler registry if provided or auto-discovered (US2: Handler Registry Validation)
    let handler_registry = if let Some(path) = handlers_path {
        let registry = HandlerRegistry::load_from_json(&path).map_err(|e| {
            CheckError::HandlerRegistryLoadError {
                path: path.clone(),
                source: serde_json::Error::io(std::io::Error::other(e.to_string())),
            }
        })?;
        Some(registry)
    } else {
        None
    };

    // Load model info if provided or auto-discovered (US3: Binding Validation Against Model)
    let model_info = if let Some(path) = model_path {
        let model =
            crate::commands::check::model::ModelInfo::load_from_json(&path).map_err(|e| {
                CheckError::ModelInfoLoadError {
                    path: path.clone(),
                    source: serde_json::Error::io(std::io::Error::other(e.to_string())),
                }
            })?;
        Some(model)
    } else {
        None
    };

    let mut errors = Vec::new();
    let mut files_checked = 0;

    // Find all .dampen files
    for entry in WalkDir::new(input_path)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path()
                .extension()
                .map(|ext| ext == "dampen")
                .unwrap_or(false)
        })
    {
        let file_path = entry.path();
        files_checked += 1;

        if args.verbose {
            eprintln!("Checking: {}", file_path.display());
        }

        // Read and parse the file
        let content = fs::read_to_string(file_path)?;

        // First check for XML declaration
        validate_xml_declaration(&content, file_path, &mut errors);

        // Only proceed to parse if XML declaration is valid
        if !errors.is_empty() {
            continue;
        }

        match parser::parse(&content) {
            Ok(document) => {
                // Validate the document structure
                validate_document(
                    &document,
                    file_path,
                    &handler_registry,
                    &model_info,
                    &mut errors,
                );

                // Validate references (themes, classes)
                validate_references(&document, file_path, &mut errors);

                // Validate widgets with styles, layout, breakpoints, and states
                validate_widget_with_styles(&document.root, file_path, &document, &mut errors);

                // Validate widget versions (Phase 8: Widget Version Validation)
                // This produces warnings for widgets requiring higher schema versions
                let version_warnings = dampen_core::validate_widget_versions(&document);
                if !version_warnings.is_empty() {
                    for warning in version_warnings {
                        eprintln!(
                            "Warning: {} in {}:{}:{}",
                            warning.format_message(),
                            file_path.display(),
                            warning.span.line,
                            warning.span.column
                        );
                        eprintln!("  Suggestion: {}", warning.suggestion());
                        eprintln!();
                    }
                }
            }
            Err(parse_error) => {
                errors.push(CheckError::ParseError {
                    file: file_path.to_path_buf(),
                    line: parse_error.span.line,
                    col: parse_error.span.column,
                    message: parse_error.to_string(),
                });
            }
        }
    }

    if args.verbose {
        eprintln!("Checked {} files", files_checked);
    }

    // Report errors
    if !errors.is_empty() {
        // T048: Strict mode logic - in strict mode, all validation issues are errors
        // (Currently all validation issues are already treated as errors, so this is
        // primarily for future extensibility when we might add warnings)
        let error_label = "error(s)"; // TODO: differentiate warnings in non-strict mode
        eprintln!("Found {} {}:", errors.len(), error_label);

        for error in &errors {
            // T049: Error formatting - distinguish warnings from errors in strict mode
            let prefix = "ERROR"; // TODO: use "WARNING" for warnings in non-strict mode
            eprintln!("  [{}] {}", prefix, error);
        }

        // In strict mode, exit with code 1 on any error
        // (This is already the default behavior)
        Err(errors.remove(0))
    } else {
        if args.verbose {
            let status = if args.strict {
                "✓ All files passed validation (strict mode)"
            } else {
                "✓ All files passed validation"
            };
            eprintln!("{}", status);
        }
        Ok(())
    }
}

fn validate_xml_declaration(content: &str, file_path: &Path, errors: &mut Vec<CheckError>) {
    // Check if content starts with proper XML declaration
    let trimmed = content.trim_start();
    if !trimmed.starts_with("<?xml version=\"1.0\"") {
        errors.push(CheckError::XmlValidationError {
            file: file_path.to_path_buf(),
            line: 1,
            col: 1,
            message: "Missing or invalid XML declaration. Expected: <?xml version=\"1.0\" encoding=\"UTF-8\"?>".to_string(),
        });
    }
}

fn validate_document(
    document: &dampen_core::ir::DampenDocument,
    file_path: &Path,
    handler_registry: &Option<crate::commands::check::handlers::HandlerRegistry>,
    model_info: &Option<crate::commands::check::model::ModelInfo>,
    errors: &mut Vec<CheckError>,
) {
    use crate::commands::check::cross_widget::RadioGroupValidator;

    // Get all valid widget names
    let valid_widgets: HashSet<String> = WidgetKind::all_variants()
        .iter()
        .map(|w| format!("{:?}", w).to_lowercase())
        .collect();

    // Create radio group validator to collect radio buttons across the tree
    let mut radio_validator = RadioGroupValidator::new();

    // Validate each widget in the tree
    validate_widget_node(
        &document.root,
        file_path,
        &valid_widgets,
        handler_registry,
        model_info,
        &mut radio_validator,
        errors,
    );

    // After all widgets are validated, check radio groups for consistency
    let radio_errors = radio_validator.validate();
    for error in radio_errors {
        // Convert cross_widget::CheckError to main_command::CheckError
        match error {
            crate::commands::check::errors::CheckError::DuplicateRadioValue {
                value,
                group,
                file,
                line,
                col,
                first_file,
                first_line,
                first_col,
            } => {
                errors.push(CheckError::XmlValidationError {
                    file: file.clone(),
                    line,
                    col,
                    message: format!(
                        "Duplicate radio value '{}' in group '{}'. First occurrence: {}:{}:{}",
                        value,
                        group,
                        first_file.display(),
                        first_line,
                        first_col
                    ),
                });
            }
            crate::commands::check::errors::CheckError::InconsistentRadioHandlers {
                group,
                file,
                line,
                col,
                handlers,
            } => {
                errors.push(CheckError::XmlValidationError {
                    file: file.clone(),
                    line,
                    col,
                    message: format!(
                        "Radio group '{}' has inconsistent on_select handlers. Found handlers: {}",
                        group, handlers
                    ),
                });
            }
            _ => {}
        }
    }
}

fn validate_widget_node(
    node: &dampen_core::ir::WidgetNode,
    file_path: &Path,
    valid_widgets: &HashSet<String>,
    handler_registry: &Option<crate::commands::check::handlers::HandlerRegistry>,
    model_info: &Option<crate::commands::check::model::ModelInfo>,
    radio_validator: &mut crate::commands::check::cross_widget::RadioGroupValidator,
    errors: &mut Vec<CheckError>,
) {
    use crate::commands::check::attributes;
    use crate::commands::check::suggestions;

    // Check if widget kind is valid
    let widget_name = format!("{:?}", node.kind).to_lowercase();
    if !valid_widgets.contains(&widget_name) && !matches!(node.kind, WidgetKind::Custom(_)) {
        errors.push(CheckError::InvalidWidget {
            widget: widget_name.clone(),
            file: file_path.to_path_buf(),
            line: node.span.line,
            col: node.span.column,
        });
    }

    // Validate widget attributes (US1: Unknown Attribute Detection)
    let attr_names: Vec<String> = node.attributes.keys().map(|s| s.to_string()).collect();
    let unknown_attrs = attributes::validate_widget_attributes(&node.kind, &attr_names);

    for (attr, _suggestion_opt) in unknown_attrs {
        let schema = attributes::WidgetAttributeSchema::for_widget(&node.kind);
        let all_valid = schema.all_valid_names();
        let suggestion = suggestions::suggest(&attr, &all_valid, 3);

        errors.push(CheckError::UnknownAttribute {
            attr,
            widget: widget_name.clone(),
            file: file_path.to_path_buf(),
            line: node.span.line,
            col: node.span.column,
            suggestion,
        });
    }

    // Validate required attributes (US7: Required Attribute Validation)
    let missing_required = attributes::validate_required_attributes(&node.kind, &attr_names);
    for missing_attr in missing_required {
        errors.push(CheckError::XmlValidationError {
            file: file_path.to_path_buf(),
            line: node.span.line,
            col: node.span.column,
            message: format!(
                "Missing required attribute '{}' for widget '{}'",
                missing_attr, widget_name
            ),
        });
    }

    // Validate event handlers (US2: Handler Registry Validation)
    if let Some(registry) = handler_registry {
        for event_binding in &node.events {
            if !registry.contains(&event_binding.handler) {
                // Generate suggestion using Levenshtein distance
                let all_handler_names = registry.all_names();
                let handler_refs: Vec<&str> =
                    all_handler_names.iter().map(|s| s.as_str()).collect();
                let suggestion = suggestions::suggest(&event_binding.handler, &handler_refs, 3);

                errors.push(CheckError::UnknownHandler {
                    handler: event_binding.handler.clone(),
                    file: file_path.to_path_buf(),
                    line: event_binding.span.line,
                    col: event_binding.span.column,
                    suggestion,
                });
            }
        }
    } else {
        // If no registry provided, only check for empty handlers
        for event_binding in &node.events {
            if event_binding.handler.is_empty() {
                errors.push(CheckError::UnknownHandler {
                    handler: "<empty>".to_string(),
                    file: file_path.to_path_buf(),
                    line: event_binding.span.line,
                    col: event_binding.span.column,
                    suggestion: String::new(),
                });
            }
        }
    }

    // Validate attribute bindings (US3: Binding Validation Against Model)
    if let Some(model) = model_info {
        for (attr_name, attr_value) in &node.attributes {
            validate_attribute_bindings(
                attr_name,
                attr_value,
                file_path,
                node.span.line,
                node.span.column,
                model,
                errors,
            );
        }
    }

    // Validate attribute values (style, layout, etc.)
    for attr_value in node.attributes.values() {
        validate_attribute_value(
            attr_value,
            file_path,
            node.span.line,
            node.span.column,
            errors,
        );
    }

    // Collect radio button information for cross-widget validation (US4: Radio Group Validation)
    if matches!(node.kind, WidgetKind::Radio) {
        // Extract radio button attributes
        let group_id = node
            .attributes
            .get("id")
            .and_then(|v| match v {
                AttributeValue::Static(s) => Some(s.as_str()),
                _ => None,
            })
            .unwrap_or("default");

        let value = node
            .attributes
            .get("value")
            .and_then(|v| match v {
                AttributeValue::Static(s) => Some(s.as_str()),
                _ => None,
            })
            .unwrap_or("");

        // Find on_select handler
        let handler = node
            .events
            .iter()
            .find(|e| e.event == EventKind::Select)
            .map(|e| e.handler.clone());

        radio_validator.add_radio(
            group_id,
            value,
            file_path.to_str().unwrap_or("unknown"),
            node.span.line,
            node.span.column,
            handler,
        );
    }

    // Recursively validate children
    for child in &node.children {
        validate_widget_node(
            child,
            file_path,
            valid_widgets,
            handler_registry,
            model_info,
            radio_validator,
            errors,
        );
    }
}

fn validate_attribute_bindings(
    _attr_name: &str,
    value: &dampen_core::ir::AttributeValue,
    file_path: &Path,
    line: u32,
    col: u32,
    model: &crate::commands::check::model::ModelInfo,
    errors: &mut Vec<CheckError>,
) {
    // Only validate binding expressions
    if let dampen_core::ir::AttributeValue::Binding(binding_expr) = value {
        // Validate field access in the expression
        validate_expr_fields(&binding_expr.expr, file_path, line, col, model, errors);
    }
}

fn validate_expr_fields(
    expr: &dampen_core::expr::Expr,
    file_path: &Path,
    line: u32,
    col: u32,
    model: &crate::commands::check::model::ModelInfo,
    errors: &mut Vec<CheckError>,
) {
    match expr {
        dampen_core::expr::Expr::FieldAccess(field_access) => {
            // Convert Vec<String> to Vec<&str>
            let field_parts: Vec<&str> = field_access.path.iter().map(|s| s.as_str()).collect();

            if !model.contains_field(&field_parts) {
                // Generate available fields list
                let all_paths = model.all_field_paths();
                let available = if all_paths.len() > 5 {
                    format!("{} ({} total)", &all_paths[..5].join(", "), all_paths.len())
                } else {
                    all_paths.join(", ")
                };

                let field_path = field_access.path.join(".");

                errors.push(CheckError::InvalidBinding {
                    field: field_path,
                    file: file_path.to_path_buf(),
                    line,
                    col,
                });

                // Add more detailed error with available fields
                eprintln!("  Available fields: {}", available);
            }
        }
        dampen_core::expr::Expr::MethodCall(method_call) => {
            // Validate the receiver expression
            validate_expr_fields(&method_call.receiver, file_path, line, col, model, errors);
            // Validate arguments
            for arg in &method_call.args {
                validate_expr_fields(arg, file_path, line, col, model, errors);
            }
        }
        dampen_core::expr::Expr::BinaryOp(binary_op) => {
            // Validate both sides of the binary operation
            validate_expr_fields(&binary_op.left, file_path, line, col, model, errors);
            validate_expr_fields(&binary_op.right, file_path, line, col, model, errors);
        }
        dampen_core::expr::Expr::UnaryOp(unary_op) => {
            // Validate the operand
            validate_expr_fields(&unary_op.operand, file_path, line, col, model, errors);
        }
        dampen_core::expr::Expr::Conditional(conditional) => {
            // Validate all parts of the conditional
            validate_expr_fields(&conditional.condition, file_path, line, col, model, errors);
            validate_expr_fields(
                &conditional.then_branch,
                file_path,
                line,
                col,
                model,
                errors,
            );
            validate_expr_fields(
                &conditional.else_branch,
                file_path,
                line,
                col,
                model,
                errors,
            );
        }
        dampen_core::expr::Expr::Literal(_) => {
            // Literals don't reference fields, nothing to validate
        }
        dampen_core::expr::Expr::SharedFieldAccess(shared_access) => {
            // Validate shared field paths similar to regular field access
            if shared_access.path.is_empty() || shared_access.path.iter().any(|f| f.is_empty()) {
                errors.push(CheckError::InvalidBinding {
                    field: "shared.<empty>".to_string(),
                    file: file_path.to_path_buf(),
                    line,
                    col,
                });
            }
        }
    }
}

fn validate_attribute_value(
    value: &dampen_core::ir::AttributeValue,
    file_path: &Path,
    line: u32,
    col: u32,
    errors: &mut Vec<CheckError>,
) {
    match value {
        dampen_core::ir::AttributeValue::Static(_) => {
            // Static values are always valid
        }
        dampen_core::ir::AttributeValue::Binding(binding_expr) => {
            // For now, we'll do basic validation of the binding expression
            // In a real implementation, we'd check against the model fields
            validate_binding_expr(&binding_expr.expr, file_path, line, col, errors);
        }
        dampen_core::ir::AttributeValue::Interpolated(parts) => {
            for part in parts {
                match part {
                    dampen_core::ir::InterpolatedPart::Literal(_) => {
                        // Literals are always valid
                    }
                    dampen_core::ir::InterpolatedPart::Binding(binding_expr) => {
                        validate_binding_expr(&binding_expr.expr, file_path, line, col, errors);
                    }
                }
            }
        }
    }
}

fn validate_binding_expr(
    expr: &dampen_core::expr::Expr,
    file_path: &Path,
    line: u32,
    col: u32,
    errors: &mut Vec<CheckError>,
) {
    match expr {
        dampen_core::expr::Expr::FieldAccess(field_access) => {
            // For now, we'll assume any field name is valid
            // In a real implementation, we'd check against the model fields
            if field_access.path.is_empty() || field_access.path.iter().any(|f| f.is_empty()) {
                errors.push(CheckError::InvalidBinding {
                    field: "<empty>".to_string(),
                    file: file_path.to_path_buf(),
                    line,
                    col,
                });
            }
        }
        dampen_core::expr::Expr::MethodCall(_) => {
            // Method calls are generally valid if the method exists
            // For now, we'll assume they're valid
        }
        dampen_core::expr::Expr::BinaryOp(_) => {
            // Binary operations are valid if both operands are valid
            // We'd need to recursively validate the operands
        }
        dampen_core::expr::Expr::UnaryOp(_) => {
            // Unary operations are valid if the operand is valid
        }
        dampen_core::expr::Expr::Conditional(_) => {
            // Conditionals are valid if all parts are valid
        }
        dampen_core::expr::Expr::Literal(_) => {
            // Literals are always valid
        }
        dampen_core::expr::Expr::SharedFieldAccess(_) => {
            // Shared field access is valid if the field exists in shared state
            // For now, we'll assume they're valid
        }
    }
}

// Helper extension to get all widget variants
trait WidgetKindExt {
    fn all_variants() -> Vec<WidgetKind>;
}

impl WidgetKindExt for WidgetKind {
    fn all_variants() -> Vec<WidgetKind> {
        vec![
            WidgetKind::Column,
            WidgetKind::Row,
            WidgetKind::Container,
            WidgetKind::Scrollable,
            WidgetKind::Stack,
            WidgetKind::Text,
            WidgetKind::Image,
            WidgetKind::Svg,
            WidgetKind::Button,
            WidgetKind::TextInput,
            WidgetKind::Checkbox,
            WidgetKind::Slider,
            WidgetKind::PickList,
            WidgetKind::Toggler,
            WidgetKind::Space,
            WidgetKind::Rule,
            WidgetKind::Radio,
            WidgetKind::ComboBox,
            WidgetKind::ProgressBar,
            WidgetKind::Tooltip,
            WidgetKind::Grid,
            WidgetKind::Canvas,
            WidgetKind::Float,
            WidgetKind::For,
        ]
    }
}

/// Validate all references (themes, classes) in the document
fn validate_references(
    document: &dampen_core::ir::DampenDocument,
    file_path: &Path,
    errors: &mut Vec<CheckError>,
) {
    // Validate global theme reference
    if let Some(global_theme) = &document.global_theme {
        if !document.themes.contains_key(global_theme) {
            errors.push(CheckError::UnknownTheme {
                theme: global_theme.clone(),
                file: file_path.to_path_buf(),
                line: 1,
                col: 1,
            });
        }
    }

    // Validate each theme definition (US5: Theme Property Validation)
    for (name, theme) in &document.themes {
        if let Err(msg) = theme.validate(false) {
            // Check if it's a circular dependency error
            if msg.contains("circular") || msg.contains("Circular") {
                errors.push(CheckError::XmlValidationError {
                    file: file_path.to_path_buf(),
                    line: 1,
                    col: 1,
                    message: format!("Theme '{}' validation error: {}", name, msg),
                });
            } else {
                errors.push(CheckError::InvalidStyleValue {
                    attr: format!("theme '{}'", name),
                    file: file_path.to_path_buf(),
                    line: 1,
                    col: 1,
                    message: msg,
                });
            }
        }
    }

    // Validate each style class definition (US5: Circular Dependency Detection)
    for (name, class) in &document.style_classes {
        if let Err(msg) = class.validate(&document.style_classes) {
            // Check if it's a circular dependency error
            if msg.contains("circular") || msg.contains("Circular") {
                errors.push(CheckError::XmlValidationError {
                    file: file_path.to_path_buf(),
                    line: 1,
                    col: 1,
                    message: format!("Style class '{}' has circular dependency: {}", name, msg),
                });
            } else {
                errors.push(CheckError::InvalidStyleValue {
                    attr: format!("class '{}'", name),
                    file: file_path.to_path_buf(),
                    line: 1,
                    col: 1,
                    message: msg,
                });
            }
        }
    }
}

/// Validate a widget node with all its styles, layout, and references
fn validate_widget_with_styles(
    node: &dampen_core::ir::WidgetNode,
    file_path: &Path,
    document: &dampen_core::ir::DampenDocument,
    errors: &mut Vec<CheckError>,
) {
    // Validate structured style properties
    if let Some(style) = &node.style {
        if let Err(msg) = style.validate() {
            errors.push(CheckError::InvalidStyleValue {
                attr: "structured style".to_string(),
                file: file_path.to_path_buf(),
                line: node.span.line,
                col: node.span.column,
                message: msg,
            });
        }
    }

    // Validate structured layout constraints
    if let Some(layout) = &node.layout {
        if let Err(msg) = layout.validate() {
            errors.push(CheckError::InvalidLayoutConstraint {
                file: file_path.to_path_buf(),
                line: node.span.line,
                col: node.span.column,
                message: msg,
            });
        }
    }

    // Validate style class references
    for class_name in &node.classes {
        if !document.style_classes.contains_key(class_name) {
            errors.push(CheckError::UnknownStyleClass {
                class: class_name.clone(),
                file: file_path.to_path_buf(),
                line: node.span.line,
                col: node.span.column,
            });
        }
    }

    // Validate theme reference
    if let Some(theme_ref) = &node.theme_ref {
        match theme_ref {
            AttributeValue::Static(theme_name) => {
                if !document.themes.contains_key(theme_name) {
                    errors.push(CheckError::UnknownTheme {
                        theme: theme_name.clone(),
                        file: file_path.to_path_buf(),
                        line: node.span.line,
                        col: node.span.column,
                    });
                }
            }
            AttributeValue::Binding(_) | AttributeValue::Interpolated(_) => {
                // Binding expressions can't be validated at check time
                // They will be evaluated at runtime
            }
        }
    }

    // Validate inline style attributes
    validate_style_attributes(node, file_path, errors);

    // Validate inline layout attributes
    validate_layout_attributes(node, file_path, errors);

    // Validate breakpoint attributes
    validate_breakpoint_attributes(node, file_path, errors);

    // Validate state attributes
    validate_state_attributes(node, file_path, errors);

    // Recursively validate children
    for child in &node.children {
        validate_widget_with_styles(child, file_path, document, errors);
    }
}

/// Validate inline style attributes
fn validate_style_attributes(
    node: &dampen_core::ir::WidgetNode,
    file_path: &Path,
    errors: &mut Vec<CheckError>,
) {
    for (attr_name, attr_value) in &node.attributes {
        match attr_name.as_str() {
            "background" => {
                if let AttributeValue::Static(value) = attr_value {
                    if let Err(msg) = style_parser::parse_background_attr(value) {
                        errors.push(CheckError::InvalidStyleValue {
                            attr: attr_name.clone(),
                            file: file_path.to_path_buf(),
                            line: node.span.line,
                            col: node.span.column,
                            message: msg,
                        });
                    }
                }
            }
            "color" | "border_color" => {
                if let AttributeValue::Static(value) = attr_value {
                    if let Err(msg) = style_parser::parse_color_attr(value) {
                        errors.push(CheckError::InvalidStyleValue {
                            attr: attr_name.clone(),
                            file: file_path.to_path_buf(),
                            line: node.span.line,
                            col: node.span.column,
                            message: msg,
                        });
                    }
                }
            }
            "border_width" | "opacity" => {
                if let AttributeValue::Static(value) = attr_value {
                    if let Err(msg) = style_parser::parse_float_attr(value, attr_name) {
                        errors.push(CheckError::InvalidStyleValue {
                            attr: attr_name.clone(),
                            file: file_path.to_path_buf(),
                            line: node.span.line,
                            col: node.span.column,
                            message: msg,
                        });
                    }
                }
            }
            "border_radius" => {
                if let AttributeValue::Static(value) = attr_value {
                    if let Err(msg) = style_parser::parse_border_radius(value) {
                        errors.push(CheckError::InvalidStyleValue {
                            attr: attr_name.clone(),
                            file: file_path.to_path_buf(),
                            line: node.span.line,
                            col: node.span.column,
                            message: msg,
                        });
                    }
                }
            }
            "border_style" => {
                if let AttributeValue::Static(value) = attr_value {
                    if let Err(msg) = style_parser::parse_border_style(value) {
                        errors.push(CheckError::InvalidStyleValue {
                            attr: attr_name.clone(),
                            file: file_path.to_path_buf(),
                            line: node.span.line,
                            col: node.span.column,
                            message: msg,
                        });
                    }
                }
            }
            "shadow" => {
                if let AttributeValue::Static(value) = attr_value {
                    if let Err(msg) = style_parser::parse_shadow_attr(value) {
                        errors.push(CheckError::InvalidStyleValue {
                            attr: attr_name.clone(),
                            file: file_path.to_path_buf(),
                            line: node.span.line,
                            col: node.span.column,
                            message: msg,
                        });
                    }
                }
            }
            "transform" => {
                if let AttributeValue::Static(value) = attr_value {
                    if let Err(msg) = style_parser::parse_transform(value) {
                        errors.push(CheckError::InvalidStyleValue {
                            attr: attr_name.clone(),
                            file: file_path.to_path_buf(),
                            line: node.span.line,
                            col: node.span.column,
                            message: msg,
                        });
                    }
                }
            }
            _ => {} // Autres attributs gérés ailleurs
        }
    }
}

/// Validate inline layout attributes
fn validate_layout_attributes(
    node: &dampen_core::ir::WidgetNode,
    file_path: &Path,
    errors: &mut Vec<CheckError>,
) {
    for (attr_name, attr_value) in &node.attributes {
        match attr_name.as_str() {
            "width" | "height" | "min_width" | "max_width" | "min_height" | "max_height" => {
                if let AttributeValue::Static(value) = attr_value {
                    if let Err(msg) = style_parser::parse_length_attr(value) {
                        errors.push(CheckError::InvalidStyleValue {
                            attr: attr_name.clone(),
                            file: file_path.to_path_buf(),
                            line: node.span.line,
                            col: node.span.column,
                            message: msg,
                        });
                    }
                }
            }
            "padding" => {
                if let AttributeValue::Static(value) = attr_value {
                    if let Err(msg) = style_parser::parse_padding_attr(value) {
                        errors.push(CheckError::InvalidStyleValue {
                            attr: attr_name.clone(),
                            file: file_path.to_path_buf(),
                            line: node.span.line,
                            col: node.span.column,
                            message: msg,
                        });
                    }
                }
            }
            "spacing" => {
                if let AttributeValue::Static(value) = attr_value {
                    if let Err(msg) = style_parser::parse_spacing(value) {
                        errors.push(CheckError::InvalidStyleValue {
                            attr: attr_name.clone(),
                            file: file_path.to_path_buf(),
                            line: node.span.line,
                            col: node.span.column,
                            message: msg,
                        });
                    }
                }
            }
            "align_items" => {
                if let AttributeValue::Static(value) = attr_value {
                    if let Err(msg) = style_parser::parse_alignment(value) {
                        errors.push(CheckError::InvalidStyleValue {
                            attr: attr_name.clone(),
                            file: file_path.to_path_buf(),
                            line: node.span.line,
                            col: node.span.column,
                            message: msg,
                        });
                    }
                }
            }
            "justify_content" => {
                if let AttributeValue::Static(value) = attr_value {
                    if let Err(msg) = style_parser::parse_justification(value) {
                        errors.push(CheckError::InvalidStyleValue {
                            attr: attr_name.clone(),
                            file: file_path.to_path_buf(),
                            line: node.span.line,
                            col: node.span.column,
                            message: msg,
                        });
                    }
                }
            }
            "direction" => {
                if let AttributeValue::Static(value) = attr_value {
                    if let Err(msg) = Direction::parse(value) {
                        errors.push(CheckError::InvalidStyleValue {
                            attr: attr_name.clone(),
                            file: file_path.to_path_buf(),
                            line: node.span.line,
                            col: node.span.column,
                            message: msg,
                        });
                    }
                }
            }
            "position" => {
                if matches!(node.kind, WidgetKind::Tooltip) {
                } else if let AttributeValue::Static(value) = attr_value {
                    if let Err(msg) = Position::parse(value) {
                        errors.push(CheckError::InvalidStyleValue {
                            attr: attr_name.clone(),
                            file: file_path.to_path_buf(),
                            line: node.span.line,
                            col: node.span.column,
                            message: msg,
                        });
                    }
                }
            }
            "top" | "right" | "bottom" | "left" => {
                if let AttributeValue::Static(value) = attr_value {
                    if let Err(msg) = style_parser::parse_float_attr(value, attr_name) {
                        errors.push(CheckError::InvalidStyleValue {
                            attr: attr_name.clone(),
                            file: file_path.to_path_buf(),
                            line: node.span.line,
                            col: node.span.column,
                            message: msg,
                        });
                    }
                }
            }
            "z_index" => {
                if let AttributeValue::Static(value) = attr_value {
                    if let Err(msg) = style_parser::parse_int_attr(value, attr_name) {
                        errors.push(CheckError::InvalidStyleValue {
                            attr: attr_name.clone(),
                            file: file_path.to_path_buf(),
                            line: node.span.line,
                            col: node.span.column,
                            message: msg,
                        });
                    }
                }
            }
            _ => {} // Autres attributs gérés ailleurs
        }
    }
}

/// Validate breakpoint attributes (mobile:, tablet:, desktop:)
fn validate_breakpoint_attributes(
    node: &dampen_core::ir::WidgetNode,
    file_path: &Path,
    errors: &mut Vec<CheckError>,
) {
    for (breakpoint, attrs) in &node.breakpoint_attributes {
        for (attr_name, attr_value) in attrs {
            // Valider que l'attribut de base est valide
            let base_attr = attr_name.as_str();
            let full_attr = format!("{:?}:{}", breakpoint, base_attr);

            // Utiliser les mêmes validateurs que pour les attributs normaux
            let is_style_attr = matches!(
                base_attr,
                "background"
                    | "color"
                    | "border_width"
                    | "border_color"
                    | "border_radius"
                    | "border_style"
                    | "shadow"
                    | "opacity"
                    | "transform"
            );

            let is_layout_attr = matches!(
                base_attr,
                "width"
                    | "height"
                    | "min_width"
                    | "max_width"
                    | "min_height"
                    | "max_height"
                    | "padding"
                    | "spacing"
                    | "align_items"
                    | "justify_content"
                    | "direction"
                    | "position"
                    | "top"
                    | "right"
                    | "bottom"
                    | "left"
                    | "z_index"
            );

            if !is_style_attr && !is_layout_attr {
                errors.push(CheckError::InvalidBreakpoint {
                    attr: full_attr,
                    file: file_path.to_path_buf(),
                    line: node.span.line,
                    col: node.span.column,
                });
                continue;
            }

            // Valider la valeur selon le type d'attribut
            if let AttributeValue::Static(value) = attr_value {
                let result: Result<(), String> = match base_attr {
                    "background" => style_parser::parse_background_attr(value).map(|_| ()),
                    "color" | "border_color" => style_parser::parse_color_attr(value).map(|_| ()),
                    "border_width" | "opacity" => {
                        style_parser::parse_float_attr(value, base_attr).map(|_| ())
                    }
                    "border_radius" => style_parser::parse_border_radius(value).map(|_| ()),
                    "border_style" => style_parser::parse_border_style(value).map(|_| ()),
                    "shadow" => style_parser::parse_shadow_attr(value).map(|_| ()),
                    "transform" => style_parser::parse_transform(value).map(|_| ()),
                    "width" | "height" | "min_width" | "max_width" | "min_height"
                    | "max_height" => style_parser::parse_length_attr(value).map(|_| ()),
                    "padding" => style_parser::parse_padding_attr(value).map(|_| ()),
                    "spacing" => style_parser::parse_spacing(value).map(|_| ()),
                    "align_items" => style_parser::parse_alignment(value).map(|_| ()),
                    "justify_content" => style_parser::parse_justification(value).map(|_| ()),
                    "direction" => Direction::parse(value).map(|_| ()),
                    "position" => Position::parse(value).map(|_| ()),
                    "top" | "right" | "bottom" | "left" => {
                        style_parser::parse_float_attr(value, base_attr).map(|_| ())
                    }
                    "z_index" => style_parser::parse_int_attr(value, base_attr).map(|_| ()),
                    _ => Ok(()),
                };

                if let Err(msg) = result {
                    errors.push(CheckError::InvalidStyleValue {
                        attr: full_attr,
                        file: file_path.to_path_buf(),
                        line: node.span.line,
                        col: node.span.column,
                        message: msg,
                    });
                }
            }
        }
    }
}

/// Validate state attributes (hover:, focus:, active:, disabled:)
fn validate_state_attributes(
    node: &dampen_core::ir::WidgetNode,
    file_path: &Path,
    errors: &mut Vec<CheckError>,
) {
    for (attr_name, attr_value) in &node.attributes {
        if attr_name.contains(':') {
            let parts: Vec<&str> = attr_name.split(':').collect();
            if parts.len() >= 2 {
                let prefix = parts[0];
                let base_attr = parts[1];

                // Valider le préfixe d'état
                if !["hover", "focus", "active", "disabled"].contains(&prefix) {
                    errors.push(CheckError::InvalidState {
                        attr: attr_name.clone(),
                        file: file_path.to_path_buf(),
                        line: node.span.line,
                        col: node.span.column,
                    });
                    continue;
                }

                // Valider que l'attribut de base est valide
                let is_valid_attr = matches!(
                    base_attr,
                    "background"
                        | "color"
                        | "border_width"
                        | "border_color"
                        | "border_radius"
                        | "border_style"
                        | "shadow"
                        | "opacity"
                        | "transform"
                        | "width"
                        | "height"
                        | "min_width"
                        | "max_width"
                        | "min_height"
                        | "max_height"
                        | "padding"
                        | "spacing"
                        | "align_items"
                        | "justify_content"
                        | "direction"
                        | "position"
                        | "top"
                        | "right"
                        | "bottom"
                        | "left"
                        | "z_index"
                );

                if !is_valid_attr {
                    errors.push(CheckError::InvalidState {
                        attr: attr_name.clone(),
                        file: file_path.to_path_buf(),
                        line: node.span.line,
                        col: node.span.column,
                    });
                    continue;
                }

                // Valider la valeur
                if let AttributeValue::Static(value) = attr_value {
                    let result: Result<(), String> = match base_attr {
                        "background" => style_parser::parse_background_attr(value).map(|_| ()),
                        "color" | "border_color" => {
                            style_parser::parse_color_attr(value).map(|_| ())
                        }
                        "border_width" | "opacity" => {
                            style_parser::parse_float_attr(value, base_attr).map(|_| ())
                        }
                        "border_radius" => style_parser::parse_border_radius(value).map(|_| ()),
                        "border_style" => style_parser::parse_border_style(value).map(|_| ()),
                        "shadow" => style_parser::parse_shadow_attr(value).map(|_| ()),
                        "transform" => style_parser::parse_transform(value).map(|_| ()),
                        "width" | "height" | "min_width" | "max_width" | "min_height"
                        | "max_height" => style_parser::parse_length_attr(value).map(|_| ()),
                        "padding" => style_parser::parse_padding_attr(value).map(|_| ()),
                        "spacing" => style_parser::parse_spacing(value).map(|_| ()),
                        "align_items" => style_parser::parse_alignment(value).map(|_| ()),
                        "justify_content" => style_parser::parse_justification(value).map(|_| ()),
                        "direction" => Direction::parse(value).map(|_| ()),
                        "position" => Position::parse(value).map(|_| ()),
                        "top" | "right" | "bottom" | "left" => {
                            style_parser::parse_float_attr(value, base_attr).map(|_| ())
                        }
                        "z_index" => style_parser::parse_int_attr(value, base_attr).map(|_| ()),
                        _ => Ok(()),
                    };

                    if let Err(msg) = result {
                        errors.push(CheckError::InvalidStyleValue {
                            attr: attr_name.clone(),
                            file: file_path.to_path_buf(),
                            line: node.span.line,
                            col: node.span.column,
                            message: msg,
                        });
                    }
                }
            }
        }
    }
}
