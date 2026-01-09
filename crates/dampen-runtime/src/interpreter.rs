//! Event dispatch and interpreter for Dampen applications

use dampen_core::{EventBinding, HandlerRegistry};
use std::any::Any;

/// Interpreter that dispatches events to handlers
#[derive(Clone)]
pub struct Interpreter {
    handler_registry: HandlerRegistry,
}

impl Interpreter {
    /// Create a new interpreter with a handler registry
    pub fn new(handler_registry: HandlerRegistry) -> Self {
        Self { handler_registry }
    }

    /// Dispatch an event to its handler
    pub fn dispatch(
        &self,
        event: &EventBinding,
        model: &mut dyn Any,
        value: Option<Box<dyn Any>>,
    ) -> Result<Option<Box<dyn Any>>, DispatchError> {
        let handler = self
            .handler_registry
            .get(&event.handler)
            .ok_or_else(|| DispatchError::HandlerNotFound(event.handler.clone()))?;

        match (handler, value) {
            (dampen_core::HandlerEntry::Simple(handler), None) => {
                handler(model);
                Ok(None)
            }
            (dampen_core::HandlerEntry::WithValue(handler), Some(value)) => {
                handler(model, value);
                Ok(None)
            }
            (dampen_core::HandlerEntry::WithCommand(handler), None) => {
                let command = handler(model);
                Ok(Some(command))
            }
            _ => Err(DispatchError::SignatureMismatch(event.handler.clone())),
        }
    }
}

/// Errors that can occur during event dispatch
#[derive(Debug, thiserror::Error)]
pub enum DispatchError {
    #[error("Handler '{0}' not found")]
    HandlerNotFound(String),

    #[error("Handler '{0}' signature mismatch")]
    SignatureMismatch(String),

    #[error("Handler '{0}' requires a value parameter but none was provided")]
    MissingValue(String),

    #[error("Handler '{0}' does not accept a value parameter")]
    UnexpectedValue(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    use dampen_core::HandlerRegistry;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize, Default)]
    struct TestModel {
        count: i32,
    }

    #[test]
    fn test_interpreter_dispatch() {
        let registry = HandlerRegistry::new();
        registry.register_simple("increment", |model: &mut dyn Any| {
            let model = model.downcast_mut::<TestModel>().unwrap();
            model.count += 1;
        });

        let interpreter = Interpreter::new(registry);
        let mut model = TestModel::default();
        let event = EventBinding {
            event: dampen_core::EventKind::Click,
            handler: "increment".to_string(),
            param: None,
            span: dampen_core::Span::new(0, 0, 1, 1),
        };

        let result = interpreter.dispatch(&event, &mut model, None);
        assert!(result.is_ok());
        assert_eq!(model.count, 1);
    }
}
