//! validation for attribute checking.
//!
//! # Examples
//!
//! ```
//! use dampen_core::ir::WidgetKind;
//! use dampen_core::schema::get_widget_schema;
//!
//! let schema = get_widget_schema(&WidgetKind::Button);
//! assert!(schema.events.contains(&"on_click"));
//! ```

use crate::ir::WidgetKind;
use std::collections::HashSet;

/// Represents the validation contract for a single widget type.
///
/// Contains lists of valid attributes categorized by type (required, optional, events, etc.).
///
/// # Examples
///
/// ```
/// use dampen_core::schema::WidgetSchema;
///
/// let schema = WidgetSchema {
///     required: &["value"],
///     optional: &[],
///     events: &[],
///     style_attributes: &[],
///     layout_attributes: &[],
/// };
///
/// assert!(schema.all_valid().contains("value"));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WidgetSchema {
    /// Attributes that MUST be present on the widget.
    pub required: &'static [&'static str],
    /// Attributes that MAY be present on the widget.
    pub optional: &'static [&'static str],
    /// Event handler attributes (e.g., "on_click").
    pub events: &'static [&'static str],
    /// Styling attributes (e.g., "background", "color").
    pub style_attributes: &'static [&'static str],
    /// Layout attributes (e.g., "width", "padding").
    pub layout_attributes: &'static [&'static str],
}

/// Common styling attributes shared by most widgets.
pub const COMMON_STYLE_ATTRIBUTES: &[&str] = &[
    "background",
    "color",
    "border_color",
    "border_width",
    "border_radius",
    "border_style",
    "shadow",
    "opacity",
    "transform",
    "style",
    "text_color",
    "shadow_color",
    "shadow_offset",
    "shadow_blur_radius",
];

/// Common layout attributes shared by most widgets.
pub const COMMON_LAYOUT_ATTRIBUTES: &[&str] = &[
    "width",
    "height",
    "min_width",
    "max_width",
    "min_height",
    "max_height",
    "padding",
    "spacing",
    "align_items",
    "justify_content",
    "align",
    "align_x",
    "align_y",
    "align_self",
    "direction",
    "position",
    "top",
    "right",
    "bottom",
    "left",
    "z_index",
    "class",
    "theme",
    "theme_ref",
];

/// Common event attributes shared by most interactive widgets.
pub const COMMON_EVENTS: &[&str] = &[
    "on_click",
    "on_press",
    "on_release",
    "on_change",
    "on_input",
    "on_submit",
    "on_select",
    "on_toggle",
    "on_scroll",
];

impl WidgetSchema {
    /// Returns a `HashSet` containing all valid attributes for this schema.
    ///
    /// This combines required, optional, events, style, and layout attributes.
    pub fn all_valid(&self) -> HashSet<&'static str> {
        let mut set = HashSet::new();
        set.extend(self.required.iter().cloned());
        set.extend(self.optional.iter().cloned());
        set.extend(self.events.iter().cloned());
        set.extend(self.style_attributes.iter().cloned());
        set.extend(self.layout_attributes.iter().cloned());
        set
    }

    /// Returns a `Vec` containing all valid attribute names.
    pub fn all_valid_names(&self) -> Vec<&'static str> {
        let mut names = Vec::new();
        names.extend_from_slice(self.required);
        names.extend_from_slice(self.optional);
        names.extend_from_slice(self.events);
        names.extend_from_slice(self.style_attributes);
        names.extend_from_slice(self.layout_attributes);
        names
    }
}

