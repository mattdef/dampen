use gravity_core::{parse, WidgetNode};
use gravity_runtime::{ThemeManager, StyleCascade};
use gravity_iced::{render, IcedBackend};
use iced::widget::{column, text, button, row, container};
use iced::{Element, Task, Theme as IcedTheme};

#[derive(Clone, Debug)]
pub enum Message {
    PrimaryClicked,
    DangerClicked,
    BaseClicked,
    Nested1,
    Nested2,
    NestedDanger,
    Increment,
    Decrement,
    Reset,
    DynamicClick,
    DynamicDanger,
}

pub struct AppState {
    document: gravity_core::GravityDocument,
    theme_manager: ThemeManager,
    style_cascade: StyleCascade,
    count: i32,
    clicks: i32,
}

impl AppState {
    fn new() -> Self {
        let ui_path = std::path::PathBuf::from("examples/class-demo/ui/main.gravity");
        let xml = match std::fs::read_to_string(&ui_path) {
            Ok(content) => content,
            Err(e) => {
                eprintln!("Error: Failed to read UI file: {}", e);
                // Fallback to inline XML
                r#"<gravity>
                    <themes>
                        <theme name="light">
                            <palette primary="#3498db" secondary="#2ecc71" background="#ecf0f1" text="#2c3e50" />
                            <typography font_family="sans-serif" font_size_base="16" />
                            <spacing unit="8" />
                        </theme>
                    </themes>
                    <style_classes>
                        <style name="button_primary" background="#3498db" color="#ffffff" padding="12 24" />
                    </style_classes>
                    <global_theme name="light" />
                    <column padding="40" spacing="20">
                        <text value="Class Demo" size="32" weight="bold" />
                        <button class="button_primary" label="Test" on_click="test" />
                    </column>
                </gravity>"#.to_string()
            }
        };

        let document = parse(&xml).unwrap_or_else(|e| {
            eprintln!("Error: Failed to parse UI: {}", e);
            gravity_core::GravityDocument::default()
        });

        // Initialize theme manager
        let mut theme_manager = ThemeManager::new();
        theme_manager.load_from_document(&document);

        // Initialize style cascade
        let style_cascade = StyleCascade::new(&document);

        Self {
            document,
            theme_manager,
            style_cascade,
            count: 0,
            clicks: 0,
        }
    }

    fn get_current_theme(&self) -> IcedTheme {
        if let Some(theme) = self.theme_manager.get_current_theme() {
            gravity_iced::theme_adapter::ThemeAdapter::to_iced(theme)
        } else {
            IcedTheme::Light
        }
    }
}

fn update(state: &mut AppState, message: Message) -> Task<Message> {
    match message {
        Message::PrimaryClicked => {
            eprintln!("[EVENT] Primary button clicked");
            state.clicks += 1;
        }
        Message::DangerClicked => {
            eprintln!("[EVENT] Danger button clicked");
            state.clicks += 1;
        }
        Message::BaseClicked => {
            eprintln!("[EVENT] Base button clicked");
            state.clicks += 1;
        }
        Message::Nested1 => {
            eprintln!("[EVENT] Nested button 1 clicked");
            state.clicks += 1;
        }
        Message::Nested2 => {
            eprintln!("[EVENT] Nested button 2 clicked");
            state.clicks += 1;
        }
        Message::NestedDanger => {
            eprintln!("[EVENT] Nested danger clicked");
            state.clicks += 1;
        }
        Message::Increment => {
            state.count += 1;
            eprintln!("[EVENT] Count: {}", state.count);
        }
        Message::Decrement => {
            state.count -= 1;
            eprintln!("[EVENT] Count: {}", state.count);
        }
        Message::Reset => {
            state.count = 0;
            eprintln!("[EVENT] Count reset");
        }
        Message::DynamicClick => {
            state.clicks += 1;
            eprintln!("[EVENT] Dynamic click: {}", state.clicks);
        }
        Message::DynamicDanger => {
            state.clicks += 1;
            eprintln!("[EVENT] Dynamic danger: {}", state.clicks);
        }
    }
    Task::none()
}

