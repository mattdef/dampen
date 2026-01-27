use crate::HandlerMessage;
use crate::builder::DampenWidgetBuilder;
use dampen_core::binding::BindingValue;
use dampen_core::expr::evaluate_binding_expr_with_shared;
use dampen_core::ir::node::{AttributeValue, WidgetKind, WidgetNode};
use iced::widget::table::{Column, Table};
use iced::{Element, Length, Renderer, Theme};

impl<'a> DampenWidgetBuilder<'a> {
    pub(in crate::builder) fn build_data_table(
        &self,
        node: &WidgetNode,
    ) -> Element<'a, HandlerMessage, Theme, Renderer>
    where
        HandlerMessage: Clone + 'static,
    {
        // 1. Resolve data binding
        let data_values: Vec<BindingValue> = match node.attributes.get("data") {
            Some(AttributeValue::Binding(expr)) => {
                // Try context first, then model
                let binding_result = if let Some(ctx_value) = self.resolve_from_context(expr) {
                    Ok(ctx_value)
                } else {
                    evaluate_binding_expr_with_shared(expr, self.model, self.shared_context)
                };

                match binding_result {
                    Ok(BindingValue::List(items)) => items,
                    Ok(_) => {
                        #[cfg(debug_assertions)]
                        eprintln!("[DampenWidgetBuilder] DataTable 'data' is not a list");
                        Vec::new()
                    }
                    Err(e) => {
                        #[cfg(debug_assertions)]
                        eprintln!("[DampenWidgetBuilder] DataTable evaluation error: {}", e);
                        Vec::new()
                    }
                }
            }
            _ => Vec::new(),
        };

        // Wrap data with index for context
        // Capture data for event handling
        // let data_source = data_values.clone();
        let data: Vec<(usize, BindingValue)> = data_values.into_iter().enumerate().collect();

        // 2. Build columns
        let columns: Vec<Column<'a, 'a, (usize, BindingValue), HandlerMessage, Theme, Renderer>> = node
            .children
            .iter()
            .filter(|c| c.kind == WidgetKind::DataColumn)
            .map(|col_node| {
                let header = col_node
                    .attributes
                    .get("header")
                    .map(|a| self.evaluate_attribute(a))
                    .unwrap_or_default();

                let field = col_node
                    .attributes
                    .get("field")
                    .map(|a| self.evaluate_attribute(a));

                let width = col_node
                    .attributes
                    .get("width")
                    .map(|a| self.evaluate_attribute(a))
                    .map(|s| self.parse_length_string(&s))
                    .unwrap_or(Length::Fill);

                // Capture builder and children for template rendering
                let builder = self.clone();
                let children = col_node.children.clone();

                // Use iced::widget::table::column function
                let col = iced::widget::table::column(
                    iced::widget::text(header),
                    move |(index, item): (usize, BindingValue)| {
                        if let Some(ref f) = field {
                            // Simple field access
                            let val = item.get_field(f).unwrap_or(BindingValue::None);
                            Element::from(iced::widget::text(val.to_display_string()))
                        } else {
                            // Template handling (T012/T013)
                            // Find child to render (either <template> content or direct children)
                            let template_content = if let Some(tmpl) = children.iter().find(|c| matches!(c.kind, WidgetKind::Custom(ref s) if s == "template")) {
                                &tmpl.children
                            } else {
                                &children
                            };

                            if let Some(root) = template_content.first() {
                                // Clone builder to create local context scope
                                let scoped_builder = builder.clone();
                                scoped_builder.push_context("index", BindingValue::Integer(index as i64));
                                scoped_builder.push_context("item", item.clone());
                                
                                let widget = scoped_builder.build_widget(root);
                                
                                // Pop context happens when scoped_builder is dropped? 
                                // No, we modified the RefCell in scoped_builder.
                                // Since scoped_builder shares the same RefCell (it was shallow cloned but RefCell is deep cloned in logic? No.)
                                // Wait, RefCell<Vec<...>> in DampenWidgetBuilder.
                                // If DampenWidgetBuilder derives Clone, `RefCell` is NOT implicitly deep cloned if it's `RefCell<T>`.
                                // `RefCell` does NOT implement Clone unless T implements Clone?
                                // Actually `RefCell` implements Clone if T does.
                                // `Vec` implements Clone. `HashMap` implements Clone.
                                // So `self.clone()` creates a NEW RefCell with CLONED data.
                                // So modifications to scoped_builder.binding_context DO NOT affect `builder` or `self`.
                                // So we don't need to pop.
                                
                                widget
                            } else {
                                Element::from(iced::widget::text(""))
                            }
                        }
                    },
                );
                
                col.width(width)
            })
            .collect();

        // 3. Create Table
        let table = Table::new(columns, data);

        // Handle on_row_click
        // TODO: Re-enable when Table API for row clicks is identified
        /*
        if let Some(event) = node.events.iter().find(|e| e.event == EventKind::RowClick) {
            let handler_name = event.handler.clone();
            let param_expr = event.param.clone();
            let data_source = data_source.clone();
            let builder = self.clone();
            let message_factory = self.message_factory.clone();

            table = table.on_click(move |index: usize| {
                let item = data_source.get(index).cloned().unwrap_or(BindingValue::None);

                let param_value = if let Some(ref expr) = param_expr {
                    builder.push_context("index", BindingValue::Integer(index as i64));
                    builder.push_context("item", item.clone());

                    let val = builder
                        .evaluate_binding_with_context(expr)
                        .unwrap_or(BindingValue::None);

                    builder.pop_context();
                    Some(val.to_display_string())
                } else {
                    None
                };

                message_factory(&handler_name, param_value)
            });
        }
        */

        // 4. Apply layout and styling
        self.apply_style_layout(table, node)
    }

    /// Helper to parse length string locally (if not available in self)
    fn parse_length_string(&self, s: &str) -> Length {
        let s = s.trim().to_lowercase();
        if s == "fill" {
            Length::Fill
        } else if s == "shrink" {
            Length::Shrink
        } else if let Some(pct) = s.strip_suffix('%') {
            if let Ok(p) = pct.parse::<f32>() {
                // Iced doesn't have a direct percentage, use FillPortion as approximation
                let portion = ((p / 100.0) * 16.0).round() as u16;
                let portion = portion.max(1);
                Length::FillPortion(portion)
            } else {
                Length::Shrink
            }
        } else if let Ok(v) = s.parse::<f32>() {
            Length::Fixed(v)
        } else {
            Length::Fill // Default to fill for columns usually good
        }
    }
}
