pub mod error;
pub mod gradient;
pub mod lexer;
pub mod style_parser;
pub mod theme_parser;

use crate::expr::tokenize_binding_expr;
use crate::expr::{BindingExpr, Expr, LiteralExpr};
use crate::ir::{
    AttributeValue, DampenDocument, EventBinding, EventKind, InterpolatedPart, SchemaVersion, Span,
    WidgetKind, WidgetNode,
};
use crate::parser::error::{ParseError, ParseErrorKind};
use roxmltree::{Document, Node, NodeType};
use std::collections::HashMap;

/// Maximum schema version supported by this framework release.
///
/// Files declaring a version higher than this will be rejected with an error.
/// Update this constant when the framework adds support for new schema versions.
pub const MAX_SUPPORTED_VERSION: SchemaVersion = SchemaVersion { major: 1, minor: 0 };

/// Parse a version string in "major.minor" format into a SchemaVersion.
///
/// # Arguments
///
/// * `version_str` - Raw version string from XML attribute (e.g., "1.0")
/// * `span` - Source location for error reporting
///
/// # Returns
///
/// `Ok(SchemaVersion)` on success, `Err(ParseError)` for invalid formats.
///
/// # Examples
///
/// ```ignore
/// let v = parse_version_string("1.0", span)?;
/// assert_eq!(v.major, 1);
/// assert_eq!(v.minor, 0);
/// ```
pub fn parse_version_string(version_str: &str, span: Span) -> Result<SchemaVersion, ParseError> {
    let trimmed = version_str.trim();

    // Reject empty strings
    if trimmed.is_empty() {
        return Err(ParseError {
            kind: ParseErrorKind::InvalidValue,
            message: "Version attribute cannot be empty".to_string(),
            span,
            suggestion: Some("Use format: version=\"1.0\"".to_string()),
        });
    }

    // Split on "." and validate exactly 2 parts
    let parts: Vec<&str> = trimmed.split('.').collect();
    if parts.len() != 2 {
        return Err(ParseError {
            kind: ParseErrorKind::InvalidValue,
            message: format!(
                "Invalid version format '{}'. Expected 'major.minor' (e.g., '1.0')",
                trimmed
            ),
            span,
            suggestion: Some("Use format: version=\"1.0\"".to_string()),
        });
    }

    // Parse major version
    let major = parts[0].parse::<u16>().map_err(|_| ParseError {
        kind: ParseErrorKind::InvalidValue,
        message: format!(
            "Invalid version format '{}'. Expected 'major.minor' (e.g., '1.0')",
            trimmed
        ),
        span,
        suggestion: Some("Use format: version=\"1.0\"".to_string()),
    })?;

    // Parse minor version
    let minor = parts[1].parse::<u16>().map_err(|_| ParseError {
        kind: ParseErrorKind::InvalidValue,
        message: format!(
            "Invalid version format '{}'. Expected 'major.minor' (e.g., '1.0')",
            trimmed
        ),
        span,
        suggestion: Some("Use format: version=\"1.0\"".to_string()),
    })?;

    Ok(SchemaVersion { major, minor })
}

/// Validate that a parsed version is supported by this framework.
///
/// # Arguments
///
/// * `version` - Parsed version to validate
/// * `span` - Source location for error reporting
///
/// # Returns
///
/// `Ok(())` if the version is supported, `Err(ParseError)` if the version
/// is newer than `MAX_SUPPORTED_VERSION`.
pub fn validate_version_supported(version: &SchemaVersion, span: Span) -> Result<(), ParseError> {
    if (version.major, version.minor) > (MAX_SUPPORTED_VERSION.major, MAX_SUPPORTED_VERSION.minor) {
        return Err(ParseError {
            kind: ParseErrorKind::UnsupportedVersion,
            message: format!(
                "Schema version {}.{} is not supported. Maximum supported version: {}.{}",
                version.major,
                version.minor,
                MAX_SUPPORTED_VERSION.major,
                MAX_SUPPORTED_VERSION.minor
            ),
            span,
            suggestion: Some(format!(
                "Upgrade dampen-core to support v{}.{}, or use version=\"{}.{}\"",
                version.major,
                version.minor,
                MAX_SUPPORTED_VERSION.major,
                MAX_SUPPORTED_VERSION.minor
            )),
        });
    }
    Ok(())
}

