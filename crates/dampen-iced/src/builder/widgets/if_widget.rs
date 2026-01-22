//! If widget builder

use crate::HandlerMessage;
use crate::builder::DampenWidgetBuilder;
use dampen_core::ir::node::WidgetNode;
use iced::{Element, Renderer, Theme};

impl<'a> DampenWidgetBuilder<'a> {
    /// Build an <if> widget
    ///
    /// Renders its children only if the condition evaluates to true.
    pub(in crate::builder) fn build_if(
        &self,
        node: &WidgetNode,
    ) -> Element<'a, HandlerMessage, Theme, Renderer>
    where
        HandlerMessage: Clone + 'static,
    {
        use dampen_core::ir::node::AttributeValue;

        // Evaluate condition
        let condition = match node.attributes.get("condition") {
            Some(AttributeValue::Binding(expr)) => {
                // Try context first, then model
                if let Some(ctx_value) = self.resolve_from_context(expr) {
                    ctx_value.to_bool()
                } else {
                    use dampen_core::expr::evaluate_binding_expr_with_shared;
                    evaluate_binding_expr_with_shared(expr, self.model, self.shared_context)
                        .map(|v| v.to_bool())
                        .unwrap_or(false)
                }
            }
            Some(AttributeValue::Static(s)) => s == "true" || s == "1",
            _ => {
                #[cfg(debug_assertions)]
                eprintln!("[DampenWidgetBuilder] If widget missing 'condition' attribute");
                false
            }
        };

        if condition {
            let children: Vec<_> = node
                .children
                .iter()
                .map(|child| self.build_widget(child))
                .collect();
            iced::widget::column(children).into()
        } else {
            iced::widget::column(vec![]).into()
        }
    }
}
