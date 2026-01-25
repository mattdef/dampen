//! The canvas module provides a powerful drawing widget for Dampen.
//!
//! It supports two primary modes of operation:
//! 1. **Declarative**: Shapes defined in XML that update automatically when the model changes.
//! 2. **Custom**: A bridge to the standard Iced [`canvas::Program`](iced::widget::canvas::Program) for complex, manual drawing.

pub mod custom;
pub mod events;
pub mod program;
pub mod shapes;

pub use custom::*;
pub use events::*;
pub use program::{CanvasContent, CanvasProgramWrapper, DeclarativeProgram};
pub use shapes::*;
