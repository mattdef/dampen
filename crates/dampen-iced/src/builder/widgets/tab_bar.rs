//! TabBar widget builder

use crate::HandlerMessage;
use crate::builder::DampenWidgetBuilder;
use dampen_core::ir::node::WidgetNode;
use iced::{Element, Padding, Renderer, Theme};

/// Maps icon names to Unicode characters
fn resolve_icon(name: &str) -> char {
    match name {
        "home" => '\u{F015}',
        "settings" => '\u{F013}',
        "user" => '\u{F007}',
        "search" => '\u{F002}',
        "add" => '\u{F067}',
        "delete" => '\u{F1F8}',
        "edit" => '\u{F044}',
        "save" => '\u{F0C7}',
        "close" => '\u{F00D}',
        "back" => '\u{F060}',
        "forward" => '\u{F061}',
        _ => '\u{F111}', // Circle as fallback for unknown icons
    }
}

/// Parse padding value from string
/// Supports: "10" (all sides) or "10 20 10 20" (top right bottom left)
fn parse_padding(value: &str) -> Padding {
    let parts: Vec<f32> = value
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    match parts.len() {
        1 => Padding::new(parts[0]),
        2 => Padding::from([parts[0], parts[1]]),
        4 => Padding {
            top: parts[0],
            right: parts[1],
            bottom: parts[2],
            left: parts[3],
        },
        _ => Padding::new(0.0),
    }
}

/// Parse length value from string
/// Supports: "fill", "shrink", or numeric value (pixels)
fn parse_length(value: &str) -> iced::Length {
    match value.trim() {
        "fill" => iced::Length::Fill,
        "shrink" => iced::Length::Shrink,
        _ => {
            if let Ok(pixels) = value.parse::<f32>() {
                iced::Length::Fixed(pixels)
            } else {
                iced::Length::Shrink
            }
        }
    }
}

