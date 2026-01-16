//! PickList widget builder

use crate::HandlerMessage;
use crate::builder::DampenWidgetBuilder;
use dampen_core::ir::node::WidgetNode;
use iced::{Element, Renderer, Theme};

impl<'a> DampenWidgetBuilder<'a> {
    /// Build a pick list widget from Dampen XML definition
    ///
    /// Supports the following attributes:
    /// - `options`: Comma-separated list of options
    /// - `selected`: String binding for selected option
    /// - `placeholder`: Placeholder text (currently unused)
    /// - `on_select`: Handler called on selection with selected value
    ///
    /// Events: Select (sends HandlerMessage::Handler(name, Some(selected_value)))
    pub(in crate::builder) fn build_pick_list(
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

        if self.verbose {
            eprintln!(
                "[DampenWidgetBuilder] Building pick_list: options={:?}, selected={:?}",
                options, selected
            );
        }

        // Get handler from events
        let on_select = node
            .events
            .iter()
            .find(|e| e.event == dampen_core::EventKind::Select)
            .map(|e| e.handler.clone());

        if self.verbose {
            if let Some(handler) = &on_select {
                eprintln!(
                    "[DampenWidgetBuilder] PickList has select event: handler={}",
                    handler
                );
            } else {
                eprintln!("[DampenWidgetBuilder] PickList has no select event");
            }
        }

        let pick_list = if let Some(handler_name) = on_select {
            if self.handler_registry.is_some() {
                if self.verbose {
                    eprintln!(
                        "[DampenWidgetBuilder] PickList: Attaching on_select with handler '{}'",
                        handler_name
                    );
                }
                iced::widget::pick_list(options, selected, move |selected_value| {
                    HandlerMessage::Handler(handler_name.clone(), Some(selected_value))
                })
            } else {
                if self.verbose {
                    eprintln!(
                        "[DampenWidgetBuilder] PickList: No handler_registry, cannot attach on_select"
                    );
                }
                iced::widget::pick_list(options, selected, |_| {
                    HandlerMessage::Handler("dummy".to_string(), None)
                })
            }
        } else {
            // If no handler, still need to provide one, but since no event, perhaps use a dummy
            iced::widget::pick_list(options, selected, |_| {
                HandlerMessage::Handler("dummy".to_string(), None)
            })
        };

        // TODO: State-aware styling available via map_picklist_status() - see style_mapping.rs
        // Note: Status::Opened is a struct variant with is_hovered field

        pick_list.into()
    }
}
