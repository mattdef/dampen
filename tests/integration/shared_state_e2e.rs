//! End-to-end integration tests for shared state with hot-reload.
//!
//! These tests verify that shared state is preserved when XML files are
//! hot-reloaded during development.

use dampen_core::{AppState, HandlerRegistry, SharedContext, parse};
use dampen_macros::UiModel;
use serde::{Deserialize, Serialize};

/// Test shared state
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, UiModel)]
struct TestSharedState {
    pub theme: String,
    pub user_count: i32,
}

impl Default for TestSharedState {
    fn default() -> Self {
        Self {
            theme: "dark".to_string(),
            user_count: 42,
        }
    }
}

/// Test model
#[derive(Clone, Debug, Default, Serialize, Deserialize, UiModel)]
struct TestModel {
    pub local_state: String,
}

#[test]
fn test_hot_reload_preserves_shared_state() {
    // GIVEN: An application with shared state
    let xml_v1 = r#"<dampen>
        <column>
            <text value="Version 1 UI" />
            <text value="{shared.theme}" />
        </column>
    </dampen>"#;

    let document_v1 = parse(xml_v1).expect("Failed to parse XML v1");
    let shared = SharedContext::new(TestSharedState {
        theme: "dark".to_string(),
        user_count: 100,
    });

    let mut state = AppState::with_shared(
        document_v1,
        TestModel {
            local_state: "Initial".to_string(),
        },
        HandlerRegistry::new(),
        shared.clone(),
    );

    // Verify initial state
    assert_eq!(state.shared().unwrap().theme, "dark");
    assert_eq!(state.shared().unwrap().user_count, 100);

    // WHEN: User modifies XML file (simulating file save during development)
    let xml_v2 = r#"<dampen>
        <column spacing="20">
            <text value="Version 2 UI - Redesigned!" size="24" />
            <text value="Theme: {shared.theme}" />
            <text value="Users: {shared.user_count}" />
        </column>
    </dampen>"#;

    let document_v2 = parse(xml_v2).expect("Failed to parse XML v2");

    // Hot-reload the new document
    state.hot_reload(document_v2);

    // THEN: Shared state is preserved
    assert_eq!(
        state.shared().unwrap().theme,
        "dark",
        "Shared state theme should be preserved after hot-reload"
    );
    assert_eq!(
        state.shared().unwrap().user_count,
        100,
        "Shared state user_count should be preserved after hot-reload"
    );

    // AND: Local model state is also preserved
    assert_eq!(
        state.model.local_state, "Initial",
        "Local model state should be preserved after hot-reload"
    );
}

#[test]
fn test_multiple_hot_reloads_preserve_shared_state() {
    // GIVEN: An application with shared state that undergoes multiple reloads
    let xml_v1 = r#"<dampen><text value="V1" /></dampen>"#;
    let document_v1 = parse(xml_v1).expect("Failed to parse XML v1");

    let shared = SharedContext::new(TestSharedState::default());
    let mut state = AppState::with_shared(
        document_v1,
        TestModel::default(),
        HandlerRegistry::new(),
        shared.clone(),
    );

    // Modify shared state
    state.shared_mut().unwrap().user_count = 500;
    state.shared_mut().unwrap().theme = "light".to_string();

    // WHEN: Multiple hot-reloads happen
    for i in 2..=5 {
        let xml = format!(r#"<dampen><text value="V{}" /></dampen>"#, i);
        let document = parse(&xml).expect("Failed to parse XML");
        state.hot_reload(document);

        // THEN: Shared state is still preserved after each reload
        assert_eq!(
            state.shared().unwrap().user_count,
            500,
            "Shared state should be preserved after reload {}",
            i
        );
        assert_eq!(
            state.shared().unwrap().theme,
            "light",
            "Shared state should be preserved after reload {}",
            i
        );
    }
}

#[test]
fn test_hot_reload_with_shared_state_modifications() {
    // GIVEN: An application where shared state is modified between reloads
    let xml_v1 = r#"<dampen><text value="V1" /></dampen>"#;
    let document_v1 = parse(xml_v1).expect("Failed to parse XML v1");

    let shared = SharedContext::new(TestSharedState::default());
    let mut state = AppState::with_shared(
        document_v1,
        TestModel::default(),
        HandlerRegistry::new(),
        shared.clone(),
    );

    // WHEN: State is modified, then hot-reload happens
    state.shared_mut().unwrap().user_count = 200;

    let xml_v2 = r#"<dampen><text value="V2" /></dampen>"#;
    let document_v2 = parse(xml_v2).expect("Failed to parse XML v2");
    state.hot_reload(document_v2);

    // Modify again after reload
    state.shared_mut().unwrap().user_count = 300;

    let xml_v3 = r#"<dampen><text value="V3" /></dampen>"#;
    let document_v3 = parse(xml_v3).expect("Failed to parse XML v3");
    state.hot_reload(document_v3);

    // THEN: Latest modification is preserved
    assert_eq!(
        state.shared().unwrap().user_count,
        300,
        "Latest shared state modification should be preserved"
    );
}

#[test]
fn test_hot_reload_without_shared_state_works() {
    // GIVEN: An AppState without shared state (backward compatibility)
    let xml_v1 = r#"<dampen><text value="V1" /></dampen>"#;
    let document_v1 = parse(xml_v1).expect("Failed to parse XML v1");

    let mut state = AppState::<TestModel, ()>::with_model(
        document_v1,
        TestModel {
            local_state: "Test".to_string(),
        },
    );

    // WHEN: Hot-reload happens
    let xml_v2 = r#"<dampen><text value="V2" /></dampen>"#;
    let document_v2 = parse(xml_v2).expect("Failed to parse XML v2");
    state.hot_reload(document_v2);

    // THEN: Still works fine (no panic, no errors)
    assert_eq!(state.model.local_state, "Test");
    assert!(state.shared_context.is_none());
}