impl<'a> DampenWidgetBuilder<'a> {
    /// Build a TabBar widget from a WidgetNode
    pub(in crate::builder) fn build_tab_bar(
        &self,
        node: &WidgetNode,
    ) -> Element<'a, HandlerMessage, Theme, Renderer>
    where
        HandlerMessage: Clone + 'static,
    {
        #[cfg(debug_assertions)]
        eprintln!("[DampenWidgetBuilder] Building TabBar");

        // Get selected index (default to 0)
        let selected_index = node
            .attributes
            .get("selected")
            .and_then(|attr| {
                let value = self.evaluate_attribute(attr);
                value.parse::<usize>().ok()
            })
            .unwrap_or(0);

        // T038: Clamp selected index to valid range [0, num_tabs)
        let num_tabs = node.children.len();
        let selected_index = if num_tabs == 0 {
            0 // Empty TabBar, keep at 0
        } else if selected_index >= num_tabs {
            // Index out of bounds, clamp to last valid index
            num_tabs - 1
        } else {
            selected_index
        };

        #[cfg(debug_assertions)]
        eprintln!(
            "[DampenWidgetBuilder] TabBar selected index: {}",
            selected_index
        );

        // Find on_select event handler
        let on_select_event = node
            .events
            .iter()
            .find(|e| e.event == dampen_core::EventKind::Select);

        // Build tab labels
        let tab_labels: Vec<iced_aw::tab_bar::TabLabel> = node
            .children
            .iter()
            .map(|child| self.build_tab_label(child))
            .collect();

        // Build tab contents for each tab
        let mut tab_contents: Vec<Vec<Element<'a, HandlerMessage, Theme, Renderer>>> = node
            .children
            .iter()
            .map(|child| {
                child
                    .children
                    .iter()
                    .map(|child_node| self.build_widget(child_node))
                    .collect()
            })
            .collect();

        // Create TabBar with on_select callback
        // The iced_aw API requires the callback in the constructor
        let mut tab_bar = if let Some(event_binding) = on_select_event {
            if self.handler_registry.is_some() {
                let handler_name = event_binding.handler.clone();

                #[cfg(debug_assertions)]
                eprintln!(
                    "[DampenWidgetBuilder] TabBar: Attaching on_select with handler '{}'",
                    handler_name
                );

                iced_aw::TabBar::new(move |idx: usize| {
                    HandlerMessage::Handler(handler_name.clone(), Some(idx.to_string()))
                })
            } else {
                iced_aw::TabBar::new(|_idx: usize| {
                    HandlerMessage::Handler("noop".to_string(), None)
                })
            }
        } else {
            iced_aw::TabBar::new(|_idx: usize| HandlerMessage::Handler("noop".to_string(), None))
        };

        // Set active tab
        tab_bar = tab_bar.set_active_tab(&selected_index);

        // Apply icon_size if specified
        if let Some(icon_size_attr) = node.attributes.get("icon_size") {
            let icon_size_value = self.evaluate_attribute(icon_size_attr);
            if let Ok(icon_size) = icon_size_value.parse::<f32>() {
                tab_bar = tab_bar.icon_size(icon_size);
            }
        }

        // Apply text_size if specified
        if let Some(text_size_attr) = node.attributes.get("text_size") {
            let text_size_value = self.evaluate_attribute(text_size_attr);
            if let Ok(text_size) = text_size_value.parse::<f32>() {
                tab_bar = tab_bar.text_size(text_size);
            }
        }

        // T045: Apply spacing if specified
        if let Some(spacing_attr) = node.attributes.get("spacing") {
            let spacing_value = self.evaluate_attribute(spacing_attr);
            if let Ok(spacing) = spacing_value.parse::<f32>() {
                tab_bar = tab_bar.spacing(iced::Pixels(spacing));
            }
        }

        // T045: Apply padding if specified
        if let Some(padding_attr) = node.attributes.get("padding") {
            let padding_value = self.evaluate_attribute(padding_attr);
            // Parse padding - can be a single value or four values (top right bottom left)
            let padding = parse_padding(&padding_value);
            tab_bar = tab_bar.padding(padding);
        }

        // T045: Apply width if specified
        if let Some(width_attr) = node.attributes.get("width") {
            let width_value = self.evaluate_attribute(width_attr);
            let length = parse_length(&width_value);
            tab_bar = tab_bar.width(length);
        }

        // T045: Apply height if specified
        if let Some(height_attr) = node.attributes.get("height") {
            let height_value = self.evaluate_attribute(height_attr);
            let length = parse_length(&height_value);
            tab_bar = tab_bar.height(length);
        }

        // Add tabs
        for (idx, label) in tab_labels.into_iter().enumerate() {
            tab_bar = tab_bar.push(idx, label);
        }

        // Apply custom style to highlight selected tab
        tab_bar = tab_bar.style(move |theme: &iced::Theme, status| {
            let base_style = iced_aw::style::tab_bar::Style::default();

            match status {
                iced_aw::style::Status::Selected => iced_aw::style::tab_bar::Style {
                    tab_label_background: iced::Background::Color(theme.palette().primary),
                    tab_label_border_color: theme.palette().primary,
                    text_color: iced::Color::WHITE,
                    icon_color: iced::Color::WHITE,
                    ..base_style
                },
                _ => base_style,
            }
        });

        // Build content column for the selected tab
        // We need to move the content widgets out of tab_contents since Element doesn't implement Clone
        let content_element: Element<'a, HandlerMessage, Theme, Renderer> =
            if selected_index < tab_contents.len() {
                let content_widgets = std::mem::take(&mut tab_contents[selected_index]);
                match content_widgets.len() {
                    0 => iced::widget::column![].into(),
                    1 => content_widgets
                        .into_iter()
                        .next()
                        .unwrap_or_else(|| iced::widget::column![].into()),
                    _ => iced::widget::Column::with_children(content_widgets).into(),
                }
            } else {
                iced::widget::column![].into()
            };

        // Combine TabBar and content in a column
        let result = iced::widget::column![tab_bar, content_element];

        result.into()
    }

    /// Build a TabLabel from a Tab widget node
    fn build_tab_label(&self, node: &WidgetNode) -> iced_aw::tab_bar::TabLabel {
        // Get label text
        let label_text = node
            .attributes
            .get("label")
            .map(|attr| self.evaluate_attribute(attr));

        // Get icon if specified
        let icon_char = node.attributes.get("icon").map(|attr| {
            let icon_name = self.evaluate_attribute(attr);
            resolve_icon(&icon_name)
        });

        // Build TabLabel based on what we have
        match (icon_char, label_text) {
            (Some(icon), Some(label)) => {
                // Both icon and label
                iced_aw::tab_bar::TabLabel::IconText(icon, label)
            }
            (Some(icon), None) => {
                // Icon only
                iced_aw::tab_bar::TabLabel::Icon(icon)
            }
            (None, Some(label)) => {
                // Text only
                iced_aw::tab_bar::TabLabel::Text(label)
            }
            (None, None) => {
                // Default fallback
                iced_aw::tab_bar::TabLabel::Text("Tab".to_string())
            }
        }
    }
}
