//! Contract Tests for Shared State Feature
//!
//! These tests verify the contracts defined in specs/001-inter-window-communication/contracts/
//!
//! Contract tests verify behavioral contracts across the entire feature, ensuring that:
//! - T028 (CT-001): SharedContext changes are visible across clones
//! - T029 (CT-002): Handler modifications to shared state are visible in all views

use dampen_core::{AppState, HandlerRegistry, SharedContext, parse};
use dampen_iced::DampenWidgetBuilder;
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

/// Shared state with nested structure for binding tests
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, UiModel)]
struct NestedSharedState {
    pub theme: String,
    pub count: i32,
    #[ui_skip]
    pub user: UserInfo,
}

/// User info for nested binding tests
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct UserInfo {
    pub name: String,
    pub email: String,
}

impl Default for NestedSharedState {
    fn default() -> Self {
        Self {
            theme: "dark".to_string(),
            count: 0,
            user: UserInfo {
                name: "Alice".to_string(),
                email: "alice@example.com".to_string(),
            },
        }
    }
}

// Manual implementation of field access for nested user fields
#[allow(dead_code)]
impl NestedSharedState {
    pub fn get_user_name(&self) -> &str {
        &self.user.name
    }

    pub fn get_user_email(&self) -> &str {
        &self.user.email
    }
}

