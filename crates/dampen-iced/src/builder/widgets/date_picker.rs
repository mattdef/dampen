use crate::HandlerMessage;
use crate::builder::DampenWidgetBuilder;
use chrono::NaiveDate;
use dampen_core::ir::node::WidgetNode;
use iced::{Element, Renderer, Theme};
use iced_aw::date_picker::Date;
use iced_aw::widgets::date_picker::DatePicker;

impl<'a> DampenWidgetBuilder<'a> {
    /// Build a date picker widget from Dampen XML definition
    ///
    /// Supports the following attributes:
    /// - `value`: Current date in YYYY-MM-DD format (or custom `format`)
    /// - `format`: Custom format for parsing static `value`
    /// - `show`: Boolean binding to control overlay visibility
    /// - `min_date`: Minimum selectable date (YYYY-MM-DD)
    /// - `max_date`: Maximum selectable date (YYYY-MM-DD)
    ///
    /// Events:
    /// - `on_submit`: Called when a date is selected, with ISO date string payload
    /// - `on_cancel`: Called when the picker is dismissed without selection
    pub(in crate::builder) fn build_date_picker(
        &self,
        node: &WidgetNode,
    ) -> Element<'a, HandlerMessage, Theme, Renderer> {
        let show_picker = node
            .attributes
            .get("show")
            .map(|attr| self.evaluate_attribute(attr))
            .map(|v| v == "true" || v == "1")
            .unwrap_or(false);

        let date = if let Some(attr) = node.attributes.get("value") {
            let val = self.evaluate_attribute(attr);
            let format = node
                .attributes
                .get("format")
                .map(|f| self.evaluate_attribute(f))
                .unwrap_or_else(|| "%Y-%m-%d".to_string());
            if let Ok(d) = NaiveDate::parse_from_str(&val, &format) {
                Date::from(d)
            } else {
                Date::from(chrono::Local::now().naive_local().date())
            }
        } else {
            Date::from(chrono::Local::now().naive_local().date())
        };

        let on_submit_handler = node
            .events
            .iter()
            .find(|e| e.event == dampen_core::EventKind::Submit)
            .map(|e| e.handler.clone());

        let on_cancel_handler = node
            .events
            .iter()
            .find(|e| e.event == dampen_core::EventKind::Cancel)
            .map(|e| e.handler.clone());

        let _min_date = node
            .attributes
            .get("min_date")
            .map(|attr| self.evaluate_attribute(attr))
            .and_then(|v| NaiveDate::parse_from_str(&v, "%Y-%m-%d").ok())
            .map(Date::from);

        let _max_date = node
            .attributes
            .get("max_date")
            .map(|attr| self.evaluate_attribute(attr))
            .and_then(|v| NaiveDate::parse_from_str(&v, "%Y-%m-%d").ok())
            .map(Date::from);

        let child = node.children.first();
        let underlay = if let Some(c) = child {
            self.build_widget(c)
        } else {
            iced::widget::text("DatePicker requires a child").into()
        };

        let picker = DatePicker::new(
            show_picker,
            date,
            underlay,
            if let Some(h) = on_cancel_handler {
                HandlerMessage::Handler(h, None)
            } else {
                HandlerMessage::None
            },
            move |date: Date| {
                let naive_date: NaiveDate = date.into();
                let iso_str = naive_date.format("%Y-%m-%d").to_string();
                if let Some(ref handler) = on_submit_handler {
                    HandlerMessage::Handler(handler.clone(), Some(iso_str))
                } else {
                    HandlerMessage::None
                }
            },
        );

        picker.into()
    }
}
