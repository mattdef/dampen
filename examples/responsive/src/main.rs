use gravity_core::{parse, AttributeValue, WidgetKind, WidgetNode};
use gravity_iced::style_mapping::{map_length, map_padding};
use gravity_runtime::resolve_tree_breakpoint_attributes;
use iced::widget::{button, column, container, row, text};
use iced::{window, Element, Padding, Subscription, Task};

#[derive(Clone, Debug)]
pub enum Message {
    Left,
    Center,
    Right,
    WindowResized(f32),
}

pub struct AppState {
    document: gravity_core::GravityDocument,
    viewport_width: f32,
    resolved_document: Option<gravity_core::GravityDocument>,
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
                </column>"#
                    .to_string()
            }
        };

        let document = parse(&xml).unwrap_or_else(|e| {
            eprintln!("Error: Failed to parse UI: {}", e);
            gravity_core::GravityDocument::default()
        });

        Self {
            document,
            viewport_width: 800.0, // Default window size
            resolved_document: None,
        }
    }
}

fn update(state: &mut AppState, message: Message) -> Task<Message> {
    match message {
        Message::Left => println!("Left button clicked"),
        Message::Center => println!("Center button clicked"),
        Message::Right => println!("Right button clicked"),
        Message::WindowResized(width) => {
            state.viewport_width = width;
            // Update resolved document
            let resolved = resolve_tree_breakpoint_attributes(&state.document.root, width);
            let mut new_doc = state.document.clone();
            new_doc.root = resolved;
            state.resolved_document = Some(new_doc);
            println!("Window resized to {}px", width);
        }
    }
    // Initialize resolved document on first update
    if state.resolved_document.is_none() {
        let resolved =
            resolve_tree_breakpoint_attributes(&state.document.root, state.viewport_width);
        let mut new_doc = state.document.clone();
        new_doc.root = resolved;
        state.resolved_document = Some(new_doc);
    }
    Task::none()
}

fn subscription(_state: &AppState) -> Subscription<Message> {
    // Subscribe to window resize events
    window::resize_events().map(|(_id, size)| Message::WindowResized(size.width))
}

fn render_node<'a>(node: &'a WidgetNode) -> Element<'a, Message> {
    // Get layout from structured field, fallback to attributes for backward compatibility
    let width = node
        .layout
        .as_ref()
        .and_then(|l| l.width.as_ref())
        .map(|w| map_length(&Some(w.clone())))
        .unwrap_or(iced::Length::Shrink);

    let height = node
        .layout
        .as_ref()
        .and_then(|l| l.height.as_ref())
        .map(|h| map_length(&Some(h.clone())))
        .unwrap_or(iced::Length::Shrink);

    let padding = node
        .layout
        .as_ref()
        .map(|l| map_padding(l))
        .unwrap_or(Padding::new(0.0));

    match node.kind {
        WidgetKind::Text => {
            let value = match node.attributes.get("value") {
                Some(AttributeValue::Static(v)) => v.clone(),
                _ => String::new(),
            };
            let text_widget = text(value).width(width).height(height);
            container(text_widget).padding(padding).into()
        }
        WidgetKind::Button => {
            let label = match node.attributes.get("label") {
                Some(AttributeValue::Static(l)) => l.clone(),
                _ => String::new(),
            };
            let msg = if let Some(AttributeValue::Static(handler)) = node.attributes.get("on_click")
            {
                match handler.as_str() {
                    "left" => Message::Left,
                    "center" => Message::Center,
                    "right" => Message::Right,
                    _ => Message::Center,
                }
            } else {
                Message::Center
            };
            let btn = button(text(label))
                .on_press(msg)
                .width(width)
                .height(height);
            container(btn).padding(padding).into()
        }
        WidgetKind::Column => {
            let spacing = node.layout.as_ref().and_then(|l| l.spacing).unwrap_or(0.0);

            let children: Vec<_> = node
                .children
                .iter()
                .map(|child| render_node(child))
                .collect();

            let col = column(children)
                .spacing(spacing as f32)
                .width(width)
                .height(height);

            container(col).padding(padding).into()
        }
        WidgetKind::Row => {
            let spacing = node.layout.as_ref().and_then(|l| l.spacing).unwrap_or(0.0);

            let children: Vec<_> = node
                .children
                .iter()
                .map(|child| render_node(child))
                .collect();

            let row_widget = row(children)
                .spacing(spacing as f32)
                .width(width)
                .height(height);

            container(row_widget).padding(padding).into()
        }
        WidgetKind::Container => {
            let children: Vec<_> = node
                .children
                .iter()
                .map(|child| render_node(child))
                .collect();

            let mut container_widget = if children.is_empty() {
                container(text(""))
            } else {
                container(children.into_iter().next().unwrap())
            };

            container_widget = container_widget
                .width(width)
                .height(height)
                .padding(padding);

            // Apply style if present
            if let Some(style) = &node.style {
                use gravity_iced::style_mapping::map_style_properties;
                let iced_style = map_style_properties(style);
                container_widget = container_widget.style(move |_theme| iced_style);
            }

            container_widget.into()
        }
        _ => column(Vec::new()).into(),
    }
}

fn view(state: &AppState) -> Element<'_, Message> {
    // Use the resolved document
    if let Some(ref doc) = state.resolved_document {
        render_node(&doc.root)
    } else {
        // Fallback to unresolved
        render_node(&state.document.root)
    }
}

pub fn main() -> iced::Result {
    iced::application(AppState::new, update, view)
        .subscription(subscription)
        .run()
}
