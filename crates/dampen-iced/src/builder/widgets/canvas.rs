//! Canvas widget builder

use crate::HandlerMessage;
use crate::builder::DampenWidgetBuilder;
use crate::canvas::events::{CanvasEventHandlers, CanvasHandlerNames};
use crate::canvas::{
    CanvasContent, CanvasProgramWrapper, CanvasShape, CircleShape, CustomProgramContainer,
    DeclarativeProgram, GroupShape, LineShape, RectShape, TextShape, Transform,
};
use dampen_core::binding::BindingValue;
use dampen_core::ir::WidgetKind;
use dampen_core::ir::node::{AttributeValue, WidgetNode};
use iced::{Color, Element, Length, Renderer, Theme};

impl<'a> DampenWidgetBuilder<'a> {
    /// Build a Canvas widget from a node
    pub(in crate::builder) fn build_canvas(
        &self,
        node: &WidgetNode,
    ) -> Element<'a, HandlerMessage, Theme, Renderer>
    where
        HandlerMessage: Clone + 'static,
    {
        // Parse width and height
        let width = self
            .resolve_length(node, "width")
            .unwrap_or(Length::Fixed(400.0));
        let height = self
            .resolve_length(node, "height")
            .unwrap_or(Length::Fixed(300.0));

        // Check for custom program binding
        let content = if let Some(program_attr) = node.attributes.get("program") {
            let val = self.evaluate_attribute_value(program_attr);
            if let BindingValue::Custom(arc) = val {
                if let Ok(container) = arc.clone().downcast::<CustomProgramContainer<()>>() {
                    // Extract Arc<dyn DampenCanvasProgram<()>> from container
                    CanvasContent::Custom(container.0.clone())
                } else if let Ok(_container) = arc
                    .clone()
                    .downcast::<CustomProgramContainer<HandlerMessage>>()
                {
                    #[cfg(debug_assertions)]
                    eprintln!(
                        "Warning: Custom program using CustomProgramContainer<HandlerMessage>. For better compatibility, use CustomProgramContainer<()>"
                    );

                    // We can't easily convert dyn DampenCanvasProgram<HandlerMessage> to dyn DampenCanvasProgram<()>
                    // so for now we fallback to empty.
                    CanvasContent::Declarative(DeclarativeProgram::new(vec![]))
                } else {
                    #[cfg(debug_assertions)]
                    eprintln!(
                        "Failed to downcast custom program binding. Expected CustomProgramContainer<()>"
                    );
                    CanvasContent::Declarative(DeclarativeProgram::new(vec![]))
                }
            } else {
                // Fallback to shapes if program attribute is present but not a Custom binding
                let shapes = self.parse_canvas_shapes(&node.children);
                CanvasContent::Declarative(DeclarativeProgram::new(shapes))
            }
        } else {
            // Parse shapes (declarative)
            let shapes = self.parse_canvas_shapes(&node.children);
            CanvasContent::Declarative(DeclarativeProgram::new(shapes))
        };

        // Parse event handlers
        let handlers = self.parse_canvas_handlers(node);

        // Apply handlers if declarative
        let content = if let CanvasContent::Declarative(mut prog) = content {
            if let Some(handlers) = handlers {
                prog = prog.with_handlers(handlers);
            }
            CanvasContent::Declarative(prog)
        } else {
            content
        };

        let program_wrapper = CanvasProgramWrapper::new(content);

        iced::widget::canvas(program_wrapper)
            .width(width)
            .height(height)
            .into()
    }

    fn parse_canvas_handlers(
        &self,
        node: &WidgetNode,
    ) -> Option<CanvasEventHandlers<HandlerMessage>> {
        let on_click = self.get_handler_name(node, dampen_core::ir::EventKind::CanvasClick);
        let on_drag = self.get_handler_name(node, dampen_core::ir::EventKind::CanvasDrag);
        let on_move = self.get_handler_name(node, dampen_core::ir::EventKind::CanvasMove);
        let on_release = self.get_handler_name(node, dampen_core::ir::EventKind::CanvasRelease);

        if on_click.is_none() && on_drag.is_none() && on_move.is_none() && on_release.is_none() {
            return None;
        }

        Some(CanvasEventHandlers {
            handler_names: CanvasHandlerNames {
                on_click,
                on_drag,
                on_move,
                on_release,
            },
            msg_factory: |name, event| {
                // Serialize event to JSON string
                let json = serde_json::to_string(&event).unwrap_or_default();
                HandlerMessage::Handler(name.to_string(), Some(json))
            },
        })
    }

