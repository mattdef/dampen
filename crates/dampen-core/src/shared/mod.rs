//! Shared state container for inter-window communication.
//!
//! This module provides the [`SharedContext`] type for sharing state across
//! multiple views in a Dampen application.
//!
//! # Overview
//!
//! `SharedContext<S>` is a thread-safe, reference-counted container that allows
//! multiple views to read and write a shared state. When one view modifies the
//! shared state, all other views immediately see the change.
//!
//! # Example
//!
//! ```rust
//! use dampen_core::SharedContext;
//! use dampen_core::UiBindable;
//! use dampen_core::BindingValue;
//!
//! #[derive(Default, Clone)]
//! struct SharedState {
//!     theme: String,
//!     language: String,
//! }
//!
//! impl UiBindable for SharedState {
//!     fn get_field(&self, path: &[&str]) -> Option<BindingValue> {
//!         match path {
//!             ["theme"] => Some(BindingValue::String(self.theme.clone())),
//!             ["language"] => Some(BindingValue::String(self.language.clone())),
//!             _ => None,
//!         }
//!     }
//!     fn available_fields() -> Vec<String> {
//!         vec!["theme".to_string(), "language".to_string()]
//!     }
//! }
//!
//! // Create shared context
//! let ctx = SharedContext::new(SharedState::default());
//!
//! // Clone for another view (same underlying state)
//! let ctx2 = ctx.clone();
//!
//! // Modify in one view
//! ctx.write().theme = "dark".to_string();
//!
//! // See change in another view
//! assert_eq!(ctx2.read().theme, "dark");
//! ```
//!
//! # Thread Safety
//!
//! `SharedContext` uses `Arc<RwLock<S>>` internally, making it safe to share
//! across threads. Multiple readers can access the state simultaneously, but
//! writers get exclusive access.
//!
//! # See Also
//!
//! - [`AppState`](crate::state::AppState) - Per-view state container that can hold a `SharedContext`
//! - [`UiBindable`](crate::binding::UiBindable) - Trait required for shared state types

use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

use crate::binding::{BindingValue, UiBindable};

/// Thread-safe shared state container.
///
/// `SharedContext` wraps user-defined shared state in an `Arc<RwLock<S>>`,
/// enabling safe concurrent access from multiple views. Each view receives
/// a cloned reference to the same underlying state.
///
/// # Type Parameters
///
/// * `S` - The shared state type. Must implement:
///   - [`UiBindable`] for XML binding access (e.g., `{shared.field}`)
///   - `Send + Sync` for thread safety
///   - `'static` for Arc storage
///
/// # Example
///
/// ```rust
/// use dampen_core::SharedContext;
/// use dampen_core::{UiBindable, BindingValue};
///
/// #[derive(Default, Clone)]
/// struct SharedState {
///     theme: String,
///     user_name: Option<String>,
/// }
///
/// impl UiBindable for SharedState {
///     fn get_field(&self, path: &[&str]) -> Option<BindingValue> {
///         match path {
///             ["theme"] => Some(BindingValue::String(self.theme.clone())),
///             ["user_name"] => match &self.user_name {
///                 Some(name) => Some(BindingValue::String(name.clone())),
///                 None => Some(BindingValue::None),
///             },
///             _ => None,
///         }
///     }
///     fn available_fields() -> Vec<String> {
///         vec!["theme".to_string(), "user_name".to_string()]
///     }
/// }
///
/// let ctx = SharedContext::new(SharedState::default());
/// let ctx2 = ctx.clone(); // Same underlying state
///
/// ctx.write().theme = "dark".to_string();
/// assert_eq!(ctx2.read().theme, "dark");
/// ```
///
/// # Lock Poisoning
///
/// If a thread panics while holding a write lock, the lock becomes "poisoned".
/// The `read()` and `write()` methods will panic in this case. Use `try_read()`
/// and `try_write()` for fallible access.
#[derive(Debug)]
pub struct SharedContext<S>
where
    S: UiBindable + Send + Sync + 'static,
{
    state: Arc<RwLock<S>>,
}

