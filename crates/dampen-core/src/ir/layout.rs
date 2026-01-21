//! Layout system types for Dampen UI framework
//!
//! This module defines the IR types for layout constraints, sizing, alignment,
//! and responsive breakpoints. All types are backend-agnostic and serializable.

use serde::{Deserialize, Serialize};

/// Layout constraints for widget sizing and positioning
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct LayoutConstraints {
    /// Primary sizing constraints
    pub width: Option<Length>,
    pub height: Option<Length>,

    /// Size constraints in pixels
    pub min_width: Option<f32>,
    pub max_width: Option<f32>,
    pub min_height: Option<f32>,
    pub max_height: Option<f32>,

    /// Inner spacing (padding)
    pub padding: Option<Padding>,

    /// Gap between child widgets
    pub spacing: Option<f32>,

    /// Alignment properties
    pub align_items: Option<Alignment>,
    pub justify_content: Option<Justification>,
    pub align_self: Option<Alignment>,

    /// Direct alignment (for Container, Text, etc.)
    pub align_x: Option<Alignment>,
    pub align_y: Option<Alignment>,

    /// Layout direction
    pub direction: Option<Direction>,

    /// Positioning
    pub position: Option<Position>,
    pub top: Option<f32>,
    pub right: Option<f32>,
    pub bottom: Option<f32>,
    pub left: Option<f32>,
    pub z_index: Option<i32>,
}

impl LayoutConstraints {
    /// Validates constraint relationships
    ///
    /// Returns an error if:
    /// - min_width > max_width
    /// - min_height > max_height
    /// - spacing is negative
    /// - padding values are negative
    /// - fill_portion is not 1-255
    /// - percentage is not 0.0-100.0
    pub fn validate(&self) -> Result<(), String> {
        if let (Some(min), Some(max)) = (self.min_width, self.max_width)
            && min > max
        {
            return Err(format!("min_width ({}) > max_width ({})", min, max));
        }

        if let (Some(min), Some(max)) = (self.min_height, self.max_height)
            && min > max
        {
            return Err(format!("min_height ({}) > max_height ({})", min, max));
        }

        if let Some(spacing) = self.spacing
            && spacing < 0.0
        {
            return Err(format!("spacing must be non-negative, got {}", spacing));
        }

        if let Some(padding) = &self.padding
            && (padding.top < 0.0
                || padding.right < 0.0
                || padding.bottom < 0.0
                || padding.left < 0.0)
        {
            return Err("padding values must be non-negative".to_string());
        }

        if let Some(Length::FillPortion(n)) = self.width
            && n == 0
        {
            return Err(format!("fill_portion must be 1-255, got {}", n));
        }

        if let Some(Length::FillPortion(n)) = self.height
            && n == 0
        {
            return Err(format!("fill_portion must be 1-255, got {}", n));
        }

        if let Some(Length::Percentage(p)) = self.width
            && !(0.0..=100.0).contains(&p)
        {
            return Err(format!("percentage must be 0.0-100.0, got {}", p));
        }

        if let Some(Length::Percentage(p)) = self.height
            && !(0.0..=100.0).contains(&p)
        {
            return Err(format!("percentage must be 0.0-100.0, got {}", p));
        }

        // Position-related validation
        if self.position.is_some() {
            // If position is set, at least one offset should be provided
            if self.top.is_none()
                && self.right.is_none()
                && self.bottom.is_none()
                && self.left.is_none()
            {
                return Err(
                    "position requires at least one offset (top, right, bottom, or left)"
                        .to_string(),
                );
            }
        }

        Ok(())
    }
}

/// Length specification for widget sizing
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Length {
    /// Exact pixel value
    Fixed(f32),
    /// Expand to fill available space
    Fill,
    /// Minimize to content size
    Shrink,
    /// Proportional fill (1-255)
    FillPortion(u8),
    /// Percentage of parent (0.0-100.0)
    Percentage(f32),
}