fn render_node<'a>(
    node: &'a WidgetNode,
    count: i32,
    clicks: i32,
    theme_manager: &ThemeManager,
    style_cascade: &StyleCascade,
) -> Element<'a, Message> {
    use gravity_iced::style_mapping::{map_color, map_length, map_padding, map_style_properties};
    
    match node.kind {
        WidgetKind::Text => {
            let value = match node.attributes.get("value") {
                Some(gravity_core::AttributeValue::Static(v)) => {
                    let mut result = v.clone();
                    if result.contains("{count}") {
                        result = result.replace("{count}", &count.to_string());
                    }
                    if result.contains("{clicks}") {
                        result = result.replace("{clicks}", &clicks.to_string());
                    }
                    result
                }
                _ => String::new(),
            };
            
            let mut text_widget = text(value);
            
            // Apply theme-aware styling
            if let Some(theme) = theme_manager.get_current_theme() {
                if let Some(size_attr) = node.attributes.get("size") {
                    if let gravity_core::AttributeValue::Static(size_str) = size_attr {
                        if let Ok(size) = size_str.parse::<f32>() {
                            text_widget = text_widget.size(size);
                        }
                    }
                }
                
                // Apply class-based styling
                let resolved = style_cascade.resolve(
                    node.style.as_ref(),
                    &node.classes,
                    None,
                );
                
                if let Some(color) = &resolved.color {
                    text_widget = text_widget.color(map_color(color));
                }
            }
            
            text_widget.into()
        }
        WidgetKind::Button => {
            let label = match node.attributes.get("label") {
                Some(gravity_core::AttributeValue::Static(l)) => l.clone(),
                _ => String::new(),
            };
            
            let msg = if let Some(gravity_core::AttributeValue::Static(handler)) = node.attributes.get("on_click") {
                match handler.as_str() {
                    "primary_clicked" => Message::PrimaryClicked,
                    "danger_clicked" => Message::DangerClicked,
                    "base_clicked" => Message::BaseClicked,
                    "nested1" => Message::Nested1,
                    "nested2" => Message::Nested2,
                    "nested_danger" => Message::NestedDanger,
                    "increment" => Message::Increment,
                    "decrement" => Message::Decrement,
                    "reset" => Message::Reset,
                    "dynamic_click" => Message::DynamicClick,
                    "dynamic_danger" => Message::DynamicDanger,
                    _ => Message::BaseClicked,
                }
            } else {
                Message::BaseClicked
            };
            
            // Resolve styles
            let resolved = style_cascade.resolve(
                node.style.as_ref(),
                &node.classes,
                None,
            );
            
            let mut btn = button(text(label)).on_press(msg);
            
            // Apply resolved styles
            if let Some(bg) = &resolved.background {
                use iced::widget::button::Style;
                use iced::Background;
                
                if let Background::Color(color) = map_background(bg) {
                    btn = btn.style(move |_theme, _status| Style {
                        background: Some(Background::Color(color)),
                        text_color: resolved.color.as_ref().map(|c| map_color(c)).or(Some(iced::Color::WHITE)),
                        ..Default::default()
                    });
                }
            }
            
            btn.into()
        }
        WidgetKind::Column => {
            let children: Vec<_> = node.children.iter()
                .map(|child| render_node(child, count, clicks, theme_manager, style_cascade))
                .collect();
            
            let mut col = column(children);
            
            // Apply layout
            if let Some(layout) = &node.layout {
                if let Some(padding) = &layout.padding {
                    col = col.padding(map_padding(layout));
                }
                if let Some(spacing) = layout.spacing {
                    col = col.spacing(spacing as u16);
                }
            }
            
            col.into()
        }
        WidgetKind::Row => {
            let children: Vec<_> = node.children.iter()
                .map(|child| render_node(child, count, clicks, theme_manager, style_cascade))
                .collect();
            
            let mut row_widget = row(children);
            
            // Apply layout
            if let Some(layout) = &node.layout {
                if let Some(padding) = &layout.padding {
                    row_widget = row_widget.padding(map_padding(layout));
                }
                if let Some(spacing) = layout.spacing {
                    row_widget = row_widget.spacing(spacing as u16);
                }
            }
            
            row_widget.into()
        }
        WidgetKind::Container => {
            let children: Vec<_> = node.children.iter()
                .map(|child| render_node(child, count, clicks, theme_manager, style_cascade))
                .collect();
            
            // Resolve styles
            let resolved = style_cascade.resolve(
                node.style.as_ref(),
                &node.classes,
                None,
            );
            
            // Apply to container
            let mut container_widget = container(column(children));
            
            // Apply style
            if let Some(bg) = &resolved.background {
                container_widget = container_widget.style(|_theme, _status| {
                    use iced::widget::container::Style;
                    use iced::Background;
                    
                    Style {
                        background: Some(map_background(bg)),
                        text_color: resolved.color.as_ref().map(|c| map_color(c)),
                        border: iced::Border {
                            width: resolved.border.as_ref().map(|b| b.width).unwrap_or(0.0),
                            color: resolved.border.as_ref().map(|b| map_color(&b.color)).unwrap_or(iced::Color::TRANSPARENT),
                            radius: gravity_iced::style_mapping::map_border_radius(&resolved.border.as_ref().map(|b| b.radius.clone()).unwrap_or_default()),
                        },
                        shadow: if let Some(shadow) = &resolved.shadow {
                            iced::Shadow {
                                color: map_color(&shadow.color),
                                offset: iced::Vector::new(shadow.offset_x, shadow.offset_y),
                                blur_radius: shadow.blur_radius,
                            }
                        } else {
                            iced::Shadow::default()
                        },
                    }
                });
            }
            
            // Apply layout
            if let Some(layout) = &node.layout {
                if let Some(padding) = &layout.padding {
                    container_widget = container_widget.padding(map_padding(layout));
                }
                if let Some(width) = &layout.width {
                    container_widget = container_widget.width(map_length(width));
                }
                if let Some(height) = &layout.height {
                    container_widget = container_widget.height(map_length(height));
                }
            }
            
            container_widget.into()
        }
        _ => column(Vec::new()).into(),
    }
}

