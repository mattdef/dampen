use crate::canvas::custom::AnyState;
use crate::canvas::events::{CanvasEventHandlers, create_canvas_event};
use crate::canvas::shapes::{
    CanvasShape, CircleShape, GroupShape, LineShape, RectShape, TextShape, Transform,
};
use iced::widget::canvas::{self, Cache, Event, Frame, Geometry, Path, Stroke};
use iced::{Color, Point, Rectangle, Renderer, Size, Theme, Vector, mouse};
use std::cell::RefCell;

/// The content of a canvas, which can be either a list of declarative shapes
/// or a custom drawing program.
#[derive(Debug)]
pub enum CanvasContent<M> {
    /// A set of shapes defined declaratively in XML.
    Declarative(DeclarativeProgram<M>),
    /// A custom implementation of [`DampenCanvasProgram`].
    Custom(std::sync::Arc<dyn crate::canvas::custom::DampenCanvasProgram<()>>),
}

/// A canvas program that renders a list of static or bound shapes.
#[derive(Debug)]
pub struct DeclarativeProgram<M> {
    shapes: Vec<CanvasShape>,
    event_handlers: Option<CanvasEventHandlers<M>>,
}

/// The state of a canvas program.
#[derive(Debug)]
pub enum CanvasState {
    /// State for a declarative program.
    Declarative(DeclarativeState),
    /// State for a custom program.
    Custom(AnyState),
}

impl Default for CanvasState {
    fn default() -> Self {
        CanvasState::Declarative(DeclarativeState::default())
    }
}

/// The internal state of a declarative canvas program, including its render cache.
#[derive(Debug, Default)]
pub struct DeclarativeState {
    cache: RefCell<Cache>,
    last_shapes: RefCell<Vec<CanvasShape>>,
    // Interaction state
    is_dragging: bool,
    last_position: Option<Point>,
}

impl<M> DeclarativeProgram<M> {
    /// Creates a new [`DeclarativeProgram`] with the given shapes.
    pub fn new(shapes: Vec<CanvasShape>) -> Self {
        Self {
            shapes,
            event_handlers: None,
        }
    }

    /// Adds event handlers to the declarative program.
    pub fn with_handlers(mut self, handlers: CanvasEventHandlers<M>) -> Self {
        self.event_handlers = Some(handlers);
        self
    }
}

/// A wrapper that adapts [`CanvasContent`] to work with the Iced [`canvas::Program`] trait.
pub struct CanvasProgramWrapper<M> {
    content: CanvasContent<M>,
}

impl<M> CanvasProgramWrapper<M> {
    /// Creates a new [`CanvasProgramWrapper`] from the given content.
    pub fn new(content: CanvasContent<M>) -> Self {
        Self { content }
    }
}

impl<M> canvas::Program<M> for CanvasProgramWrapper<M> {
    type State = CanvasState;

    fn draw(
        &self,
        state: &Self::State,
        renderer: &Renderer,
        theme: &Theme,
        bounds: Rectangle,
        cursor: iced::mouse::Cursor,
    ) -> Vec<Geometry> {
        match &self.content {
            CanvasContent::Declarative(program) => {
                if let CanvasState::Declarative(state) = state {
                    program.draw(state, renderer, theme, bounds, cursor)
                } else {
                    // State mismatch - should not happen if initialized correctly
                    vec![]
                }
            }
            CanvasContent::Custom(program) => {
                if let CanvasState::Custom(state) = state {
                    program.draw(state, renderer, theme, bounds, cursor)
                } else {
                    // Create default state if mismatch (or first run? no, state passed by Iced)
                    // If mismatch, we can't easily recover here without mut state
                    vec![]
                }
            }
        }
    }

