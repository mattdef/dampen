use gravity_core::{parse, HandlerRegistry};
use gravity_iced::{GravityWidgetBuilder, HandlerMessage};
use gravity_macros::UiModel;
use iced::{Element, Task};
use serde::{Deserialize, Serialize};

#[derive(Default, UiModel, Serialize, Deserialize, Clone, Debug)]
struct Model {
    count: i32,
}

type Message = HandlerMessage;

struct AppState {
    model: Model,
    document: gravity_core::GravityDocument,
}

impl AppState {
    fn new() -> Self {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<gravity>
    <styles>
        <style name="card">
            <base 
                background="#ffffff"
                padding="20"
                border_radius="12"
                border_width="1"
                border_color="#e0e0e0"
                width="fill" />
        </style>
    </styles>
    
    <column padding="40" spacing="30">
        <text value="Test Container Issue" size="24" weight="bold" />
        
        <!-- Section that works -->
        <container class="card">
            <column spacing="15">
                <text value="1. This section works" size="20" weight="bold" color="#2c3e50" />
                <text value="Simple text" size="12" color="#27ae60" />
            </column>
        </container>
        
        <!-- Section that doesn't work -->
        <container class="card">
            <column spacing="15">
                <text value="2. This section should show containers below" size="20" weight="bold" color="#2c3e50" />
                
                <container background="#3498db" padding="10" border_radius="4" width="200">
                    <text value="Fixed width: 200px" size="12" color="#ffffff" />
                </container>
                
                <container background="#2ecc71" padding="10" border_radius="4" width="fill">
                    <text value="Fill width" size="12" color="#ffffff" />
                </container>
            </column>
        </container>
    </column>
</gravity>"#;

        let document = parse(xml).expect("Failed to parse XML");

        Self {
            model: Model::default(),
            document,
        }
    }
}

fn update(_state: &mut AppState, _message: Message) -> Task<Message> {
    Task::none()
}

fn view(state: &AppState) -> Element<'_, Message> {
    GravityWidgetBuilder::from_document(&state.document, &state.model, None).build()
}

pub fn main() -> iced::Result {
    iced::application(AppState::new, update, view).run()
}