fn view(state: &AppState) -> Element<Message> {
    render_node(
        &state.document.root,
        state.count,
        state.clicks,
        &state.theme_manager,
        &state.style_cascade,
    )
}

pub fn main() -> iced::Result {
    let state = AppState::new();
    let initial_theme = state.get_current_theme();
    
    println!("=== Style Classes Demo ===");
    println!("Features:");
    println!("- Reusable style classes");
    println!("- Class inheritance (extends)");
    println!("- State variants (hover, active)");
    println!("- Multiple classes per widget");
    println!("- Hot-reload support");
    println!("");
    println!("Try hovering over buttons to see state changes!");
    println!("Edit ui/main.gravity to see hot-reload in action!");
    println!("");
    
    iced::application(
        || state,
        update,
        view,
    )
    .theme(|_state| initial_theme)
    .run()
}

// Helper functions from style_mapping (needed for standalone example)
mod style_mapping {
    use gravity_core::ir::style::{Background, Color};
    
    pub fn map_color(color: &Color) -> iced::Color {
        iced::Color {
            r: color.r,
            g: color.g,
            b: color.b,
            a: color.a,
        }
    }
    
    pub fn map_background(background: &Background) -> iced::Background {
        match background {
            Background::Color(color) => iced::Background::Color(map_color(color)),
            Background::Gradient(_) => iced::Background::Color(iced::Color::WHITE),
            Background::Image { .. } => iced::Background::Color(iced::Color::TRANSPARENT),
        }
    }
    
    pub fn map_padding(layout: &gravity_core::ir::layout::LayoutConstraints) -> iced::Padding {
        if let Some(padding) = &layout.padding {
            iced::Padding::from(padding.top as u16)
        } else {
            iced::Padding::new(0)
        }
    }
    
    pub fn map_length(length: &gravity_core::ir::layout::Length) -> iced::Length {
        match length {
            gravity_core::ir::layout::Length::Fixed(v) => iced::Length::Fixed(*v),
            gravity_core::ir::layout::Length::Fill => iced::Length::Fill,
            gravity_core::ir::layout::Length::Shrink => iced::Length::Shrink,
            gravity_core::ir::layout::Length::FillPortion(p) => iced::Length::FillPortion(*p as u16),
        }
    }
    
    pub fn map_border_radius(radius: &gravity_core::ir::style::BorderRadius) -> iced::BorderRadius {
        iced::BorderRadius::from(radius.top_left)
    }
}
