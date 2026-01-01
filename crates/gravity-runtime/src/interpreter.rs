//! Event dispatch and interpreter for Gravity applications

use gravity_core::{parse, EventBinding, GravityDocument, HandlerRegistry};
use std::any::Any;
use std::time::Instant;

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
            (gravity_core::HandlerEntry::Simple(handler), None) => {
                handler(model);
                Ok(None)
            }
            (gravity_core::HandlerEntry::WithValue(handler), Some(value)) => {
                handler(model, value);
                Ok(None)
            }
            (gravity_core::HandlerEntry::WithCommand(handler), None) => {
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

/// Hot-reload enabled interpreter with state management
#[allow(dead_code)]
pub struct HotReloadInterpreter {
    interpreter: Interpreter,
    current_document: Option<GravityDocument>,
    state_file: Option<std::path::PathBuf>,
}

impl HotReloadInterpreter {
    /// Create a new hot-reload interpreter
    pub fn new(handler_registry: HandlerRegistry) -> Self {
        Self {
            interpreter: Interpreter::new(handler_registry),
            current_document: None,
            state_file: None,
        }
    }

    /// Set the state file path for persistence
    pub fn with_state_file(mut self, path: std::path::PathBuf) -> Self {
        self.state_file = Some(path);
        self
    }

    /// Load initial document
    #[allow(clippy::print_stderr)]
    pub fn load_document(&mut self, xml: &str) -> Result<(), String> {
        let start = Instant::now();

        match parse(xml) {
            Ok(doc) => {
                self.current_document = Some(doc);
                let duration = start.elapsed();
                eprintln!("[INFO] Document loaded in {}ms", duration.as_millis());
                Ok(())
            }
            Err(e) => {
                eprintln!("[ERROR] Failed to parse document: {}", e);
                Err(e.to_string())
            }
        }
    }

    /// Reload document from XML (preserving state)
    #[allow(clippy::print_stderr)]
    pub fn reload_document(&mut self, xml: &str) -> Result<ReloadResult, String> {
        let start = Instant::now();

        // Parse new document
        match parse(xml) {
            Ok(new_doc) => {
                self.current_document = Some(new_doc);

                let duration = start.elapsed();
                let latency_ms = duration.as_millis();

                eprintln!("[INFO] Reload completed in {}ms", latency_ms);

                Ok(ReloadResult::Success {
                    latency_ms,
                    state_restored: false, // State restoration handled separately
                })
            }
            Err(e) => {
                let duration = start.elapsed();
                eprintln!(
                    "[ERROR] Reload failed after {}ms: {}",
                    duration.as_millis(),
                    e
                );

                Ok(ReloadResult::Failure {
                    error: e.to_string(),
                    latency_ms: duration.as_millis(),
                })
            }
        }
    }

    /// Get the current document
    pub fn document(&self) -> Option<&GravityDocument> {
        self.current_document.as_ref()
    }

    /// Get mutable access to document (for rebuilding widget tree)
    pub fn document_mut(&mut self) -> Option<&mut GravityDocument> {
        self.current_document.as_mut()
    }

    /// Save current state to file
    pub fn save_state<T: serde::Serialize>(&self, model: T) -> Result<(), String> {
        if let Some(state_file) = &self.state_file {
            let state = crate::state::RuntimeState::new(model);
            crate::state::save_state_to_file(&state, state_file).map_err(|e| e.to_string())
        } else {
            Err("No state file configured".to_string())
        }
    }

    /// Load state from file
    pub fn load_state<T: for<'de> serde::Deserialize<'de>>(&self) -> Result<Option<T>, String> {
        if let Some(state_file) = &self.state_file {
            if state_file.exists() {
                let state =
                    crate::state::load_state_from_file(state_file).map_err(|e| e.to_string())?;
                return Ok(Some(state.model));
            }
        }
        Ok(None)
    }
}

/// Result of a reload operation
#[derive(Debug, Clone)]
pub enum ReloadResult {
    /// Reload succeeded
    Success {
        latency_ms: u128,
        state_restored: bool,
    },
    /// Reload failed
    Failure { error: String, latency_ms: u128 },
}

impl ReloadResult {
    /// Check if reload was successful
    pub fn is_success(&self) -> bool {
        matches!(self, ReloadResult::Success { .. })
    }

    /// Get latency in milliseconds
    pub fn latency_ms(&self) -> u128 {
        match self {
            ReloadResult::Success { latency_ms, .. } => *latency_ms,
            ReloadResult::Failure { latency_ms, .. } => *latency_ms,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gravity_core::HandlerRegistry;
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
            event: gravity_core::EventKind::Click,
            handler: "increment".to_string(),
            span: gravity_core::Span::new(0, 0, 1, 1),
        };

        let result = interpreter.dispatch(&event, &mut model, None);
        assert!(result.is_ok());
        assert_eq!(model.count, 1);
    }

    #[test]
    fn test_hot_reload_interpreter() {
        let registry = HandlerRegistry::new();
        let mut interpreter = HotReloadInterpreter::new(registry);

        let xml = r#"<column><text value="Test" /></column>"#;
        let result = interpreter.load_document(xml);

        assert!(result.is_ok());
        assert!(interpreter.document().is_some());
    }

    #[test]
    fn test_reload_result() {
        let success = ReloadResult::Success {
            latency_ms: 100,
            state_restored: true,
        };

        assert!(success.is_success());
        assert_eq!(success.latency_ms(), 100);

        let failure = ReloadResult::Failure {
            error: "Parse error".to_string(),
            latency_ms: 50,
        };

        assert!(!failure.is_success());
        assert_eq!(failure.latency_ms(), 50);
    }
}
