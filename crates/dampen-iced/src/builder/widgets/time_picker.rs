use crate::HandlerMessage;
use crate::builder::DampenWidgetBuilder;
use chrono::NaiveTime;
use dampen_core::ir::node::WidgetNode;
use iced::{Element, Renderer, Theme};
use iced_aw::time_picker::Time;
use iced_aw::widgets::time_picker::TimePicker;

impl<'a> DampenWidgetBuilder<'a> {
    /// Build a time picker widget from Dampen XML definition
    ///
    /// Supports the following attributes:
    /// - `value`: Current time in HH:MM:SS format (or custom `format`)
    /// - `format`: Custom format for parsing static `value`
    /// - `show`: Boolean binding to control overlay visibility
    /// - `use_24h`: Use 24-hour format (static boolean)
    /// - `show_seconds`: Show seconds selector (static boolean)
    ///
    /// Events:
    /// - `on_submit`: Called when a time is selected, with HH:MM:SS string payload
    /// - `on_cancel`: Called when the picker is dismissed without selection
    pub(in crate::builder) fn build_time_picker(
        &self,
        node: &WidgetNode,
    ) -> Element<'a, HandlerMessage, Theme, Renderer> {
        let show_picker = node
            .attributes
            .get("show")
            .map(|attr| self.evaluate_attribute(attr))
            .map(|v| v == "true" || v == "1")
            .unwrap_or(false);

        let time = if let Some(attr) = node.attributes.get("value") {
            let val = self.evaluate_attribute(attr);
            let format = node
                .attributes
                .get("format")
                .map(|f| self.evaluate_attribute(f))
                .unwrap_or_else(|| "%H:%M:%S".to_string());
            if let Ok(t) = NaiveTime::parse_from_str(&val, &format) {
                Time::from(t)
            } else {
                Time::from(chrono::Local::now().naive_local().time())
            }
        } else {
            Time::from(chrono::Local::now().naive_local().time())
        };

        // Configuration
        let use_24h = node
            .attributes
            .get("use_24h")
            .map(|attr| self.evaluate_attribute(attr))
            .map(|v| v == "true" || v == "1")
            .unwrap_or(false);

        let show_seconds = node
            .attributes
            .get("show_seconds")
            .map(|attr| self.evaluate_attribute(attr))
            .map(|v| v == "true" || v == "1")
            .unwrap_or(false);

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

        let child = node.children.first();
        let underlay = if let Some(c) = child {
            self.build_widget(c)
        } else {
            iced::widget::text("TimePicker requires a child").into()
        };

        let mut picker = TimePicker::new(
            show_picker,
            time,
            underlay,
            if let Some(h) = on_cancel_handler {
                HandlerMessage::Handler(h, None)
            } else {
                HandlerMessage::None
            },
            move |time: Time| {
                let naive_time: NaiveTime = time.into();
                let iso_str = naive_time.format("%H:%M:%S").to_string();
                if let Some(ref handler) = on_submit_handler {
                    HandlerMessage::Handler(handler.clone(), Some(iso_str))
                } else {
                    HandlerMessage::None
                }
            },
        );

        if use_24h {
            picker = picker.use_24h();
        }

        if show_seconds {
            picker = picker.show_seconds();
        }

        picker.into()
    }
}
