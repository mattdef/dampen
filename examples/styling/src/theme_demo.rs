//! Theme switching demo
//!
//! Demonstrates runtime theme switching with state preservation.

use gravity_core::{parse, GravityDocument};
use gravity_runtime::{ThemeManager, StyleCascade};
use iced::widget::{column, text, button, row, container};
use iced::{Element, Task, Theme as IcedTheme};

#[derive(Clone, Debug)]
pub enum Message {
    SwitchToLight,
    SwitchToDark,
    SwitchToCustom,
    Increment,
    Decrement,
}

pub struct ThemeDemoState {
    document: GravityDocument,
    theme_manager: ThemeManager,
    count: i32,
}

impl ThemeDemoState {
    pub fn new() -> Self {
        let ui_path = std::path::PathBuf::from("examples/styling/ui/theme_demo.gravity");
        let xml = match std::fs::read_to_string(&ui_path) {
            Ok(content) => content,
            Err(e) => {
                eprintln!("Error: Failed to read UI file: {}", e);
                // Fallback to inline XML
                r#"<gravity>
                    <themes>
                        <theme name="custom">
                            <palette primary="#e74c3c" secondary="#e67e22" background="#ecf0f1" text="#2c3e50" />
                            <typography font_family="Inter" font_size_base="16" />
                            <spacing unit="8" />
                        </theme>
                    </themes>
                    <global_theme name="light" />
                    <column padding="40" spacing="20">
                        <text value="Theme Demo" size="32" weight="bold" />
                        <text value="Click buttons to switch themes" size="16" />
                        <row spacing="10">
                            <button label="Light" on_click="switch_light" />
                            <button label="Dark" on_click="switch_dark" />
                            <button label="Custom" on_click="switch_custom" />
                        </row>
                        <text value="Count: {count}" size="24" />
                        <row spacing="10">
                            <button label="+" on_click="increment" />
                            <button label="-" on_click="decrement" />
                        </row>
                    </column>
                </gravity>"#.to_string()
            }
        };

        let document = parse(&xml).unwrap_or_else(|e| {
            eprintln!("Error: Failed to parse UI: {}", e);
            GravityDocument::default()
        });

        // Initialize theme manager
        let mut theme_manager = ThemeManager::new();
        theme_manager.load_from_document(&document);

        Self {
            document,
            theme_manager,
            count: 0,
        }
    }

    pub fn get_current_theme(&self) -> IcedTheme {
        if let Some(theme) = self.theme_manager.get_current_theme() {
            ThemeAdapter::to_iced(theme)
        } else {
            IcedTheme::Light
        }
    }
}

fn update(state: &mut ThemeDemoState, message: Message) -> Task<Message> {
    match message {
        Message::SwitchToLight => {
            let _ = state.theme_manager.set_theme("light".to_string());
        }
        Message::SwitchToDark => {
            let _ = state.theme_manager.set_theme("dark".to_string());
        }
        Message::SwitchToCustom => {
            let _ = state.theme_manager.set_theme("custom".to_string());
        }
        Message::Increment => state.count += 1,
        Message::Decrement => state.count -= 1,
    }
    Task::none()
}

fn render_node<'a>(
    node: &'a gravity_core::WidgetNode,
    count: i32,
    theme_manager: &ThemeManager,
) -> Element<'a, Message> {
    use gravity_core::{WidgetKind, AttributeValue};

    match node.kind {
        WidgetKind::Text => {
            let value = match node.attributes.get("value") {
                Some(AttributeValue::Static(v)) => {
                    if v.contains("{count}") {
                        v.replace("{count}", &count.to_string())
                    } else {
                        v.clone()
                    }
                }
                _ => String::new(),
            };
            
            // Apply theme-aware styling
            let mut text_widget = text(value);
            
            // Get theme for color
            if let Some(theme) = theme_manager.get_current_theme() {
                text_widget = text_widget.color(map_color(&theme.palette.text));
            }
            
            text_widget.into()
        }
        WidgetKind::Button => {
            let label = match node.attributes.get("label") {
                Some(AttributeValue::Static(l)) => l.clone(),
                _ => String::new(),
            };
            
            let msg = if let Some(AttributeValue::Static(handler)) = node.attributes.get("on_click") {
                match handler.as_str() {
                    "switch_light" => Message::SwitchToLight,
                    "switch_dark" => Message::SwitchToDark,
                    "switch_custom" => Message::SwitchToCustom,
                    "increment" => Message::Increment,
                    "decrement" => Message::Decrement,
                    _ => Message::Increment,
                }
            } else {
                Message::Increment
            };
            
            // Apply theme-aware styling
            let mut btn = button(text(label)).on_press(msg);
            
            // Get theme colors for button
            if let Some(theme) = theme_manager.get_current_theme() {
                use iced::widget::button::Style;
                use iced::Background;
                
                let primary = map_color(&theme.palette.primary);
                let text_color = map_color(&theme.palette.text);
                
                btn = btn.style(move |_theme, _status| Style {
                    background: Some(Background::Color(primary)),
                    text_color: Some(text_color),
                    ..Default::default()
                });
            }
            
            btn.into()
        }
        WidgetKind::Column => {
            let children: Vec<_> = node.children.iter()
                .map(|child| render_node(child, count, theme_manager))
                .collect();
            column(children).spacing(20).into()
        }
        WidgetKind::Row => {
            let children: Vec<_> = node.children.iter()
                .map(|child| render_node(child, count, theme_manager))
                .collect();
            row(children).spacing(10).into()
        }
        WidgetKind::Container => {
            let children: Vec<_> = node.children.iter()
                .map(|child| render_node(child, count, theme_manager))
                .collect();
            
            // Apply theme-aware container styling
            let mut container_widget = container(column(children));
            
            if let Some(theme) = theme_manager.get_current_theme() {
                use iced::widget::container::Style;
                use iced::Background;
                
                container_widget = container_widget.style(Style {
                    background: Some(Background::Color(map_color(&theme.palette.surface))),
                    text_color: Some(map_color(&theme.palette.text)),
                    border: iced::Border::default(),
                    shadow: iced::Shadow::default(),
                });
            }
            
            container_widget.into()
        }
        _ => column(Vec::new()).into(),
    }
}

fn view(state: &ThemeDemoState) -> Element<Message> {
    // Create a styled container with the current theme
    let content = render_node(&state.document.root, state.count, &state.theme_manager);
    
    // Wrap in a container with theme background
    let theme = state.get_current_theme();
    container(content)
        .style(|_theme, _status| iced::widget::container::Style {
            background: Some(iced::Background::Color(iced::Color::from_rgb(0.95, 0.95, 0.95))),
            ..Default::default()
        })
        .padding(20)
        .into()
}

pub fn main() -> iced::Result {
    let state = ThemeDemoState::new();
    let initial_theme = state.get_current_theme();
    
    iced::application(
        || state,
        update,
        view,
    )
    .theme(|_state| initial_theme)
    .run()
}

/// Map Gravity Color to Iced Color
fn map_color(color: &gravity_core::ir::style::Color) -> iced::Color {
    iced::Color {
        r: color.r,
        g: color.g,
        b: color.b,
        a: color.a,
    }
}
