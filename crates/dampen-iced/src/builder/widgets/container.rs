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
                let column = iced::widget::column(children);
                self.apply_style_layout(column, node)
            }
        }
    }
}
