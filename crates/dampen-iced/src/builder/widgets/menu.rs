use crate::HandlerMessage;
use crate::builder::DampenWidgetBuilder;
use crate::builder::helpers::{resolve_boolean_attribute, resolve_handler_param};
use dampen_core::ir::node::WidgetNode;
use dampen_core::ir::{EventKind, WidgetKind};
use iced::widget::{button, rule, text};
use iced::{Element, Length, Renderer, Theme};
use iced_aw::menu::{Item, Menu, MenuBar};

impl<'a> DampenWidgetBuilder<'a> {
    pub(in crate::builder) fn build_menu(
        &self,
        node: &WidgetNode,
    ) -> Element<'a, HandlerMessage, Theme, Renderer> {
        let items = self.build_menu_items(&node.children);
        let menu_bar = MenuBar::new(items);

        // Apply styling and layout
        self.apply_style_layout(menu_bar, node)
    }

    pub(in crate::builder) fn build_menu_item(
        &self,
        node: &WidgetNode,
    ) -> Element<'a, HandlerMessage, Theme, Renderer> {
        // Standalone menu item (rarely used outside menu)
        let label = if let Some(attr) = node.attributes.get("label") {
            self.evaluate_attribute(attr)
        } else {
            String::new()
        };

        let mut btn = button(text(label));

        // Handle on_click
        if let Some(event) = node.events.iter().find(|e| e.event == EventKind::Click) {
            let msg = if let Some(param_expr) = &event.param {
                match resolve_handler_param(self, param_expr) {
                    Ok(value) => {
                        (self.message_factory)(&event.handler, Some(value.to_display_string()))
                    }
                    Err(e) => {
                        #[cfg(debug_assertions)]
                        eprintln!("{}", e);
                        (self.message_factory)(&event.handler, None)
                    }
                }
            } else {
                (self.message_factory)(&event.handler, None)
            };

            if !resolve_boolean_attribute(self, node, "disabled", false) {
                btn = btn.on_press(msg);
            }
        }

        self.apply_style_layout(btn, node)
    }

    pub(in crate::builder) fn build_menu_separator(
        &self,
        node: &WidgetNode,
    ) -> Element<'a, HandlerMessage, Theme, Renderer> {
        // Horizontal rule as separator
        // TODO: height/color attributes
        let rule: iced::widget::Rule<'_, Theme> = rule::horizontal(1);
        self.apply_style_layout(rule, node)
    }

    // Helper to recursively build menu items tree
    fn build_menu_items(
        &self,
        nodes: &[WidgetNode],
    ) -> Vec<Item<'a, HandlerMessage, Theme, Renderer>> {
        nodes
            .iter()
            .filter_map(|node| match node.kind {
                WidgetKind::MenuItem => Some(self.build_menu_item_struct(node)),
                WidgetKind::MenuSeparator => Some(self.build_menu_separator_item(node)),
                _ => None,
            })
            .collect()
    }

    fn build_menu_item_struct(
        &self,
        node: &WidgetNode,
    ) -> Item<'a, HandlerMessage, Theme, Renderer> {
        let label = if let Some(attr) = node.attributes.get("label") {
            self.evaluate_attribute(attr)
        } else {
            String::new()
        };

        // Build the content button
        // Note: For menu items inside MenuBar, we might want different styling than standard buttons
        // iced_aw usually handles basic styling, but we can customize the content.
        let mut btn = button(text(label))
            .width(Length::Fill)
            .style(iced::widget::button::text);

        // Handle on_click
        if let Some(event) = node.events.iter().find(|e| e.event == EventKind::Click) {
            let msg = if let Some(param_expr) = &event.param {
                match resolve_handler_param(self, param_expr) {
                    Ok(value) => {
                        (self.message_factory)(&event.handler, Some(value.to_display_string()))
                    }
                    Err(e) => {
                        #[cfg(debug_assertions)]
                        eprintln!("{}", e);
                        (self.message_factory)(&event.handler, None)
                    }
                }
            } else {
                (self.message_factory)(&event.handler, None)
            };

            if !resolve_boolean_attribute(self, node, "disabled", false) {
                btn = btn.on_press(msg);
            }
        }

        // TODO: Apply styling from node attributes (padding, etc) via apply_style_layout?
        // But Item expects Element.
        // We can wrap button in container with style.
        // But apply_style_layout applies Container.

        let content: Element<_, _, _> = btn.into();

        // Check for nested menu (submenu)
        let submenu_node = node.children.iter().find(|c| c.kind == WidgetKind::Menu);

        if let Some(submenu) = submenu_node {
            let items = self.build_menu_items(&submenu.children);
            let menu = Menu::new(items);
            // TODO: Attributes from submenu (max_width, etc)
            Item::with_menu(content, menu)
        } else {
            Item::new(content)
        }
    }

    fn build_menu_separator_item(
        &self,
        _node: &WidgetNode,
    ) -> Item<'a, HandlerMessage, Theme, Renderer> {
        // Separator item
        let rule: iced::widget::Rule<'_, Theme> = rule::horizontal(1);
        Item::new(rule)
    }

    // Placeholder for US2
    pub(in crate::builder) fn build_context_menu(
        &self,
        node: &WidgetNode,
    ) -> Element<'a, HandlerMessage, Theme, Renderer> {
        // Just return the underlay for now (first child)
        if let Some(underlay) = node.children.first() {
            self.build_widget(underlay)
        } else {
            iced::widget::column(vec![]).into()
        }
    }
}