/// Warning about a widget requiring a higher schema version than declared.
///
/// This is non-blocking validation - widgets may work but compatibility is not guaranteed.
#[derive(Debug, Clone, PartialEq)]
pub struct ValidationWarning {
    /// The widget that requires a higher version
    pub widget_kind: WidgetKind,
    /// The schema version declared in the document
    pub declared_version: SchemaVersion,
    /// The minimum version required by the widget
    pub required_version: SchemaVersion,
    /// Source location of the widget
    pub span: Span,
}

impl ValidationWarning {
    /// Format the warning as a human-readable message
    pub fn format_message(&self) -> String {
        format!(
            "Widget '{}' requires schema v{}.{} but document declares v{}.{}",
            widget_kind_name(&self.widget_kind),
            self.required_version.major,
            self.required_version.minor,
            self.declared_version.major,
            self.declared_version.minor
        )
    }

    /// Get a suggestion for resolving the warning
    pub fn suggestion(&self) -> String {
        format!(
            "Update to <dampen version=\"{}.{}\"> or remove this widget",
            self.required_version.major, self.required_version.minor
        )
    }
}

/// Helper function to get widget kind name as string
fn widget_kind_name(kind: &WidgetKind) -> String {
    match kind {
        WidgetKind::Column => "column".to_string(),
        WidgetKind::Row => "row".to_string(),
        WidgetKind::Container => "container".to_string(),
        WidgetKind::Scrollable => "scrollable".to_string(),
        WidgetKind::Stack => "stack".to_string(),
        WidgetKind::Text => "text".to_string(),
        WidgetKind::Image => "image".to_string(),
        WidgetKind::Svg => "svg".to_string(),
        WidgetKind::Button => "button".to_string(),
        WidgetKind::TextInput => "text_input".to_string(),
        WidgetKind::Checkbox => "checkbox".to_string(),
        WidgetKind::Slider => "slider".to_string(),
        WidgetKind::PickList => "pick_list".to_string(),
        WidgetKind::Toggler => "toggler".to_string(),
        WidgetKind::Space => "space".to_string(),
        WidgetKind::Rule => "rule".to_string(),
        WidgetKind::Radio => "radio".to_string(),
        WidgetKind::ComboBox => "combobox".to_string(),
        WidgetKind::ProgressBar => "progress_bar".to_string(),
        WidgetKind::Tooltip => "tooltip".to_string(),
        WidgetKind::Grid => "grid".to_string(),
        WidgetKind::Canvas => "canvas".to_string(),
        WidgetKind::Float => "float".to_string(),
        WidgetKind::For => "for".to_string(),
        WidgetKind::Custom(name) => name.clone(),
    }
}

/// Validate that all widgets in the document are compatible with the declared schema version.
///
/// Returns warnings (not errors) for widgets that require a higher version than declared.
/// This is non-blocking validation to help developers identify potential compatibility issues.
///
/// # Arguments
///
/// * `document` - The parsed document to validate
///
/// # Returns
///
/// A vector of `ValidationWarning` for widgets requiring higher versions.
/// Empty vector means all widgets are compatible with the declared version.
///
/// # Examples
///
/// ```rust
/// use dampen_core::{parse, validate_widget_versions};
///
/// let xml = r#"<dampen version="1.0"><canvas width="400" height="200" program="{chart}" /></dampen>"#;
/// let doc = parse(xml).unwrap();
/// let warnings = validate_widget_versions(&doc);
/// assert_eq!(warnings.len(), 1); // Canvas requires v1.1
/// ```
pub fn validate_widget_versions(document: &DampenDocument) -> Vec<ValidationWarning> {
    let mut warnings = Vec::new();
    validate_widget_tree(&document.root, &document.version, &mut warnings);
    warnings
}

/// Recursively validate widget tree for version compatibility
fn validate_widget_tree(
    node: &WidgetNode,
    doc_version: &SchemaVersion,
    warnings: &mut Vec<ValidationWarning>,
) {
    let min_version = node.kind.minimum_version();

    // Check if widget requires a higher version than declared
    if (min_version.major, min_version.minor) > (doc_version.major, doc_version.minor) {
        warnings.push(ValidationWarning {
            widget_kind: node.kind.clone(),
            declared_version: *doc_version,
            required_version: min_version,
            span: node.span,
        });
    }

    // Recursively check children
    for child in &node.children {
        validate_widget_tree(child, doc_version, warnings);
    }
}