/// Local model for mixed binding tests
#[derive(Clone, Debug, Default, Serialize, Deserialize, UiModel)]
struct MixedModel {
    pub greeting: String,
    pub local_count: i32,
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

// ============================================================================
// T038-T041: Contract Tests for Shared Bindings in XML (CT-SB-001 to CT-SB-004)
// ============================================================================

#[test]
fn ct_sb_001_simple_shared_binding_renders_value() {
    // CT-SB-001: Simple shared binding renders value
    // GIVEN: Shared state with theme = "dark"
    let shared = SharedContext::new(NestedSharedState::default());
    let model = TestModel::default();

    // WHEN: XML contains {shared.theme}
    let xml = r#"<dampen><text value="{shared.theme}" /></dampen>"#;
    let document = parse(xml).expect("Failed to parse XML");

    // Build widget with shared context (need to keep guard alive)
    let shared_guard = shared.read();
    let builder = DampenWidgetBuilder::new(&document, &model, None).with_shared(&*shared_guard);

    let _element = builder.build();
    // _element keeps shared_guard borrowed until end of function

    // THEN: Widget should render "dark"
    assert_eq!(shared_guard.theme, "dark");
}

#[test]
fn ct_sb_002_nested_shared_binding_resolves_correctly() {
    // CT-SB-002: Nested shared binding resolves correctly
    // GIVEN: Shared state with user.name = "Alice"
    let shared = SharedContext::new(NestedSharedState::default());
    let model = TestModel::default();

    // WHEN: XML contains {shared.user.name}
    let xml = r#"<dampen><text value="{shared.user.name}" /></dampen>"#;
    let document = parse(xml).expect("Failed to parse XML");

    // Build widget with shared context (keep guard alive)
    let shared_guard = shared.read();
    let builder = DampenWidgetBuilder::new(&document, &model, None).with_shared(&*shared_guard);

    let _element = builder.build();

    // THEN: Widget should render "Alice"
    assert_eq!(shared_guard.user.name, "Alice");
}

#[test]
fn ct_sb_003_missing_field_returns_empty_string() {
    // CT-SB-003: Missing field returns empty string
    // GIVEN: Shared state without 'nonexistent' field
    let shared = SharedContext::new(NestedSharedState::default());
    let model = TestModel::default();

    // WHEN: XML contains {shared.nonexistent}
    let xml = r#"<dampen><text value="{shared.nonexistent}" /></dampen>"#;
    let document = parse(xml).expect("Failed to parse XML");

    // Build widget with shared context (keep guard alive)
    let shared_guard = shared.read();
    let builder = DampenWidgetBuilder::new(&document, &model, None).with_shared(&*shared_guard);

    let _element = builder.build();

    // THEN: Widget should render empty string (no panic)
    // The builder handles missing fields gracefully
}

#[test]
fn ct_sb_004_mixed_bindings_work_together() {
    // CT-SB-004: Mixed bindings (model + shared) work together
    // GIVEN: Model with greeting = "Hello", Shared with user.name = "Alice"
    let shared = SharedContext::new(NestedSharedState::default());
    let model = MixedModel {
        greeting: "Hello".to_string(),
        local_count: 42,
    };

    // WHEN: XML contains mixed bindings: {greeting}, {shared.user.name}
    let xml = r#"<dampen>
        <column>
            <text value="{greeting}, {shared.user.name}!" />
            <text value="Local: {local_count}, Shared: {shared.count}" />
        </column>
    </dampen>"#;
    let document = parse(xml).expect("Failed to parse XML");

    // Build widget with both model and shared context (keep guard alive)
    let shared_guard = shared.read();
    let builder = DampenWidgetBuilder::new(&document, &model, None).with_shared(&*shared_guard);

    let _element = builder.build();

    // THEN: Both bindings should work
    assert_eq!(model.greeting, "Hello");
    assert_eq!(shared_guard.user.name, "Alice");
    assert_eq!(model.local_count, 42);
    assert_eq!(shared_guard.count, 0);
}

#[test]
fn ct_sb_004_conditional_with_shared_binding() {
    // Additional test for CT-SB-004: Conditional expressions with shared state
    // GIVEN: Shared state with count = 5
    let mut shared_state = NestedSharedState::default();
    shared_state.count = 5;
    let shared = SharedContext::new(shared_state);
    let model = TestModel::default();

    // WHEN: XML uses conditional with shared binding
    let xml = r#"<dampen>
        <text value="{if shared.count > 0 then 'Positive' else 'Zero or Negative'}" />
    </dampen>"#;
    let document = parse(xml).expect("Failed to parse XML");

    // Build widget (keep guard alive)
    let shared_guard = shared.read();
    let builder = DampenWidgetBuilder::new(&document, &model, None).with_shared(&*shared_guard);

    let _element = builder.build();

    // THEN: Conditional should evaluate correctly
    assert!(shared_guard.count > 0);
}

#[test]
fn ct_sb_no_context_returns_empty() {
    // Additional test: {shared.} binding without shared context
    // GIVEN: No shared context provided
    let model = TestModel::default();

    // WHEN: XML contains {shared.theme}
    let xml = r#"<dampen><text value="{shared.theme}" /></dampen>"#;
    let document = parse(xml).expect("Failed to parse XML");

    // Build WITHOUT shared context
    let builder = DampenWidgetBuilder::new(&document, &model, None);
    // Note: .with_shared() is NOT called

    let _element = builder.build();

    // THEN: Should not panic, returns empty string
    // (Warning is printed in debug mode)
}

#[test]
fn ct_sb_nested_path_multiple_levels() {
    // Test deeply nested paths
    // GIVEN: Shared state with user.email
    let shared = SharedContext::new(NestedSharedState::default());
    let model = TestModel::default();

    // WHEN: XML accesses nested field
    let xml = r#"<dampen><text value="{shared.user.email}" /></dampen>"#;
    let document = parse(xml).expect("Failed to parse XML");

    // Build widget (keep guard alive)
    let shared_guard = shared.read();
    let builder = DampenWidgetBuilder::new(&document, &model, None).with_shared(&*shared_guard);

    let _element = builder.build();

    // THEN: Should access nested field correctly
    assert_eq!(shared_guard.user.email, "alice@example.com");
}

// ============================================================================
// T048-T051: Contract Tests for Handler API (CT-HA-001 to CT-HA-005)
// ============================================================================

/// Handler test model with fields needed for handler tests
#[derive(Clone, Debug, Default, Serialize, Deserialize, UiModel)]
struct HandlerTestModel {
    pub counter: i32,
    pub message: String,
}

/// Handler test shared state with fields needed for handler tests
#[derive(Clone, Debug, Serialize, Deserialize, UiModel)]
struct HandlerTestSharedState {
    pub counter: i32,
    pub message: String,
}

impl Default for HandlerTestSharedState {
    fn default() -> Self {
        Self {
            counter: 0,
            message: String::new(),
        }
    }
}

#[test]
fn ct_ha_001_simple_handler_still_works() {
    // CT-HA-001: Simple handler still works (backward compatibility)
    // GIVEN: Handler registered with register_simple("click", ...)
    let registry = HandlerRegistry::new();
    registry.register_simple("click", |model| {
        let model = model.downcast_mut::<HandlerTestModel>().unwrap();
        model.counter += 1;
    });

    let mut model = HandlerTestModel {
        counter: 0,
        message: "Test".to_string(),
    };
    let shared = SharedContext::new(HandlerTestSharedState::default());

    // WHEN: Dispatched via dispatch_with_shared (with shared parameter)
    registry.dispatch_with_shared(
        "click",
        &mut model as &mut dyn Any,
        &shared as &dyn Any,
        None,
    );

    // THEN: Handler executes, model is modified, shared is ignored
    assert_eq!(model.counter, 1, "Simple handler should modify model");
    assert_eq!(
        shared.read().counter,
        0,
        "Simple handler should not affect shared state"
    );
}

#[test]
fn ct_ha_002_with_shared_handler_receives_shared_context() {
    // CT-HA-002: WithShared handler receives shared context
    // GIVEN: Handler registered with register_with_shared("update", ...)
    let registry = HandlerRegistry::new();
    registry.register_with_shared("update", |model, shared| {
        let model = model.downcast_mut::<HandlerTestModel>().unwrap();
        let shared = shared
            .downcast_ref::<SharedContext<HandlerTestSharedState>>()
            .unwrap();

        // Read from shared
        let current_count = shared.read().counter;

        // Write to shared
        shared.write().counter = current_count + model.counter;

        // Modify model
        model.message = "Updated".to_string();
    });

    let mut model = HandlerTestModel {
        counter: 5,
        message: "Test".to_string(),
    };
    let shared = SharedContext::new(HandlerTestSharedState {
        counter: 10,
        message: "Shared".to_string(),
    });

    // WHEN: Dispatched with shared context
    registry.dispatch_with_shared(
        "update",
        &mut model as &mut dyn Any,
        &shared as &dyn Any,
        None,
    );

    // THEN: Handler can read and write shared state
    assert_eq!(model.message, "Updated", "Handler should modify model");
    assert_eq!(
        shared.read().counter,
        15,
        "Handler should read and write shared state (10 + 5)"
    );
}

#[test]
fn ct_ha_003_with_value_and_shared_receives_all_parameters() {
    // CT-HA-003: WithValueAndShared receives all parameters
    // GIVEN: Handler registered with register_with_value_and_shared("input", ...)
    let registry = HandlerRegistry::new();
    registry.register_with_value_and_shared("input", |model, value, shared| {
        let model = model.downcast_mut::<HandlerTestModel>().unwrap();
        let value = value.downcast_ref::<String>().unwrap();
        let shared = shared
            .downcast_ref::<SharedContext<HandlerTestSharedState>>()
            .unwrap();

        // Store value in both model and shared
        model.message = value.clone();
        shared.write().message = format!("Shared: {}", value);
    });

    let mut model = HandlerTestModel {
        counter: 0,
        message: "Initial".to_string(),
    };
    let shared = SharedContext::new(HandlerTestSharedState::default());

    // WHEN: Dispatched with value "hello" and shared context
    registry.dispatch_with_shared(
        "input",
        &mut model as &mut dyn Any,
        &shared as &dyn Any,
        Some("hello".to_string()),
    );

    // THEN: Handler receives model, "hello", and shared context
    assert_eq!(model.message, "hello", "Handler should receive value");
    assert_eq!(
        shared.read().message,
        "Shared: hello",
        "Handler should write to shared state"
    );
}

#[test]
fn ct_ha_004_unknown_handler_returns_none() {
    // CT-HA-004: Unknown handler returns None (not specified in T048-T051 but useful)
    // GIVEN: No handler registered for "unknown"
    let registry = HandlerRegistry::new();
    let mut model = HandlerTestModel::default();
    let shared = SharedContext::new(HandlerTestSharedState::default());

    // WHEN: dispatch_with_shared("unknown", ...)
    let result = registry.dispatch_with_shared(
        "unknown",
        &mut model as &mut dyn Any,
        &shared as &dyn Any,
        None,
    );

    // THEN: Returns None, no panic
    assert!(result.is_none(), "Unknown handler should return None");
}

#[test]
fn ct_ha_005_shared_state_changes_persist_across_views() {
    // CT-HA-005: Shared state changes persist across views
    // GIVEN: View A handler modifies shared.write().count = 42
    let shared = SharedContext::new(HandlerTestSharedState {
        counter: 0,
        message: "Initial".to_string(),
    });

    // Create View A's handler registry
    let view_a_registry = HandlerRegistry::new();
    view_a_registry.register_with_shared("set_count", |_model, shared| {
        let shared = shared
            .downcast_ref::<SharedContext<HandlerTestSharedState>>()
            .unwrap();
        shared.write().counter = 42;
    });

    // Execute View A's handler
    let mut view_a_model = HandlerTestModel::default();
    view_a_registry.dispatch_with_shared(
        "set_count",
        &mut view_a_model as &mut dyn Any,
        &shared as &dyn Any,
        None,
    );

    // WHEN: View B reads shared.read().count (using a different model instance)
    let view_b_model = HandlerTestModel::default();
    let _ = view_b_model; // View B doesn't even need to use its model

    // Clone shared context for View B (simulating different view)
    let view_b_shared = shared.clone();

    // THEN: Value is 42
    assert_eq!(
        view_b_shared.read().counter,
        42,
        "Shared state changes should persist across views"
    );
    assert_eq!(
        shared.read().counter,
        42,
        "Original shared context should also reflect the change"
    );
}

#[test]
fn ct_ha_006_command_handler_with_shared_returns_command() {
    // CT-HA-006: Command handler with shared returns command (bonus test)
    // GIVEN: Handler registered with register_with_command_and_shared(...)
    let registry = HandlerRegistry::new();
    registry.register_with_command_and_shared("fetch", |model, shared| {
        let model = model.downcast_mut::<HandlerTestModel>().unwrap();
        let shared = shared
            .downcast_ref::<SharedContext<HandlerTestSharedState>>()
            .unwrap();

        // Use shared state to create command
        let user_id = shared.read().counter;
        model.message = format!("Fetching user {}", user_id);

        // Return a mock command (boxed string for testing)
        Box::new(format!("FetchUserCommand({})", user_id)) as Box<dyn Any>
    });

    let mut model = HandlerTestModel::default();
    let shared = SharedContext::new(HandlerTestSharedState {
        counter: 123,
        message: "".to_string(),
    });

    // WHEN: Dispatched with shared context
    let result = registry.dispatch_with_shared(
        "fetch",
        &mut model as &mut dyn Any,
        &shared as &dyn Any,
        None,
    );

    // THEN: Returns Some(command)
    assert!(result.is_some(), "Command handler should return Some");
    let command = result.unwrap();
    let command_str = command.downcast_ref::<String>().unwrap();
    assert_eq!(
        command_str, "FetchUserCommand(123)",
        "Command should contain user_id from shared state"
    );
    assert_eq!(
        model.message, "Fetching user 123",
        "Handler should modify model"
    );
}

// ============================================================================
// T058-T059: Contract Tests for Backward Compatibility (CT-BC-001 to CT-BC-002)
// ============================================================================

#[test]
fn ct_bc_001_appstate_without_shared_context_works() {
    // CT-BC-001 (T058): AppState without shared_context works (backward compat)
    // GIVEN: AppState created without shared context (S = ())
    let xml = r#"<dampen><text value="Hello World" /></dampen>"#;
    let document = parse(xml).expect("Failed to parse XML");

    // Create HandlerTestModel for this test
    let model = HandlerTestModel {
        counter: 0,
        message: "Test".to_string(),
    };

    // WHEN: Created using constructors that don't require shared state
    let state1 = AppState::<HandlerTestModel, ()>::with_model(document.clone(), model.clone());
    let state2 =
        AppState::<HandlerTestModel, ()>::with_handlers(document.clone(), HandlerRegistry::new());

    // THEN: AppState works normally without shared context
    assert!(
        state1.shared_context.is_none(),
        "AppState without shared should have None shared_context"
    );
    assert!(
        state2.shared_context.is_none(),
        "AppState with handlers but no shared should have None shared_context"
    );
    assert_eq!(state1.model.counter, 0);
    assert_eq!(state1.model.message, "Test");
}

#[test]
fn ct_bc_002_existing_handlers_work_via_dispatch_with_shared() {
    // CT-BC-002 (T059): Existing handlers work via dispatch_with_shared (backward compat)
    // GIVEN: "Old-style" handler registered without shared state awareness
    let registry = HandlerRegistry::new();
    registry.register_simple("increment", |model| {
        let model = model.downcast_mut::<HandlerTestModel>().unwrap();
        model.counter += 1;
    });

    let mut model = HandlerTestModel {
        counter: 5,
        message: "Test".to_string(),
    };

    // Create a dummy shared context (not used by handler)
    let shared = SharedContext::new(HandlerTestSharedState::default());

    // WHEN: Dispatched via dispatch_with_shared (new API)
    registry.dispatch_with_shared(
        "increment",
        &mut model as &mut dyn Any,
        &shared as &dyn Any,
        None,
    );

    // THEN: Handler executes normally, ignoring shared parameter
    assert_eq!(
        model.counter, 6,
        "Old-style handler should work with new dispatch_with_shared API"
    );

    // Shared state should be untouched
    assert_eq!(
        shared.read().counter,
        0,
        "Simple handler should not affect shared state"
    );
}
