//! Grid widget builder

use crate::HandlerMessage;
use crate::builder::DampenWidgetBuilder;
use dampen_core::ir::node::{AttributeValue, WidgetNode};
use iced::{Element, Renderer, Theme};

impl<'a> DampenWidgetBuilder<'a> {
    /// Build a grid widget
    ///
    /// Creates a grid layout by grouping children into rows based on the `columns` attribute.
    ///
    /// # Example XML
    ///
    /// ```xml
    /// <grid columns="3" spacing="10">
    ///     <text value="Cell 1" />
    ///     <text value="Cell 2" />
    ///     <text value="Cell 3" />
    ///     <text value="Cell 4" />
    /// </grid>
    /// ```
    pub(in crate::builder) fn build_grid(
        &self,
        node: &WidgetNode,
    ) -> Element<'a, HandlerMessage, Theme, Renderer>
    where
        HandlerMessage: Clone + 'static,
    {
        // Parse columns attribute (validated by parser, so it exists)
        let columns = match node.attributes.get("columns") {
            Some(AttributeValue::Static(s)) => s.parse::<usize>().unwrap_or(1),
            _ => 1,
        };

        // Parse spacing attribute
        let spacing = match node.attributes.get("spacing") {
            Some(attr) => self.evaluate_attribute(attr).parse::<f32>().unwrap_or(10.0),
            None => 10.0,
        };

        #[cfg(debug_assertions)]
        eprintln!(
            "[DampenWidgetBuilder] Building Grid: {} columns, spacing {}",
            columns, spacing
        );

        // Group child nodes into rows and build widgets
        let mut rows = Vec::new();

        for chunk in node.children.chunks(columns) {
            let row_widgets: Vec<_> = chunk.iter().map(|child| self.build_widget(child)).collect();

            let row = iced::widget::row(row_widgets).spacing(spacing);
            rows.push(row.into());
        }

        iced::widget::column(rows).spacing(spacing).into()
    }
}
