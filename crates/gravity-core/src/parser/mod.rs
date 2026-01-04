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

    // Check if root is <gravity> wrapper
    let root_tag = root.tag_name().name();

    if root_tag == "gravity" {
        // Parse <gravity> document with themes and widgets
        parse_gravity_document(root, xml)
    } else {
        // Parse direct widget (backward compatibility)
        let root_widget = parse_node(root, xml)?;

        Ok(GravityDocument {
            version: SchemaVersion { major: 1, minor: 0 },
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
            // Check for required 'width' and 'height' attributes
            if !attributes.contains_key("width") {
                return Err(ParseError {
                    kind: ParseErrorKind::MissingAttribute,
                    message: format!("{:?} widget requires 'width' attribute", kind),
                    span,
                    suggestion: Some("Add width attribute: width=\"400\"".to_string()),
                });
            }
            if !attributes.contains_key("height") {
                return Err(ParseError {
                    kind: ParseErrorKind::MissingAttribute,
                    message: format!("{:?} widget requires 'height' attribute", kind),
                    span,
                    suggestion: Some("Add height attribute: height=\"200\"".to_string()),
                });
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
        "combobox" => WidgetKind::ComboBox,
        "progress_bar" => WidgetKind::ProgressBar,
        "tooltip" => WidgetKind::Tooltip,
        "grid" => WidgetKind::Grid,
        "canvas" => WidgetKind::Canvas,
        "float" => WidgetKind::Float,
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
                events.push(EventBinding {
                    event,
                    handler: value.to_string(),
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
        theme_ref: None,
        classes,
        breakpoint_attributes,
    })
}

/// Parse a <gravity> document with themes and widgets
fn parse_gravity_document(root: Node, source: &str) -> Result<GravityDocument, ParseError> {
    let mut themes = HashMap::new();
    let mut style_classes = HashMap::new();
    let mut root_widget = None;
    let mut global_theme = None;

    // Iterate through children of <gravity>
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
                        message: "Multiple root widgets found in <gravity>".to_string(),
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
        message: "No root widget found in <gravity>".to_string(),
        span: get_span(root, source),
        suggestion: Some("Add a widget like <column> or <row> inside <gravity>".to_string()),
    })?;

    Ok(GravityDocument {
        version: SchemaVersion { major: 1, minor: 0 },
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
pub fn parse_enum_value<T: std::str::FromStr>(
    value: &str,
    valid_variants: &[&str],
) -> Result<T, String>
where
    T: std::fmt::Display,
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
