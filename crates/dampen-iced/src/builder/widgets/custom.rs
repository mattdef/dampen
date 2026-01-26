//! Custom widget builder

use crate::HandlerMessage;
use crate::builder::DampenWidgetBuilder;
use dampen_core::ir::node::WidgetNode;
use iced::{Element, Renderer, Theme};

impl<'a> DampenWidgetBuilder<'a> {
    pub(in crate::builder) fn build_custom(
        &self,
        _node: &WidgetNode,
    ) -> Element<'a, HandlerMessage, Theme, Renderer>
    where
        HandlerMessage: Clone + 'static,
    {
        iced::widget::column(vec![]).into()
    }
}
