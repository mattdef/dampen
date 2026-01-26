//! For loop widget builder

use crate::HandlerMessage;
use crate::builder::DampenWidgetBuilder;
use dampen_core::binding::BindingValue;
use dampen_core::expr::evaluate_binding_expr_with_shared;
use dampen_core::ir::node::{AttributeValue, WidgetNode};
use iced::{Element, Renderer, Theme};

impl<'a> DampenWidgetBuilder<'a> {
    /// Build a `<for>` loop widget
    ///
    /// Iterates over a collection and renders the child widgets for each item,
    /// with the loop variable available in the binding context.
    ///
    /// # Example XML
    ///
    /// ```xml
    /// <for each="item" in="{items}">
    ///     <text value="{item.text}" />
    /// </for>
    /// ```
    pub(in crate::builder) fn build_for(
        &self,
        node: &WidgetNode,
    ) -> Element<'a, HandlerMessage, Theme, Renderer>
    where
        HandlerMessage: Clone + 'static,
    {
        // Get variable name
        let var_name = match node.attributes.get("each") {
            Some(AttributeValue::Static(name)) => name.clone(),
            _ => {
                #[cfg(debug_assertions)]
                eprintln!("[DampenWidgetBuilder] For loop missing 'each' attribute");
                return iced::widget::column(vec![]).into();
            }
        };

        // Evaluate collection binding
        let collection_values = match node.attributes.get("in") {
            Some(AttributeValue::Binding(expr)) => {
                // Try context first, then model
                let binding_result = if let Some(ctx_value) = self.resolve_from_context(expr) {
                    Ok(ctx_value)
                } else {
                    evaluate_binding_expr_with_shared(expr, self.model, self.shared_context)
                };

                match binding_result {
                    Ok(BindingValue::List(items)) => items,
                    Ok(other) => {
                        #[cfg(debug_assertions)]
                        eprintln!(
                            "[DampenWidgetBuilder] For loop 'in' is not a list: {:?}",
                            other
                        );
                        return iced::widget::column(vec![]).into();
                    }
                    Err(e) => {
                        #[cfg(debug_assertions)]
                        eprintln!("[DampenWidgetBuilder] For loop evaluation error: {}", e);
                        return iced::widget::column(vec![]).into();
                    }
                }
            }
            _ => {
                #[cfg(debug_assertions)]
                eprintln!("[DampenWidgetBuilder] For loop missing 'in' binding");
                return iced::widget::column(vec![]).into();
            }
        };

        #[cfg(debug_assertions)]
        eprintln!(
            "[DampenWidgetBuilder] For loop rendering {} items as '{}'",
            collection_values.len(),
            var_name
        );

        // Render children for each item
        let mut rendered_children = Vec::new();

        for (index, item_value) in collection_values.iter().enumerate() {
            // Push context
            self.push_context(&var_name, item_value.clone());
            self.push_context("index", BindingValue::Integer(index as i64));

            // Render all template children
            for child in &node.children {
                rendered_children.push(self.build_widget(child));
            }

            // Pop context
            self.pop_context(); // index
            self.pop_context(); // item
        }

        // Return as column
        iced::widget::column(rendered_children).into()
    }
}
