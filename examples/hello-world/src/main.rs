use gravity_core::{parse, WidgetNode, WidgetKind, AttributeValue};
use iced::widget::{column, text, button};
use iced::{Element, Task};

/// Messages for the application
#[derive(Clone, Debug)]
enum Message {
    Greet,
}

/// Application state
struct AppState {
    document: gravity_core::GravityDocument,
}

impl AppState {
    fn new() -> Self {
        // Load and parse the XML file
        let xml = include_str!("../ui/main.gravity");
        let document = parse(xml).expect("Failed to parse XML");
        
        Self { document }
    }
}

/// Update function
fn update(_state: &mut AppState, message: Message) -> Task<Message> {
    match message {
        Message::Greet => {
            println!("Button clicked!");
        }
    }
    Task::none()
}

/// Helper to render a widget node
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
            button(text(label))
                .on_press(Message::Greet)
                .into()
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
            iced::widget::row(children).into()
        }
        _ => column(Vec::new()).into(),
    }
}

/// View function
fn view(state: &AppState) -> Element<Message> {
    render_node(&state.document.root)
}

pub fn main() -> iced::Result {
    iced::application(AppState::new, update, view).run()
}