    fn update(
        &self,
        state: &mut Self::State,
        event: &Event,
        bounds: Rectangle,
        cursor: iced::mouse::Cursor,
    ) -> Option<canvas::Action<M>> {
        match &self.content {
            CanvasContent::Declarative(program) => {
                if let CanvasState::Declarative(state) = state {
                    program.update(state, event, bounds, cursor)
                } else {
                    // Initialize correct state if mismatched (e.g. hot reload switched program type)
                    *state = CanvasState::Declarative(DeclarativeState::default());
                    if let CanvasState::Declarative(state) = state {
                        program.update(state, event, bounds, cursor)
                    } else {
                        None
                    }
                }
            }
            CanvasContent::Custom(program) => {
                if let CanvasState::Custom(inner_state) = state {
                    program
                        .update(inner_state, event.clone(), bounds, cursor)
                        .map(|_| canvas::Action::request_redraw())
                } else {
                    // Initialize correct state
                    *state = CanvasState::Custom(program.create_state());
                    if let CanvasState::Custom(inner_state) = state {
                        program
                            .update(inner_state, event.clone(), bounds, cursor)
                            .map(|_| canvas::Action::request_redraw())
                    } else {
                        None
                    }
                }
            }
        }
    }

    fn mouse_interaction(
        &self,
        state: &Self::State,
        bounds: Rectangle,
        cursor: iced::mouse::Cursor,
    ) -> mouse::Interaction {
        match &self.content {
            CanvasContent::Declarative(program) => {
                if let CanvasState::Declarative(state) = state {
                    program.mouse_interaction(state, bounds, cursor)
                } else {
                    mouse::Interaction::default()
                }
            }
            CanvasContent::Custom(program) => {
                if let CanvasState::Custom(state) = state {
                    program.mouse_interaction(state, bounds, cursor)
                } else {
                    mouse::Interaction::default()
                }
            }
        }
    }
}

// Logic for DeclarativeProgram is now separated from trait impl to allow composition
impl<M> DeclarativeProgram<M> {
    fn draw(
        &self,
        state: &DeclarativeState,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: iced::mouse::Cursor,
    ) -> Vec<Geometry> {
        let cache = state.cache.borrow_mut();
        let mut last_shapes = state.last_shapes.borrow_mut();

        // Check if shapes changed or if cache is empty (initial draw)
        if *last_shapes != self.shapes {
            cache.clear();
            *last_shapes = self.shapes.clone();
        }

        let geometry = cache.draw(renderer, bounds.size(), |frame| {
            for shape in &self.shapes {
                draw_shape(frame, shape);
            }
        });
        vec![geometry]
    }

    fn update(
        &self,
        state: &mut DeclarativeState,
        event: &Event,
        bounds: Rectangle,
        cursor: iced::mouse::Cursor,
    ) -> Option<canvas::Action<M>> {
        let handlers = self.event_handlers.as_ref()?;
        let position = cursor.position_in(bounds)?;

        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                state.is_dragging = true;
                state.last_position = Some(position);

                if let Some(ref name) = handlers.handler_names.on_click {
                    let event = create_canvas_event(
                        dampen_core::handler::CanvasEventKind::Click,
                        position,
                        None,
                    );
                    return Some(canvas::Action::publish((handlers.msg_factory)(name, event)));
                }
            }
            Event::Mouse(mouse::Event::CursorMoved { .. }) => {
                if state.is_dragging {
                    let delta = state
                        .last_position
                        .map(|last| (position.x - last.x, position.y - last.y));
                    state.last_position = Some(position);

                    if let Some(ref name) = handlers.handler_names.on_drag {
                        let event = create_canvas_event(
                            dampen_core::handler::CanvasEventKind::Drag,
                            position,
                            delta,
                        );
                        return Some(canvas::Action::publish((handlers.msg_factory)(name, event)));
                    }
                } else if let Some(ref name) = handlers.handler_names.on_move {
                    let event = create_canvas_event(
                        dampen_core::handler::CanvasEventKind::Move,
                        position,
                        None,
                    );
                    return Some(canvas::Action::publish((handlers.msg_factory)(name, event)));
                }
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {
                state.is_dragging = false;
                state.last_position = None;

                if let Some(ref name) = handlers.handler_names.on_release {
                    let event = create_canvas_event(
                        dampen_core::handler::CanvasEventKind::Release,
                        position,
                        None,
                    );
                    return Some(canvas::Action::publish((handlers.msg_factory)(name, event)));
                }
            }
            _ => {}
        }

