//! Styling Example - Demonstrates layout, sizing, and styling capabilities
//!
//! This example showcases:
//! - Padding and spacing
//! - Width and height constraints
//! - Background colors and gradients
//! - Borders and shadows
//! - Opacity and transforms

use gravity_core::parse;
use gravity_iced::{IcedBackend, render};
use iced::{Application, Command, Element, Settings, Theme};

pub fn main() -> iced::Result {
    StylingExample::run(Settings {
        title: "Gravity Styling Example".to_string(),
        ..Default::default()
    })
}

#[derive(Debug, Clone)]
pub enum Message {
    Increment,
    Decrement,
    Reset,
}

pub struct StylingExample {
    count: i32,
    doc: gravity_core::ir::GravityDocument,
}

impl Application for StylingExample {
    type Message = Message;
    type Theme = Theme;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        // Example UI with styling
        let xml = r#"
<gravity>
    <themes>
        <theme name="custom">
            <palette
                primary="#3498db"
                background="#ecf0f1"
                text="#2c3e50"
            />
        </theme>
    </themes>
    
    <column 
        padding="40" 
        spacing="20"
        background="#ffffff"
        width="80%"
        max_width="600"
        align_items="center"
        theme="custom">
        
        <text 
            value="Styled Gravity App" 
            size="32"
            weight="bold"
            color="#2c3e50"
            padding="10" />
        
        <text 
            value="Count: {count}" 
            size="24"
            color="#3498db"
            background="linear-gradient(90deg, #3498db, #2ecc71)"
            padding="20 40"
            border_radius="8"
            border_width="2"
            border_color="#2980b9" />
        
        <row spacing="10">
            <button 
                label="Increment" 
                on_click="increment"
                background="#27ae60"
                color="#ffffff"
                padding="12 24"
                border_radius="4"
                width="120" />
            
            <button 
                label="Decrement" 
                on_click="decrement"
                background="#e74c3c"
                color="#ffffff"
                padding="12 24"
                border_radius="4"
                width="120" />
        </row>
        
        <button 
            label="Reset" 
            on_click="reset"
            background="transparent"
            color="#3498db"
            border_width="2"
            border_color="#3498db"
            padding="12 24"
            border_radius="4"
            hover:background="rgba(52, 152, 219, 0.1)"
            shadow="0 2 4 #00000020" />
        
        <container 
            background="#f8f9fa"
            padding="20"
            border_radius="8"
            width="fill"
            opacity="0.9">
            <text 
                value="This container has a subtle background and rounded corners" 
                color="#6c757d"
                size="14" />
        </container>
    </column>
</gravity>
        "#;

        let doc = parse(xml).expect("Failed to parse UI");

        (
            Self { count: 0, doc },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        format!("Gravity Styling - Count: {}", self.count)
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Increment => self.count += 1,
            Message::Decrement => self.count -= 1,
            Message::Reset => self.count = 0,
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let backend = IcedBackend::new(|handler, _param| {
            // Map handler strings to messages
            match handler.as_str() {
                "increment" => Box::new(Message::Increment),
                "decrement" => Box::new(Message::Decrement),
                "reset" => Box::new(Message::Reset),
                _ => Box::new(Message::Reset),
            }
        });

        // Note: In a full implementation, you would:
        // 1. Apply theme from document
        // 2. Resolve style cascade
        // 3. Pass model state for binding evaluation
        // 4. Handle state-based styling
        
        // For now, render the basic structure
        render(&self.doc.root, &backend)
    }

    fn theme(&self) -> Theme {
        Theme::Light
    }
}
