use dampen_core::AppState;
use dampen_core::handler::CanvasEvent;
use dampen_core::handler::HandlerRegistry;
use dampen_iced::canvas::custom::{CanvasAdapter, CustomProgramContainer};
use dampen_macros::{UiModel, dampen_ui, inventory_handlers, ui_handler};
use iced::widget::canvas::{self, Geometry, Path, Stroke};
use iced::{Color, Point, Rectangle, Renderer, Theme, Vector, mouse::Cursor};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

// Import Message and CurrentView from the parent module
use crate::{CurrentView, Message};

#[dampen_ui("canvas.dampen")]
mod _app {}

#[derive(UiModel, Serialize, Deserialize, Clone, Debug)]
pub struct Model {
    pub circle_x: f32,
    pub circle_y: f32,
    pub circle_radius: f32,
    pub circle_dragging: bool,
    pub rect_color: String,
    // Note: custom program is not serializable
    #[serde(skip)]
    pub clock: Option<CustomProgramContainer<()>>,
}

impl Default for Model {
    fn default() -> Self {
        let clock = Clock::default();
        let adapter = CanvasAdapter::new(clock);
        let container = CustomProgramContainer(Arc::new(adapter));

        Self {
            circle_x: 200.0,
            circle_y: 150.0,
            circle_radius: 50.0,
            circle_dragging: false,
            rect_color: "#3498db".to_string(),
            clock: Some(container),
        }
    }
}

pub fn create_app_state() -> AppState<Model> {
    let document = _app::document();
    let handler_registry = create_handler_registry();

    AppState::with_all(document, Model::default(), handler_registry)
}

#[ui_handler]
pub fn on_canvas_click(model: &mut Model, event: CanvasEvent) {
    let dx = event.x - model.circle_x;
    let dy = event.y - model.circle_y;
    let distance_sq = dx * dx + dy * dy;

    if distance_sq <= model.circle_radius * model.circle_radius {
        model.circle_dragging = true;
        model.circle_x = event.x;
        model.circle_y = event.y;
    }
}

#[ui_handler]
pub fn on_canvas_drag(model: &mut Model, event: CanvasEvent) {
    if model.circle_dragging {
        model.circle_x = event.x;
        model.circle_y = event.y;
    }
}

#[ui_handler]
pub fn on_canvas_release(model: &mut Model, _event: CanvasEvent) {
    model.circle_dragging = false;
}

#[ui_handler]
pub fn change_color(model: &mut Model) {
    if model.rect_color == "#3498db" {
        model.rect_color = "#34db7d".to_string();
    } else {
        model.rect_color = "#3498db".to_string();
    }
}

inventory_handlers! { change_color, on_canvas_click, on_canvas_drag, on_canvas_release }

pub fn create_handler_registry() -> HandlerRegistry {
    let registry = HandlerRegistry::new();

    registry.register_with_command("switch_to_window", |_model: &mut dyn std::any::Any| {
        Box::new(iced::Task::done(Message::SwitchToView(CurrentView::Window)))
    });

    registry.register_simple("change_color", |model: &mut dyn std::any::Any| {
        if let Some(m) = model.downcast_mut::<Model>() {
            change_color(m);
        }
    });

    registry.register_with_value(
        "on_canvas_click",
        |model: &mut dyn std::any::Any, val: Box<dyn std::any::Any>| {
            if let Some(m) = model.downcast_mut::<Model>()
                && let Some(json) = val.downcast_ref::<String>()
                && let Ok(event) = serde_json::from_str::<CanvasEvent>(json)
            {
                on_canvas_click(m, event);
            }
        },
    );
    registry.register_with_value(
        "on_canvas_drag",
        |model: &mut dyn std::any::Any, val: Box<dyn std::any::Any>| {
            if let Some(m) = model.downcast_mut::<Model>()
                && let Some(json) = val.downcast_ref::<String>()
                && let Ok(event) = serde_json::from_str::<CanvasEvent>(json)
            {
                on_canvas_drag(m, event);
            }
        },
    );
    registry.register_with_value(
        "on_canvas_release",
        |model: &mut dyn std::any::Any, val: Box<dyn std::any::Any>| {
            if let Some(m) = model.downcast_mut::<Model>()
                && let Some(json) = val.downcast_ref::<String>()
                && let Ok(event) = serde_json::from_str::<CanvasEvent>(json)
            {
                on_canvas_release(m, event);
            }
        },
    );
    registry
}

// Custom Clock Program
#[derive(Debug, Default)]
struct Clock {
    cache: std::sync::Mutex<canvas::Cache>,
}

impl<M> canvas::Program<M> for Clock {
    type State = ();

    fn update(
        &self,
        _state: &mut Self::State,
        _event: &iced::Event,
        _bounds: Rectangle,
        _cursor: Cursor,
    ) -> Option<canvas::Action<M>> {
        None
    }

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: Cursor,
    ) -> Vec<Geometry> {
        let cache = self.cache.lock().unwrap();
        let geometry = cache.draw(renderer, bounds.size(), |frame| {
            let center = frame.center();
            let radius = frame.width().min(frame.height()) / 2.0;

            frame.translate(Vector::new(center.x, center.y));

            let background = Path::circle(Point::ORIGIN, radius);
            frame.fill(&background, Color::from_rgb8(0x12, 0x93, 0xD8));

            let hour_hand = Path::line(Point::ORIGIN, Point::new(0.0, -0.5 * radius));
            frame.stroke(
                &hour_hand,
                Stroke::default()
                    .with_width(4.0)
                    .with_color(Color::WHITE)
                    .with_line_cap(canvas::LineCap::Round),
            );
        });

        vec![geometry]
    }
}