        None
    }

    fn mouse_interaction(
        &self,
        _state: &DeclarativeState,
        bounds: Rectangle,
        cursor: iced::mouse::Cursor,
    ) -> mouse::Interaction {
        if cursor.is_over(bounds) {
            mouse::Interaction::Crosshair
        } else {
            mouse::Interaction::default()
        }
    }
}

fn draw_shape(frame: &mut Frame, shape: &CanvasShape) {
    match shape {
        CanvasShape::Rect(rect) => draw_rect(frame, rect),
        CanvasShape::Circle(circle) => draw_circle(frame, circle),
        CanvasShape::Line(line) => draw_line(frame, line),
        CanvasShape::Text(text) => draw_text(frame, text),
        CanvasShape::Group(group) => draw_group(frame, group),
    }
}

fn draw_rect(frame: &mut Frame, rect: &RectShape) {
    let top_left = Point::new(rect.x, rect.y);
    let size = Size::new(rect.width, rect.height);

    let path = if rect.radius > 0.0 {
        Path::rounded_rectangle(top_left, size, rect.radius.into())
    } else {
        Path::rectangle(top_left, size)
    };

    if let Some(fill) = rect.fill {
        frame.fill(&path, fill);
    }

    if let Some(stroke_color) = rect.stroke {
        let stroke = Stroke {
            style: canvas::Style::Solid(stroke_color),
            width: rect.stroke_width,
            line_cap: canvas::LineCap::Round,
            ..Default::default()
        };
        frame.stroke(&path, stroke);
    }
}

fn draw_circle(frame: &mut Frame, circle: &CircleShape) {
    let center = Point::new(circle.cx, circle.cy);
    let path = Path::circle(center, circle.radius);

    if let Some(fill) = circle.fill {
        frame.fill(&path, fill);
    }

    if let Some(stroke_color) = circle.stroke {
        let stroke = Stroke {
            style: canvas::Style::Solid(stroke_color),
            width: circle.stroke_width,
            line_cap: canvas::LineCap::Round,
            ..Default::default()
        };
        frame.stroke(&path, stroke);
    }
}

fn draw_line(frame: &mut Frame, line: &LineShape) {
    let start = Point::new(line.x1, line.y1);
    let end = Point::new(line.x2, line.y2);
    let path = Path::line(start, end);

    if let Some(stroke_color) = line.stroke {
        let stroke = Stroke {
            style: canvas::Style::Solid(stroke_color),
            width: line.stroke_width,
            line_cap: canvas::LineCap::Round,
            ..Default::default()
        };
        frame.stroke(&path, stroke);
    }
}

fn draw_text(frame: &mut Frame, text: &TextShape) {
    let position = Point::new(text.x, text.y);
    let content = canvas::Text {
        content: text.content.clone(),
        position,
        color: text.color.unwrap_or(Color::BLACK),
        size: text.size.into(),
        ..Default::default()
    };
    frame.fill_text(content);
}

fn draw_group(frame: &mut Frame, group: &GroupShape) {
    match &group.transform {
        Some(Transform::Translate(x, y)) => {
            frame.with_save(|frame| {
                frame.translate(Vector::new(*x, *y));
                for child in &group.children {
                    draw_shape(frame, child);
                }
            });
        }
        Some(Transform::Rotate(angle)) => {
            frame.with_save(|frame| {
                frame.rotate(*angle);
                for child in &group.children {
                    draw_shape(frame, child);
                }
            });
        }
        Some(Transform::Scale(factor)) => {
            frame.with_save(|frame| {
                frame.scale(*factor);
                for child in &group.children {
                    draw_shape(frame, child);
                }
            });
        }
        Some(Transform::ScaleXY(x, y)) => {
            frame.with_save(|frame| {
                frame.scale_nonuniform(Vector::new(*x, *y));
                for child in &group.children {
                    draw_shape(frame, child);
                }
            });
        }
        Some(Transform::Matrix(_)) => {
            // TODO: Implement matrix transform
            for child in &group.children {
                draw_shape(frame, child);
            }
        }
        None => {
            for child in &group.children {
                draw_shape(frame, child);
            }
        }
    }
}
