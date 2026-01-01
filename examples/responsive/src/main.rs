use gravity_core::{parse, WidgetNode, WidgetKind, AttributeValue};
use iced::widget::{column, text, button, row};
use iced::{Element, Task};

#[derive(Clone, Debug)]
pub enum Message {
    Left,
    Center,
    Right,
}

pub struct AppState {
    document: gravity_core::GravityDocument,
}

impl AppState {
    fn new() -> Self {
        let ui_path = std::path::PathBuf::from("examples/responsive/ui/main.gravity");
        let xml = match std::fs::read_to_string(&ui_path) {
            Ok(content) => content,
            Err(e) => {
                eprintln!("Error: Failed to read UI file: {}", e);
                r#"<column padding="40" spacing="20">
                    <text value="Error: Could not load ui/main.gravity" size="18" />
                </column>"#.to_string()
            }
        };

        let document = parse(&xml).unwrap_or_else(|e| {
            eprintln!("Error: Failed to parse UI: {}", e);
            gravity_core::GravityDocument::default()
        });

        Self { document }
    }
}

fn update(state: &mut AppState, message: Message) -> Task<Message> {
    match message {
        Message::Left => println!("Left"),
        Message::Center => println!("Center"),
        Message::Right => println!("Right"),
    }
    Task::none()
}

fn render_node<'a>(node: &'a WidgetNode) -> Element<'a, Message> {
    match node.kind {
        WidgetKind::Text => {
            let value = match node.attributes.get("value") {
                Some(AttributeValue::Static(v)) => v.clone(),
                _ => String::new(),
            };
            text(value).into()
        }
        WidgetKind::Button => {
            let label = match node.attributes.get("label") {
                Some(AttributeValue::Static(l)) => l.clone(),
                _ => String::new(),
            };
            let msg = if let Some(AttributeValue::Static(handler)) = node.attributes.get("on_click") {
                match handler.as_str() {
                    "left" => Message::Left,
                    "center" => Message::Center,
                    "right" => Message::Right,
                    _ => Message::Center,
                }
            } else {
                Message::Center
            };
            button(text(label)).on_press(msg).into()
        }
        WidgetKind::Column => {
            let children: Vec<_> = node.children.iter()
                .map(|child| render_node(child))
                .collect();
            column(children).into()
        }
        WidgetKind::Row => {
            let children: Vec<_> = node.children.iter()
                .map(|child| render_node(child))
                .collect();
            row(children).into()
        }
        WidgetKind::Container => {
            let children: Vec<_> = node.children.iter()
                .map(|child| render_node(child))
                .collect();
            if children.is_empty() {
                text("").into()
            } else {
                children.into_iter().next().unwrap()
            }
        }
        _ => column(Vec::new()).into(),
    }
}

fn view(state: &AppState) -> Element<Message> {
    render_node(&state.document.root)
}

pub fn main() -> iced::Result {
    iced::application(AppState::new, update, view).run()
}