impl<S> SharedContext<S>
where
    S: UiBindable + Send + Sync + 'static,
{
    /// Create a new SharedContext with initial state.
    ///
    /// # Arguments
    ///
    /// * `initial` - The initial shared state value
    ///
    /// # Example
    ///
    /// ```rust
    /// use dampen_core::SharedContext;
    /// use dampen_core::{UiBindable, BindingValue};
    ///
    /// #[derive(Default)]
    /// struct MyState { counter: i32 }
    ///
    /// impl UiBindable for MyState {
    ///     fn get_field(&self, path: &[&str]) -> Option<BindingValue> {
    ///         match path {
    ///             ["counter"] => Some(BindingValue::Integer(self.counter as i64)),
    ///             _ => None,
    ///         }
    ///     }
    ///     fn available_fields() -> Vec<String> { vec!["counter".to_string()] }
    /// }
    ///
    /// let ctx = SharedContext::new(MyState { counter: 42 });
    /// assert_eq!(ctx.read().counter, 42);
    /// ```
    pub fn new(initial: S) -> Self {
        Self {
            state: Arc::new(RwLock::new(initial)),
        }
    }

    /// Acquire read access to shared state.
    ///
    /// Returns a guard that provides immutable access to the shared state.
    /// Multiple readers can hold guards simultaneously.
    ///
    /// # Panics
    ///
    /// Panics if the lock is poisoned (a thread panicked while holding write lock).
    /// Use [`try_read`](Self::try_read) for fallible access.
    ///
    /// # Example
    ///
    /// ```rust
    /// use dampen_core::SharedContext;
    /// use dampen_core::{UiBindable, BindingValue};
    ///
    /// #[derive(Default)]
    /// struct MyState { value: String }
    ///
    /// impl UiBindable for MyState {
    ///     fn get_field(&self, path: &[&str]) -> Option<BindingValue> {
    ///         match path {
    ///             ["value"] => Some(BindingValue::String(self.value.clone())),
    ///             _ => None,
    ///         }
    ///     }
    ///     fn available_fields() -> Vec<String> { vec!["value".to_string()] }
    /// }
    ///
    /// let ctx = SharedContext::new(MyState { value: "hello".to_string() });
    /// let guard = ctx.read();
    /// assert_eq!(guard.value, "hello");
    /// ```
    #[allow(clippy::expect_used)]
    pub fn read(&self) -> RwLockReadGuard<'_, S> {
        self.state.read().expect("SharedContext lock poisoned")
    }

    /// Acquire write access to shared state.
    ///
    /// Returns a guard that provides mutable access to the shared state.
    /// Only one writer can hold the guard at a time, and no readers can
    /// access the state while a write guard is held.
    ///
    /// # Panics
    ///
    /// Panics if the lock is poisoned.
    /// Use [`try_write`](Self::try_write) for fallible access.
    ///
    /// # Example
    ///
    /// ```rust
    /// use dampen_core::SharedContext;
    /// use dampen_core::{UiBindable, BindingValue};
    ///
    /// #[derive(Default)]
    /// struct MyState { counter: i32 }
    ///
    /// impl UiBindable for MyState {
    ///     fn get_field(&self, path: &[&str]) -> Option<BindingValue> {
    ///         match path {
    ///             ["counter"] => Some(BindingValue::Integer(self.counter as i64)),
    ///             _ => None,
    ///         }
    ///     }
    ///     fn available_fields() -> Vec<String> { vec!["counter".to_string()] }
    /// }
    ///
    /// let ctx = SharedContext::new(MyState { counter: 0 });
    /// ctx.write().counter += 1;
    /// assert_eq!(ctx.read().counter, 1);
    /// ```
    #[allow(clippy::expect_used)]
    pub fn write(&self) -> RwLockWriteGuard<'_, S> {
        self.state.write().expect("SharedContext lock poisoned")
    }

    /// Try to acquire read access without blocking.
    ///
    /// Returns `None` if the lock is currently held for writing or is poisoned.
    /// This is useful when you want to avoid blocking on a potentially
    /// long-held write lock.
    ///
    /// # Example
    ///
    /// ```rust
    /// use dampen_core::SharedContext;
    /// use dampen_core::{UiBindable, BindingValue};
    ///
    /// #[derive(Default)]
    /// struct MyState { value: i32 }
    ///
    /// impl UiBindable for MyState {
    ///     fn get_field(&self, path: &[&str]) -> Option<BindingValue> {
    ///         match path {
    ///             ["value"] => Some(BindingValue::Integer(self.value as i64)),
    ///             _ => None,
    ///         }
    ///     }
    ///     fn available_fields() -> Vec<String> { vec!["value".to_string()] }
    /// }
    ///
    /// let ctx = SharedContext::new(MyState { value: 42 });
    /// if let Some(guard) = ctx.try_read() {
    ///     assert_eq!(guard.value, 42);
    /// }
    /// ```
    pub fn try_read(&self) -> Option<RwLockReadGuard<'_, S>> {
        self.state.try_read().ok()
    }

    /// Try to acquire write access without blocking.
    ///
    /// Returns `None` if the lock is currently held (for reading or writing)
    /// or is poisoned.
    ///
    /// # Example
    ///
    /// ```rust
    /// use dampen_core::SharedContext;
    /// use dampen_core::{UiBindable, BindingValue};
    ///
    /// #[derive(Default)]
    /// struct MyState { value: i32 }
    ///
    /// impl UiBindable for MyState {
    ///     fn get_field(&self, path: &[&str]) -> Option<BindingValue> {
    ///         match path {
    ///             ["value"] => Some(BindingValue::Integer(self.value as i64)),
    ///             _ => None,
    ///         }
    ///     }
    ///     fn available_fields() -> Vec<String> { vec!["value".to_string()] }
    /// }
    ///
    /// let ctx = SharedContext::new(MyState { value: 0 });
    /// if let Some(mut guard) = ctx.try_write() {
    ///     guard.value = 100;
    /// }
    /// ```
    pub fn try_write(&self) -> Option<RwLockWriteGuard<'_, S>> {
        self.state.try_write().ok()
    }
}

