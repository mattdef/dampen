//! Contract Tests for Shared State Feature
//!
//! These tests verify the contracts defined in specs/001-inter-window-communication/contracts/
//!
//! Contract tests verify behavioral contracts across the entire feature, ensuring that:
//! - T028 (CT-001): SharedContext changes are visible across clones
//! - T029 (CT-002): Handler modifications to shared state are visible in all views

use dampen_core::{HandlerRegistry, SharedContext};
use dampen_macros::UiModel;
use serde::{Deserialize, Serialize};
use std::any::Any;

/// Shared state for contract tests
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, UiModel)]
struct TestSharedState {
    pub theme: String,
    pub counter: i32,
    pub enabled: bool,
}

impl Default for TestSharedState {
    fn default() -> Self {
        Self {
            theme: "light".to_string(),
            counter: 0,
            enabled: false,
        }
    }
}

/// Local model for contract tests
#[derive(Clone, Debug, Default, Serialize, Deserialize, UiModel)]
struct TestModel {
    pub status: String,
}

// ============================================================================
// T028: Contract Test CT-001 - SharedContext changes visible across clones
// ============================================================================

#[test]
fn ct_001_shared_context_changes_visible_across_clones() {
    // GIVEN: A SharedContext with initial state
    let shared = SharedContext::new(TestSharedState::default());

    // WHEN: Clone the context
    let clone1 = shared.clone();
    let clone2 = shared.clone();

    // AND: Modify state through first clone
    {
        let mut guard = clone1.write();
        guard.theme = "dark".to_string();
        guard.counter = 42;
        guard.enabled = true;
    }

    // THEN: Changes should be visible through second clone
    {
        let guard = clone2.read();
        assert_eq!(guard.theme, "dark", "Theme change not visible in clone2");
        assert_eq!(guard.counter, 42, "Counter change not visible in clone2");
        assert_eq!(guard.enabled, true, "Enabled change not visible in clone2");
    }

    // AND: Changes should be visible through original
    {
        let guard = shared.read();
        assert_eq!(guard.theme, "dark", "Theme change not visible in original");
        assert_eq!(guard.counter, 42, "Counter change not visible in original");
        assert_eq!(
            guard.enabled, true,
            "Enabled change not visible in original"
        );
    }
}

#[test]
fn ct_001_multiple_clones_see_same_state() {
    // GIVEN: A SharedContext and multiple clones simulating different views
    let shared = SharedContext::new(TestSharedState::default());
    let view1_shared = shared.clone(); // View 1 (Main window)
    let view2_shared = shared.clone(); // View 2 (Settings window)
    let view3_shared = shared.clone(); // View 3 (Admin panel)

    // WHEN: View 2 modifies the theme
    view2_shared.write().theme = "dark".to_string();

    // THEN: All views see the new theme
    assert_eq!(view1_shared.read().theme, "dark");
    assert_eq!(view2_shared.read().theme, "dark");
    assert_eq!(view3_shared.read().theme, "dark");
    assert_eq!(shared.read().theme, "dark");

    // WHEN: View 3 modifies the counter
    view3_shared.write().counter = 100;

    // THEN: All views see the new counter
    assert_eq!(view1_shared.read().counter, 100);
    assert_eq!(view2_shared.read().counter, 100);
    assert_eq!(view3_shared.read().counter, 100);
    assert_eq!(shared.read().counter, 100);
}

#[test]
fn ct_001_concurrent_reads_do_not_block() {
    // GIVEN: A SharedContext with some state
    let shared = SharedContext::new(TestSharedState {
        theme: "dark".to_string(),
        counter: 42,
        enabled: true,
    });

    // WHEN: Multiple clones read simultaneously
    let clone1 = shared.clone();
    let clone2 = shared.clone();
    let clone3 = shared.clone();

    let guard1 = clone1.read();
    let guard2 = clone2.read();
    let guard3 = clone3.read();

    // THEN: All guards see the same state (RwLock allows concurrent reads)
    assert_eq!(guard1.theme, "dark");
    assert_eq!(guard2.theme, "dark");
    assert_eq!(guard3.theme, "dark");

    assert_eq!(guard1.counter, 42);
    assert_eq!(guard2.counter, 42);
    assert_eq!(guard3.counter, 42);
}

// ============================================================================
// T029: Contract Test CT-002 - Handler modifies shared state, all views see change
// ============================================================================

#[test]
fn ct_002_handler_modifies_shared_all_views_see_change() {
    // GIVEN: A HandlerRegistry with a shared-state handler
    let shared = SharedContext::new(TestSharedState::default());
    let registry = HandlerRegistry::new();

    // Register handler that modifies shared state
    let shared_for_handler = shared.clone();
    registry.register_with_shared(
        "toggle_theme",
        move |_model: &mut dyn Any, shared_any: &dyn Any| {
            let shared_ctx = shared_any
                .downcast_ref::<SharedContext<TestSharedState>>()
                .unwrap_or(&shared_for_handler);

            let mut guard = shared_ctx.write();
            guard.theme = if guard.theme == "light" {
                "dark".to_string()
            } else {
                "light".to_string()
            };
        },
    );

    // Create clones simulating multiple views
    let view1_shared = shared.clone();
    let view2_shared = shared.clone();

    // Initial state
    assert_eq!(view1_shared.read().theme, "light");
    assert_eq!(view2_shared.read().theme, "light");

    // WHEN: Handler is dispatched from View 1
    let mut model = TestModel::default();
    registry.dispatch_with_shared(
        "toggle_theme",
        &mut model as &mut dyn Any,
        &shared as &dyn Any,
        None,
    );

    // THEN: Both views see the updated state
    assert_eq!(
        view1_shared.read().theme,
        "dark",
        "View 1 should see theme change"
    );
    assert_eq!(
        view2_shared.read().theme,
        "dark",
        "View 2 should see theme change"
    );

    // WHEN: Handler is dispatched again from View 2
    registry.dispatch_with_shared(
        "toggle_theme",
        &mut model as &mut dyn Any,
        &view2_shared as &dyn Any,
        None,
    );

    // THEN: Both views see the updated state
    assert_eq!(
        view1_shared.read().theme,
        "light",
        "View 1 should see second toggle"
    );
    assert_eq!(
        view2_shared.read().theme,
        "light",
        "View 2 should see second toggle"
    );
}

