//! Styling system types for Dampen UI framework
//!
//! This module defines the IR types for visual styling properties including
//! backgrounds, colors, borders, shadows, opacity, and transforms.
//! All types are backend-agnostic and serializable.

use serde::{Deserialize, Serialize};

/// Complete style properties for a widget
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct StyleProperties {
    /// Background fill
    pub background: Option<Background>,
    /// Foreground/text color
    pub color: Option<Color>,
    /// Border styling
    pub border: Option<Border>,
    /// Drop shadow
    pub shadow: Option<Shadow>,
    /// Opacity (0.0 = transparent, 1.0 = opaque)
    pub opacity: Option<f32>,
    /// Visual transformations
    pub transform: Option<Transform>,
}

impl StyleProperties {
    /// Validates all style properties
    ///
    /// Returns an error if:
    /// - Opacity is not in 0.0-1.0 range
    /// - Colors are invalid
    pub fn validate(&self) -> Result<(), String> {
        if let Some(opacity) = self.opacity {
            if !(0.0..=1.0).contains(&opacity) {
                return Err(format!("opacity must be 0.0-1.0, got {}", opacity));
            }
        }

        if let Some(ref color) = self.color {
            color.validate()?;
        }

        if let Some(ref background) = self.background {
            background.validate()?;
        }

        if let Some(ref border) = self.border {
            border.validate()?;
        }

        Ok(())
    }
}

/// Background fill type
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Background {
    /// Solid color
    Color(Color),
    /// Gradient fill
    Gradient(Gradient),
    /// Image background
    Image { path: String, fit: ImageFit },
}

impl Background {
    pub fn validate(&self) -> Result<(), String> {
        match self {
            Background::Color(color) => color.validate(),
            Background::Gradient(gradient) => gradient.validate(),
            Background::Image { .. } => Ok(()),
        }
    }
}

/// Image fitting strategy
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ImageFit {
    Fill,
    Contain,
    Cover,
    ScaleDown,
}

/// Color representation (RGBA, 0.0-1.0 range)
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    /// Parse color from CSS string
    ///
    /// Supports:
    /// - Hex: "#3498db", "#3498dbff"
    /// - RGB: "rgb(52, 152, 219)", "rgba(52, 152, 219, 0.8)"
    /// - HSL: "hsl(204, 70%, 53%)", "hsla(204, 70%, 53%, 0.8)"
    /// - Named: "red", "blue", "transparent"
    pub fn parse(s: &str) -> Result<Self, String> {
        let css_color =
            csscolorparser::parse(s).map_err(|e| format!("Invalid color '{}': {}", s, e))?;

        let [r, g, b, a] = css_color.to_array();

        Ok(Color {
            r: r as f32,
            g: g as f32,
            b: b as f32,
            a: a as f32,
        })
    }

    /// Validate color values
    pub fn validate(&self) -> Result<(), String> {
        if self.r < 0.0 || self.r > 1.0 {
            return Err(format!("Red component out of range: {}", self.r));
        }
        if self.g < 0.0 || self.g > 1.0 {
            return Err(format!("Green component out of range: {}", self.g));
        }
        if self.b < 0.0 || self.b > 1.0 {
            return Err(format!("Blue component out of range: {}", self.b));
        }
        if self.a < 0.0 || self.a > 1.0 {
            return Err(format!("Alpha component out of range: {}", self.a));
        }
        Ok(())
    }
}

/// Gradient fill
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Gradient {
    Linear {
        angle: f32,
        stops: Vec<ColorStop>,
    },
    Radial {
        shape: RadialShape,
        stops: Vec<ColorStop>,
    },
}