impl<S> Clone for SharedContext<S>
where
    S: UiBindable + Send + Sync + 'static,
{
    /// Clone the SharedContext.
    ///
    /// This creates a new `SharedContext` that references the same underlying
    /// state. Modifications through one clone are visible to all others.
    fn clone(&self) -> Self {
        Self {
            state: Arc::clone(&self.state),
        }
    }
}

/// Implement UiBindable for SharedContext by delegating to the inner state.
///
/// This allows SharedContext to be used directly in widget builders and bindings
/// without needing to extract the inner state first.
///
/// # Example
///
/// ```rust,ignore
/// use dampen_core::{SharedContext, UiBindable, BindingValue};
///
/// #[derive(Default)]
/// struct MyState { value: i32 }
///
/// impl UiBindable for MyState {
///     fn get_field(&self, path: &[&str]) -> Option<BindingValue> {
///         match path {
///             ["value"] => Some(BindingValue::Integer(self.value as i64)),
///             _ => None,
///         }
///     }
///     fn available_fields() -> Vec<String> { vec!["value".to_string()] }
/// }
///
/// let ctx = SharedContext::new(MyState { value: 42 });
/// // Can use ctx directly as &dyn UiBindable
/// assert_eq!(ctx.get_field(&["value"]), Some(BindingValue::Integer(42)));
/// ```
impl<S> UiBindable for SharedContext<S>
where
    S: UiBindable + Send + Sync + 'static,
{
    fn get_field(&self, path: &[&str]) -> Option<BindingValue> {
        // Acquire read lock and delegate to inner state
        let guard = self.read();
        guard.get_field(path)
    }

    fn available_fields() -> Vec<String> {
        // Delegate to the inner type's available fields
        S::available_fields()
    }
}

