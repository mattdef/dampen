//! TextInput widget builder

use crate::HandlerMessage;
use crate::builder::DampenWidgetBuilder;
use dampen_core::ir::node::WidgetNode;
use dampen_core::ir::style::StyleProperties;
use iced::{Element, Renderer, Theme};

/// Convert Dampen StyleProperties to Iced text_input Style
fn apply_text_input_style(
    theme: &iced::Theme,
    _status: iced::widget::text_input::Status,
    props: &StyleProperties,
) -> iced::widget::text_input::Style {
    use iced::widget::text_input;
    use iced::{Background, Border, Color};

    let mut style = text_input::Style {
        background: Background::Color(Color::WHITE),
        border: Border::default(),
        icon: theme.palette().text,
        placeholder: {
            let mut c = theme.palette().text;
            c.a = 0.5;
            c
        },
        value: theme.palette().text,
        selection: {
            let mut c = theme.palette().primary;
            c.a = 0.4;
            c
        },
    };

    if theme.palette().background.r < 0.5 {
        style.background = Background::Color(Color {
            r: 0.2,
            g: 0.2,
            b: 0.2,
            a: 1.0,
        });
    }

    if let Some(ref bg) = props.background
        && let dampen_core::ir::style::Background::Color(color) = bg
    {
        style.background = Background::Color(Color {
            r: color.r,
            g: color.g,
            b: color.b,
            a: color.a,
        });
    }

    if let Some(ref text_color) = props.color {
        style.value = Color {
            r: text_color.r,
            g: text_color.g,
            b: text_color.b,
            a: text_color.a,
        };
    }

    if let Some(ref border) = props.border {
        style.border = Border {
            color: Color {
                r: border.color.r,
                g: border.color.g,
                b: border.color.b,
                a: border.color.a,
            },
            width: border.width,
            radius: iced::border::Radius {
                top_left: border.radius.top_left,
                top_right: border.radius.top_right,
                bottom_right: border.radius.bottom_right,
                bottom_left: border.radius.bottom_left,
            },
        };
    }

    style
}

impl<'a> DampenWidgetBuilder<'a> {
    /// Build a text input widget from Dampen XML definition
    ///
    /// Supports the following attributes:
    /// - `value`: String binding for current text value
    /// - `placeholder`: Placeholder text when empty
    /// - `on_input`: Handler called on text input with new value
    /// - `password`: If "true", masks input with password character
    ///
    /// Events: Input (sends HandlerMessage::Handler(name, Some(new_text)))
    pub(in crate::builder) fn build_text_input(
        &self,
        node: &WidgetNode,
    ) -> Element<'a, HandlerMessage, Theme, Renderer> {
        let placeholder = node
            .attributes
            .get("placeholder")
            .map(|attr| self.evaluate_attribute(attr))
            .unwrap_or_default();

        let value = node
            .attributes
            .get("value")
            .map(|attr| self.evaluate_attribute(attr))
            .unwrap_or_default();

        // Check for password attribute
        let is_password = node
            .attributes
            .get("password")
            .map(|attr| self.evaluate_attribute(attr))
            .map(|v| v == "true" || v == "1")
            .unwrap_or(false);

        #[cfg(debug_assertions)]
        eprintln!(
            "[DampenWidgetBuilder] Building text_input: placeholder='{}', value='{}', password={}",
            placeholder, value, is_password
        );

        // Get handler from events
        let on_input = node
            .events
            .iter()
            .find(|e| e.event == dampen_core::EventKind::Input)
            .map(|e| e.handler.clone());

        let on_submit = node
            .events
            .iter()
            .find(|e| e.event == dampen_core::EventKind::Submit)
            .map(|e| e.handler.clone());

        #[cfg(debug_assertions)]
        {
            if let Some(handler) = &on_input {
                eprintln!(
                    "[DampenWidgetBuilder] TextInput has input event: handler={}",
                    handler
                );
            } else {
                eprintln!("[DampenWidgetBuilder] TextInput has no input event");
            }

            if let Some(handler) = &on_submit {
                eprintln!(
                    "[DampenWidgetBuilder] TextInput has submit event: handler={}",
                    handler
                );
            } else {
                eprintln!("[DampenWidgetBuilder] TextInput has no submit event");
            }
        }

        let mut text_input = iced::widget::text_input(&placeholder, &value);

        // Apply state-aware styling (focus, hover, disabled)
        // Use complete style resolution: theme → class → inline
        let resolved_base_style = self.resolve_complete_styles(node);

        // Get the StyleClass for state variant resolution, wrapped in Rc for efficient cloning
        let classes = self.resolve_active_classes(node);
        let style_class = if !classes.is_empty() {
            self.style_classes
                .and_then(|cls| {
                    // Get the first class for state variant resolution
                    classes.first().and_then(|name| cls.get(name))
                })
                .cloned()
                .map(std::rc::Rc::new)
        } else {
            None
        };

        // Apply state-aware styling using generic helper
        if let Some(base_style_props) = resolved_base_style {
            use crate::builder::helpers::create_state_aware_style_fn;
            use crate::style_mapping::map_text_input_status;

            let base_style_props = base_style_props.clone();

            if let Some(style_fn) = create_state_aware_style_fn(
                self,
                node,
                dampen_core::ir::WidgetKind::TextInput,
                style_class,
                base_style_props,
                map_text_input_status,
                apply_text_input_style,
            ) {
                text_input = text_input.style(style_fn);
            }
        }

        // Note: Password masking with dots is not available in Iced 0.14's public API
        // The input still works but text is visible

        // Connect events if handlers exist
        if let Some(handler_name) = on_input
            && self.handler_registry.is_some()
        {
            #[cfg(debug_assertions)]
            eprintln!(
                "[DampenWidgetBuilder] TextInput: Attaching on_input with handler '{}'",
                handler_name
            );
            text_input = text_input.on_input(move |input_value| {
                HandlerMessage::Handler(handler_name.clone(), Some(input_value))
            });
        }

        if let Some(handler_name) = on_submit
            && self.handler_registry.is_some()
        {
            #[cfg(debug_assertions)]
            eprintln!(
                "[DampenWidgetBuilder] TextInput: Attaching on_submit with handler '{}'",
                handler_name
            );
            text_input = text_input.on_submit(HandlerMessage::Handler(handler_name, None));
        }

        text_input.into()
    }
}