#[test]
fn ct_002_multiple_handlers_modify_different_fields() {
    // GIVEN: A HandlerRegistry with multiple handlers
    let shared = SharedContext::new(TestSharedState::default());
    let registry = HandlerRegistry::new();

    // Handler 1: Modifies theme
    let shared_clone = shared.clone();
    registry.register_with_shared(
        "set_theme_dark",
        move |_model: &mut dyn Any, shared_any: &dyn Any| {
            let shared_ctx = shared_any
                .downcast_ref::<SharedContext<TestSharedState>>()
                .unwrap_or(&shared_clone);
            shared_ctx.write().theme = "dark".to_string();
        },
    );

    // Handler 2: Increments counter
    let shared_clone = shared.clone();
    registry.register_with_shared(
        "increment_counter",
        move |_model: &mut dyn Any, shared_any: &dyn Any| {
            let shared_ctx = shared_any
                .downcast_ref::<SharedContext<TestSharedState>>()
                .unwrap_or(&shared_clone);
            shared_ctx.write().counter += 1;
        },
    );

    // Handler 3: Toggles enabled
    let shared_clone = shared.clone();
    registry.register_with_shared(
        "toggle_enabled",
        move |_model: &mut dyn Any, shared_any: &dyn Any| {
            let shared_ctx = shared_any
                .downcast_ref::<SharedContext<TestSharedState>>()
                .unwrap_or(&shared_clone);
            let mut guard = shared_ctx.write();
            guard.enabled = !guard.enabled;
        },
    );

    let view_shared = shared.clone();
    let mut model = TestModel::default();

    // WHEN: Handlers are dispatched in sequence
    registry.dispatch_with_shared(
        "set_theme_dark",
        &mut model as &mut dyn Any,
        &shared as &dyn Any,
        None,
    );
    registry.dispatch_with_shared(
        "increment_counter",
        &mut model as &mut dyn Any,
        &shared as &dyn Any,
        None,
    );
    registry.dispatch_with_shared(
        "increment_counter",
        &mut model as &mut dyn Any,
        &shared as &dyn Any,
        None,
    );
    registry.dispatch_with_shared(
        "toggle_enabled",
        &mut model as &mut dyn Any,
        &shared as &dyn Any,
        None,
    );

    // THEN: All modifications are visible
    let guard = view_shared.read();
    assert_eq!(guard.theme, "dark", "Theme should be dark");
    assert_eq!(guard.counter, 2, "Counter should be 2");
    assert_eq!(guard.enabled, true, "Enabled should be true");
}

#[test]
fn ct_002_handler_with_value_parameter() {
    // GIVEN: A HandlerRegistry with a value-accepting handler
    let shared = SharedContext::new(TestSharedState::default());
    let registry = HandlerRegistry::new();

    // Register handler that takes a value parameter
    let shared_clone = shared.clone();
    registry.register_with_value_and_shared(
        "set_theme",
        move |_model: &mut dyn Any, value: Box<dyn Any>, shared_any: &dyn Any| {
            let shared_ctx = shared_any
                .downcast_ref::<SharedContext<TestSharedState>>()
                .unwrap_or(&shared_clone);

            if let Some(theme) = value.downcast_ref::<String>() {
                shared_ctx.write().theme = theme.clone();
            }
        },
    );

    let view_shared = shared.clone();
    let mut model = TestModel::default();

    // WHEN: Handler is dispatched with a value
    registry.dispatch_with_shared(
        "set_theme",
        &mut model as &mut dyn Any,
        &shared as &dyn Any,
        Some("custom-theme".to_string()),
    );

    // THEN: The value is applied to shared state
    assert_eq!(view_shared.read().theme, "custom-theme");
}

#[test]
fn ct_002_shared_state_persists_across_view_switches() {
    // GIVEN: Two views with their own models but shared SharedContext
    let shared = SharedContext::new(TestSharedState::default());

    // View 1: Settings view
    let view1_registry = HandlerRegistry::new();
    let shared_clone = shared.clone();
    view1_registry.register_with_shared(
        "set_counter",
        move |_model: &mut dyn Any, shared_any: &dyn Any| {
            let shared_ctx = shared_any
                .downcast_ref::<SharedContext<TestSharedState>>()
                .unwrap_or(&shared_clone);
            shared_ctx.write().counter = 999;
        },
    );

    // View 2: Main view (just reads shared state, no handlers)
    let view2_shared = shared.clone();

    let mut view1_model = TestModel {
        status: "settings".to_string(),
    };

    // WHEN: User is in View 1 and triggers handler
    view1_registry.dispatch_with_shared(
        "set_counter",
        &mut view1_model as &mut dyn Any,
        &shared as &dyn Any,
        None,
    );

    // THEN: After switching to View 2, the state persists
    assert_eq!(
        view2_shared.read().counter,
        999,
        "State should persist when switching views"
    );
}
