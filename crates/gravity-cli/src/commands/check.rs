#![allow(clippy::print_stderr, clippy::print_stdout)]

//! Check command - validates Gravity UI files

use clap::Args;
use gravity_core::ir::layout::{Direction, Position};
use gravity_core::{
    ir::{AttributeValue, WidgetKind},
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

    #[error("Unknown handler '{handler}' in {file}:{line}:{col}")]
    UnknownHandler {
        handler: String,
        file: PathBuf,
        line: u32,
        col: u32,
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

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

#[derive(Args)]
pub struct CheckArgs {
    /// Directory containing .gravity files to check
    #[arg(short, long, default_value = "ui")]
    pub input: String,

    /// Enable verbose output
    #[arg(short, long)]
    pub verbose: bool,
}

pub fn execute(args: &CheckArgs) -> Result<(), CheckError> {
    let input_path = Path::new(&args.input);

    if !input_path.exists() {
        return Err(CheckError::DirectoryNotFound(input_path.to_path_buf()));
    }

    if args.verbose {
        eprintln!("Checking Gravity UI files in: {}", input_path.display());
    }

    let mut errors = Vec::new();
    let mut files_checked = 0;

    // Find all .gravity files
    for entry in WalkDir::new(input_path)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path()
                .extension()
                .map(|ext| ext == "gravity")
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
                validate_document(&document, file_path, &mut errors);

                // Validate references (themes, classes)
                validate_references(&document, file_path, &mut errors);

                // Validate widgets with styles, layout, breakpoints, and states
                validate_widget_with_styles(&document.root, file_path, &document, &mut errors);
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
        eprintln!("Found {} error(s):", errors.len());
        for error in &errors {
            eprintln!("  {}", error);
        }
        // Return the first error for exit code purposes
        Err(errors.remove(0))
    } else {
        if args.verbose {
            eprintln!("✓ All files passed validation");
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
    document: &gravity_core::ir::GravityDocument,
    file_path: &Path,
    errors: &mut Vec<CheckError>,
) {
    // Get all valid widget names
    let valid_widgets: HashSet<String> = WidgetKind::all_variants()
        .iter()
        .map(|w| format!("{:?}", w).to_lowercase())
        .collect();

    // Validate each widget in the tree
    validate_widget_node(&document.root, file_path, &valid_widgets, errors);
}

fn validate_widget_node(
    node: &gravity_core::ir::WidgetNode,
    file_path: &Path,
    valid_widgets: &HashSet<String>,
    errors: &mut Vec<CheckError>,
) {
    // Check if widget kind is valid
    let widget_name = format!("{:?}", node.kind).to_lowercase();
    if !valid_widgets.contains(&widget_name) && !matches!(node.kind, WidgetKind::Custom(_)) {
        errors.push(CheckError::InvalidWidget {
            widget: widget_name,
            file: file_path.to_path_buf(),
            line: node.span.line,
            col: node.span.column,
        });
    }

    // Validate event handlers
    for event_binding in &node.events {
        // For now, we'll assume any handler name is valid
        // In a real implementation, we'd check against registered handlers
        if event_binding.handler.is_empty() {
            errors.push(CheckError::UnknownHandler {
                handler: "<empty>".to_string(),
                file: file_path.to_path_buf(),
                line: event_binding.span.line,
                col: event_binding.span.column,
            });
        }
    }

    // Validate attribute bindings
    for attr_value in node.attributes.values() {
        validate_attribute_value(
            attr_value,
            file_path,
            node.span.line,
            node.span.column,
            errors,
        );
    }

    // Recursively validate children
    for child in &node.children {
        validate_widget_node(child, file_path, valid_widgets, errors);
    }
}

fn validate_attribute_value(
    value: &gravity_core::ir::AttributeValue,
    file_path: &Path,
    line: u32,
    col: u32,
    errors: &mut Vec<CheckError>,
) {
    match value {
        gravity_core::ir::AttributeValue::Static(_) => {
            // Static values are always valid
        }
        gravity_core::ir::AttributeValue::Binding(binding_expr) => {
            // For now, we'll do basic validation of the binding expression
            // In a real implementation, we'd check against the model fields
            validate_binding_expr(&binding_expr.expr, file_path, line, col, errors);
        }
        gravity_core::ir::AttributeValue::Interpolated(parts) => {
            for part in parts {
                match part {
                    gravity_core::ir::InterpolatedPart::Literal(_) => {
                        // Literals are always valid
                    }
                    gravity_core::ir::InterpolatedPart::Binding(binding_expr) => {
                        validate_binding_expr(&binding_expr.expr, file_path, line, col, errors);
                    }
                }
            }
        }
    }
}

fn validate_binding_expr(
    expr: &gravity_core::expr::Expr,
    file_path: &Path,
    line: u32,
    col: u32,
    errors: &mut Vec<CheckError>,
) {
    match expr {
        gravity_core::expr::Expr::FieldAccess(field_access) => {
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
        gravity_core::expr::Expr::MethodCall(_) => {
            // Method calls are generally valid if the method exists
            // For now, we'll assume they're valid
        }
        gravity_core::expr::Expr::BinaryOp(_) => {
            // Binary operations are valid if both operands are valid
            // We'd need to recursively validate the operands
        }
        gravity_core::expr::Expr::UnaryOp(_) => {
            // Unary operations are valid if the operand is valid
        }
        gravity_core::expr::Expr::Conditional(_) => {
            // Conditionals are valid if all parts are valid
        }
        gravity_core::expr::Expr::Literal(_) => {
            // Literals are always valid
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
        ]
    }
}

/// Validate all references (themes, classes) in the document
fn validate_references(
    document: &gravity_core::ir::GravityDocument,
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

    // Validate each theme definition
    for (name, theme) in &document.themes {
        if let Err(msg) = theme.validate() {
            errors.push(CheckError::InvalidStyleValue {
                attr: format!("theme '{}'", name),
                file: file_path.to_path_buf(),
                line: 1,
                col: 1,
                message: msg,
            });
        }
    }

    // Validate each style class definition
    for (name, class) in &document.style_classes {
        if let Err(msg) = class.validate(&document.style_classes) {
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

/// Validate a widget node with all its styles, layout, and references
fn validate_widget_with_styles(
    node: &gravity_core::ir::WidgetNode,
    file_path: &Path,
    document: &gravity_core::ir::GravityDocument,
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
        if !document.themes.contains_key(theme_ref) {
            errors.push(CheckError::UnknownTheme {
                theme: theme_ref.clone(),
                file: file_path.to_path_buf(),
                line: node.span.line,
                col: node.span.column,
            });
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
    node: &gravity_core::ir::WidgetNode,
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
    node: &gravity_core::ir::WidgetNode,
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
                if let AttributeValue::Static(value) = attr_value {
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
    node: &gravity_core::ir::WidgetNode,
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
    node: &gravity_core::ir::WidgetNode,
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
