//! Canvas event handling logic.

use dampen_core::handler::CanvasEvent;
use iced::Point;

/// Handlers for canvas interaction events.
#[derive(Debug, Clone)]
pub struct CanvasEventHandlers<M> {
    /// The names of the handler functions in the model.
    pub handler_names: CanvasHandlerNames,
    /// A factory function to create messages from handler names and events.
    pub msg_factory: fn(&str, CanvasEvent) -> M,
}

/// The names of handlers for specific canvas events.
#[derive(Debug, Clone, Default)]
pub struct CanvasHandlerNames {
    /// Handler for click events.
    pub on_click: Option<String>,
    /// Handler for drag events.
    pub on_drag: Option<String>,
    /// Handler for mouse movement events.
    pub on_move: Option<String>,
    /// Handler for mouse release events.
    pub on_release: Option<String>,
}

/// Helper to construct a [`CanvasEvent`] from an Iced position and delta.
pub fn create_canvas_event(
    kind: dampen_core::handler::CanvasEventKind,
    position: Point,
    delta: Option<(f32, f32)>,
) -> CanvasEvent {
    let (delta_x, delta_y) = match delta {
        Some((dx, dy)) => (Some(dx), Some(dy)),
        None => (None, None),
    };

    CanvasEvent {
        kind,
        x: position.x,
        y: position.y,
        delta_x,
        delta_y,
    }
}
