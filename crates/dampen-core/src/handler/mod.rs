//! Handler system for event dispatch

use std::any::Any;
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, RwLock};

/// Registry of event handlers
#[derive(Clone, Debug)]
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

impl std::fmt::Debug for HandlerEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HandlerEntry::Simple(_) => f.write_str("Simple(handler)"),
            HandlerEntry::WithValue(_) => f.write_str("WithValue(handler)"),
            HandlerEntry::WithCommand(_) => f.write_str("WithCommand(handler)"),
        }
    }
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

    /// Dispatches a handler by name, executing it with the provided model and optional value.
    ///
    /// This is a convenience method that combines `get()` and handler invocation.
    ///
    /// # Arguments
    ///
    /// * `handler_name` - Name of the handler to dispatch
    /// * `model` - Mutable reference to the model (as `&mut dyn Any`)
    /// * `value` - Optional string value passed to WithValue handlers
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use dampen_core::HandlerRegistry;
    ///
    /// let registry = HandlerRegistry::new();
    /// registry.register_simple("greet", |model| {
    ///     let model = model.downcast_mut::<MyModel>().unwrap();
    ///     model.count += 1;
    /// });
    ///
    /// let model = &mut MyModel { count: 0 } as &mut dyn std::any::Any;
    /// registry.dispatch("greet", model, None);
    /// ```
    pub fn dispatch(&self, handler_name: &str, model: &mut dyn Any, value: Option<String>) {
        if let Some(entry) = self.get(handler_name) {
            match entry {
                HandlerEntry::Simple(h) => h(model),
                HandlerEntry::WithValue(h) => {
                    let val = value.unwrap_or_default();
                    h(model, Box::new(val));
                }
                HandlerEntry::WithCommand(h) => {
                    h(model);
                }
            }
        }
    }

    /// Dispatches a handler by name and returns any command/task it produces.
    ///
    /// This method is similar to `dispatch()` but returns the command/task from
    /// `WithCommand` handlers instead of discarding it. This is essential for
    /// integrating with the Elm/MVU pattern where handlers can return tasks.
    ///
    /// # Arguments
    ///
    /// * `handler_name` - Name of the handler to dispatch
    /// * `model` - Mutable reference to the model (as `&mut dyn Any`)
    /// * `value` - Optional string value passed to WithValue handlers
    ///
    /// # Returns
    ///
    /// * `Some(Box<dyn Any>)` - The command/task from a `WithCommand` handler
    /// * `None` - For `Simple` and `WithValue` handlers, or if handler not found
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use dampen_core::HandlerRegistry;
    /// use iced::Task;
    ///
    /// let registry = HandlerRegistry::new();
    /// registry.register_with_command("navigate", |model| {
    ///     let model = model.downcast_mut::<MyModel>().unwrap();
    ///     Box::new(Task::done(Message::SwitchView))
    /// });
    ///
    /// let model = &mut MyModel::default() as &mut dyn std::any::Any;
    /// if let Some(boxed_task) = registry.dispatch_with_command("navigate", model, None) {
    ///     if let Ok(task) = boxed_task.downcast::<Task<Message>>() {
    ///         return *task;
    ///     }
    /// }
    /// ```
    pub fn dispatch_with_command(
        &self,
        handler_name: &str,
        model: &mut dyn Any,
        value: Option<String>,
    ) -> Option<Box<dyn Any>> {
        if let Some(entry) = self.get(handler_name) {
            match entry {
                HandlerEntry::Simple(h) => {
                    h(model);
                    None
                }
                HandlerEntry::WithValue(h) => {
                    let val = value.unwrap_or_default();
                    h(model, Box::new(val));
                    None
                }
                HandlerEntry::WithCommand(h) => Some(h(model)),
            }
        } else {
            None
        }
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

/// Build-time analysis structure for circular dependency detection
#[derive(Debug, Clone)]
pub struct HandlerCallGraph {
    /// Map of handler name to its dependencies
    dependencies: HashMap<String, Vec<String>>,
}

impl HandlerCallGraph {
    /// Create a new empty call graph
    pub fn new() -> Self {
        Self {
            dependencies: HashMap::new(),
        }
    }

    /// Add a dependency edge (from depends on to)
    pub fn add_dependency(&mut self, from: &str, to: &str) {
        self.dependencies
            .entry(from.to_string())
            .or_default()
            .push(to.to_string());
    }

    /// Detect if adding edge would create a cycle
    pub fn would_create_cycle(&self, from: &str, to: &str) -> bool {
        // Check if 'to' can reach 'from' (which would create a cycle)
        let mut visited = HashSet::new();
        self.can_reach(to, from, &mut visited)
    }

    /// Check if 'from' can reach 'to' via dependencies
    fn can_reach(&self, from: &str, to: &str, visited: &mut HashSet<String>) -> bool {
        if from == to {
            return true;
        }

        if visited.contains(from) {
            return false;
        }

        visited.insert(from.to_string());

        if let Some(deps) = self.dependencies.get(from) {
            for dep in deps {
                if self.can_reach(dep, to, visited) {
                    return true;
                }
            }
        }

        false
    }

    /// Get all handlers that depend on the given handler
    pub fn dependents_of(&self, handler: &str) -> Vec<String> {
        self.dependencies
            .iter()
            .filter_map(|(k, v)| {
                if v.contains(&handler.to_string()) {
                    Some(k.clone())
                } else {
                    None
                }
            })
            .collect()
    }

    /// Detect cycles in the call graph using DFS
    pub fn detect_cycles(&self) -> Option<Vec<String>> {
        let mut visited = HashSet::new();
        let mut recursion_stack = HashSet::new();
        let mut path = Vec::new();

        for handler in self.dependencies.keys() {
            if !visited.contains(handler) {
                if let Some(cycle) =
                    self.dfs_detect_cycle(handler, &mut visited, &mut recursion_stack, &mut path)
                {
                    return Some(cycle);
                }
            }
        }

        None
    }

    fn dfs_detect_cycle(
        &self,
        handler: &str,
        visited: &mut HashSet<String>,
        recursion_stack: &mut HashSet<String>,
        path: &mut Vec<String>,
    ) -> Option<Vec<String>> {
        visited.insert(handler.to_string());
        recursion_stack.insert(handler.to_string());
        path.push(handler.to_string());

        if let Some(deps) = self.dependencies.get(handler) {
            for dep in deps {
                if !visited.contains(dep) {
                    if let Some(cycle) = self.dfs_detect_cycle(dep, visited, recursion_stack, path)
                    {
                        return Some(cycle);
                    }
                } else if recursion_stack.contains(dep) {
                    // Found a cycle - construct the cycle path
                    if let Some(cycle_start) = path.iter().position(|h| h == dep) {
                        let mut cycle = path[cycle_start..].to_vec();
                        cycle.push(dep.to_string());
                        return Some(cycle);
                    }
                }
            }
        }

        path.pop();
        recursion_stack.remove(handler);
        None
    }
}

impl Default for HandlerCallGraph {
    fn default() -> Self {
        Self::new()
    }
}
