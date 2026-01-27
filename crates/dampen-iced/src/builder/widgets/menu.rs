use crate::HandlerMessage;
use crate::builder::DampenWidgetBuilder;
use crate::builder::helpers::{resolve_boolean_attribute, resolve_handler_param};
use dampen_core::ir::node::WidgetNode;
use dampen_core::ir::{EventKind, WidgetKind};
use iced::widget::{Space, button, container, row, rule, text};
use iced::{Element, Length, Renderer, Theme};
use iced_aw::menu::{Item, Menu, MenuBar};

impl<'a> DampenWidgetBuilder<'a> {
    /// Build a Menu widget (renders as a MenuBar)
    ///
    /// This is used for top-level menu bars (e.g. File, Edit, Help).
    pub(in crate::builder) fn build_menu(
        &self,
        node: &WidgetNode,
    ) -> Element<'a, HandlerMessage, Theme, Renderer> {
        let items = self.build_menu_items(&node.children, true);
        let mut menu_bar = MenuBar::new(items);

        // Resolve styles for the menu (from classes or inline)
        let resolved_style = self.resolve_complete_styles(node).unwrap_or_default();

        // Extract background color from XML if present
        let custom_bg = resolved_style.background.as_ref().and_then(|bg| match bg {
            dampen_core::ir::style::Background::Color(c) => Some(*c),
            _ => None,
        });

        // Extract border properties
        let border_radius = resolved_style
            .border
            .as_ref()
            .map(|b| b.radius.top_left)
            .unwrap_or(4.0);
        let border_color = resolved_style.border.as_ref().map(|b| b.color);

        // Standard MenuBar styling
        menu_bar = menu_bar.style(move |theme: &iced::Theme, _status| {
            let palette = theme.extended_palette();

            // Resolve background color: XML priority -> Theme fallback
            let mut bg_color = if let Some(c) = custom_bg {
                iced::Color {
                    r: c.r,
                    g: c.g,
                    b: c.b,
                    a: 1.0,
                }
            } else {
                palette.background.base.color
            };
            bg_color.a = 1.0;

            let b_color = if let Some(c) = border_color {
                if c.a > 0.0 {
                    iced::Color {
                        r: c.r,
                        g: c.g,
                        b: c.b,
                        a: 1.0,
                    }
                } else {
                    palette.background.strong.color
                }
            } else {
                palette.background.strong.color
            };

            iced_aw::menu::Style {
                bar_background: bg_color.into(),
                bar_border: iced::Border {
                    color: iced::Color::TRANSPARENT,
                    width: 0.0,
                    radius: 0.0.into(),
                },
                menu_background: bg_color.into(),
                menu_border: iced::Border {
                    color: b_color,
                    width: 1.0,
                    radius: border_radius.into(),
                },
                ..Default::default()
            }
        });

