pub mod error;
pub mod gradient;
pub mod lexer;
pub mod style_parser;
pub mod theme_parser;

use crate::expr::tokenize_binding_expr;
use crate::ir::{
    AttributeValue, EventBinding, EventKind, GravityDocument, InterpolatedPart, SchemaVersion,
    Span, WidgetKind, WidgetNode,
};
use crate::parser::error::{ParseError, ParseErrorKind};
use roxmltree::{Document, Node, NodeType};
use std::collections::HashMap;

/// Parse XML markup into a GravityDocument.
///
/// This is the main entry point for the parser. It takes XML markup and
/// converts it into the Intermediate Representation (IR) suitable for
/// rendering or code generation.
///
/// # Arguments
///
/// * `xml` - XML markup string
///
/// # Returns
///
/// `Ok(GravityDocument)` on success, `Err(ParseError)` on failure
///
/// # Examples
///
/// ```rust
/// use gravity_core::parse;
///
/// let xml = r#"<column><text value="Hello" /></column>"#;
/// let doc = parse(xml).unwrap();
/// assert_eq!(doc.root.children.len(), 1);
/// ```
///
/// # Errors
///
/// Returns `ParseError` for:
/// - Invalid XML syntax
/// - Unknown widget elements
/// - Invalid attribute values
/// - Malformed binding expressions
pub fn parse(xml: &str) -> Result<GravityDocument, ParseError> {
    // Parse XML using roxmltree
    let doc = Document::parse(xml).map_err(|e| ParseError {
        kind: ParseErrorKind::XmlSyntax,
        message: e.to_string(),
        span: Span::new(0, 0, 1, 1),
        suggestion: None,
    })?;

    // Find root element (skip XML declaration)
    let root = doc.root().first_child().ok_or_else(|| ParseError {
        kind: ParseErrorKind::XmlSyntax,
        message: "No root element found".to_string(),
        span: Span::new(0, 0, 1, 1),
        suggestion: None,
    })?;

    // Parse the root widget
    let root_widget = parse_node(root, xml)?;

    Ok(GravityDocument {
        version: SchemaVersion { major: 1, minor: 0 },
        root: root_widget,
        themes: HashMap::new(),
        style_classes: HashMap::new(),
        global_theme: None,
    })
}

/// Parse a single XML node into a WidgetNode
fn parse_node(node: Node, source: &str) -> Result<WidgetNode, ParseError> {
    // Only process element nodes
    if node.node_type() != NodeType::Element {
        return Err(ParseError {
            kind: ParseErrorKind::XmlSyntax,
            message: "Expected element node".to_string(),
            span: Span::new(0, 0, 1, 1),
            suggestion: None,
        });
    }

    // Get element name and map to WidgetKind
    let tag_name = node.tag_name().name();
    let kind = match tag_name {
        "column" => WidgetKind::Column,
        "row" => WidgetKind::Row,
        "container" => WidgetKind::Container,
        "scrollable" => WidgetKind::Scrollable,
        "stack" => WidgetKind::Stack,
        "text" => WidgetKind::Text,
        "image" => WidgetKind::Image,
        "svg" => WidgetKind::Svg,
        "button" => WidgetKind::Button,
        "text_input" => WidgetKind::TextInput,
        "checkbox" => WidgetKind::Checkbox,
        "slider" => WidgetKind::Slider,
        "pick_list" => WidgetKind::PickList,
        "toggler" => WidgetKind::Toggler,
        "space" => WidgetKind::Space,
        "rule" => WidgetKind::Rule,
        _ => {
            return Err(ParseError {
                kind: ParseErrorKind::UnknownWidget,
                message: format!("Unknown widget: <{}>", tag_name),
                span: get_span(node, source),
                suggestion: Some("Did you mean one of the standard widgets?".to_string()),
            });
        }
    };

    // Parse attributes
    let mut attributes = std::collections::HashMap::new();
    let mut events = Vec::new();
    let mut id = None;

    for attr in node.attributes() {
        let name = attr.name();
        let value = attr.value();

        // Check for id attribute
        if name == "id" {
            id = Some(value.to_string());
            continue;
        }

        // Check for event attributes (on_click, on_change, etc.)
        if name.starts_with("on_") {
            let event_kind = match name {
                "on_click" => Some(EventKind::Click),
                "on_press" => Some(EventKind::Press),
                "on_release" => Some(EventKind::Release),
                "on_change" => Some(EventKind::Change),
                "on_input" => Some(EventKind::Input),
                "on_submit" => Some(EventKind::Submit),
                "on_select" => Some(EventKind::Select),
                "on_toggle" => Some(EventKind::Toggle),
                "on_scroll" => Some(EventKind::Scroll),
                _ => None,
            };

            if let Some(event) = event_kind {
                events.push(EventBinding {
                    event,
                    handler: value.to_string(),
                    span: get_span(node, source),
                });
                continue;
            }
        }

        // Parse attribute value (check for bindings)
        let attr_value = parse_attribute_value(value, get_span(node, source))?;
        attributes.insert(name.to_string(), attr_value);
    }

    // Parse children
    let mut children = Vec::new();
    for child in node.children() {
        if child.node_type() == NodeType::Element {
            children.push(parse_node(child, source)?);
        }
    }

    Ok(WidgetNode {
        kind,
        id,
        attributes,
        events,
        children,
        span: get_span(node, source),
        style: None,
        layout: None,
        theme_ref: None,
        classes: Vec::new(),
        breakpoint_attributes: HashMap::new(),
    })
}

