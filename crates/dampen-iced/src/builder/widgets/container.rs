//! Container widget builder

use crate::HandlerMessage;
use crate::builder::DampenWidgetBuilder;
use dampen_core::ir::node::WidgetNode;
use iced::{Element, Renderer, Theme};

impl<'a> DampenWidgetBuilder<'a> {
    pub(in crate::builder) fn build_container(
        &self,
        node: &WidgetNode,
    ) -> Element<'a, HandlerMessage, Theme, Renderer>
    where
        HandlerMessage: Clone + 'static,
    {
        // Container can have multiple children - wrap them in a column if needed
        match node.children.len() {
            0 => {
                // Empty container - use empty space
                self.apply_style_layout(iced::widget::Space::new(), node)
            }
            1 => {
                // Single child - use it directly
                let child = self.build_widget(&node.children[0]);
                self.apply_style_layout(child, node)
            }
            _ => {
                // Multiple children - wrap in a column
                let children: Vec<_> = node
                    .children
                    .iter()
                    .map(|child| self.build_widget(child))
                    .collect();
                let mut column = iced::widget::column(children);

                // Apply layout properties to the internal column
                if let Some(layout) = self.resolve_layout(node) {
                    if let Some(spacing) = layout.spacing {
                        column = column.spacing(spacing);
                    }
                    // Column supports align_x for horizontal alignment of children
                    if let Some(align_x) = layout.align_x {
                        use crate::style_mapping::map_alignment;
                        column = column.align_x(map_alignment(align_x));
                    }
                    // For align_items, we use it as horizontal alignment (align_x)
                    // since column arranges children vertically
                    if let Some(align_items) = layout.align_items {
                        use crate::style_mapping::map_alignment;
                        column = column.align_x(map_alignment(align_items));
                    }
                }

                self.apply_style_layout(column, node)
            }
        }
    }
}