        // Apply styling and layout
        self.apply_style_layout(menu_bar, node)
    }

    /// Build a MenuItem widget (renders as a Button, potentially with submenu)
    ///
    /// If used outside a Menu, it renders as a standalone button.
    pub(in crate::builder) fn build_menu_item(
        &self,
        node: &WidgetNode,
    ) -> Element<'a, HandlerMessage, Theme, Renderer> {
        self.build_menu_item_content(node, false)
    }

    /// Build a MenuSeparator widget
    pub(in crate::builder) fn build_menu_separator(
        &self,
        node: &WidgetNode,
    ) -> Element<'a, HandlerMessage, Theme, Renderer> {
        // Horizontal rule as separator
        // TODO: height/color attributes
        let rule: iced::widget::Rule<'_, Theme> = rule::horizontal(1);
        self.apply_style_layout(rule, node)
    }

    /// Build a ContextMenu widget
    pub(in crate::builder) fn build_context_menu(
        &self,
        node: &WidgetNode,
    ) -> Element<'a, HandlerMessage, Theme, Renderer> {
        // Just return the underlay for now (first child)
        let underlay_node = node.children.first();
        let underlay = if let Some(n) = underlay_node {
            self.build_widget(n)
        } else {
            iced::widget::column(vec![]).into()
        };

        // If second child is menu, wrap in ContextMenu
        if let Some(menu_node) = node.children.get(1) {
            if menu_node.kind == WidgetKind::Menu {
                let menu_node_clone = menu_node.clone();
                let builder = self.clone();

                return iced_aw::ContextMenu::new(underlay, move || {
                    builder.build_menu_as_column(&menu_node_clone)
                })
                .into();
            }
        }

        underlay
    }

    // Helper to build menu as a vertical column (for ContextMenu)
    fn build_menu_as_column(
        &self,
        node: &WidgetNode,
    ) -> Element<'a, HandlerMessage, Theme, Renderer> {
        let children: Vec<_> = node
            .children
            .iter()
            .map(|child| match child.kind {
                WidgetKind::MenuItem => self.build_menu_item_content(child, false),
                WidgetKind::MenuSeparator => rule::horizontal(1).into(),
                _ => text("").into(),
            })
            .collect();

        container(iced::widget::column(children).padding(5).spacing(2))
            .style(|theme: &iced::Theme| {
                let palette = theme.extended_palette();
                let mut bg_color = palette.background.weak.color;
                bg_color.a = 1.0; // Force opaque for context menu

                container::Style {
                    background: Some(bg_color.into()),
                    border: iced::Border {
                        color: palette.background.strong.color,
                        width: 1.0,
                        radius: 4.0.into(),
                    },
                    ..Default::default()
                }
            })
            .into()
    }

    // Helper to recursively build menu items tree
    fn build_menu_items(
        &self,
        nodes: &[WidgetNode],
        is_top_level: bool,
    ) -> Vec<Item<'a, HandlerMessage, Theme, Renderer>> {
        nodes
            .iter()
            .filter_map(|node| match node.kind {
                WidgetKind::MenuItem => Some(self.build_menu_item_struct(node, is_top_level)),
                WidgetKind::MenuSeparator => Some(self.build_menu_separator_item(node)),
                _ => None,
            })
            .collect()
    }

    fn build_menu_item_struct(
        &self,
        node: &WidgetNode,
        is_top_level: bool,
    ) -> Item<'a, HandlerMessage, Theme, Renderer> {
        let content = self.build_menu_item_content(node, is_top_level);

        if let Some(submenu_node) = node.children.iter().find(|c| c.kind == WidgetKind::Menu) {
            let items = self.build_menu_items(&submenu_node.children, false);

            // Extract attributes from submenu XML
            let off_y = submenu_node
                .attributes
                .get("offset_y")
                .map(|v| self.evaluate_attribute(v).parse::<f32>().unwrap_or(0.0))
                .unwrap_or(0.0);

            let mut menu = Menu::new(items).offset(off_y);

            // Default width for submenus to avoid layout issues
            let mut width = 150.0;
            if let Some(w_attr) = submenu_node.attributes.get("width") {
                let w_str = self.evaluate_attribute(w_attr);
                if let Ok(w_val) = w_str.parse::<f32>() {
                    width = w_val;
                }
            }
            menu = menu.width(width);

            // Spacing between items
            let spacing = submenu_node
                .attributes
                .get("spacing")
                .map(|v| self.evaluate_attribute(v).parse::<f32>().unwrap_or(2.0))
                .unwrap_or(2.0);
            menu = menu.spacing(spacing);

            Item::with_menu(content, menu)
        } else {
            Item::new(content)
        }
    }

    fn build_menu_item_content(
        &self,
        node: &WidgetNode,
        is_top_level: bool,
    ) -> Element<'a, HandlerMessage, Theme, Renderer> {
        let label = if let Some(attr) = node.attributes.get("label") {
            self.evaluate_attribute(attr)
        } else {
            String::new()
        };

        let icon_str = node
            .attributes
            .get("icon")
            .map(|attr| self.evaluate_attribute(attr));

        let shortcut_str = node
            .attributes
            .get("shortcut")
            .map(|attr| self.evaluate_attribute(attr));

        // Build content row
        let mut content_row = row![]
            .align_y(iced::Alignment::Center)
            .spacing(10)
            .width(Length::Shrink);

        if let Some(icon) = icon_str {
            content_row = content_row.push(text(icon).width(20));
        }

        content_row = content_row.push(text(label).width(Length::Fill));

        if let Some(shortcut) = shortcut_str {
            content_row = content_row.push(Space::new().width(Length::Fill));
            content_row = content_row.push(text(shortcut));
        }

        // Check for nested menu (submenu)
        let has_submenu = node.children.iter().any(|c| c.kind == WidgetKind::Menu);

        if is_top_level && has_submenu {
            // Submenu trigger in MenuBar: Use a button with hover support
            button(content_row)
                .padding([6, 12])
                .width(Length::Shrink)
                .style(|theme: &iced::Theme, status| {
                    let palette = theme.extended_palette();
                    let mut style = iced::widget::button::Style::default();
                    style.background = match status {
                        iced::widget::button::Status::Hovered
                        | iced::widget::button::Status::Pressed => {
                            Some(palette.background.strong.color.into())
                        }
                        _ => None,
                    };
                    style.text_color = palette.background.base.text;
                    style
                })
                .into()
        } else {
            let mut btn = button(content_row)
                .padding([6, 12]) // Consistent padding
                .width(Length::Fill)
                .style(|theme: &iced::Theme, status| {
                    let palette = theme.extended_palette();
                    let mut style = iced::widget::button::Style::default();
                    style.background = match status {
                        iced::widget::button::Status::Hovered
                        | iced::widget::button::Status::Pressed => {
                            Some(palette.primary.weak.color.into())
                        }
                        _ => None,
                    };
                    style.text_color = match status {
                        iced::widget::button::Status::Hovered
                        | iced::widget::button::Status::Pressed => palette.primary.weak.text,
                        _ => palette.background.base.text,
                    };
                    style
                });

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
    }

    fn build_menu_separator_item(
        &self,
        _node: &WidgetNode,
    ) -> Item<'a, HandlerMessage, Theme, Renderer> {
        // Separator item
        let rule: iced::widget::Rule<'_, Theme> = rule::horizontal(1);
        Item::new(rule)
    }
}