impl Gradient {
    /// Validate gradient
    ///
    /// Returns an error if:
    /// - Less than 2 or more than 8 color stops (Iced limitation)
    /// - Color stop offsets not sorted or out of range
    /// - Angle not normalized
    pub fn validate(&self) -> Result<(), String> {
        let stops = match self {
            Gradient::Linear { angle, stops } => {
                // Normalize angle to 0.0-360.0
                if *angle < 0.0 || *angle > 360.0 {
                    return Err(format!("Gradient angle must be 0.0-360.0, got {}", angle));
                }
                stops
            }
            Gradient::Radial { stops, .. } => stops,
        };

        if stops.len() < 2 {
            return Err("Gradient must have at least 2 color stops".to_string());
        }

        if stops.len() > 8 {
            return Err(
                "Gradient cannot have more than 8 color stops (Iced limitation)".to_string(),
            );
        }

        let mut last_offset = -1.0;
        for stop in stops {
            if stop.offset < 0.0 || stop.offset > 1.0 {
                return Err(format!(
                    "Color stop offset must be 0.0-1.0, got {}",
                    stop.offset
                ));
            }

            if stop.offset <= last_offset {
                return Err("Color stop offsets must be in ascending order".to_string());
            }

            stop.color.validate()?;
            last_offset = stop.offset;
        }

        Ok(())
    }
}

/// Color stop for gradients
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ColorStop {
    pub color: Color,
    /// Offset in gradient (0.0 = start, 1.0 = end)
    pub offset: f32,
}

/// Radial gradient shape
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RadialShape {
    Circle,
    Ellipse,
}

/// Border styling
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Border {
    pub width: f32,
    pub color: Color,
    pub radius: BorderRadius,
    pub style: BorderStyle,
}

impl Border {
    pub fn validate(&self) -> Result<(), String> {
        if self.width < 0.0 {
            return Err(format!(
                "Border width must be non-negative, got {}",
                self.width
            ));
        }
        self.color.validate()?;
        self.radius.validate()?;
        Ok(())
    }
}

/// Border radius (corner rounding)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BorderRadius {
    pub top_left: f32,
    pub top_right: f32,
    pub bottom_right: f32,
    pub bottom_left: f32,
}

impl BorderRadius {
    /// Parse from string
    ///
    /// # Formats
    /// - `"<all>"`: All corners (e.g., "8")
    /// - `"<tl> <tr> <br> <bl>"`: Individual corners
    pub fn parse(s: &str) -> Result<Self, String> {
        let parts: Vec<&str> = s.split_whitespace().collect();

        match parts.len() {
            1 => {
                let all: f32 = parts[0]
                    .parse()
                    .map_err(|_| format!("Invalid border radius: {}", s))?;
                Ok(BorderRadius {
                    top_left: all,
                    top_right: all,
                    bottom_right: all,
                    bottom_left: all,
                })
            }
            4 => {
                let tl: f32 = parts[0]
                    .parse()
                    .map_err(|_| format!("Invalid top-left radius: {}", parts[0]))?;
                let tr: f32 = parts[1]
                    .parse()
                    .map_err(|_| format!("Invalid top-right radius: {}", parts[1]))?;
                let br: f32 = parts[2]
                    .parse()
                    .map_err(|_| format!("Invalid bottom-right radius: {}", parts[2]))?;
                let bl: f32 = parts[3]
                    .parse()
                    .map_err(|_| format!("Invalid bottom-left radius: {}", parts[3]))?;
                Ok(BorderRadius {
                    top_left: tl,
                    top_right: tr,
                    bottom_right: br,
                    bottom_left: bl,
                })
            }
            _ => Err(format!(
                "Invalid border radius format: '{}'. Expected 1 or 4 values",
                s
            )),
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.top_left < 0.0
            || self.top_right < 0.0
            || self.bottom_right < 0.0
            || self.bottom_left < 0.0
        {
            return Err("Border radius values must be non-negative".to_string());
        }
        Ok(())
    }
}

/// Border line style
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BorderStyle {
    Solid,
    Dashed,
    Dotted,
}

impl BorderStyle {
    pub fn parse(s: &str) -> Result<Self, String> {
        match s.trim().to_lowercase().as_str() {
            "solid" => Ok(BorderStyle::Solid),
            "dashed" => Ok(BorderStyle::Dashed),
            "dotted" => Ok(BorderStyle::Dotted),
            _ => Err(format!(
                "Invalid border style: '{}'. Expected solid, dashed, or dotted",
                s
            )),
        }
    }
}

/// Drop shadow
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Shadow {
    pub offset_x: f32,
    pub offset_y: f32,
    pub blur_radius: f32,
    pub color: Color,
}

impl Shadow {
    /// Parse from string format: "offset_x offset_y blur color"
    ///
    /// # Example
    /// ```rust
    /// use dampen_core::ir::style::Shadow;
    ///
    /// let shadow = Shadow::parse("2 2 4 #00000040").unwrap();
    /// assert_eq!(shadow.offset_x, 2.0);
    /// assert_eq!(shadow.offset_y, 2.0);
    /// assert_eq!(shadow.blur_radius, 4.0);
    /// ```
    pub fn parse(s: &str) -> Result<Self, String> {
        let parts: Vec<&str> = s.split_whitespace().collect();

        if parts.len() < 4 {
            return Err(format!(
                "Invalid shadow format: '{}'. Expected: offset_x offset_y blur color",
                s
            ));
        }

        let offset_x: f32 = parts[0]
            .parse()
            .map_err(|_| format!("Invalid offset_x: {}", parts[0]))?;
        let offset_y: f32 = parts[1]
            .parse()
            .map_err(|_| format!("Invalid offset_y: {}", parts[1]))?;
        let blur_radius: f32 = parts[2]
            .parse()
            .map_err(|_| format!("Invalid blur_radius: {}", parts[2]))?;

        // Color is everything after the first 3 parts
        let color_str = parts[3..].join(" ");
        let color = Color::parse(&color_str)?;

        Ok(Shadow {
            offset_x,
            offset_y,
            blur_radius,
            color,
        })
    }
}

/// Visual transformation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Transform {
    /// Uniform scale
    Scale(f32),
    /// Non-uniform scale
    ScaleXY { x: f32, y: f32 },
    /// Rotation in degrees
    Rotate(f32),
    /// Translation in pixels
    Translate { x: f32, y: f32 },
    /// Multiple composed transforms
    Multiple(Vec<Transform>),
}

