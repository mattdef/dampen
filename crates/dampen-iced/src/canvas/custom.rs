use iced::widget::canvas::{self, Event, Geometry};
use iced::{Rectangle, Renderer, Theme, mouse::Cursor};
use std::any::Any;
use std::fmt::Debug;

/// A trait for type-erased canvas programs that can be used in Dampen.
///
/// This is similar to [`iced::widget::canvas::Program`] but with type-erased state.
pub trait DampenCanvasProgram<M>: Debug + Send + Sync {
    /// Draws the canvas onto the provided [`Renderer`].
    fn draw(
        &self,
        state: &AnyState,
        renderer: &Renderer,
        theme: &Theme,
        bounds: Rectangle,
        cursor: Cursor,
    ) -> Vec<Geometry>;

    /// Updates the canvas state based on an [`Event`].
    fn update(
        &self,
        state: &mut AnyState,
        event: Event,
        bounds: Rectangle,
        cursor: Cursor,
    ) -> Option<canvas::Action<M>>;

    /// Handles mouse interaction, returning the appropriate cursor icon.
    fn mouse_interaction(
        &self,
        state: &AnyState,
        bounds: Rectangle,
        cursor: Cursor,
    ) -> iced::mouse::Interaction;

    /// Creates the initial state for the canvas program.
    fn create_state(&self) -> Box<dyn Any>;
}

/// Type alias for type-erased state.
pub type AnyState = Box<dyn Any>;

/// A container that holds a thread-safe, shared canvas program.
///
/// This is used to transport custom programs through the [`BindingValue`](dampen_core::binding::BindingValue) system.
#[derive(Debug)]
pub struct CustomProgramContainer<M>(pub std::sync::Arc<dyn DampenCanvasProgram<M> + 'static>);

impl<M> Clone for CustomProgramContainer<M> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

/// An adapter that allows any [`iced::widget::canvas::Program`] to be used as a [`DampenCanvasProgram`].
#[derive(Debug)]
pub struct CanvasAdapter<P, M>
where
    P: canvas::Program<M> + Debug + Send + Sync + 'static,
    P::State: 'static,
    M: 'static + Debug + Send + Sync,
{
    program: P,
    _marker: std::marker::PhantomData<M>,
}

impl<P, M> CanvasAdapter<P, M>
where
    P: canvas::Program<M> + Debug + Send + Sync + 'static,
    P::State: 'static,
    M: 'static + Debug + Send + Sync,
{
    /// Creates a new [`CanvasAdapter`] wrapping the given program.
    pub fn new(program: P) -> Self {
        Self {
            program,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<P, M> DampenCanvasProgram<M> for CanvasAdapter<P, M>
where
    P: canvas::Program<M> + Debug + Send + Sync + 'static,
    P::State: 'static,
    M: 'static + Debug + Send + Sync,
{
    fn draw(
        &self,
        state: &AnyState,
        renderer: &Renderer,
        theme: &Theme,
        bounds: Rectangle,
        cursor: Cursor,
    ) -> Vec<Geometry> {
        if let Some(state) = state.downcast_ref::<P::State>() {
            self.program.draw(state, renderer, theme, bounds, cursor)
        } else {
            vec![]
        }
    }

    fn update(
        &self,
        state: &mut AnyState,
        event: Event,
        bounds: Rectangle,
        cursor: Cursor,
    ) -> Option<canvas::Action<M>> {
        if let Some(state) = state.downcast_mut::<P::State>() {
            // Check if Iced 0.14 Program::update takes &Event or Event
            // The compiler error said P::update expected &Event.
            self.program.update(state, &event, bounds, cursor)
        } else {
            None
        }
    }

    fn mouse_interaction(
        &self,
        state: &AnyState,
        bounds: Rectangle,
        cursor: Cursor,
    ) -> iced::mouse::Interaction {
        if let Some(state) = state.downcast_ref::<P::State>() {
            self.program.mouse_interaction(state, bounds, cursor)
        } else {
            iced::mouse::Interaction::default()
        }
    }

    fn create_state(&self) -> Box<dyn Any> {
        Box::new(P::State::default())
    }
}

use dampen_core::binding::{BindingValue, ToBindingValue};

// Allow CustomProgramContainer to be converted to BindingValue
impl<M> ToBindingValue for CustomProgramContainer<M>
where
    M: 'static + Send + Sync + Debug,
{
    fn to_binding_value(&self) -> BindingValue {
        // Explicitly cast to Any to avoid lifetime confusion?
        let container: CustomProgramContainer<M> = self.clone();
        let any: std::sync::Arc<dyn std::any::Any + Send + Sync> = std::sync::Arc::new(container);
        BindingValue::Custom(any)
    }
}
