use gravity_core::{parse, HandlerRegistry};
use gravity_iced::{GravityWidgetBuilder, HandlerMessage};
use gravity_macros::UiModel;
use iced::{Element, Task};
use serde::{Deserialize, Serialize};

/// T079: Simple Canvas Program implementation for demonstration
///
/// Note: This is a placeholder implementation since canvas::Program
/// cannot be directly bound from XML yet. In a real application,
/// you would implement canvas::Program trait for custom visualizations.
struct SimpleChartProgram;

impl iced::widget::canvas::Program<HandlerMessage> for SimpleChartProgram {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &iced::Renderer,
        _theme: &iced::Theme,
        bounds: iced::Rectangle,
        _cursor: iced::mouse::Cursor,
    ) -> Vec<iced::widget::canvas::Geometry> {
        use iced::widget::canvas::{Cache, Frame, Path, Stroke};
        use iced::Color;

        let mut cache = Cache::new();

        let geometry = cache.draw(renderer, bounds.size(), |frame: &mut Frame| {
            // Draw a simple bar chart visualization
            let width = bounds.width;
            let height = bounds.height;

            // Background
            frame.fill_rectangle(
                iced::Point::ORIGIN,
                iced::Size::new(width, height),
                Color::from_rgb(0.98, 0.98, 0.98),
            );

            // Draw 5 bars
            let bar_width = width / 6.0;
            let values = [0.3, 0.6, 0.8, 0.5, 0.7];

            for (i, &value) in values.iter().enumerate() {
                let x = bar_width * (i as f32 + 0.5);
                let bar_height = height * value * 0.8;
                let y = height - bar_height - 20.0;

                // Bar
                frame.fill_rectangle(
                    iced::Point::new(x, y),
                    iced::Size::new(bar_width * 0.8, bar_height),
                    Color::from_rgb(0.3, 0.6, 0.9),
                );

                // Border
                let path = Path::rectangle(
                    iced::Point::new(x, y),
                    iced::Size::new(bar_width * 0.8, bar_height),
                );
                frame.stroke(
                    &path,
                    Stroke::default()
                        .with_width(2.0)
                        .with_color(Color::from_rgb(0.2, 0.4, 0.7)),
                );
            }

            // Title
            frame.fill_text(iced::widget::canvas::Text {
                content: "Sample Chart".to_string(),
                position: iced::Point::new(width / 2.0, 10.0),
                color: Color::from_rgb(0.2, 0.2, 0.2),
                size: 16.0.into(),
                ..Default::default()
            });
        });

        vec![geometry]
    }
}

/// Application model with canvas program
#[derive(Default, UiModel, Serialize, Deserialize, Clone, Debug)]
struct Model {
    // Canvas programs would be stored here
    // For now, they're created in view() due to binding limitations
    #[serde(skip)]
    #[ui_skip]
    pub _placeholder: String,
}

/// Messages for application (using HandlerMessage from gravity-iced)
type Message = HandlerMessage;

/// Application state
struct AppState {
    model: Model,
    document: gravity_core::GravityDocument,
    handler_registry: HandlerRegistry,
}

impl AppState {
    fn new() -> Self {
        // Load canvas.gravity file
        let xml = std::fs::read_to_string("examples/widget-showcase/ui/canvas.gravity")
            .unwrap_or_else(|_| {
                // Fallback if file not found
                r#"<?xml version="1.0" encoding="UTF-8" ?>
<column spacing="20" padding="20">
    <text value="Canvas Widget Showcase" size="24" weight="bold" />
    <text value="Canvas file not found - using fallback" size="12" />
    <canvas width="400" height="300" program="{simple_chart}" />
</column>"#
                    .to_string()
            });

        let document = parse(&xml).expect("Failed to parse canvas.gravity");

        // Create handler registry
        let mut handler_registry = HandlerRegistry::new();

        // Register canvas_clicked handler
        handler_registry.register_simple("canvas_clicked", |_state| {
            println!("Canvas clicked!");
        });

        Self {
            model: Model::default(),
            document,
            handler_registry,
        }
    }
}

/// Update function
fn update(state: &mut AppState, message: Message) -> Task<Message> {
    match message {
        HandlerMessage::Handler(handler_name, value_opt) => {
            // Handle canvas click with coordinates if available
            if handler_name == "canvas_clicked" {
                if let Some(value) = value_opt {
                    println!("Canvas clicked at: {}", value);
                } else {
                    println!("Canvas clicked (no coordinates)");
                }
            } else {
                println!("Handler called: {}", handler_name);
            }

            Task::none()
        }
    }
}

/// View function
fn view(state: &AppState) -> Element<'_, Message> {
    GravityWidgetBuilder::new(
        &state.document.root,
        &state.model,
        Some(&state.handler_registry),
    )
    .with_verbose(true) // Enable verbose logging to see Canvas info
    .build()
}

/// Main function
pub fn main() -> iced::Result {
    iced::application(AppState::new, update, view).run()
}
