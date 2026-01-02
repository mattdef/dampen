use gravity_core::{parse, AttributeValue, WidgetKind, WidgetNode};
use iced::widget::{button, column, row, text};
use iced::{Element, Task};

#[derive(Clone, Debug)]
pub enum Message {
    Increment,
    Decrement,
    Reset,
}

pub struct AppState {
    document: gravity_core::GravityDocument,
    count: i32,
}

impl AppState {
    fn new() -> Self {
        let ui_path = std::path::PathBuf::from("examples/styling/ui/main.gravity");
        let xml = match std::fs::read_to_string(&ui_path) {
            Ok(content) => content,
            Err(e) => {
                eprintln!("Error: Failed to read UI file: {}", e);
                r#"<column padding="40" spacing="20">
                    <text value="Error: Could not load ui/main.gravity" size="18" />
                </column>"#
                    .to_string()
            }
        };

        let document = parse(&xml).unwrap_or_else(|e| {
            eprintln!("Error: Failed to parse UI: {}", e);
            gravity_core::GravityDocument::default()
        });

        Self { document, count: 0 }
    }
}

fn update(state: &mut AppState, message: Message) -> Task<Message> {
    match message {
        Message::Increment => state.count += 1,
        Message::Decrement => state.count -= 1,
        Message::Reset => state.count = 0,
    }
    Task::none()
}

fn render_node<'a>(node: &'a WidgetNode, count: i32) -> Element<'a, Message> {
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
            text(value).into()
        }
        WidgetKind::Button => {
            let label = match node.attributes.get("label") {
                Some(AttributeValue::Static(l)) => l.clone(),
                _ => String::new(),
            };
            let msg = if let Some(AttributeValue::Static(handler)) = node.attributes.get("on_click")
            {
                match handler.as_str() {
                    "increment" => Message::Increment,
                    "decrement" => Message::Decrement,
                    "reset" => Message::Reset,
                    _ => Message::Reset,
                }
            } else {
                Message::Reset
            };
            button(text(label)).on_press(msg).into()
        }
        WidgetKind::Column => {
            let children: Vec<_> = node
                .children
                .iter()
                .map(|child| render_node(child, count))
                .collect();
            column(children).into()
        }
        WidgetKind::Row => {
            let children: Vec<_> = node
                .children
                .iter()
                .map(|child| render_node(child, count))
                .collect();
            row(children).into()
        }
        _ => column(Vec::new()).into(),
    }
}

fn view(state: &AppState) -> Element<'_, Message> {
    render_node(&state.document.root, state.count)
}

pub fn main() -> iced::Result {
    iced::application(AppState::new, update, view).run()
}
