//! Runtime shape definitions for the Canvas widget.

use iced::Color;

/// A runtime representation of a shape that can be drawn on a canvas.
#[derive(Debug, Clone, PartialEq)]
pub enum CanvasShape {
    /// A rectangle shape.
    Rect(RectShape),
    /// A circle shape.
    Circle(CircleShape),
    /// A line segment.
    Line(LineShape),
    /// Text drawn on the canvas.
    Text(TextShape),
    /// A group of shapes with an optional transformation.
    Group(GroupShape),
}

/// A rectangle shape with optional fill, stroke, and rounded corners.
#[derive(Debug, Clone, PartialEq)]
pub struct RectShape {
    /// The X coordinate of the top-left corner.
    pub x: f32,
    /// The Y coordinate of the top-left corner.
    pub y: f32,
    /// The width of the rectangle.
    pub width: f32,
    /// The height of the rectangle.
    pub height: f32,
    /// The fill color of the rectangle.
    pub fill: Option<Color>,
    /// The stroke color of the rectangle.
    pub stroke: Option<Color>,
    /// The width of the stroke.
    pub stroke_width: f32,
    /// The radius of the corners for a rounded rectangle.
    pub radius: f32,
}

/// A circle shape with optional fill and stroke.
#[derive(Debug, Clone, PartialEq)]
pub struct CircleShape {
    /// The X coordinate of the center.
    pub cx: f32,
    /// The Y coordinate of the center.
    pub cy: f32,
    /// The radius of the circle.
    pub radius: f32,
    /// The fill color of the circle.
    pub fill: Option<Color>,
    /// The stroke color of the circle.
    pub stroke: Option<Color>,
    /// The width of the stroke.
    pub stroke_width: f32,
}

/// A line segment between two points.
#[derive(Debug, Clone, PartialEq)]
pub struct LineShape {
    /// The X coordinate of the start point.
    pub x1: f32,
    /// The Y coordinate of the start point.
    pub y1: f32,
    /// The X coordinate of the end point.
    pub x2: f32,
    /// The Y coordinate of the end point.
    pub y2: f32,
    /// The stroke color of the line.
    pub stroke: Option<Color>,
    /// The width of the stroke.
    pub stroke_width: f32,
}

/// A text element drawn at a specific position on the canvas.
#[derive(Debug, Clone, PartialEq)]
pub struct TextShape {
    /// The X coordinate of the text baseline.
    pub x: f32,
    /// The Y coordinate of the text baseline.
    pub y: f32,
    /// The string content to display.
    pub content: String,
    /// The font size.
    pub size: f32,
    /// The text color.
    pub color: Option<Color>,
}

/// A container for multiple shapes that can be transformed as a single unit.
#[derive(Debug, Clone, PartialEq)]
pub struct GroupShape {
    /// An optional transformation to apply to all children.
    pub transform: Option<Transform>,
    /// The child shapes within this group.
    pub children: Vec<CanvasShape>,
}

/// Geometric transformations that can be applied to groups.
#[derive(Debug, Clone, PartialEq)]
pub enum Transform {
    /// Translate the coordinate system by (x, y).
    Translate(f32, f32),
    /// Rotate the coordinate system by an angle in radians.
    Rotate(f32),
    /// Uniformly scale the coordinate system.
    Scale(f32),
    /// Scale the coordinate system non-uniformly by (x, y).
    ScaleXY(f32, f32),
    /// A 2D transformation matrix [a, b, c, d, e, f].
    Matrix([f32; 6]),
}