/// Parse XML markup into a DampenDocument.
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
/// `Ok(DampenDocument)` on success, `Err(ParseError)` on failure
///
/// # Examples
///
/// ```rust
/// use dampen_core::parse;
///
/// let xml = r#"<dampen><column><text value="Hello" /></column></dampen>"#;
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
pub fn parse(xml: &str) -> Result<DampenDocument, ParseError> {
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

    // Check if root is <dampen> wrapper
    let root_tag = root.tag_name().name();

    if root_tag == "dampen" {
        // Parse <dampen> document with themes and widgets
        parse_dampen_document(root, xml)
    } else {
        // Parse direct widget (backward compatibility)
        // Default to version 1.0 for backward compatibility
        let root_widget = parse_node(root, xml)?;

        Ok(DampenDocument {
            version: SchemaVersion::default(),
            root: root_widget,
            themes: HashMap::new(),
            style_classes: HashMap::new(),
            global_theme: None,
        })
    }
}

/// Validate widget-specific required attributes
fn validate_widget_attributes(
    kind: &WidgetKind,
    attributes: &std::collections::HashMap<String, AttributeValue>,
    span: Span,
) -> Result<(), ParseError> {
    match kind {
        WidgetKind::ComboBox | WidgetKind::PickList => {
            // Check for required 'options' attribute
            if let Some(AttributeValue::Static(options_value)) = attributes.get("options") {
                if options_value.trim().is_empty() {
                    return Err(ParseError {
                        kind: ParseErrorKind::MissingAttribute,
                        message: format!(
                            "{:?} widget requires 'options' attribute to be non-empty",
                            kind
                        ),
                        span,
                        suggestion: Some(
                            "Add a comma-separated list: options=\"Option1,Option2\"".to_string(),
                        ),
                    });
                }
            } else {
                return Err(ParseError {
                    kind: ParseErrorKind::MissingAttribute,
                    message: format!("{:?} widget requires 'options' attribute", kind),
                    span,
                    suggestion: Some(
                        "Add options attribute: options=\"Option1,Option2\"".to_string(),
                    ),
                });
            }
        }
        WidgetKind::Canvas => {
            // Check for required 'width' attribute
            if !attributes.contains_key("width") {
                return Err(ParseError {
                    kind: ParseErrorKind::MissingAttribute,
                    message: format!("{:?} widget requires 'width' attribute", kind),
                    span,
                    suggestion: Some("Add width attribute: width=\"400\"".to_string()),
                });
            }
            // Check for required 'height' attribute
            if !attributes.contains_key("height") {
                return Err(ParseError {
                    kind: ParseErrorKind::MissingAttribute,
                    message: format!("{:?} widget requires 'height' attribute", kind),
                    span,
                    suggestion: Some("Add height attribute: height=\"200\"".to_string()),
                });
            }
            // Check for required 'program' attribute
            if !attributes.contains_key("program") {
                return Err(ParseError {
                    kind: ParseErrorKind::MissingAttribute,
                    message: format!("{:?} widget requires 'program' attribute", kind),
                    span,
                    suggestion: Some("Add program attribute: program=\"{{chart}}\"".to_string()),
                });
            }

            // Validate width size range [50, 4000]
            if let Some(AttributeValue::Static(width_str)) = attributes.get("width") {
                if let Ok(width) = width_str.parse::<u32>() {
                    if !(50..=4000).contains(&width) {
                        return Err(ParseError {
                            kind: ParseErrorKind::InvalidValue,
                            message: format!(
                                "Canvas width must be between 50 and 4000 pixels, found {}",
                                width
                            ),
                            span,
                            suggestion: Some("Use width value between 50 and 4000".to_string()),
                        });
                    }
                }
            }

            // Validate height size range [50, 4000]
            if let Some(AttributeValue::Static(height_str)) = attributes.get("height") {
                if let Ok(height) = height_str.parse::<u32>() {
                    if !(50..=4000).contains(&height) {
                        return Err(ParseError {
                            kind: ParseErrorKind::InvalidValue,
                            message: format!(
                                "Canvas height must be between 50 and 4000 pixels, found {}",
                                height
                            ),
                            span,
                            suggestion: Some("Use height value between 50 and 4000".to_string()),
                        });
                    }
                }
            }
        }
        WidgetKind::Grid => {
            // Check for required 'columns' attribute
            if !attributes.contains_key("columns") {
                return Err(ParseError {
                    kind: ParseErrorKind::MissingAttribute,
                    message: format!("{:?} widget requires 'columns' attribute", kind),
                    span,
                    suggestion: Some("Add columns attribute: columns=\"5\"".to_string()),
                });
            }
            // Validate columns value range [1, 20]
            if let Some(AttributeValue::Static(cols)) = attributes.get("columns") {
                if let Ok(cols_num) = cols.parse::<u32>() {
                    if !(1..=20).contains(&cols_num) {
                        return Err(ParseError {
                            kind: ParseErrorKind::InvalidValue,
                            message: format!(
                                "Grid columns must be between 1 and 20, found {}",
                                cols_num
                            ),
                            span,
                            suggestion: Some("Use columns value between 1 and 20".to_string()),
                        });
                    }
                }
            }
        }
        WidgetKind::Tooltip => {
            // Check for required 'message' attribute
            if !attributes.contains_key("message") {
                return Err(ParseError {
                    kind: ParseErrorKind::MissingAttribute,
                    message: format!("{:?} widget requires 'message' attribute", kind),
                    span,
                    suggestion: Some("Add message attribute: message=\"Help text\"".to_string()),
                });
            }
        }
        WidgetKind::For => {
            // Check for required 'each' attribute
            if !attributes.contains_key("each") {
                return Err(ParseError {
                    kind: ParseErrorKind::MissingAttribute,
                    message: "For loop requires 'each' attribute to name the loop variable"
                        .to_string(),
                    span,
                    suggestion: Some("Add each attribute: each=\"item\"".to_string()),
                });
            }
            // Check for required 'in' attribute
            if !attributes.contains_key("in") {
                return Err(ParseError {
                    kind: ParseErrorKind::MissingAttribute,
                    message: "For loop requires 'in' attribute with collection binding".to_string(),
                    span,
                    suggestion: Some("Add in attribute: in=\"{items}\"".to_string()),
                });
            }
        }
        _ => {}
    }
    Ok(())
}