/// Returns the validation schema for a given widget kind.
///
/// # Examples
///
/// ```
/// use dampen_core::ir::WidgetKind;
/// use dampen_core::schema::get_widget_schema;
///
/// let schema = get_widget_schema(&WidgetKind::Text);
/// assert!(schema.required.contains(&"value"));
/// ```
pub fn get_widget_schema(kind: &WidgetKind) -> WidgetSchema {
    match kind {
        WidgetKind::Text => WidgetSchema {
            required: &["value"],
            optional: &["size", "weight", "color"],
            events: COMMON_EVENTS,
            style_attributes: COMMON_STYLE_ATTRIBUTES,
            layout_attributes: COMMON_LAYOUT_ATTRIBUTES,
        },
        WidgetKind::Image => WidgetSchema {
            required: &["src"],
            optional: &["width", "height", "fit", "filter_method", "path"],
            events: COMMON_EVENTS,
            style_attributes: COMMON_STYLE_ATTRIBUTES,
            layout_attributes: COMMON_LAYOUT_ATTRIBUTES,
        },
        WidgetKind::Button => WidgetSchema {
            required: &[],
            optional: &["label", "enabled"],
            events: &["on_click", "on_press", "on_release"],
            style_attributes: COMMON_STYLE_ATTRIBUTES,
            layout_attributes: COMMON_LAYOUT_ATTRIBUTES,
        },
        WidgetKind::TextInput => WidgetSchema {
            required: &[],
            optional: &["placeholder", "value", "password", "icon", "size"],
            events: &["on_input", "on_submit", "on_change", "on_paste"],
            style_attributes: COMMON_STYLE_ATTRIBUTES,
            layout_attributes: COMMON_LAYOUT_ATTRIBUTES,
        },
        WidgetKind::Checkbox => WidgetSchema {
            required: &[],
            optional: &["checked", "label", "icon", "size"],
            events: &["on_toggle"],
            style_attributes: COMMON_STYLE_ATTRIBUTES,
            layout_attributes: COMMON_LAYOUT_ATTRIBUTES,
        },
        WidgetKind::Radio => WidgetSchema {
            required: &["label", "value"],
            optional: &[
                "selected",
                "disabled",
                "size",
                "text_size",
                "text_line_height",
                "text_shaping",
            ],
            events: &["on_select"],
            style_attributes: COMMON_STYLE_ATTRIBUTES,
            layout_attributes: COMMON_LAYOUT_ATTRIBUTES,
        },
        WidgetKind::Slider => WidgetSchema {
            required: &[],
            optional: &["min", "max", "value", "step"],
            events: &["on_change", "on_release"],
            style_attributes: COMMON_STYLE_ATTRIBUTES,
            layout_attributes: COMMON_LAYOUT_ATTRIBUTES,
        },
        WidgetKind::Column | WidgetKind::Row | WidgetKind::Container => WidgetSchema {
            required: &[],
            optional: &[],
            events: COMMON_EVENTS,
            style_attributes: COMMON_STYLE_ATTRIBUTES,
            layout_attributes: COMMON_LAYOUT_ATTRIBUTES,
        },
        WidgetKind::Scrollable => WidgetSchema {
            required: &[],
            optional: &[],
            events: &["on_scroll"],
            style_attributes: COMMON_STYLE_ATTRIBUTES,
            layout_attributes: COMMON_LAYOUT_ATTRIBUTES,
        },
        WidgetKind::Stack => WidgetSchema {
            required: &[],
            optional: &[],
            events: COMMON_EVENTS,
            style_attributes: COMMON_STYLE_ATTRIBUTES,
            layout_attributes: COMMON_LAYOUT_ATTRIBUTES,
        },
        WidgetKind::Svg => WidgetSchema {
            required: &["src"],
            optional: &["width", "height", "path"],
            events: COMMON_EVENTS,
            style_attributes: COMMON_STYLE_ATTRIBUTES,
            layout_attributes: COMMON_LAYOUT_ATTRIBUTES,
        },
        WidgetKind::PickList => WidgetSchema {
            required: &[],
            optional: &["placeholder", "selected", "options"],
            events: &["on_select"],
            style_attributes: COMMON_STYLE_ATTRIBUTES,
            layout_attributes: COMMON_LAYOUT_ATTRIBUTES,
        },
        WidgetKind::Toggler => WidgetSchema {
            required: &[],
            optional: &["checked", "active", "label"],
            events: &["on_toggle"],
            style_attributes: COMMON_STYLE_ATTRIBUTES,
            layout_attributes: COMMON_LAYOUT_ATTRIBUTES,
        },
        WidgetKind::Space | WidgetKind::Rule => WidgetSchema {
            required: &[],
            optional: &[],
            events: &[],
            style_attributes: COMMON_STYLE_ATTRIBUTES,
            layout_attributes: COMMON_LAYOUT_ATTRIBUTES,
        },
        WidgetKind::ComboBox => WidgetSchema {
            required: &[],
            optional: &["placeholder", "value", "options"],
            events: &["on_input", "on_select"],
            style_attributes: COMMON_STYLE_ATTRIBUTES,
            layout_attributes: COMMON_LAYOUT_ATTRIBUTES,
        },
        WidgetKind::ProgressBar => WidgetSchema {
            required: &[],
            optional: &["value", "min", "max", "style"],
            events: &[],
            style_attributes: COMMON_STYLE_ATTRIBUTES,
            layout_attributes: COMMON_LAYOUT_ATTRIBUTES,
        },
        WidgetKind::Tooltip => WidgetSchema {
            required: &[],
            optional: &["message", "position", "delay"],
            events: COMMON_EVENTS,
            style_attributes: COMMON_STYLE_ATTRIBUTES,
            // Tooltip is a special case that typically wraps another widget but doesn't have layout itself in the same way?
            // Data model says "no layout attributes".
            layout_attributes: &[],
        },
        WidgetKind::Grid => WidgetSchema {
            required: &[],
            optional: &["columns"],
            events: COMMON_EVENTS,
            style_attributes: COMMON_STYLE_ATTRIBUTES,
            layout_attributes: COMMON_LAYOUT_ATTRIBUTES,
        },
        WidgetKind::Canvas => WidgetSchema {
            required: &[],
            optional: &["program"],
            events: &["on_draw"],
            style_attributes: COMMON_STYLE_ATTRIBUTES,
            layout_attributes: COMMON_LAYOUT_ATTRIBUTES,
        },
        WidgetKind::Float => WidgetSchema {
            required: &[],
            optional: &[],
            events: COMMON_EVENTS,
            style_attributes: COMMON_STYLE_ATTRIBUTES,
            layout_attributes: COMMON_LAYOUT_ATTRIBUTES,
        },
        WidgetKind::For => WidgetSchema {
            required: &["each", "in"],
            optional: &["template"],
            events: &[],
            style_attributes: COMMON_STYLE_ATTRIBUTES,
            layout_attributes: COMMON_LAYOUT_ATTRIBUTES,
        },
        WidgetKind::If => WidgetSchema {
            required: &["condition"],
            optional: &[],
            events: &[],
            style_attributes: COMMON_STYLE_ATTRIBUTES,
            layout_attributes: COMMON_LAYOUT_ATTRIBUTES,
        },
        WidgetKind::Custom(_) => WidgetSchema {
            required: &[],
            optional: &[],
            events: &[],
            style_attributes: &[],
            layout_attributes: &[],
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::WidgetKind;

    #[test]
    fn test_button_schema_contains_expected_attributes() {
        let schema = WidgetKind::Button.schema();
        let valid = schema.all_valid();

        assert!(valid.contains("on_click"));
        assert!(valid.contains("label"));
        // Should also contain common attributes
        assert!(valid.contains("width"));
        assert!(valid.contains("background"));
    }

    #[test]
    fn test_container_schema_includes_layout_attributes() {
        let schema = WidgetKind::Container.schema();
        let valid = schema.all_valid();

        assert!(valid.contains("padding"));
        assert!(valid.contains("align_x"));
        assert!(valid.contains("width"));
    }

    #[test]
    fn test_textinput_schema_includes_size() {
        let schema = WidgetKind::TextInput.schema();
        let valid = schema.all_valid();

        assert!(valid.contains("size"));
        assert!(valid.contains("placeholder"));
    }

    #[test]
    fn test_custom_widget_returns_permissive_schema() {
        let schema = WidgetKind::Custom("MyWidget".to_string()).schema();
        assert!(schema.required.is_empty());
    }

    #[test]
    fn test_all_widget_kinds_have_schema() {
        let kinds = [
            WidgetKind::Column,
            WidgetKind::Row,
            WidgetKind::Text,
            WidgetKind::Button,
            WidgetKind::Image,
        ];

        for kind in kinds {
            let _ = kind.schema();
        }
    }
}
