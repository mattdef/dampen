//! ComboBox widget builder

use crate::HandlerMessage;
use crate::builder::DampenWidgetBuilder;
use dampen_core::ir::node::WidgetNode;
use iced::{Element, Renderer, Theme};

impl<'a> DampenWidgetBuilder<'a> {
    /// Build a combo box widget from Dampen XML definition
    ///
    /// ComboBox is implemented using pick_list as a dropdown selector.
    /// Supports the following attributes:
    /// - `options`: Comma-separated list of options
    /// - `selected`: String binding for selected option
    /// - `placeholder`: Placeholder text when nothing is selected
    /// - `on_select`: Handler called on selection with selected value
    ///
    /// Events: Select (sends HandlerMessage::Handler(name, Some(selected_value)))
    pub(in crate::builder) fn build_combo_box(
        &self,
        node: &WidgetNode,
    ) -> Element<'a, HandlerMessage, Theme, Renderer> {
        let options_str = node
            .attributes
            .get("options")
            .map(|attr| self.evaluate_attribute(attr))
            .unwrap_or_default();

        let options: Vec<String> = options_str
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        let selected_str = node
            .attributes
            .get("selected")
            .map(|attr| self.evaluate_attribute(attr))
            .unwrap_or_default();

        let selected = if selected_str.is_empty() {
            None
        } else {
            options.iter().find(|o| *o == &selected_str).cloned()
        };

        #[cfg(debug_assertions)]
        eprintln!(
            "[DampenWidgetBuilder] Building combo_box: options={:?}, selected={:?}",
            options, selected
        );

        // Get handler from events
        let on_select = node
            .events
            .iter()
            .find(|e| e.event == dampen_core::EventKind::Select)
            .map(|e| e.handler.clone());

        #[cfg(debug_assertions)]
        if let Some(handler) = &on_select {
            eprintln!(
                "[DampenWidgetBuilder] ComboBox has select event: handler={}",
                handler
            );
        } else {
            eprintln!("[DampenWidgetBuilder] ComboBox has no select event");
        }

        // Use pick_list as combobox implementation
        let combo_box = if let Some(handler_name) = on_select {
            if self.handler_registry.is_some() {
                #[cfg(debug_assertions)]
                eprintln!(
                    "[DampenWidgetBuilder] ComboBox: Attaching on_select with handler '{}'",
                    handler_name
                );
                iced::widget::pick_list(options, selected, move |selected_value| {
                    HandlerMessage::Handler(handler_name.clone(), Some(selected_value))
                })
            } else {
                #[cfg(debug_assertions)]
                eprintln!(
                    "[DampenWidgetBuilder] ComboBox: No handler_registry, cannot attach on_select"
                );
                iced::widget::pick_list(options, selected, |_| {
                    HandlerMessage::Handler("dummy".to_string(), None)
                })
            }
        } else {
            iced::widget::pick_list(options, selected, |_| {
                HandlerMessage::Handler("dummy".to_string(), None)
            })
        };

        // TODO: State-aware styling for ComboBox - see style_mapping.rs
        // NOTE: Current implementation uses pick_list widget (so would use map_picklist_status)
        // If switching to real combo_box widget, use map_text_input_status() since
        // Iced combo_box uses text_input::Status enum (no separate combo_box::Status)

        combo_box.into()
    }
}