/// Validate Tooltip widget has exactly one child
fn validate_tooltip_children(children: &[WidgetNode], span: Span) -> Result<(), ParseError> {
    if children.is_empty() {
        return Err(ParseError {
            kind: ParseErrorKind::InvalidValue,
            message: "Tooltip widget must have exactly one child widget".to_string(),
            span,
            suggestion: Some("Wrap a single widget in <tooltip></tooltip>".to_string()),
        });
    }
    if children.len() > 1 {
        return Err(ParseError {
            kind: ParseErrorKind::InvalidValue,
            message: format!(
                "Tooltip widget must have exactly one child, found {}",
                children.len()
            ),
            span,
            suggestion: Some("Wrap only one widget in <tooltip></tooltip>".to_string()),
        });
    }
    Ok(())
}

/// Validate Canvas widget has no children (is a leaf widget)
fn validate_canvas_children(children: &[WidgetNode], span: Span) -> Result<(), ParseError> {
    if !children.is_empty() {
        return Err(ParseError {
            kind: ParseErrorKind::InvalidValue,
            message: format!(
                "Canvas widget cannot have children, found {}",
                children.len()
            ),
            span,
            suggestion: Some("Canvas is a leaf widget - remove child elements".to_string()),
        });
    }
    Ok(())
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
        "radio" => WidgetKind::Radio,
        "combobox" => WidgetKind::ComboBox,
        "progress_bar" => WidgetKind::ProgressBar,
        "tooltip" => WidgetKind::Tooltip,
        "grid" => WidgetKind::Grid,
        "canvas" => WidgetKind::Canvas,
        "float" => WidgetKind::Float,
        "for" => WidgetKind::For,
        _ => {
            return Err(ParseError {
                kind: ParseErrorKind::UnknownWidget,
                message: format!("Unknown widget: <{}>", tag_name),
                span: get_span(node, source),
                suggestion: Some("Did you mean one of the standard widgets?".to_string()),
            });
        }
    };

    // Parse attributes - separate breakpoint-prefixed from regular
    let mut attributes = std::collections::HashMap::new();
    let mut breakpoint_attributes = std::collections::HashMap::new();
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
                // Parse handler name and optional parameter
                // Syntax: "handler_name", "handler_name:{expression}", or "handler_name:'value'"
                let (handler_name, param) = if let Some(colon_pos) = value.find(':') {
                    let handler = value[..colon_pos].to_string();
                    let param_str = &value[colon_pos + 1..];

                    // Check for single-quoted string: 'value'
                    if param_str.starts_with('\'')
                        && param_str.ends_with('\'')
                        && param_str.len() >= 2
                    {
                        let quoted_value = &param_str[1..param_str.len() - 1];
                        // Create a static string binding expression
                        let expr = BindingExpr {
                            expr: Expr::Literal(LiteralExpr::String(quoted_value.to_string())),
                            span: Span::new(
                                colon_pos + 1,
                                colon_pos + 1 + param_str.len(),
                                1,
                                colon_pos as u32 + 1,
                            ),
                        };
                        (handler, Some(expr))
                    } else {
                        // Remove surrounding braces if present: {item.id} -> item.id
                        let param_clean = param_str.trim_matches('{').trim_matches('}');

                        // Parse parameter as binding expression
                        match crate::expr::tokenize_binding_expr(param_clean, 0, 1, 1) {
                            Ok(expr) => (handler, Some(expr)),
                            Err(_) => {
                                // If parsing fails, treat the whole string as handler name
                                (value.to_string(), None)
                            }
                        }
                    }
                } else {
                    (value.to_string(), None)
                };

                events.push(EventBinding {
                    event,
                    handler: handler_name,
                    param,
                    span: get_span(node, source),
                });
                continue;
            }
        }

        // Check for breakpoint-prefixed attributes (e.g., "mobile-spacing", "tablet-width")
        // Note: We use hyphen instead of colon to avoid XML namespace issues
        if let Some((prefix, attr_name)) = name.split_once('-') {
            if let Ok(breakpoint) = crate::ir::layout::Breakpoint::parse(prefix) {
                // Store in breakpoint_attributes map
                let attr_value = parse_attribute_value(value, get_span(node, source))?;
                breakpoint_attributes
                    .entry(breakpoint)
                    .or_insert_with(HashMap::new)
                    .insert(attr_name.to_string(), attr_value);
                continue;
            }
        }

        // Parse attribute value (check for bindings)
        let attr_value = parse_attribute_value(value, get_span(node, source))?;
        attributes.insert(name.to_string(), attr_value);
    }

    // Extract class attribute into classes field
    let classes = if let Some(AttributeValue::Static(class_attr)) = attributes.get("class") {
        class_attr
            .split_whitespace()
            .map(|s| s.to_string())
            .collect()
    } else {
        Vec::new()
    };

    // Extract theme attribute into theme_ref field (supports both static and binding)
    let theme_ref = attributes.get("theme").cloned();

    // Parse children
    let mut children = Vec::new();
    for child in node.children() {
        if child.node_type() == NodeType::Element {
            children.push(parse_node(child, source)?);
        }
    }

    // Validate Tooltip has exactly one child
    if kind == WidgetKind::Tooltip {
        validate_tooltip_children(&children, get_span(node, source))?;
    }

    // Validate Canvas has no children (leaf widget)
    if kind == WidgetKind::Canvas {
        validate_canvas_children(&children, get_span(node, source))?;
    }

    // Parse layout and style attributes into structured fields
    let layout = parse_layout_attributes(&kind, &attributes).map_err(|e| ParseError {
        kind: ParseErrorKind::InvalidValue,
        message: e,
        span: get_span(node, source),
        suggestion: None,
    })?;
    let style = parse_style_attributes(&attributes).map_err(|e| ParseError {
        kind: ParseErrorKind::InvalidValue,
        message: e,
        span: get_span(node, source),
        suggestion: None,
    })?;

    // Validate widget-specific required attributes
    validate_widget_attributes(&kind, &attributes, get_span(node, source))?;

    Ok(WidgetNode {
        kind,
        id,
        attributes,
        events,
        children,
        span: get_span(node, source),
        style,
        layout,
        theme_ref,
        classes,
        breakpoint_attributes,
    })
}