impl Length {
    /// Parse from string representation
    ///
    /// # Examples
    /// ```rust
    /// use dampen_core::ir::layout::Length;
    ///
    /// assert_eq!(Length::parse("200"), Ok(Length::Fixed(200.0)));
    /// assert_eq!(Length::parse("fill"), Ok(Length::Fill));
    /// assert_eq!(Length::parse("shrink"), Ok(Length::Shrink));
    /// assert_eq!(Length::parse("fill_portion(3)"), Ok(Length::FillPortion(3)));
    /// assert_eq!(Length::parse("50%"), Ok(Length::Percentage(50.0)));
    /// ```
    pub fn parse(s: &str) -> Result<Self, String> {
        let s = s.trim();

        if s.eq_ignore_ascii_case("fill") {
            return Ok(Length::Fill);
        }

        if s.eq_ignore_ascii_case("shrink") {
            return Ok(Length::Shrink);
        }

        // Parse fill_portion(n)
        if s.starts_with("fill_portion(") && s.ends_with(')') {
            let inner = &s[13..s.len() - 1];
            let n: u8 = inner
                .parse()
                .map_err(|_| format!("Invalid fill_portion: {}", s))?;
            return Ok(Length::FillPortion(n));
        }

        // Parse percentage
        if let Some(num) = s.strip_suffix('%') {
            let p: f32 = num
                .parse()
                .map_err(|_| format!("Invalid percentage: {}", s))?;
            return Ok(Length::Percentage(p));
        }

        // Parse fixed pixel value
        let pixels: f32 = s
            .parse()
            .map_err(|_| format!("Invalid length value: {}", s))?;
        Ok(Length::Fixed(pixels))
    }
}

/// Padding specification (top, right, bottom, left)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Padding {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

impl Padding {
    /// Parse padding from string
    ///
    /// # Formats
    /// - `"<all>"`: All sides (e.g., "10")
    /// - `"<v> <h>"`: Vertical and horizontal (e.g., "10 20")
    /// - `"<t> <r> <b> <l>"`: Individual sides (e.g., "10 20 30 40")
    ///
    /// # Examples
    /// ```rust
    /// use dampen_core::ir::layout::Padding;
    ///
    /// assert_eq!(Padding::parse("10"), Ok(Padding { top: 10.0, right: 10.0, bottom: 10.0, left: 10.0 }));
    /// assert_eq!(Padding::parse("10 20"), Ok(Padding { top: 10.0, right: 20.0, bottom: 10.0, left: 20.0 }));
    /// assert_eq!(Padding::parse("10 20 30 40"), Ok(Padding { top: 10.0, right: 20.0, bottom: 30.0, left: 40.0 }));
    /// ```
    pub fn parse(s: &str) -> Result<Self, String> {
        let parts: Vec<&str> = s.split_whitespace().collect();

        match parts.len() {
            1 => {
                let all: f32 = parts[0]
                    .parse()
                    .map_err(|_| format!("Invalid padding: {}", s))?;
                Ok(Padding {
                    top: all,
                    right: all,
                    bottom: all,
                    left: all,
                })
            }
            2 => {
                let v: f32 = parts[0]
                    .parse()
                    .map_err(|_| format!("Invalid vertical padding: {}", parts[0]))?;
                let h: f32 = parts[1]
                    .parse()
                    .map_err(|_| format!("Invalid horizontal padding: {}", parts[1]))?;
                Ok(Padding {
                    top: v,
                    right: h,
                    bottom: v,
                    left: h,
                })
            }
            4 => {
                let t: f32 = parts[0]
                    .parse()
                    .map_err(|_| format!("Invalid top padding: {}", parts[0]))?;
                let r: f32 = parts[1]
                    .parse()
                    .map_err(|_| format!("Invalid right padding: {}", parts[1]))?;
                let b: f32 = parts[2]
                    .parse()
                    .map_err(|_| format!("Invalid bottom padding: {}", parts[2]))?;
                let l: f32 = parts[3]
                    .parse()
                    .map_err(|_| format!("Invalid left padding: {}", parts[3]))?;
                Ok(Padding {
                    top: t,
                    right: r,
                    bottom: b,
                    left: l,
                })
            }
            _ => Err(format!(
                "Invalid padding format: '{}'. Expected 1, 2, or 4 values",
                s
            )),
        }
    }
}