    fn get_handler_name(
        &self,
        node: &WidgetNode,
        kind: dampen_core::ir::EventKind,
    ) -> Option<String> {
        node.events
            .iter()
            .find(|e| e.event == kind)
            .map(|e| e.handler.clone())
    }

    fn parse_canvas_shapes(&self, nodes: &[WidgetNode]) -> Vec<CanvasShape> {
        let mut shapes = Vec::new();
        for node in nodes {
            if let Some(shape) = self.parse_canvas_shape(node) {
                shapes.push(shape);
            }
        }
        shapes
    }

    fn parse_canvas_shape(&self, node: &WidgetNode) -> Option<CanvasShape> {
        match node.kind {
            WidgetKind::CanvasRect => {
                let x = self.resolve_f32(node, "x", 0.0);
                let y = self.resolve_f32(node, "y", 0.0);
                let width = self.resolve_f32(node, "width", 0.0);
                let height = self.resolve_f32(node, "height", 0.0);
                let fill = self.resolve_color(node, "fill");
                let stroke = self.resolve_color(node, "stroke");
                let stroke_width = self.resolve_f32(node, "stroke_width", 1.0);
                let radius = self.resolve_f32(node, "radius", 0.0);

                Some(CanvasShape::Rect(RectShape {
                    x,
                    y,
                    width,
                    height,
                    fill,
                    stroke,
                    stroke_width,
                    radius,
                }))
            }
            WidgetKind::CanvasCircle => {
                let cx = self.resolve_f32(node, "cx", 0.0);
                let cy = self.resolve_f32(node, "cy", 0.0);
                let radius = self.resolve_f32(node, "radius", 0.0);
                let fill = self.resolve_color(node, "fill");
                let stroke = self.resolve_color(node, "stroke");
                let stroke_width = self.resolve_f32(node, "stroke_width", 1.0);

                Some(CanvasShape::Circle(CircleShape {
                    cx,
                    cy,
                    radius,
                    fill,
                    stroke,
                    stroke_width,
                }))
            }
            WidgetKind::CanvasLine => {
                let x1 = self.resolve_f32(node, "x1", 0.0);
                let y1 = self.resolve_f32(node, "y1", 0.0);
                let x2 = self.resolve_f32(node, "x2", 0.0);
                let y2 = self.resolve_f32(node, "y2", 0.0);
                let stroke = self.resolve_color(node, "stroke");
                let stroke_width = self.resolve_f32(node, "stroke_width", 1.0);

                Some(CanvasShape::Line(LineShape {
                    x1,
                    y1,
                    x2,
                    y2,
                    stroke,
                    stroke_width,
                }))
            }
            WidgetKind::CanvasText => {
                let x = self.resolve_f32(node, "x", 0.0);
                let y = self.resolve_f32(node, "y", 0.0);
                // Use content attribute
                let content_attr = node
                    .attributes
                    .get("content")
                    .map(|a| self.evaluate_attribute(a))
                    .unwrap_or_default();

                let size = self.resolve_f32(node, "size", 16.0);
                let color = self.resolve_color(node, "color");

                Some(CanvasShape::Text(TextShape {
                    x,
                    y,
                    content: content_attr,
                    size,
                    color,
                }))
            }
            WidgetKind::CanvasGroup => {
                let transform = self.resolve_transform(node);
                let children = self.parse_canvas_shapes(&node.children);
                Some(CanvasShape::Group(GroupShape {
                    transform,
                    children,
                }))
            }
            WidgetKind::For => {
                // T049: Add support for for-each loops inside canvas
                let mut shapes = Vec::new();

                // Get collection binding
                if let Some(in_attr) = node.attributes.get("in") {
                    // This evaluates to a collection (BindingValue::List or similar)
                    let collection_val = match in_attr {
                        AttributeValue::Binding(expr) => {
                            self.evaluate_binding_with_context(expr).ok()
                        }
                        _ => None,
                    };

                    if let Some(dampen_core::binding::BindingValue::List(items)) = collection_val {
                        // Get variable name
                        let var_name = match node.attributes.get("each") {
                            Some(AttributeValue::Static(s)) => s.clone(),
                            _ => "item".to_string(),
                        };

                        // Iterate items
                        for item in items {
                            // Push context
                            self.push_context(&var_name, item);

                            // Parse children recursively
                            let child_shapes = self.parse_canvas_shapes(&node.children);
                            for s in child_shapes {
                                shapes.push(s);
                            }

                            // Pop context
                            self.pop_context();
                        }
                    }
                }

                // We wrap them in a Group with no transform.
                if shapes.is_empty() {
                    None
                } else {
                    Some(CanvasShape::Group(GroupShape {
                        transform: None,
                        children: shapes,
                    }))
                }
            }
            WidgetKind::If => {
                // T050: Handle If
                let condition = self.resolve_boolean_attribute(node, "condition", false);
                if condition {
                    let children = self.parse_canvas_shapes(&node.children);
                    if children.is_empty() {
                        None
                    } else {
                        Some(CanvasShape::Group(GroupShape {
                            transform: None,
                            children,
                        }))
                    }
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    // Helper wrapper for resolve_boolean_attribute since it's defined in helpers but might be useful here
    fn resolve_boolean_attribute(&self, node: &WidgetNode, name: &str, default: bool) -> bool {
        crate::builder::helpers::resolve_boolean_attribute(self, node, name, default)
    }

    fn resolve_f32(&self, node: &WidgetNode, name: &str, default: f32) -> f32 {
        if let Some(attr) = node.attributes.get(name) {
            let s = self.evaluate_attribute(attr);
            s.parse::<f32>().unwrap_or(default)
        } else {
            default
        }
    }

    fn resolve_color(&self, node: &WidgetNode, name: &str) -> Option<Color> {
        if let Some(attr) = node.attributes.get(name) {
            let s = self.evaluate_attribute(attr);
            use crate::builder::helpers::parse_color;
            parse_color(&s).map(|c| Color {
                r: c.r,
                g: c.g,
                b: c.b,
                a: c.a,
            })
        } else {
            None
        }
    }

    fn resolve_length(&self, node: &WidgetNode, name: &str) -> Option<Length> {
        if let Some(attr) = node.attributes.get(name) {
            let s = self.evaluate_attribute(attr);
            crate::builder::helpers::parse_length(&s)
        } else {
            None
        }
    }

    fn resolve_transform(&self, node: &WidgetNode) -> Option<Transform> {
        if let Some(attr) = node.attributes.get("transform") {
            let s = self.evaluate_attribute(attr);
            self.parse_transform_string(&s)
        } else {
            None
        }
    }

    fn parse_transform_string(&self, s: &str) -> Option<Transform> {
        let s = s.trim();
        if let Some(inner) = s
            .strip_prefix("translate(")
            .and_then(|s| s.strip_suffix(")"))
        {
            let parts: Vec<f32> = inner
                .split(',')
                .filter_map(|p| p.trim().parse().ok())
                .collect();
            if parts.len() == 2 {
                return Some(Transform::Translate(parts[0], parts[1]));
            }
        }
        if let Some(inner) = s.strip_prefix("rotate(").and_then(|s| s.strip_suffix(")"))
            && let Ok(angle) = inner.trim().parse::<f32>()
        {
            return Some(Transform::Rotate(angle));
        }
        if let Some(inner) = s.strip_prefix("scale(").and_then(|s| s.strip_suffix(")")) {
            let parts: Vec<f32> = inner
                .split(',')
                .filter_map(|p| p.trim().parse().ok())
                .collect();
            if parts.len() == 1 {
                return Some(Transform::Scale(parts[0]));
            } else if parts.len() == 2 {
                return Some(Transform::ScaleXY(parts[0], parts[1]));
            }
        }
        if let Some(inner) = s.strip_prefix("matrix(").and_then(|s| s.strip_suffix(")")) {
            let parts: Vec<f32> = inner
                .split(',')
                .filter_map(|p| p.trim().parse().ok())
                .collect();
            if parts.len() == 6 {
                let mut matrix = [0.0; 6];
                matrix.copy_from_slice(&parts);
                return Some(Transform::Matrix(matrix));
            }
        }
        None
    }
}