/// Parse attribute value, detecting binding expressions
fn parse_attribute_value(value: &str, span: Span) -> Result<AttributeValue, ParseError> {
    // Check if value contains binding syntax {expr}
    if value.contains('{') && value.contains('}') {
        // Parse interpolated parts
        let mut parts = Vec::new();
        let mut remaining = value;

        while let Some(start_pos) = remaining.find('{') {
            // Add literal before {
            if start_pos > 0 {
                parts.push(InterpolatedPart::Literal(
                    remaining[..start_pos].to_string(),
                ));
            }

            // Find closing }
            if let Some(end_pos) = remaining[start_pos..].find('}') {
                let expr_start = start_pos + 1;
                let expr_end = start_pos + end_pos;
                let expr_str = &remaining[expr_start..expr_end];

                // Parse the expression
                let binding_expr = tokenize_binding_expr(
                    expr_str,
                    span.start + expr_start,
                    span.line,
                    span.column + expr_start as u32,
                )
                .map_err(|e| ParseError {
                    kind: ParseErrorKind::InvalidExpression,
                    message: format!("Invalid expression: {}", e),
                    span: Span::new(
                        span.start + expr_start,
                        span.start + expr_end,
                        span.line,
                        span.column + expr_start as u32,
                    ),
                    suggestion: None,
                })?;

                parts.push(InterpolatedPart::Binding(binding_expr));

                // Move past the }
                remaining = &remaining[expr_end + 1..];
            } else {
                // No closing }, treat rest as literal
                parts.push(InterpolatedPart::Literal(remaining.to_string()));
                break;
            }
        }

        // Add remaining literal
        if !remaining.is_empty() {
            parts.push(InterpolatedPart::Literal(remaining.to_string()));
        }

        // If only one binding with no literals, return Binding
        // If multiple parts, return Interpolated
        if parts.len() == 1 {
            match &parts[0] {
                InterpolatedPart::Binding(expr) => {
                    return Ok(AttributeValue::Binding(expr.clone()));
                }
                InterpolatedPart::Literal(lit) => {
                    return Ok(AttributeValue::Static(lit.clone()));
                }
            }
        } else {
            return Ok(AttributeValue::Interpolated(parts));
        }
    }

    // Static value
    Ok(AttributeValue::Static(value.to_string()))
}

/// Extract span information from roxmltree node
fn get_span(node: Node, source: &str) -> Span {
    let range = node.range();

    // Calculate line and column from byte offset
    let (line, col) = calculate_line_col(source, range.start);

    Span {
        start: range.start,
        end: range.end,
        line,
        column: col,
    }
}

/// Calculate line and column from byte offset
fn calculate_line_col(source: &str, offset: usize) -> (u32, u32) {
    let mut line = 1;
    let mut col = 1;

    for (i, c) in source.chars().enumerate() {
        if i >= offset {
            break;
        }
        if c == '\n' {
            line += 1;
            col = 1;
        } else {
            col += 1;
        }
    }

    (line, col)
}
