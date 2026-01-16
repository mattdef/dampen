//! Column widget builder

use crate::HandlerMessage;
use crate::builder::DampenWidgetBuilder;
use crate::convert::map_layout_constraints;
use dampen_core::ir::node::WidgetNode;
use iced::{Element, Renderer, Theme};

impl<'a> DampenWidgetBuilder<'a> {
    pub(in crate::builder) fn build_column(
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

        let mut column = iced::widget::column(children);

        // Apply spacing and width/height from resolved layout
        if let Some(layout) = self.resolve_layout(node) {
            if let Some(spacing) = layout.spacing {
                column = column.spacing(spacing);
            }
            // Apply width and height directly to the column
            if layout.width.is_some() {
                let iced_layout = map_layout_constraints(&layout);
                column = column.width(iced_layout.width);
            }
            if layout.height.is_some() {
                let iced_layout = map_layout_constraints(&layout);
                column = column.height(iced_layout.height);
            }
        }

        self.apply_style_layout(column, node)
    }
}
