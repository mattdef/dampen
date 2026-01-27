/// Position of dropdown menu relative to the trigger widget.
///
/// Used by the `position` attribute on `<menu>` elements.
///
/// # Variants
///
/// * `Bottom` - Menu appears below the trigger (default)
/// * `Top` - Menu appears above the trigger
/// * `Left` - Menu appears to the left of the trigger
/// * `Right` - Menu appears to the right of the trigger
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
pub enum MenuPosition {
    /// Menu appears below the trigger
    #[default]
    Bottom,
    /// Menu appears above the trigger
    Top,
    /// Menu appears to the left of the trigger
    Left,
    /// Menu appears to the right of the trigger
    Right,
}

impl MenuPosition {
    /// Parse a string into a MenuPosition
    pub fn parse(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "bottom" => Some(Self::Bottom),
            "top" => Some(Self::Top),
            "left" => Some(Self::Left),
            "right" => Some(Self::Right),
            _ => None,
        }
    }
}