impl Transform {
    /// Parse from string
    ///
    /// # Examples
    /// ```rust
    /// use dampen_core::ir::style::Transform;
    ///
    /// assert_eq!(Transform::parse("scale(1.2)"), Ok(Transform::Scale(1.2)));
    /// assert_eq!(Transform::parse("rotate(45)"), Ok(Transform::Rotate(45.0)));
    /// assert_eq!(Transform::parse("translate(10, 20)"), Ok(Transform::Translate { x: 10.0, y: 20.0 }));
    /// ```
    pub fn parse(s: &str) -> Result<Self, String> {
        let s = s.trim();

        // Scale
        if s.starts_with("scale(") && s.ends_with(')') {
            let inner = &s[6..s.len() - 1];
            let value: f32 = inner
                .parse()
                .map_err(|_| format!("Invalid scale value: {}", s))?;
            return Ok(Transform::Scale(value));
        }

        // ScaleXY
        if s.starts_with("scale(") && s.ends_with(')') {
            let inner = &s[6..s.len() - 1];
            let parts: Vec<&str> = inner.split(',').collect();
            if parts.len() == 2 {
                let x: f32 = parts[0]
                    .trim()
                    .parse()
                    .map_err(|_| format!("Invalid scale x: {}", parts[0]))?;
                let y: f32 = parts[1]
                    .trim()
                    .parse()
                    .map_err(|_| format!("Invalid scale y: {}", parts[1]))?;
                return Ok(Transform::ScaleXY { x, y });
            }
        }

        // Rotate
        if s.starts_with("rotate(") && s.ends_with(')') {
            let inner = &s[7..s.len() - 1];
            let value: f32 = inner
                .parse()
                .map_err(|_| format!("Invalid rotate value: {}", s))?;
            return Ok(Transform::Rotate(value));
        }

        // Translate
        if s.starts_with("translate(") && s.ends_with(')') {
            let inner = &s[10..s.len() - 1];
            let parts: Vec<&str> = inner.split(',').collect();
            if parts.len() == 2 {
                let x: f32 = parts[0]
                    .trim()
                    .parse()
                    .map_err(|_| format!("Invalid translate x: {}", parts[0]))?;
                let y: f32 = parts[1]
                    .trim()
                    .parse()
                    .map_err(|_| format!("Invalid translate y: {}", parts[1]))?;
                return Ok(Transform::Translate { x, y });
            }
        }

        Err(format!(
            "Invalid transform format: '{}'. Expected scale(n), rotate(n), or translate(x, y)",
            s
        ))
    }
}