/// Widget alignment on cross-axis
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Alignment {
    /// Top for column, left for row
    Start,
    /// Centered
    Center,
    /// Bottom for column, right for row
    End,
    /// Fill cross-axis
    Stretch,
}

impl Alignment {
    /// Parse from string
    pub fn parse(s: &str) -> Result<Self, String> {
        match s.trim().to_lowercase().as_str() {
            "start" => Ok(Alignment::Start),
            "center" => Ok(Alignment::Center),
            "end" => Ok(Alignment::End),
            "stretch" => Ok(Alignment::Stretch),
            _ => Err(format!(
                "Invalid alignment: '{}'. Expected start, center, end, or stretch",
                s
            )),
        }
    }
}

/// Widget justification on main-axis
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Justification {
    /// Pack at start
    Start,
    /// Pack at center
    Center,
    /// Pack at end
    End,
    /// First at start, last at end, evenly spaced
    SpaceBetween,
    /// Equal space around each item
    SpaceAround,
    /// Equal space between items
    SpaceEvenly,
}

impl Justification {
    /// Parse from string
    pub fn parse(s: &str) -> Result<Self, String> {
        match s.trim().to_lowercase().as_str() {
            "start" => Ok(Justification::Start),
            "center" => Ok(Justification::Center),
            "end" => Ok(Justification::End),
            "space_between" => Ok(Justification::SpaceBetween),
            "space_around" => Ok(Justification::SpaceAround),
            "space_evenly" => Ok(Justification::SpaceEvenly),
            _ => Err(format!(
                "Invalid justification: '{}'. Expected start, center, end, space_between, space_around, or space_evenly",
                s
            )),
        }
    }
}

/// Layout direction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Direction {
    Horizontal,
    HorizontalReverse,
    Vertical,
    VerticalReverse,
}

impl Direction {
    /// Parse from string
    pub fn parse(s: &str) -> Result<Self, String> {
        match s.trim().to_lowercase().as_str() {
            "horizontal" => Ok(Direction::Horizontal),
            "horizontal_reverse" => Ok(Direction::HorizontalReverse),
            "vertical" => Ok(Direction::Vertical),
            "vertical_reverse" => Ok(Direction::VerticalReverse),
            _ => Err(format!(
                "Invalid direction: '{}'. Expected horizontal, horizontal_reverse, vertical, or vertical_reverse",
                s
            )),
        }
    }
}

/// Position type for widget positioning
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Position {
    /// Relative to normal flow (default)
    Relative,
    /// Absolute positioning relative to nearest positioned ancestor
    Absolute,
}

impl Position {
    /// Parse from string
    pub fn parse(s: &str) -> Result<Self, String> {
        match s.trim().to_lowercase().as_str() {
            "relative" => Ok(Position::Relative),
            "absolute" => Ok(Position::Absolute),
            _ => Err(format!(
                "Invalid position: '{}'. Expected relative or absolute",
                s
            )),
        }
    }
}

/// Responsive breakpoint
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Breakpoint {
    /// < 640px
    Mobile,
    /// 640px - 1024px
    Tablet,
    /// >= 1024px
    Desktop,
}

impl Breakpoint {
    /// Determine breakpoint from viewport width
    pub fn from_viewport_width(width: f32) -> Self {
        match width {
            w if w < 640.0 => Breakpoint::Mobile,
            w if w < 1024.0 => Breakpoint::Tablet,
            _ => Breakpoint::Desktop,
        }
    }

    /// Parse from string
    pub fn parse(s: &str) -> Result<Self, String> {
        match s.trim().to_lowercase().as_str() {
            "mobile" => Ok(Breakpoint::Mobile),
            "tablet" => Ok(Breakpoint::Tablet),
            "desktop" => Ok(Breakpoint::Desktop),
            _ => Err(format!(
                "Invalid breakpoint: '{}'. Expected mobile, tablet, or desktop",
                s
            )),
        }
    }
}
