//! Responsive Example - Demonstrates responsive layout with breakpoints
//!
//! This example showcases:
//! - Mobile/tablet/desktop breakpoints
//! - Responsive spacing and padding
//! - Adaptive layout direction
//! - Viewport-based sizing

use gravity_core::parse;
use gravity_iced::{IcedBackend, render};
use iced::{Application, Command, Element, Settings, Theme};

pub fn main() -> iced::Result {
    ResponsiveExample::run(Settings {
        title: "Gravity Responsive Example".to_string(),
        ..Default::default()
    })
}

#[derive(Debug, Clone)]
pub enum Message {
    WindowResized { width: u32, height: u32 },
}

pub struct ResponsiveExample {
    viewport_width: u32,
    doc: gravity_core::ir::GravityDocument,
}

impl Application for ResponsiveExample {
    type Message = Message;
    type Theme = Theme;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        // Example UI with responsive attributes
        let xml = r#"
<gravity>
    <column 
        padding="20"
        mobile:padding="16"
        tablet:padding="24"
        desktop:padding="32"
        
        spacing="10"
        mobile:spacing="8"
        tablet:spacing="12"
        desktop:spacing="20"
        
        align_items="center">
        
        <text 
            value="Resize the window to see responsive changes!" 
            size="18"
            mobile:size="14"
            desktop:size="24"
            color="#2c3e50"
            weight="bold" />
        
        <row 
            mobile:direction="vertical"
            desktop:direction="horizontal"
            spacing="10"
            mobile:spacing="8"
            desktop:spacing="20">
            
            <container 
                background="#3498db"
                padding="20"
                mobile:padding="12"
                desktop:padding="32"
                width="fill"
                mobile:width="fill"
                desktop:width="fill_portion(2)"
                border_radius="8">
                <text 
                    value="Flexible Box 1" 
                    color="#ffffff"
                    mobile:size="12"
                    desktop:size="16" />
            </container>
            
            <container 
                background="#2ecc71"
                padding="20"
                mobile:padding="12"
                desktop:padding="32"
                width="fill"
                mobile:width="fill"
                desktop:width="fill_portion(1)"
                border_radius="8">
                <text 
                    value="Flexible Box 2" 
                    color="#ffffff"
                    mobile:size="12"
                    desktop:size="16" />
            </container>
        </row>
        
        <container 
            background="#f8f9fa"
            padding="16"
            mobile:padding="8"
            desktop:padding="24"
            width="80%"
            max_width="800"
            border_radius="4"
            border_width="1"
            border_color="#dee2e6">
            <text 
                value="This container adapts its size and spacing based on viewport width" 
                color="#495057"
                mobile:size="12"
                tablet:size="14"
                desktop:size="16" />
        </container>
        
        <column 
            mobile:spacing="4"
            tablet:spacing="8"
            desktop:spacing="12"
            align_items="start">
            <text value="Breakpoint Demo:" color="#6c757d" size="14" />
            <text value="• Mobile (< 640px): Compact layout" color="#28a745" size="12" />
            <text value="• Tablet (640-1024px): Medium spacing" color="#ffc107" size="12" />
            <text value="• Desktop (≥ 1024px): Spacious layout" color="#007bff" size="12" />
        </column>
    </column>
</gravity>
        "#;

        let doc = parse(xml).expect("Failed to parse UI");

        (
            Self {
                viewport_width: 800, // Initial width
                doc,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        format!("Responsive Example - {}px", self.viewport_width)
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::WindowResized { width, .. } => {
                self.viewport_width = width;
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let backend = IcedBackend::new(|_handler, _param| {
            // In a full implementation, this would handle events
            Box::new(Message::WindowResized { width: 0, height: 0 })
        });

        // Note: In a full implementation:
        // 1. Determine current breakpoint from viewport_width
        // 2. Resolve breakpoint-specific attributes
        // 3. Apply responsive styles
        
        render(&self.doc.root, &backend)
    }

    fn theme(&self) -> Theme {
        Theme::Light
    }
    
    fn subscription(&self) -> iced::Subscription<Message> {
        // In a full implementation, subscribe to window resize events
        iced::Subscription::none()
    }
}