/// Parse a <dampen> document with themes and widgets
fn parse_dampen_document(root: Node, source: &str) -> Result<DampenDocument, ParseError> {
    let mut themes = HashMap::new();
    let mut style_classes = HashMap::new();
    let mut root_widget = None;
    let mut global_theme = None;

    // Parse version attribute from <dampen> root element
    let span = get_span(root, source);
    let version = if let Some(version_attr) = root.attribute("version") {
        let parsed = parse_version_string(version_attr, span)?;
        validate_version_supported(&parsed, span)?;
        parsed
    } else {
        // Default to version 1.0 for backward compatibility
        SchemaVersion::default()
    };

    // Iterate through children of <dampen>
    for child in root.children() {
        if child.node_type() != NodeType::Element {
            continue;
        }

        let tag_name = child.tag_name().name();

        match tag_name {
            "themes" => {
                // Parse themes section
                for theme_node in child.children() {
                    if theme_node.node_type() == NodeType::Element
                        && theme_node.tag_name().name() == "theme"
                    {
                        let theme =
                            crate::parser::theme_parser::parse_theme_from_node(theme_node, source)?;
                        let name = theme_node
                            .attribute("name")
                            .map(|s| s.to_string())
                            .unwrap_or_else(|| "default".to_string());
                        themes.insert(name, theme);
                    }
                }
            }
            "style_classes" | "classes" | "styles" => {
                // Parse style classes
                for class_node in child.children() {
                    if class_node.node_type() == NodeType::Element {
                        let tag = class_node.tag_name().name();
                        if tag == "class" || tag == "style" {
                            let class = crate::parser::theme_parser::parse_style_class_from_node(
                                class_node, source,
                            )?;
                            style_classes.insert(class.name.clone(), class);
                        }
                    }
                }
            }
            "global_theme" => {
                // Set global theme reference
                if let Some(theme_name) = child.attribute("name") {
                    global_theme = Some(theme_name.to_string());
                }
            }
            _ => {
                // This should be a widget - parse as root
                if root_widget.is_some() {
                    return Err(ParseError {
                        kind: ParseErrorKind::XmlSyntax,
                        message: "Multiple root widgets found in <dampen>".to_string(),
                        span: get_span(child, source),
                        suggestion: Some("Only one root widget is allowed".to_string()),
                    });
                }
                root_widget = Some(parse_node(child, source)?);
            }
        }
    }

    // Ensure we have a root widget
    let root_widget = root_widget.ok_or_else(|| ParseError {
        kind: ParseErrorKind::XmlSyntax,
        message: "No root widget found in <dampen>".to_string(),
        span: get_span(root, source),
        suggestion: Some("Add a widget like <column> or <row> inside <dampen>".to_string()),
    })?;

    Ok(DampenDocument {
        version,
        root: root_widget,
        themes,
        style_classes,
        global_theme,
    })
}