/// Special implementation for unit type when shared state is not used.
///
/// This allows applications that don't use shared state to still compile
/// without requiring a real shared state type.
impl SharedContext<()> {
    /// Create an empty shared context (no-op).
    ///
    /// Used internally when an application doesn't configure shared state.
    /// The resulting context holds unit type `()` and is essentially a no-op.
    ///
    /// # Example
    ///
    /// ```rust
    /// use dampen_core::SharedContext;
    ///
    /// let ctx = SharedContext::<()>::empty();
    /// // Can still call read/write, but they just return ()
    /// let _guard = ctx.read();
    /// ```
    pub fn empty() -> Self {
        Self::new(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::BindingValue;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::thread;

    /// Test helper: Simple shared state
    #[derive(Default, Clone)]
    struct TestState {
        counter: i32,
        name: String,
    }

    impl UiBindable for TestState {
        fn get_field(&self, path: &[&str]) -> Option<BindingValue> {
            match path {
                ["counter"] => Some(BindingValue::Integer(self.counter as i64)),
                ["name"] => Some(BindingValue::String(self.name.clone())),
                _ => None,
            }
        }

        fn available_fields() -> Vec<String> {
            vec!["counter".to_string(), "name".to_string()]
        }
    }

    // ========================================
    // T011: Test SharedContext read/write access
    // ========================================

    #[test]
    fn test_shared_context_new() {
        let ctx = SharedContext::new(TestState {
            counter: 42,
            name: "test".to_string(),
        });
        assert_eq!(ctx.read().counter, 42);
        assert_eq!(ctx.read().name, "test");
    }

    #[test]
    fn test_shared_context_read() {
        let ctx = SharedContext::new(TestState {
            counter: 10,
            name: "hello".to_string(),
        });

        let guard = ctx.read();
        assert_eq!(guard.counter, 10);
        assert_eq!(guard.name, "hello");
    }

    #[test]
    fn test_shared_context_write() {
        let ctx = SharedContext::new(TestState::default());

        {
            let mut guard = ctx.write();
            guard.counter = 100;
            guard.name = "updated".to_string();
        }

        assert_eq!(ctx.read().counter, 100);
        assert_eq!(ctx.read().name, "updated");
    }

    #[test]
    fn test_shared_context_try_read() {
        let ctx = SharedContext::new(TestState {
            counter: 5,
            ..Default::default()
        });

        // Should succeed when no write lock is held
        let guard = ctx.try_read();
        assert!(guard.is_some());
        assert_eq!(guard.unwrap().counter, 5);
    }

    #[test]
    fn test_shared_context_try_write() {
        let ctx = SharedContext::new(TestState::default());

        // Should succeed when no lock is held
        let guard = ctx.try_write();
        assert!(guard.is_some());
    }

    // ========================================
    // T012: Test SharedContext clone shares state
    // ========================================

    #[test]
    fn test_shared_context_clone_shares_state() {
        let ctx1 = SharedContext::new(TestState {
            counter: 0,
            name: "original".to_string(),
        });

        // Clone the context
        let ctx2 = ctx1.clone();

        // Modify through ctx1
        ctx1.write().counter = 42;
        ctx1.write().name = "modified".to_string();

        // Verify ctx2 sees the changes
        assert_eq!(ctx2.read().counter, 42);
        assert_eq!(ctx2.read().name, "modified");

        // Modify through ctx2
        ctx2.write().counter = 100;

        // Verify ctx1 sees the changes
        assert_eq!(ctx1.read().counter, 100);
    }

    #[test]
    fn test_shared_context_multiple_clones() {
        let original = SharedContext::new(TestState {
            counter: 1,
            ..Default::default()
        });

        let clone1 = original.clone();
        let clone2 = original.clone();
        let clone3 = clone1.clone();

        // All clones point to the same state
        original.write().counter = 999;

        assert_eq!(clone1.read().counter, 999);
        assert_eq!(clone2.read().counter, 999);
        assert_eq!(clone3.read().counter, 999);
    }

    // ========================================
    // T013: Test SharedContext thread safety with concurrent access
    // ========================================

    #[test]
    fn test_shared_context_concurrent_reads() {
        let ctx = SharedContext::new(TestState {
            counter: 42,
            name: "concurrent".to_string(),
        });

        let read_count = Arc::new(AtomicUsize::new(0));
        let mut handles = vec![];

        // Spawn multiple reader threads
        for _ in 0..10 {
            let ctx_clone = ctx.clone();
            let count = Arc::clone(&read_count);

            let handle = thread::spawn(move || {
                let guard = ctx_clone.read();
                assert_eq!(guard.counter, 42);
                assert_eq!(guard.name, "concurrent");
                count.fetch_add(1, Ordering::SeqCst);
            });

            handles.push(handle);
        }

        // Wait for all threads
        for handle in handles {
            handle.join().expect("Thread panicked");
        }

        assert_eq!(read_count.load(Ordering::SeqCst), 10);
    }

    #[test]
    fn test_shared_context_concurrent_writes() {
        let ctx = SharedContext::new(TestState::default());
        let mut handles = vec![];

        // Spawn multiple writer threads, each incrementing counter
        for i in 0..10 {
            let ctx_clone = ctx.clone();

            let handle = thread::spawn(move || {
                let mut guard = ctx_clone.write();
                guard.counter += 1;
                guard.name = format!("writer-{}", i);
            });

            handles.push(handle);
        }

        // Wait for all threads
        for handle in handles {
            handle.join().expect("Thread panicked");
        }

        // Counter should be exactly 10 (each thread incremented once)
        assert_eq!(ctx.read().counter, 10);
    }

    #[test]
    fn test_shared_context_mixed_read_write() {
        let ctx = SharedContext::new(TestState {
            counter: 0,
            ..Default::default()
        });

        let mut handles = vec![];

        // Spawn writer threads
        for _ in 0..5 {
            let ctx_clone = ctx.clone();
            let handle = thread::spawn(move || {
                for _ in 0..100 {
                    ctx_clone.write().counter += 1;
                }
            });
            handles.push(handle);
        }

        // Spawn reader threads
        for _ in 0..5 {
            let ctx_clone = ctx.clone();
            let handle = thread::spawn(move || {
                for _ in 0..100 {
                    let _ = ctx_clone.read().counter;
                }
            });
            handles.push(handle);
        }

        // Wait for all threads
        for handle in handles {
            handle.join().expect("Thread panicked");
        }

        // Counter should be exactly 500 (5 writers * 100 increments each)
        assert_eq!(ctx.read().counter, 500);
    }

    // ========================================
    // Additional tests for edge cases
    // ========================================

    #[test]
    fn test_shared_context_empty() {
        let ctx = SharedContext::<()>::empty();
        // Should be able to read and write without panicking
        let _read_guard = ctx.read();
        drop(_read_guard);
        let _write_guard = ctx.write();
        drop(_write_guard);
    }

    #[test]
    fn test_shared_context_ui_bindable_integration() {
        let ctx = SharedContext::new(TestState {
            counter: 123,
            name: "bindable".to_string(),
        });

        // Test UiBindable through the SharedContext
        let guard = ctx.read();
        assert_eq!(
            guard.get_field(&["counter"]),
            Some(BindingValue::Integer(123))
        );
        assert_eq!(
            guard.get_field(&["name"]),
            Some(BindingValue::String("bindable".to_string()))
        );
        assert_eq!(guard.get_field(&["nonexistent"]), None);
    }
}
