//! Style cascading and resolution
//!
//! This module implements the style cascade algorithm that merges styles
//! from multiple sources with proper precedence.

use gravity_core::ir::style::StyleProperties;
use gravity_core::ir::theme::{StyleClass, WidgetState};
use gravity_core::ir::GravityDocument;
use std::collections::HashMap;

/// Resolves final style properties using cascade rules
///
/// Precedence (highest to lowest):
/// 1. Inline styles on widget
/// 2. State-based styles (hover, focus, active, disabled)
/// 3. Style classes (in order, with inheritance)
/// 4. Theme defaults
/// 5. Widget defaults
#[derive(Debug, Clone)]
pub struct StyleCascade {
    /// All available style classes
    classes: HashMap<String, StyleClass>,
    /// Current widget state
    current_state: Option<WidgetState>,
}

impl StyleCascade {
    /// Create a new style cascade resolver
    pub fn new(doc: &GravityDocument) -> Self {
        Self {
            classes: doc.style_classes.clone(),
            current_state: None,
        }
    }

    /// Set the current widget state
    pub fn with_state(mut self, state: WidgetState) -> Self {
        self.current_state = Some(state);
        self
    }

    /// Resolve final style properties for a widget
    ///
    /// # Arguments
    /// * `inline_style` - Direct style properties on the widget
    /// * `class_names` - List of class names to apply
    /// * `theme_style` - Theme default style for this widget type
    ///
    /// # Returns
    /// Merged StyleProperties with all cascades applied
    pub fn resolve(
        &self,
        inline_style: Option<&StyleProperties>,
        class_names: &[String],
        theme_style: Option<&StyleProperties>,
    ) -> StyleProperties {
        let mut result = StyleProperties::default();

        // 1. Start with theme defaults
        if let Some(theme) = theme_style {
            result = merge_styles(&result, theme);
        }

        // 2. Apply class styles (in order, with inheritance)
        for class_name in class_names {
            if let Some(class) = self.classes.get(class_name) {
                let class_style = self.resolve_class_style(class);
                result = merge_styles(&result, &class_style);
            }
        }

        // 3. Apply state-based styles if applicable
        if let Some(state) = self.current_state {
            for class_name in class_names {
                if let Some(class) = self.classes.get(class_name) {
                    if let Some(state_style) = class.state_variants.get(&state) {
                        result = merge_styles(&result, state_style);
                    }
                }
            }
        }

        // 4. Apply inline styles (highest priority)
        if let Some(inline) = inline_style {
            result = merge_styles(&result, inline);
        }

        result
    }

    /// Resolve a single class's style, including inherited styles
    fn resolve_class_style(&self, class: &StyleClass) -> StyleProperties {
        let mut result = StyleProperties::default();

        // Apply inherited classes first
        for parent_name in &class.extends {
            if let Some(parent) = self.classes.get(parent_name) {
                let parent_style = self.resolve_class_style(parent);
                result = merge_styles(&result, &parent_style);
            }
        }

        // Then apply this class's own style
        result = merge_styles(&result, &class.style);

        result
    }
}

/// Merge two style properties (second overrides first)
pub fn merge_styles(base: &StyleProperties, override_style: &StyleProperties) -> StyleProperties {
    StyleProperties {
        background: override_style
            .background
            .clone()
            .or_else(|| base.background.clone()),
        color: override_style.color.or(base.color),
        border: override_style
            .border
            .clone()
            .or_else(|| base.border.clone()),
        shadow: override_style.shadow.or(base.shadow),
        opacity: override_style.opacity.or(base.opacity),
        transform: override_style
            .transform
            .clone()
            .or_else(|| base.transform.clone()),
    }
}

/// Resolve layout constraints with cascade
pub fn resolve_layout(
    _base: Option<&LayoutConstraints>,
    class_layouts: &[Option<LayoutConstraints>],
    inline: Option<&LayoutConstraints>,
) -> Option<LayoutConstraints> {
    // Start with base
    let mut result = LayoutConstraints::default();
    let mut has_any = false;

    // Apply class layouts in order
    for layout in class_layouts.iter().flatten() {
        has_any = true;
        merge_layout(&mut result, layout);
    }

    // Apply inline layout
    if let Some(layout) = inline {
        has_any = true;
        merge_layout(&mut result, layout);
    }

    if has_any {
        Some(result)
    } else {
        None
    }
}

/// Merge layout constraints (override non-None values)
fn merge_layout(base: &mut LayoutConstraints, override_layout: &LayoutConstraints) {
    if override_layout.width.is_some() {
        base.width = override_layout.width.clone();
    }
    if override_layout.height.is_some() {
        base.height = override_layout.height.clone();
    }
    if override_layout.min_width.is_some() {
        base.min_width = override_layout.min_width;
    }
    if override_layout.max_width.is_some() {
        base.max_width = override_layout.max_width;
    }
    if override_layout.min_height.is_some() {
        base.min_height = override_layout.min_height;
    }
    if override_layout.max_height.is_some() {
        base.max_height = override_layout.max_height;
    }
    if override_layout.padding.is_some() {
        base.padding = override_layout.padding.clone();
    }
    if override_layout.spacing.is_some() {
        base.spacing = override_layout.spacing;
    }
    if override_layout.align_items.is_some() {
        base.align_items = override_layout.align_items;
    }
    if override_layout.justify_content.is_some() {
        base.justify_content = override_layout.justify_content;
    }
    if override_layout.align_self.is_some() {
        base.align_self = override_layout.align_self;
    }
    if override_layout.direction.is_some() {
        base.direction = override_layout.direction;
    }
}

use gravity_core::ir::layout::LayoutConstraints;
