//! Row widget builder

use crate::HandlerMessage;
use crate::builder::DampenWidgetBuilder;
use crate::convert::map_layout_constraints;
use dampen_core::ir::node::WidgetNode;
use iced::{Element, Renderer, Theme};

impl<'a> DampenWidgetBuilder<'a> {
    pub(in crate::builder) fn build_row(
        &self,
        node: &WidgetNode,
    ) -> Element<'a, HandlerMessage, Theme, Renderer>
    where
        HandlerMessage: Clone + 'static,
    {
        let children: Vec<_> = node
            .children
            .iter()
            .map(|child| self.build_widget(child))
            .collect();

        let mut row = iced::widget::row(children);

        // Apply spacing and width/height from resolved layout
        if let Some(layout) = self.resolve_layout(node) {
            if let Some(spacing) = layout.spacing {
                row = row.spacing(spacing);
            }
            // Apply width and height directly to the row
            if layout.width.is_some() {
                let iced_layout = map_layout_constraints(&layout);
                row = row.width(iced_layout.width);
            }
            if layout.height.is_some() {
                let iced_layout = map_layout_constraints(&layout);
                row = row.height(iced_layout.height);
            }
        }

        self.apply_style_layout(row, node)
    }
}
