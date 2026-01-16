//! Float widget builder

use crate::HandlerMessage;
use crate::builder::DampenWidgetBuilder;
use dampen_core::ir::node::WidgetNode;
use iced::{Element, Renderer, Theme};

impl<'a> DampenWidgetBuilder<'a> {
    pub(in crate::builder) fn build_float(
        &self,
        node: &WidgetNode,
    ) -> Element<'a, HandlerMessage, Theme, Renderer>
    where
        HandlerMessage: Clone + 'static,
    {
        if self.verbose {
            eprintln!("[DampenWidgetBuilder] Building float widget (placeholder)");
        }

        // For now, render children in a column as a placeholder
        // In a full implementation, this would use absolute positioning
        let children: Vec<_> = node
            .children
            .iter()
            .map(|child| self.build_widget(child))
            .collect();

        iced::widget::column(children).into()
    }
}
