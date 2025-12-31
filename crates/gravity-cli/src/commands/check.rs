//! Check command - validates Gravity UI files

use clap::Args;
use gravity_core::{parser, ir::WidgetKind};
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
    ParseError { file: PathBuf, line: u32, col: u32, message: String },
    
    #[error("XML validation error in {file}:{line}:{col}: {message}")]
    XmlValidationError { file: PathBuf, line: u32, col: u32, message: String },
    
    #[error("Invalid widget '{widget}' in {file}:{line}:{col}")]
    InvalidWidget { widget: String, file: PathBuf, line: u32, col: u32 },
    
    #[error("Unknown handler '{handler}' in {file}:{line}:{col}")]
    UnknownHandler { handler: String, file: PathBuf, line: u32, col: u32 },
    
    #[error("Invalid binding field '{field}' in {file}:{line}:{col}")]
    InvalidBinding { field: String, file: PathBuf, line: u32, col: u32 },
    
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
            e.path().extension().map(|ext| ext == "gravity").unwrap_or(false)
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
                // Validate the document
                validate_document(&document, file_path, &mut errors);
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
        return Err(errors.remove(0));
    } else {
        if args.verbose {
            eprintln!("✓ All files passed validation");
        }
        Ok(())
    }
}

fn validate_xml_declaration(
    content: &str,
    file_path: &Path,
    errors: &mut Vec<CheckError>,
) {
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
    for (_attr_name, attr_value) in &node.attributes {
        validate_attribute_value(attr_value, file_path, node.span.line, node.span.column, errors);
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