/// Parse comma-separated list into Vec<String>
pub fn parse_comma_separated(value: &str) -> Vec<String> {
    value
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

/// Parse a simple enum value (case-insensitive) and return the matched variant
pub fn parse_enum_value<T>(value: &str, valid_variants: &[&str]) -> Result<T, String>
where
    T: std::str::FromStr + std::fmt::Display,
{
    let normalized = value.trim().to_lowercase();
    for variant in valid_variants.iter() {
        if variant.to_lowercase() == normalized {
            return T::from_str(variant).map_err(|_| {
                format!(
                    "Failed to parse '{}' as {}",
                    variant,
                    std::any::type_name::<T>()
                )
            });
        }
    }
    Err(format!(
        "Invalid value '{}'. Valid options: {}",
        value,
        valid_variants.join(", ")
    ))
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

/// Parse layout-related attributes from the attributes map
fn parse_layout_attributes(
    kind: &WidgetKind,
    attributes: &HashMap<String, AttributeValue>,
) -> Result<Option<crate::ir::layout::LayoutConstraints>, String> {
    use crate::ir::layout::LayoutConstraints;
    use crate::parser::style_parser::{
        parse_alignment, parse_constraint, parse_float_attr, parse_int_attr, parse_justification,
        parse_length_attr, parse_padding_attr, parse_spacing,
    };

    let mut layout = LayoutConstraints::default();
    let mut has_any = false;

    // Parse width
    if let Some(AttributeValue::Static(value)) = attributes.get("width") {
        layout.width = Some(parse_length_attr(value)?);
        has_any = true;
    }

    // Parse height
    if let Some(AttributeValue::Static(value)) = attributes.get("height") {
        layout.height = Some(parse_length_attr(value)?);
        has_any = true;
    }

    // Parse min/max constraints
    if let Some(AttributeValue::Static(value)) = attributes.get("min_width") {
        layout.min_width = Some(parse_constraint(value)?);
        has_any = true;
    }

    if let Some(AttributeValue::Static(value)) = attributes.get("max_width") {
        layout.max_width = Some(parse_constraint(value)?);
        has_any = true;
    }

    if let Some(AttributeValue::Static(value)) = attributes.get("min_height") {
        layout.min_height = Some(parse_constraint(value)?);
        has_any = true;
    }

    if let Some(AttributeValue::Static(value)) = attributes.get("max_height") {
        layout.max_height = Some(parse_constraint(value)?);
        has_any = true;
    }

    // Parse padding
    if let Some(AttributeValue::Static(value)) = attributes.get("padding") {
        layout.padding = Some(parse_padding_attr(value)?);
        has_any = true;
    }

    // Parse spacing
    if let Some(AttributeValue::Static(value)) = attributes.get("spacing") {
        layout.spacing = Some(parse_spacing(value)?);
        has_any = true;
    }

    // Parse alignment
    if let Some(AttributeValue::Static(value)) = attributes.get("align_items") {
        layout.align_items = Some(parse_alignment(value)?);
        has_any = true;
    }

    if let Some(AttributeValue::Static(value)) = attributes.get("justify_content") {
        layout.justify_content = Some(parse_justification(value)?);
        has_any = true;
    }

    if let Some(AttributeValue::Static(value)) = attributes.get("align_self") {
        layout.align_self = Some(parse_alignment(value)?);
        has_any = true;
    }

    // Parse direct alignment (align_x, align_y)
    if let Some(AttributeValue::Static(value)) = attributes.get("align_x") {
        layout.align_x = Some(parse_alignment(value)?);
        has_any = true;
    }

    if let Some(AttributeValue::Static(value)) = attributes.get("align_y") {
        layout.align_y = Some(parse_alignment(value)?);
        has_any = true;
    }

    // Parse align shorthand (sets both align_items and justify_content)
    if let Some(AttributeValue::Static(value)) = attributes.get("align") {
        let alignment = parse_alignment(value)?;
        layout.align_items = Some(alignment);
        layout.justify_content = Some(match alignment {
            crate::ir::layout::Alignment::Start => crate::ir::layout::Justification::Start,
            crate::ir::layout::Alignment::Center => crate::ir::layout::Justification::Center,
            crate::ir::layout::Alignment::End => crate::ir::layout::Justification::End,
            crate::ir::layout::Alignment::Stretch => crate::ir::layout::Justification::Center,
        });
        has_any = true;
    }

    // Parse direction
    if let Some(AttributeValue::Static(value)) = attributes.get("direction") {
        layout.direction = Some(crate::ir::layout::Direction::parse(value)?);
        has_any = true;
    }

    // Parse position (skip for Tooltip - it has its own position attribute)
    if !matches!(kind, WidgetKind::Tooltip) {
        if let Some(AttributeValue::Static(value)) = attributes.get("position") {
            layout.position = Some(crate::ir::layout::Position::parse(value)?);
            has_any = true;
        }
    }

    // Parse position offsets
    if let Some(AttributeValue::Static(value)) = attributes.get("top") {
        layout.top = Some(parse_float_attr(value, "top")?);
        has_any = true;
    }

    if let Some(AttributeValue::Static(value)) = attributes.get("right") {
        layout.right = Some(parse_float_attr(value, "right")?);
        has_any = true;
    }

    if let Some(AttributeValue::Static(value)) = attributes.get("bottom") {
        layout.bottom = Some(parse_float_attr(value, "bottom")?);
        has_any = true;
    }

    if let Some(AttributeValue::Static(value)) = attributes.get("left") {
        layout.left = Some(parse_float_attr(value, "left")?);
        has_any = true;
    }

    // Parse z-index
    if let Some(AttributeValue::Static(value)) = attributes.get("z_index") {
        layout.z_index = Some(parse_int_attr(value, "z_index")?);
        has_any = true;
    }

    // Validate the layout
    if has_any {
        layout
            .validate()
            .map_err(|e| format!("Layout validation failed: {}", e))?;
        Ok(Some(layout))
    } else {
        Ok(None)
    }
}

/// Parse style-related attributes from the attributes map
fn parse_style_attributes(
    attributes: &HashMap<String, AttributeValue>,
) -> Result<Option<crate::ir::style::StyleProperties>, String> {
    use crate::parser::style_parser::{
        build_border, build_style_properties, parse_background_attr, parse_border_color,
        parse_border_radius, parse_border_style, parse_border_width, parse_color_attr,
        parse_opacity, parse_shadow_attr, parse_transform,
    };

    let mut background = None;
    let mut color = None;
    let mut border_width = None;
    let mut border_color = None;
    let mut border_radius = None;
    let mut border_style = None;
    let mut shadow = None;
    let mut opacity = None;
    let mut transform = None;
    let mut has_any = false;

    // Parse background
    if let Some(AttributeValue::Static(value)) = attributes.get("background") {
        background = Some(parse_background_attr(value)?);
        has_any = true;
    }

    // Parse color
    if let Some(AttributeValue::Static(value)) = attributes.get("color") {
        color = Some(parse_color_attr(value)?);
        has_any = true;
    }

    // Parse border attributes
    if let Some(AttributeValue::Static(value)) = attributes.get("border_width") {
        border_width = Some(parse_border_width(value)?);
        has_any = true;
    }

    if let Some(AttributeValue::Static(value)) = attributes.get("border_color") {
        border_color = Some(parse_border_color(value)?);
        has_any = true;
    }

    if let Some(AttributeValue::Static(value)) = attributes.get("border_radius") {
        border_radius = Some(parse_border_radius(value)?);
        has_any = true;
    }

    if let Some(AttributeValue::Static(value)) = attributes.get("border_style") {
        border_style = Some(parse_border_style(value)?);
        has_any = true;
    }

    // Parse shadow
    if let Some(AttributeValue::Static(value)) = attributes.get("shadow") {
        shadow = Some(parse_shadow_attr(value)?);
        has_any = true;
    }

    // Parse opacity
    if let Some(AttributeValue::Static(value)) = attributes.get("opacity") {
        opacity = Some(parse_opacity(value)?);
        has_any = true;
    }

    // Parse transform
    if let Some(AttributeValue::Static(value)) = attributes.get("transform") {
        transform = Some(parse_transform(value)?);
        has_any = true;
    }

    if has_any {
        let border = build_border(border_width, border_color, border_radius, border_style)?;
        let style = build_style_properties(background, color, border, shadow, opacity, transform)?;
        Ok(Some(style))
    } else {
        Ok(None)
    }
}

/// Validates that there are no circular dependencies in UI file includes
///
/// **T125**: Currently, Dampen does not support file includes/imports in XML,
/// so circular dependencies are not possible. This function is a placeholder
/// for future validation when UI file composition is added.
///
/// # Future Implementation
///
/// When UI file includes are added (e.g., `<include src="header.dampen" />`),
/// this function should:
/// 1. Build a dependency graph of all included files
/// 2. Detect cycles using depth-first search or topological sort
/// 3. Return ParseError with the cycle path if detected
///
/// # Arguments
///
/// * `file_path` - The root UI file being parsed
/// * `visited` - Set of already visited files (for cycle detection)
///
/// # Returns
///
/// `Ok(())` if no circular dependencies exist, or `Err(ParseError)` with
/// the dependency cycle information.
///
/// # Example Error Message
///
/// ```text
/// Circular UI file dependency detected:
///   app.dampen -> header.dampen -> nav.dampen -> app.dampen
/// ```
#[allow(dead_code)]
pub fn validate_no_circular_dependencies(
    _file_path: &std::path::Path,
    _visited: &mut std::collections::HashSet<std::path::PathBuf>,
) -> Result<(), ParseError> {
    // Placeholder: No includes supported yet, so no circular dependencies possible
    Ok(())
}

#[cfg(test)]
mod circular_dependency_tests {
    use super::*;
    use std::collections::HashSet;
    use std::path::PathBuf;

    #[test]
    fn test_no_circular_dependencies_without_includes() {
        // T125: Validate that single files have no circular dependencies
        let file_path = PathBuf::from("test.dampen");
        let mut visited = HashSet::new();

        let result = validate_no_circular_dependencies(&file_path, &mut visited);
        assert!(
            result.is_ok(),
            "Single file should have no circular dependencies"
        );
    }

    // Future tests when includes are supported:
    // - test_detect_simple_circular_dependency: A -> B -> A
    // - test_detect_complex_circular_dependency: A -> B -> C -> D -> B
    // - test_allow_diamond_dependencies: A->B, A->C, B->D, C->D (this is OK, not circular)
    // - test_self_include_rejected: A -> A
}
