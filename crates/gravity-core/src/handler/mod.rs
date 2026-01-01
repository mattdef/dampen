//! Handler system for event dispatch

use std::any::Any;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// Registry of event handlers
#[derive(Clone)]
pub struct HandlerRegistry {
    handlers: Arc<RwLock<HashMap<String, HandlerEntry>>>,
}

/// Entry in the handler registry
#[derive(Clone)]
#[allow(clippy::type_complexity)]
pub enum HandlerEntry {
    /// Simple handler: `fn(&mut Model)`
    Simple(Arc<dyn Fn(&mut dyn Any) + Send + Sync>),

    /// Handler with value: `fn(&mut Model, T)`
    WithValue(Arc<dyn Fn(&mut dyn Any, Box<dyn Any>) + Send + Sync>),

    /// Handler returning command: `fn(&mut Model) -> Command<Message>`
    WithCommand(Arc<dyn Fn(&mut dyn Any) -> Box<dyn Any> + Send + Sync>),
}

impl HandlerRegistry {
    /// Create a new empty handler registry
    pub fn new() -> Self {
        Self {
            handlers: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Register a simple handler
    pub fn register_simple<F>(&self, name: &str, handler: F)
    where
        F: Fn(&mut dyn Any) + Send + Sync + 'static,
    {
        if let Ok(mut handlers) = self.handlers.write() {
            handlers.insert(name.to_string(), HandlerEntry::Simple(Arc::new(handler)));
        }
    }

    /// Register a handler with a value parameter
    pub fn register_with_value<F>(&self, name: &str, handler: F)
    where
        F: Fn(&mut dyn Any, Box<dyn Any>) + Send + Sync + 'static,
    {
        if let Ok(mut handlers) = self.handlers.write() {
            handlers.insert(name.to_string(), HandlerEntry::WithValue(Arc::new(handler)));
        }
    }

    /// Register a handler that returns a command
    pub fn register_with_command<F>(&self, name: &str, handler: F)
    where
        F: Fn(&mut dyn Any) -> Box<dyn Any> + Send + Sync + 'static,
    {
        if let Ok(mut handlers) = self.handlers.write() {
            handlers.insert(
                name.to_string(),
                HandlerEntry::WithCommand(Arc::new(handler)),
            );
        }
    }

    /// Look up a handler by name
    pub fn get(&self, name: &str) -> Option<HandlerEntry> {
        self.handlers.read().ok()?.get(name).cloned()
    }

    /// Check if a handler exists
    pub fn contains(&self, name: &str) -> bool {
        if let Ok(handlers) = self.handlers.read() {
            handlers.contains_key(name)
        } else {
            false
        }
    }
}

impl Default for HandlerRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Handler metadata for compile-time validation
#[derive(Debug, Clone, PartialEq)]
pub struct HandlerSignature {
    /// Handler name
    pub name: String,

    /// Parameter type if applicable
    pub param_type: Option<String>,

    /// Whether handler returns Command
    pub returns_command: bool,
}
